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

fn make_thumbnail(filename: &str) -> MyImage {
    let original = image::open(&Path::new(filename)).unwrap();
    MyImage(original.resize(600, 600, image::imageops::FilterType::Lanczos3))
}

fn main() {
    let content_type = jpeg_type();
    let thumbnail = make_thumbnail("test.jpg");
    
    let closure = move |_: &mut Request| {
        let b : Box<WriteBody + Send> = Box::new(thumbnail.clone());
        Ok(Response::with((content_type.clone(), status::Ok, b)))
    };
    
    Iron::new(closure).http("localhost:3000").unwrap();
}
