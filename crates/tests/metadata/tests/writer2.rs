#[test]
fn writer2() {
    use metadata::writer2::*;

    let mut types = std::collections::BTreeMap::new();

    types.insert(
        TypeName::new("TestWindows.Foundation", "Rect"),
        Item::Struct(Struct {
            fields: vec![Field::new("X", Type::F32), Field::new("Y", Type::F32), Field::new("Width", Type::F32), Field::new("Height", Type::F32)],
        }),
    );

    types.insert(
        TypeName::new("TestWindows.Foundation", "AsyncStatus"),
        Item::Enum(Enum {
            constants: vec![Constant::new("Canceled", Value::I32(2)), Constant::new("Completed", Value::I32(1)), Constant::new("Error", Value::I32(3)), Constant::new("Started", Value::I32(0))],
        }),
    );

    write("/git/test.winmd", true, &types);
}
