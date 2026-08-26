use std::{
    process::{
        exit
    }
};

use crate::kern::*;


pub struct BayernOrUnwrapped<T> {
    pub code: i32,
    pub msg: Option<String>,
    pub fnc: Option<Box<dyn FnOnce()>>,
    unwrapped: Option<T>
}

impl<T> BayernOrUnwrapped<T> {

    // Starter
    pub fn new(option: Option<T>) -> Self {
        Self {
            code: 0,
            msg: None,
            fnc: None,
            unwrapped: option
        }
    }

    // Modifiers
    pub fn code(mut self, code: i32) -> Self {
        self.code = code;
    return self ; }
    
    pub fn fnc<F>(mut self, fnc: F) -> Self 
        where 
            F: FnOnce() + 'static 
    {
        self.fnc = Some(Box::new(fnc));
    return self ; }
    
    pub fn msg<AsStr>(mut self, msg: AsStr) -> Self 
        where 
            AsStr: AsRef<str>
    {
        let new_msg_string = msg.as_ref().to_owned();

        self.msg = Some(new_msg_string);

    return self ; }

    // Additional Messages
    pub fn msg_d<AsStr>(self, msg: AsStr) -> Self 
        where 
            AsStr: AsRef<str>
    {
        let mut new_msg_with_ln = msg.as_ref().to_owned();
        new_msg_with_ln.push('.');
        
        let mod_bayern = self.msg(new_msg_with_ln); 
    return mod_bayern ; }
    
    pub fn msg_ln<AsStr>(self, msg: AsStr) -> Self 
        where 
            AsStr: AsRef<str>
    {
        let mut new_msg_with_ln = msg.as_ref().to_owned();
        new_msg_with_ln.push('\n');
        
        let mod_bayern = self.msg(new_msg_with_ln); 
    return mod_bayern ; }

    pub fn msg_dln<AsStr>(self, msg: AsStr) -> Self 
        where 
            AsStr: AsRef<str>
    {
        let mut new_msg_with_ln = msg.as_ref().to_owned();
        new_msg_with_ln.push_str(".\n");
        
        let mod_bayern = self.msg(new_msg_with_ln); 
    return mod_bayern ; }

    // Ender 
    pub fn bye(self) -> T {

        if let Some(value) = self.unwrapped {
            return value;
        }

        let code = self.code;

        if let Some(fnc) = self.fnc {
            let _ = fnc();
        }

        if let Some(msg) = self.msg {
            eprint!("{}", msg);
        }

        exit(code);
    }

    pub fn exit(self, code: i32) -> T {
        let new_bayern = self.code(code);
        new_bayern.bye()
    }
}

pub trait UnwrapOrBye<T> {
    fn unwrap_or_bye(self) -> BayernOrUnwrapped<T>;
}

impl<T> UnwrapOrBye<T> for Option<T> {
    fn unwrap_or_bye(self) -> BayernOrUnwrapped<T> {
        BayernOrUnwrapped::new(self)
    }
}

impl<T, E> UnwrapOrBye<T> for Result<T, E> {
    fn unwrap_or_bye(self) -> BayernOrUnwrapped<T> {
        match self {
            Ok(val) => BayernOrUnwrapped::new(Some(val)),
            Err(_) => BayernOrUnwrapped::new(None)
        }
    }
}

impl <T, E> UnwrapOrBye<T> for Possible<T, E> {
    fn unwrap_or_bye(self) -> BayernOrUnwrapped<T> {
        match self {
            Possible::Okay(val) => BayernOrUnwrapped::new(Some(val)),
            _ => BayernOrUnwrapped::new(None)
        }
    }
}

