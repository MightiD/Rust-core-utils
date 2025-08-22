use std::io::{self, Write};

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

fn main() {
    let range = 100000;
    print!("\x1B[?25l"); //hide cursor so you cant see it jump around all the time
    for i in 0..range {
        let bar = progress_bar(i + 1, range);
        print!("{}\r", bar);
        io::stdout().flush().unwrap();
    }

    print!("\x1B[?25h"); //show cursor again
    println!(); //newline at the end
}
