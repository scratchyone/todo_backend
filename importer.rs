#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
use argon2::{self, Config};
use postgres::{Client, NoTls};
use rocket::http::Method;
use rocket_contrib::json::Json;
use rocket_cors;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
use serde::Deserialize;
use serde::Serialize;
use serde_json;
use std::fs;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use uuid::Uuid;

//fn save(db: &Database) {
//fs::write("db.json", serde_json::to_string(&db).unwrap()).unwrap();
//}
#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    text: String,
    done: bool,
    key: String,
}
/*#[derive(Deserialize, Serialize)]
struct Database {
    users: Mutex<HashMap<String, User>>,
    salt: String,
}*/
fn main() {
    thread::sleep(time::Duration::from_millis(2000));
    let mut client = Client::connect("host=db user=postgres password=example", NoTls).unwrap();
    let contents = fs::read_to_string("dad.json").expect("Something went wrong reading the file");
    let dad: Vec<Todo> = serde_json::from_str(&contents).unwrap();
    for item in dad {
        client
            .execute(
                "INSERT INTO todos VALUES ($1, $2, $3, $4)",
                &[&"stajdude", &item.text, &item.done, &item.key],
            )
            .unwrap();
    }
}
