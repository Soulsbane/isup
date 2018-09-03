extern crate reqwest;
#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Deserialize)]
struct IsUpResult {
    domain: String,
    port: i32,
    status_code: i32,
    response_ip: String,
    response_code: i32,
    response_time: f32
}

fn main() {
	let url = "google.com";
	let final_url = "http://isitup.org/".to_owned() + url + ".json";
	let json_result: IsUpResult = reqwest::get(&final_url).unwrap().json().unwrap();
	println!("{}", json_result.status_code);

	//let body = reqwest::get(&final_url).unwrap()
    //	.text().unwrap();

	//println!("{}", body);

}
