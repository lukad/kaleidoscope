use pest::{error::Error, iterators::Pair, Parser};
use pest_derive::Parser;

use crate::ast::{Expr::*, Statement::*, *};

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct KaleidoscopeParser;

fn parse_expr(pair: Pair<Rule>) -> Expr {
    let p = pair.into_inner().nth(0).unwrap();
    match p.as_rule() {
        Rule::number => Number(p.as_str().parse().unwrap()),
        Rule::identifier => Variable(p.as_str().to_string()),
        Rule::call => {
            let mut inner = p.into_inner();
            let name = inner.next().unwrap().as_str().to_string();
            let args = inner.map(parse_expr).collect();
            Call(name, args)
        }
        _ => unreachable!(),
    }
}

fn parse_prototype(pair: Pair<Rule>) -> Prototype {
    let mut proto_inner = pair.into_inner();
    let name = proto_inner.next().unwrap().as_str().to_string();
    let args = proto_inner.map(|pair| pair.as_str().to_string()).collect();
    Prototype(name, args)
}

fn parse_statement(pair: Pair<Rule>) -> Option<Statement> {
    match pair.as_rule() {
        Rule::function => {
            let mut inner = pair.into_inner();
            let prototype = parse_prototype(inner.next().unwrap());
            let expr = parse_expr(inner.next().unwrap());
            Some(Function(prototype, expr))
        }
        Rule::external => {
            let mut inner = pair.into_inner();
            let prototype = parse_prototype(inner.next().unwrap());
            Some(Extern(prototype))
        }
        Rule::expr => Some(Expression(parse_expr(pair))),
        Rule::EOI => None,
        _ => unreachable!(),
    }
}

pub fn parse(input: &str) -> Result<Program, Error<Rule>> {
    let pair: Pair<Rule> = KaleidoscopeParser::parse(Rule::program, input)?
        .next()
        .unwrap();

    let statements = pair.into_inner().map(parse_statement).flatten().collect();

    Ok(Program(statements))
}

#[cfg(test)]
#[path = "./tests/parser_test.rs"]
mod parser_test;
