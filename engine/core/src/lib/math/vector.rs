use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::{Sqrt, Unit, Zero};

#[repr(C)]
pub struct Vector<T, const N: usize>([T; N]);

impl<T, const N: usize> Vector<T, N> {
  pub const PARTS: usize = N;

  pub fn from_parts(parts: [T; N]) -> Self {
    Self(parts)
  }

  pub fn parts(&self) -> &[T; N] {
    &self.0
  }
}

impl<T: Clone, const N: usize> Clone for Vector<T, N> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T: Copy, const N: usize> Copy for Vector<T, N> {}

impl<T: Copy + Zero + AddAssign + Mul<T, Output = T>, const N: usize> Vector<T, N> {
  pub fn sq_magnitude(&self) -> T {
    let mut ret = T::zero();
    for i in 0..N {
      ret += self.0[i] * self.0[i];
    }
    ret
  }

  pub fn dot(&self, rhs: &Self) -> T {
    let mut ret = T::zero();
    for i in 0..N {
      ret += self.0[i] * rhs.0[i];
    }
    ret
  }
}

impl<T: Copy + Zero + AddAssign + Mul<T, Output = T> + Sqrt, const N: usize> Vector<T, N> {
  pub fn magnitude(&self) -> T {
    self.sq_magnitude().sqrt()
  }
}

impl<
    T: Copy + Zero + PartialEq + DivAssign + AddAssign + Mul<T, Output = T> + Sqrt,
    const N: usize,
  > Vector<T, N>
{
  pub fn normalize(&mut self) -> T {
    let mag = self.magnitude();
    if mag != T::zero() {
      for i in 0..N {
        self.0[i] /= mag;
      }
    }
    mag
  }

  pub fn normalized(&self) -> Self {
    let mut ret = *self;
    ret.normalize();
    ret
  }
}

impl<T: std::fmt::Debug, const N: usize> std::fmt::Debug for Vector<T, N> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("Vector").field(&self.0).finish()
  }
}

impl<T: std::fmt::Display, const N: usize> std::fmt::Display for Vector<T, N> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "({})",
      self
        .0
        .iter()
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join(", ")
    )
  }
}

impl<T: Copy + Zero, const N: usize> Zero for Vector<T, N> {
  fn zero() -> Self {
    Self::from_parts([T::zero(); N])
  }
}

impl<T: Default + Copy, const N: usize> Default for Vector<T, N> {
  fn default() -> Self {
    Self::from_parts([T::default(); N])
  }
}

impl<T: Unit + Copy, const N: usize> Unit for Vector<T, N> {
  fn unit() -> Self {
    Self::from_parts([T::unit(); N])
  }
}

impl<T: PartialEq + Copy, const N: usize> PartialEq for Vector<T, N> {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl<T: PartialOrd + Copy, const N: usize> PartialOrd for Vector<T, N> {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    self.0.partial_cmp(&other.0)
  }
}

impl<T: std::hash::Hash, const N: usize> std::hash::Hash for Vector<T, N> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.0.hash(state);
  }
}

impl<T: AddAssign + Copy, const N: usize> AddAssign<Vector<T, N>> for Vector<T, N> {
  fn add_assign(&mut self, rhs: Vector<T, N>) {
    for i in 0..N {
      self.0[i] += rhs.0[i];
    }
  }
}

impl<T: AddAssign + Copy, const N: usize> Add for Vector<T, N> {
  type Output = Self;

  fn add(mut self, rhs: Self) -> Self::Output {
    self.add_assign(rhs);
    self
  }
}

impl<T: SubAssign + Copy, const N: usize> SubAssign<Vector<T, N>> for Vector<T, N> {
  fn sub_assign(&mut self, rhs: Vector<T, N>) {
    for i in 0..N {
      self.0[i] -= rhs.0[i];
    }
  }
}

impl<T: SubAssign + Copy, const N: usize> Sub for Vector<T, N> {
  type Output = Self;

  fn sub(mut self, rhs: Self) -> Self::Output {
    self.sub_assign(rhs);
    self
  }
}

impl<T: MulAssign + Copy, const N: usize> MulAssign<Vector<T, N>> for Vector<T, N> {
  fn mul_assign(&mut self, rhs: Vector<T, N>) {
    for i in 0..N {
      self.0[i] *= rhs.0[i];
    }
  }
}

impl<T: MulAssign + Copy, const N: usize> Mul for Vector<T, N> {
  type Output = Self;

  fn mul(mut self, rhs: Self) -> Self::Output {
    self.mul_assign(rhs);
    self
  }
}

impl<T: DivAssign + Copy, const N: usize> DivAssign<Vector<T, N>> for Vector<T, N> {
  fn div_assign(&mut self, rhs: Vector<T, N>) {
    for i in 0..N {
      self.0[i] /= rhs.0[i];
    }
  }
}

impl<T: DivAssign + Copy, const N: usize> Div for Vector<T, N> {
  type Output = Self;

