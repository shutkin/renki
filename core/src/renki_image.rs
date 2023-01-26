use image::{GenericImageView, ColorType};
use crate::matrix::Matrix2d;
use crate::geom::{Point, Geom};

#[derive(Clone)]
pub struct RenkiImage {
    pub width: usize,
    pub height: usize,
    pub channels: Vec<Vec<f32>>,
    pub alpha: Vec<f32>,
}

impl RenkiImage {
    pub fn from_img(path: &String) -> Result<RenkiImage, String> {
        let img = image::open(path.as_str()).map_err(|e| e.to_string())?;
        let (img_width, img_height) = img.dimensions();
        let pixels = img.to_rgb8().to_vec();
        let channel_size = img_width as usize * img_height as usize;
        let mut channels = vec![Vec::with_capacity(channel_size); 3];
        for i in 0..(pixels.len() / 3) {
            channels[0].push(pixels[i * 3] as f32);
            channels[1].push(pixels[i * 3 + 1] as f32);
            channels[2].push(pixels[i * 3 + 2] as f32);
        }
        Result::Ok(RenkiImage { width: img_width as usize, height: img_height as usize, channels, alpha: vec![1_f32; channel_size] })
    }

    fn to_rgb8(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.width as usize * self.height as usize * 3);
        for i in 0..self.channels[0].len() {
            let r = self.channels[0][i];
            let g = if self.channels.len() == 3 { self.channels[1][i] } else { self.channels[0][i] };
            let b = if self.channels.len() == 3 { self.channels[2][i] } else { self.channels[0][i] };
            result.push(if r < 0_f32 { 0_u8 } else if r > 255_f32 { 255_u8 } else { r as u8 });
            result.push(if g < 0_f32 { 0_u8 } else if g > 255_f32 { 255_u8 } else { g as u8 });
            result.push(if b < 0_f32 { 0_u8 } else if b > 255_f32 { 255_u8 } else { b as u8 });
        }
        result
    }

    pub fn save(self: &RenkiImage, name: &str) {
        image::save_buffer(name, &self.to_rgb8(), self.width as u32, self.height as u32, ColorType::Rgb8)
            .expect("failed to write image");
    }

    fn calc_area_in_pixel(points: &Vec<Point>, pixel_x: i32, pixel_y: i32) -> f64 {
        let mut area = 0.0;
        let mut triangle = vec![Point::zero(); 3];
        triangle[0] = points[0].translate(-pixel_x as f64, -pixel_y as f64);
        for index in 0..points.len() - 2 {
            triangle[1] = points[index + 1].translate(-pixel_x as f64, -pixel_y as f64);
            triangle[2] = points[index + 2].translate(-pixel_x as f64, -pixel_y as f64);
            let clipped_polygon = Geom::clip(&triangle);
            if clipped_polygon.len() >= 3 {
                area += Geom::polygon_area(&clipped_polygon);
            }
        }
        area
    }

    pub fn transform(&self, matrix: &Matrix2d, width: usize, height: usize, alpha: f64) -> RenkiImage {
        let channel_size = width * height;
        let mut data = Vec::new();
        for _channel_index in 0..self.channels.len() {
            data.push(vec![0_f32; channel_size]);
        }
        let mut alpha_data = vec![0_f32; channel_size];

        let mut transformed_pixel = vec![Point::zero(); 4];
        for y in 0..self.height {
            for x in 0..self.width {
                let source_index = y * self.width + x;
                transformed_pixel[0] = matrix.apply(&Point::new(x as f64, y as f64));
                transformed_pixel[1] = matrix.apply(&Point::new(x as f64, y as f64 + 1.0));
                transformed_pixel[2] = matrix.apply(&Point::new(x as f64 + 1.0, y as f64 + 1.0));
                transformed_pixel[3] = matrix.apply(&Point::new(x as f64 + 1.0, y as f64));
                let x_min = transformed_pixel.iter().map(|p| p.x as i32).min().unwrap();
                let x_max = transformed_pixel.iter().map(|p| p.x as i32).max().unwrap();
                let y_min = transformed_pixel.iter().map(|p| p.y as i32).min().unwrap();
                let y_max = transformed_pixel.iter().map(|p| p.y as i32).max().unwrap();
                for y_dest in y_min..=y_max {
                    if y_dest >= 0 && y_dest < height as i32 {
                        for x_dest in x_min..=x_max {
                            if x_dest >= 0 && x_dest < width as i32 {
                                let dest_index = y_dest as usize * width + x_dest as usize;
                                let area = RenkiImage::calc_area_in_pixel(&transformed_pixel, x_dest, y_dest);
                                for channel_index in 0..data.len() {
                                    let v = self.channels[channel_index][source_index];
                                    data[channel_index][dest_index] += v * area as f32;
                                }
                                alpha_data[dest_index] += (area * alpha) as f32;
                            }
                        }
                    }
                }
            }
        }
        RenkiImage { width, height, channels: data, alpha: alpha_data }
    }

    pub fn blend(&self, image: &RenkiImage) -> RenkiImage {
        let channel_size = self.width * self.height;
        let mut data = Vec::new();
        for channel_index in 0..self.channels.len() {
            let mut channel_data = Vec::with_capacity(channel_size);
            for i in 0..channel_size {
                let alpha = image.alpha[i];
                let v = self.channels[channel_index][i] * (1_f32 - alpha) + image.channels[channel_index][i] * alpha;
                channel_data.push(v);
            }
            data.push(channel_data);
        }
        RenkiImage { width: self.width, height: self.height, channels: data, alpha: vec![1_f32; channel_size] }
    }
}