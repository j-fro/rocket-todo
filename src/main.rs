#![feature(plugin, custom_derive)]
#![feature(proc_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate postgres;

#[macro_use]
extern crate serde_derive;

use rocket::request::Form;
use rocket::response::Redirect;
use rocket_contrib::Template;
use postgres::{Connection, TlsMode};

#[derive(Serialize)]
#[derive(Debug)]
#[derive(FromForm)]
struct Task {
    name: String,
    complete: bool
}

#[get("/")]
fn index() -> Template {
    let tasks = query_tasks();
    Template::render("index", &tasks)
}

#[post("/", data="<task>")]
fn new_task(task: Form<Task>) -> Redirect {
    let task = task.get();
    println!("{:?}", task);
    insert_task(task);
    Redirect::to("/")
}

fn query_tasks() -> Vec<Task> {
    let conn = Connection::connect("postgres://jacobfroman@localhost:5432/rust-todo", TlsMode::None).unwrap();
    let mut tasks = Vec::new();
    for row in &conn.query("SELECT * FROM tasks", &[]).unwrap() {
        let task = Task {
            name: row.get(1),
            complete: row.get(2)
        };
        tasks.push(task);
    };
    tasks
}

fn insert_task(task: &Task) -> () {
    let conn = Connection::connect("postgres://jacobfroman@localhost:5432/rust-todo", TlsMode::None).unwrap();
    conn.execute("INSERT INTO tasks (name, complete) VALUES ($1, $2)", &[&task.name, &task.complete]).unwrap();
}

fn main() {
    rocket::ignite().mount("/", routes![index, new_task]).launch()
}
