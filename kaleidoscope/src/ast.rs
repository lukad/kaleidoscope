#[derive(Debug, PartialEq)]
pub enum Expr<'a> {
    Number(f64),
    Variable(&'a str),
    Infix(Box<Expr<'a>>, char, Box<Expr<'a>>),
    Call(&'a str, Vec<Expr<'a>>),
}

#[derive(Debug, PartialEq)]
pub struct Prototype<'a>(pub &'a str, pub Vec<&'a str>);

#[derive(Debug, PartialEq)]
pub enum Statement<'a> {
    Function(Prototype<'a>, Expr<'a>),
    Extern(Prototype<'a>),
    Expression(Expr<'a>),
}

#[derive(Debug, PartialEq)]
pub struct Program<'a>(pub Vec<Statement<'a>>);
