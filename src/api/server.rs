use std::io::Cursor;
use rocket::{ Rocket };
use rocket::http::{ ContentType, Status };
use rocket::request::{ Request };
use rocket::response::{ self, Responder, Response };
use rocket_contrib::serve::{ StaticFiles };
use serde::{ Serialize };
use serde_json;

use crate::api::routes::{ index, images };
use crate::api::config::get_config;
use crate::api::logging::Logger;
use crate::api::catchers;

#[derive(Debug, Serialize)]
pub struct SimpleResponse {
	pub message: &'static str
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
	pub error: String
}

#[derive(Debug)]
pub struct JsonResponse {
	status: Status,
	json: String
}

impl JsonResponse {
	pub fn parse_new<T>(status: Status, json: T) -> Self
		where T: Serialize
	{
		Self {
			status,
			json: serde_json::to_string(&json).unwrap()
		}
	}
}

impl<'a> Responder<'a> for JsonResponse {
    fn respond_to(self, _: &Request) -> response::Result<'static> {
		Response::build()
			.status(self.status)
			.header(ContentType::JSON)
			.sized_body(Cursor::new(self.json))
			.ok()
	}
}

pub fn build_server() -> Rocket {
	rocket::custom(get_config())
		.attach(Logger::default())
		.register(catchers![catchers::not_found])
		.mount("/", routes![index::index, index::test, images::upload_image])
		.mount("/", StaticFiles::from("public"))
}