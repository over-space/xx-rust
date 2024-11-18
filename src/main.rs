use std::{fmt::Debug, io::Write};

// #[derive(Debug)]
struct BufBuilder {
    size: u32,
    buf: Vec<u8>,
}

impl BufBuilder {
    fn new() -> Self {
        BufBuilder {
            size: 1024,
            buf: Vec::with_capacity(1024),
        }
    }
}

impl Debug for BufBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BufBuilder")
            .field("buf", &self.buf)
            .field("size", &self.size)
            .finish()
    }
}

impl Write for BufBuilder {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.buf.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn main() {
    let mut buf = BufBuilder::new();
    buf.write_all(b"hello world!!!").unwrap();
    println!("{:?}", buf)
}
