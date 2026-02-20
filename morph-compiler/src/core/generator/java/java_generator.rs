use crate::core::{FieldType, GenerationError, Generator, Packet};
use crate::utils::MorphResult::{Errors, Success};
use crate::utils::{capitalize, hash_str_to_i8_positive, MorphResult};
use serde::Deserialize;
use std::fmt::Display;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct JavaOptions {
    pub output_dir: String,

    pub package: String,

    #[serde(default = "default_true")]
    pub no_args_constructor: bool,

    #[serde(default = "default_true")]
    pub generate_to_string: bool,

    #[serde(default = "default_true")]
    pub generate_equals: bool,

    #[serde(default = "default_true")]
    pub generate_hashcode: bool,

}

fn default_true() -> bool { true }

pub struct JavaGenerator {
    pub options: JavaOptions,
}

enum JavaType {
    Bool,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    Char,
    Str,
    Array(Box<JavaType>),
    Class(String),
}

impl JavaType {
    fn is_primitive(&self) -> bool {
        match self {
            JavaType::Bool |
            JavaType::Byte |
            JavaType::Short |
            JavaType::Int |
            JavaType::Long |
            JavaType::Float |
            JavaType::Double |
            JavaType::Char => true,
            _=> false
        }
    }
}

impl Display for JavaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JavaType::Bool => write!(f, "boolean"),
            JavaType::Byte => write!(f, "byte"),
            JavaType::Short => write!(f, "short"),
            JavaType::Int => write!(f, "int"),
            JavaType::Long => write!(f, "long"),
            JavaType::Float => write!(f, "float"),
            JavaType::Double => write!(f, "double"),
            JavaType::Char => write!(f, "char"),
            JavaType::Str => write!(f, "String"),
            JavaType::Array(inner_type) => {
                if inner_type.is_primitive() {

                    match convert_primitive_to_wrapper(inner_type) {
                        Some(wrapper) => write!(f, "List<{}>", wrapper),
                        None => write!(f, "List<{}>", inner_type),
                    }

                } else {
                    write!(f, "List<{}>", inner_type)

                }
            }
            JavaType::Class(class_name) => write!(f, "{}", class_name),
        }
    }
}

fn convert_to_java_type(field_type: &FieldType) -> JavaType {

    match field_type {

        FieldType::Bool => JavaType::Bool,
        FieldType::I8 | FieldType::U8 => JavaType::Byte,
        FieldType::I16 | FieldType::U16 => JavaType::Short,
        FieldType::I32 | FieldType::U32 => JavaType::Int,
        FieldType::I64 | FieldType::U64 => JavaType::Long,
        FieldType::F32 => JavaType::Float,
        FieldType::F64 => JavaType::Double,
        FieldType::Char => JavaType::Char,
        FieldType::Str => JavaType::Str,
        FieldType::Array(array_type) => {
            JavaType::Array(
                Box::new(convert_to_java_type(&**array_type))
            )
        },
        FieldType::Nested(class_name) => JavaType::Class(class_name.clone())

    }

}

enum JavaPrimitiveWrapper {
    BoolWrapper,
    ByteWrapper,
    ShortWrapper,
    IntWrapper,
    LongWrapper,
    FloatWrapper,
    DoubleWrapper,
    CharWrapper,
}

impl Display for JavaPrimitiveWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JavaPrimitiveWrapper::BoolWrapper => write!(f, "Boolean"),
            JavaPrimitiveWrapper::ByteWrapper => write!(f, "Byte"),
            JavaPrimitiveWrapper::ShortWrapper => write!(f, "Short"),
            JavaPrimitiveWrapper::IntWrapper => write!(f, "Integer"),
            JavaPrimitiveWrapper::LongWrapper => write!(f, "Long"),
            JavaPrimitiveWrapper::FloatWrapper => write!(f, "Float"),
            JavaPrimitiveWrapper::DoubleWrapper => write!(f, "Double"),
            JavaPrimitiveWrapper::CharWrapper => write!(f, "Character"),
        }
    }
}

fn convert_primitive_to_wrapper(field_type: &JavaType) -> Option<JavaPrimitiveWrapper> {

    match field_type {
        JavaType::Bool => Some(JavaPrimitiveWrapper::BoolWrapper),
        JavaType::Byte => Some(JavaPrimitiveWrapper::ByteWrapper),
        JavaType::Short => Some(JavaPrimitiveWrapper::ShortWrapper),
        JavaType::Int => Some(JavaPrimitiveWrapper::IntWrapper),
        JavaType::Long => Some(JavaPrimitiveWrapper::LongWrapper),
        JavaType::Float => Some(JavaPrimitiveWrapper::FloatWrapper),
        JavaType::Double => Some(JavaPrimitiveWrapper::DoubleWrapper),
        JavaType::Char => Some(JavaPrimitiveWrapper::CharWrapper),
        _ => None,
    }

}

