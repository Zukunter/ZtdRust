#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use]
pub enum Possible<T, E> {
    Okay(T),
    Error(E),
    Null,
}

impl <T, E> Possible<T, E> {

    pub fn okay(self) -> Option<T> 
    // where 
    //     T: [const] Destruct,
    //     E: [const] Destruct
    {
        match self {
            Self::Okay(val) => Some(val),
            _ => None
        }
    }

    pub fn error(self) -> Option<E> 
    // where 
    //     T: [const] Destruct,
    //     E: [const] Destruct
    {
        match self {
            Self::Error(err) => Some(err),
            _ => None
        }
    }

    pub const fn as_ref(&self) -> Possible<&T,&E> {
        match *self {
            Self::Okay(ref val) => Possible::Okay(val),
            Self::Error(ref error) => Possible::Error(error),
            Self::Null => Possible::Null
        }
    }

     pub const fn as_mut(&mut self) -> Possible<&mut T, &mut E> {
        match *self {
            Self::Okay(ref mut x) => Possible::Okay(x),
            Self::Error(ref mut x) => Possible::Error(x),
            Self::Null => Possible::Null
        }
     }
}


