use std::process::exit;

#[inline]
pub fn bye_msg(code: i32, msg: &str) -> ! {
    eprint!("{}", msg);
    exit(code);
}

#[macro_export]
macro_rules! bye_msg {
    ($code:expr, $($msg:tt)*) => {
        $crate::bayern::bye_msg($code, &format!($($msg)*));
    };
    ($code:expr,$($msg:tt)*) => {
        $crate::bayern::bye_msg($code, &format!($($msg)*));
    };
    ($($msg:tt)*) => {
        $crate::bayern::bye_msg(0, &format!($($msg)*));
    }
}
