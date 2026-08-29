#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[must_use]
pub enum Possible<T, E> {
    Okay(T),
    Error(E),
    Null
}

impl<T, E> Possible<T, E> {
    pub fn okay(self) -> Option<T>
    {
        match self {
            Self::Okay(val) => Some(val),
            _ => None,
        }
    }

    pub fn error(self) -> Option<E>
    {
        match self {
            Self::Error(err) => Some(err),
            _ => None,
        }
    }

    pub const fn as_ref(&self) -> Possible<&T, &E> {
        match *self {
            Self::Okay(ref val) => Possible::Okay(val),
            Self::Error(ref error) => Possible::Error(error),
            Self::Null => Possible::Null,
        }
    }

    pub const fn as_mut(&mut self) -> Possible<&mut T, &mut E> {
        match *self {
            Self::Okay(ref mut x) => Possible::Okay(x),
            Self::Error(ref mut x) => Possible::Error(x),
            Self::Null => Possible::Null,
        }
    }

    pub const fn is_okay(&self) -> bool {
        match self {
            Self::Okay(_) => true,
            _ => false
        }
    }

    pub const fn is_error(&self) -> bool {
        match self {
            Self::Error(_) => true,
            _ => false
        }
    }

    pub const fn is_null(&self) -> bool {
        match self {
            Self::Null => true,
            _ => false
        }
    }

    pub fn map<U, F>(self, f: F) -> Possible<U, E>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Self::Okay(value) => Possible::Okay(f(value)),
            Self::Error(error) => Possible::Error(error),
            Self::Null => Possible::Null,
        }
    }

}
