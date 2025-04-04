#[derive(PartialEq, Debug, Clone, Copy)]

pub struct Coords<const D: usize>([f32; D]);

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Point<const D: usize = 2> {
    pub coords: Coords<D>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Vector<const D: usize = 2> {
    pub coords: Coords<D>,
}

#[derive(Debug)]
pub struct PointComb<const D: usize = 2> {
    point: Point<D>,
    factor: f32,
}

impl Point<2> {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            coords: Coords([x, y]),
        }
    }

    pub fn x(&self) -> f32 {
        self.coords.0[0]
    }

    pub fn y(&self) -> f32 {
        self.coords.0[1]
    }
}

impl Vector<2> {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            coords: Coords([x, y]),
        }
    }

    pub fn x(&self) -> f32 {
        self.coords.0[0]
    }

    pub fn y(&self) -> f32 {
        self.coords.0[1]
    }
}

impl Point<3> {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            coords: Coords([x, y, z]),
        }
    }

    pub fn x(&self) -> f32 {
        self.coords.0[0]
    }

    pub fn y(&self) -> f32 {
        self.coords.0[1]
    }

    pub fn z(&self) -> f32 {
        self.coords.0[2]
    }
}

impl Vector<3> {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            coords: Coords([x, y, z]),
        }
    }

    pub fn x(&self) -> f32 {
        self.coords.0[0]
    }

    pub fn y(&self) -> f32 {
        self.coords.0[1]
    }

    pub fn z(&self) -> f32 {
        self.coords.0[2]
    }
}

impl<const D: usize> PointComb<D> {
    /// Tries to apply the affine combination to return a valid Point. Panics if ∑ factors != 1.0
    pub fn into_point(self) -> Point<D> {
        const PRECISION: f32 = 1024.0;
        let rounded_factor = (self.factor * PRECISION).round() / PRECISION;

        assert_eq!(
            rounded_factor, 1.0,
            "Not an affine combination, sum of factors: {}",
            rounded_factor
        );
        self.point
    }
}

// Point + Vector = Point
impl<const D: usize> std::ops::Add<Vector<D>> for Point<D> {
    type Output = Self;

    fn add(self, rhs: Vector<D>) -> Self::Output {
        Self {
            coords: self.coords + rhs.coords,
        }
    }
}

// Vector + Point = Point
impl<const D: usize> std::ops::Add<Point<D>> for Vector<D> {
    type Output = Point<D>;

    fn add(self, rhs: Point<D>) -> Self::Output {
        Point {
            coords: self.coords + rhs.coords,
        }
    }
}

// Vector + Vector = Vector
impl<const D: usize> std::ops::Add for Vector<D> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            coords: self.coords + rhs.coords,
        }
    }
}

// Point - Point = Vector
impl<const D: usize> std::ops::Sub for Point<D> {
    type Output = Vector<D>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            coords: self.coords - rhs.coords,
        }
    }
}

// Vector * float = Vector
impl<const D: usize> std::ops::Mul<f32> for Vector<D> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            coords: self.coords * rhs,
        }
    }
}

// float * Vector = Vector
impl<const D: usize> std::ops::Mul<Vector<D>> for f32 {
    type Output = Vector<D>;

    fn mul(self, rhs: Vector<D>) -> Self::Output {
        Vector {
            coords: rhs.coords * self,
        }
    }
}

impl<const D: usize> std::ops::Add for Point<D> {
    type Output = PointComb<D>;

    fn add(self, rhs: Self) -> Self::Output {
        PointComb {
            point: Point {
                coords: self.coords + rhs.coords,
            },
            factor: 2.0,
        }
    }
}

impl<const D: usize> std::ops::Mul<f32> for Point<D> {
    type Output = PointComb<D>;

    fn mul(self, rhs: f32) -> Self::Output {
        PointComb {
            point: Point {
                coords: self.coords * rhs,
            },
            factor: rhs,
        }
    }
}

impl<const D: usize> std::ops::Mul<Point<D>> for f32 {
    type Output = PointComb<D>;

