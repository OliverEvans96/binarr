#[cfg(test)]
pub mod test_utils {

    use num::{Complex, Float};

    pub fn approx_eq_real_arr<R>(a: &[R], b: &[R])
    where
        R: Float + approx::AbsDiffEq<Epsilon = R> + std::fmt::Debug,
        R: From<f32>,
        R::Epsilon: Copy,
    {
        assert_eq!(a.len(), b.len(), "len");

        let multiplier: R = 10.0.into();
        let eps = multiplier * R::epsilon();
        for i in 0..a.len() {
            approx::assert_abs_diff_eq!(a[i], b[i], epsilon = eps);
        }
    }

    pub fn approx_eq_complex_arr<R>(a: &[Complex<R>], b: &[Complex<R>])
    where
        R: Float + approx::AbsDiffEq<Epsilon = R> + std::fmt::Debug,
        R: From<f32>,
    {
        assert_eq!(a.len(), b.len(), "len");

        let multiplier: R = 10.0.into();
        let eps = multiplier * R::epsilon();
        for i in 0..a.len() {
            approx::assert_abs_diff_eq!(a[i].re, b[i].re, epsilon = eps);
            approx::assert_abs_diff_eq!(a[i].im, b[i].im, epsilon = eps);
        }
    }
}
