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
use std::sync::{Arc, Mutex};
use std::{thread, time};
use uuid::Uuid;

//fn save(db: &Database) {
//fs::write("db.json", serde_json::to_string(&db).unwrap()).unwrap();
//}
#[derive(Serialize, Deserialize)]
struct ResetRequest {
    username: String,
    password: String,
    token: String,
}
#[derive(Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize)]
struct MeRequest {
    token: String,
    username: String,
}
#[derive(Serialize, Deserialize)]
struct EmailRequest {
    token: String,
    username: String,
    email: String,
}
#[derive(Serialize, Deserialize)]
struct UpdateRequest {
    token: String,
    username: String,
    action: String,
    id: String,
    text: String,
    done: bool,
}
#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    hashword: String,
    todos: Vec<Todo>,
    tokens: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    text: String,
    done: bool,
    id: String,
    num: i32,
}
/*#[derive(Deserialize, Serialize)]
struct Database {
    users: Mutex<HashMap<String, User>>,
    salt: String,
}*/

#[get("/")]
fn index() -> &'static str {
    "online"
}
#[post("/login", format = "application/json", data = "<data>")]
fn login(data: Json<LoginRequest>) -> Json<serde_json::Value> {
    let mut client = Client::connect("host=db user=postgres password=example", NoTls).unwrap();
    if let Ok(user) = client.query_one(
        "SELECT * FROM users WHERE username = $1",
        &[&data.username.to_string()],
    ) {
        let hashword: String = user.get("hashword");
        if argon2::verify_encoded(&hashword, &data.password.as_bytes()).unwrap() {
            let token = Uuid::new_v4().to_string();
            client
                .execute(
                    "INSERT INTO tokens VALUES ($1, $2);",
                    &[&data.username.to_string(), &token],
                )
                .unwrap();
            Json(serde_json::json!({
                "error": false,
                "response": {
            "token": token
                }
            }))
        } else {
            Json(serde_json::json!({
                "error": true, "error_message": "Incorrect username/password"
            }))
        }
    } else {
        Json(serde_json::json!({
            "error": true, "error_message": "User doesn't exist"
        }))
    }
}
#[post("/me", format = "application/json", data = "<data>")]
fn me(data: Json<MeRequest>) -> Json<serde_json::Value> {
    let mut client = Client::connect("host=db user=postgres password=example", NoTls).unwrap();
    if let Ok(_) = client.query_one("SELECT * FROM users WHERE username = $1", &[&data.username]) {
        if let Ok(_) = client.query_one(
            "SELECT username FROM tokens WHERE token = $1",
            &[&data.token],
        ) {
            let mut todos: Vec<Todo> = client
                .query(
                    "SELECT * FROM todos WHERE username = $1 ORDER BY num",
                    &[&data.username.to_string()],
                )
                .unwrap()
                .iter()
                .map(|n| Todo {
                    text: n.get("todo"),
                    done: n.get("done"),
                    id: n.get("id"),
                    num: n.get("num"),
                })
                .collect();
            Json(serde_json::json!({
                "error": false,
                "response": {
                    "user": {"username": data.username, "todos": todos},
                }
            }))
        } else {
            Json(serde_json::json!({
                "error": true, "error_message": "Incorrect token"
            }))
        }
    } else {
        Json(serde_json::json!({
            "error": true, "error_message": "User doesn't exist"
        }))
    }
}
#[post("/email", format = "application/json", data = "<data>")]
fn email(data: Json<EmailRequest>) -> Json<serde_json::Value> {
    //let mut client = Client::connect("host=db user=postgres password=example", NoTls).unwrap();
    /*
    let dbusers = database.users.lock().unwrap();
    if let Some(user) = dbusers.get(&data.username) {
        if user.tokens.contains(&data.token) {
            if fast_chemail::is_valid_email(&data.email) {
                let email = SendableEmail::new(
                    Envelope::new(
                        Some(EmailAddress::new("scratchywon@gmail.com".to_string()).unwrap()),
                        vec![EmailAddress::new("connorstaj@gmail.com".to_string()).unwrap()],
                    )
                    .unwrap(),
                    "id".to_string(),
                    "Hello world".to_string().into_bytes(),
                );

                // Open a local connection on port 25
                let mut mailer = SmtpClient::new_unencrypted_localhost().unwrap().transport();
                // Send the email
                let result = mailer.send(email);

                if result.is_ok() {
                    println!("Email sent");
                } else {
                    println!("Could not send email: {:?}", result);
                }
                Json(serde_json::json!({
                    "error": false
                }))
            } else {
                Json(serde_json::json!({
                    "error": true, "error_message": "Invalid email"
                }))
            }
        } else {
            Json(serde_json::json!({
                "error": true, "error_message": "Incorrect token"
            }))
        }
    } else {
        Json(serde_json::json!({
            "error": true, "error_message": "User doesn't exist"
        }))
    }*/
    Json(serde_json::json!({
        "error": true, "error_message": "User doesn't exist"
    }))
}
#[post("/update", format = "application/json", data = "<data>")]
fn update(data: Json<UpdateRequest>) -> Json<serde_json::Value> {
    let mut client = Client::connect("host=db user=postgres password=example", NoTls).unwrap();
    if let Ok(user) = client.query_one(
        "SELECT * FROM users WHERE username = $1",
        &[&data.username.to_string()],
    ) {
        if let Ok(_) = client.query_one(
            "SELECT username FROM tokens WHERE token = $1",
            &[&data.token],
        ) {
            match data.action.as_ref() {
                "add" => {
                    client
                        .execute(
                            "INSERT INTO todos VALUES ($1, $2, $3, $4)",
                            &[&data.username, &data.text, &false, &data.id],
                        )
                        .unwrap();
                    Json(serde_json::json!({
                        "error": false,
                    }))
                }
                "remove" => {
                    client
                        .execute("DELETE FROM todos WHERE id = $1", &[&data.id])
                        .unwrap();
                    Json(serde_json::json!({
                        "error": false,
                    }))
                }
                "done" => {
                    client
                        .execute(
                            "UPDATE todos SET done = $1 WHERE id = $2",
                            &[&data.done, &data.id],
                        )
                        .unwrap();
                    Json(serde_json::json!({
                        "error": false,
                    }))
                }
                _ => Json(serde_json::json!({
                    "error": true, "error_message": "Invalid change"
                })),
            }
        } else {
            Json(serde_json::json!({
                "error": true, "error_message": "Incorrect token"
            }))
        }
    } else {
        Json(serde_json::json!({
            "error": true, "error_message": "User doesn't exist"
        }))
    }
}
#[post("/logout", format = "application/json", data = "<data>")]
fn logout(data: Json<MeRequest>) -> Json<serde_json::Value> {
    let mut client = Client::connect("host=db user=postgres password=example", NoTls).unwrap();
    if let Ok(_) = client.query_one(
        "SELECT username FROM users WHERE username = $1",
        &[&data.username.to_string()],
    ) {
        if let Ok(_) = client.query_one(
            "SELECT username FROM tokens WHERE token = $1",
            &[&data.token],
        ) {
            client
                .execute("DELETE FROM tokens WHERE token = $1", &[&data.token])
                .unwrap();
            Json(serde_json::json!({
                "error": false
            }))
        } else {
            Json(serde_json::json!({
                "error": true, "error_message": "Incorrect token"
            }))
        }
    } else {
        Json(serde_json::json!({
            "error": true, "error_message": "User doesn't exist"
        }))
    }
}
#[post("/change_password", format = "application/json", data = "<data>")]
fn change_password(data: Json<ResetRequest>) -> Json<serde_json::Value> {
    let mut client = Client::connect("host=db user=postgres password=example", NoTls).unwrap();
    let argonconfig: argon2::Config = Config::default();
    match client.query_one(
        "SELECT username FROM users WHERE username = $1",
        &[&data.username.to_string()],
    ) {
        Ok(_) => {
            if let Ok(_) = client.query_one(
                "SELECT username FROM tokens WHERE token = $1",
                &[&data.token],
            ) {
                let token = Uuid::new_v4().to_string();
                client
                    .execute(
                        "UPDATE users SET hashword = $2 WHERE username = $1",
                        &[
                            &data.username.to_string(),
                            &argon2::hash_encoded(
                                data.password.as_bytes(),
                                "SALTINES".as_bytes(),
                                &argonconfig,
                            )
                            .unwrap(),
                        ],
                    )
                    .unwrap();
                client
                    .execute(
                        "DELETE FROM tokens WHERE username = $1",
                        &[&data.username.to_string()],
                    )
                    .unwrap();
                client
                    .execute(
                        "INSERT INTO tokens VALUES ($1, $2);",
                        &[&data.username.to_string(), &token],
                    )
                    .unwrap();
                Json(serde_json::json!({
                    "error": false,
                    "response": {
                        "token": token
                    }
                }))
            } else {
                Json(serde_json::json!({
                    "error": true, "error_message": "Invalid token"
                }))
            }
        }
        Err(_) => Json(serde_json::json!({
            "error": true, "error_message": "User doesn't exist"
        })),
    }
}
#[post("/signup", format = "application/json", data = "<data>")]
fn signup(data: Json<LoginRequest>) -> Json<serde_json::Value> {
    let mut client = Client::connect("host=db user=postgres password=example", NoTls).unwrap();
    let argonconfig: argon2::Config = Config::default();
    match client.query_one(
        "SELECT username FROM users WHERE username = $1",
        &[&data.username.to_string()],
    ) {
        Err(_) => {
            let token = Uuid::new_v4().to_string();
            client
                .execute(
                    "INSERT INTO users VALUES ($1, $2);",
                    &[
                        &data.username.to_string(),
                        &argon2::hash_encoded(
                            data.password.as_bytes(),
                            "SALTINES".as_bytes(),
                            &argonconfig,
                        )
                        .unwrap(),
                    ],
                )
                .unwrap();
            client
                .execute(
                    "INSERT INTO tokens VALUES ($1, $2);",
                    &[&data.username.to_string(), &token],
                )
                .unwrap();
            Json(serde_json::json!({
                "error": false,
                "response": {
                    "token": token
                }
            }))
        }
        Ok(_) => Json(serde_json::json!({
            "error": true, "error_message": "User already exists"
        })),
    }
}
fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[
        "http://localhost:3000",
        "https://scratchyone.com",
        "https://www.scratchyone.com",
    ]);

    CorsOptions {
        // 5.
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}

fn main() {
    thread::sleep(time::Duration::from_millis(2000));
    let mut client = Client::connect("host=db user=postgres password=example", NoTls).unwrap();
    client
        .batch_execute(
            "
CREATE EXTENSION IF NOT EXISTS citext;
CREATE TABLE IF NOT EXISTS users (
    username citext PRIMARY KEY,
    hashword text
);
CREATE TABLE IF NOT EXISTS tokens (
    username citext,
    token text
);
CREATE TABLE IF NOT EXISTS todos (
    username citext,
    todo text,
    done boolean,
    id text,
    num serial
)",
        )
        .unwrap();
    let cfg = rocket::config::Config::build(rocket::config::Environment::Development)
        .port(80)
        .address("0.0.0.0")
        .unwrap();
    rocket::custom(cfg)
        .attach(make_cors())
        .mount(
            "/",
            routes![
                index,
                login,
                signup,
                me,
                logout,
                update,
                email,
                change_password
            ],
        )
        .launch();
}
