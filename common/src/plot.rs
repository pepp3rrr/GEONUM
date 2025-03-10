use crate::Point;

pub trait Plot {
    fn sample(&self, t: f32) -> Point;
}

pub trait BoundingBox {
    fn bounding_box(&self) -> (Point, Point);
}

impl BoundingBox for Vec<Point<2>> {
    fn bounding_box(&self) -> (Point, Point) {
        let right = self
            .iter()
            .map(|p| p.x())
            .max_by(|a, b| a.total_cmp(&b))
            .unwrap();
        let top = self
            .iter()
            .map(|p| p.y())
            .max_by(|a, b| a.total_cmp(&b))
            .unwrap();
        let left = self
            .iter()
            .map(|p| p.x())
            .max_by(|a, b| b.total_cmp(&a))
            .unwrap();
        let bottom = self
            .iter()
            .map(|p| p.y())
            .max_by(|a, b| b.total_cmp(&a))
            .unwrap();

        (Point::<2>::new(left, bottom), Point::<2>::new(right, top))
    }
}
