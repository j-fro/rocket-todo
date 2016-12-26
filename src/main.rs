#![feature(plugin, custom_derive, proc_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate postgres;

#[macro_use]
extern crate serde_derive;

use std::path::{Path, PathBuf};
use rocket::request::Form;
use rocket::response::{Redirect, NamedFile};
use rocket_contrib::{Template, JSON};
use postgres::{Connection, TlsMode};

#[derive(Serialize, Deserialize, Debug, FromForm, Default)]
struct Task {
    id: i32,
    name: String,
    complete: bool
}

const CONN_STRING: &'static str = "postgres://jacobfroman@localhost:5432/rust-todo";

#[get("/")]
fn index() -> Template {
    let tasks = query_tasks();
    Template::render("index", &tasks)
}

#[post("/", data="<task>")]
fn new_task(task: Form<Task>) -> Result<String, String> {
    let task = task.get();
    let response = match insert_task(task) {
        Ok(rows) => Ok(rows.to_string()),
        Err(err) => Err(err.to_string())
    };
    response
}

#[put("/", format="application/json", data="<task>")]
fn edit_task(task: JSON<Task>) -> Result<String, String> {
    let response = match update_task(&task) {
        Ok(rows) => {
            println!("Ok: {:?}", rows);
            Ok(rows.to_string())},
        Err(err) => {
            println!("Error: {:?}", err);
            Err(err.to_string())
        }
    };
    response
}

#[delete("/", format="application/json", data="<task>")]
fn delete_task(task: JSON<Task>) -> Redirect {
    delete_task_from_db(task.id);
    Redirect::to("/")
}

/// Static file handler
#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn query_tasks() -> Vec<Task> {
    let conn = Connection::connect(CONN_STRING, TlsMode::None).unwrap();
    let mut tasks = Vec::new();
    for row in &conn.query("SELECT * FROM tasks ORDER BY complete", &[]).unwrap() {
        let task = Task {
            id: row.get(0),
            name: row.get(1),
            complete: row.get(2)
        };
        tasks.push(task);
    };
    tasks
}

fn insert_task(task: &Task) -> Result<u64, postgres::error::Error> {
    let conn = Connection::connect(CONN_STRING, TlsMode::None).unwrap();
    conn.execute("INSERT INTO tasks (name, complete) VALUES ($1, $2)", &[&task.name, &task.complete])
}

fn update_task(task: &Task) -> Result<u64, postgres::error::Error> {
    let conn = Connection::connect(CONN_STRING, TlsMode::None).unwrap();
    conn.execute("UPDATE tasks SET name=$1, complete=$2 WHERE id=$3",
        &[&task.name, &task.complete, &task.id])
}

fn delete_task_from_db(id: i32) -> () {
    let conn = Connection::connect(CONN_STRING, TlsMode::None).unwrap();
    conn.execute("DELETE FROM tasks WHERE id=$1", &[&id]).unwrap();
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, new_task, edit_task, delete_task, files])
        .launch()
}
