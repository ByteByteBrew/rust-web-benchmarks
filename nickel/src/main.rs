#![warn(rust_2018_idioms)]

use common::{format_fortunes_html, get_fortunes, get_json_response, HELLO_WORLD};
use nickel::*;

fn main() {
    let mut server = Nickel::new();

    server.get(
        "/plaintext",
        middleware! { |_, mut response|
            response.set(MediaType::Txt);
            HELLO_WORLD.to_string()
        },
    );

    server.get(
        "/json",
        middleware! { |_, mut response|
            response.set(MediaType::Json);
            serde_json::to_string(&get_json_response()).unwrap()
        },
    );

    server.get(
        "/fortunes",
        middleware! { |_, mut response|
            response.set(MediaType::Html);
            format_fortunes_html(get_fortunes())
        },
    );

    server.listen("127.0.0.1:8080").unwrap();
}
