use std::{
    process::{
        exit
    }
};

use crate::kern::*;

pub struct BayernOr<T> {
    pub code: i32,
    pub msg: String,
    pub fncs: Vec<Box<dyn FnOnce()>>,
    unwrapped: Option<T>
}

impl<T> BayernOr<T> {

    /* Initializers */

        pub fn new(option: Option<T>) -> Self {
            Self {
                code: 0,
                msg: String::new(),
                fncs: Vec::new(),
                unwrapped: option
            }
        }

        pub fn default(option: Option<T>) -> Self {
            Self::new(option)
        }

    /* Chainers */

        pub fn code(&mut self, code: i32) -> &mut Self {
             self.code = code;
        self }
    
        pub fn fnc<F>(&mut self, fnc: F) -> &mut Self 
            where 
                F: FnOnce() + 'static
        {
            self.fncs.push(Box::new(fnc));
        self }

        pub fn msg<AsStr>(&mut self, msg: AsStr) -> &mut Self
            where
                AsStr: AsRef<str>
        {
            let msg_ref = msg.as_ref();

            self.msg
                .push_str(msg_ref);

        self }

        /* Adders */

            fn msg_plus<AsStr>(&mut self, msg: AsStr, tail: &str) -> &mut Self 
                where 
                    AsStr: AsRef<str>
            {
                self.msg(msg);
                self.msg(tail);
            self }

            pub fn msgd<AsStr>(&mut self, msg: AsStr) -> &mut Self 
                where 
                    AsStr: AsRef<str>
            {
                self.msg_plus(msg, ".");
            self }

            pub fn msgln<AsStr>(&mut self, msg: AsStr) -> &mut Self 
                where 
                    AsStr: AsRef<str>
            {
                self.msg_plus(msg, "\n");
            self }

            pub fn msgdln<AsStr>(&mut self, msg: AsStr) -> &mut Self 
                where 
                    AsStr: AsRef<str>
            {
                self.msg_plus(msg, ".\n");
            self }

    /* Erasers */
    
        pub fn erase_code(&mut self) -> &mut Self {
            self.code(0);
        self }

        pub fn erase_msg(&mut self) -> &mut Self {
            self.msg = String::new();
        self }

        pub fn erase_fnc(&mut self) -> &mut Self {
            self.fncs.clear();
        self }

    /* Overwriters */

        pub fn over_fnc<F>(&mut self, fnc: F) -> &mut Self 
            where 
                F: FnOnce() + 'static
        {
            self.erase_fnc();
            self.fnc(fnc);
        self }

        pub fn over_msg<AsStr>(&mut self, msg: AsStr) -> &mut Self 
            where 
                AsStr: AsRef<str>
        {
            self.erase_msg();
            self.msg(msg);
        self }

        /* Adders */

            fn over_msg_plus<AsStr>(&mut self, msg: AsStr, tail: &str) -> &mut Self 
                where 
                    AsStr: AsRef<str>
            {
                self.over_msg(msg);
                self.msg(tail);
            self }

            pub fn over_msgd<AsStr>(&mut self, msg: AsStr) -> &mut Self 
                where 
                    AsStr: AsRef<str>
            {
                self.over_msg_plus(msg, ".");
            self }

            pub fn over_msgln<AsStr>(&mut self, msg: AsStr) -> &mut Self 
                where 
                    AsStr: AsRef<str>
            {
                self.over_msg_plus(msg, "\n");
            self }

            pub fn over_msgdln<AsStr>(&mut self, msg: AsStr) -> &mut Self 
                where 
                    AsStr: AsRef<str>
            {
                self.over_msg_plus(msg, ".\n");
            self }

    /* Exiters */
    
        pub fn bye(self) -> T {
 
            /* Return if theres something */ 
            if let Some(value) = self.unwrapped {
                return value;
            }

            let fncs = self.fncs;
            for fnc in fncs {
                let _ = fnc();
            }

            let msg = self.msg;
            eprint!("{msg}");


            let code = self.code;
            exit(code);
        }
        
        pub fn exit(mut self, code: i32) -> T {
            self.code(code);
            let val = self.bye();
        val }

}

pub trait UnwrapOrBye<T> {
    fn unwrap_or_bye(self) -> BayernOr<T>;
}

impl<T> UnwrapOrBye<T> for Option<T> {
    fn unwrap_or_bye(self) -> BayernOr<T> {
        BayernOr::new(self)
    }
}

impl<T, E> UnwrapOrBye<T> for Result<T, E> {
    fn unwrap_or_bye(self) -> BayernOr<T> {
        match self {
            Ok(val) => BayernOr::new(Some(val)),
            Err(_) => BayernOr::new(None)
        }
    }
}

impl <T, E> UnwrapOrBye<T> for Possible<T, E> {
    fn unwrap_or_bye(self) -> BayernOr<T> {
        match self {
            Possible::Okay(val) => BayernOr::new(Some(val)),
            _ => BayernOr::new(None)
        }
    }
}



