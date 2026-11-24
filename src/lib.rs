use nalgebra::{SMatrix, Vector2};

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
        use nalgebra::Complex;
        unsafe {
            let r = _mm_set1_pd(z.re.exp());
            let cos_theta = _mm_set1_pd(z.im.cos());
            let sin_theta = _mm_set1_pd(z.im.sin());
            let re = _mm_mul_pd(r, cos_theta);
            let im = _mm_mul_pd(r, sin_theta);
            let _swe: SMatrix<Complex<f64>, 2, 2> = SMatrix::<Complex<f64>, 2, 2>::new(
                Complex::new(2.0, 1.0), Complex::new(-1.0, 0.8),
                Complex::new(0.5, -1.0), Complex::new(3.8, 2.0  )
            );
            Complex::new(3.0 ,  4.0);
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
        let b0 = Self::add(x[1], x[5]);
        let b1 = Self::add(x[1], Self::mul(n1, x[5]));
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
        let _o2 = Self::add(b0, Self::mul(n1, b2));
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
    pub fn golden_sweeq_probe() -> [Self; 17] {
        [
        Self::new(1.00000, 0.000000),
        Self::new(-0.747369, 0.675490),
        Self::new(0.087426, 0.996171),
        Self::new(0.608412, 0.793621),
        Self::new(-0.984685,0.174345 ),
        Self::new(0.843580, -0.537003),
        Self::new(-0.259936, 0.965626),
        Self::new(-0.461487, -0.887147),
        Self::new(-0.461487, -0.887147),
        Self::new(0.939181, 0.343423),
        Self::new(-0.923880, 0.382683),
        Self::new(0.424372, -0.905488),
        Self::new(0.300706, 0.953717),
        Self::new(-0.867767, 0.496973),
        Self::new(0.974928, -0.819152),
        Self::new(-0.573576, -0.819152),
        Self::new(0.130526, 0.991445),


        ]


      
}
pub fn chatotic_golden_inverse_manifold() -> [Self; 31] {
    [
    Self::new(0.985410, 0.000000),
    Self::new(-0.726612, -0.66534), 
    Self::new(0.086150, 0.981638),
    Self::new(0.599536, 0.782042),
    Self::new(-0.970318, 0.171801),
    Self::new(0.831272, 0.529168),
    Self::new(-0.256143, -0.951537),
    Self::new(-0.45754, 0.874203),
    Self::new(0.925478, 0.338412),
    Self::new(-0.910401, 0.377099),
    Self::new(0.418180, 0.892277),
    Self::new(-0.855106, -0.489722),
    Self::new(0.960704, 0.219274),
    Self::new(-0.565207, 0.807200),
    Self::new(-0.128621, -0.976979),
    Self::new(0.941288, -0.289412),
    Self::new(-0.491023, 0.854119),
    Self::new(-0.334109, -0.927104),
    Self::new(0.894512, 0.412908),
    Self::new(-0.811045, 0.559102),
    Self::new(0.214890, -0.961894),
    Self::new(0.534018, 0.828115),
    Self::new(-0.953210, -0.251004),
    Self::new(0.779104, -0.603118),
    Self::new(-0.167412, 0.971205),
    Self::new(0.669812, -0.722419),
    Self::new(0.92415, 0.081904),
    Self::new(-0.608112, 0.775209),
    Self::new(-0.077105, -0.983114),
    Self::new(0.718209, 0.674118),
    Self::new(-0.986112, -0.004109),
     
    ]
}
pub fn hw_fragility(a: Complex, b: Complex, c: Complex,  ) -> Complex {
    Self::new(
        ((b.re * b.re * b.im * b.im ) - (a.re * a.re * a.im *  a.im  )) / (b.re * b.re * b.im * b.im +0.0001  ),
        ((a.im - a.re - c.re * a.im ) - 2.0 * (b.im * a.re - b.re * a.im )) / (a.re * a.re + a.im  * a.im + 0.0001 ),
    )
    
}
pub fn jump(a: Complex, b: Complex, c: Complex, d: Complex, e: Complex, f: Complex,  g: Complex) -> Complex {
    Self::new(
        ((b.re - a.re ) / (a.re  + 0.0001)) / ( c.re  * a.re  * 0.004 * d.re * a.re * g.re), 
        ((b.im - a.im / (a.im + 0.0001)) ) / (e.im * d.im *  f.im *  g.im * d.im + 0.0001 ),

    )
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
