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
            .pop()
            .expect("Should not be empty")
    }

    fn compute_woker(t: f32, control: Vec<Point>) -> Vec<Point> {
        let mut iter = control.into_iter();
        let mut new = Vec::<Point>::new();

        let mut prev: Option<Point> = None;
        loop {
            let a = match prev {
                Some(prev) => prev,
                None => {
                    let first = iter.next().expect("Received empty control list");
                    first
                }
            };

            match iter.next() {
                Some(b) => {
                    let coord = a + (b - a) * t;
                    new.push(coord);

                    prev = Some(b);
                }
                None => break,
            };
        }

        if new.len() > 1 {
            Self::compute_woker(t, new)
        } else {
            new
        }
    }
}
