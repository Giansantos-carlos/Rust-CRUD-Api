use postgres::{Client, NoTls}
use postgres::Error as PostgresError;
use std::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::env;

#[macro_use]
extern crate serde_derive;


//Model user Strutc whith Id, Name and Email

#[derive(Serialize, Deserialize)]
struct User{
    id: Option<i32>,
    name: String,
    email: String,
}

// Database_url 
const DB_URL : &str = !env::var("DATABASE_URL").expect(msg)