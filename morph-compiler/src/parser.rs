use crate::token::{FieldType, OperatorKind, Token, TokenKind};

#[derive(Debug, Clone)]
pub struct Packet {
    pub id: i32,
    pub name: String,
    pub fields: Vec<Field>
}

#[derive(Debug, Clone)]
pub struct Field {
    pub typ: FieldType,
    pub name: String
}

pub struct ParseError {
    pub message: String
}

pub enum ParseResult {
    Ok(Vec<Packet>),
    Err(Vec<ParseError>),
}

pub trait AstParser {
    fn parse(&self, tokens: &Vec<Token>) -> std::io::Result<Vec<Packet>>;

}

pub struct SimpleParser;

impl SimpleParser {

    fn parse_packet(&self, iter: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> std::io::Result<Packet> {

        iter.next();

        //Expect packet name
        let name = match iter.next() {
            Some(token) => match &token.kind {
                TokenKind::Qualifier(s) => s.clone(),
                _ => return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Expected packet name at line: {} column: {}", token.line, token.column)
                ))
            },
            None => return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unexpected end of tokens")
            ))
        };

        // Expect '{'
        match iter.next() {
            Some(token) if matches!(token.kind, TokenKind::BraceOpen) => {},
            Some(token) => return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Expected '{{' at line: {} column: {}", token.line, token.column)
            )),
            None => return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unexpected end of tokens, expected '{{'")
            ))
        }

        // Parse 'id = <number>'
        match iter.next() {
            Some(token) if matches!(token.kind, TokenKind::PacketIdentifier) => {},
            Some(token) => return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Expected 'id' at line: {} column: {}", token.line, token.column)
            )),
            None => return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unexpected end of tokens, expected 'id'")
            ))
        }

        match iter.next() {
            Some(token) if matches!(token.kind, TokenKind::Operator(OperatorKind::Equals)) => {},
            Some(token) => return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Expected '=' at line: {} column: {}", token.line, token.column)
            )),
            None => return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unexpected end of tokens, expected '='")
            ))
        }

        let id = match iter.next() {
            Some(token) => match &token.kind {
                TokenKind::Number(n) => *n,
                _ => return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Expected number for packet id at line: {} column: {}", token.line, token.column)
                ))
            },
            None => return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unexpected end of tokens, expected packet id number")
            ))
        };

        // Parse fields
        let fields = self.parse_fields(iter)?;

        // Expect '}'
        match iter.next() {
            Some(token) if matches!(token.kind, TokenKind::BraceClose) => {},
            Some(token) => return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Expected '}}' at line: {} column: {}", token.line, token.column)
            )),
            None => return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unexpected end of tokens, expected '}}'")
            ))
        }

        Ok(Packet { id, name, fields })
    }

    fn parse_fields(&self, iter: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> std::io::Result<Vec<Field>> {
        // Expect 'fields' keyword
        match iter.next() {
            Some(token) if matches!(token.kind, TokenKind::Fields) => {},
            Some(token) => return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Expected 'fields' at line: {} column: {}", token.line, token.column)
            )),
            None => return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unexpected end of tokens, expected 'fields'")
            ))
        }

        // Expect '{'
        match iter.next() {
            Some(token) if matches!(token.kind, TokenKind::BraceOpen) => {},
            Some(token) => return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Expected '{{' after 'fields' at line: {} column: {}", token.line, token.column)
            )),
            None => return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("Unexpected end of tokens, expected '{{' after 'fields'")
            ))
        }

        let mut fields = Vec::new();

        // Parse field declarations until we hit '}'
        while let Some(token) = iter.peek() {
            match &token.kind {
                TokenKind::BraceClose => {
                    iter.next(); // consume the '}'
                    break;
                },
                TokenKind::FieldType(field_type) => {
                    let typ = field_type.clone();
                    iter.next();

                    let name = match iter.next() {
                        Some(token) => match &token.kind {
                            TokenKind::Qualifier(s) => s.clone(),
                            _ => return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidData,
                                format!("Expected field name at line: {} column: {}", token.line, token.column)
                            ))
                        },
                        None => return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("Unexpected end of tokens, expected field name")
                        ))
                    };

                    fields.push(Field { typ, name });
                },
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        format!("Unexpected token in fields block: {:?} at line: {} column: {}", token.kind, token.line, token.column)
                    ))
                }
            }
        }

        Ok(fields)
    }
}

impl AstParser for SimpleParser {
    fn parse(&self, tokens: &Vec<Token>) -> std::io::Result<Vec<Packet>> {
        let mut packets = Vec::new();
        let mut iter = tokens.iter().peekable();

        while let Some(token) = iter.peek() {
            match &token.kind {
                TokenKind::PacketKeyword => {
                    let packet = self.parse_packet(&mut iter)?;
                    packets.push(packet);
                },
                _ => return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("Expected 'packet' at line: {} column: {}", token.line, token.column)
                ))
            }
        }

        Ok(packets)
    }
}