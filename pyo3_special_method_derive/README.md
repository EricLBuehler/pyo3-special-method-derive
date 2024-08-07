# pyo3-special-method-derive

This crate enables you to automatically derive Python dunder methods for your Rust crate using PyO3.

## Key features
- The following methods may be automatically derived on structs and enums:
    - `__str__`
    - `__repr__`
    - `__dir__`
    - `__getattr__`
    - `__dict__`
- Support for structs and enums (only unit and complex enums due to a PyO3 limitation)
- Support for skipping variants or fields per derive macro with the `#[skip(...)]` attribute
- Automatically skip struct fields which are not `pub`

## Example
```rust
#[pyclass]
#[derive(Dir, Str, Repr)]
struct Person {
    pub name: String,
    occupation: String,
    #[pyo3_smd(skip)]
    pub phone_num: String,
}
```

## PyO3 feature note
To use `pyo3-special-method-derive`, you should enable the `multiple-pymethods` feature on PyO3:
```
pyo3 = { version = "0.22", features = ["multiple-pymethods"] }
```