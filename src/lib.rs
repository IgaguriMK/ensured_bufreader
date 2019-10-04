//! Provides `EnsuredBuffer` that impls [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html).
//!
//!

#![warn(missing_docs)]
use std::error;
use std::fmt;
use std::io::{self, BufRead, Read};

/// Default buffer _capacity_
///
/// Current value is 8 kiB, but may change in the future.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use ensured_bufreader::{DEFAULT_BUFFER_SIZE, EnsuredBufReader};
///
/// fn main() -> std::io::Result<()> {
///     let f = File::open("README.md")?;
///     let r = EnsuredBufReader::new(f);
///
///     assert_eq!(r.get_capacity(), DEFAULT_BUFFER_SIZE);
///     Ok(())
/// }
/// ```
pub const DEFAULT_BUFFER_SIZE: usize = 8 * 1024;

/// Default _ensured_ size.
///
/// Current value is 128 B, but may change in the future.
/// # Examples
///
/// ```
/// use std::fs::File;
/// use ensured_bufreader::{DEFAULT_ENSURED_BYTES, EnsuredBufReader};
///
/// fn main() -> std::io::Result<()> {
///     let f = File::open("README.md")?;
///     let r = EnsuredBufReader::new(f);
///
///     assert_eq!(r.get_ensured_size(), DEFAULT_ENSURED_BYTES);
///     Ok(())
/// }
/// ```
pub const DEFAULT_ENSURED_BYTES: usize = 128;

/// A [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html)er that ensures _ensured_ bytes in buffer.
///
/// `EnsuredBufReader` keeps _ensured_ bytes in buffer if it can read from underlying reader.
/// To fetch bytes into buffer, call `fill_buf()`.
pub struct EnsuredBufReader<R, B>
where
    R: Read,
    B: AsRef<[u8]> + AsMut<[u8]>,
{
    inner: R,
    buf: B,
    pos: usize,
    cap: usize,
    ensured_size: usize,
}

impl<R: Read> EnsuredBufReader<R, Vec<u8>> {
    /// Creates a new `EnsuredBufReader` with a default _capacity_ (`DEFAULT_BUFFER_SIZE`) and a default _ensured_ size (`DEFAULT_ENSURED_BYTES`).
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::File;
    /// use ensured_bufreader::EnsuredBufReader;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let f = File::open("README.md")?;
    ///     let r = EnsuredBufReader::new(f);
    ///     Ok(())
    /// }
    /// ```
    pub fn new(inner: R) -> EnsuredBufReader<R, Vec<u8>> {
        EnsuredBufReader::with_capacity_and_ensured_size(
            DEFAULT_BUFFER_SIZE,
            DEFAULT_ENSURED_BYTES,
            inner,
        )
    }

    /// Creates a new `EnsuredBufReader` with a specified minimal `min_capacity`.
    ///
    /// If specified `min_capacity` is too small, more bigger _capacity_ will be set automatically.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::File;
    /// use ensured_bufreader::EnsuredBufReader;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let f = File::open("README.md")?;
    ///     let r = EnsuredBufReader::with_capacity(1024, f);
    ///     Ok(())
    /// }
    /// ```
    pub fn with_capacity(min_capacity: usize, inner: R) -> EnsuredBufReader<R, Vec<u8>> {
        if min_capacity < 2 * DEFAULT_ENSURED_BYTES {
            EnsuredBufReader::with_capacity_and_ensured_size(
                2 * DEFAULT_ENSURED_BYTES,
                DEFAULT_ENSURED_BYTES,
                inner,
            )
        } else {
            EnsuredBufReader::with_capacity_and_ensured_size(
                min_capacity,
                DEFAULT_ENSURED_BYTES,
                inner,
            )
        }
    }