fn primitive_or_wrapper_string(field_type: &JavaType) -> String {
    if field_type.is_primitive() {
        match convert_primitive_to_wrapper(field_type) {
            Some(wrapper) => wrapper.to_string(),
            None => field_type.to_string(),
        }
    } else {
        field_type.to_string()
    }

}

fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = false;

    for (i, c) in s.chars().enumerate() {
        if c == ' ' || c == '_' || c == '-' {
            capitalize_next = true;
        } else if i == 0 {
            result.push(c.to_ascii_lowercase());
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

impl JavaGenerator {

    fn generate_class(&self, packet: &Packet) -> String {

        let mut content = String::new();

        self.write_package(&mut content);
        self.write_imports(&mut content);
        if self.has_arrays(packet) {
            self.write_import_list(&mut content);
        }
        self.write_class(&mut content, packet);
        self.write_packet_id(&mut content, packet);
        self.write_fields(&mut content, packet);
        self.write_constructors(&mut content, packet);
        self.write_packet_id_getter(&mut content);
        self.write_getters(&mut content, packet);
        self.write_setters(&mut content, packet);
        self.write_encode_method(&mut content, packet);
        self.write_decode_method(&mut content, packet);
        self.write_registry_registration(&mut content, packet);
        if self.options.generate_to_string {
            self.write_to_string(&mut content, packet);
        }
        if self.options.generate_equals {
            self.write_equals(&mut content, packet);
        }
        if self.options.generate_hashcode {
            self.write_hash_code(&mut content, packet);
        }
        content.push_str("}");

        content

    }

    fn has_arrays(&self, packet: &Packet) -> bool {

        for field in &packet.fields {
            if let FieldType::Array(_) = &field.typ {
                return true;
            }
        }

        false

    }
    
    fn write_package(&self, content: &mut String) {
        content.push_str(format!("package {};\n\n", self.options.package).as_str());
    }

    fn write_imports(&self, content: &mut String) {
        content.push_str("import me.bottdev.morph.runtime.MorphPacket;\n");
        content.push_str("import me.bottdev.morph.runtime.BinaryWriter;\n");
        content.push_str("import me.bottdev.morph.runtime.BinaryReader;\n");
        content.push_str("import me.bottdev.morph.runtime.PacketRegistries;\n\n");
        content.push_str("import java.io.ByteArrayOutputStream;\n");
        content.push_str("import java.io.InputStream;\n");
        content.push_str("import java.io.IOException;\n");
        content.push_str("import java.util.Objects;\n");
    }

    fn write_import_list(&self, content: &mut String) {
        content.push_str("import java.util.List;\n");
        content.push_str("import java.util.ArrayList;\n");
    }

    fn write_class(&self, content: &mut String, packet: &Packet) {
        content.push_str(format!("\npublic final class {} implements MorphPacket {{\n\n", packet.name).as_str());
    }

    fn write_packet_id(&self, content: &mut String, packet: &Packet) {
        let hash = hash_str_to_i8_positive(packet.name.as_str());
        content.push_str(format!("\tpublic static final byte PACKET_ID = {};\n", hash).as_str());
    }

    fn write_packet_id_getter(&self, content: &mut String) {
        content.push_str("\t@Override\n");
        content.push_str("\tpublic byte getPacketId() {{\n");
        content.push_str("\t\treturn PACKET_ID;\n");
        content.push_str("\t}}\n\n");
    }

    fn write_fields(&self, content: &mut String, packet: &Packet) {
        content.push_str("\n");
        for field in &packet.fields {
            let java_type = convert_to_java_type(&field.typ);
            let camel_case = to_camel_case(field.name.as_str());

            if let FieldType::Array(_) = &field.typ {
                content.push_str(format!("\tprivate {} {} = new ArrayList<>();\n", java_type, camel_case).as_str());

            } else {
                content.push_str(format!("\tprivate {} {};\n", java_type, camel_case).as_str());

            }

        }
        content.push_str("\n");
    }

    fn write_constructors(&self, content: &mut String, packet: &Packet) {
        self.write_all_args_constructor(content, packet);
        if self.options.no_args_constructor {
            self.write_no_args_constructor(content, packet);
        }
    }

    fn write_all_args_constructor(&self, content: &mut String, packet: &Packet) {
        content.push_str(format!("\tpublic {}(\n", packet.name).as_str());

        for (i, field) in packet.fields.iter().enumerate() {
            let java_type = convert_to_java_type(&field.typ);
            let camel_case = to_camel_case(&field.name);
            let comma = if i == packet.fields.len() - 1 { "" } else { "," };
            content.push_str(format!("\t\t{} {}{}\n", java_type, camel_case, comma).as_str());
        }

        content.push_str("\t) {\n");

        for field in &packet.fields {
            let camel_case = to_camel_case(&field.name);
            content.push_str(format!("\t\tthis.{} = {};\n", camel_case, camel_case).as_str());
        }

        content.push_str("\t}\n\n");
    }

    fn write_no_args_constructor(&self, content: &mut String, packet: &Packet) {
        content.push_str(format!("\tpublic {}() {{}}\n\n", packet.name).as_str());
    }

    fn write_getters(&self, content: &mut String, packet: &Packet) {

        for field in &packet.fields {

            let java_type = convert_to_java_type(&field.typ);
            let camel_case = to_camel_case(field.name.as_str());
            let capitalized = capitalize(camel_case.as_str());

            let getter_name = match java_type {
                JavaType::Bool => format!("is{}", capitalized),
                _ => format!("get{}", capitalized)
            };

            content.push_str(format!("\tpublic {} {}() {{\n", java_type, getter_name).as_str());
            content.push_str(format!("\t\treturn {};\n", camel_case).as_str());
            content.push_str("\t}\n\n");


        }

    }

    fn write_setters(&self, content: &mut String, packet: &Packet) {

        for field in &packet.fields {

            let java_type = convert_to_java_type(&field.typ);
            let camel_case = to_camel_case(field.name.as_str());
            let capitalized = capitalize(camel_case.as_str());

            content.push_str(format!("\tpublic void set{}({} value) {{\n", capitalized, java_type).as_str());
            content.push_str(format!("\t\tthis.{} = value;\n", camel_case).as_str());
            content.push_str("\t}\n\n");


        }

    }

    fn write_encode_method(&self, content: &mut String, packet: &Packet) {
        content.push_str("\t@Override\n");
        content.push_str("\tpublic void encode(ByteArrayOutputStream out, boolean encodeId) {\n\n");


        content.push_str("\t\tif (encodeId) BinaryWriter.writeByte(out, PACKET_ID);\n\n");

        for field in &packet.fields {
            let name = field.name.as_str();
            let java_type = convert_to_java_type(&field.typ);
            self.write_encode_field(content, name, &java_type, 1);
        }

        content.push_str("\t}\n");
    }

    fn write_encode_field(
        &self,
        content: &mut String,
        name: &str,
        java_type: &JavaType,
        indent: usize
    ) {

        let indent_str = "\t".repeat(indent);
        let camel_case = to_camel_case(name);

        match java_type {
            JavaType::Bool => {
                content.push_str(format!(
                    "\t{}BinaryWriter.writeBoolean(out, {});\n\n", indent_str, camel_case
                ).as_str());
            }
            JavaType::Byte => {
                content.push_str(format!(
                    "\t{}BinaryWriter.writeByte(out, {});\n\n", indent_str, camel_case
                ).as_str());
            }
            JavaType::Short => {
                content.push_str(format!(
                    "\t{}BinaryWriter.writeShort(out, {});\n\n", indent_str, camel_case
                ).as_str());
            }
            JavaType::Int => {
                content.push_str(format!(
                    "\t{}BinaryWriter.writeInt(out, {});\n\n", indent_str, camel_case
                ).as_str());
            }
            JavaType::Long => {
                content.push_str(format!(
                    "\t{}BinaryWriter.writeLong(out, {});\n\n", indent_str, camel_case
                ).as_str());
            }
            JavaType::Float => {
                content.push_str(format!(
                    "\t{}BinaryWriter.writeFloat(out, {});\n\n", indent_str, camel_case
                ).as_str());
            }
            JavaType::Double => {
                content.push_str(format!(
                    "\t{}BinaryWriter.writeDouble(out, {});\n\n", indent_str, camel_case
                ).as_str());
            }
            JavaType::Char => {
                content.push_str(format!(
                    "\t{}BinaryWriter.writeChar(out, {});\n\n", indent_str, camel_case
                ).as_str());
            }
            JavaType::Str => {
                content.push_str(format!(
                    "\t{}BinaryWriter.writeString(out, {});\n\n", indent_str, camel_case
                ).as_str());
            }
            JavaType::Array(inner) => {

                content.push_str(format!(
                    "\t{}BinaryWriter.writeInt(out, {}.size());\n", indent_str, camel_case
                ).as_str());

                let item_name = match indent > 1 {
                    true => format!("{}Item{}", camel_case, indent - 1),
                    false => format!("{}Item", camel_case)
                };

                content.push_str(format!("\t{}for ({} {} : {}) {{\n\n", indent_str, inner, item_name, camel_case).as_str());
                self.write_encode_field(content, item_name.as_str(), &**inner, indent + 1);
                content.push_str(format!("\t{}}}\n\n", indent_str).as_str());
            }
            JavaType::Class(_) => {
                content.push_str(format!(
                    "\t{}{}.encode(out, false);\n\n", indent_str, camel_case
                ).as_str());
            }
        }
    }

    fn write_decode_method(&self, content: &mut String, packet: &Packet) {
        content.push_str(format!(
            "\tpublic static {} decode(InputStream in) throws IOException {{\n",
            packet.name
        ).as_str());

        content.push_str("\t\ttry {\n");
        for field in &packet.fields {
            let java_type = convert_to_java_type(&field.typ);
            self.write_decode_field(content, field.name.as_str(), &java_type, 1);
        }

        content.push_str(format!("\t\t\treturn new {}(\n", packet.name).as_str());
        for (i, field) in packet.fields.iter().enumerate() {
            let comma = if i == packet.fields.len() - 1 { "" } else { "," };
            content.push_str(format!("\t\t\t\t{}{}\n", field.name, comma).as_str());
        }
        content.push_str("\t\t\t);\n\n");

        content.push_str("\t\t} catch (Exception e) {\n");
        content.push_str(format!("\t\t\tthrow new IOException(\"Failed to decode {}\", e);\n\n", packet.name).as_str());
        content.push_str("\t\t}\n");

        content.push_str("\t}\n\n");
    }

    fn write_decode_field(
        &self,
        content: &mut String,
        name: &str,
        java_type: &JavaType,
        indent: usize
    ) {

        let indent_str = "\t".repeat(indent);
        let camel_case = to_camel_case(name);

        match java_type {
            JavaType::Bool => {
                content.push_str(format!("\t\t{}boolean {} = BinaryReader.readBoolean(in);\n\n", indent_str, name).as_str());
            }
            JavaType::Byte => {
                content.push_str(format!("\t\t{}byte {} = BinaryReader.readByte(in);\n\n", indent_str, name).as_str());
            }
            JavaType::Short => {
                content.push_str(format!("\t\t{}short {} = BinaryReader.readShort(in);\n\n", indent_str, name).as_str());
            }
            JavaType::Int => {
                content.push_str(format!("\t\t{}int {} = BinaryReader.readInt(in);\n\n", indent_str, name).as_str());
            }
            JavaType::Long => {
                content.push_str(format!("\t\t{}long {} = BinaryReader.readLong(in);\n\n", indent_str, name).as_str());
            }
            JavaType::Float => {
                content.push_str(format!("\t\t{}float {} = BinaryReader.readFloat(in);\n\n", indent_str, name).as_str());
            }
            JavaType::Double => {
                content.push_str(format!("\t\t{}double {} = BinaryReader.readDouble(in);\n", indent_str, name).as_str());
            }
            JavaType::Char => {
                content.push_str(format!("\t\t{}char {} = BinaryReader.readChar(in);\n\n", indent_str, name).as_str());
            }
            JavaType::Str => {
                content.push_str(format!("\t\t{}String {} = BinaryReader.readString(in);\n\n", indent_str, name).as_str());

            }
            JavaType::Array(inner_type) => {

                let length_name = format!("{}Length", camel_case);
                let index_name = format!("{}Index", camel_case);

                content.push_str(format!("\t\t{}int {} = BinaryReader.readInt(in);\n", indent_str, length_name).as_str());

                content.push_str(
                    format!("\t\t{}ArrayList<{}> {} = new ArrayList<>();\n",
                            indent_str, primitive_or_wrapper_string(&**inner_type), name
                    ).as_str()
                );

                content.push_str(
                    format!("\t\t{}for (int {} = 0; {} < {}; {}++) {{\n",
                                         indent_str, index_name, index_name, length_name, index_name).as_str()
                );

                match &**inner_type {
                    JavaType::Bool |
                    JavaType::Byte |
                    JavaType::Short |
                    JavaType::Int |
                    JavaType::Long |
                    JavaType::Float |
                    JavaType::Double |
                    JavaType::Char |
                    JavaType::Str |
                    JavaType::Class(_) => {
                        let item_name = format!("{}_item", name);
                        self.write_decode_field(content, item_name.as_str(), &**inner_type, indent + 1);
                        content.push_str(
                            format!("\t\t\t{}{}.add({});\n", indent_str, name, item_name).as_str(),
                        );
                    }
                    JavaType::Array(_) => {
                        let item_name = format!("{}_item", name);
                        self.write_decode_field(content, item_name.as_str(), &**inner_type, indent + 1);
                        content.push_str(
                            format!("\t\t\t{}{}.add({});\n", indent_str, name, item_name).as_str(),
                        );
                    }
                }

                content.push_str(format!("\t\t{}}}\n\n", indent_str).as_str());

            }
            JavaType::Class(class_name) => {
                content.push_str(format!("\t\t{}{} {} = {}.decode(in);\n\n", indent_str, class_name, name, class_name).as_str());
            }
        }
    }

    fn write_registry_registration(&self, content: &mut String, packet: &Packet) {
        content.push_str("\tstatic {\n");
        content.push_str(format!("\t\tPacketRegistries.DEFAULT.register(PACKET_ID, {}::decode);\n", packet.name).as_str());
        content.push_str("\t}\n\n")
    }

    fn write_to_string(&self, content: &mut String, packet: &Packet) {
        content.push_str("\t@Override\n");
        content.push_str("\tpublic String toString() {\n");
        content.push_str(format!("\t\treturn \"{}{{\" +\n", packet.name).as_str());

        if packet.fields.is_empty() {
            content.push_str("\t\t'}';\n");
        } else {
            for (i, field) in packet.fields.iter().enumerate() {
                let camel_case = to_camel_case(&field.name);
                let prefix = if i == 0 { "" } else { "\", \" + " };
                content.push_str(format!("\t\t\t{}\"{}=\" + {} +\n", prefix, camel_case, camel_case).as_str());
            }
            content.push_str("\t\t'}';\n");
        }

        content.push_str("\t}\n\n");
    }

    fn write_equals(&self, content: &mut String, packet: &Packet) {
        content.push_str("\t@Override\n");
        content.push_str("\tpublic boolean equals(Object o) {\n");
        content.push_str("\t\tif (this == o) return true;\n");
        content.push_str("\t\tif (o == null || getClass() != o.getClass()) return false;\n");
        content.push_str(format!("\t\t{} that = ({}) o;\n", packet.name, packet.name).as_str());

        let comparisons: Vec<String> = packet.fields.iter().map(|field| {
            let camel_case = to_camel_case(&field.name);
            match &field.typ {
                FieldType::Array(_) | FieldType::Str | FieldType::Nested(_) => {
                    format!("Objects.equals({}, that.{})", camel_case, camel_case)
                }
                _ => format!("{} == that.{}", camel_case, camel_case),
            }
        }).collect();

        let body = if comparisons.is_empty() {
            "true".to_string()
        } else {
            comparisons.join("\n\t\t\t&& ")
        };

        content.push_str(&format!("\t\treturn {};\n", body));
        content.push_str("\t}\n\n");
    }

    fn write_hash_code(&self, content: &mut String, packet: &Packet) {
        content.push_str("\t@Override\n");
        content.push_str("\tpublic int hashCode() {\n");

        if packet.fields.is_empty() {
            content.push_str("\t\treturn 0;\n");
        } else {
            let args: Vec<String> = packet.fields.iter()
                .map(|field| to_camel_case(&field.name))
                .collect();
            content.push_str(&format!("\t\treturn Objects.hash({});\n", args.join(", ")));
        }

        content.push_str("\t}\n\n");
    }

}

impl Generator for JavaGenerator {

    fn generate(&self, packets: &Vec<Packet>) -> MorphResult<()> {

        let output_dir = Path::new(&self.options.output_dir);

        if let Err(err) = std::fs::create_dir_all(output_dir) {
            let morph_err = GenerationError {
                message: format!("Failed to create output directory: {}", err.to_string()),
            };
            return Errors(vec![Box::new(morph_err)]);
        }

        for packet in packets {

            let content = self.generate_class(packet);
            let path = output_dir.join(format!("{}.java", packet.name));

            match std::fs::write(&path, content) {
                Ok(_) => {}
                Err(err) => {
                    let morph_err = GenerationError {
                        message: format!("Failed to write {}.java file: {}", packet.name, err),
                    };
                    return Errors(vec![Box::new(morph_err)]);
                }
            }


        }

        Success(())

    }

}