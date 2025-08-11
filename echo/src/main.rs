use std::env;

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
    let args: Vec<String> = env::args().collect();
    let input_string = create_string_from_args(&args);
    dbg!(args);
    println!("{input_string}");
}