    /// Creates a new `EnsuredBufReader` with a specified `ensured_size`.
    ///
    /// `ensured_size` must be positive.
    ///
    /// If specified `ensured_size` is larger than `DEFAULT_ENSURED_BYTES / 2`, `capacity` will be set to `2 * ensured_size`.
    ///
    /// # Panics
    ///
    /// Panics if `ensured_size` is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::File;
    /// use ensured_bufreader::EnsuredBufReader;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let f = File::open("README.md")?;
    ///     let r = EnsuredBufReader::with_ensured_size(16, f);
    ///     Ok(())
    /// }
    /// ```
    pub fn with_ensured_size(ensured_size: usize, inner: R) -> EnsuredBufReader<R, Vec<u8>> {
        if ensured_size > DEFAULT_BUFFER_SIZE / 2 {
            EnsuredBufReader::with_capacity_and_ensured_size(2 * ensured_size, ensured_size, inner)
        } else {
            EnsuredBufReader::with_capacity_and_ensured_size(
                DEFAULT_BUFFER_SIZE,
                ensured_size,
                inner,
            )
        }
    }

    /// Creates a new `EnsuredBufReader` with a specified `capacity` and `ensured_size`.
    ///
    /// `capacity` must be larger than or equal to `ensured_size`.
    /// `ensured_size` must be positive.
    ///
    /// # Panics
    ///
    /// Panics if `capacity` is smaller than `ensured_size`.
    /// Panics if `ensured_size` is 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::File;
    /// use ensured_bufreader::EnsuredBufReader;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let f = File::open("README.md")?;
    ///     let r = EnsuredBufReader::with_capacity_and_ensured_size(1024, 32, f);
    ///     Ok(())
    /// }
    /// ```
    pub fn with_capacity_and_ensured_size(
        capacity: usize,
        ensured_size: usize,
        inner: R,
    ) -> EnsuredBufReader<R, Vec<u8>> {
        assert_ne!(ensured_size, 0, "'ensure' must be positive.");
        assert!(
            capacity >= ensured_size,
            "'capacity' ({}) must be larger than or equal to 'ensured_size' ({}).",
            capacity,
            ensured_size
        );
        EnsuredBufReader {
            inner,
            buf: vec![0; capacity],
            pos: 0,
            cap: 0,
            ensured_size,
        }
    }
}

