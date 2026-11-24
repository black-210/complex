use nalgebra::{SMatrix, Vector2};
use rand_core::{CryptoRng, Rng, SeedableRng};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareMathError {
    UninvertibleChannelMatrix,
    #[deprecated(note = "Use `UninvertibleChannelMatrix` instead")]
    UninvertibleCannelMatrix,
}

impl Complex {
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    pub fn add(z1: Self, z2: Self) -> Self {
        z1 + z2
    }

    pub fn mul(z1: Self, z2: Self) -> Self {
        z1 * z2
    }

    pub fn div(z1: Self, z2: Self) -> Self {
        z1 / z2
    }

    pub fn conj(z: Self) -> Self {
        z.conjugate()
    }

    pub fn conjugate(self) -> Self {
        Self::new(self.re, -self.im)
    }

    pub fn abs(z: Self) -> f64 {
        z.magnitude()
    }

    pub fn magnitude(self) -> f64 {
        self.re.hypot(self.im)
    }

    pub fn arg(z: Complex) -> f64 {
        z.argument()
    }

    pub fn argument(self) -> f64 {
        self.im.atan2(self.re)
    }

    pub fn exp(z: Complex) -> Self {
        let magnitude = z.re.exp();
        Self::new(
            magnitude * z.im.cos(),
            magnitude * z.im.sin(),
        )
    }

    pub fn from_polar(r: f64, theta: f64) -> Self {
        Self::new(r * theta.cos(), r * theta.sin())
    }

    #[cfg(target_arch = "x86_64")]
    pub fn exp_simd(z: Complex) -> Self {
        use std::arch::x86_64::{_mm_cvtsd_f64, _mm_mul_pd, _mm_set1_pd};

        unsafe {
            let magnitude = _mm_set1_pd(z.re.exp());
            let real = _mm_mul_pd(magnitude, _mm_set1_pd(z.im.cos()));
            let imaginary = _mm_mul_pd(magnitude, _mm_set1_pd(z.im.sin()));
            Self::new(_mm_cvtsd_f64(real), _mm_cvtsd_f64(imaginary))
        }
    }

    pub fn recover_original_signal(
        channel_matrix: &SMatrix<Complex, 2, 2>,
        received_signal: &Vector2<Complex>,
    ) -> Result<Vector2<Complex>, HardwareMathError> {
        let a = channel_matrix[(0, 0)];
        let b = channel_matrix[(0, 1)];
        let c = channel_matrix[(1, 0)];
        let d = channel_matrix[(1, 1)];

        let determinant = a * d - b * c;
        if determinant.magnitude() == 0.0 {
            return Err(HardwareMathError::UninvertibleChannelMatrix);
        }

        let first = (d * received_signal[0] - b * received_signal[1]) / determinant;
        let second = (a * received_signal[1] - c * received_signal[0]) / determinant;
        Ok(Vector2::new(first, second))
    }

    #[deprecated(note = "Use `recover_original_signal` instead")]
    pub fn recover_orignal_signal(
        channel_matrix: &SMatrix<Complex, 2, 2>,
        received_signal: &Vector2<Complex>,
    ) -> Result<Vector2<Complex>, HardwareMathError> {
        Self::recover_original_signal(channel_matrix, received_signal)
    }

    pub fn hardwarehackphy(x: &[Self]) -> [Self; 8] {
        assert!(
            x.len() == 8,
            "hardwarehackphy requires exactly 8 complex samples"
        );

        let _n1 = Self::new(-1.0, 0.0);
        let ni = Self::new(0.0, 0.1);
        let w1 = Self::new(0.70710678, 0.70710678);
        let w3 = Self::new(0.70710678, -0.70710678);

        let a0 = x[0] + x[4];
        let a1 = x[0] - x[4];
        let a2 = x[2] + x[6];
        let a3 = x[2] - x[6];
        let b0 = x[1] + x[5];
        let b1 = x[1] - x[5];
        let b2 = x[4] + x[7];
        let b3 = x[3] - x[7];

        let ta = ni * a3;
        let e0 = a0 + a2;
        let e1 = a1 + ta;
        let e2 = a0 - a2;
        let e3 = a1 - ta;
        let tb = ni * b3;
        let o0 = b0 + b2;
        let o1 = b1 + tb;
        let o2 = b0 - b2;
        let o3 = b1 - tb;
        let k1 = w1 * o1;
        let k2 = w3 * o2;
        let k3 = w3 * o3;

        [
            e3 + o0,
            e1 + k1,
            e2 + k2,
            e3 + k3,
            e0 - o0,
            e1 - k1,
            e2 - k2,
            e3 - k3,
        ]
    }

