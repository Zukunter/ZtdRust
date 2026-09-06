mod path; 

pub use std::format as f;

pub use path::{
    PathToString
};
pub use crate::{
    bye_expl,
    bye_fnc,
    bye_msg
};
pub use crate::{
    bayern::{
        BayernOr,
        UnwrapOrBayern,
        UnwrapOrByeOption,
        UnwrapOrByePossible,
        UnwrapOrByeResult
    }
};
pub use crate::variants::{
    Possible::{
        self, *
    },
    OptionToPossible,
    ResultToPossible
};
