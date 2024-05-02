#![no_main]

use libfuzzer_sys::fuzz_target;
mod calcu;
fuzz_target!(|data: (i32,i32)| {
    let (a,b) = data;
    let _ = calcu::calculate(a, b);
});


