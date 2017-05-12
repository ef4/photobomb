#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
use rocket::response::{Response,Content};
use rocket::http::ContentType;

extern crate magick_rust;
use magick_rust::{MagickWand, magick_wand_genesis};
use std::sync::{Once, ONCE_INIT};
use std::io;
use std::vec::Vec;

// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when we're done.
static START: Once = ONCE_INIT;

fn _resize() -> Result<Vec<u8>, &'static str> {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    wand.read_image("test.jpg")?;
    wand.fit(240, 240);
    wand.write_image_blob("JPG")
}

// magick_rust returns string errors, which seems dubious and requires us to convert them.
fn resize() -> io::Result<Vec<u8>> {
    match _resize() {
        Ok(bytes) => Ok(bytes),
        Err(err_string) => Err(io::Error::new(io::ErrorKind::Other, err_string))
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

#[get("/")]
fn index() -> io::Result<Content<Response<'static>>> {
    let bytes = resize()?;
    Ok(Content(ContentType::JPEG, Response::build().sized_body(std::io::Cursor::new(bytes)).finalize()))
}
