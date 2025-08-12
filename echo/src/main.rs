use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    //do not output the trailing newline
    #[arg(short = 'n')]
    n: bool, 

    //enable interpretation of backslash escapes
    #[arg(short = 'e')]
    e: bool, 

    //disable interpretation of backslash escapes (default)
    #[arg(short = 'E')]
    E: bool, 

    // the user input
    #[arg()]
    input: Vec<String>,
}

fn main() {
    let args = Cli::parse();

    for word in args.input.iter() {
        // need trailing white space so doesnt put words together
        print!("{} ", word)
    }

    // TODO: make not have trailing whitespace on last word

    if !args.n {
        println!("")
    }

    // dbg!(args);
    // println!("{input_string}");
}
