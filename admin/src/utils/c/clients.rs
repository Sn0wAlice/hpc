use serde_json::json;
use crate::utils::{command, contactserver::ask};


pub async fn exec(command:String, words:Vec<&str>) {
    println!("Command: {}", command);

    match command.as_str() {
        "clients list" => {
            let data = json!({});
            let res = ask("api/admin/clients".to_string(), data).await;

            if res.get("error").unwrap() == true {
                println!("Error: {}", res.get("error_msg").unwrap());
                return;
            }
            
            println!("\n[admin] Available clients:");
            println!("{}\n", res.get("clients").unwrap());
        }

        command if command.starts_with("clients exec") => {

            let mut execute_string = String::new();
            for i in 3..words.len() {
                execute_string.push_str(words[i]);
                if i < words.len() - 1 {
                    execute_string.push(' ');
                }
            }

            let data = json!({
                "client_uuid": words[2],
                "execute": execute_string
            });

            let _ = ask("api/clients/exec".to_string(), data).await;
        }

        command if command.starts_with("clients debug") => {
            let data = json!({
                "client_uuid": words[2]
            });
            let res = ask("api/admin/debug".to_string(), data).await;

            // check if res.get("error") exist
            if res.get("error").is_none() {
                if !res.get("status").is_none() {
                    println!("\n[admin] Debug message sent to client: {}", words[2]);
                    return;
                }
            } else {
                println!("Error: {}", res.get("error_msg").unwrap());
            }

        }

        _ => {
            println!("Error: Command not found");
        }
    }
}