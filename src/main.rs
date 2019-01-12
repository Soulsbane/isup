#[macro_use]
extern crate human_panic;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;

use clap::{App, Arg};
use failure::{Error, ResultExt};

#[derive(Deserialize)]
struct IsUpResult {
	// INFO: Rust will warn for any struct member that isn't used with serde. This prevents those warnings.
	#[serde(default)]
	_domain: String,
	#[serde(default)]
	_port: i32,
	status_code: i32,
	#[serde(default)]
	_response_ip: String,
	#[serde(default)]
	_response_code: i32,
	#[serde(default)]
	_response_time: f32,
}

fn get_url() -> String {
	let matches = App::new(crate_name!())
		.version(crate_version!())
		.author(crate_authors!("\n"))
		.about(crate_description!())
		.arg(
			Arg::with_name("URL")
				.required(true)
				.takes_value(true)
				.index(1)
				.help("URL of the website to check"),
		)
		.get_matches();

	let url = matches.value_of("URL").unwrap();
	let final_url = "http://isitup.org/".to_owned() + &url + ".json";

	return final_url;
}

fn get_json_file() -> Result<IsUpResult, Error> {
	let url = get_url();
	let json_result: IsUpResult = reqwest::get(&url)
		.context("Failed to connect to that URL")?
		.json()
		.context("Could not parse JSON response.")?;

	return Ok(json_result);
}

fn get_status() {
	match get_json_file() {
		Ok(result) => println!("{}", get_status_message(result.status_code)),
		Err(e) => eprintln!("{:?}", e),
	}
}

fn get_status_message(status_code: i32) -> String {
	match status_code {
		1 => {
			return "It's up!".to_owned();
		}
		2 => {
			return "It's not just you, it's down!".to_owned();
		}
		3 => {
			return "Not a valid domain name.".to_owned();
		}
		_ => {
			return "Unknown error!".to_owned();
		}
	}
}

fn main() {
	setup_panic!();
	get_status();
}
