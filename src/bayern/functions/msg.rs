use std::{
    process::{
        exit
    }
};

#[inline]
pub fn bye_msg<AsStr>(code: i32, msg: AsStr) -> ! 
where 
    AsStr: AsRef<str>
{
    let msg_ref = msg.as_ref();
    eprint!("{}", msg_ref);
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
