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

fn create_string_from_args(args: &Vec<String>) -> String {

    //must be from first part of input
    //as it would add nothing to args[1]
    //resulting in a leading space
    let mut input = String::from(&args[1]);

    for (i, arg) in args[2..].iter().enumerate() {
        //probably not the most efficient way to add 2 strings
        //but it works
        input = format!("{} {}", input, arg);
    }

    input

}

fn main() {
    let args = Cli::parse();
    let input_string = create_string_from_args(&args.input);
    dbg!(args);
    println!("{input_string}");
}
