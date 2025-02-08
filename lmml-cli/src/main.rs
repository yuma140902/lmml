#![deny(rust_2018_idioms)]
#![deny(clippy::all)]
#![deny(clippy::nursery)]

use std::{io::Write, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use lmml::ast::{EvalEnv, LmmlAst};
use nom::IResult;
use nom_language::error::VerboseError;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    subcommand: SubCommand,
}

#[derive(Debug, clap::Subcommand)]
pub enum SubCommand {
    /// ファイルを演奏する
    Load { file: PathBuf },
    /// 対話的に演奏する
    Repl,
}

fn unwrap_or_show_error(
    ast: IResult<&str, LmmlAst, VerboseError<&str>>,
    input: &str,
) -> anyhow::Result<LmmlAst> {
    match ast {
        Err(err) => {
            match err {
                nom::Err::Incomplete(_) => {
                    eprintln!("nom::Err::Incomplete");
                }
                nom::Err::Error(e) | nom::Err::Failure(e) => {
                    eprintln!("{}", nom_language::error::convert_error(input, e));
                }
            }
            anyhow::bail!("LMMLに構文エラーがあります")
        }
        Ok((_, ast)) => Ok(ast),
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.subcommand {
        SubCommand::Load { file } => {
            let input = std::fs::read_to_string(&file)
                .with_context(|| format!("ファイル \"{}\"を開けませんでした", file.display()))?;
            println!("lmml:");
            println!("{}", input);
            println!();
            let input = lmml_parser::remove_comments(&input);
            let ast = lmml_parser::parse_lmml(&input);

            println!("parser result:");
            println!("=== AST ===");
            println!("{:#?}", ast);
            println!();

            let ast = unwrap_or_show_error(ast, &input)?;

            let timeline = ast.to_timeline(&mut EvalEnv::default());
            println!("=== Timeline ===");
            println!("{}", timeline);

            let (_stream, stream_handle) = rodio::OutputStream::try_default().with_context(|| {
        "音声出力ストリームの取得に失敗しました。Windows WASAPIでのみ動作確認しています。"
    })?;
            let sink = rodio::Sink::try_new(&stream_handle).with_context(|| {
                "音声出力の作成に失敗しました。Windows WASAPIでのみ動作確認しています。"
            })?;
            timeline.play(&sink);
            sink.sleep_until_end();
        }
        SubCommand::Repl => {
            let (_stream, stream_handle) = rodio::OutputStream::try_default().with_context(|| {
        "音声出力ストリームの取得に失敗しました。Windows WASAPIでのみ動作確認しています。"
    })?;
            let sink = rodio::Sink::try_new(&stream_handle).with_context(|| {
                "音声出力の作成に失敗しました。Windows WASAPIでのみ動作確認しています。"
            })?;

            let mut env = EvalEnv::default();
            loop {
                print!("{} :{} ", env.current(), env.current_channel);
                std::io::stdout()
                    .flush()
                    .with_context(|| "標準出力への出力エラー")?;
                let mut line = String::new();
                std::io::stdin()
                    .read_line(&mut line)
                    .with_context(|| "標準入力からの読み込みエラー")?;

                let line = line.trim();
                if line.is_empty() || line.starts_with(';') {
                    continue;
                }

                let ast = lmml_parser::parse_lmml(line);
                let ast = match unwrap_or_show_error(ast, line) {
                    Err(e) => {
                        println!("{}", e);
                        continue;
                    }
                    Ok(ast) => ast,
                };
                let timeline = ast.to_timeline(&mut env);
                println!("=== Timeline ===");
                println!("{}", timeline);
                timeline.play(&sink);
            }
        }
    }

    Ok(())
}
