#![allow(unused)]
use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    net::SocketAddr,
};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let app: Router = Router::new()
        .route("/", get(handler))
        .route("/createUser", post(create_user))
        .route("/users", get(get_all_users))
        .route("/users/:username", get(get_user_by_name))
        .route("/deleteUser/:username", post(delete_user))
        .route("/createNote", post(create_note))
        .route("/notes/:id", get(get_note))
        .route("/deleteNote/:id", post(delete_note))
        .route("/updateNote/:id", post(update_note))
        .route("/notes", get(get_notes));

    let addr: SocketAddr = "127.0.0.1:3000".parse().unwrap();
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> &'static str {
    "Hello, World!....."
}

#[derive(Serialize, Deserialize)]
struct User {
    username: String,
    password: String,
}
async fn create_user(Json(user): Json<User>) -> impl IntoResponse {
    let json = serde_json::to_string(&user).unwrap();
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("users.json")
        .unwrap();
    file.write_all(format!("{}\n", json).as_bytes()).unwrap();
    (StatusCode::CREATED, Json(user))
}
async fn get_all_users() -> impl IntoResponse {
    let mut file = File::open("users.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let users: Vec<User> = contents
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    (StatusCode::OK, Json(users))
}
//async fn get_user_by_username(Json(username): Json<String>) -> impl IntoResponse {
//    let mut file = File::open("users.json").unwrap();
//    let mut contents = String::new();
//    file.read_to_string(&mut contents).unwrap();
//    let users: Vec<User> = contents
//        .lines()
//        .map(|line| serde_json::from_str(line).unwrap())
//        .collect();
//    let user = users.into_iter().find(|user| user.username == username);
//    (StatusCode::OK, Json(user))
//}
async fn get_user_by_name(Path(username): Path<String>) -> impl IntoResponse {
    let mut file = File::open("users.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let users: Vec<User> = contents
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    let user = users.into_iter().find(|user| user.username == username);
    (StatusCode::OK, Json(user))
}
//detete user from users.json by user name
async fn delete_user(Path(username): Path<String>) -> impl IntoResponse {
    let mut file = File::open("users.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut users: Vec<User> = contents
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    users.retain(|user| user.username != username);
    let mut file = File::create("users.json").unwrap();
    file.write_all(serde_json::to_string(&users).unwrap().as_bytes())
        .unwrap();
    (StatusCode::OK, Json(users))
}

#[derive(Serialize, Deserialize, Clone)]
struct Note {
    id: Uuid,
    title: String,
    body: String,
    created_by: String,
}
async fn create_note(Json(note): Json<Note>) -> impl IntoResponse {
    let mut notes: Vec<Note> = Vec::new();

    // Try to read the existing notes from the JSON file
    let read_file = File::open("notes.json");
    if let Ok(mut file) = read_file {
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        notes = serde_json::from_str(&contents).unwrap();
    }

    // Clone the note and add the clone to the list of notes
    let note_clone = note.clone();
    notes.push(note_clone);

    // Write the updated list of notes back to the JSON file
    let mut file = File::create("notes.json").unwrap();
    file.write_all(serde_json::to_string(&notes).unwrap().as_bytes())
        .unwrap();

    (StatusCode::CREATED, Json(note))
}
async fn get_notes() -> impl IntoResponse {
    let mut file = File::open("notes.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let notes: Vec<Note> = serde_json::from_str(&contents).unwrap();
    (StatusCode::OK, Json(notes))
}
async fn get_note(Path(id): Path<Uuid>) -> impl IntoResponse {
    let mut file = File::open("notes.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let notes: Vec<Note> = serde_json::from_str(&contents).unwrap();
    let note = notes.into_iter().find(|note| note.id == id);
    (StatusCode::OK, Json(note))
}
async fn delete_note(Path(id): Path<Uuid>) -> impl IntoResponse {
    let mut file = File::open("notes.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut notes: Vec<Note> = serde_json::from_str(&contents).unwrap();
    notes.retain(|note| note.id != id);
    let mut file = File::create("notes.json").unwrap();
    file.write_all(serde_json::to_string(&notes).unwrap().as_bytes())
        .unwrap();
    (StatusCode::OK, Json(notes))
}
async fn update_note(Path(note_id): Path<Uuid>, Json(note): Json<Note>) -> impl IntoResponse {
    let notes: Vec<Note> = Vec::new();
    let mut file = File::open("notes.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut notes: Vec<Note> = serde_json::from_str(&contents).unwrap();
    let note_index = notes.iter().position(|note| note.id == note_id).unwrap();
    notes[note_index] = note.clone();
    let mut file = File::create("notes.json").unwrap();
    file.write_all(serde_json::to_string(&notes).unwrap().as_bytes())
        .unwrap();
    (StatusCode::OK, Json(notes))
}
