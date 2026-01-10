// src/math_forest/algebra/complex/complex.rs
#![allow(dead_code)]

use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg, AddAssign, MulAssign, SubAssign, DivAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex {
    pub re: f64, // 实部 Real
    pub im: f64, // 虚部 Imaginary
}

impl Complex {
    // 常量定义
    pub const ZERO: Complex = Complex::new(0.0, 0.0);
    pub const ONE: Complex = Complex::new(1.0, 0.0);
    pub const I: Complex = Complex::new(0.0, 1.0);
    pub const NAN: Complex = Complex::new(f64::NAN, f64::NAN);

    #[inline(always)]
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    #[inline(always)]
    pub const fn from_real(re: f64) -> Self {
        Self { re, im: 0.0 }
    }

    #[inline]
    pub fn len_sq(&self) -> f64 { self.re * self.re + self.im * self.im }

    #[inline]
    pub fn len(&self) -> f64 { self.len_sq().sqrt() }

    #[inline]
    pub fn arg(&self) -> f64 { self.im.atan2(self.re) }

    pub fn min(self, other: Self) -> Self { if self.len_sq() > other.len_sq() { other } else { self } }
    pub fn max(self, other: Self) -> Self { if self.len_sq() > other.len_sq() { self } else { other } }

    pub fn is_zero(&self) -> bool { self.re.abs() < 1e-12 && self.im.abs() < 1e-12 }
    pub fn is_nan(&self) -> bool { self.re.is_nan() || self.im.is_nan() }

    #[inline]
    pub fn conj(self) -> Self { Self::new(self.re, -self.im) }

    pub fn reciprocal(self) -> Self {
        let den = self.len_sq();
        Self::new(self.re / den, -self.im / den)
    }

    pub fn ln(self) -> Self {
        Self::new(self.len().ln(), self.arg())
    }

    pub fn exp(self) -> Self {
        let r = self.re.exp();
        let (sin, cos) = self.im.sin_cos();
        Self::new(r * cos, r * sin)
    }

    // z^w = exp(w * ln(z))
    pub fn pow(self, other: Complex) -> Self {
        if self.is_zero() { return Self::ZERO; } // 简化处理
        (self.ln() * other).exp()
    }

    // z^x (x is real)
    pub fn powf(self, n: f64) -> Self {
        if self.is_zero() { return Self::ZERO; }
        (self.ln() * n).exp()
    }

    pub fn sqrt(self) -> Self {
        // 优化：避免昂贵的 log/exp，使用代数公式
        let r = self.len();
        // 主值：实部 >= 0
        let re_part = ((r + self.re) * 0.5).sqrt();
        let im_part = ((r - self.re) * 0.5).sqrt().copysign(self.im);
        Self::new(re_part, im_part)
    }

    pub fn sin(self) -> Self {
        Self::new(self.re.sin() * self.im.cosh(), self.re.cos() * self.im.sinh())
    }

    pub fn cos(self) -> Self {
        Self::new(self.re.cos() * self.im.cosh(), -self.re.sin() * self.im.sinh())
    }

    pub fn tan(self) -> Self {
        let two_re = 2.0 * self.re;
        let two_im = 2.0 * self.im;
        let den = two_re.cos() + two_im.cosh();
        Self::new(two_re.sin() / den, two_im.sinh() / den)
    }
}

// ====================== 运算符重载 ======================
// 为了简洁，这里只保留最核心的 Struct-Struct 和 Struct-f64
// 实际库中可以使用宏来减少重复

impl Add for Complex {
    type Output = Complex;
    #[inline] fn add(self, rhs: Self) -> Self::Output { Complex::new(self.re + rhs.re, self.im + rhs.im) }
}
impl Add<f64> for Complex {
    type Output = Complex;
    #[inline] fn add(self, rhs: f64) -> Self::Output { Complex::new(self.re + rhs, self.im) }
}
impl Add<Complex> for f64 {
    type Output = Complex;
    #[inline] fn add(self, rhs: Complex) -> Complex { Complex::new(self + rhs.re, rhs.im) }
}

