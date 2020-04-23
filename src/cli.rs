use rust_test::http;

use seahorse::{Context, Command, Flag, FlagType};

pub fn get_public_ip_action(c: &Context) {
    let default_url = "https://httpbin.org/ip";

    let url = match c.string_flag("url") {
        Some(s) => {
            println!("Using custom url from --url: {}", s);
            s
        },
        None => {
            println!("Using default URL {}", default_url);
            default_url.to_string()
        },
    };

    let result = match http::get::http_get(&url) {
        Ok(response) => response,
        Err(error) => {
            eprintln!("Problem doing the HTTP GET: {:?}", error);
            std::process::exit(1);
        },
    };

    let result = match http::common::check_error(result) {
        Ok(response) => response,
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    };

    let json = match http::common::parse_json(result) {
        Ok(result) => result,
        Err(error) => {
            eprintln!("Problem parsing IP JSON: {:?}", error);
            std::process::exit(1);
        },
    };

    if json["origin"].is_string() {
        println!("My public IP is: {}", json["origin"]);
        std::process::exit(0);
    } else {
        eprintln!("Cannot find key origin from response: {:#?}", json);
        std::process::exit(1);
    }
}

pub fn get_public_ip_command() -> Command {
    Command::new()
        .name("get_public_ip")
        .usage("rust_test get_public_ip")
        .action(get_public_ip_action)
        .flag(Flag::new(
                "url",
                "--url <url>",
                FlagType::String,
            )
        )
}
