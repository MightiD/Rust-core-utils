use std::fs;
use std::path::PathBuf;
use std::process;

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short = 'r')]
    recursive: bool,

    #[arg()]
    paths: Vec<String>,
}

fn progress_bar(current: usize, length: usize) -> String {
    let fill_char = "-";
    let not_filled = " ";

    let bar_width = 50;

    let percentage_done = current * 100 / length;
    let bar_chars = percentage_done * bar_width / 100;

    let mut bar = String::from("[");

    let color = if percentage_done <= 30 {
        "\x1B[31m"
    } else if percentage_done <=60 {
        "\x1B[33m"
    } else {
        "\x1B[32m"
    };

    for _ in 1..bar_chars {
        bar.push_str(color);
        bar.push_str(fill_char);
    }

    for _ in bar_chars..bar_width {
        bar.push_str(not_filled);
    }

    // color to white, adds progress and percentage
    bar.push_str(format!("\x1B[0m] {}/{} ({}%)", current, length, percentage_done).as_str());

    bar

}

fn get_total_items(path: &str, search_sub_dirs: bool, path_array: &mut Vec<PathBuf>) -> usize {
    let mut items = 0;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            match entry {
                Ok(entry) => {

                    if entry.path().is_dir() {
                        if search_sub_dirs {
                            let tmp = match entry.path().to_str() {
                                Some(path) => {
                                    get_total_items(path, true, path_array)
                                }
                                None => {
                                    eprintln!("There was an error converting a dir into a string");
                                    0
                                }
                            };
                            items += tmp;
                        }
                        else {
                            path_array.push(entry.path());
                            items += 1;
                        }
                        
                    }
                    else {
                        path_array.push(entry.path());
                        items += 1;
                    }
                }
                Err(_) => {
                    eprintln!("There was an error");
                    process::exit(1);
                }
            }
        }
    }

    items
}

fn main() {
    let args = Cli::parse();

    let mut items = 0;

    let mut paths: Vec<PathBuf> = Vec::new();

    if args.paths.len() < 1 {
        println!("rm: missing operand\nTry 'rm --help' for more information.");
        process::exit(1);
    }

    // this loop is to get the number of items we're dealing with for the progress bar
    for (_, item) in args.paths.iter().enumerate() {
        match fs::metadata(item) {
            Ok(meta) => {
                if meta.is_file() || meta.is_symlink() {
                    items += 1;
                    let item_path = PathBuf::from(item);
                    paths.push(item_path);
                    items += 1;

                } else if meta.is_dir() {
                    //if -r, go over all sub paths
                    if args.recursive {
                        items += get_total_items(item, args.recursive, &mut paths);
                    }
                    //if not, increment for the dir
                    else {
                        items += 1;
                    }
                }
            }
            Err(_) => {
                println!("rm: cannot access '{}': No such file or directory", item);
                process::exit(1);
            }
        }

        dbg!(&paths);
        dbg!(&paths.len());
        println!("{}", items);

        // print!("\x1B[?25l");
        // let bar = progress_bar(i + 1, args.len() - 1);
        // print!("{}\r", bar);
        // io::stdout().flush().unwrap();
    }

    print!("\x1B[?25h\n");
}
