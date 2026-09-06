use std::{
    process::{
        exit
    }
};

#[inline]
pub fn bye_expl<F, AsStr>(code: i32, fnc: F, msg: AsStr) -> ! 
where 
    F: FnOnce(),
    AsStr: AsRef<str>
{
    let _ = fnc();

    let msg_ref = msg.as_ref();
    eprint!("{}", msg_ref);

    exit(code);
}

#[macro_export]
macro_rules! bye_expl {
    ($code:expr, $fnc_to_exec:expr, $($msg:tt)*) => {
        $crate::bayern::bye_expl($code, $fnc_to_exec, &format!($($msg)*));
    };
    ($code:expr,$fnc_to_exec:expr, $($msg:tt)*) => {
        $crate::bayern::bye_expl($code, $fnc_to_exec, &format!($($msg)*));
    };
    ($code:expr,$fnc_to_exec:expr,$($msg:tt)*) => {
        $crate::bayern::bye_expl($code, $fnc_to_exec, &format!($($msg)*));
    };
    ($code:expr, $fnc_to_exec:expr,$($msg:tt)*) => {
        $crate::bayern::bye_expl($code, $fnc_to_exec, &format!($($msg)*));
    };
    ($fnc_to_exec:expr, $($msg:tt)*) => {
        $crate::bayern::bye_expl(0, $fnc_to_exec, &format!($($msg)*));
    };
    ($fnc_to_exec:expr,$($msg:tt)*) => {
        $crate::bayern::bye_expl(0, $fnc_to_exec, &format!($($msg)*));
    }
}
