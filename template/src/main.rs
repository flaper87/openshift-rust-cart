#[macro_use] extern crate nickel;

use std::env;
use nickel::Nickel;

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            "Hello world!"
        }
    });

    let addr = format!("{}:{}", env::var("HOST").unwrap(), env::var("PORT").unwrap());
    server.listen(&*addr);
}
