#[test]
fn writer() {
    let temp_file = std::env::temp_dir().join("test2.winmd");
    {
        use metadata::writer2 as writer;

        let buffer = writer::write("module", &[], &[]);
        std::fs::write(temp_file, buffer);
    }
}
