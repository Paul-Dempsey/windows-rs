#[test]
fn writer2() {
    use metadata::writer::*;

    let mut types = Vec::new();

    types.push(
        Item::Struct(Struct {
            name: ("TestWindows.Foundation".into(), "Rect".into()),
            fields: vec![Field::new("X", Type::F32), Field::new("Y", Type::F32), Field::new("Width", Type::F32), Field::new("Height", Type::F32)],
        }),
    );

    types.push(
        Item::Enum(Enum {
            name: ("TestWindows.Foundation".into(), "AsyncStatus".into()),
            constants: vec![Constant::new("Canceled", Value::I32(2)), Constant::new("Completed", Value::I32(1)), Constant::new("Error", Value::I32(3)), Constant::new("Started", Value::I32(0))],
        }),
    );

    write("/git/test.winmd", true, &[], &types);
}
