use nalgebra::{SMatrix, Vector2, coordinates::X};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

pub enum HardwareMathError {
    UninvertibleCannelMatrix,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add(z1: Self, z2: Self) -> Self {
        Self::new(z1.re + z2.re, z1.im + z2.im)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn mul(z1: Self, z2: Self) -> Self {
        Self::new(z1.re * z2.re - z1.im * z2.im, z1.re * z2.im + z1.im * z2.re)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn div(z1: Self, z2: Self) -> Self {
        let d = z2.re * z2.re + z2.im * z2.im;
        Self::new(
            (z1.re * z2.re + z1.im * z2.im) / d,
            (z1.im * z2.re - z1.re * z2.im) / d,
        )
    }

    pub fn conj(z: Self) -> Self {
        Self::new(z.re, -z.im)
    }

    pub fn abs(z: Self) -> f64 {
        (z.re * z.re + z.im * z.im).sqrt()
    }

    pub fn arg(z: Complex) -> f64 {
        z.im.atan2(z.re)
    }

    pub fn exp(z: Complex) -> Self {
        let r = z.re.exp();
        let theta = z.im;
        Self::new(r * theta.cos(), r * theta.sin())
    }

    pub fn from_polar(r: f64, theta: f64) -> Self {
        Self::new(r * theta.cos(), r * theta.sin())
    }

    #[cfg(target_arch = "x86_64")]
    pub fn exp_simd(z: Complex) -> Self {
        use std::arch::x86_64::*;
        use nalgebra::{Complex, Matrix, Vector2};
        unsafe {
            let r = _mm_set1_pd(z.re.exp());
            let cos_theta = _mm_set1_pd(z.im.cos());
            let sin_theta = _mm_set1_pd(z.im.sin());
            let re = _mm_mul_pd(r, cos_theta);
            let im = _mm_mul_pd(r, sin_theta);
            let swe: SMatrix<Complex<f64>, 2, 2> = SMatrix::<Complex<f64>, 2, 2>::new(
                Complex::new(2.0, 1.0), Complex::new(-1.0, 0.8),
                Complex::new(0.5, -1.0), Complex::new(3.8, 2.0  )
            );
            Complex::new(3.0 ,  4.0);
            let received_signal = Vector2::new(
                Complex::new(1.0, 2.0),
                Complex::new(3.0 , -1.0)
                        );
            Self::new(_mm_cvtsd_f64(re), _mm_cvtsd_f64(im))
        }
    }
    pub fn recover_orignal_signal(
        channel_matrix: &SMatrix<Complex, 2, 2>,
        received_signal: &Vector2<Complex>,
    ) -> Result<Vector2<Complex>, HardwareMathError> {
        let a = channel_matrix[(0, 0)];
        let b = channel_matrix[(0, 1)];
        let c = channel_matrix[(1, 0)];
        let d = channel_matrix[(1, 1)];
        let bc = Self::mul(b, c);
        let determinant = Self::add(Self::mul(a, d), Self::new(-bc.re, -bc.im));

        if Self::abs(determinant) == 0.0 {
            return Err(HardwareMathError::UninvertibleCannelMatrix);
        }

        let by = Self::mul(b, received_signal[1]);
        let dx = Self::mul(d, received_signal[0]);
        let cx = Self::mul(c, received_signal[0]);
        let ay = Self::mul(a, received_signal[1]);
        let first = Self::div(Self::add(dx, Self::new(-by.re, -by.im)), determinant);
        let second = Self::div(Self::add(ay, Self::new(-cx.re, -cx.im)), determinant);

        Ok(Vector2::new(first, second))
    }
    pub fn hardwarehackphy(x: &[Self]) -> [Self; 8] {
        let n1 = Self::new(-1.0, -0.0);
        let ni = Self::new(0.0, 0.1);
        let w1 = Self::new(0.70710678, 0.70710678);
        let w3 = Self::new(0.70710678, -0.70710678);
        let a0 = Self::add( x[0], x[4]); 
        let a1 = Self::add(x[0], Self::mul(n1, x[4]) );
        let a2 = Self::add(x[2], x[6]);
        let a3 = Self::add(x[2], Self::mul(n1, x[6]));
        let b0 =  Self::add(x[1], x[5]);
        let b1 =  Self::add(x[1], Self::mul(n1, x[5]));
        let b2 = Self::add(x[4],  x[7]);
        let b3 = Self::add(x[3], Self::mul(n1, x[7]));
        let ta = Self::mul(ni, a3);
        let e0 = Self::add(a0, a2);
        let e1 = Self::add(a1, ta);
        let e2 = Self::add(a0, Self::mul(n1, a2));
        let e3 = Self::add(a1, Self::mul(n1, ta));
        let tb = Self::mul(ni, b3);
        let o0 = Self::add(b0, b2);
        let o1 = Self::add(b1, tb);
        let o2 = Self::add(b0, Self::mul(n1, b2));
        let o3 = Self::add(b1, Self::mul(n1, tb));
        let k1 = Self::mul(w1, o1);
        let k2 = Self::mul(w3, o3);
        let k3 = Self::mul(w3, o3);


        [
            Self::add(e3, o0),
            Self::add(e1, k1),
            Self::add(e2, k2),
            Self::add(e3, k3),
            Self::add(e0, Self::mul(n1, o0)),
            Self::add(e1, Self::mul(n1, k1)),
            Self::add(e2, Self::mul(n1, k2)),
            Self::add(e3, Self::mul(n1, k3)),

            


        ]
        
    }

    

}

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::add(self, rhs)
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::mul(self, rhs)
    }
}

impl std::ops::Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::div(self, rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let z = Complex::new(1.0, 2.0);
        assert_eq!(z.re, 1.0);
        assert_eq!(z.im, 2.0);
    }

    #[test]
    fn test_arithmetic() {
        let z1 = Complex::new(1.0, 2.0);
        let z2 = Complex::new(3.0, 4.0);

        assert_eq!(Complex::add(z1, z2), Complex::new(4.0, 6.0));
        assert_eq!(Complex::mul(z1, z2), Complex::new(-5.0, 10.0));
        assert_eq!(Complex::div(z1, z2), Complex::new(11.0 / 25.0, 2.0 / 25.0));
        assert_eq!(z1 + z2, Complex::new(4.0, 6.0));
        assert_eq!(z1 * z2, Complex::new(-5.0, 10.0));
        assert_eq!(z1 / z2, Complex::new(11.0 / 25.0, 2.0 / 25.0));
    }

    #[test]
    fn test_complex_helpers() {
        let z = Complex::new(3.0, 4.0);

        assert_eq!(Complex::conj(z), Complex::new(3.0, -4.0));
        assert_eq!(Complex::abs(z), 5.0);
        assert!((Complex::arg(z) - 4.0_f64.atan2(3.0)).abs() < f64::EPSILON);
        assert_eq!(Complex::from_polar(5.0, 0.0), Complex::new(5.0, 0.0));
    }

    #[test]
    fn test_exp() {
        let z = Complex::exp(Complex::new(0.0, 0.0));
        assert_eq!(z, Complex::new(1.0, 0.0));
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn test_exp_simd() {
        let z = Complex::new(0.5, 1.0);
        assert_eq!(Complex::exp_simd(z), Complex::exp(z));
    }
}
