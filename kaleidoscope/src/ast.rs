#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    Variable(String),
    Infix(Box<Expr>, char, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Debug, PartialEq)]
pub struct Prototype(pub String, pub Vec<String>);

#[derive(Debug, PartialEq)]
pub enum Statement {
    Function(Prototype, Expr),
    Extern(Prototype),
    Expression(Expr),
}

#[derive(Debug, PartialEq)]
pub struct Program(pub Vec<Statement>);
