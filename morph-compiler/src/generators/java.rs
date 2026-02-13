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
            content.push_str(format!("package {}\n\n", self.package_name).as_str());
            content.push_str(format!("public final class {} {{\n\n", packet.name).as_str());
            self.generate_fields(&mut content, packet);
            self.generate_constructor(&mut content, packet);
            self.generate_getters(&mut content, packet);
            self.generate_setters(&mut content, packet);
            content.push_str("}");

            fs::write(&path, content).unwrap();
        }

    }

    fn generate_fields(&self, content: &mut String, packet: &Packet) {
        for field in &packet.fields {

            let java_type = self.convert_field_type(&field.typ);
            content.push_str(format!("\tprivate {} {}\n", java_type, field.name).as_str());

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
            content.push_str(&format!("\t\tthis.{} = {}\n", field.name, field.name));
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

}

impl Generator for JavaGenerator {

    fn generate(&self, output_dir: &Path, packets: &Vec<Packet>) {
        println!("Generating Java code in directory {}", output_dir.to_string_lossy());
        self.generate_packets(output_dir, packets);
    }

}