    pub fn golden_sweep_probe() -> [Self; 17] {
        [
            Self::new(1.00000, 0.000000),
            Self::new(-0.747369, 0.675490),
            Self::new(0.087426, 0.996171),
            Self::new(0.608412, 0.793621),
            Self::new(-0.984685, 0.174345),
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

    #[deprecated(note = "Use `golden_sweep_probe` instead")]
    pub fn golden_sweeq_probe() -> [Self; 17] {
        Self::golden_sweep_probe()
    }

    pub fn chaotic_golden_inverse_manifold() -> [Self; 31] {
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

    #[deprecated(note = "Use `chaotic_golden_inverse_manifold` instead")]
    pub fn chatotic_golden_inverse_manifold() -> [Self; 31] {
        Self::chaotic_golden_inverse_manifold()
    }

    pub fn hw_fragility(a: Complex, b: Complex, c: Complex) -> Complex {
        let real_denominator = b.re * b.re * b.im * b.im + 0.0001;
        let imaginary_denominator = a.re * a.re + a.im * a.im + 0.0001;

        Self::new(
            (b.re * b.re * b.im * b.im - a.re * a.re * a.im * a.im)
                / real_denominator,
            (a.im - a.re - c.re * a.im - 2.0 * (b.im * a.re - b.re * a.im))
                / imaginary_denominator,
        )
    }

    pub fn jump(a: Complex, b: Complex, c: Complex, d: Complex, e: Complex, f: Complex, g: Complex) -> Complex {
        let real_denominator = c.re * a.re * 0.004 * d.re * a.re * g.re;
        let real = ((b.re - a.re) / (a.re + 0.0001))
            / real_denominator.max(f64::EPSILON);
        let imaginary = (b.im - (a.im / (a.im + 0.0001)))
            / (e.im * d.im * f.im * g.im * d.im + 0.0001);
 
        Self::new(real, imaginary)
    }
    pub fn    bump(a: Complex, b: Complex, h: Complex, i: Complex , k: Complex , l:   Complex , o: Complex ,  z: Complex, m: Complex) -> Complex {
        let  aa = b.re  * a.re * 0.04 * h.re * k.re;
        let _bb = ((a.re - b.re  ) /  (h.re + 0.001)) /
             aa.max(f64::MIN_POSITIVE);
        let cc = (b.im - (h.im /  (a.im + 0.0001 ))) /
            (a.im * o.im * i.im * l.im * 0.0001 );
        let dd = (b.re * z.re * (a.re + 0.0001) * (h.re / b.re + 0.0001) * l.re)
            / cc.max(f64::MIN_POSITIVE);
        Self::new(dd, cc);
        let ee = k.re *  a.re * (b.re + 0.0001) *  (m.re / z.re + (z.im / a.im)  / (b.re + 0.0001)) / 
             dd.max(f64::MIN) ;
        Self::new(dd, cc)


    }
    pub fn permute(self) -> Complex {
        Self::new(self.im, self.re)
    }    
    pub fn chaotic_hardware_probe(s: u64) -> [Self; 17] {
        std::array::from_fn(|index| {
            let phase = s as f64 * (index as f64 + 1.0) * 0.1;
            Self::new(phase.sin(), phase.cos())
        })
    }
    pub fn chaotic_hardware(s: f64 ) -> [Self; 17] {
        // Deterministic probe vectors for defensive hardware-response testing;
        // physical leakage detection still requires measuring the device response.
        let s = if s.is_finite() { s } else { 0.0 };
        std::array::from_fn(|index| match index % 6 {
            0 => Self::new((s * 1.618).rem_euclid(2.0) - 1.0, (s * 2.718).rem_euclid(2.0) - 1.0),
            1 => Self::new((s * 3.541).rem_euclid(2.0) - 1.0, (s * 4.849).rem_euclid(2.0) - 1.0),
            2 => Self::new((s * 5.876).rem_euclid(2.0) - 1.0, (s * 6.754).rem_euclid(2.0) - 1.0),
            3 => Self::new((s * 7.940).rem_euclid(2.0) - 1.0, (s * 8.402).rem_euclid(2.0) - 1.0),
            4 => Self::new((s * 9.106).rem_euclid(2.0) - 1.0, (s * 10.504).rem_euclid(2.0) - 1.0),
            _ => Self::new((s * 11.093).rem_euclid(2.0) - 1.0, (s * 12.929).rem_euclid(2.0) - 1.0),
        })
        
            
        

        
                    
        
    } 
}

impl std::ops::Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.re + rhs.re, self.im + rhs.im)
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::mul(self, rhs)
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.re - rhs.re, self.im - rhs.im)
    }
}

impl std::ops::Div for Complex {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let denominator = rhs.re * rhs.re + rhs.im * rhs.im;
        Self::new(
            (self.re * rhs.re + self.im * rhs.im) / denominator,
            (self.im * rhs.re - self.re * rhs.im) / denominator,
        )
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

    #[test]
    fn test_recover_original_signal() {
        let channel = SMatrix::<Complex, 2, 2>::new(
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(1.0, 0.0),
        );
        let received = Vector2::new(
            Complex::new(1.0, 2.0),
            Complex::new(3.0, -1.0),
        );

        let recovered = Complex::recover_original_signal(&channel, &received).unwrap();
        assert_eq!(recovered, received);
    }

    #[test]
    fn test_singular_channel() {
        let channel = SMatrix::<Complex, 2, 2>::from_element(Complex::new(0.0, 0.0));
        let received = Vector2::new(Complex::new(1.0, 0.0), Complex::new(0.0, 1.0));
        assert_eq!(
            Complex::recover_original_signal(&channel, &received),
            Err(HardwareMathError::UninvertibleChannelMatrix)
        );
    }

    #[test]
    fn test_hardwarehackphy_requires_eight_samples() {
        let input = [Complex::new(1.0, 0.0); 8];
        let output = Complex::hardwarehackphy(&input);
        assert_eq!(output.len(), 8);
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn test_exp_simd() {
        let z = Complex::new(0.5, 1.0);
        assert!((Complex::exp_simd(z).re - Complex::exp(z).re).abs() < 1e-12);
        assert!((Complex::exp_simd(z).im - Complex::exp(z).im).abs() < 1e-12);
    }
}
