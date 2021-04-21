// practice problem: 2.10
pub unsafe fn inplace_swap(x: *mut u32, y: *mut u32) {
    *x = *x ^ *y;
    *y = *x ^ *y;
    *x = *x ^ *y
}

// practice problen 2.11 from Computer System: A Programmer Perspective
pub fn reverse_array(a: &mut [u32]) {
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