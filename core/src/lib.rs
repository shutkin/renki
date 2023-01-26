use std::collections::HashMap;
use crate::renki_image::RenkiImage;
use crate::scenario::Scenario;

mod renki_image;
mod matrix;
mod geom;
mod scenario;

#[cfg(test)]
mod tests {
    use crate::geom::{Geom, Point};
    use crate::renki_image::RenkiImage;
    use crate::matrix::Matrix2d;
    use crate::scenario::Scenario;
    use std::collections::HashMap;

    #[test]
    fn algorithm_test() {
        let subject_polygon = vec![Point {x: 0.5, y: 0.5}, Point {x: 0.75, y: 1.25}, Point {x: 1.25, y: 0.25}];
        let result = Geom::clip(&subject_polygon);
        println!("Result 1: {:?}", &result);
        let result_area = Geom::polygon_area(&result);
        println!("Result 1 area {}", result_area);

        let subject_polygon = vec![Point {x: 0.75, y: 1.25}, Point {x: 1.25, y: 1.25}, Point {x: 1.25, y: 0.25}];
        let result = Geom::clip(&subject_polygon);
        println!("Result 2: {:?}", &result);
        let result_area = Geom::polygon_area(&result);
        println!("Result 2 area {}", result_area);
    }

    #[test]
    fn test_image_transform() {
        let image = RenkiImage::from_img(&String::from("sample0.jpg")).expect("Failed to load file");
        let matrix = Matrix2d::translate(128.0, 72.0)
            .multiply(&Matrix2d::scale(1.25))
            .multiply(&Matrix2d::rotation(0.125))
            .multiply(&Matrix2d::translate(-100.0, -50.0));
        let image = image.transform(&matrix, 256, 144, 1.0);
        image.save("sample0_result.png");
    }

    #[test]
    fn test_scenario_render() {
        let files = vec![String::from("sample0.jpg"), String::from("sample1.jpg")];
        let mut images_map = HashMap::new();
        for filename in &files {
            let image = RenkiImage::from_img(filename).expect("Failed to load image");
            images_map.insert(filename.clone(), image);
        }
        let scenario = Scenario::generate_scenario(&files, &images_map, 144, 144, 100);
    }
}

pub struct RenkiCore {}

impl RenkiCore {
    pub fn render_images(files: &Vec<String>, length: usize) {
        let mut images_map = HashMap::new();
        for filename in files {
            let image = RenkiImage::from_img(filename).expect("Failed to load image");
            images_map.insert(filename.clone(), image);
        }
        let scenario = Scenario::generate_scenario(&files, &images_map, 1080, 1920, length);
        scenario.render(&images_map, "frames/");
    }
}
