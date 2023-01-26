use crate::geom::Point;

pub struct Matrix2d {
    data: Vec<Vec<f64>>
}

impl Matrix2d {
    pub fn translate(tx: f64, ty: f64) -> Matrix2d {
        Matrix2d {
            data: vec!(
                vec!(1_f64, 0_f64, tx),
                vec!(0_f64, 1_f64, ty),
                vec!(0_f64, 0_f64, 1_f64)
            )
        }
    }

    pub fn scale(s: f64) -> Matrix2d {
        Matrix2d {
            data: vec!(
                vec!(s, 0_f64, 0_f64),
                vec!(0_f64, s, 0_f64),
                vec!(0_f64, 0_f64, 1_f64)
            )
        }
    }

    pub fn rotation(angle: f64) -> Matrix2d {
        Matrix2d {
            data: vec!(
                vec!(angle.cos(), -angle.sin(), 0_f64),
                vec!(angle.sin(), angle.cos(), 0_f64),
                vec!(0_f64, 0_f64, 1_f64)
            )
        }
    }

    pub fn apply(&self, p: &Point) -> Point {
        Point {
            x: p.x * self.data[0][0] + p.y * self.data[0][1] + self.data[0][2],
            y: p.x * self.data[1][0] + p.y * self.data[1][1] + self.data[1][2],
        }
    }

    pub fn multiply(&self, m: &Matrix2d) -> Matrix2d {
        let mut n = vec!(vec!(0_f64, 0_f64, 0_f64), vec!(0_f64, 0_f64, 0_f64), vec!(0_f64, 0_f64, 0_f64));
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    n[i][j] += m.data[i][k] * self.data[k][j]
                }
            }
        }
        Matrix2d { data: n }
    }
}
