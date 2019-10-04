## Version 0.2.0

### Breaking Changes

* Change
    - `EnsuredBufReader` now takes buffer type parameter `B`.
* Removed
    - `EnsuredBufReader::with_capacity`
    - `EnsuredBufReader::with_ensured_size`
    - `EnsuredBufReader::fill_buf_to_expected_size`