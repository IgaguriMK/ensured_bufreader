## Version 0.2.0

### Breaking Changes

* Change
    - `EnsuredBufReader` now takes buffer type parameter `B`.
* Removed
    - `EnsuredBufReader::with_capacity`
    - `EnsuredBufReader::with_ensured_size`
    - `EnsuredBufReader::fill_buf_to_expected_size`

### New Features

* Now, `EnsuredBufReader` can works with external buffer. For example, you can provide `&mut [u8]` as buffer.
    - From `&mut [u8]`
        + `EnsuredBufReader::from_mut_ref`
        + `EnsuredBufReader::from_mut_ref_and_ensured_size`
    - From any type `B: AsRef<[u8]> + AsMut<[u8]>`
        + `EnsuredBufReader::from_buffer`
        + `EnsuredBufReader::from_buffer_and_ensured_size`
* Instead of `fill_buf_to_expected_size`, new method `fill_buf_to_expected_size` is added.
    - This method returns error when expected size is too large.

## Version 0.1.0

### New Features

* `EnsuredBufReader`, a buffer supports ensures size of buffer contents.