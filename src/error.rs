#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Image(image::ImageError),
    PageOverflow,
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<image::ImageError> for Error {
    fn from(err: image::ImageError) -> Self {
        Error::Image(err)
    }
}
