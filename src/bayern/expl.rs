use std::process::exit;

#[inline]
pub fn bye_expl<AnyFunction>(code: i32, fnc_to_exec: AnyFunction, msg: &str) -> ! 
    where AnyFunction: FnOnce()
{
    let _ = fnc_to_exec();
    eprint!("{}", msg);
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
