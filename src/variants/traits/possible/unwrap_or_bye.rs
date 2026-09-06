use crate::{
    bayern::{
        Bayern,
        BayernOr,
        BayernRef,

        UnwrapOrBayern,
        UnwrapOrByePossible
    }, 
    kern::Possible::{self, *}
};

impl<'a, T, E> UnwrapOrBayern<'a, T> for Possible<T, E> {
    fn unwrap_or_bayern(self) -> BayernOr<'a, T> {
        BayernOr::new(
            self.okay(),    
            BayernRef::Owned(Bayern::default())            
        )
    }

    fn unwrap_or_bayern_from(self, bayern: &'a mut Bayern) -> BayernOr<'a, T> {
        BayernOr::new(
            self.okay(),
            BayernRef::Borrowed(bayern)
        )
    }
}

impl<T, E> UnwrapOrByePossible<T, E> for Possible<T, E> {
    fn unwrap_or_bye<FncErr, FncNull>(self, fnc_err: FncErr, fnc_null: FncNull) -> T
       where 
           FncErr: FnOnce(&mut Bayern, E) -> T,
           FncNull: FnOnce(&mut Bayern) -> T
       {
           let mut bayern = Bayern::default();

           match self {
               Okay(val) => val,
               Error(e) => fnc_err(&mut bayern, e),
               Null => fnc_null(&mut bayern)
           }
       }   
}
