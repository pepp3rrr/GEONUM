#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

// Point + Vec2 = Point
impl std::ops::Add<Vec2> for Point {
    type Output = Self;

    fn add(self, rhs: Vec2) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Vec2 + Point = Point
impl std::ops::Add<Point> for Vec2 {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Vec2 + Vec2 = Vec2
impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// Point - Point = Vec2
impl std::ops::Sub for Point {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

// Vec2 * float = Vec2
impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

// float * Vec2 = Vec2
impl std::ops::Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub_points() {
        let a = Point::new(1., 3.);
        let b = Point::new(2., 5.);

        let result = a - b;
        assert_eq!(result, Vec2::new(-1., -2.));
    }

    #[test]
    fn add_vec_to_point() {
        let point = Point::new(1., 2.);
        let vec = Vec2::new(7., 3.);

        let result = point + vec;
        assert_eq!(result, Point::new(8., 5.));
    }

    #[test]
    fn mul_vec_float() {
        let vec = Vec2::new(8., 3.);
        let float = 0.5;

        let result = vec * float;
        assert_eq!(result, Vec2::new(4., 1.5));
    }

    #[test]
    fn add_vecs() {
        let a = Vec2::new(1., 3.);
        let b = Vec2::new(-1., -2.);

        let result = a + b;
        assert_eq!(result, Vec2::new(0., 1.));
    }
}
