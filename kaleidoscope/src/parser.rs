use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::{
        complete::{alpha1, char},
        is_alphanumeric,
    },
    combinator::{all_consuming, cut, map, not, opt, peek, recognize},
    error::{context, ParseError},
    multi::{many1, separated_list},
    number::complete::double,
    sequence::{pair, preceded, terminated, tuple},
    IResult,
};

use crate::ast::{Expr, Program, Prototype, Statement};

const WS_CHARS: &str = " \n";
const STATEMENT_SEPERATOR_CHARS: &str = ";\n";

fn ws1(input: &str) -> IResult<&str, &str> {
    take_while1(move |c| WS_CHARS.contains(c))(input)
}

fn ws(input: &str) -> IResult<&str, &str> {
    take_while(move |c| WS_CHARS.contains(c))(input)
}

fn statement_seperator(input: &str) -> IResult<&str, &str> {
    take_while1(move |c| STATEMENT_SEPERATOR_CHARS.contains(c))(input)
}

fn list<I, O1, O2, O3, E: ParseError<I>, S, F, W>(
    item: F,
    sep: S,
    start: S,
    end: S,
    sp: W,
) -> impl Fn(I) -> IResult<I, Vec<O1>, E>
where
    I: Clone + PartialEq,
    F: Fn(I) -> IResult<I, O1, E>,
    S: Fn(I) -> IResult<I, O2, E>,
    W: Fn(I) -> IResult<I, O3, E>,
{
    move |input: I| {
        let (input, _) = start(input)?;
        let (input, args) = separated_list(
            preceded(|x| sp(x), |x| sep(x)),
            preceded(|x| sp(x), |x| item(x)),
        )(input)?;
        preceded(|x| sp(x), |x| end(x))(input).map(|(i, _)| (i, args))
    }
}

fn identifier(input: &str) -> IResult<&str, &str> {
    let (input, _) = not(tag("def"))(input)?;
    let (input, _) = not(tag("extern"))(input)?;
    let end = || take_while(move |c| is_alphanumeric(c as u8) || c == '_');
    context(
        "identifier",
        recognize(alt((
            preceded(char('_'), preceded(alpha1, end())),
            preceded(alpha1, end()),
        ))),
    )(input)
}

fn number(input: &str) -> IResult<&str, Expr> {
    context("number", map(double, Expr::Number))(input)
}

fn variable(input: &str) -> IResult<&str, Expr> {
    context("variable", map(identifier, Expr::Variable))(input)
}

fn arguments(input: &str) -> IResult<&str, Vec<Expr>> {
    context(
        "arguments",
        preceded(
            peek(char('(')),
            cut(list(expr, char(','), char('('), char(')'), ws)),
        ),
    )(input)
}

fn call(input: &str) -> IResult<&str, Expr> {
    context(
        "call",
        map(tuple((identifier, arguments)), |(name, args)| {
            Expr::Call(name, args)
        }),
    )(input)
}

fn expr(input: &str) -> IResult<&str, Expr> {
    context("expression", alt((call, number, variable)))(input)
}

fn prototype(input: &str) -> IResult<&str, Prototype> {
    context(
        "prototype",
        map(
            pair(
                identifier,
                list(identifier, char(','), char('('), char(')'), ws),
            ),
            |(name, args)| Prototype(name, args),
        ),
    )(input)
}

fn external(input: &str) -> IResult<&str, Statement> {
    context(
        "extern",
        map(
            preceded(tag("extern"), cut(preceded(ws1, prototype))),
            Statement::Extern,
        ),
    )(input)
}

fn function(input: &str) -> IResult<&str, Statement> {
    context(
        "function",
        map(
            preceded(
                tag("def"),
                cut(pair(preceded(ws1, prototype), preceded(ws1, expr))),
            ),
            |(proto, body)| Statement::Function(proto, body),
        ),
    )(input)
}

fn statement(input: &str) -> IResult<&str, Statement> {
    context(
        "statement",
        alt((external, function, map(expr, Statement::Expression))),
    )(input)
}

fn statements(input: &str) -> IResult<&str, Vec<Statement>> {
    context(
        "one or more statements",
        separated_list(many1(statement_seperator), statement),
    )(input)
}

pub fn parse(input: &str) -> IResult<&str, Program> {
    map(
        all_consuming(preceded(
            ws,
            terminated(statements, opt(alt((ws, statement_seperator)))),
        )),
        Program,
    )(input)
}

#[cfg(test)]
#[path = "./tests/parser_test.rs"]
mod parser_test;
