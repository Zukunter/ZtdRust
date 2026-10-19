pub use std::format as f;

pub use crate::vfs::PathToString;

pub use crate::{
    // Raw
    printd,
    prints,
    printds,
    printdln,

    // Jetz
    printdj,
    printsj,
    printdsj,
    printdlnj,

    // Print Raw Jetz
    printj,
    printlnj
};

pub use crate::{
    bye_expl,
    bye_fnc,
    bye_msg
};
pub use crate::Debugger;
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
