use image::{DynamicImage, GenericImageView};
use std::io::Write;
use std::path::Path;
use std::{collections::HashMap, io::Seek};

use crate::{Error, PageConfig};

fn encode_unicode_string(s: &str) -> Vec<u8> {
    let mut encoded = vec![0xFE, 0xFF];
    encoded.extend(s.encode_utf16().flat_map(|c| vec![(c >> 8) as u8, c as u8]));
    encoded
}

// PDF related
pub struct PDF<W: Write + Seek> {
    default_page_config: PageConfig,
    // writer: W,
    writer: std::io::BufWriter<W>,
    current_page: usize,
    total_pages: usize,
    // outlines: HashMap<usize, String>,
    outlines: Vec<(usize, String)>,
    xref_offset: usize,
    object_offsets: HashMap<usize, usize>,
}

impl<W: Write + Seek> PDF<W> {
    pub fn create(
        writer: W,
        default_page_config: PageConfig,
        total_pages: usize,
    ) -> std::io::Result<Self> {
        let mut pdf = PDF {
            default_page_config,
            writer: std::io::BufWriter::new(writer),
            current_page: 0,
            xref_offset: 0,
            total_pages,
            outlines: Vec::new(),
            object_offsets: HashMap::new(),
        };
        pdf.write_top()?;
        Ok(pdf)
    }

    fn obj_start(&mut self, i: usize) -> std::io::Result<()> {
        // Remember the offset of the object
        let offset = self.writer.seek(std::io::SeekFrom::Current(0))?;
        self.object_offsets.insert(i, offset as usize);
        self.writer.write_all(format!("{} 0 obj\n", i).as_bytes())?;
        self.writer.write_all(b"<<\n")?;

        Ok(())
    }

    fn obj_end(&mut self) -> std::io::Result<()> {
        self.writer.write_all(b">>\n")?;
        self.writer.write_all(b"endobj\n")?;
        Ok(())
    }

    fn write_top(&mut self) -> std::io::Result<()> {
        // PDF Header
        self.writer.write_all(b"%PDF-1.7\n")?;
        // Root Object
        self.obj_start(1)?;
        self.writer.write_all(b"/Type /Catalog\n")?;
        self.writer.write_all(b"/Pages 2 0 R\n")?;
        self.writer.write_all(b"/Outlines 3 0 R\n")?;
        self.writer.write_all(b"/PageMode /UseOutlines\n")?;
        self.obj_end()?;
        // Pages Object
        self.write_pages()?;
        self.writer.flush()?;
        Ok(())
    }

    fn write_outlines(&mut self) -> std::io::Result<()> {
        self.obj_start(3)?;
        self.writer.write_all(b"/Type /Outlines\n")?;
        self.writer.write_all(b"/Count ")?;

        let count = self.outlines.len();
        let outlines = self.outlines.clone();

        self.writer.write_all(format!("{}", count).as_bytes())?;

        let start = self.total_objects() - count + 1; // ID of the first outline

        if count > 0 {
            self.writer
                .write_all(format!("/First {} 0 R\n", start).as_bytes())?;
            self.writer
                .write_all(format!("/Last {} 0 R\n", self.total_objects()).as_bytes())?;
        }
        self.obj_end()?;

        for (i, (page_id, title)) in outlines.iter().enumerate() {
            self.obj_start(start + i)?;
            self.writer.write_all(b"/Title (")?;
            // self.writer.write_all(format!("({})", title).as_bytes())?;
            // /Title (\xFE\xFF...)
            // Encode the title as UTF-16BE
            self.writer.write_all(&encode_unicode_string(title))?;
            self.writer.write_all(b")\n")?;

            self.writer.write_all(b"\n")?;

            self.writer.write_all(b"/Parent 3 0 R\n")?;

            self.writer.write_all(b"/Dest ")?;
            self.writer
                .write_all(format!("[{} 0 R /XYZ 0 0 0]", page_id).as_bytes())?;

            if i > 0 {
                self.writer
                    .write_all(format!("/Prev {} 0 R\n", start + i - 1).as_bytes())?;
            }
            if i < count - 1 {
                self.writer
                    .write_all(format!("/Next {} 0 R\n", start + i + 1).as_bytes())?;
            }

            self.obj_end()?;
        }

        Ok(())
    }

    fn write_pages(&mut self) -> std::io::Result<()> {
        // self.writer.write_all(b"2 0 obj\n")?;
        // self.writer.write_all(b"<<\n")?;
        self.obj_start(2)?;
        self.writer.write_all(b"/Type /Pages\n")?;
        // self.writer
        //     .write_all(format!("/Count {}\n", self.total_pages).as_bytes())?;

        self.writer.write_all(b"/Count ")?;
        // Remember current position
        // self.count_offset = self.writer.seek(std::io::SeekFrom::Current(0))? as usize;
        // self.writer.write_all(b"000000\n")?;
        self.writer
            .write_all(self.total_pages.to_string().as_bytes())?;

        self.writer.write_all(b"/Kids [\n")?;
        for i in 0..self.total_pages {
            self.writer
                .write_all(format!("{} 0 R\n", i * 3 + 4).as_bytes())?;
        }

        self.writer.write_all(b"]\n")?;
        // self.writer.write_all(b">>\n")?;
        // self.writer.write_all(b"endobj\n")?;
        self.obj_end()?;
        Ok(())
    }

    fn write_xref(&mut self) -> std::io::Result<()> {
        // Remember current position
        self.xref_offset = self.writer.seek(std::io::SeekFrom::Current(0))? as usize;
        self.writer.write_all(b"xref\n")?;
        self.writer
            .write_all(format!("0 {}\n", self.total_objects()).as_bytes())?;

        self.writer.write_all(b"0000000000 65535 f\n")?;

        for i in 1..self.total_objects() {
            let offset = self.object_offsets.get(&i).unwrap();
            self.writer
                .write_all(format!("{:010} 00000 n\n", offset).as_bytes())?;
        }
        Ok(())
    }

