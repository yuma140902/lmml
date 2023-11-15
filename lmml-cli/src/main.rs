use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    lmml: String,
}

fn main() {
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

    if let Ok((_, ast)) = ast {
        let timeline = ast.to_timeline();
        println!("=== Timeline ===");
        println!("{:#?}", timeline);
    }
}
