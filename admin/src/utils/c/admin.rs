use serde_json::json;
use crate::utils::contactserver::ask;


pub async fn exec(command:String, words:Vec<&str>) {
    println!("Command: {}", command);

    match command.as_str() {
        "admin clients list" => {
            let data = json!({});
            let res = ask("api/admin/clients".to_string(), data).await;

            if res.get("error").unwrap() == true {
                println!("Error: {}", res.get("error_msg").unwrap());
                return;
            }
            
            println!("\n[admin] Available clients:");
            println!("{}\n", res.get("clients").unwrap());
        }
        _ => {
            println!("Error: Command not found");
        }
    }
}