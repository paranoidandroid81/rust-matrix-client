use std::io;
use std::io::Write;
use rpassword;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to my simple Rust Matrix Client!");

    let user_name = retrieve_input("username");

    let password = rpassword::prompt_password("Please enter your password: ").unwrap();

    let client = reqwest::Client::new();

    let login_resp = client.get("https://matrix.org/.well-known/matrix/client")
        .send()
        .await?
        .text()
        .await?;
    
    println!("{login_resp:?}");

    Ok(())
}

fn retrieve_input(input_name: &str) -> String {
    print!("Please enter your {input_name}: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .ok()
        .expect("Couldn't read {input_name}");    

    input
}
