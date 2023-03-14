use std::io;
use std::io::BufRead;

#[derive(Default)]
pub struct LineReader(String);

impl LineReader {
    pub fn read(&mut self, buf_read: &mut impl BufRead) -> Result<&str, io::Error> {
        let LineReader(buf) = self;
        buf.clear();
        buf_read.read_line(buf)?;
        Ok(buf.as_str())
    }
}
