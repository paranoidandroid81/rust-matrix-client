use std::io;
use std::io::Write;
use rpassword;
use std::collections::HashMap;
use simple_matrix_client::models::{ WellKnownResponse, LoginRequest, LoginResponse };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to my simple Rust Matrix Client!");

    let mut user_name = retrieve_input("username");
    user_name.pop(); //remove \n 

    let password = rpassword::prompt_password("Please enter your password: ").unwrap();

    let mut server_name = retrieve_input("server name (leave blank to use matrix.org)");
    if server_name.trim().is_empty() {
        server_name = "matrix.org".to_owned();
    }
    println!("{server_name}");

    let client = reqwest::Client::new();

    let well_known_resp = client.get(format!("https://{server_name}/.well-known/matrix/client"))
        .send()
        .await?
        .json::<WellKnownResponse>()
        .await?;
    
    println!("{well_known_resp:?}");

    let base_url = well_known_resp.get_base_url();

    let login_req = LoginRequest::new_username_password_login(user_name, password);

    println!("{login_req:?}");


    let login_resp = client.post(format!("{base_url}/_matrix/client/v3/login"))
        .json(&login_req)
        .send()
        .await?
        .json::<LoginResponse>()
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