  fn div(mut self, rhs: Self) -> Self::Output {
    self.div_assign(rhs);
    self
  }
}

impl<T: Copy> Vector<T, 1> {
  pub fn new(x: T) -> Self {
    Self::from_parts([x])
  }

  pub fn x(&self) -> T {
    self.0[0]
  }
}

impl<T: Copy> Vector<T, 2> {
  pub fn new(x: T, y: T) -> Self {
    Self::from_parts([x, y])
  }

  pub fn x(&self) -> T {
    self.0[0]
  }

  pub fn y(&self) -> T {
    self.0[1]
  }
}

impl<T: Copy + Mul<T, Output = T> + Sub<T, Output = T>> Vector<T, 2> {
  pub fn cross(&self, rhs: &Self) -> T {
    self.0[0] * rhs.0[1] - self.0[1] * rhs.0[0]
  }
}

impl<T: Copy + Unit + Zero> Vector<T, 2> {
  pub fn unit_x() -> Self {
    Self::new(T::unit(), T::zero())
  }
  pub fn unit_y() -> Self {
    Self::new(T::zero(), T::unit())
  }
}

impl<T: Copy> Vector<T, 3> {
  pub fn new(x: T, y: T, z: T) -> Self {
    Self::from_parts([x, y, z])
  }

  pub fn x(&self) -> T {
    self.0[0]
  }

  pub fn y(&self) -> T {
    self.0[1]
  }

  pub fn z(&self) -> T {
    self.0[2]
  }
}

impl<T: Copy + Mul<T, Output = T> + Sub<T, Output = T>> Vector<T, 3> {
  pub fn cross(&self, rhs: &Self) -> Self {
    Self::new(
      self.0[1] * rhs.0[2] - self.0[2] * rhs.0[1],
      self.0[2] * rhs.0[0] - self.0[0] * rhs.0[2],
      self.0[0] * rhs.0[1] - self.0[1] * rhs.0[0],
    )
  }
}

impl<T: Copy + Unit + Zero> Vector<T, 3> {
  pub fn unit_x() -> Self {
    Self::new(T::unit(), T::zero(), T::zero())
  }
  pub fn unit_y() -> Self {
    Self::new(T::zero(), T::unit(), T::zero())
  }
  pub fn unit_z() -> Self {
    Self::new(T::zero(), T::zero(), T::unit())
  }
}

impl<T: Copy> Vector<T, 4> {
  pub fn new(x: T, y: T, z: T, w: T) -> Self {
    Self::from_parts([x, y, z, w])
  }

  pub fn x(&self) -> T {
    self.0[0]
  }

  pub fn y(&self) -> T {
    self.0[1]
  }

  pub fn z(&self) -> T {
    self.0[2]
  }

  pub fn w(&self) -> T {
    self.0[3]
  }
}

impl<T: Copy + Unit + Zero> Vector<T, 4> {
  pub fn unit_x() -> Self {
    Self::new(T::unit(), T::zero(), T::zero(), T::unit())
  }
  pub fn unit_y() -> Self {
    Self::new(T::zero(), T::unit(), T::zero(), T::unit())
  }
  pub fn unit_z() -> Self {
    Self::new(T::zero(), T::zero(), T::unit(), T::unit())
  }
  pub fn unit_w() -> Self {
    Self::new(T::zero(), T::zero(), T::zero(), T::unit())
  }
}

impl<T: Copy + Zero + Add<T, Output = T> + Mul<T, Output = T> + Sub<T, Output = T>> Vector<T, 4> {
  pub fn cross(&self, rhs: &Self) -> Self {
    Self::new(
      self.0[1] * rhs.0[2] - self.0[2] * rhs.0[1],
      self.0[2] * rhs.0[0] - self.0[0] * rhs.0[2],
      self.0[0] * rhs.0[1] - self.0[1] * rhs.0[0],
      // T::zero(),
      self.0[0] * rhs.0[1] * rhs.0[2] - self.0[1] * rhs.0[0] * rhs.0[2]
        + self.0[2] * rhs.0[0] * rhs.0[1]
        - self.0[0] * rhs.0[2] * rhs.0[1],
    )
  }
}

macro_rules! decl_vec_types {
  ($( ($ty:ty, $size:expr, $name: ident) ),*) => {
    $(
      pub type $name = Vector<$ty, $size>;
    )*
  };
}

