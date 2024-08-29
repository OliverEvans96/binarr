#[cfg(test)]
mod tests {
    use num::complex::{Complex32, Complex64};

    use crate::utils::test_utils::{approx_eq_complex_arr, approx_eq_real_arr};

    #[test]
    fn test_serialize_real_f32() -> anyhow::Result<()> {
        let arr: Vec<f32> = vec![0.10857025, 0.41395682, 0.16843101, 0.57240325, 0.9430884];
        let expected = "[0.10857025,0.41395682,0.16843101,0.57240325,0.9430884]";

        let ser = serde_json::to_string(&arr)?;
        assert_eq!(&ser, expected);

        let deser: Vec<f32> = serde_json::from_str(&ser)?;
        approx_eq_real_arr(&arr, &deser);

        Ok(())
    }

    #[test]
    fn test_serialize_real_f64() -> anyhow::Result<()> {
        let arr: Vec<f64> = vec![
            0.4579429730336094,
            0.8354191333625977,
            0.643391447220919,
            0.5045068639499088,
            0.4720705364428668,
        ];
        let expected = "[0.4579429730336094,0.8354191333625977,0.643391447220919,0.5045068639499088,0.4720705364428668]";

        let ser = serde_json::to_string(&arr)?;
        assert_eq!(&ser, expected);

        let deser: Vec<f64> = serde_json::from_str(&ser)?;
        approx_eq_real_arr(&arr, &deser);

        Ok(())
    }

    #[test]
    fn test_serialize_complex_f32() -> anyhow::Result<()> {
        let arr = vec![
            Complex32::new(0.5448807, 0.03460838),
            Complex32::new(0.17070715, 0.48423922),
            Complex32::new(0.13161005, 0.7391105),
            Complex32::new(0.98467505, 0.8441286),
            Complex32::new(0.5651925, 0.9657714),
        ];

        let expected = "[[0.5448807,0.03460838],[0.17070715,0.48423922],[0.13161005,0.7391105],[0.98467505,0.8441286],[0.5651925,0.9657714]]";

        let ser = serde_json::to_string(&arr)?;
        assert_eq!(&ser, expected);

        let deser: Vec<Complex32> = serde_json::from_str(&ser)?;
        approx_eq_complex_arr(&arr, &deser);

        Ok(())
    }

    #[test]
    fn test_serialize_complex_f64() -> anyhow::Result<()> {
        let arr = vec![
            Complex64::new(0.5523248663610563, 0.0873612471755758),
            Complex64::new(0.4061179987527149, 0.4143352695310981),
            Complex64::new(0.2947733073971981, 0.3524631548956809),
            Complex64::new(0.780429096725468, 0.8380619601765059),
            Complex64::new(0.0071720034003714, 0.2236682893683702),
        ];

        let expected = "[[0.5523248663610563,0.0873612471755758],[0.4061179987527149,0.4143352695310981],[0.2947733073971981,0.3524631548956809],[0.780429096725468,0.8380619601765059],[0.0071720034003714,0.2236682893683702]]";

        let ser = serde_json::to_string(&arr)?;
        assert_eq!(&ser, expected);

        let deser: Vec<Complex64> = serde_json::from_str(&ser)?;
        approx_eq_complex_arr(&arr, &deser);

        Ok(())
    }
}
