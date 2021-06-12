use crate::api::server::{ JsonResponse, SimpleResponse };
use rocket::http::{ Status };

use serde::{ Serialize };

#[derive(Debug, Serialize)]
struct TestResponse {
	message: &'static str,
	number: i32,
	numbers: Vec<i32>
}

#[get("/")]
pub fn index() -> JsonResponse {
	JsonResponse::parse_new(Status::Ok, SimpleResponse {
		message: "Hello, world!"
	})
}

#[get("/test")]
pub fn test() -> JsonResponse {
	JsonResponse::parse_new(Status::Ok, TestResponse {
		message: "This is a test response!",
		number: 1,
		numbers: vec![ 1, 2, 3, 4 ]
	})
}