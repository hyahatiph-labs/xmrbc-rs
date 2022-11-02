use diesel::prelude::*;
use self::models::*;
use xmrbc::*;

// manual message retreival
fn main() {
    use self::schema::messages::dsl::*;

    let connection = &mut establish_connection();
    let results = messages
        .filter(published.eq(true))
        .limit(5)
        .load::<TxMessage>(connection)
        .expect("Error loading messages");

    println!("Displaying {} messages", results.len());
    for m in results {
        println!("{}", m.subaddress);
        println!("-----------\n");
        println!("{}", m.body);
    }
}
