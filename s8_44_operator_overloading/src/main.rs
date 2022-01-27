use std::ops::{Add, AddAssign, Neg};
// Not required:
// use std::cmp::{PartialEq};

// Eq relies on PartialEq.
// Ord relies on PartialOrd.
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Complex<T>
{
    re: T,
    im: T
}

impl<T> Complex<T> {
    fn new(re: T, im: T) -> Complex<T> {
        Complex::<T> { re, im }
    }
}

impl<T> Add for Complex<T>
    where T: Add<Output = T>
{
    type Output = Complex<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

impl<T> AddAssign for Complex<T>
    where T: AddAssign<T>
{
    fn add_assign(&mut self, rhs: Self) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> Neg for Complex<T>
    where T: Neg<Output=T>
{
    type Output = Complex<T>;

    fn neg(self) -> Self::Output {
        Complex {
            re: -self.re,
            im: -self.im
        }
    }
}

// Not required, can be derivied.
// impl<T> PartialEq for Complex<T>
//     where T: PartialEq
// {
//     fn eq(&self, rhs: &Self) -> bool {
//         self.re == rhs.re &&  self.im == rhs.im
//     }
// }

// Not required, can be derivied.
// impl<T: Eq> Eq for Complex<T> where T: Eq {}

// partial eq
// full eq: x = x
// NAN = not a number 0/0 inf/inf
// NAN == NAN -> false
// So we cannot support full eq for floating point numbers.

fn main() {
    let mut a = Complex::new(1.0, 2.0);
    let mut b = Complex::new(3.0, 4.0);

    // Require the `Add` trait.
    // println!("{:?}", a + b);

    // Require the `AddAssign` trait.
    // a += b;
    // println!("{:?}", a);

    // println!("{:?}", -a);

    println!("{:?}", a == b);
}