impl<R: Read, B: AsRef<[u8]> + AsMut<[u8]>> EnsuredBufReader<R, B> {
    /// Returns a reference to current buffer.
    /// The buffer filled at least _ensured_ bytes if `EnsuredBufReader` could read from underlying reader.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::File;
    /// use std::io::{self, BufRead};
    /// use ensured_bufreader::EnsuredBufReader;
    ///
    /// fn main() -> io::Result<()> {
    ///     let f = File::open("README.md")?;
    ///     let mut r = EnsuredBufReader::new(f);
    ///
    ///     // Read bytes from file and consume 8 bytes.
    ///     let read_bytes = r.fill_buf()?.to_owned();
    ///     r.consume(8);
    ///     
    ///     // Get buffer.
    ///     // Current buffer should be　8 bytes shorter than `read_bytes`.
    ///     let buf = r.buffer();
    ///     assert_eq!(buf, &read_bytes[8..]);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn buffer(&self) -> &[u8] {
        &self.buf.as_ref()[self.pos..self.cap]
    }

    /// Try to fill buffer and return reference to buffer.
    /// The buffer filled at least `expected_size` bytes if `EnsuredBufReader` could read from underlying reader.
    ///
    /// # Errors
    ///
    /// Returns error that has `.kind() == ErrorKind::InvalidInput` if `expected_size` is larger than _capacity_.
    ///
    /// # Examples
    ///
    /// The buffer will be filled to `expected_size`.
    ///
    /// ```
    /// use std::fs::File;
    /// use std::io::{self, BufRead};
    /// use ensured_bufreader::EnsuredBufReader;
    ///
    /// fn main() -> io::Result<()> {
    ///     let f = File::open("README.md")?;
    ///     let mut r = EnsuredBufReader::with_capacity_and_ensured_size(1024, 1, f);
    ///
    ///     // Fill buffer.
    ///     let read_bytes = r.fill_buf_to_expected_size(512)?;
    ///     assert!(read_bytes.len() >= 512);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// If `expected_size` is larger than _capacity_, error will be returned.
    ///
    /// ```
    /// use std::fs::File;
    /// use std::io::{self, BufRead, ErrorKind};
    /// use ensured_bufreader::EnsuredBufReader;
    ///
    /// fn main() -> io::Result<()> {
    ///     let f = File::open("README.md")?;
    ///     let mut r = EnsuredBufReader::with_capacity_and_ensured_size(512, 1, f);
    ///
    ///     let err = r.fill_buf_to_expected_size(513).unwrap_err();
    ///     assert_eq!(err.kind(), ErrorKind::InvalidInput);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn fill_buf_to_expected_size(&mut self, expected_size: usize) -> io::Result<&[u8]> {
        if self.current_bytes() >= expected_size {
            return Ok(self.buffer());
        }

        if self.buf.as_mut().len() < expected_size {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                ExpectedSizeTooLargeError(),
            ));
        }
        if self.buf.as_mut().len() - self.pos < expected_size {
            self.move_buf_to_head()
        }
        while self.current_bytes() < expected_size {
            let n = self.inner.read(&mut self.buf.as_mut()[self.cap..])?;
            if n == 0 {
                // Reach EOF
                break;
            }
            self.cap += n;
        }

        Ok(self.buffer())
    }

    /// Get current _capacity_ size.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::File;
    /// use ensured_bufreader::EnsuredBufReader;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let f = File::open("README.md")?;
    ///     let r = EnsuredBufReader::new(f);
    ///
    ///     assert_eq!(r.get_capacity(), 8192);
    ///     Ok(())
    /// }
    /// ```
    pub fn get_capacity(&self) -> usize {
        self.buf.as_ref().len()
    }

    /// Get current _ensured_ size.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::fs::File;
    /// use ensured_bufreader::EnsuredBufReader;
    ///
    /// fn main() -> std::io::Result<()> {
    ///     let f = File::open("README.md")?;
    ///     let r = EnsuredBufReader::new(f);
    ///
    ///     assert_eq!(r.get_ensured_size(), 128);
    ///     Ok(())
    /// }
    /// ```
    pub fn get_ensured_size(&self) -> usize {
        self.ensured_size
    }

    /// Returns count of bytes in buffer.
    pub fn current_bytes(&self) -> usize {
        self.cap - self.pos
    }

    fn move_buf_to_head(&mut self) {
        self.buf.as_mut().copy_within(self.pos..self.cap, 0);
        self.cap -= self.pos;
        self.pos = 0;
    }
}

impl<R: Read, B: AsRef<[u8]> + AsMut<[u8]>> Read for EnsuredBufReader<R, B> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.fill_buf()?.read(buf)?;
        self.consume(n);
        Ok(n)
    }
}

impl<R: Read, B: AsRef<[u8]> + AsMut<[u8]>> BufRead for EnsuredBufReader<R, B> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.fill_buf_to_expected_size(self.ensured_size)
    }

    fn consume(&mut self, amt: usize) {
        assert!(
            amt <= self.current_bytes(),
            "the amt must be <= the number of bytes in the buffer returned by fill_buf."
        );
        self.pos += amt;
    }
}

impl<R, B> fmt::Debug for EnsuredBufReader<R, B>
where
    R: Read + fmt::Debug,
    B: AsRef<[u8]> + AsMut<[u8]>,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.debug_struct("EnsuredBufReader")
            .field("reader", &self.inner)
            .field(
                "buffer",
                &format_args!("{}/{}", self.cap - self.pos, self.buf.as_ref().len()),
            )
            .finish()
    }
}

/// An error type may be returned from [`.fill_buf_to_expected_size()`](struct.EnsuredBufReader.html#method.fill_buf_to_expected_size).
#[derive(Debug, Clone, Copy)]
pub struct ExpectedSizeTooLargeError();

impl fmt::Display for ExpectedSizeTooLargeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "internal buffer is too small.")
    }
}

impl error::Error for ExpectedSizeTooLargeError {}
