#![feature(time2)]
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

use std::time::SystemTime;

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

fn make_thumbnail(original: image::DynamicImage, size: u32) -> Box<MyImage> {
    Box::new(MyImage(original.resize(size, size, image::imageops::FilterType::Lanczos3)))
}

fn log_time<T, R>(label: &str, operation: T) -> R
    where T : Fn() -> R {
        let start = SystemTime::now();
        let output = operation();
        let duration = start.elapsed().unwrap();
        println!("{} {}ms", label, duration.as_secs() * 1000 + (duration.subsec_nanos() / 1000000) as u64);
        output
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
    let original = log_time("load original", || image::open(&Path::new("test.jpg")).unwrap());
    let closure = move |req: &mut Request| {
        let size = get_integer_param(req, "size", 600);
        let thumbnail = log_time("make_thumbnail", || make_thumbnail(log_time("original.clone", ||original.clone()), size));

        // This extra annotation seems like it should not be
        // necessary, but it is.
        let b : Box<WriteBody + Send> = thumbnail;

        Ok(Response::with((content_type.clone(), status::Ok, b)))
    };
    
    Iron::new(closure).http("localhost:3000").unwrap();
}
