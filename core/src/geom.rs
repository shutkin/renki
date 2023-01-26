#[derive(Debug,Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    pub fn zero() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    pub fn translate(&self, x: f64, y: f64) -> Point {
        Point { x: self.x + x, y: self.y + y }
    }
}

pub struct Geom {}

impl Geom {
    fn intersection_with_clipping(clip_index: usize, p0: &Point, p1: &Point) -> Option<Point> {
        match clip_index {
            0 => {
                let t = (0.0 - p0.y) / (p1.y - p0.y);
                if t > 0.0 && t < 1.0 {
                    let x = p0.x + (p1.x - p0.x) * t;
                    Some(Point {x, y: 0.0})
                } else {None}
            },
            1 => {
                let t = (0.0 - p0.x) / (p1.x - p0.x);
                if t > 0.0 && t < 1.0 {
                    let y = p0.y + (p1.y - p0.y) * t;
                    Some(Point { x: 0.0, y })
                } else {None}
            },
            2 => {
                let t = (1.0 - p0.y) / (p1.y - p0.y);
                if t > 0.0 && t < 1.0 {
                    let x = p0.x + (p1.x - p0.x) * t;
                    Some(Point { x, y: 1.0 })
                } else {None}
            },
            3 => {
                let t = (1.0 - p0.x) / (p1.x - p0.x);
                if t > 0.0 && t < 1.0 {
                    let y = p0.y + (p1.y - p0.y) * t;
                    Some(Point { x: 1.0, y })
                } else {None}
            },
            _ => None
        }
    }

    fn is_right_side(clip_index: usize, p: &Point) -> bool {
        match clip_index {
            0 => p.y > 0.0,
            1 => p.x > 0.0,
            2 => p.y < 1.0,
            3 => p.x < 1.0,
            _ => false
        }
    }

    pub fn clip(subject_polygon: &Vec<Point>) -> Vec<Point> {
        let mut result_ring = subject_polygon.clone();
        for clip_index in 0..4 {
            let input = result_ring;
            //println!("Clip index {}: cur result {:?}", clip_index, &input);
            let mut p0 = input.last().unwrap();
            result_ring = vec![];
            for p1 in input.iter() {
                let intersection = Geom::intersection_with_clipping(clip_index, p0, p1);
                if intersection.is_some() {
                    //println!("Found intersection {:?}", &intersection);
                    if Geom::is_right_side(clip_index, &p0) {
                        result_ring.push(p0.clone());
                    }
                    result_ring.push(intersection.unwrap());
                } else {
                    if Geom::is_right_side(clip_index, &p0) {
                        result_ring.push(p0.clone());
                    }
                }
                p0 = p1;
            }
            if result_ring.is_empty() {break;}
        }
        result_ring
    }

    pub fn polygon_area(polygon: &Vec<Point>) -> f64 {
        let mut area = 0.0;
        for i in 0..polygon.len() - 2 {
            let p0 = &polygon[0];
            let p1 = &polygon[i + 1];
            let p2 = &polygon[i + 2];
            area += ((p1.x - p0.x) * (p2.y - p0.y) - (p2.x - p0.x) * (p1.y - p0.y)).abs() / 2.0;
        }
        area
    }

}