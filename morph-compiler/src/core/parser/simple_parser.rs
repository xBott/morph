use crate::core::{AstParser, Field, FieldType, OperatorKind, Packet, ParserError, Token, TokenKind};
use crate::core::FieldType::{Array, Nested};
use crate::core::token::KeywordKind;
use crate::core::token::KeywordKind::{Fields, Identifier};
use crate::core::TokenKind::{Keyword, Qualifier, BraceClose};
use crate::utils::MorphResult::{Errors, Success};
use crate::utils::{MorphError, MorphResult};

pub struct SimpleParser;

impl SimpleParser {

    fn parse_packet(&self, iter: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> MorphResult<Packet> {

        let mut all_errors: Vec<Box<dyn MorphError>> = Vec::new();

        iter.next();

        let id: i32;
        let is_auto: bool;
        let name: String;
        let mut fields: Vec<Field> = Vec::new();

        match self.parse_packet_qualifier(iter) {
            Success(value) => name = value,
            Errors(errors) => {
                name = "not found".to_string();
                all_errors.extend(errors)
            },
        }

        match self.parse_brace_open(iter) {
            Errors(errors) => all_errors.extend(errors),
            _ => {}
        }

        match self.parse_packet_id_keyword(iter) {
            Errors(errors) => all_errors.extend(errors),
            _ => {}
        }

        match self.parse_operator(iter, OperatorKind::Equals) {
            Errors(errors) => all_errors.extend(errors),
            _ => {}
        }
        
        if self.has_keyword(iter, KeywordKind::Auto) {
            id = -1;
            is_auto = true;
            match self.parse_auto_keyword(iter) {
                Errors(errors) => all_errors.extend(errors),
                _ => {}
            }

        } else {
            is_auto = false;
            match self.parse_i32_number(iter) {
                Success(num) => {
                    id = num;
                },
                Errors(errors) => {
                    id = -1;
                    all_errors.extend(errors)
                },
            }

        }

        if self.has_keyword(iter, Fields) {

            iter.next();

            match self.parse_brace_open(iter) {
                Errors(errors) => all_errors.extend(errors),
                _ => {}
            }

            if !self.has_token(iter, BraceClose) {
                match self.parse_fields(iter) {
                    Success(parsed_fields) => fields.extend(parsed_fields),
                    Errors(errors) => all_errors.extend(errors),
                }
            }

            match self.parse_brace_close(iter) {
                Errors(errors) => all_errors.extend(errors),
                _ => {}
            }
        }

        match self.parse_brace_close(iter) {
            Errors(errors) => all_errors.extend(errors),
            _ => {}
        }

        if all_errors.is_empty() {
            Success(Packet { id, is_auto, name, fields })
        } else {
            Errors(all_errors)
        }

    }

    fn has_token(
        &self, iter:
        &mut std::iter::Peekable<std::slice::Iter<Token>>,
        expected: TokenKind
    ) -> bool {
        match iter.peek() {
            Some(token) => {
                if (*token).kind == expected {
                    iter.next();
                    true
                } else {
                    false
                }
            }
            None => false
        }
    }

    fn has_keyword(
        &self, iter:
        &mut std::iter::Peekable<std::slice::Iter<Token>>,
        expected: KeywordKind
    ) -> bool {
        match iter.peek() {
            Some(token) => {
                if let Keyword(keyword_kind) = &token.kind {
                    expected == *keyword_kind

                } else {
                    false

                }
            }
            None => false
        }
    }

    fn expect<F, R>(
        &self,
        iter: &mut std::iter::Peekable<std::slice::Iter<Token>>,
        f: F
    ) -> MorphResult<R>
    where
        F: FnOnce(&Token) -> MorphResult<R>,
    {
        match iter.next() {
            Some(token) => { f(token) }
            None => {
                let err = ParserError {
                    message: "Unexpected end of tokens".to_string(),
                    token: None
                };
                Errors(vec![Box::new(err)])
            }
        }
    }

    fn expect_kind(
        &self,
        iter: &mut std::iter::Peekable<std::slice::Iter<Token>>,
        expected: TokenKind
    ) -> MorphResult<()> {

        self.expect(iter, |token| {

            if expected == token.kind {
                Success(())

            } else {
                let err = ParserError {
                    message: format!("Expected '{}', but got '{}'", expected, token.kind),
                    token: Some(token.clone())
                };
                Errors(vec![Box::new(err)])
            }

        })
    }

    fn parse_packet_qualifier(
        &self,
        iter: &mut std::iter::Peekable<std::slice::Iter<Token>>
    ) -> MorphResult<String> {

        self.expect(iter, |token| {
            if let Qualifier(name) = &token.kind {
                Success(name.clone())

            } else {
                let err = ParserError {
                    message: format!("Expected packet qualifier, but got '{}'", token.kind),
                    token: Some(token.clone())
                };
                Errors(vec![Box::new(err)])
            }
        })

    }

    fn parse_brace_open(
        &self, iter:
        &mut std::iter::Peekable<std::slice::Iter<Token>>
    ) -> MorphResult<()> {
        self.expect_kind(iter, TokenKind::BraceOpen)
    }

    fn parse_brace_close(
        &self, iter:
        &mut std::iter::Peekable<std::slice::Iter<Token>>
    ) -> MorphResult<()> {
        self.expect_kind(iter, BraceClose)
    }

    fn parse_keyword(
        &self, iter:
        &mut std::iter::Peekable<std::slice::Iter<Token>>,
        expected: KeywordKind
    ) -> MorphResult<()> {

        self.expect(iter, |token| {
            match &token.kind {
                Keyword(keyword_kind) => {
                    if keyword_kind.clone() == expected {

                        Success(())

                    } else {
                        let err = ParserError {
                            message: format!("Expected keyword '{}', but got another keyword '{}", expected, keyword_kind),
                            token: Some(token.clone())
                        };

                        Errors(vec![Box::new(err)])
                    }
                }
                _ => {
                    let err = ParserError {
                        message: format!("Expected keyword '{}', but got '{}", expected, token.kind),
                        token: Some(token.clone())
                    };

                    Errors(vec![Box::new(err)])
                }
            }
        })

    }

    fn parse_packet_id_keyword(&self, iter: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> MorphResult<()> {
        self.parse_keyword(iter, Identifier)
    }

    fn parse_operator(
        &self,
        iter: &mut std::iter::Peekable<std::slice::Iter<Token>>,
        expected: OperatorKind
    ) -> MorphResult<()> {

        self.expect(iter, |token| {
            if let TokenKind::Operator(kind) = &token.kind {
                if expected == *kind {
                    Success(())
                } else {
                    let err = ParserError {
                        message: format!("Expected operator '{}', but got operator '{}'", expected, kind),
                        token: Some(token.clone())
                    };
                    Errors(vec![Box::new(err)])
                }

            } else {
                let err = ParserError {
                    message: format!("Expected operator '{}', but got '{}'", expected, token.kind),
                    token: Some(token.clone())
                };
                Errors(vec![Box::new(err)])
            }
        })

    }

    fn parse_auto_keyword(&self, iter: &mut std::iter::Peekable<std::slice::Iter<Token>>) -> MorphResult<()> {
        self.parse_keyword(iter, KeywordKind::Auto)
    }

    fn parse_i32_number(
        &self,
        iter: &mut std::iter::Peekable<std::slice::Iter<Token>>
    ) -> MorphResult<i32> {
        self.expect(iter, |token| {
            if let TokenKind::Number(num) = &token.kind {
                Success(*num)
            } else {
                let err = ParserError {
                    message: format!("Expected i32 number, but got '{}'", token.kind),
                    token: Some(token.clone())
                };
                Errors(vec![Box::new(err)])
            }
        })
    }

    fn parse_fields(
        &self,
        iter: &mut std::iter::Peekable<std::slice::Iter<Token>>
    ) -> MorphResult<Vec<Field>> {

        let mut all_errors: Vec<Box<dyn MorphError>> = Vec::new();
        let mut fields: Vec<Field> = Vec::new();

        while let Some(token) = iter.peek() {

            match &token.kind {

                Keyword(keyword_kind) => {

                    match &keyword_kind {

                        KeywordKind::FieldDefinition(field_type) => {
                            iter.next();
                            match self.parse_simple_field(iter, &field_type) {
                                Success(field) => fields.push(field),
                                Errors(errors) => { all_errors.extend(errors) }
                            }
                        }

                        KeywordKind::Array => {
                            iter.next();
                            match self.parse_array_field(iter) {
                                Success(field) => fields.push(field),
                                Errors(errors) => { all_errors.extend(errors) }
                            }
                        }

                        _ => {}

                    }

                }

                Qualifier(packet_name) => {

                    iter.next();
                    let field_type = Nested(packet_name.clone());
                    match self.parse_simple_field(iter, &field_type) {
                        Success(field) => fields.push(field),
                        Errors(errors) => { all_errors.extend(errors) }
                    }

                }
                BraceClose => {
                    break;
                }
                _ => {
                    let err = ParserError {
                        message: format!("Expected field type or array keyword, but got '{}'", token.kind),
                        token: Some((*token).clone())
                    };
                    all_errors.push(Box::new(err));
                    break;
                }
            }

        }

        if all_errors.is_empty() {
            Success(fields)

        } else {
            Errors(all_errors)

        }
    }

    fn parse_simple_field(
        &self,
        iter: &mut std::iter::Peekable<std::slice::Iter<Token>>,
        field_type: &FieldType
    ) -> MorphResult<Field> {

        self.expect(iter, |token| {

            match &token.kind {
                Qualifier(name) => {
                    let field = Field {
                        typ: field_type.clone(),
                        name: name.clone()
                    };
                    Success(field)
                }
                _ => {
                    let err = ParserError {
                        message: format!("Expected field name, but got '{}'", token.kind),
                        token: Some(token.clone())
                    };
                    Errors(vec![Box::new(err)])
                }
            }

        })

    }

    fn parse_array_field(
        &self,
        iter: &mut std::iter::Peekable<std::slice::Iter<Token>>
    ) -> MorphResult<Field> {

        let array_type = match self.expect(iter, |token| {

            match &token.kind {
                Keyword(keyword_kind) => {
                    match keyword_kind {
                        KeywordKind::FieldDefinition(field_type) => {
                            Success(Array(Box::new(field_type.clone())))
                        }
                        KeywordKind::Array => {
                            let err = ParserError {
                                message: "Nested arrays are not allowed".to_string(),
                                token: Some(token.clone())
                            };
                            Errors(vec![Box::new(err)])
                        }
                        _ => {
                            let err = ParserError {
                                message: format!("Expected field type or array keyword field type, but got '{}'", token.kind),
                                token: Some(token.clone())
                            };
                            Errors(vec![Box::new(err)])
                        }
                    }
                }

                Qualifier(packet_name) => {
                    Success(Nested(packet_name.clone()))

                }
                _ => {
                    let err = ParserError {
                        message: format!("Expected array field type, but got '{}'", token.kind),
                        token: Some(token.clone())
                    };
                    Errors(vec![Box::new(err)])
                }
            }

        }) {
            Success(field_type) => Array(Box::new(field_type.clone())),
            Errors(errors) => {
                return Errors(errors);
            }
        };

        self.parse_simple_field(iter, &array_type)

    }

}

impl AstParser for SimpleParser {

    fn parse(&self, tokens: &Vec<Token>) -> MorphResult<Vec<Packet>> {

        let mut packets: Vec<Packet> = Vec::new();
        let mut all_errors: Vec<Box<dyn MorphError>> = Vec::new();

        let mut iter = tokens.iter().peekable();

        while let Some(token) = iter.peek() {
            match &token.kind {
                Keyword(keyword_kind) => {
                    match keyword_kind {

                        KeywordKind::Packet => {
                            match self.parse_packet(&mut iter) {
                                Success(packet) => packets.push(packet),
                                Errors(errors) => all_errors.extend(errors)
                            }
                        }

                        _ => {
                            let err = ParserError {
                                message: format!("Expected keyword 'packet', but got keyword '{}'", &token.kind),
                                token: Some((*token).clone())
                            };
                            all_errors.push(Box::new(err));
                            iter.next();
                        }

                    }

                },
                _ => {
                    let err = ParserError {
                        message: format!("Expected keyword 'packet', but got '{}'", &token.kind),
                        token: Some((*token).clone())
                    };
                    all_errors.push(Box::new(err));
                    iter.next();
                }
            }
        }

        if all_errors.is_empty() {
            Success(packets)
        } else {
            Errors(all_errors)
        }
    }

}