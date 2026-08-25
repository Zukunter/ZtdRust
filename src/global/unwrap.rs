use std::{
    fmt::{
        Display
    },
    process::{
        exit
    }
};

use crate::{
    global::{
        Possible
    }
};

use crate::bayerns::msg::bye_msg;
use crate::bayerns::fnc::bye_fnc;
use crate::bayerns::expl::bye_expl;

pub trait UnwrapOrBye<T, E: Display>: Sized {
    fn unwrap_or_bye_msg(self, code: i32, msg: &str) -> T;

    fn unwrap_or_bye_fnc<AnyFunction>(self, code: i32, fnc_to_exec: AnyFunction) -> T
    where AnyFunction: FnOnce();

    fn unwrap_or_bye_expl<AnyFunction>(self, code: i32, fnc_to_exec: AnyFunction, msg: &str) -> T
    where AnyFunction: FnOnce();

    fn unwrap_or_bye(self, code: i32) -> T;
}

impl<T> UnwrapOrBye<T, u8> for Option<T> {
    fn unwrap_or_bye_msg(self, code: i32, msg: &str) -> T {
        match self {
            Some(val) => val,
            None => bye_msg(code, &format!("{}", msg))
        } 
    }
    fn unwrap_or_bye_fnc<AnyFunction>(self, code: i32, fnc_to_exec: AnyFunction) -> T
        where AnyFunction: FnOnce()
    {
        match self {
            Some(val) => val,
            None => bye_fnc(code, fnc_to_exec)
        }
    }
    fn unwrap_or_bye_expl<AnyFunction>(self, code: i32, fnc_to_exec: AnyFunction, msg: &str) -> T
        where AnyFunction: FnOnce()
    {
        match self {
            Some(val) => val,
            None => bye_expl(code, fnc_to_exec, &format!("{}", msg))
        }
    }
    fn unwrap_or_bye(self, code: i32) -> T {
        match self {
            Some(val) => val,
            None => exit(code)
        }
    }
}

impl<T, E: Display> UnwrapOrBye<T, E> for Result<T, E> {
    fn unwrap_or_bye_msg(self, code: i32, msg: &str) -> T {
        match self {
            Ok(val) => val,
            Err(_) => bye_msg(code, &format!("{}", msg))
        } 
    }
    fn unwrap_or_bye_fnc<AnyFunction>(self, code: i32, fnc_to_exec: AnyFunction) -> T
        where AnyFunction: FnOnce()
    {
        match self {
            Ok(val) => val,
            Err(_) => bye_fnc(code, fnc_to_exec)
        }
    }
    fn unwrap_or_bye_expl<AnyFunction>(self, code: i32, fnc_to_exec: AnyFunction, msg: &str) -> T
        where AnyFunction: FnOnce()
    {
        match self {
            Ok(val) => val,
            Err(_) => bye_expl(code, fnc_to_exec, &format!("{}", msg))
        }
    }
    fn unwrap_or_bye(self, code: i32) -> T {
        match self {
            Ok(val) => val,
            Err(_) => exit(code)
        }
    }
}

impl<T, E: Display> UnwrapOrBye<T, E> for Possible<T, E> {
    fn unwrap_or_bye_msg(self, code: i32, msg: &str) -> T {
        match self {
            Okey(val) => val,
            Error(_) => bye_msg(code, &format!("{}", msg)),
            Null => bye_msg(code, &format!("{}", msg))
        } 
    }
    fn unwrap_or_bye_fnc<AnyFunction>(self, code: i32, fnc_to_exec: AnyFunction) -> T
        where AnyFunction: FnOnce()
    {
        match self {
            Okey(val) => val,
            Error(_) => bye_fnc(code, fnc_to_exec),
            Null => bye_fnc(code, fnc_to_exec)
        }
    }
    fn unwrap_or_bye_expl<AnyFunction>(self, code: i32, fnc_to_exec: AnyFunction, msg: &str) -> T
        where AnyFunction: FnOnce()
    {
        match self {
            Okey(val) => val,
            Error(_) => bye_expl(code, fnc_to_exec, &format!("{}", msg)),
            Null => bye_expl(code, fnc_to_exec, &format!("{}", msg)),
        }
    }
    fn unwrap_or_bye(self, code: i32) -> T {
        match self {
            Okey(val) => val,
            _ => exit(code)
        }
    }
}
