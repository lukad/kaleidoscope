use super::{call, external, function, identifier, number, parse};
use crate::ast::{Expr::*, Program, Prototype, Statement::*};
use nom::{error::ErrorKind, error_position, Err};

#[test]
fn parses_identifier_with_leading_underscore() {
    assert_eq!(identifier("_foo_bar1_"), Ok(("", "_foo_bar1_")))
}

#[test]
fn parses_identifier_in_allcaps() {
    assert_eq!(identifier("FOOBAR"), Ok(("", "FOOBAR")))
}

#[test]
fn fails_to_parse_underscore_only_identifier() {
    assert_eq!(
        identifier("_"),
        Err(Err::Error(error_position!("_", ErrorKind::Alpha)))
    )
}

#[test]
fn fails_to_parse_identifier_with_leading_digit() {
    assert_eq!(
        identifier("1foo"),
        Err(Err::Error(error_position!("1foo", ErrorKind::Alpha)))
    )
}

#[test]
fn parses_integer_number() {
    assert_eq!(number("12345"), Ok(("", Number(12345.0))))
}

#[test]
fn parses_float() {
    assert_eq!(number("0.1234"), Ok(("", Number(0.1234))));
    assert_eq!(number("0.0001"), Ok(("", Number(0.0001))))
}

#[test]
fn parses_negative_number() {
    assert_eq!(number("-10"), Ok(("", Number(-10.0))))
}

#[test]
fn parses_negative_float() {
    assert_eq!(number("-10.0"), Ok(("", Number(-10.0))));
    assert_eq!(number("-0.0"), Ok(("", Number(-0.0))))
}

#[test]
fn parses_sci_notation_float() {
    assert_eq!(number("1.0e-10"), Ok(("", Number(1.0e-10))));
    assert_eq!(number("1e0"), Ok(("", Number(1e0))));
    assert_eq!(number("0.1e13"), Ok(("", Number(0.1e13))))
}

#[test]
fn parses_extern_definition_without_args() {
    assert_eq!(
        external("extern foo()"),
        Ok(("", Extern(Prototype("foo", vec![]))))
    )
}

#[test]
fn parses_extern_definition_with_args() {
    assert_eq!(
        external("extern foo(bar, baz)"),
        Ok(("", Extern(Prototype("foo", vec!["bar", "baz"]))))
    )
}

#[test]
fn parses_call_without_args() {
    assert_eq!(call("foo()"), Ok(("", Call("foo", vec![]))))
}

#[test]
fn parses_call_with_args() {
    assert_eq!(
        call("foo(0.5, bar)"),
        Ok(("", Call("foo", vec![Number(0.5), Variable("bar")])))
    )
}

#[test]
fn parses_def() {
    assert_eq!(
        function("def foo(bar, baz) bar"),
        Ok((
            "",
            Function(Prototype("foo", vec!["bar", "baz"]), Variable("bar"))
        ))
    )
}

#[test]
fn parses_an_example_program() {
    assert_eq!(
        parse(include_str!("./print_double.kl")).unwrap(),
        (
            "",
            Program(vec![
                Extern(Prototype("mul", vec!["a", "b"])),
                Extern(Prototype("print", vec!["x"])),
                Function(
                    Prototype("double", vec!["x"]),
                    Call("mul", vec![Variable("x"), Number(2.0)])
                ),
                Expression(Call("print", vec![Call("double", vec![Number(32.0)])]),)
            ])
        )
    )
}
