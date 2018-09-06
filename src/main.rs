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

//1 Website is alive.
//2 Website appears down.
//3 Domain was not valid.

fn main() {
	let url = get_url();
	let json_result: IsUpResult = reqwest::get(&url).unwrap().json().unwrap();

	println!("{}", json_result.status_code);
}
