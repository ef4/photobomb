#![feature(slice_patterns, advanced_slice_patterns)]
use std::path::Path;
use std::io;
use std::error::Error;

extern crate image;

extern crate iron;
use iron::prelude::*;
use iron::mime::Mime;
use iron::status;
use iron::response::WriteBody;

extern crate urlencoded;
use urlencoded::UrlEncodedQuery;


#[derive(Clone)]
struct MyImage(image::DynamicImage);

impl WriteBody for MyImage {
    fn write_body(&mut self, res: &mut iron::response::ResponseBody) -> io::Result<()> {
        let &mut MyImage(ref i) = self;
        match i.save(res, image::ImageFormat::JPEG) {
            Ok(value) => Ok(value),
            Err(image::ImageError::IoError(err)) => Err(err),
            Err(image_err) => Err(io::Error::new(io::ErrorKind::Other, image_err.description()))
        }
    }
}

fn jpeg_type() -> Mime {
    "image/jpeg".parse::<Mime>().unwrap()
}

fn make_thumbnail(filename: &str, size: u32) -> Box<MyImage> {
    let original = image::open(&Path::new(filename)).unwrap();
    Box::new(MyImage(original.resize(size, size, image::imageops::FilterType::Lanczos3)))
}

fn get_integer_param(req: &mut Request, key : &str, default_value : u32) -> u32 {
    req.get_ref::<UrlEncodedQuery>().ok().and_then(|hashmap| {
        hashmap.get(key)
    }).and_then(|values| {
        values.get(0)
    }).and_then(|string_value| {
        string_value.parse::<u32>().ok()
    }).unwrap_or(default_value)
}


fn main() {
    let content_type = jpeg_type();
    
    let closure = move |req: &mut Request| {

        let size = get_integer_param(req, "size", 600);
        let thumbnail = make_thumbnail("test.jpg", size);
        let b : Box<WriteBody + Send> = thumbnail;
        Ok(Response::with((content_type.clone(), status::Ok, b)))
    };
    
    Iron::new(closure).http("localhost:3000").unwrap();
}
