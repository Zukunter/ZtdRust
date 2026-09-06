use crate::bayern::Bayern;

pub trait UnwrapOrByeOption<T> {
    fn unwrap_or_bye<FncNull>(self, fnc_null: FncNull) -> T
    where 
        FncNull: FnOnce(&mut Bayern) -> T;
}

pub trait UnwrapOrByeResult<T, E> {
    fn unwrap_or_bye<FncErr>(self, fnc_err: FncErr) -> T
    where 
        FncErr: FnOnce(&mut Bayern, E) -> T;
}

pub trait UnwrapOrByePossible<T, E> { 
    fn unwrap_or_bye<FncErr, FncNull>(self, fnc_err: FncErr, fnc_null: FncNull) -> T
    where 
        FncErr: FnOnce(&mut Bayern, E) -> T,
        FncNull: FnOnce(&mut Bayern) -> T;
}
