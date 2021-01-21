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
    println!("array after reverse by raw pointer: {:?}", a);
}

// practice problem: 2.10 
unsafe fn inplace_swap(x: *mut u32, y: *mut u32) {
    *x = *x ^ *y;
    *y = *x ^ *y;
    *x = *x ^ *y
}

// practice problen 2.11 from Computer System: A Programmer Perspective 
fn reverse_array(a: &mut [u32]) {
    let mut first: usize = 0;
    let mut last: usize = a.len() - 1;

    while first < last {
        unsafe {
            inplace_swap(&mut a[first] as *mut u32, &mut a[last] as *mut u32);
        }
        first += 1;
        last -= 1;
    }
}
