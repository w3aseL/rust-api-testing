#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;
extern crate fern;
#[macro_use]
extern crate lazy_static;
extern crate multipart;

pub mod logger;
pub mod api;

use crate::logger::setup_logger;
use crate::api::server::build_server;

fn main() {
    if let Err(_) = setup_logger() {
        error!("Failed to set up logger!")
    }

    build_server().launch();
}
