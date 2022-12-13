use std::path::Path;

use crate::data::ViewImageData;
use druid::{
    piet,
    widget::{Image, SizedBox},
    ImageBuf, Widget, WidgetExt,
};
use image::{GenericImageView, ImageBuffer, Pixel};

pub fn view_image(data: &ViewImageData) -> Box<dyn Widget<ViewImageData>> {
    if let Some(url) = &data.url_image {
        let path_image = Path::new(url.as_str());

        let image = image::open(path_image);
        if let Ok(file_image) = image {
            let (width, height) = file_image.dimensions();
            // let format = buffer_render.format().unwrap();
            let druid_format_image = piet::ImageFormat::RgbaSeparate;

            let mut image_buffer = ImageBuffer::new(width, height);

            for (x, y, pixel) in file_image.pixels() {
                image_buffer.put_pixel(x, y, pixel.map(|p| p.saturating_sub(65)))
            }

            let pixels = image_buffer.as_raw().as_ref();
            let image_data =
                ImageBuf::from_raw(pixels, druid_format_image, width as usize, height as usize);
            let view_image = Image::new(image_data);

            return SizedBox::new(view_image).center().boxed();
        }
    }
    SizedBox::empty().boxed()
}