decl_vec_types!(
  (f32, 1, Vec1f32),
  (f32, 2, Vec2f32),
  (f32, 3, Vec3f32),
  (f32, 4, Vec4f32),
  (f64, 1, Vec1f64),
  (f64, 2, Vec2f64),
  (f64, 3, Vec3f64),
  (f64, 4, Vec4f64),
  (bool, 1, Vec1b),
  (bool, 2, Vec2b),
  (bool, 3, Vec3b),
  (bool, 4, Vec4b),
  (i8, 1, Vec1i8),
  (i8, 2, Vec2i8),
  (i8, 3, Vec3i8),
  (i8, 4, Vec4i8),
  (i16, 1, Vec1i16),
  (i16, 2, Vec2i16),
  (i16, 3, Vec3i16),
  (i16, 4, Vec4i16),
  (i32, 1, Vec1i32),
  (i32, 2, Vec2i32),
  (i32, 3, Vec3i32),
  (i32, 4, Vec4i32),
  (i64, 1, Vec1i64),
  (i64, 2, Vec2i64),
  (i64, 3, Vec3i64),
  (i64, 4, Vec4i64),
  (i128, 1, Vec1i128),
  (i128, 2, Vec2i128),
  (i128, 3, Vec3i128),
  (i128, 4, Vec4i128),
  (u8, 1, Vec1u8),
  (u8, 2, Vec2u8),
  (u8, 3, Vec3u8),
  (u8, 4, Vec4u8),
  (u16, 1, Vec1u16),
  (u16, 2, Vec2u16),
  (u16, 3, Vec3u16),
  (u16, 4, Vec4u16),
  (u32, 1, Vec1u32),
  (u32, 2, Vec2u32),
  (u32, 3, Vec3u32),
  (u32, 4, Vec4u32),
  (u64, 1, Vec1u64),
  (u64, 2, Vec2u64),
  (u64, 3, Vec3u64),
  (u64, 4, Vec4u64),
  (u128, 1, Vec1u128),
  (u128, 2, Vec2u128),
  (u128, 3, Vec3u128),
  (u128, 4, Vec4u128),
  (usize, 1, Vec1us),
  (usize, 2, Vec2us),
  (usize, 3, Vec3us),
  (usize, 4, Vec4us),
  (isize, 1, Vec1is),
  (isize, 2, Vec2is),
  (isize, 3, Vec3is),
  (isize, 4, Vec4is)
);

#[cfg(test)]
mod tests {
  use crate::{Sqrt, Unit, Vec2i8, Vec4u8, Zero};

  use super::{Vec2f32, Vec2u8};

  #[test]
  fn zero() {
    let v = Vec2f32::zero();
    assert_eq!(v.x(), 0f32);
    assert_eq!(v.y(), 0f32);
  }

  #[test]
  fn unit() {
    assert_eq!(Vec4u8::unit(), Vec4u8::new(1, 1, 1, 1));
    assert_eq!(Vec4u8::unit_x(), Vec4u8::new(1, 0, 0, 1));
    assert_eq!(Vec4u8::unit_y(), Vec4u8::new(0, 1, 0, 1));
    assert_eq!(Vec4u8::unit_z(), Vec4u8::new(0, 0, 1, 1));
    assert_eq!(Vec4u8::unit_w(), Vec4u8::new(0, 0, 0, 1));
  }

  #[test]
  fn add() {
    let v1 = Vec2u8::new(1, 2);
    let v2 = Vec2u8::new(3, 4);
    let v3 = v1 + v2;
    assert_eq!(v3, Vec2u8::new(4, 6));
  }

  #[test]
  fn sub() {
    let v1 = Vec2u8::new(3, 4);
    let v2 = Vec2u8::new(1, 2);
    let v3 = v1 - v2;
    assert_eq!(v3, Vec2u8::new(2, 2));
  }

  #[test]
  fn mul() {
    let v1 = Vec2u8::new(3, 4);
    let v2 = Vec2u8::new(1, 2);
    let v3 = v1 * v2;
    assert_eq!(v3, Vec2u8::new(3, 8));
  }

  #[test]
  fn div() {
    let v1 = Vec2u8::new(10, 20);
    let v2 = Vec2u8::new(2, 2);
    let v3 = v1 / v2;
    assert_eq!(v3, Vec2u8::new(5, 10));
  }

  #[test]
  fn squared_magnitude() {
    assert_eq!(Vec2u8::new(2, 3).sq_magnitude(), (2 * 2 + 3 * 3) as u8)
  }

  #[test]
  fn magnitude() {
    assert_eq!(
      Vec2u8::new(2, 3).magnitude(),
      ((2 * 2 + 3 * 3) as u8).sqrt()
    )
  }

  #[test]
  fn normalized() {
    let expected_mag = ((2 * 2 + 3 * 3) as u8).sqrt();
    let mut v = Vec2u8::new(2, 3);
    let mag = v.normalize();
    assert_eq!(mag, expected_mag);
    assert_eq!(
      v,
      Vec2u8::new(2, 3) / Vec2u8::new(expected_mag, expected_mag)
    )
  }

  #[test]
  fn dot() {
    assert_eq!(Vec2u8::new(2, 3).dot(&Vec2u8::new(4, 5)), 2 * 4 + 3 * 5);
  }

  #[test]
  fn cross() {
    assert_eq!(Vec2i8::new(2, 3).cross(&Vec2i8::new(4, 5)), -2);
  }
}
