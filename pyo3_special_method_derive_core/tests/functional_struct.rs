use pyo3::pyclass;
use pyo3_special_method_derive::{Dir, Getattr, Repr, Str};

#[pyclass]
#[derive(Dir, Str, Repr)]
#[allow(dead_code)]
struct WithFields {
    pub dora: u32,
    pub my: String,
    pub name: f32,
}

#[test]
fn test_with_dir() {
    let dir = WithFields {
        dora: 0,
        my: "".to_string(),
        name: 0.0,
    }
    .__dir__();
    assert_eq!(
        vec!["dora".to_string(), "my".to_string(), "name".to_string()],
        dir
    );
}

#[test]
fn test_with_str() {
    let pi = std::f32::consts::PI;
    let res = WithFields {
        dora: 299792458,
        my: "Hello world".to_string(),
        name: pi,
    }
    .__str__();
    assert_eq!(
        format!("WithFields(dora=299792458, my=\"Hello world\", name={pi})"),
        res
    );
}

#[test]
fn test_with_repr() {
    let pi = std::f32::consts::PI;
    let res = WithFields {
        dora: 299792458,
        my: "Hello world".to_string(),
        name: pi,
    }
    .__repr__();
    assert_eq!(
        format!("WithFields(dora=299792458, my=\"Hello world\", name={pi})"),
        res
    );
}

#[pyclass]
#[derive(Dir)]
#[allow(dead_code)]
struct UnitNoFields;

#[test]
fn test_no_fields() {
    let fields: Vec<String> = UnitNoFields.__dir__();
    assert_eq!(Vec::<String>::new(), fields);
}

#[pyclass]
#[derive(Getattr)]
#[allow(dead_code)]
struct PrivateFields {
    x: u32,
}
