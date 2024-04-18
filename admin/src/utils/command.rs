use crate::utils::c::*;


pub async fn manager(command:String) {

    // cut the string into a vector of words
    let words: Vec<&str> = command.split_whitespace().collect();

    // match the first word
    match words[0] {
        "admin" => {
            // if the first word is admin, then call the admin function
            admin::exec(command.clone(), words).await;
        },
        _ => {
            // if the first word is not admin, then print an error message
            println!("Error: Command not found");
        }
    }
}