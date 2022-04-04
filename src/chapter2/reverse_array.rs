use super::inplace_swap;

pub fn reverse_array(a: *mut i32, cnt: usize) {
    let mut first = 0;
    let mut last = cnt - 1;
    while first <= last {
        unsafe {
            inplace_swap(a.add(first), a.add(last));
        };
        first += 1;
        last -= 1;
    }
}