impl Sub for Complex {
    type Output = Complex;
    #[inline] fn sub(self, rhs: Self) -> Self::Output { Complex::new(self.re - rhs.re, self.im - rhs.im) }
}
impl Sub<f64> for Complex {
    type Output = Complex;
    #[inline] fn sub(self, rhs: f64) -> Self::Output { Complex::new(self.re - rhs, self.im) }
}
impl Sub<Complex> for f64 {
    type Output = Complex;
    #[inline] fn sub(self, rhs: Complex) -> Complex { Complex::new(self - rhs.re, -rhs.im) }
}

impl Mul for Complex {
    type Output = Complex;
    #[inline] fn mul(self, rhs: Self) -> Self::Output {
        Complex::new(self.re * rhs.re - self.im * rhs.im, self.re * rhs.im + self.im * rhs.re)
    }
}
impl Mul<f64> for Complex {
    type Output = Complex;
    #[inline] fn mul(self, rhs: f64) -> Self::Output { Complex::new(self.re * rhs, self.im * rhs) }
}
impl Mul<Complex> for f64 {
    type Output = Complex;
    #[inline] fn mul(self, rhs: Complex) -> Complex { Complex::new(self * rhs.re, self * rhs.im) }
}

impl Div for Complex {
    type Output = Complex;
    fn div(self, rhs: Self) -> Self::Output {
        let den = rhs.re * rhs.re + rhs.im * rhs.im;
        Complex::new((self.re * rhs.re + self.im * rhs.im) / den, (self.im * rhs.re - self.re * rhs.im) / den)
    }
}
impl Div<f64> for Complex {
    type Output = Complex;
    #[inline] fn div(self, rhs: f64) -> Self::Output { Complex::new(self.re / rhs, self.im / rhs) }
}
impl Div<Complex> for f64 {
    type Output = Complex;
    fn div(self, rhs: Complex) -> Complex {
        let den = rhs.len_sq();
        Complex::new(self * rhs.re / den, -self * rhs.im / den)
    }
}

impl Neg for Complex {
    type Output = Complex;
    #[inline] fn neg(self) -> Complex { Complex::new(-self.re, -self.im) }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, rhs: Self) { self.re += rhs.re; self.im += rhs.im; }
}
impl SubAssign for Complex {
    fn sub_assign(&mut self, rhs: Self) { self.re -= rhs.re; self.im -= rhs.im; }
}
impl MulAssign<f64> for Complex {
    fn mul_assign(&mut self, rhs: f64) { self.re *= rhs; self.im *= rhs; }
}
// 追加到 complex.rs 末尾

// ====================== 赋值运算符补全 (Assign Traits) ======================

impl AddAssign<f64> for Complex {
    #[inline] fn add_assign(&mut self, rhs: f64) { self.re += rhs; }
}

// -=
impl SubAssign<f64> for Complex {
    #[inline] fn sub_assign(&mut self, rhs: f64) { self.re -= rhs; }
}

// *=
impl MulAssign for Complex {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        let re = self.re * rhs.re - self.im * rhs.im;
        let im = self.re * rhs.im + self.im * rhs.re;
        self.re = re;
        self.im = im;
    }
}

// /= (你指出的缺失项)
impl DivAssign for Complex {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        let den = rhs.len_sq();
        let re = (self.re * rhs.re + self.im * rhs.im) / den;
        let im = (self.im * rhs.re - self.re * rhs.im) / den;
        self.re = re;
        self.im = im;
    }
}
impl DivAssign<f64> for Complex {
    #[inline] fn div_assign(&mut self, rhs: f64) { self.re /= rhs; self.im /= rhs; }
}
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 优化显示逻辑：0+3i -> 3i, 5+0i -> 5, 5-3i
        if self.im.abs() < 1e-12 { return write!(f, "{:.4}", self.re); }
        if self.re.abs() < 1e-12 { return write!(f, "{:.4}i", self.im); }
        let sign = if self.im < 0.0 { "-" } else { "+" };
        write!(f, "{:.4} {} {:.4}i", self.re, sign, self.im.abs())
    }
}