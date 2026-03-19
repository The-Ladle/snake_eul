use core::ops::{Add, Mul, Sub};
use micromath::F32Ext;
#[derive(Debug)]
pub struct Vector<const N: usize> {
    data: [f32; N],
}

pub type Vec2 = Vector<2>;
pub type Vec3 = Vector<3>;

impl<const N: usize> Vector<N> {
    #[inline]
    pub const fn new(data: [f32; N]) -> Self {
        Self { data }
    }

    #[inline]
    pub const fn zero() -> Self {
        Self::new([0.0; N])
    }

    #[inline]
    pub const fn ones() -> Self {
        Self::new([1.0; N])
    }

    pub fn from_slice(slice: &[f32]) -> Self {
        assert_eq!(slice.len(), N, "slice length mismatch");
        let mut data = [0.0f32; N];
        data.copy_from_slice(slice);
        Self {data}
    }

    #[inline]
    pub fn as_slice(&self) -> &[f32] {
        &self.data
    }

    #[inline]
    pub const fn len(&self) -> usize {
        N
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        let mut total: f32 = 0.0;
        for i in 0..N {
            total += self.data[i] * rhs.data[i];
        }
        total
    }
}

impl Vec2 {
    pub const fn xy(x: f32, y: f32) -> Self {
        Self::new([x, y])
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }

    pub fn from_angle(angle: f32) -> Self {
        Self::new([angle.cos(), angle.sin()])
    }

    pub fn angle(&self) -> f32 {
        self.data[1].atan2(self.data[0])
    }
}
impl Vec3 {
    pub const fn xyz(x: f32, y: f32, z: f32) -> Self {
        Self::new([x, y, z])
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }
    pub fn y(&self) -> f32 {
        self.data[1]
    }
    pub fn z(&self) -> f32 {
        self.data[2]
    }

    pub fn from_yaw_pitch(yaw: f32, pitch: f32) -> Self {
        let hypot = pitch.cos();
        Self::new([yaw.cos() * hypot, yaw.sin() * hypot, pitch.sin()])
    }

    pub fn yaw_pitch(&self) -> (f32, f32) {
        (self.data[2].asin(), self.data[1].atan2(self.data[0]))
    }
    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new([
            self.data[1] * rhs.data[2] - self.data[2] * rhs.data[1],
            self.data[2] * rhs.data[0] - self.data[0] * rhs.data[2],
            self.data[0] * rhs.data[1] - self.data[1] * rhs.data[0]
        ])
    }
}

impl<const N: usize> Add for Vector<N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut new_vec: [f32; N] = [0.0; N];
        for i in 0..N {
            new_vec[i] = self.data[i] + rhs.data[i];
        }
        Self::new(new_vec)
    }
}

impl<const N: usize> Mul<f32> for Vector<N> {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut new_vec: [f32; N] = self.data;
        for i in 0..N {
            new_vec[i] = rhs * self.data[i];
        }
        Self::new(new_vec)
    }
}

impl<const N: usize> Mul<Vector<N>> for f32 {
    type Output = Vector<N>;

    fn mul(self, rhs: Vector<N>) -> Self::Output {
        let mut new_vec: [f32; N] = rhs.data;
        for i in 0..N {
            new_vec[i] = self * rhs.data[i];
        }
        Vector::new(new_vec)
    }
}

impl<const N: usize> PartialEq<Self> for Vector<N> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..N {
            if self.data[i] != other.data[i] {
                return false;
            }
        }
        true
    }
}

impl<const N: usize> Eq for Vector<N> {}