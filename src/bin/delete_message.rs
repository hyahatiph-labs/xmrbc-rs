use diesel::prelude::*;
use std::env::args;
use xmrbc::*;

// manual message deletion
fn main() {
    use self::schema::messages::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(messages.filter(subaddress.like(pattern)))
        .execute(connection)
        .expect("Error deleting messages");

    println!("Deleted {} messages", num_deleted);
}

