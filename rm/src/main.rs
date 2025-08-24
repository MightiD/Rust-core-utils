use std::fs;
use std::path::PathBuf;
use std::process;
use std::io::{self, ErrorKind, Write};

use clap::{Parser};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short = 'r')]
    recursive: bool,

    #[arg(short = 'p')]
    progress: bool,

    #[arg(short = 'f')]
    force: bool,

    #[arg(short = 'z')]
    are_you_sure: bool,

    #[arg()]
    paths: Vec<String>,
}

fn input(message: String) -> String {
    print!("{message}");
    io::stdout().flush().unwrap();
    let mut prompt = String::new();
    io::stdin()
        .read_line(&mut prompt)
        .expect("Failed to read user input");
    let prompt = prompt.trim().to_lowercase();
    prompt
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

fn get_items_in_dir(path: &str, search_sub_dirs: bool, path_array: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            match entry {
                Ok(entry) => {
                    if entry.path().is_dir() {
                        if search_sub_dirs {
                            match entry.path().to_str() {
                                Some(path) => {
                                    get_items_in_dir(path, true, path_array)
                                }
                                None => {
                                    eprintln!("There was an error converting a dir into a string");
                                }
                            };
                        }
                        path_array.push(entry.path());
                    }
                    else {
                        path_array.push(entry.path());
                    }
                }
                Err(_) => {
                    eprintln!("There was an error");
                    process::exit(1);
                }
            }
        }
    }
}

fn delete(item: &PathBuf, args: &Cli) -> String {
    let mut errors = String::new();

    let tmp_error = {
        //this branch handles regular files
        if item.is_file() || item.is_symlink() {
            match fs::remove_file(item) {
                Ok(_) => None,
                Err(e) => Some(e)
            }

        // handles dirs
        } else {
            if !args.recursive {
                eprintln!("rm: cannot remove '{}': Is a directory", item.to_string_lossy());
                None
            } else {
                match fs::remove_dir(item) {
                    Ok(_) => None,
                    Err(e) => Some(e)
                }
            }
        }
    };

    if let Some(err) = tmp_error {
        match err.kind() {
            ErrorKind::PermissionDenied => {
                errors.push_str("PermDenied");
            }
            ErrorKind::DirectoryNotEmpty => {
                errors.push_str("DirNotEmpty");
            }
            _ => {
                errors = "".to_string();
            }
        }
    }

    errors
}
fn main() {
    let args = Cli::parse();

    let mut paths: Vec<PathBuf> = Vec::new();

    let messages: Vec<String> = vec![
        String::from("Are you sure you want to delete these files? "),
        String::from("But are you really sure you want to? "),
        String::from("But think of how the files would feel? "),
        String::from("Youre a murderer if you delete these files "),
        String::from("How can you sleep at night knowing you delete innocent files who did nothing to you ")
    ];

    if args.paths.len() < 1 {
        println!("rm: missing operand\nTry 'rm --help' for more information.");
        process::exit(1);
    }

    if args.are_you_sure {
        for (i, message) in messages.iter().enumerate() {
            let response = input(message.to_string());
            if response == "y" || response == "yes" {
                if i == messages.len() - 1 {
                    println!("Fine, you got what you wanted you murderer");
                }
                continue;
            } else {
                println!("Youre a good person, you saved lives of innocent files today!");
                process::exit(0);
            }
        }
    }

    // this loop is to get the number of items we're dealing with for the progress bar
    for item in args.paths.iter() {
        let meta = match fs::metadata(item) {
            Ok(meta) => meta,
            Err(_) => {
                if !args.force {
                    println!("rm: cannot access '{}': No such file or directory", item);
                    process::exit(1);
                }
                continue;
            }
        };

        if meta.is_file() || meta.is_symlink() {
            let item_path = PathBuf::from(item);
            paths.push(item_path);
        }
        else if meta.is_dir() {
            //if -r, go over all sub paths
            if args.recursive {
                get_items_in_dir(item, args.recursive, &mut paths);
            }
            let item_path = PathBuf::from(item);
            paths.push(item_path);
        }
    }

    //sort and reverse so deletes all items in dir before dir
    paths.sort();
    paths.reverse();
    
    for (i, item) in paths.iter().enumerate() {
        let perms = match fs::metadata(item) {
            Ok(m) => m,
            Err(_) => {
                if !args.force {
                    println!("rm: cannot access '{}': No such file or directory", item.to_string_lossy());
                }
                continue;
            }
        }.permissions();

        let mut errors = String::new();

        if perms.readonly() {
            if args.force {
                errors = delete(item, &args);
            } else {
                let prompt = input(format!("rm: remove write-protected directory '{}'? ", item.to_string_lossy()));

                if prompt == "y" || prompt == "yes" {
                    errors = delete(item, &args);
                }
            }
        } else {
            errors = delete(item, &args);
        }

        match errors.as_str() {
            "PermDenied" => {
                eprintln!("rm: cannot remove '{}': Permission denied", item.to_string_lossy());
            }
            "DirNotEmpty" => {
                eprintln!("rm: cannot remove '{}': Directory not empty", item.to_string_lossy());
            }
            _ => {}
        }

        if args.progress {
            print!("\x1B[?25l"); // dont show cursor
            let bar = progress_bar(i + 1, paths.len());
            print!("{}\r", bar);
            io::stdout().flush().unwrap();
        }
    }

    if args.progress {print!("\x1B[?25h\n");} //show cursor again
    
}
