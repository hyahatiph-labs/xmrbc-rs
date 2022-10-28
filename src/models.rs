use diesel::prelude::*;

#[derive(Queryable)]
pub struct TxMessage {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

use crate::schema::messages;

#[derive(Insertable)]
#[diesel(table_name = messages)]
pub struct NewTxMessage<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
