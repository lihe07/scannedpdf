use std::{
    io::{BufWriter, Cursor},
    path::PathBuf,
};

use criterion::{criterion_group, criterion_main, Criterion};

fn basic_scannedpdf() {
    let config = scannedpdf::PageConfig::new();

    let mut buf = Vec::new();

    // let mut file = scannedpdf::create("./assets/basic.pdf", config, 3).unwrap();
    let mut file = scannedpdf::PDF::create(Cursor::new(&mut buf), config, 3).unwrap();

    for entry in std::fs::read_dir("./assets").unwrap() {
        let entry = entry.unwrap();
        if entry.file_name().into_string().unwrap().ends_with(".jpg") {
            file.add_page_from_path(entry.path(), None, Default::default())
                .unwrap();
        }
    }
    file.finish().unwrap();
}

fn basic_printpdf() {
    // Do the same thing with printpdf
    let (doc, page, layer) = printpdf::PdfDocument::new(
        "PDF_Document_title",
        printpdf::Mm(210.0),
        printpdf::Mm(297.0),
        "Layer 1",
    );
    let mut current_layer = doc.get_page(page).get_layer(layer);

    let mut images = Vec::new();
    for entry in std::fs::read_dir("./assets").unwrap() {
        let entry = entry.unwrap();
        if entry.file_name().into_string().unwrap().ends_with(".jpg") {
            images.push(entry.path());
        }
    }

    for (i, path) in images.iter().enumerate() {
        let d_image = printpdf::image_crate::open(path).unwrap();

        let image = printpdf::Image::from_dynamic_image(&d_image);
        image.add_to_layer(
            current_layer,
            printpdf::ImageTransform {
                translate_x: None,
                translate_y: None,
                rotate: None,
                scale_x: None,
                scale_y: None,
                dpi: Some(300.0),
            },
        );

        if i < images.len() - 1 {
            let (page, layer) = doc.add_page(printpdf::Mm(210.0), printpdf::Mm(297.0), "Layer 1");
            current_layer = doc.get_page(page).get_layer(layer);
        } else {
            break;
        }
    }

    let mut buf = Vec::new();
    doc.save(&mut BufWriter::new(&mut buf)).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("basic_printpdf", |b| b.iter(basic_printpdf));
    c.bench_function("basic_scannedpdf", |b| b.iter(basic_scannedpdf));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
