#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    PacketKeyword,
    Identifier(String),
    BraceOpen,
    BraceClose,
    Type(String),
}

pub fn as_token_kind(content: &str) -> Option<TokenKind> {

    match content {
        "packet" => Some(TokenKind::PacketKeyword),
        "{" => Some(TokenKind::BraceOpen),
        "}" => Some(TokenKind::BraceClose),
        "string" | "int" => Some(TokenKind::Type(content.to_string())),
        _ => Some(TokenKind::Identifier(content.to_string())),
    }

}