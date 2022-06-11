/// .
///
/// # Safety
///
/// .
pub unsafe fn inplace_swap(x: *mut i32, y: *mut i32) {
    let tmp = *x ^ *y;
    *x ^= tmp;
    *y = *x ^ tmp;
}