    fn mul(self, rhs: Point<D>) -> Self::Output {
        PointComb {
            point: Point {
                coords: rhs.coords * self,
            },
            factor: self,
        }
    }
}

impl<const D: usize> std::ops::Add for PointComb<D> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            point: Point {
                coords: self.point.coords + rhs.point.coords,
            },
            factor: self.factor + rhs.factor,
        }
    }
}

impl<const D: usize> std::ops::Sub for PointComb<D> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            point: Point {
                coords: self.point.coords - rhs.point.coords,
            },
            factor: self.factor - rhs.factor,
        }
    }
}

impl<const D: usize> std::ops::Add<Point<D>> for PointComb<D> {
    type Output = Self;

    fn add(self, rhs: Point<D>) -> Self::Output {
        Self {
            point: Point {
                coords: self.point.coords + rhs.coords,
            },
            factor: self.factor + 1.0,
        }
    }
}

impl<const D: usize> std::ops::Add<PointComb<D>> for Point<D> {
    type Output = PointComb<D>;

    fn add(self, rhs: PointComb<D>) -> Self::Output {
        PointComb {
            point: Point {
                coords: self.coords + rhs.point.coords,
            },
            factor: rhs.factor + 1.0,
        }
    }
}

impl<const D: usize> std::ops::Mul<f32> for PointComb<D> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        PointComb {
            point: Point {
                coords: self.point.coords * rhs,
            },
            factor: self.factor * rhs,
        }
    }
}

impl<const D: usize> std::ops::Mul<PointComb<D>> for f32 {
    type Output = PointComb<D>;

    fn mul(self, rhs: PointComb<D>) -> Self::Output {
        PointComb {
            point: Point {
                coords: rhs.point.coords * self,
            },
            factor: self * rhs.factor,
        }
    }
}

impl<const D: usize> std::ops::Add for Coords<D> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(std::array::from_fn(|i| self.0[i] + rhs.0[i]))
    }
}

impl<const D: usize> std::ops::Sub for Coords<D> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(std::array::from_fn(|i| self.0[i] - rhs.0[i]))
    }
}

impl<const D: usize> std::ops::Mul for Coords<D> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(std::array::from_fn(|i| self.0[i] * rhs.0[i]))
    }
}

impl<const D: usize> std::ops::Mul<f32> for Coords<D> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(std::array::from_fn(|i| self.0[i] * rhs))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub_points() {
        let a = Point::<2>::new(1., 3.);
        let b = Point::<2>::new(2., 5.);

        let result = a - b;
        assert_eq!(result, Vector::<2>::new(-1., -2.));
    }

    #[test]
    fn add_vec_to_point() {
        let point = Point::<2>::new(1., 2.);
        let vec = Vector::<2>::new(7., 3.);

        let result = point + vec;
        assert_eq!(result, Point::<2>::new(8., 5.));
    }

    #[test]
    fn mul_vec_float() {
        let vec = Vector::<2>::new(8., 3.);
        let float = 0.5;

        let result = vec * float;
        assert_eq!(result, Vector::<2>::new(4., 1.5));
    }

    #[test]
    fn add_vecs() {
        let a = Vector::<2>::new(1., 3.);
        let b = Vector::<2>::new(-1., -2.);

        let result = a + b;
        assert_eq!(result, Vector::<2>::new(0., 1.));
    }

    #[test]
    fn affine_comb() {
        let a = Point::<2>::new(1., 3.);
        let b = Point::<2>::new(2., 5.);
        let c = Point::<2>::new(-1., 2.);

        let comb = (1. / 4.) * a + (2. / 4.) * b + (1. / 4.) * c;

        assert_eq!(comb.into_point(), Point::<2>::new(1., 3.75));
    }

    #[test]
    #[should_panic]
    fn affine_comb_fail() {
        let a = Point::<2>::new(1., 3.);
        let b = Point::<2>::new(2., 5.);
        let c = Point::<2>::new(-1., 2.);

        let comb = (1. / 4.) * a + (3. / 4.) * b + (1. / 4.) * c;

        let _ = comb.into_point();
    }
}
