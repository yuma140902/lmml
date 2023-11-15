use anyhow::Context;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    lmml: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    println!("lmml:");
    println!("{}", args.lmml);
    println!();
    println!("remove comments:");
    let input = lmml_parser::remove_comments(&args.lmml);
    println!("Input: {}", input);
    println!();

    let ast = lmml_parser::parse_lmml(&input);

    println!("parser result:");
    println!("=== AST ===");
    println!("{:#?}", ast);
    println!();

    let ast = match ast {
        Err(err) => {
            match err {
                nom::Err::Incomplete(_) => {
                    eprintln!("nom::Err::Incomplete");
                }
                nom::Err::Error(e) | nom::Err::Failure(e) => {
                    eprintln!("{}", input);
                    let pos = input.chars().count() - e.input.chars().count();
                    for _ in 0..pos {
                        eprint!(" ");
                    }
                    eprintln!("^");
                }
            }
            anyhow::bail!("LMMLに構文エラーがあります")
        }
        Ok((_, ast)) => ast,
    };

    let timeline = ast.to_timeline();
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

    Ok(())
}
