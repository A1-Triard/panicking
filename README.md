![maintenance: actively developed](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# panicking

Provides a `std::thread::panicking` analog available in the `no_std` context.

The crate has two features — `"abort"` and `"std"`, and a final application
should enable at least one of them, otherwise a linkage error will be emitted.
