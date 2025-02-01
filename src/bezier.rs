use crate::types::*;

pub struct Bezier {
    control: Vec<Point>,
}

impl Bezier {
    pub fn new(control: Vec<Point>) -> Self {
        Self { control }
    }

    pub fn compute(&self, t: f32) -> Point {
        Self::compute_woker(t, self.control.clone())
    }

    fn compute_woker(t: f32, mut control: Vec<Point>) -> Point {
        let n = control.len();

        if n == 1 {
            return control.pop().expect("Should not be empty");
        }

        let mut new = Vec::<Point>::new();
        for k in 1..n {
            let a = control.get(k - 1).unwrap().clone();
            let b = control.get(k).unwrap().clone();

            // (1-t)a + tb <=> a -ta + tb <=> a + t(b - a)
            // This form respects point and vec operation rules
            let coord = a + (b - a) * t;
            new.push(coord);
        }

        Self::compute_woker(t, new)
    }
}
