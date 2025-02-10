use crate::Point;

pub trait Plot {
    fn sample(&self, t: f32) -> Point;

    fn bounding_box(&self) -> (Point, Point);
}
