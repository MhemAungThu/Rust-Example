fn main() {
    let x: u32 = 0x_87654321;
    let mask: u32 = 0xff;

    // Practice Problem 2.12
    let a = x & mask;
    let b = !(x | mask) ^ (x & mask);
    let c = x | mask;

    println!("(A): {:#08x}", a);
    println!("(B): {:#08x}", b);
    println!("(C): {:#08x}", c);
}