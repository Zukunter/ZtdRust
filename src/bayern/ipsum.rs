use std::{
    process::{
        exit
    }
};

pub struct Bayern {
    pub code: i32,
    pub msg: Option<String>,
    pub fnc: Option<Box<dyn FnOnce()>>
}

impl Bayern {

    // Starter
    pub fn new() -> Self {
        Self {
            code: 0,
            msg: None,
            fnc: None
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
    pub fn bye(self) -> ! {
        let code = self.code;
       
        if let Some(msg) = self.msg {
            eprint!("{}", msg);
        }

        if let Some(fnc) = self.fnc {
            let _ = fnc();
        }

        exit(code);
    }

}
