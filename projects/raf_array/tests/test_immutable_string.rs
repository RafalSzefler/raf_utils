use raf_array::immutable_string::ImmutableString;
use rstest::rstest;

#[rstest]
#[case("")]
#[case("a")]
#[case("b")]
#[case("c")]
#[case("A")]
#[case("B")]
#[case("C")]
#[case("aBc")]
#[case("AbC")]
#[case("a  Bc    ")]
#[case("    abc")]
#[case("ąćę")]
fn test_imm_string_1(#[case] expected: &str) {
    let imm = ImmutableString::new(expected).unwrap();
    assert_eq!(imm.as_str(), expected);
    let imm2 = ImmutableString::new(expected).unwrap();
    assert_eq!(imm2.as_str(), expected);
    assert_eq!(imm.id(), imm2.id());
    assert_eq!(imm2.as_str().as_ptr(), imm.as_str().as_ptr());
}
