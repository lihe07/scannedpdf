// Image related

use std::io::{Cursor, Write};

use image::{DynamicImage, GenericImageView};

use crate::{Alignment, Error, PageConfig, PageSize};

pub fn preprocess(mut image: image::DynamicImage, page_config: &PageConfig) -> image::DynamicImage {
    if page_config.size != PageSize::Image {
        // If image is larger than page, resize it
        let (width, height) = image.dimensions();
        let (mut page_width, mut page_height) = page_config.size.dimensions();
        let (margin_x, margin_y) = page_config.margin.dimensions();

        page_width -= margin_x;
        page_height -= margin_y;

        if width > page_width || height > page_height {
            image = image.resize(
                page_width,
                page_height,
                image::imageops::FilterType::Lanczos3,
            );
        }
    }

    image
}

pub fn get_operands(image: (u32, u32), page_config: &PageConfig) -> Vec<u8> {
    // Calculate 6 operands for the image
    let (image_width, image_height) = image;

    let (page_width, page_height) = page_config.size.dimensions();
    let (mut margin_x, mut margin_y) = page_config.margin.dimensions();
    margin_x /= 2;
    margin_y /= 2;

    // X position
    let x = match page_config.horizontal_alignment {
        Alignment::Start => margin_x,
        Alignment::Center => ((page_width - image_width) / 2).max(margin_x),
        Alignment::End => page_width - image_width - margin_x,
        Alignment::Custom(x) => x.max(margin_x),
    };

    // Y position
    let y = match page_config.vertical_alignment {
        Alignment::Start => page_height - image_height - margin_y,
        Alignment::Center => ((page_height - image_height) / 2).max(margin_y),
        Alignment::End => margin_y,
        Alignment::Custom(y) => y.max(margin_y),
    };

    format!(
        "{} {} {} {} {} {} cm\n",
        image_width, 0, 0, image_height, x, y
    )
    .into_bytes()
}

pub fn encode_image(image: DynamicImage, quality: u8) -> Result<Vec<u8>, Error> {
    let mut data = Vec::new();

    image.write_to(
        &mut Cursor::new(&mut data),
        image::ImageOutputFormat::Jpeg(quality),
    )?;

    #[cfg(feature = "flate2")]
    {
        let mut encoder =
            flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());

        encoder.write_all(&data)?;

        encoder.finish().map_err(Error::Io)
    }

    #[cfg(not(feature = "flate2"))]
    {
        Ok(data)
    }
}
