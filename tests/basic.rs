

#[test]
fn images_to_pdf() {
    let config = scannedpdf::Config::new()
        .memory(true) // Force to store in memory
        .resize(true); // Auto resize image

    let mut file = scannedpdf::create("./assets/test.pdf", config).unwrap();

    for entry in std::fs::read_dir("./assets").unwrap() {
        let entry = entry.unwrap();
        if entry.file_name().into_string().unwrap().ends_with(".jpg") {
            file.add_page_by_path(entry.path(), Default::default());
        }
    }
}