    fn write_trailer(&mut self) -> std::io::Result<()> {
        self.writer.write_all(b"trailer\n")?;
        self.writer.write_all(b"<<\n")?;
        self.writer
            .write_all(format!("/Size {}\n", self.total_objects()).as_bytes())?;
        self.writer.write_all(b"/Root 1 0 R\n")?;
        self.writer.write_all(b">>\n")?;
        self.writer.write_all(b"startxref\n")?;
        self.writer
            .write_all(format!("{}\n", self.xref_offset).as_bytes())?;
        self.writer.write_all(b"%%EOF\n")?;
        Ok(())
    }

    fn write_image_obj(
        &mut self,
        index: usize,
        image: DynamicImage,
        quality: u8,
    ) -> Result<(), Error> {
        let (width, height) = image.dimensions();

        self.obj_start(index)?;
        self.writer.write_all(b"/Type /XObject\n")?;
        self.writer.write_all(b"/Subtype /Image\n")?;
        self.writer.write_all(b"/Width ")?;
        self.writer.write_all(format!("{}\n", width).as_bytes())?;
        self.writer.write_all(b"/Height ")?;
        self.writer.write_all(format!("{}\n", height).as_bytes())?;
        self.writer.write_all(b"/ColorSpace /DeviceRGB\n")?;
        self.writer.write_all(b"/BitsPerComponent 8\n")?;

        #[cfg(feature = "flate2")]
        {
            self.writer
                .write_all(b"/Filter [/FlateDecode /DCTDecode]\n")?;
        }
        #[cfg(not(feature = "flate2"))]
        {
            self.writer.write_all(b"/Filter /DCTDecode\n")?;
        }

        self.writer.write_all(b"/Length ")?;

        let data = crate::image::encode_image(image, quality)?;

        self.writer
            .write_all(format!("{}\n", data.len()).as_bytes())?;
        self.writer.write_all(b">>\n")?;
        self.writer.write_all(b"stream\n")?;
        self.writer.write_all(&data)?;
        self.writer.write_all(b"\nendstream\n")?;
        self.writer.write_all(b"endobj\n")?;

        Ok(())
    }

    pub fn add_page_from_image(
        &mut self,
        image: image::DynamicImage,
        outline: Option<String>,
        page_config: Option<PageConfig>,
    ) -> Result<(), Error> {
        if self.current_page >= self.total_pages {
            return Err(Error::PageOverflow);
        }

        let page_config = page_config.as_ref().unwrap_or(&self.default_page_config);

        let image = crate::image::preprocess(image, &page_config);

        let (page_width, page_height) = page_config.size.dimensions();

        let i = self.current_page * 3 + 4;

        // Contents Object
        let mut contents = Vec::new();
        contents.write_all(b"q\n")?;

        contents.extend(crate::image::get_operands(image.dimensions(), &page_config));

        contents.write_all(b"/Img Do\n")?;
        contents.write_all(b"Q\n")?;

        self.write_image_obj(i + 1, image, page_config.quality)?;

        // Page Object
        self.obj_start(i)?;
        self.writer.write_all(b"/Type /Page\n")?;
        self.writer.write_all(b"/Parent 2 0 R\n")?;

        // Media Box
        self.writer.write_all(b"/MediaBox [0 0 ")?;
        self.writer
            .write_all(format!("{} {}", page_width, page_height).as_bytes())?;
        self.writer.write_all(b"]\n")?;

        // Resources
        self.writer.write_all(b"/Resources <<\n")?;
        self.writer.write_all(b"/XObject <<\n")?;
        self.writer
            .write_all(format!("/Img {} 0 R\n", i + 1).as_bytes())?;
        self.writer.write_all(b">>\n")?;
        self.writer.write_all(b">>\n")?;

        // Contents
        self.writer.write_all(b"/Contents ")?;
        self.writer
            .write_all(format!("{} 0 R\n", i + 2).as_bytes())?;

        self.obj_end()?;

        self.obj_start(i + 2)?;

        self.writer
            .write_all(format!("/Length {}\n", contents.len()).as_bytes())?;
        self.writer.write_all(b">>\n")?;
        self.writer.write_all(b"stream\n")?;
        self.writer.write_all(&contents)?;
        self.writer.write_all(b"\nendstream\n")?;
        self.writer.write_all(b"endobj\n")?;

        self.writer.flush()?;
        if let Some(outline) = outline {
            // self.outlines.insert(i, outline);
            self.outlines.push((i, outline));
        }
        self.current_page += 1;
        Ok(())
    }

    pub fn add_page_from_path<P: AsRef<Path>>(
        &mut self,
        image_path: P,
        outline: Option<String>,
        page_config: Option<PageConfig>,
    ) -> Result<(), Error> {
        let image = image::open(image_path)?;
        self.add_page_from_image(image, outline, page_config)
    }

    fn total_objects(&self) -> usize {
        3 + self.total_pages * 3 + self.outlines.len()
    }

    pub fn finish(mut self) -> std::io::Result<()> {
        self.write_outlines()?;
        self.write_xref()?;
        self.write_trailer()?;
        // // // Update the count
        // self.writer
        //     .seek(std::io::SeekFrom::Start(self.count_offset as u64))?;
        // self.writer
        //     .write_all(format!("{:06}", self.total_pages).as_bytes())?;
        // self.writer.flush()?;
        // Close the writer
        drop(self.writer);
        Ok(())
    }
}
