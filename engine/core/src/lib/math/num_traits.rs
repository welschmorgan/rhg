pub trait Zero {
  fn zero() -> Self;
}

macro_rules! zero_impl {
  ( $($typ:ty),* ) => {
    $(
    impl Zero for $typ {
      fn zero() -> Self {
        0 as $typ
      }
    }
  )*
  };
}

zero_impl!(f32, f64, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl Zero for bool {
  fn zero() -> Self {
    false
  }
}

pub trait Unit {
  fn unit() -> Self;
}

macro_rules! unit_impl {
  ( $($typ:ty),* ) => {
    $(
    impl Unit for $typ {
      fn unit() -> Self {
        1 as $typ
      }
    }
  )*
  };
}

unit_impl!(f32, f64, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

impl Unit for bool {
  fn unit() -> Self {
    true
  }
}

pub trait Sqrt {
  fn sqrt(&self) -> Self;
}

macro_rules! sqrt_impl {
  ( $($typ:ty),* ) => {
    $(
    impl Sqrt for $typ {
      fn sqrt(&self) -> Self {
        f64::sqrt(*self as f64) as $typ
      }
    }
  )*
  };
}

sqrt_impl!(f32, f64, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
