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

fn main() {
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
	let final_url = "http://isitup.org/".to_owned() + url + ".json";
	let json_result: IsUpResult = reqwest::get(&final_url).unwrap().json().unwrap();

	println!("{}", json_result.status_code);
}
