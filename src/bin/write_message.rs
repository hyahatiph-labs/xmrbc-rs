use std::io::{stdin, Read};
use xmrbc::*;

// manual message creation
fn main() {
    let connection = &mut establish_connection();

    let mut subaddress = String::new();
    let mut body = String::new();

    println!("Enter subaddress: ");
    stdin().read_line(&mut subaddress).unwrap();
    let subaddress = subaddress.trim_end(); // Remove the trailing newline

    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        subaddress, EOF
    );
    stdin().read_to_string(&mut body).unwrap();

    let msg = create_message(connection, subaddress, &body);
    println!("\nSaved draft {} with id {}", subaddress, msg.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
