use std::fmt::{Debug, Display};

pub trait MorphError: Debug + Display {
    fn message(&self) -> String;

}

pub enum MorphResult<T> {
    Success(T),
    Errors(Vec<Box<dyn MorphError>>),
}

pub fn print_morph_errors(errors: &Vec<Box<dyn MorphError>>, indent: &str) {
    for err in errors {
        eprintln!("{}-> {}", indent, err.message());
    }
}