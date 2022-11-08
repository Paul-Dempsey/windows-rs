#[test]
fn writer() {
    let temp_file = std::env::temp_dir().join("test2.winmd");
    {
        use metadata::writer2::*;

        let mut items = vec![];

        items.push(Item::Struct(Struct {
            namespace: "Test".to_string(),
            name: "Inner".to_string(),
            fields: vec![Field::new("Value32", Type::F32), Field::new("Value64", Type::F64)],
            winrt: true,
        }));

        items.push(Item::Struct(Struct { namespace: "Test".to_string(), name: "Outer".to_string(), fields: vec![Field::new("Value", Type::new("Test", "Inner"))], winrt: true }));

        let buffer = write("module", &items, &[]);
        std::fs::write(temp_file, buffer);
    }
}
