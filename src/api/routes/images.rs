use rocket::{ Data };
use rocket::http::{ ContentType, Status };
use image::io::{ Reader as ImageReader };
use image::imageops;
use std::fmt::Error;
use std::io::{self, Cursor, Read, Write};

use multipart::server::Multipart;

use crate::api::server::{ JsonResponse, SimpleResponse, ErrorResponse };

// TODO: Rewrite for Multipart to correctly pull image data

fn process_upload(boundary: &str, data: Data) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();

    // saves all fields, any field longer than 10kB goes to a temporary directory
    // Entries could implement FromData though that would give zero control over
    // how the files are saved; Multipart would be a good impl candidate though
    let mut mp = Multipart::with_body(data.open(), boundary);
	mp.foreach_entry(|mut entry| match &*entry.headers.name {
		"image" => {
			info!("Received file with name {}", entry.headers.filename.unwrap());

			entry.data.read_to_end(&mut out).expect("No file found!");
		},
		_ => {
			info!("Found an unknown entry...")
		}
	})
	.expect("Failed to iterate through entries!");

    Ok(out)
}

#[post("/upload-image", data="<data>")]
pub fn upload_image(content_type: &ContentType, data: Data) -> Result<JsonResponse, JsonResponse> {
	if !content_type.is_form_data() {
		return Err(JsonResponse::parse_new(Status::BadRequest, SimpleResponse {
			message: "Content-Type is not multipart/form-data!"
		}))
	}

	let (_, boundary) = content_type.params().find(|&(k, _)| k == "boundary").ok_or_else(
		|| JsonResponse::parse_new(Status::BadRequest, SimpleResponse {
			message: "Content-Type: \"multipart/form-data\" boundary param not provided!"
		})
	)?;

	match process_upload(boundary, data) {
        Ok(resp) => {
			let mut img = match ImageReader::new(Cursor::new(resp)).with_guessed_format().expect("Failed to guess format!").decode() {
				Ok(img) => img,
				Err(err) => {
					return Err(JsonResponse::parse_new(Status::InternalServerError, ErrorResponse {
						error: err.to_string()
					}))
				}
			};

			img.invert();

			if let Err(_) = img.save("test_blur.jpg") {
				return Err(JsonResponse::parse_new(Status::InternalServerError, SimpleResponse {
					message: "Failed to save the file!"
				}))
			}

			Ok(JsonResponse::parse_new(Status::Ok, SimpleResponse {
				message: "Saved test image!"
			}))
		},
        Err(err) => Err(JsonResponse::parse_new(Status::InternalServerError, ErrorResponse {
			error: err.to_string()
		}))
		
    }
}