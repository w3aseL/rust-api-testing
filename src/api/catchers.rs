use rocket::{ Request };
use rocket::http::{ Status };
use crate::api::server::{ SimpleResponse, JsonResponse };

#[catch(404)]
pub fn not_found(_: &Request) -> JsonResponse {
	JsonResponse::parse_new(Status::NotFound, SimpleResponse {
		message: "Method not found!"
	})
}