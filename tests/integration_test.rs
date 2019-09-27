use std::io::Read;

#[test]
fn read_long() {
    let orig_bytes = util::random_concat(
        &[
            b"a",
            "À".as_bytes(),
            "あ".as_bytes(),
            "\u{1F600}".as_bytes(),
        ],
        0,
        256 * 1024,
    );

    for mut r in util::gen_readers(&orig_bytes) {
        let mut read_bytes = Vec::<u8>::with_capacity(orig_bytes.len());
        let mut read_buf = read_bytes.as_mut_slice();
        loop {
            let n = r.read(read_buf).unwrap();
            if n == 0 {
                break;
            }
            read_buf = &mut read_buf[n..];
        }
        assert_eq!(orig_bytes, read_bytes);
    }
}

mod util {
    use rand::rngs::SmallRng;
    use rand::seq::SliceRandom;
    use rand::SeedableRng;

    use ensured_bufreader::EnsuredBufReader;

    pub fn gen_readers<'a>(
        orig_bytes: &'a [u8],
    ) -> impl Iterator<Item = EnsuredBufReader<&'a [u8]>> {
        vec![
            EnsuredBufReader::new(orig_bytes),
            EnsuredBufReader::with_capacity(1, orig_bytes),
            EnsuredBufReader::with_capacity(512, orig_bytes),
            EnsuredBufReader::with_ensure(0, orig_bytes),
            EnsuredBufReader::with_ensure(1, orig_bytes),
            EnsuredBufReader::with_ensure(16 * 1024, orig_bytes),
            EnsuredBufReader::with_capacity_and_ensure(1, 0, orig_bytes),
            EnsuredBufReader::with_capacity_and_ensure(1, 1, orig_bytes),
            EnsuredBufReader::with_capacity_and_ensure(3, 3, orig_bytes),
        ]
        .into_iter()
    }

    pub fn random_concat(parts: &[&[u8]], seed: u64, min_len: usize) -> Vec<u8> {
        let mut rng = SmallRng::seed_from_u64(seed);
        let mut res = Vec::new();
        while res.len() < min_len {
            res.extend_from_slice(parts.choose(&mut rng).expect("parts should not empty"));
        }
        res
    }
}
