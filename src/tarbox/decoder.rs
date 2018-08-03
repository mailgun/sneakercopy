use std::io::Read;

#[derive(Clone, Debug)]
pub struct Decoder<R: Read> {
    inner: Option<R>,
}