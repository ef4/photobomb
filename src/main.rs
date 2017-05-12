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
use std::path::PathBuf;
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
fn resize(path: PathBuf, fit: FitSize) -> io::Result<Vec<u8>> {
    match path.to_str() {
        Some(filename) => {
            match _resize(filename, fit) {
                Ok(bytes) => Ok(bytes),
                Err(err_string) => Err(io::Error::new(io::ErrorKind::Other, err_string))
            }
        },
        None => Err(io::Error::new(io::ErrorKind::Other, "Not a valid path"))
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index]).mount("/", routes![index_defaults]).launch();
}

struct FitSize(usize);

impl<'v> FromFormValue<'v> for FitSize {
    type Error = &'v str;
    fn from_form_value(form_value: &'v str) -> Result<FitSize, &'v str> {
        Ok(FitSize(usize::from_form_value(form_value)?))
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

type ImageResponse = io::Result<Content<Stream<io::Cursor<Vec<u8>>>>>;

#[get("/<filename..>?<image_options>")]
fn index(filename: PathBuf, image_options: ImageOptions) -> ImageResponse {
    transform_image(filename, image_options)
}

#[get("/<filename..>")]
fn index_defaults(filename: PathBuf) -> ImageResponse {
    transform_image(filename, default_options())
}

fn transform_image(filename: PathBuf, image_options: ImageOptions) -> ImageResponse {
    let bytes = resize(filename, image_options.fit)?;
    Ok(Content(ContentType::JPEG, Stream::from(io::Cursor::new(bytes))))
}
