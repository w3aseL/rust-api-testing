use rocket::config::{ Config, Environment, LoggingLevel };
use dotenv;

fn get_env(key: &str) -> String {
	dotenv::var(key).unwrap()
}

fn get_stage() -> Environment {
	match get_env("ENV_STAGE").as_str() {
		"prod" => Environment::Production,
		"dev" => Environment::Development,
		"staging" => Environment::Staging,
		_ => Environment::Development
	}
}

fn get_logging_level() -> LoggingLevel {
	match get_env("API_LOGGING").as_str() {
		"critical" => LoggingLevel::Critical,
		"normal" => LoggingLevel::Normal,
		"debug" => LoggingLevel::Debug,
		"none" => LoggingLevel::Off,
		_ => LoggingLevel::Off
	}
}

pub fn get_config() -> Config {
	let config = Config::build(get_stage())
		.address(get_env("API_IP").as_str())
		.port(get_env("BACKEND_PORT").parse::<u16>().unwrap())
		.log_level(get_logging_level())
		.unwrap();

	config
}
