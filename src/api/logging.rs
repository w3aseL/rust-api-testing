use rocket::{ Rocket, Request, Data, Response };
use rocket::http::{ Method, Status, ContentType };
use rocket::fairing::{ Fairing, Info, Kind };

#[derive(Default)]
pub struct Logger {}

impl Fairing for Logger {
	fn info(&self) -> Info {
		Info {
			name: "Logger Fairing",
			kind: Kind::Launch | Kind::Request | Kind::Response
		}
	}

	fn on_launch(&self, rocket: &Rocket) { 
		let config = rocket.config();

		info!("Environment Stage: {}", config.environment.to_string());
		info!("Server launched on {}:{}", config.address, config.port);
	}

	fn on_request(&self, request: &mut Request<'_>, data: &Data) { 

	}

    fn on_response(&self, request: &Request<'_>, response: &mut Response<'_>) { 
		info!(
			"{} {} {}",
			response.status(),
			request.method(),
			request.uri()
		)
	}
}