mod possible; 

pub use possible::{
    Possible::{
        self,
        Okay,
        Error,
        Null
    }
};

pub use crate::bayern::{
    UnwrapOrBye,
};
pub use crate::{
    bye_expl,
    bye_fnc,
    bye_msg
};


