use crate::{
    bayern::{
        Bayern,
        BayernOr,
        BayernRef,

        UnwrapOrBayern,
        UnwrapOrByeResult
    }
};

impl<'a, T, E> UnwrapOrBayern<'a, T> for Result<T, E> {
    fn unwrap_or_bayern(self) -> BayernOr<'a, T> {
        BayernOr::new(
            self.ok(),
            BayernRef::Owned(Bayern::default())
        )
    }

    fn unwrap_or_bayern_from(self, bayern: &'a mut Bayern) -> BayernOr<'a, T> {
        BayernOr::new(
            self.ok(),
            BayernRef::Borrowed(bayern)
        )
    }
}

impl<T, E> UnwrapOrByeResult<T, E> for Result<T, E> {
    fn unwrap_or_bye<FncErr>(self, fnc_err: FncErr) -> T
    where 
        FncErr: FnOnce(&mut Bayern, E) -> T
    {
        let mut bayern = Bayern::default();

        match self {
            Ok(val) => val,
            Err(e) => fnc_err(&mut bayern, e)
        }
    }
}
