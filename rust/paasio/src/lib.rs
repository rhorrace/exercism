use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    buffer: R,
    n_bytes: usize,
    n_reads: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self {
            buffer: wrapped,
            n_bytes: 0,
            n_reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.buffer
    }

    pub fn bytes_through(&self) -> usize {
        self.n_bytes
    }

    pub fn reads(&self) -> usize {
        self.n_reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        match self.buffer.read(buf) {
            Err(e) => Err(e),
            Ok(n) => {
                self.n_reads += 1;
                self.n_bytes += n;
                Ok(n)
            }
        }
    }
}

pub struct WriteStats<W> {
    buffer: W,
    n_bytes: usize,
    n_writes: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self {
            buffer: wrapped,
            n_bytes: 0,
            n_writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.buffer
    }

    pub fn bytes_through(&self) -> usize {
        self.n_bytes
    }

    pub fn writes(&self) -> usize {
        self.n_writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        match self.buffer.write(buf) {
            Err(e) => Err(e),
            Ok(n) => {
                self.n_writes += 1;
                self.n_bytes += n;
                Ok(n)
            }
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.buffer.flush()
    }
}
