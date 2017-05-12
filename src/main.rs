#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]
extern crate rocket;
use rocket::response::{Stream,Content};
use rocket::http::ContentType;
use rocket::request::{FromFormValue,FromForm};

extern crate magick_rust;
use magick_rust::{MagickWand, magick_wand_genesis};
use std::sync::{Once, ONCE_INIT};
use std::io;
use std::vec::Vec;

// Used to make sure MagickWand is initialized exactly once. Note that we
// do not bother shutting down, we simply exit when we're done.
static START: Once = ONCE_INIT;

fn _resize(filename: &str, fit: FitSize) -> Result<Vec<u8>, &'static str> {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    wand.read_image(filename)?;
    let FitSize(ufit) = fit;
    wand.fit(ufit, ufit);
    wand.write_image_blob("JPG")
}

// magick_rust returns string errors, which seems dubious and requires us to convert them.
fn resize(filename: &str, fit: FitSize) -> io::Result<Vec<u8>> {
    match _resize(filename, fit) {
        Ok(bytes) => Ok(bytes),
        Err(err_string) => Err(io::Error::new(io::ErrorKind::Other, err_string))
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index]).mount("/", routes![index_defaults]).launch();
}

struct FitSize(usize);

impl<'v> FromFormValue<'v> for FitSize {
    type Error = &'v str;
    fn from_form_value(form_value: &'v str) -> Result<FitSize, &'v str> {
        let pixels = usize::from_form_value(form_value)?;
        Ok(FitSize(pixels))
    }
    fn default() -> Option<Self> {
        Some(FitSize(100))
    }
}

#[derive(FromForm)]
struct ImageOptions {
    fit: FitSize
}

fn default_options() -> ImageOptions {
    let mut items = rocket::request::FormItems::from("");
    match ImageOptions::from_form_items(&mut items) {
        Ok(i) => i,
        Err(e) => panic!(e)
    }
}

#[get("/<filename>?<image_options>")]
fn index(filename: &str, image_options: ImageOptions) -> io::Result<Content<Stream<io::Cursor<Vec<u8>>>>> {
    let bytes = resize(filename, image_options.fit)?;
    Ok(Content(ContentType::JPEG, Stream::from(io::Cursor::new(bytes))))
}

#[get("/<filename>")]
fn index_defaults(filename: &str) -> io::Result<Content<Stream<io::Cursor<Vec<u8>>>>> {
    let image_options = default_options();
    let bytes = resize(filename, image_options.fit)?;
    Ok(Content(ContentType::JPEG, Stream::from(io::Cursor::new(bytes))))
}
