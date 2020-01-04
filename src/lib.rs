use libc::{c_int, c_double};
use std::slice::*;
use trta::TrTA;

#[no_mangle]
pub extern "C"
fn sma_rs(sma: *mut c_double, prev: c_int, c: *const c_double, total: c_int, period: c_int) {
    let latest:  &[f64] = unsafe {from_raw_parts(sma, prev as usize)};
    let close:   &[f64] = unsafe {from_raw_parts(c, total as usize)};
    let dst: &mut [f64] = unsafe {from_raw_parts_mut(sma, total as usize)};

    let src = close.sma_re(period as usize, latest);

    if prev == 0 {
        dst.copy_from_slice(&src);
    } else {
        dst[(prev - 1) as usize ..].copy_from_slice(&src[(prev - 1) as usize ..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sma_rs() {
        let mut sma = [3.0, 1.0, 2.0, 7.0, 9.0];
        let p_sma = sma.as_mut_ptr() as *mut c_double;

        let prev: c_int = 3;  // incorrect first and second values of sma are reused

        let c = [1.0, 4.0, 4.0, 4.0, 7.0];
        let p_c = c.as_ptr() as *const c_double;

        let total: c_int = 5;

        let period: c_int = 3;

        sma_rs(p_sma, prev, p_c, total, period);

        assert_eq!(sma, [3.0, 1.0, 3.0, 4.0, 5.0]);  // correct sma is [1.0, 2.0, 3.0, 4.0, 5.0]
    }
}
