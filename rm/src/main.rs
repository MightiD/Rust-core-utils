use std::io::{self, Write};

fn progress_bar(current: usize, length: usize) -> String {
    let fill_char = "|";
    let not_filled = " ";

    let bar_width = 50;

    let percentage_done = current * 100 / length;
    let bar_chars = percentage_done * bar_width / 100;

    let mut bar = String::from("[");

    for _ in 1..bar_chars {
        bar.push_str(fill_char);
    }
    for _ in bar_chars..bar_width {
        bar.push_str(not_filled);
    }

    bar.push_str("]");

    bar.push_str(format!(" {}/{} ({}%)", current, length, percentage_done).as_str());

    bar

}

fn main() {
    let range = 10;
    print!("\x1B[?25l");
    for i in 0..range {
        let bar = progress_bar(i + 1, range);
        print!("{}\r", bar);
        io::stdout().flush().unwrap();
    }

    print!("\x1B[?25h");
    println!();
}
