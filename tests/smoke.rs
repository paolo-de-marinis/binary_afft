use binary_toolkit::bits::bit_zero;

#[test]
fn una_stessa_costante_in_due_basi() {
    assert_eq!(13u64, 0b1101u64);
    assert_ne!(13u64, 0b1011u64);
}

#[test]
fn collaudo_bit_zero() {
    assert_eq!(bit_zero(0), 0);
    assert_eq!(bit_zero(13), 1);
    assert_eq!(bit_zero(u64::MAX), 1);
    
}