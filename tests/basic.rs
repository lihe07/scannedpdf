use std::path::PathBuf;

fn get_images() -> Vec<PathBuf> {
    let mut images = Vec::new();
    for entry in std::fs::read_dir("./assets").unwrap() {
        let entry = entry.unwrap();
        if entry.file_name().into_string().unwrap().ends_with(".jpg") {
            images.push(entry.path());
        }
    }
    images
}

#[test]
fn images_to_pdf() {
    let config = scannedpdf::PageConfig::new();

    // Delete the file if it exists
    let _ = std::fs::remove_file("./assets/basic.pdf");

    let images = get_images();
    let mut file = scannedpdf::create("./assets/basic.pdf", config, images.len()).unwrap();

    for path in images {
        println!("Adding page from {:?}", path);
        file.add_page_from_path(path, None, Default::default())
            .unwrap();
        println!("Page added");
    }

    println!("Finishing");
    file.finish().unwrap();
    println!("Finished");
}

#[test]
fn with_margin() {
    let config = scannedpdf::PageConfig::new().margin(scannedpdf::Margin::new(100, 100));

    // Delete the file if it exists
    let _ = std::fs::remove_file("./assets/margin.pdf");

    let images = get_images();

    let mut file = scannedpdf::create("./assets/margin.pdf", config, images.len()).unwrap();

    for path in images {
        println!("Adding page from {:?}", path);
        file.add_page_from_path(path, None, Default::default())
            .unwrap();
        println!("Page added");
    }

    println!("Finishing");
    file.finish().unwrap();
    println!("Finished");
}

#[test]
fn with_outlines() {
    let config = scannedpdf::PageConfig::new();

    // Delete the file if it exists
    let _ = std::fs::remove_file("./assets/outlines.pdf");

    let images = get_images();

    let mut file = scannedpdf::create("./assets/outlines.pdf", config, images.len()).unwrap();

    // for entry in std::fs::read_dir("./assets").unwrap() {
    //     let entry = entry.unwrap();
    //     if entry.file_name().into_string().unwrap().ends_with(".jpg") {
    //         println!("Adding page from {:?}", entry.path());
    //         file.add_page_from_path(
    //             entry.path(),
    //             Some(format!(
    //                 "测试中文 图片 {}",
    //                 entry.file_name().to_str().unwrap()
    //             )),
    //             Default::default(),
    //         )
    //         .unwrap();
    //         println!("Page added");
    //     }
    // }

    let mut i = 1;
    for path in images {
        println!("Adding page from {:?}", path);
        file.add_page_from_path(
            path,
            Some(format!("测试中文 图片 {}", i)),
            Default::default(),
        )
        .unwrap();
        println!("Page added");
        i += 1;
    }

    println!("Finishing");
    file.finish().unwrap();
    println!("Finished");
}
