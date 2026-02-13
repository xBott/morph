#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    PacketKeyword,
    PacketIdentifier,
    Operator(OperatorKind),
    BraceOpen,
    BraceClose,
    Fields,
    FieldType(FieldType),
    Qualifier(String),
    Number(i32)
}

#[derive(Debug, Clone)]
pub enum OperatorKind {
    Equals,
}

#[derive(Debug, Clone)]
pub enum FieldType {
    Bool,
    I32,
    Str,
}

pub fn as_token_kind(content: &str) -> Option<TokenKind> {
    match content {
        "packet" => Some(TokenKind::PacketKeyword),
        "id" => Some(TokenKind::PacketIdentifier),
        "=" => Some(TokenKind::Operator(OperatorKind::Equals)),
        "{" => Some(TokenKind::BraceOpen),
        "}" => Some(TokenKind::BraceClose),
        "fields" => Some(TokenKind::Fields),
        "bool" => Some(TokenKind::FieldType(FieldType::Bool)),
        "i32" | "int" => Some(TokenKind::FieldType(FieldType::I32)),
        "string" => Some(TokenKind::FieldType(FieldType::Str)),
        _ => {
            if let Ok(num) = content.parse::<i32>() {
                Some(TokenKind::Number(num))

            } else {
                Some(TokenKind::Qualifier(content.to_string()))
            }
        }
    }
}
