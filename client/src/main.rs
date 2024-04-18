// export this as client

use std::thread;
pub mod socket;

fn main() {


    //let (tx, rx) = mpsc::channel();

    // create a new thread to run the server
    thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let _ = socket::init::start_server();
        });
    });


    println!("Client started!");

    loop {
        // Wait for 5 seconds
        thread::sleep(std::time::Duration::from_secs(5));
    }
}
