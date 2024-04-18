// create a terminal for the user to get the String input

use std::io::{self, Write};
use std::thread;
pub mod utils;

async fn get_user_input() {
    loop {
        let mut input = String::new();
        print!("hpc > ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        utils::command::manager(input.trim().to_string()).await;
    }
}

fn main() {

    // Read ascii art in utils/ascii.art
    let ascii_art = include_str!("../utils/ascii.art");
    println!("{}", ascii_art);

    // create a new thread to run the server
    thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            get_user_input().await;
        });
    });

    loop {}
}