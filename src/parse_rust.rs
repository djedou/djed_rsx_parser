
use combine::{ParseResult, Parser, Stream};
use combine::combinator::{between, many, none_of, parser, token, r#try};

use parse_js::{js_comment, js_whitespace};
use parse_misc::escaped_character;
use parse_rust_types::{RSChar, RSString};

pub fn rs_char<I>(input: I) -> ParseResult<RSChar, I>
where
    I: Stream<Item = char>
{
    between(token('\''), token('\''), choice!(r#try(parser(escaped_character)), none_of("'".chars())))
        .map(RSChar)
        .parse_stream(input)
}

pub fn rs_string<I>(input: I) -> ParseResult<RSString, I>
where
    I: Stream<Item = char>
{
    between(
        token('"'),
        token('"'),
        many(choice!(r#try(parser(escaped_character)), none_of("\"".chars())))
    ).map(RSString)
        .parse_stream(input)
}

pub fn rs_comment<I>(input: I) -> ParseResult<(), I>
where
    I: Stream<Item = char>
{
    parser(js_comment).parse_stream(input)
}

pub fn rs_whitespace<I>(input: I) -> ParseResult<(), I>
where
    I: Stream<Item = char>
{
    parser(js_whitespace).parse_stream(input)
}
