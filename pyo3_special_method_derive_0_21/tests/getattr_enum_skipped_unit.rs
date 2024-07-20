use pyo3::{pyclass, Python};
use pyo3_special_method_derive_0_21::Getattr;

#[derive(PartialEq)]
#[pyclass]
#[derive(Getattr)]
enum Tester {
    Alpha,
    #[pyo3_smd(skip)]
    Beta,
}

#[test]
fn test_get_attr_exception() {
    pyo3::prepare_freethreaded_python();

    let res = Tester::Beta.__getattr__("z".to_string()).unwrap_err();

    let correct_err = Python::with_gil(|py| {
        &res.value_bound(py).to_string() == "'Tester.Beta' has no attribute 'z'"
    });
    assert!(correct_err);
}
