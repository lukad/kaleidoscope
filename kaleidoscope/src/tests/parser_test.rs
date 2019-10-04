use super::{parse, KaleidoscopeParser, Rule};
use pest::{consumes_to, fails_with, parses_to};

#[test]
fn parses_identifier_with_leading_underscore() {
    parses_to!(
        parser: KaleidoscopeParser,
        input: "_foo_bar1_",
        rule: Rule::identifier,
        tokens: [identifier(0, 10)]
    );
}

#[test]
fn parses_identifier_in_allcaps() {
    parses_to!(
        parser: KaleidoscopeParser,
        input: "FOOBAR",
        rule: Rule::identifier,
        tokens: [identifier(0, 6)]
    );
}

#[test]
fn fails_to_parse_underscore_only_identifier() {
    fails_with!(
        parser: KaleidoscopeParser,
        input: "_",
        rule: Rule::identifier,
        positives: [Rule::identifier],
        negatives: [],
        pos: 0
    );
}

#[test]
fn fails_to_parse_identifier_with_leading_digit() {
    fails_with!(
        parser: KaleidoscopeParser,
        input: "1foo",
        rule: Rule::identifier,
        positives: [Rule::identifier],
        negatives: [],
        pos: 0
    );
}

#[test]
fn parses_integer_number() {
    parses_to!(
        parser: KaleidoscopeParser,
        input: "12345",
        rule: Rule::number,
        tokens: [number(0, 5)]
    );
}

#[test]
fn parses_float() {
    parses_to!(
        parser: KaleidoscopeParser,
        input: "0.1234",
        rule: Rule::number,
        tokens: [number(0, 6)]
    );

    parses_to!(
        parser: KaleidoscopeParser,
        input: "0.0001",
        rule: Rule::number,
        tokens: [number(0, 6)]
    );
}

#[test]
fn parses_negative_number() {
    parses_to!(
        parser: KaleidoscopeParser,
        input: "-10",
        rule: Rule::number,
        tokens: [number(0, 3)]
    )
}

#[test]
fn parses_negative_float() {
    parses_to!(
        parser: KaleidoscopeParser,
        input: "-10.0",
        rule: Rule::number,
        tokens: [number(0, 5)]
    );

    parses_to!(
        parser: KaleidoscopeParser,
        input: "-0.0",
        rule: Rule::number,
        tokens: [number(0, 4)]
    )
}
#[test]
fn parses_sci_notation_float() {
    parses_to!(
        parser: KaleidoscopeParser,
        input: "1.0e-10",
        rule: Rule::number,
        tokens: [number(0, 7)]
    );

    parses_to!(
        parser: KaleidoscopeParser,
        input: "1e0",
        rule: Rule::number,
        tokens: [number(0, 3)]
    );

    parses_to!(
        parser: KaleidoscopeParser,
        input: "0.1e13",
        rule: Rule::number,
        tokens: [number(0, 6)]
    )
}

#[test]
fn parses_extern_definition_without_args() {
    parses_to!(
        parser: KaleidoscopeParser,
        input: "extern foo()",
        rule: Rule::external,
        tokens: [
            external(0, 12, [
                prototype(7, 12, [
                    identifier(7, 10),
                ])
            ])
        ]
    );
}

#[test]
fn parses_extern_definition_with_args() {
    parses_to!(
        parser: KaleidoscopeParser,
        input: "extern foo(foo, bar)",
        rule: Rule::external,
        tokens: [
            external(0, 20, [
                prototype(7, 20, [
                    identifier(7, 10),
                    identifier(11, 14),
                    identifier(16, 19),
                ])
            ])
        ]
    );
}

#[test]
fn parses_call_without_args() {
    parses_to!(
        parser: KaleidoscopeParser,
        input: "foo()",
        rule: Rule::call,
        tokens: [
            call(0, 5, [
                identifier(0, 3),
            ])
        ]
    )
}

#[test]
fn parses_call_with_args() {
    parses_to!(
        parser: KaleidoscopeParser,
        input: "foo(0.5, baz)",
        rule: Rule::call,
        tokens: [
            call(0, 13, [
                identifier(0, 3),
                expr(4, 7, [number(4, 7)]),
                expr(9, 12, [identifier(9, 12)]),
            ])
        ]
    )
}

#[test]
fn parses_def() {
    parses_to!(
        parser: KaleidoscopeParser,
        input: "def foo(bar, _baz) bar",
        rule: Rule::function,
        tokens: [
            function(0, 22, [
                prototype(4, 18, [
                    identifier(4, 7),
                    identifier(8, 11),
                    identifier(13, 17),
                ]),
                expr(19, 22, [identifier(19, 22)]),
            ])
        ]
    );
}

#[test]
fn parses_an_example_program() {
    use crate::ast::{Expr::*, Program, Prototype, Statement::*};

    assert_eq!(
        parse(include_str!("./print_double.kl")).unwrap(),
        Program(vec![
            Extern(Prototype(
                "mul".to_string(),
                vec!["a".to_string(), "b".to_string()]
            )),
            Extern(Prototype("print".to_string(), vec!["x".to_string()])),
            Function(
                Prototype("double".to_string(), vec!["x".to_string()]),
                Call(
                    "mul".to_string(),
                    vec![Variable("x".to_string()), Number(2.0)]
                )
            ),
            Expression(Call(
                "print".to_string(),
                vec![Call("double".to_string(), vec![Number(32.0)])]
            ),)
        ])
    )
}
