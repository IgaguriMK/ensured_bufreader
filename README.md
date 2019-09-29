ensured_bufreader
=======

[![ensured_bufreader at crates.io](https://img.shields.io/crates/v/ensured_bufreader.svg)](https://crates.io/crates/ensured_bufreader)
[![ensured_bufreader at docs.rs](https://docs.rs/ensured_bufreader/badge.svg)](https://docs.rs/ensured_bufreader)
[![Build Status (master)](https://travis-ci.org/IgaguriMK/ensured_bufreader.svg?branch=master)](https://travis-ci.org/IgaguriMK/ensured_bufreader)
[![Build Status (dev)](https://travis-ci.org/IgaguriMK/ensured_bufreader.svg?branch=dev)](https://travis-ci.org/IgaguriMK/ensured_bufreader)

## Overview

`ensured_bufreader` provides `EnsuredBufReader` that _ensured_ length bytes in its buffer if it can read from underlying reader.

`std::io::BufReader` doesn't read bytes from underlying reader if it has buffered bytes.
This behavior is better if you need buffering for performance.
But, if you need buffering for algorithm such as peeking N bytes, `BufReader` donesn't ensure N bytes in its buffer.

If there are too few bytes in buffer when `.fill_buf()` called, `EnsuredBufReader` tries to read additional bytes from underlying reader and keep `.fill_buf()?.len() > ensured`.

## Comparition with other crates

`buffered-reader` provides same functionality as this crate.
But there is some differences.

|   | `ensured_bufreader` | `buffered-reader = "0.10.0"` |
|:-:|:--|:--|
| Implementation strategy | Uses standard trait `BufRead` and simple wrapper struct | Provides new trait and some implementaions |
| When returns error | Immediately | Saved and returned when read position error occuered |
| License | MIT or Apache-2.0 (permissive) | GPL-3.0 (copyleft) |

## License

`ensured_bufreader` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).