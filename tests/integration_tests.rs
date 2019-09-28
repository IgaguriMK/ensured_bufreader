use std::io::{BufRead, Read};

use ensured_bufreader::EnsuredBufReader;

#[test]
#[should_panic]
fn capacity_is_smaller_than_ensure_not_allowed() {
    let r: &[u8] = &[];
    let _ = EnsuredBufReader::with_capacity_and_ensure(100, 101, r);
}

#[test]
#[should_panic]
fn ensure_is_0_not_allowed_with_ensure() {
    let r: &[u8] = &[];
    let _ = EnsuredBufReader::with_ensure(0, r);
}

#[test]
#[should_panic]
fn ensure_is_0_not_allowed_with_capacity_and_ensure() {
    let r: &[u8] = &[];
    let _ = EnsuredBufReader::with_capacity_and_ensure(1024, 0, r);
}

#[test]
fn read_short() {
    let orig_bytes = "aÀあ\u{1F600}".as_bytes();

    for mut r in util::gen_readers(&orig_bytes) {
        let mut read_bytes = Vec::<u8>::with_capacity(orig_bytes.len());
        let mut read_buf = [0u8; 256];
        loop {
            let n = r.read(&mut read_buf).unwrap();
            eprintln!("{}", n);
            if n == 0 {
                break;
            }
            let bs = &read_buf[..n];
            read_bytes.extend_from_slice(bs);
        }
        assert_eq!(
            orig_bytes,
            read_bytes.as_slice(),
            "with capacity ={}, ensure = {}",
            r.get_capacity(),
            r.get_ensure()
        );
    }
}

#[test]
fn read_long() {
    let short = "aÀあ\u{1F600}".as_bytes();
    let mut orig_bytes = Vec::with_capacity(short.len() * 32 * 1024);
    for _ in 0..32 * 1024 {
        orig_bytes.extend_from_slice(short);
    }

    for mut r in util::gen_readers(&orig_bytes) {
        let mut read_bytes = Vec::<u8>::with_capacity(orig_bytes.len());
        let mut read_buf = [0u8; 256];
        loop {
            let n = r.read(&mut read_buf).unwrap();
            if n == 0 {
                break;
            }
            let bs = &read_buf[..n];
            read_bytes.extend_from_slice(bs);
        }
        assert_eq!(
            orig_bytes,
            read_bytes,
            "with capacity ={}, ensure = {}",
            r.get_capacity(),
            r.get_ensure()
        );
    }
}

#[test]
fn read_expected() {
    let sizes: &[usize] = &[
        1usize,
        2,
        3,
        511,
        512,
        513,
        16 * 1024 - 1,
        16 * 1024,
        16 * 1024 + 1,
        256 * 1024,
    ];

    for &size in sizes {
        let mut orig_bytes = Vec::with_capacity(size);
        orig_bytes.resize(size - 1, 0);
        orig_bytes.push(1);
        assert_eq!(orig_bytes.len(), size);

        for mut r in util::gen_readers(&orig_bytes) {
            let ensure = r.get_ensure();
            loop {
                let buf = r.fill_buf().unwrap();
                let n = buf.len();
                if n == 0 {
                    // EOF
                    break;
                }
                match buf.last() {
                    Some(1) => {} // reach end.
                    _ => assert!(
                        buf.len() >= ensure,
                        "buf.len() (= {}) should be larger than ensure (= {})",
                        buf.len(),
                        ensure
                    ),
                }
                r.consume(n);
            }
        }
    }
}

mod util {
    use ensured_bufreader::EnsuredBufReader;

    pub fn gen_readers<'a>(
        orig_bytes: &'a [u8],
    ) -> impl Iterator<Item = EnsuredBufReader<&'a [u8]>> {
        vec![
            EnsuredBufReader::new(orig_bytes),
            EnsuredBufReader::with_capacity(1, orig_bytes),
            EnsuredBufReader::with_capacity(512, orig_bytes),
            EnsuredBufReader::with_ensure(1, orig_bytes),
            EnsuredBufReader::with_ensure(16 * 1024, orig_bytes),
            EnsuredBufReader::with_capacity_and_ensure(1, 1, orig_bytes),
            EnsuredBufReader::with_capacity_and_ensure(3, 3, orig_bytes),
        ]
        .into_iter()
    }
}