ensured_bufreader
=======

[![ensured_bufreader at crates.io](https://img.shields.io/crates/v/ensured_bufreader.svg)](https://crates.io/crates/ensured_bufreader)
[![ensured_bufreader at docs.rs](https://docs.rs/ensured_bufreader/badge.svg)](https://docs.rs/ensured_bufreader)

## Overview

`ensured_bufreader` provides `EnsuredBufReader` that ensured specified bytes in its buffer (if it can read from underlying reader).

`std::io::BufReader` doesn't read bytes from underlying reader if it has buffered bytes.
This behavior is better if you need buffering for performance.
But, if you need buffering for algorithm such as peeking N bytes, `BufReader` donesn't ensure N bytes in its buffer.

If there are too few bytes in buffer, `EnsuredBufReader` tries to read additional bytes from underlying reader.

## License

`ensured_bufreader` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).