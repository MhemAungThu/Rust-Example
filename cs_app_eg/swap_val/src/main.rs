use swap_val::*;

fn main() {
    // stored the values in variables
    let mut x: u32 = 53;
    let mut y: u32 = 564;

    // create raw pointers to the variables
    let xp = &mut x as *mut u32;
    let yp = &mut y as *mut u32;

    // swap the pointer
    unsafe {
        inplace_swap(xp, yp);
    }

    // since pointers are own the value, the values get swap
    println!("x: {:?}, y: {:?}", x, y);
    unsafe {
        println!("{:?}", xp.as_ref());
    }

    // let mut a: [u32; 6] = [1, 2, 3, 4, 5, 6];
    // unsafe {
    //     inplace_swap(&mut a[0] as *mut u32, &mut a[5] as *mut u32);
    // }
    // println!("{:?}", a);

    let mut a: [u32; 7] = [1, 2, 3, 4, 5, 6, 7];
    reverse_array(&mut a);
    println!("array after reverse by raw pointer: {:?}\n", a);

    // practice problem 2.12
    let x_1: u32 = 0x87654321;
    let _res = x_1 & 0x000000ff;
    println!("{:08x}", _res);


    let mut t = 5;
    let demo = &mut t as *mut i32;
    println!("{:?}", !t);
    println!("{:?}", !demo);
}





