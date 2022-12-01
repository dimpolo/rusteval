use rusteval::{Interactive, Methods};

#[derive(Interactive)]
struct GenericStruct<T> {
    field: T,
}

#[Methods]
impl<T> GenericStruct<T> {
    fn get(&self) -> &T {
        &self.field
    }
}

#[test]
fn test_generic_struct() {
    let s = GenericStruct { field: 42 };
    s.eval_field("field", &mut |field| {
        assert_eq!(format!("{:?}", field.unwrap()), "42")
    });
    s.eval_method("get", "", &mut |field| {
        assert_eq!(format!("{:?}", field.unwrap()), "42")
    })
}
