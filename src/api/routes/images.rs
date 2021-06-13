use rocket::{ Data };
use rocket::http::{ ContentType, Status };
use image::io::{ Reader as ImageReader };
use std::io::{self, Cursor, Write};

use multipart::mock::StdoutTee;
use multipart::server::Multipart;
use multipart::server::save::Entries;
use multipart::server::save::SaveResult::*;

use crate::api::server::{ JsonResponse, SimpleResponse, ErrorResponse };

// TODO: Rewrite for Multipart to correctly pull image data

fn process_upload(boundary: &str, data: Data) -> io::Result<Vec<u8>> {
    let mut out = Vec::new();

    // saves all fields, any field longer than 10kB goes to a temporary directory
    // Entries could implement FromData though that would give zero control over
    // how the files are saved; Multipart would be a good impl candidate though
    match Multipart::with_body(data.open(), boundary).save().temp() {
        Full(entries) => process_entries(entries, &mut out)?,
        Partial(partial, reason) => {
            writeln!(out, "Request partially processed: {:?}", reason)?;
            if let Some(field) = partial.partial {
                writeln!(out, "Stopped on field: {:?}", field.source.headers)?;
            }

            process_entries(partial.entries, &mut out)?
        },
        Error(e) => return Err(e),
    }

    Ok(out)
}

// having a streaming output would be nice; there's one for returning a `Read` impl
// but not one that you can `write()` to
fn process_entries(entries: Entries, mut out: &mut Vec<u8>) -> io::Result<()> {
    {
        let stdout = io::stdout();
        let tee = StdoutTee::new(&mut out, &stdout);
        entries.write_debug(tee)?;
    }

    writeln!(out, "Entries processed")
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
			let img = match ImageReader::new(Cursor::new(resp)).decode() {
				Ok(img) => img,
				Err(_) => return Err(JsonResponse::parse_new(Status::InternalServerError, SimpleResponse {
					message: "Failed to read image data sent!"
				}))
			};
			
			if let Err(_) = img.save("test.jpg") {
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