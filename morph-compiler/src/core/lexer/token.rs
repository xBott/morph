use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Keyword(KeywordKind),
    Operator(OperatorKind),
    BraceOpen,
    BraceClose,
    Qualifier(String),
    Number(i32)
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Keyword(keyword) => write!(f, "{}", keyword),
            TokenKind::Operator(kind) => write!(f, "{}", kind),
            TokenKind::BraceOpen => write!(f, "{{"),
            TokenKind::BraceClose => write!(f, "}}"),
            TokenKind::Qualifier(qualifier) => write!(f, "{}", qualifier),
            TokenKind::Number(number) => write!(f, "{}", number),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeywordKind {
    Packet,
    Identifier,
    Auto,
    Fields,
    FieldDefinition(FieldType),
    Array
}

impl Display for KeywordKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeywordKind::Packet => write!(f, "packet"),
            KeywordKind::Identifier => write!(f, "id"),
            KeywordKind::Auto => write!(f, "auto"),
            KeywordKind::Fields => write!(f, "fields"),
            KeywordKind::FieldDefinition(field_type) => write!(f, "{}", field_type),
            KeywordKind::Array => write!(f, "array"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperatorKind {
    Equals,
}

impl Display for OperatorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperatorKind::Equals => write!(f, "="),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {

    Bool,

    I8,
    I16,
    I32,
    I64,

    U8,
    U16,
    U32,
    U64,

    F32,
    F64,

    Char,
    Str,

    Array(Box<FieldType>),
    Nested(String),

}

impl Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FieldType::Bool => write!(f, "bool"),

            FieldType::I8 => write!(f, "i8"),
            FieldType::I16 => write!(f, "i16"),
            FieldType::I32 => write!(f, "i32"),
            FieldType::I64 => write!(f, "i64"),

            FieldType::U8 => write!(f, "u8"),
            FieldType::U16 => write!(f, "u16"),
            FieldType::U32 => write!(f, "u32"),
            FieldType::U64 => write!(f, "u64")
            ,
            FieldType::F32 => write!(f, "f32"),
            FieldType::F64 => write!(f, "f64"),

            FieldType::Char => write!(f, "char"),
            FieldType::Str => write!(f, "str"),

            FieldType::Array(array_type) => write!(f, "array[{}]", array_type),
            FieldType::Nested(name) => write!(f, "{}", name),

        }
    }
}

pub fn as_token_kind(content: &str) -> Option<TokenKind> {
    use FieldType::*;

    match content {
        //keywords
        "packet" => Some(TokenKind::Keyword(KeywordKind::Packet)),
        "id" => Some(TokenKind::Keyword(KeywordKind::Identifier)),
        "auto" => Some(TokenKind::Keyword(KeywordKind::Auto)),
        "=" => Some(TokenKind::Operator(OperatorKind::Equals)),
        "{" => Some(TokenKind::BraceOpen),
        "}" => Some(TokenKind::BraceClose),
        "fields" => Some(TokenKind::Keyword(KeywordKind::Fields)),
        "array" => Some(TokenKind::Keyword(KeywordKind::Array)),

        //boolean type
        "bool" | "boolean" => Some(TokenKind::Keyword(KeywordKind::FieldDefinition(Bool))),

        //integer types
        "i8" | "byte" => Some(TokenKind::Keyword(KeywordKind::FieldDefinition(I8))),
        "i16" | "short" => Some(TokenKind::Keyword(KeywordKind::FieldDefinition(I16))),
        "i32" | "int" => Some(TokenKind::Keyword(KeywordKind::FieldDefinition(I32))),
        "i64" | "long" => Some(TokenKind::Keyword(KeywordKind::FieldDefinition(I64))),

        //unsigned integer types
        "u8" | "ubyte" => Some(TokenKind::Keyword(KeywordKind::FieldDefinition(U8))),
        "u16" | "ushort" => Some(TokenKind::Keyword(KeywordKind::FieldDefinition(U16))),
        "u32" | "uint" => Some(TokenKind::Keyword(KeywordKind::FieldDefinition(U32))),
        "u64" | "ulong" => Some(TokenKind::Keyword(KeywordKind::FieldDefinition(U64))),

        //float types
        "f32" | "float" => Some(TokenKind::Keyword(KeywordKind::FieldDefinition(F32))),
        "f64" | "double" => Some(TokenKind::Keyword(KeywordKind::FieldDefinition(F64))),

        //chars and strings
        "char" => Some(TokenKind::Keyword(KeywordKind::FieldDefinition(Char))),
        "string" | "str" => Some(TokenKind::Keyword(KeywordKind::FieldDefinition(Str))),

        //numbers and qualifiers
        _ => {
            if let Ok(num) = content.parse::<i32>() {
                Some(TokenKind::Number(num))
            } else {
                Some(TokenKind::Qualifier(content.to_string()))
            }
        }
    }
}
