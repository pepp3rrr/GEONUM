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

pub struct PointComb {
    point: Point,
    factor: f32,
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

impl PointComb {
    /// Tries to apply the affine combination to return a valid Point. Panics if ∑ factors != 1.0
    pub fn into_point(self) -> Point {
        assert_eq!(
            self.factor, 1.0,
            "Not an affine combination, sum of factors: {}",
            self.factor
        );
        self.point
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

impl std::ops::Add for Point {
    type Output = PointComb;

    fn add(self, rhs: Self) -> Self::Output {
        PointComb {
            point: Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            },
            factor: 2.0,
        }
    }
}

impl std::ops::Mul<f32> for Point {
    type Output = PointComb;

    fn mul(self, rhs: f32) -> Self::Output {
        PointComb {
            point: Point {
                x: self.x * rhs,
                y: self.y * rhs,
            },
            factor: rhs,
        }
    }
}

impl std::ops::Mul<Point> for f32 {
    type Output = PointComb;

    fn mul(self, rhs: Point) -> Self::Output {
        PointComb {
            point: Point {
                x: self * rhs.x,
                y: self * rhs.y,
            },
            factor: self,
        }
    }
}

impl std::ops::Add for PointComb {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            point: Point {
                x: self.point.x + rhs.point.x,
                y: self.point.y + rhs.point.y,
            },
            factor: self.factor + rhs.factor,
        }
    }
}

impl std::ops::Add<Point> for PointComb {
    type Output = Self;

    fn add(self, rhs: Point) -> Self::Output {
        Self {
            point: Point {
                x: self.point.x + rhs.x,
                y: self.point.y + rhs.y,
            },
            factor: self.factor + 1.0,
        }
    }
}

impl std::ops::Add<PointComb> for Point {
    type Output = PointComb;

    fn add(self, rhs: PointComb) -> Self::Output {
        PointComb {
            point: Point {
                x: self.x + rhs.point.x,
                y: self.y + rhs.point.y,
            },
            factor: rhs.factor + 1.0,
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

    #[test]
    fn affine_comb() {
        let a = Point::new(1., 3.);
        let b = Point::new(2., 5.);
        let c = Point::new(-1., 2.);

        let comb = (1. / 4.) * a + (2. / 4.) * b + (1. / 4.) * c;

        assert_eq!(comb.into_point(), Point::new(1., 3.75));
    }

    #[test]
    #[should_panic]
    fn affine_comb_fail() {
        let a = Point::new(1., 3.);
        let b = Point::new(2., 5.);
        let c = Point::new(-1., 2.);

        let comb = (1. / 4.) * a + (3. / 4.) * b + (1. / 4.) * c;

        let _ = comb.into_point();
    }
}
