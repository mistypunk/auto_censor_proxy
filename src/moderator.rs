use ab_glyph::{FontVec, PxScale};
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, Rgba};
use imageproc::drawing::{draw_hollow_rect_mut, draw_text_mut};
use imageproc::rect::Rect;

pub struct ContentModerator {
    font: FontVec,
}

impl ContentModerator {
    pub fn new() -> Self {
        let font_data = include_bytes!("../assets/DejaVuSans.ttf").to_vec();
        let font = FontVec::try_from_vec(font_data).expect("Erro ao carregar fonte");
        Self { font }
    }

    pub fn process_detection(
        &self,
        img: DynamicImage,
        label: &str,
        bbox: [u32; 4],
    ) -> DynamicImage {
        if self.simplify_label(label) == "LEGAL" {
            return img;
        };

        let x = bbox[0];
        let y = bbox[1];
        let w = bbox[2];
        let h = bbox[3];

        println!(
            "DEBUG: Processando {} em [x:{}, y:{}, w:{}, h:{}]",
            label, x, y, w, h
        );

        let mut rgba_img = img.to_rgba8();

        let x_s = x as u32;
        let y_s = y as u32;
        let w_s = w as u32;
        let h_s = h as u32;

        if w_s > 0 && h_s > 0 {
            let sub = DynamicImage::ImageRgba8(rgba_img.clone())
                .crop_imm(x_s, y_s, w_s, h_s)
                .resize(8, 8, FilterType::Nearest)
                .resize(w_s, h_s, FilterType::Nearest)
                .to_rgba8();

            image::imageops::replace(&mut rgba_img, &sub, x_s as i64, y_s as i64);

            let white = Rgba([255, 255, 255, 255]);

            draw_hollow_rect_mut(
                &mut rgba_img,
                Rect::at(x_s as i32, y_s as i32).of_size(w_s, h_s),
                white,
            );

            let display_text = self.simplify_label(label);
            let scale = PxScale::from(35.0);

            let text_y = if y_s > 40 {
                y_s as i32 - 35
            } else {
                y_s as i32 + 10
            };

            draw_text_mut(
                &mut rgba_img,
                white,
                x_s as i32,
                text_y,
                scale,
                &self.font,
                &display_text,
            );
        }

        // Retornamos a imagem final como DynamicImage
        DynamicImage::ImageRgba8(rgba_img)
    }
    fn simplify_label(&self, label: &str) -> String {
        match label.to_uppercase().as_str() {
            "FEMALE_BREAST_EXPOSED" => "ILLEGAL".to_string(),
            "FEMALE_GENITALIA_COVERED" => "ILLEGAL".to_string(),
            "FEMALE_GENITALIA_EXPOSED" => "ILLEGAL".to_string(),
            "BUTTOCKS_EXPOSED" => "ILLEGAL".to_string(),
            "ANUS_EXPOSED" => "ILLEGAL".to_string(),
            _ => "LEGAL".to_string(),
        }
    }
}
