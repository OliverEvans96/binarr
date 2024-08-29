use num::complex::Complex;
use num::traits::{FromBytes, ToBytes};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExtractError {
    #[error("Number of bytes must be divisible by {div}. Got {actual}")]
    InvalidLength { div: usize, actual: usize },
}

pub fn decode_real_vector<R, const N: usize>(bytes: &[u8]) -> Result<Vec<R>, ExtractError>
where
    R: FromBytes<Bytes = [u8; N]>,
{
    let nbytes = bytes.len();
    if nbytes % N != 0 {
        return Err(ExtractError::InvalidLength {
            div: N,
            actual: nbytes,
        });
    }

    let arr_len = bytes.len() / N;
    let mut arr = Vec::with_capacity(arr_len);

    for i in 0..arr_len {
        let mut chunk = [0u8; N];
        let slice = &bytes[N * i..N * (i + 1)];
        chunk.copy_from_slice(slice);
        let val = R::from_le_bytes(&chunk);
        arr.push(val);
    }

    Ok(arr)
}

pub fn decode_complex_vector<R, const N: usize>(
    bytes: &[u8],
) -> Result<Vec<Complex<R>>, ExtractError>
where
    R: FromBytes<Bytes = [u8; N]>,
{
    let nbytes = bytes.len();
    if nbytes % (2 * N) != 0 {
        return Err(ExtractError::InvalidLength {
            div: 2 * N,
            actual: nbytes,
        });
    }

    let arr_len = bytes.len() / (2 * N);
    let mut arr = Vec::with_capacity(arr_len);

    for i in 0..arr_len {
        let mut chunk = [0u8; N];

        let slice = &bytes[2 * N * i..2 * N * (i + 1)];
        chunk.copy_from_slice(&slice[0..N]);
        let real = R::from_le_bytes(&chunk);

        chunk.copy_from_slice(&slice[N..2 * N]);
        let imag = R::from_le_bytes(&chunk);

        let val = Complex::<R>::new(real, imag);
        arr.push(val);
    }

    Ok(arr)
}

pub fn encode_real_vector<R, const N: usize>(arr: &[R]) -> Vec<u8>
where
    R: ToBytes<Bytes = [u8; N]>,
{
    let len = arr.len() * N;
    let mut bytes = Vec::with_capacity(len);

    for el in arr {
        let el_bytes = el.to_le_bytes();
        bytes.extend_from_slice(&el_bytes);
    }

    bytes
}

pub fn encode_complex_vector<R, const N: usize>(arr: &[Complex<R>]) -> Vec<u8>
where
    R: ToBytes<Bytes = [u8; N]>,
{
    let len = arr.len() * 2 * N;
    let mut bytes = Vec::with_capacity(len);

    for el in arr {
        let real_bytes = el.re.to_le_bytes();
        bytes.extend_from_slice(&real_bytes);
        let imag_bytes = el.im.to_le_bytes();
        bytes.extend_from_slice(&imag_bytes);
    }

    bytes
}

#[cfg(test)]
mod tests {
    use num::complex::{Complex32, Complex64};

    use super::{
        decode_complex_vector, decode_real_vector, encode_complex_vector, encode_real_vector,
    };
    use crate::utils::test_utils::{approx_eq_complex_arr, approx_eq_real_arr};

    #[test]
    fn test_encode_complex_f32() {
        let hexstr =
            "4d7d0b3f85c10d3ddbcd2e3e34eef73ec9c4063e59363d3faa137c3fd018583f75b0103fcb3c773f";

        let bytes: Vec<u8> = hex::decode(hexstr).unwrap();

        let expected = vec![
            Complex32::new(0.5448807, 0.03460838),
            Complex32::new(0.17070715, 0.48423922),
            Complex32::new(0.13161005, 0.7391105),
            Complex32::new(0.98467505, 0.8441286),
            Complex32::new(0.5651925, 0.9657714),
        ];

        let decoded = decode_complex_vector(&bytes).unwrap();
        let encoded = encode_complex_vector(&decoded);

        approx_eq_complex_arr(&decoded, &expected);
        assert_eq!(&encoded, &bytes);
    }

    #[test]
    fn test_encode_real_f32() {
        let hexstr = "145ade3d26f2d33e2e792c3e0589123f3e6e713f";
        let expected: Vec<f32> = vec![0.10857025, 0.41395682, 0.16843101, 0.57240325, 0.9430884];

        let bytes: Vec<u8> = hex::decode(hexstr).unwrap();
        let decoded = decode_real_vector(&bytes).unwrap();
        let encoded = encode_real_vector(&decoded);

        approx_eq_real_arr(&decoded, &expected);
        assert_eq!(encoded, bytes);
    }

    #[test]
    fn test_encode_complex_f64() {
        let hexstr =
            "d039b932a5ace13f108f8e834e5db63faa6ebd58d6fdd93f4ec90d147884da3f584dc0dc90ddd23f889ad49ec18ed63fed0be97046f9e83f175adf5067d1ea3f8084d20064607d3f24a5fe9929a1cc3f";

        let bytes: Vec<u8> = hex::decode(hexstr).unwrap();

        let expected = vec![
            Complex64::new(0.5523248663610563, 0.0873612471755758),
            Complex64::new(0.4061179987527149, 0.4143352695310981),
            Complex64::new(0.2947733073971981, 0.3524631548956809),
            Complex64::new(0.780429096725468, 0.8380619601765059),
            Complex64::new(0.0071720034003714, 0.2236682893683702),
        ];

        let decoded = decode_complex_vector(&bytes).unwrap();
        let encoded = encode_complex_vector(&decoded);

        approx_eq_complex_arr(&decoded, &expected);
        assert_eq!(&encoded, &bytes);
    }

    #[test]
    fn test_encode_real_f64() {
        let hexstr =
            "f230270bf04edd3f33d707e8c0bbea3feee00aa9a996e43f56b72894eb24e03fb458db566736de3f";
        let expected: Vec<f64> = vec![
            0.4579429730336094,
            0.8354191333625977,
            0.643391447220919,
            0.5045068639499088,
            0.4720705364428668,
        ];

        let bytes: Vec<u8> = hex::decode(hexstr).unwrap();
        let decoded = decode_real_vector(&bytes).unwrap();
        let encoded = encode_real_vector(&decoded);

        approx_eq_real_arr(&decoded, &expected);
        assert_eq!(encoded, bytes);
    }
}
