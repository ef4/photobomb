#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
use rocket::response::{Stream,Content};
use rocket::http::ContentType;

extern crate magick_rust;
use magick_rust::{MagickWand, magick_wand_genesis};
use std::sync::{Once, ONCE_INIT};
use std::io;
use std::vec::Vec;

// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when we're done.
static START: Once = ONCE_INIT;

fn _resize(filename: &str, fit: usize) -> Result<Vec<u8>, &'static str> {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    wand.read_image(filename)?;
    wand.fit(fit, fit);
    wand.write_image_blob("JPG")
}

// magick_rust returns string errors, which seems dubious and requires us to convert them.
fn resize(filename: &str, fit: usize) -> io::Result<Vec<u8>> {
    match _resize(filename, fit) {
        Ok(bytes) => Ok(bytes),
        Err(err_string) => Err(io::Error::new(io::ErrorKind::Other, err_string))
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

#[get("/")]
fn index() -> io::Result<Content<Stream<io::Cursor<Vec<u8>>>>> {
    let bytes = resize("test.jpg", 300)?;
    Ok(Content(ContentType::JPEG, Stream::from(io::Cursor::new(bytes))))
}
