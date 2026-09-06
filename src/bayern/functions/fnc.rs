use std::{
    process::{
        exit
    }
};

#[inline]
pub fn bye_fnc<F>(code: i32, fnc_to_exec: F) -> ! 
where 
    F: FnOnce()
{
    let _ = fnc_to_exec();
    exit(code);
}

#[macro_export]
macro_rules! bye_fnc {
    ($code:expr, $fnc_to_exec:expr) => {
        $crate::bayern::bye_fnc($code, $fnc_to_exec);
    };
    ($code:expr,$fnc_to_exec:expr) => {
        $crate::bayern::bye_fnc($code, $fnc_to_exec);
    };
    ($fnc_to_exec:expr) => {
        $crate::bayern::bye_fnc(0, $fnc_to_exec);
    };
}
