#[test]
fn writer() {
    let temp_file = std::env::temp_dir().join("test2.winmd");
    {
        use metadata::writer2::*;

        let mut items = vec![];

        items.push(Item::Struct(Struct {
            namespace: "Test".to_string(),
            name: "Rect".to_string(),
            fields: vec![],
            winrt: true,
        }));

        let buffer = write("module", &items, &[]);
        std::fs::write(temp_file, buffer);
    }
}
