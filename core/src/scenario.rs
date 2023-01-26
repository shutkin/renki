use std::collections::HashMap;
use crate::renki_image::RenkiImage;
use crate::matrix::Matrix2d;

#[derive(Clone, Debug)]
pub struct ScenarioPoint {
    time: f64,
    anchor_x: f64,
    anchor_y: f64,
    offset_x: f64,
    offset_y: f64,
    scale: f64,
    angle: f64,
    alpha: f64,
}

#[derive(Clone, Debug)]
pub struct ImageScenario {
    image: String,
    points: Vec<ScenarioPoint>,
}

#[derive(Clone, Debug)]
pub struct Scenario {
    images: Vec<ImageScenario>,
    width: usize,
    height: usize,
    length: usize,
}

impl ImageScenario {
    fn interpolate_points(&self, time: f64) -> Option<ScenarioPoint> {
        if time < self.points[0].time {
            None
        } else if time >= self.points[self.points.len() - 1].time {
            None
        } else {
            let mut result = ScenarioPoint {time, anchor_x: 0.0, anchor_y: 0.0, offset_x: 0.0, offset_y: 0.0, scale: 0.0, angle: 0.0, alpha: 0.0};
            for i in 0..self.points.len() {
                let pi = &self.points[i];
                let mut l = 1.0;
                for j in 0..self.points.len() {
                    if i != j {
                        let pj = &self.points[j];
                        l *= (time - pj.time) / (pi.time - pj.time);
                    }
                }
                result.anchor_x += l * pi.anchor_x;
                result.anchor_y += l * pi.anchor_y;
                result.offset_x += l * pi.offset_x;
                result.offset_y += l * pi.offset_y;
                result.scale += l * pi.scale;
                result.angle += l * pi.angle;
                result.alpha += l * pi.alpha;
            }
            if result.alpha > 1.0 {result.alpha = 1.0;} else if result.alpha < 0.0 {result.alpha = 0.0;}
            Some(result)
        }
    }
}

impl Scenario {
    pub fn generate_scenario(images: &Vec<String>, images_map: &HashMap<String, RenkiImage>,
                             width: usize, height: usize, length: usize) -> Scenario {
        let mut images_scenarios = Vec::with_capacity(images.len());
        for image_index in 0..images.len() {
            let image_filename = &images[image_index];
            let image = images_map.get(image_filename).unwrap();
            let fit_scale = height as f64 / image.height as f64;

            let anchor_point_y = image.height as f64 * 0.5;
            let anchor_point_left = width as f64 / fit_scale * 0.6;
            let anchor_point_right = image.width as f64 - width as f64 / fit_scale * 0.6;
            let (anchor_point0, anchor_point1) = if image_index % 2 == 0 {
                (anchor_point_left, anchor_point_right)
            } else {
                (anchor_point_right, anchor_point_left)
            };

            let offset_point_y = height as f64 * 0.5;
            let offset_point_left = width as f64 * 0.5;
            let offset_point_right = width as f64 * 0.5;
            let (offset_point0, offset_point1) = if image_index % 2 == 0 {
                (offset_point_left, offset_point_right)
            } else {
                (offset_point_right, offset_point_left)
            };

            let start_time = image_index as f64 / images.len() as f64 / (1.0 + 0.25 / images.len() as f64);
            let end_time = (image_index + 1) as f64 / images.len() as f64 / (1.0 + 0.25 / images.len() as f64);
            let duration = (end_time - start_time) * 1.25;

            let mut points = Vec::new();
            points.push(ScenarioPoint {
                time: start_time,
                anchor_x: anchor_point0, anchor_y: anchor_point_y,
                offset_x: offset_point0, offset_y: offset_point_y,
                angle: 0.075, scale: fit_scale * 1.5, alpha: 0.0});
            points.push(ScenarioPoint {
                time: start_time + duration * 0.2,
                anchor_x: anchor_point0, anchor_y: anchor_point_y,
                offset_x: offset_point0, offset_y: offset_point_y,
                angle: 0.0, scale: fit_scale * 1.0, alpha: 0.88});
            points.push(ScenarioPoint {
                time: start_time + duration * 0.5,
                anchor_x: (anchor_point0 + anchor_point1) * 0.5, anchor_y: anchor_point_y,
                offset_x: (offset_point0 + offset_point1) * 0.5, offset_y: offset_point_y,
                angle: 0.0, scale: fit_scale * 1.125, alpha: 1.0});
            points.push(ScenarioPoint {
                time: start_time + duration * 0.8,
                anchor_x: anchor_point1, anchor_y: anchor_point_y,
                offset_x: offset_point1, offset_y: offset_point_y,
                angle: 0.0, scale: fit_scale * 1.0, alpha: 0.88});
            points.push(ScenarioPoint {
                time: start_time + duration,
                anchor_x: anchor_point1, anchor_y: anchor_point_y,
                offset_x: offset_point1, offset_y: offset_point_y,
                angle: -0.066, scale: fit_scale * 1.5, alpha: 0.0});

            let image_scenario = ImageScenario {image: image_filename.clone(), points};
            images_scenarios.push(image_scenario);
        }
        Scenario {images: images_scenarios, width, height, length}
    }

    fn render_frame(&self, time: f64, images_map: &HashMap<String, RenkiImage>) -> RenkiImage {
        let channel_size = self.width * self.height;
        let mut data = Vec::new();
        for _channel_index in 0..3 {
            data.push(vec![0_f32; channel_size]);
        }
        let mut result = RenkiImage { width: self.width, height: self.height, channels: data, alpha: Vec::new() };
        for scenario_index in 0..self.images.len() {
            let image_scenario = &self.images[scenario_index];
            let image = images_map.get(&image_scenario.image).expect("Failed to find image");

            let point = image_scenario.interpolate_points(time);
            if point.is_some() {
                let point = point.unwrap();
                let matrix = Matrix2d::translate(-point.anchor_x, -point.anchor_y)
                    .multiply(&Matrix2d::scale(point.scale))
                    .multiply(&Matrix2d::rotation(point.angle))
                    .multiply(&Matrix2d::translate(point.offset_x, point.offset_y));
                let image = image.transform(&matrix, self.width, self.height, point.alpha);
                result = result.blend(&image);
            }
        }
        result
    }

    pub fn render(&self, images_map: &HashMap<String, RenkiImage>, frames_prefix: &str) {
        for frame_index in 0..self.length {
            let time = frame_index as f64 / self.length as f64;
            let frame = self.render_frame(time, images_map);
            let frame_name = format!("{}frame{:04}.png", frames_prefix, frame_index);
            frame.save(frame_name.as_str());
            println!("Progress {}%", (time * 100.0) as i32);
        }
    }
}