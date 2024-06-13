use pyo3::pyclass;
use pyo3_special_method_derive::StrReprHelper;

#[pyclass]
#[derive(StrReprHelper)]
#[allow(dead_code)]
enum Tester {
    Alpha {
        x: u32,
    },
    Beta {
        x: u32,
        y: u32,
    },
    #[skip]
    Gamma {
        x: u32,
        y: u32,
        z: u32,
    },
}

#[test]
fn test_with_str() {
    let res = Tester::Alpha { x: 12 }.__str__();
    assert_eq!("Tester.Alpha(x=12)", &res);
}

#[test]
fn test_with_repr() {
    let res = Tester::Gamma { x: 1, y: 2, z: 3 }.__repr__();
    assert_eq!("<variant skipped>", &res);
}
