use lmml::ast::{LmmlAst, LmmlCommand, NoteChar, NoteModifier};
use nom::{
    branch::alt,
    character::complete::{char, multispace0, none_of, one_of},
    combinator::{eof, map, opt, value},
    error::ParseError,
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, terminated},
    IResult, Parser,
};
use nom_language::error::VerboseError;

pub fn parse_lmml_until_eof(input: &str) -> IResult<&str, LmmlAst, VerboseError<&str>> {
    terminated(parse_lmml, eof).parse(input)
}

pub fn parse_lmml(input: &str) -> IResult<&str, LmmlAst, VerboseError<&str>> {
    map(
        many0(delimited(
            alt((value((), multispace0), value((), opt(comment)))),
            parse_command,
            alt((value((), multispace0), value((), opt(comment)))),
        )),
        LmmlAst,
    )
    .parse(input)
}

pub fn parse_command(input: &str) -> IResult<&str, LmmlCommand, VerboseError<&str>> {
    alt((
        parse_note_command,
        parse_rest_command,
        parse_chord_command,
        parse_n_command,
        parse_octave_command,
        parse_length_command,
        parse_volume_command,
        parse_tempo_command,
        parse_waveform_command,
        parse_channel_command,
        parse_inc_octave_command,
        parse_dec_octave_command,
    ))
    .parse(input)
}

pub fn parse_note_command(input: &str) -> IResult<&str, LmmlCommand, VerboseError<&str>> {
    map(
        (
            parse_note_char,
            opt(parse_modifier),
            opt(parse_number),
            parse_dot,
        ),
        |(note, modifier, length, is_dotted)| {
            let modifier = modifier.unwrap_or(NoteModifier::Natural);
            LmmlCommand::Note {
                note,
                modifier,
                length,
                is_dotted,
            }
        },
    )
    .parse(input)
}

pub fn parse_chord_command(input: &str) -> IResult<&str, LmmlCommand, VerboseError<&str>> {
    map(
        (
            delimited(
                char('['),
                many1(pair(parse_note_char, opt(parse_modifier))),
                char(']'),
            ),
            opt(parse_number),
            parse_dot,
        ),
        |(notes, length, is_dotted)| LmmlCommand::Chord {
            notes: notes
                .into_iter()
                .map(|(n, m)| (n, m.unwrap_or(NoteModifier::Natural)))
                .collect(),
            length,
            is_dotted,
        },
    )
    .parse(input)
}

pub fn parse_rest_command(input: &str) -> IResult<&str, LmmlCommand, VerboseError<&str>> {
    map(
        preceded(one_of("Rr"), pair(opt(parse_number), parse_dot)),
        |(length, is_dotted)| LmmlCommand::Rest { length, is_dotted },
    )
    .parse(input)
}

pub fn parse_n_command(input: &str) -> IResult<&str, LmmlCommand, VerboseError<&str>> {
    map(preceded(one_of("Nn"), parse_number), |n| {
        LmmlCommand::NoteNumber(n)
    })
    .parse(input)
}

pub fn parse_octave_command(input: &str) -> IResult<&str, LmmlCommand, VerboseError<&str>> {
    map(preceded(one_of("Oo"), parse_number), |n| {
        LmmlCommand::SetOctave(n)
    })
    .parse(input)
}

pub fn parse_length_command(input: &str) -> IResult<&str, LmmlCommand, VerboseError<&str>> {
    map(
        preceded(one_of("Ll"), pair(parse_number, parse_dot)),
        |(n, d)| LmmlCommand::SetLength(n, d),
    )
    .parse(input)
}

pub fn parse_volume_command(input: &str) -> IResult<&str, LmmlCommand, VerboseError<&str>> {
    map(preceded(one_of("Vv"), parse_number), |n| {
        LmmlCommand::SetVolume(n)
    })
    .parse(input)
}

pub fn parse_tempo_command(input: &str) -> IResult<&str, LmmlCommand, VerboseError<&str>> {
    map(preceded(one_of("Tt"), parse_number), |n| {
        LmmlCommand::SetTempo(n)
    })
    .parse(input)
}

pub fn parse_waveform_command(input: &str) -> IResult<&str, LmmlCommand, VerboseError<&str>> {
    map(preceded(char('@'), parse_number), |n| {
        LmmlCommand::SetWaveform(n)
    })
    .parse(input)
}

pub fn parse_channel_command(input: &str) -> IResult<&str, LmmlCommand, VerboseError<&str>> {
    map(preceded(char(':'), parse_number), |n| {
        LmmlCommand::SetChannel(n)
    })
    .parse(input)
}

pub fn parse_inc_octave_command(input: &str) -> IResult<&str, LmmlCommand, VerboseError<&str>> {
    map(char('>'), |_| LmmlCommand::IncreaseOctave).parse(input)
}

pub fn parse_dec_octave_command(input: &str) -> IResult<&str, LmmlCommand, VerboseError<&str>> {
    map(char('<'), |_| LmmlCommand::DecreaseOctave).parse(input)
}

pub fn parse_note_char(input: &str) -> IResult<&str, NoteChar, VerboseError<&str>> {
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
    })
    .parse(input)
}

pub fn parse_modifier(input: &str) -> IResult<&str, NoteModifier, VerboseError<&str>> {
    map(one_of("+-"), |c| match c {
        '+' => NoteModifier::Sharp,
        '-' => NoteModifier::Flat,
        _ => panic!(),
    })
    .parse(input)
}

pub fn parse_dot(input: &str) -> IResult<&str, bool, VerboseError<&str>> {
    map(opt(char('.')), |c| c.is_some()).parse(input)
}

pub fn parse_number(input: &str) -> IResult<&str, u32, VerboseError<&str>> {
    map(many1(one_of("0123456789")), |s| {
        s.iter().collect::<String>().parse::<u32>().unwrap()
    })
    .parse(input)
}

fn comment<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, (), E> {
    value((), (char(';'), many0(none_of("\n\r")), one_of("\n\r"))).parse(i)
}
