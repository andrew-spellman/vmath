use std::ops::{Add, Div, Mul, Neg, Sub};

use super::Real;

pub const I: Complex = Complex {
    real: 0.0,
    imaginary: 1.0,
};

#[derive(Debug, Clone, Copy, PartialEq)]
/// a + bi
/// where a and b are real numbers
pub struct Complex {
    pub real: Real,
    pub imaginary: Real,
}

impl Complex {
    /// ```
    /// # use vmath::numbers::Complex;
    /// let z = Complex::new(1.0, 2.0);
    /// assert_eq!(z.real, 1.0);
    /// assert_eq!(z.imaginary, 2.0);
    /// ```
    pub fn new(real: Real, imaginary: Real) -> Self {
        Self { real, imaginary }
    }

    /// ```
    /// # use vmath::numbers::Complex;
    /// # use std::f64::consts::PI;
    /// let z = Complex::new(3.0_f64.sqrt() / 2.0, 0.5);
    /// let angle = z.angle();
    /// assert!((angle - PI / 6.0).abs() < f64::EPSILON);
    /// ```
    pub fn angle(self) -> Real {
        if self.real == 0.0 {
            return Real::NAN;
        }
        (self.imaginary / self.real).atan()
    }

    /// ```
    /// # use vmath::numbers::Complex;
    /// let z = Complex::new(1.0, 2.0);
    /// let conjugate = z.conjugate();
    /// assert_eq!(conjugate, Complex::new(1.0, -2.0));
    /// ```
    pub fn conjugate(mut self) -> Self {
        self.imaginary *= -1.0;
        self
    }

    /// ```
    /// # use vmath::numbers::Complex;
    /// let z = Complex::new(3.0, 4.0);
    /// let norm_squared = z.norm_squared();
    /// assert_eq!(norm_squared, 25.0);
    /// ```
    pub fn norm_squared(self) -> Real {
        self.real * self.real + self.imaginary * self.imaginary
    }

    /// ```
    /// # use vmath::numbers::Complex;
    /// let z = Complex::new(3.0, 4.0);
    /// let norm = z.norm();
    /// assert_eq!(norm, 5.0);
    /// ```
    pub fn norm(self) -> Real {
        self.norm_squared().sqrt()
    }
}

impl From<Real> for Complex {
    fn from(real: Real) -> Complex {
        Complex {
            real,
            imaginary: 0.0,
        }
    }
}

impl Neg for Complex {
    type Output = Self;
    /// ```
    /// # use vmath::numbers::Complex;
    /// let z = Complex::new(1.0, 2.0);
    /// let negative = -z;
    /// assert_eq!(negative, Complex::new(-1.0, -2.0));
    /// ```
    fn neg(self) -> Self {
        Complex {
            real: -self.real,
            imaginary: -self.imaginary,
        }
    }
}

