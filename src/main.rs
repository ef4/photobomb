extern crate tiny_http;
use std::io::fs;

fn server() {
    let server = tiny_http::ServerBuilder::new().with_port(8181).build().unwrap();

    for request in server.incoming_requests() {
        println!("received request! method: {}, url: {}, headers: {}",
                 request.get_method(),
                 request.get_url(),
                 request.get_headers()
                 );

        let response = tiny_http::Response::from_string("hello world".to_string());
        request.respond(response);
    }
}

fn walk() {
    let path = Path::new("/Users/edward/Desktop/October 1988 - June 1989");
    match fs::walk_dir(&path) {
        Ok(ref mut dirs) => show_dirs(dirs),
        Err(err) => println!("{}", err)
    }
}

fn show_dirs(dirs : &mut fs::Directories) {
    for dir in *dirs {
        println!("{}", dir.display());
    }
}

fn main() {
    walk();
    server();
}
