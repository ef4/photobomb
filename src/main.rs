use std::path::Path;
use std::io;
use std::error::Error;

extern crate image;

extern crate iron;
use iron::prelude::*;
use iron::mime::Mime;
use iron::status;
use iron::response::WriteBody;

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

fn main() {
    const DEFAULT_SIZE : u32 = 600;
    let content_type = jpeg_type();
    
    let closure = move |req: &mut Request| {
        let size = match req.url.query {
            Some(ref query) =>
                match query.parse::<u32>() {
                    Ok(size) => size,
                    Err(_) => DEFAULT_SIZE
                },
            None => DEFAULT_SIZE
        };
        let thumbnail = make_thumbnail("test.jpg", size);
        let b : Box<WriteBody + Send> = thumbnail;
        Ok(Response::with((content_type.clone(), status::Ok, b)))
    };
    
    Iron::new(closure).http("localhost:3000").unwrap();
}
