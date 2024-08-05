use colored::Colorize;

const DEBUG: bool = true;

fn get_pretty_date() -> String {
    let now = chrono::Local::now();
    now.format("%Y-%m-%d %H:%M:%S").to_string()
}


pub fn log_info(message: &str) {
    println!("[{}] {} - {}", get_pretty_date(), "INFO".green(), message);
}

pub fn log_error(message: &str) {
    println!("[{}] {} - {}", get_pretty_date(), "ERROR".red(), message);
}

pub fn log_warning(message: &str) {
    println!("[{}] {} - {}", get_pretty_date(), "WARNING".yellow(), message);
}

pub fn log_debug(message: &str) {
    if DEBUG {
        println!("[{}] {} - {}", get_pretty_date(), "DEBUG".blue(), message);
    }
}