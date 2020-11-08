

use serde::{Deserialize, Serialize};
use actix_files::Files;
use actix_files::NamedFile;

use actix_web::{get, post, web, App, cookie,HttpResponse, Error,HttpServer,HttpRequest};
use cookie::Cookie;

extern crate argon2;
use argon2::{Config};
use rand::{thread_rng, Rng};
use rand::distributions::{Alphanumeric, Uniform, Standard};
use std::time::{Duration, Instant};

pub mod db;
pub mod errors;
use db::*;
use errors::*;


#[derive(Deserialize, Debug)]
struct User_Data {
    username: String,
    password: String,
}

fn make_new_cookie(token: &String) -> String {
    let rng = thread_rng();
    let cookie = Cookie::build("token", token)
    .domain("127.0.0.1")
    .path("/")
    .secure(false)
    .finish();
    cookie.to_string()
}

fn new_token() -> String {
    let rng = thread_rng();
    rng.sample_iter(Alphanumeric).take(32).collect()
}

fn return_hashed_password(password: &[u8]) -> String{
    let rng = thread_rng();
    let random_salt: String = rng.sample_iter(Alphanumeric).take(32).collect();// Generate a random salt of 32 characters
    let config = Config::default();
    let hash = argon2::hash_encoded(password, random_salt.as_bytes(), &config).unwrap();
    return hash;
}


fn validate_password(username: String,input_password: &[u8]) -> bool  {
    let passwords = find_user_account(username).unwrap();
    for pass in passwords {
        let hashed_password: String = pass.get(0);
        let correct_password = argon2::verify_encoded(&hashed_password, &input_password).unwrap();
        if correct_password {
            return true;
        }
    }
    return false;
}


#[post("/api/authorize_user")]
async fn authorize_user(form: web::Form<User_Data>) -> Result<HttpResponse,Error> {
    let username = &form.username;
    let password = &form.password;
    let success = validate_password(username.to_string(),password.as_bytes());
    if success {
        let users_token = new_token();
        give_user_token(username.to_string(),&users_token); // give them a new token
        return Ok(HttpResponse::Found().header("Set-Cookie",make_new_cookie(&users_token)).header("Location", "/api/restricted_area").finish());
    }else{
        return Ok(HttpResponse::Unauthorized().finish());
    }
}

async fn sign_up(req: HttpRequest) -> Result<NamedFile, Error> {
    Ok(NamedFile::open("./signup.html")?)
}


async fn login(req: HttpRequest) -> Result<NamedFile, Error> {
    Ok(NamedFile::open("./login.html")?)
}

#[post("/api/create_user")]
async fn create_user(form: web::Form<User_Data>) -> Result<HttpResponse,Error> {
    let username = &form.username;
    let password = &form.password;
    let hashed_password = return_hashed_password(password.as_bytes());
    create_user_acount(username.to_string(),hashed_password).unwrap();
    Ok(HttpResponse::Found()
    .header("Location", "/login.html")
    .finish())
}

//

fn validate_token(req: HttpRequest) -> Result<bool,MyError> {
    let header_cookies = req.headers().get("cookie");
    if header_cookies.is_some() { // Checks to make sure there are some cookies
        let unparsed_cookies = header_cookies.unwrap().to_str().unwrap();
        let parsed_cookies = unparsed_cookies.split(';')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(Cookie::parse)
                    .map(|s| s.unwrap())
                    .filter(|s| s.name() == "token")
                    .nth(0);
        if parsed_cookies.is_none() {return Ok(false)};
        let users_token = find_token(parsed_cookies.unwrap().value().to_string())?;
        return Ok(!users_token.is_empty());
    }
    return Ok(false);
}
#[get("/api/restricted_area")]
async fn restricted_area(req: HttpRequest) -> Result<HttpResponse,Error> {
    let valid_token = validate_token(req)?;
    println!("{}",valid_token);
    if valid_token { // If they have a valid let them in else redirect them back to the login page
        Ok(HttpResponse::Ok().body("WElcome in"))
    }else{
        return Ok(HttpResponse::Found().header("Location", "http://127.0.0.1:8080/login").finish());
    }
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
        HttpServer::new(|| {
            App::new()
                .service(authorize_user)
                .service(create_user)
                .service(restricted_area)
                .route("/signup", web::get().to(sign_up))
                .route("/login", web::get().to(login))
                .service(Files::new("/", ".").index_file("index.html"))
        })
        .bind("127.0.0.1:8080")?
        .run()
        .await?;
        Ok(())
}
