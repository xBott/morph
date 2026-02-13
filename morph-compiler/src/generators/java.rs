use std::fs;
use std::path::Path;
use crate::utils::create_file_if_not_exists;
use crate::generator::Generator;
use crate::parser::Packet;
use crate::token::FieldType;

pub struct JavaGenerator {
    package_name: String,
}

impl JavaGenerator {

    pub fn new(package_name: String) -> Self {
        Self { package_name }
    }

    fn convert_field_type(&self, field_type: &FieldType) -> String {
        match field_type {
            FieldType::Bool => "boolean".into(),
            FieldType::I32 => "int".into(),
            FieldType::Str => "String".into(),
        }
    }

    fn generate_packets(&self, output_dir: &Path, packets: &Vec<Packet>) {

        for packet in packets {
            self.generate_packet(output_dir, packet);
        }

    }

    fn generate_packet(&self, output_dir: &Path, packet: &Packet) {

        let path = output_dir.join(format!("{}.java", packet.name));

        if let Ok(created) = create_file_if_not_exists(&path) {

            if created {
                println!("\nFile {} is created.", path.to_string_lossy());
            }

            let mut content = String::new();

            content.push_str(format!("package {};\n\n", self.package_name).as_str());
            self.generate_imports(&mut content);
            content.push_str(format!("public final class {} implements MorphPacket {{\n\n", packet.name).as_str());
            self.generate_packet_id(&mut content, packet);
            self.generate_fields(&mut content, packet);
            self.generate_constructor(&mut content, packet);
            self.generate_getters(&mut content, packet);
            self.generate_setters(&mut content, packet);
            self.generate_encode(&mut content, packet);
            self.generate_decode(&mut content, packet);
            content.push_str("}");

            fs::write(&path, content).unwrap();

        }

    }

    fn generate_imports(&self, content: &mut String) {
        content.push_str("import java.nio.ByteBuffer;\n");
        content.push_str("import java.nio.charset.StandardCharsets;\n\n");
        content.push_str("import me.bottdev.morph.runtime.MorphPacket;\n\n");
    }

    fn generate_packet_id(&self, content: &mut String, packet: &Packet) {
        content.push_str(format!("\tprivate static final int PACKET_ID = {};\n\n", packet.id).as_str());
        content.push_str("\tpublic int getPacketId() {\n");
        content.push_str("\t\treturn PACKET_ID;\n");
        content.push_str("\t}\n\n");
    }

    fn generate_fields(&self, content: &mut String, packet: &Packet) {
        for field in &packet.fields {

            let java_type = self.convert_field_type(&field.typ);
            content.push_str(format!("\tprivate {} {};\n", java_type, field.name).as_str());

        }
        content.push_str("\n");
    }

    fn generate_constructor(&self, content: &mut String, packet: &Packet) {

        content.push_str(format!("\tpublic {}(\n", packet.name).as_str());

        let mut iter = packet.fields.iter().peekable();

        while let Some(field) = iter.next() {
            let java_type = self.convert_field_type(&field.typ);

            if iter.peek().is_some() {
                content.push_str(&format!("\t\t{} {},\n", java_type, field.name));
            } else {
                content.push_str(&format!("\t\t{} {}\n", java_type, field.name));
            }
        }

        content.push_str("\t) {\n");

        for field in &packet.fields {
            content.push_str(&format!("\t\tthis.{} = {};\n", field.name, field.name));
        }

        content.push_str("\t}\n");

        content.push_str("\n");
    }

    fn generate_getters(&self, content: &mut String, packet: &Packet) {
        for field in &packet.fields {

            let java_type = self.convert_field_type(&field.typ);
            let capitalized = crate::utils::capitalize(field.name.as_str());

            let getter_name = match java_type.as_str() {
                "boolean" => format!("is{}", capitalized),
                _ => format!("get{}", capitalized),
            };

            content.push_str(format!("\n\tpublic {} {}() {{\n", java_type, getter_name).as_str());
            content.push_str(format!("\t\treturn this.{};\n", field.name).as_str());
            content.push_str("\t}\n");
        }
    }

    fn generate_setters(&self, content: &mut String, packet: &Packet) {
        for field in &packet.fields {

            let java_type = self.convert_field_type(&field.typ);
            let capitalized = crate::utils::capitalize(field.name.as_str());

            content.push_str(format!("\n\tpublic void set{}({} value) {{\n", capitalized, java_type).as_str());
            content.push_str(format!("\t\tthis.{} = value;\n", field.name).as_str());
            content.push_str("\t}\n");
        }
    }

