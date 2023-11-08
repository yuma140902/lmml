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
    println!("{}", input);
    println!();
    println!("parser result:");
    println!("{:?}", lmml_parser::parse_lmml(&input));
}
