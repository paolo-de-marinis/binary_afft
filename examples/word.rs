fn main() {
    let x: u64 = 13;
    println!("decimal: {}",x);
    println!("binary: {:064b}",x);
    println!("hex: {:016x}", x);
    assert_eq!(x, 0b1101);
    assert_eq!(x, 0xD);
}