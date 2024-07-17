crate::ix!();

/**
  | Unfortunately, std::sqrt is not marked
  | as constexpr just yet in all compilers
  |
  */
generic_float_const!(InverseRootTwo, inverse_root_two, 0.70710678118654752440);

#[test]
fn test_inverse_root_two_for_num_float() {

    let a: f32 = inverse_root_two();
    let expected_a: f32 = 0.70710677;

    assert!(
        (a - expected_a).abs() < f32::EPSILON
    );

    let b: f64 = inverse_root_two();
    let expected_b: f64 = 0.7071067811865475;

    assert!(
        (b - expected_b).abs() < f64::EPSILON
    );
}
