use std::ops::{Add, Div, Mul, Sub};

use super::Complex;

pub type Real = f64;

impl Add<Complex> for Real {
    type Output = Complex;
    /// ```
    /// # use vmath::numbers::Complex;
    /// let x = 3.0;
    /// let z = Complex::new(1.0, 2.0);
    /// let sum = x + z;
    /// assert_eq!(sum, Complex::new(4.0, 2.0));
    /// ```
    fn add(self, rhs: Complex) -> Complex {
        rhs + self
    }
}

impl Sub<Complex> for Real {
    type Output = Complex;
    /// ```
    /// # use vmath::numbers::Complex;
    /// let x = 3.0;
    /// let z = Complex::new(1.0, 2.0);
    /// let difference = x - z;
    /// assert_eq!(difference, Complex::new(2.0, -2.0));
    /// ```
    fn sub(self, rhs: Complex) -> Complex {
        self + -rhs
    }
}

impl Mul<Complex> for Real {
    type Output = Complex;
    /// ```
    /// # use vmath::numbers::Complex;
    /// let x = 2.0;
    /// let z = Complex::new(1.0, 2.0);
    /// let product = x * z;
    /// assert_eq!(product, Complex::new(2.0, 4.0));
    /// ```
    fn mul(self, rhs: Complex) -> Complex {
        rhs * self
    }
}

impl Div<Complex> for Real {
    type Output = Complex;
    /// ```
    /// # use vmath::numbers::Complex;
    /// let x = 6.0;
    /// let z = Complex::new(0.0, 3.0);
    /// let quotient = x / z;
    /// assert_eq!(quotient, Complex::new(0.0, -2.0));
    /// ```
    fn div(self, rhs: Complex) -> Complex {
        Complex::from(self) / rhs
    }
}