    fn generate_encode(&self, content: &mut String, packet: &Packet) {
        content.push_str("\t@Override");
        content.push_str("\n\tpublic byte[] encode() {\n");

        content.push_str("\t\tByteBuffer buffer = ByteBuffer.allocate(1024);\n");
        content.push_str("\t\tbuffer.putInt(PACKET_ID);\n\n");

        for field in &packet.fields {
            content.push_str(
                match field.typ {
                    FieldType::Bool => {
                        format!("\t\tbuffer.put((byte) (this.{} ? 1 : 0));\n", field.name)
                    }
                    FieldType::I32 => {
                        format!("\t\tbuffer.putInt(this.{});\n\n", field.name)
                    }
                    FieldType::Str => {
                        format!(
                            "\t\tbyte[] {}Bytes = this.{}.getBytes(StandardCharsets.UTF_8);\n\
                             \t\tbuffer.putInt({}Bytes.length);\n\
                             \t\tbuffer.put({}Bytes);\n\n",
                            field.name, field.name, field.name, field.name
                        )
                    }
                }.as_str()
            );
        }

        content.push_str("\t\tbyte[] result = new byte[buffer.position()];\n");
        content.push_str("\t\tbuffer.flip();\n");
        content.push_str("\t\tbuffer.get(result);\n");
        content.push_str("\t\treturn result;\n");

        content.push_str("\t}\n\n");
    }

    fn generate_decode(&self, content: &mut String, packet: &Packet) {
        content.push_str(format!("\n\tpublic static {} decode(byte[] data) {{\n", packet.name).as_str());

        content.push_str("\t\tByteBuffer buffer = ByteBuffer.wrap(data);\n");

        content.push_str("\t\tif (data.length < 4) {\n");
        content.push_str("\t\t\tthrow new IllegalArgumentException(\"Data too short: expected at least 4 bytes for packet ID\");\n");
        content.push_str("\t\t}\n\n");

        content.push_str("\t\tint packetId = buffer.getInt();\n\n");
        content.push_str("\t\tif (packetId != PACKET_ID) {\n");
        content.push_str("\t\t\tthrow new IllegalArgumentException(\n");
        content.push_str("\t\t\t\t\"Invalid packet ID: expected \" + PACKET_ID + \", got \" + packetId\n");
        content.push_str("\t\t\t);\n");
        content.push_str("\t\t}\n\n");

        for field in &packet.fields {
            let decode_code = match field.typ {
                FieldType::Bool => {
                    format!(
                        "\t\tif (buffer.remaining() < 1) {{\n\
                     \t\t\tthrow new IllegalArgumentException(\"Not enough data for field '{}'\");\n\
                     \t\t}}\n\
                     \t\tboolean {} = buffer.get() != 0;\n\n",
                        field.name, field.name
                    )
                }
                FieldType::I32 => {
                    format!(
                        "\t\tif (buffer.remaining() < 4) {{\n\
                     \t\t\tthrow new IllegalArgumentException(\"Not enough data for field '{}'\");\n\
                     \t\t}}\n\
                     \t\tint {} = buffer.getInt();\n\n",
                        field.name, field.name
                    )
                }
                FieldType::Str => {
                    format!(
                        "\t\tif (buffer.remaining() < 4) {{\n\
                     \t\t\tthrow new IllegalArgumentException(\"Not enough data for field '{}' length\");\n\
                     \t\t}}\n\
                     \t\tint {}Length = buffer.getInt();\n\
                     \t\tif ({}Length < 0) {{\n\
                     \t\t\tthrow new IllegalArgumentException(\"Invalid length for field '{}': \" + {}Length);\n\
                     \t\t}}\n\
                     \t\tif (buffer.remaining() < {}Length) {{\n\
                     \t\t\tthrow new IllegalArgumentException(\"Not enough data for field '{}' content\");\n\
                     \t\t}}\n\
                     \t\tbyte[] {}Bytes = new byte[{}Length];\n\
                     \t\tbuffer.get({}Bytes);\n\
                     \t\tString {} = new String({}Bytes, StandardCharsets.UTF_8);\n\n",
                        field.name, field.name, field.name, field.name, field.name,
                        field.name, field.name, field.name, field.name, field.name, field.name, field.name
                    )
                }
            };
            content.push_str(&decode_code.as_str());
        }

        content.push_str("\n\t\tif (buffer.hasRemaining()) {\n");
        content.push_str("\t\t\tthrow new IllegalArgumentException(\n");
        content.push_str("\t\t\t\t\"Extra bytes remaining after decoding: \" + buffer.remaining() + \" bytes\"\n");
        content.push_str("\t\t\t);\n");
        content.push_str("\t\t}\n\n");

        content.push_str("\t\treturn new ");
        content.push_str(&packet.name.as_str());
        content.push_str("(");

        let mut iter = packet.fields.iter().peekable();
        while let Some(field) = iter.next() {
            content.push_str(&field.name);
            if iter.peek().is_some() {
                content.push_str(", ");
            }
        }

        content.push_str(");\n");
        content.push_str("\t}\n");
    }

}

impl Generator for JavaGenerator {

    fn generate(&self, output_dir: &Path, packets: &Vec<Packet>) {
        println!("Generating Java code in directory {}", output_dir.to_string_lossy());
        self.generate_packets(output_dir, packets);
    }

}