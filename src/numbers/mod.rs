use std::ops::{Add, Div, Mul, Sub};

type Real = f64;

#[derive(Debug, Clone, Copy)]
/// a + bi
/// where a and b are real numbers
pub struct Complex {
    real: Real,
    imaginary: Real,
}

impl Complex {
    fn new(real: Real, imaginary: Real) -> Self {
        Self { real, imaginary }
    }

    fn conjugate(mut self) -> Self {
        self.imaginary *= -1.0;
        self
    }

    fn norm(self) -> Real {
        self.real
    }
}

impl Add for Complex {
    type Output = Self;
    /// ```
    /// use std::f64::consts::SQRT_2;
    /// use vmath::numbers::Complex;
    /// // z with angle pi / 4
    /// let z = Complex::new(SQRT_2 / 2.0, SQRT_2 / 2.0);
    /// // w with angle pi / 2
    /// let w = Complex::new(0.0, 1.0);
    /// // result with angle 3pi / 4
    /// let result = z * w;
    /// assert_eq!(result.real, -SQRT_2 / 2.0);
    /// assert_eq!(result.imaginary, SQRT_2 / 2.0);
    /// ```
    fn add(self, rhs: Self) -> Self {
        Self {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

impl Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self {
            real: self.real - rhs.real,
            imaginary: self.imaginary - rhs.imaginary,
        }
    }
}

impl Mul for Complex {
    type Output = Self;
    /// (a_0 + b_0 * i) * (a_1 + b_1 * i)
    /// (a_0 * a_1) + (a_0 * b_1 * i) + (b_0 * a_1 * i) + (b_0 * b_1 * i^2)
    /// ((a_0 * a_1) - (b_0 * b_1)) + i * ((a_0 * b_1) + (b_0 * a_1))
    fn mul(self, rhs: Self) -> Self {
        Self {
            real: self.real * rhs.real - self.imaginary * rhs.imaginary,
            imaginary: self.real * rhs.imaginary + self.imaginary * rhs.real,
        }
    }
}

// impl Div for Complex

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complex_mul() {}
}
