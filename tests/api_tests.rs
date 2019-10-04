use std::io::{BufRead, ErrorKind, Read};

use ensured_bufreader::{EnsuredBufReader, DEFAULT_ENSURED_BYTES};

#[test]
#[should_panic]
fn capacity_is_smaller_than_ensure_not_allowed() {
    let r: &[u8] = &[];
    let _ = EnsuredBufReader::with_capacity_and_ensured_size(100, 101, r);
}

#[test]
#[should_panic]
fn ensure_is_0_not_allowed_with_capacity_and_ensure() {
    let r: &[u8] = &[];
    let _ = EnsuredBufReader::with_capacity_and_ensured_size(1024, 0, r);
}

#[test]
fn read_long() {
    let short = "aÀあ\u{1F600}".as_bytes();
    let mut input = Vec::with_capacity(short.len() * 32 * 1024);
    for _ in 0..32 * 1024 {
        input.extend_from_slice(short);
    }

    let mut r = EnsuredBufReader::new(input.as_slice());

    let mut read_bytes = Vec::<u8>::with_capacity(input.len());
    let mut read_buf = [0u8; 256];

    loop {
        let n = r.read(&mut read_buf).unwrap();

        let bs = &read_buf[..n];
        read_bytes.extend_from_slice(bs);

        if n < DEFAULT_ENSURED_BYTES {
            break;
        }
    }

    assert_eq!(
        input,
        read_bytes,
        "with capacity ={}, ensure = {}",
        r.get_capacity(),
        r.get_ensured_size()
    );
}

#[test]
fn fill_buf_short() {
    let input = "aÀあ\u{1F600}".as_bytes();

    let mut r = EnsuredBufReader::new(input);
    let mut read_bytes = Vec::<u8>::with_capacity(input.len());

    loop {
        let buf = r.fill_buf().unwrap();
        read_bytes.extend_from_slice(buf);
        let n = buf.len();
        r.consume(n);

        if n < DEFAULT_ENSURED_BYTES {
            break;
        }
    }

    assert_eq!(
        input,
        read_bytes.as_slice(),
        "with capacity ={}, ensure = {}",
        r.get_capacity(),
        r.get_ensured_size()
    );
}

#[test]
fn fill_buf_long() {
    let short = "aÀあ\u{1F600}".as_bytes();
    let mut input = Vec::with_capacity(short.len() * 32 * 1024);
    for _ in 0..32 * 1024 {
        input.extend_from_slice(short);
    }

    let mut r = EnsuredBufReader::new(input.as_slice());

    let mut read_bytes = Vec::<u8>::with_capacity(input.len());

    loop {
        let buf = r.fill_buf().unwrap();
        read_bytes.extend_from_slice(buf);
        let n = buf.len();
        r.consume(n);

        if n < DEFAULT_ENSURED_BYTES {
            break;
        }
    }

    assert_eq!(
        input,
        read_bytes,
        "with capacity ={}, ensure = {}",
        r.get_capacity(),
        r.get_ensured_size()
    );
}

#[test]
fn works_with_given_buffer() {
    let short = "aÀあ\u{1F600}".as_bytes();
    let mut input = Vec::with_capacity(short.len() * 32 * 1024);
    for _ in 0..32 * 1024 {
        input.extend_from_slice(short);
    }

    let mut buf = [0u8; 2 * DEFAULT_ENSURED_BYTES];
    let mut r = EnsuredBufReader::from_mut_ref(&mut buf, input.as_slice());

    let mut read_bytes = Vec::<u8>::with_capacity(input.len());

    loop {
        let buf = r.fill_buf().unwrap();
        read_bytes.extend_from_slice(buf);
        let n = buf.len();
        r.consume(n);

        if n < DEFAULT_ENSURED_BYTES {
            break;
        }
    }

    assert_eq!(
        input,
        read_bytes,
        "with capacity ={}, ensure = {}",
        r.get_capacity(),
        r.get_ensured_size()
    );
}

#[test]
fn fill_buf_to_expected_size_returns_error_when_expected_size_is_too_large() {
    let short = "aÀあ\u{1F600}".as_bytes();
    let mut input = Vec::with_capacity(short.len() * 32 * 1024);
    for _ in 0..256 {
        input.extend_from_slice(short);
    }

    let mut r = EnsuredBufReader::with_capacity_and_ensured_size(512, 1, input.as_slice());

    let err = r.fill_buf_to_expected_size(513).unwrap_err();
    assert_eq!(err.kind(), ErrorKind::InvalidInput);
}
