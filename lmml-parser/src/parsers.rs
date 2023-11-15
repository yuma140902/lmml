use lmml::ast::{LmmlAst, LmmlCommand, NoteChar, NoteModifier};
use nom::{
    branch::alt,
    character::complete::{char, multispace0, none_of, one_of},
    combinator::{eof, map, opt, value},
    error::ParseError,
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated, tuple},
    IResult, Parser,
};

pub fn parse_lmml_until_eof(input: &str) -> IResult<&str, LmmlAst> {
    terminated(parse_lmml, eof)(input)
}

pub fn parse_lmml(input: &str) -> IResult<&str, LmmlAst> {
    map(many0(ws(parse_command)), LmmlAst)(input)
}

pub fn parse_command(input: &str) -> IResult<&str, LmmlCommand> {
    alt((
        parse_note_command,
        parse_rest_command,
        parse_octave_command,
        parse_length_command,
        parse_volume_command,
        parse_tempo_command,
        parse_inc_octave_command,
        parse_dec_octave_command,
    ))(input)
}

pub fn parse_note_command(input: &str) -> IResult<&str, LmmlCommand> {
    map(
        tuple((parse_note_char, opt(parse_modifier), opt(parse_length))),
        |(note, modifier, len)| {
            let length = len.map(|l| l.0);
            let is_dotted = len.map(|l| l.1).unwrap_or(false);
            let modifier = modifier.unwrap_or(NoteModifier::Natural);
            LmmlCommand::Note {
                note,
                modifier,
                length,
                is_dotted,
            }
        },
    )(input)
}

pub fn parse_rest_command(input: &str) -> IResult<&str, LmmlCommand> {
    map(preceded(one_of("Rr"), opt(parse_length)), |len| {
        let length = len.map(|l| l.0);
        let is_dotted = len.map(|l| l.1).unwrap_or(false);
        LmmlCommand::Rest { length, is_dotted }
    })(input)
}

pub fn parse_octave_command(input: &str) -> IResult<&str, LmmlCommand> {
    map(preceded(one_of("Oo"), parse_number), |n| {
        LmmlCommand::SetOctave(n)
    })(input)
}

pub fn parse_length_command(input: &str) -> IResult<&str, LmmlCommand> {
    map(preceded(one_of("Ll"), parse_length), |(n, d)| {
        LmmlCommand::SetLength(n, d)
    })(input)
}

pub fn parse_volume_command(input: &str) -> IResult<&str, LmmlCommand> {
    map(preceded(one_of("Vv"), parse_number), |n| {
        LmmlCommand::SetVolume(n)
    })(input)
}

pub fn parse_tempo_command(input: &str) -> IResult<&str, LmmlCommand> {
    map(preceded(one_of("Tt"), parse_number), |n| {
        LmmlCommand::SetTempo(n)
    })(input)
}

pub fn parse_inc_octave_command(input: &str) -> IResult<&str, LmmlCommand> {
    map(char('>'), |_| LmmlCommand::IncreaseOctave)(input)
}

pub fn parse_dec_octave_command(input: &str) -> IResult<&str, LmmlCommand> {
    map(char('<'), |_| LmmlCommand::DecreaseOctave)(input)
}

pub fn parse_note_char(input: &str) -> IResult<&str, NoteChar> {
    map(one_of("CDEFGABcdefgab"), |c| match c {
        'C' | 'c' => NoteChar::C,
        'D' | 'd' => NoteChar::D,
        'E' | 'e' => NoteChar::E,
        'F' | 'f' => NoteChar::F,
        'G' | 'g' => NoteChar::G,
        'A' | 'a' => NoteChar::A,
        'B' | 'b' => NoteChar::B,
        _ => {
            panic!()
        }
    })(input)
}

pub fn parse_modifier(input: &str) -> IResult<&str, NoteModifier> {
    map(one_of("+-"), |c| match c {
        '+' => NoteModifier::Sharp,
        '-' => NoteModifier::Flat,
        _ => panic!(),
    })(input)
}

pub fn parse_length(input: &str) -> IResult<&str, (u32, bool)> {
    pair(parse_number, parse_dot)(input)
}

pub fn parse_dot(input: &str) -> IResult<&str, bool> {
    map(opt(char('.')), |c| c.is_some())(input)
}

pub fn parse_number(input: &str) -> IResult<&str, u32> {
    map(many1(one_of("0123456789")), |s| {
        s.iter().collect::<String>().parse::<u32>().unwrap()
    })(input)
}

fn comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    value(
        (),
        tuple((char(';'), many0(none_of("\n\r")), one_of("\n\r"))),
    )
    .parse(i)
}

fn ws<'a, F, O, E: ParseError<&'a str>>(inner: F) -> impl Parser<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(
        alt((value((), multispace0), value((), opt(comment)))),
        inner,
        alt((value((), multispace0), value((), opt(comment)))),
    )
}
