use std::{
    process::{
        exit
    },
    mem::{
        take
    }
};
use crate::{
    Bayern
};

pub enum BayernRef<'a> {
    Owned(Bayern),
    Borrowed(&'a mut Bayern)
}
impl<'a> BayernRef<'a> {
    pub fn get_mut(&mut self) -> &mut Bayern {
        match self {
            BayernRef::Owned(bayern) => bayern,
            BayernRef::Borrowed(bayern) => bayern,
        }
    }
}

pub struct BayernOr<'a, T> {
    pub(self) option: Option<T>,
    pub(self) bayern: BayernRef<'a>,
    pub(self) code: Option<i32>,
    pub(self) msg: Option<String>
}

impl<'a, T> BayernOr<'a, T> {
    pub fn new(option: Option<T>, bayern: BayernRef<'a>) -> Self {
        Self {
            option,
            bayern: bayern,
            code: None,
            msg: None
        }
    }

    /* Chainers */
        pub fn code(&mut self, code: i32) -> &mut Self {
            self.code = Some(code);
        self }

        pub fn fnc<F>(&mut self, fnc: F) -> &mut Self 
            where 
                F: FnOnce() + 'static
        {
            let bayern = self.bayern.get_mut();
            bayern.fnc(fnc);

        self }

        pub fn msg<AsStr>(&mut self, msg: AsStr) -> &mut Self 
        where 
            AsStr: AsRef<str>
        {
            let msg_ref = msg.as_ref();
            self.msg
                .get_or_insert(String::new())
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
            self.code = None;
        self }

        pub fn erase_msg(&mut self) -> &mut Self {
            self.msg = None;
        self }

        pub fn erase_fnc(&mut self) -> &mut Self {
            let bayern = self.bayern.get_mut();
            bayern.erase_fnc();
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
    
        pub fn bye(mut self) -> T {
            
            if let Some(value) = self.option {
                return value;
            }

            let bayern = self.bayern.get_mut();
            let fncs = take(&mut bayern.fncs);
            for fnc in fncs {
                let _ = fnc();
            }

            let msg = match &self.msg {
                Some(msg) => msg,
                None => &bayern.msg
            };
            eprint!("{msg}");


            let code = match self.code {
                Some(code) => code,
                None => bayern.code
            };
            exit(code);     
        }
        
        pub fn exit(mut self, code: i32) -> T {
            self.code(code);
            let value = self.bye();
        value }



}
