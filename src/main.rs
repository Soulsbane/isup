extern crate reqwest;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

extern crate clap;
use clap::{App, Arg};

#[derive(Deserialize)]
struct IsUpResult {
	domain: String,
	port: i32,
	status_code: i32,
	response_ip: String,
	response_code: i32,
	response_time: f32,
}

fn get_url() -> String {
	let matches = App::new("isup")
		.version("0.1.0")
		.author("Paul Crane")
		.about("Checks if a website is up.")
		.arg(
			Arg::with_name("URL")
				.required(true)
				.takes_value(true)
				.index(1)
				.help("url to check"),
		)
		.get_matches();

	let url = matches.value_of("URL").unwrap();
	let final_url = "http://isitup.org/".to_owned() + &url + ".json";

	return final_url;
}

fn get_json_file() -> IsUpResult {
	let url = get_url();
	let json_result = reqwest::get(&url).unwrap().json().unwrap();

	return json_result;
}

fn get_status() {
	let json_result = get_json_file();
	let status_code = json_result.status_code;

	match status_code {
		1 => {
			println!("It's up!");
		}
		2 => {
			println!("It's not just you, it's down!");
		}
		3 => {
			println!("Not a valid domain name.");
		}
		_ => {
			println!("Unknown error: {}", status_code);
		}
	}
}

fn main() {
	get_status();
}
