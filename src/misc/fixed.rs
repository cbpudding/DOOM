use crate::types::Fixed;
use std::os::raw::c_longlong;

pub const FRAC_BITS: usize = 16;
pub const FRAC_UNIT: usize = 1 << FRAC_BITS;

#[export_name = "FixedMul"]
pub extern "C" fn fixed_mul(a: Fixed, b: Fixed) -> Fixed {
    (((a as c_longlong) * (b as c_longlong)) >> FRAC_BITS) as Fixed
}

#[export_name = "FixedDiv"]
pub extern "C" fn fixed_div(a: Fixed, b: Fixed) -> Fixed {
    if (a.abs() >> 14) >= b.abs() {
        if (a ^ b) < 0 {
            Fixed::MIN
        } else {
            Fixed::MAX
        }
    } else {
        fixed_div2(a, b)
    }
}

#[export_name = "FixedDiv2"]
pub extern "C" fn fixed_div2(a: Fixed, b: Fixed) -> Fixed {
    let c = (a as f64 / b as f64) * FRAC_UNIT as f64;
    if c >= 2_147_483_648.0 || c < -2_147_483_648.0 {
        panic!("FixedDiv: divide by zero");
    }
    c as Fixed
}