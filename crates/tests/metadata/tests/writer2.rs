#[test]
fn writer2() {
    use metadata::reader;
    use metadata::writer2::*;

    let files = vec![reader::File::new("../../libs/metadata/default/Windows.winmd").unwrap(), metadata::reader::File::new("../../libs/metadata/default/Windows.Win32.winmd").unwrap(), metadata::reader::File::new("../../libs/metadata/default/Windows.Win32.Interop.winmd").unwrap()];
    let reader = &reader::Reader::new(&files);
    let mut types = std::collections::BTreeMap::new();

    types.insert( TypeName::new("TestWindows.Foundation", "Rect"), Item::Struct(Struct {
        winrt: true,
        fields: vec![
            // TODO: Field::new("X", Type::F32) ?
            Field { name: "X".to_string(),  ty: Type::F32 },
            Field { name: "Y".to_string(),  ty: Type::F32 },
            Field { name: "Width".to_string(),  ty: Type::F32 },
            Field { name: "Height".to_string(),  ty: Type::F32 },
        ]
    }));

    write("/git/test.winmd", reader, &types);
}