impl Add for Complex {
    type Output = Self;
    /// ```
    /// # use vmath::numbers::Complex;
    /// let z = Complex::new(1.0, 2.0);
    /// let w = Complex::new(3.0, 4.0);
    /// let sum = z + w;
    /// assert_eq!(sum, Complex::new(4.0, 6.0));
    /// ```
    fn add(self, rhs: Self) -> Self {
        Self {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

impl Add<Real> for Complex {
    type Output = Self;
    /// ```
    /// # use vmath::numbers::Complex;
    /// let z = Complex::new(1.0, 2.0);
    /// let x = 3.0;
    /// let sum = z + x;
    /// assert_eq!(sum, Complex::new(4.0, 2.0));
    /// ```
    fn add(self, rhs: Real) -> Self {
        Complex {
            real: self.real + rhs,
            imaginary: self.imaginary,
        }
    }
}

impl Sub for Complex {
    type Output = Self;
    /// ```
    /// # use vmath::numbers::Complex;
    /// let z = Complex::new(4.0, 5.0);
    /// let w = Complex::new(1.0, 3.0);
    /// let difference = z - w;
    /// assert_eq!(difference, Complex::new(3.0, 2.0));
    /// ```
    fn sub(self, rhs: Self) -> Self {
        self + -rhs
    }
}

impl Sub<Real> for Complex {
    type Output = Self;
    /// ```
    /// # use vmath::numbers::Complex;
    /// let z = Complex::new(1.0, 2.0);
    /// let x = 3.0;
    /// let difference = z - x;
    /// assert_eq!(difference, Complex::new(-2.0, 2.0));
    /// ```
    fn sub(self, rhs: Real) -> Self {
        self + -rhs
    }
}

impl Mul for Complex {
    type Output = Self;
    /// ```
    /// # use vmath::numbers::Complex;
    /// # use std::f64::consts::SQRT_2;
    /// // z with angle pi / 4
    /// let z = Complex::new(SQRT_2 / 2.0, SQRT_2 / 2.0);
    /// // w with angle pi / 2
    /// let w = Complex::new(0.0, 1.0);
    /// // product with angle 3pi / 4
    /// let product = z * w;
    /// assert_eq!(product, Complex::new(-SQRT_2 / 2.0, SQRT_2 / 2.0));
    /// ```
    fn mul(self, rhs: Self) -> Self {
        // (a_0 + b_0 * i) * (a_1 + b_1 * i)
        // (a_0 * a_1) + (a_0 * b_1 * i) + (b_0 * a_1 * i) + (b_0 * b_1 * i^2)
        // ((a_0 * a_1) - (b_0 * b_1)) + i * ((a_0 * b_1) + (b_0 * a_1))
        Self {
            real: self.real * rhs.real - self.imaginary * rhs.imaginary,
            imaginary: self.real * rhs.imaginary + self.imaginary * rhs.real,
        }
    }
}

impl Mul<Real> for Complex {
    type Output = Self;
    /// ```
    /// # use vmath::numbers::Complex;
    /// let z = Complex::new(1.0, 2.0);
    /// let x = 2.0;
    /// let product = z * x;
    /// assert_eq!(product, Complex::new(2.0, 4.0));
    /// ```
    fn mul(self, rhs: Real) -> Self {
        Self {
            real: self.real * rhs,
            imaginary: self.imaginary * rhs,
        }
    }
}

impl Div for Complex {
    type Output = Self;
    /// ```
    /// # use vmath::numbers::Complex;
    /// # use std::f64::consts::SQRT_2;
    /// // z with angle 3pi / 4
    /// let z = Complex::new(-SQRT_2 / 2.0, SQRT_2 / 2.0);
    /// // w with angle pi / 4
    /// let w = Complex::new(SQRT_2 / 2.0, SQRT_2 / 2.0);
    /// // quotient with angle pi / 2
    /// let quotient = z / w;
    /// assert_eq!(quotient, Complex::new(0.0, 1.0));
    /// ```
    fn div(self, rhs: Self) -> Self {
        // (a_0 + b_0 * i) / (a_1 + b_1 * i)
        // ((a_0 + b_0 * i) * (a_1 - b_1 * i)) / ((a_1 + b_1 * i) * (a_1 - b_1 * i))
        // expand the denominator:
        // (a_1 + b_1 * i) * (a_1 - b_1 * i) = a_1^2 - (b_1 * i)^2
        // = a_1^2 - (-b_1^2) = a_1^2 + b_1^2
        // expand the numerator:
        // (a_0 + b_0 * i) * (a_1 - b_1 * i)
        // = a_0 * a_1 - a_0 * b_1 * i + b_0 * a_1 * i - b_0 * b_1 * i^2
        // = (a_0 * a_1 + b_0 * b_1) + (b_0 * a_1 - a_0 * b_1) * i
        // substitute:
        // ((a_0 * a_1 + b_0 * b_1) / (a_1^2 + b_1^2)) + ((b_0 * a_1 - a_0 * b_1) / (a_1^2 + b_1^2)) * i
        Self {
            real: (self.real * rhs.real + self.imaginary * rhs.imaginary) / rhs.norm_squared(),
            imaginary: (self.imaginary * rhs.real - self.real * rhs.imaginary) / rhs.norm_squared(),
        }
    }
}

impl Div<Real> for Complex {
    type Output = Self;
    /// ```
    /// # use vmath::numbers::Complex;
    /// # use std::f64::consts::SQRT_2;
    /// let z = Complex::new(2.0, 4.0);
    /// let x = 2.0;
    /// let quotient = z / x;
    /// assert_eq!(quotient, Complex::new(1.0, 2.0));
    /// ```
    fn div(self, rhs: Real) -> Self {
        Self {
            real: self.real / rhs,
            imaginary: self.imaginary / rhs,
        }
    }
}
