use std::fmt::{Debug, Display};

pub trait MorphError: Debug + Display {
    fn message(&self) -> String;

}

pub enum MorphResult<T> {
    Success(T),
    Errors(Vec<Box<dyn MorphError>>),
}

pub fn print_morph_errors(errors: &Vec<Box<dyn MorphError>>) {
    for err in errors {
        println!("{}", err.message());
    }
}