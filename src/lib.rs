use curve25519_dalek::edwards::CompressedEdwardsY;
use curve25519_dalek::edwards::EdwardsPoint;

extern "C" {
    fn hash_to_p3(hash: *const u8, p3: *mut ge_p3);
    fn ge_p3_tobytes(bytes: *mut u8, hash8_p3: *const ge_p3);
}

#[allow(non_snake_case)]
#[repr(C)]
#[derive(Debug)]
struct ge_p3 {
    X: [i32; 10],
    Y: [i32; 10],
    Z: [i32; 10],
    T: [i32; 10],
}

pub fn hash_point_to_point(point: EdwardsPoint) -> EdwardsPoint {
    let bytes = point.compress();

    let mut compressed = [0u8; 32];
    unsafe {
        let mut p3 = ge_p3 {
            X: [0; 10],
            Y: [0; 10],
            Z: [0; 10],
            T: [0; 10],
        };

        hash_to_p3(bytes.as_bytes().as_ptr() as *const u8, &mut p3);
        ge_p3_tobytes(compressed.as_mut_ptr(), &p3);
    };

    let compressed = CompressedEdwardsY::from_slice(&compressed);
    let point = compressed
        .decompress()
        .expect("generation of valid compressed point");

    point
}

#[test]
fn test_hash_point_to_point() {
    let slice =
        hex::decode("a7fbdeeccb597c2d5fdaf2ea2e10cbfcd26b5740903e7f6d46bcbf9a90384fc6").unwrap();
    let point = CompressedEdwardsY::from_slice(&slice).decompress().unwrap();

    let actual = hash_point_to_point(point);

    let slice =
        hex::decode("f055ba2d0d9828ce2e203d9896bfda494d7830e7e3a27fa27d5eaa825a79a19c").unwrap();
    let expected = CompressedEdwardsY::from_slice(&slice).decompress().unwrap();

    assert_eq!(expected, actual);
}
