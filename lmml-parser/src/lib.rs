#![deny(rust_2018_idioms)]
#![deny(clippy::all)]
#![deny(clippy::nursery)]

use lmml::ast::LmmlAst;
use nom::IResult;

mod parsers;

pub fn remove_comments(input: &str) -> String {
    let mut v = Vec::new();
    for line in input.lines() {
        if line.trim_start().starts_with(';') {
            continue;
        }
        v.push(line);
    }
    v.join("\n")
}

pub fn parse_lmml(input: &str) -> IResult<&str, LmmlAst> {
    parsers::parse_lmml_until_eof(input)
}
