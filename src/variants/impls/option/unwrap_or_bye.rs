use crate::{
    bayern::{
        Bayern,
        BayernOr,
        BayernRef,

        UnwrapOrBayern,
        UnwrapOrByeOption
    }, 
};

impl<'a, T> UnwrapOrBayern<'a, T> for Option<T> {
   fn unwrap_or_bayern(self) -> BayernOr<'a, T> {
        BayernOr::new(
            self,
            BayernRef::Owned(Bayern::default())
        )
    }

   fn unwrap_or_bayern_from(self, bayern: &'a mut Bayern) -> BayernOr<'a, T> {
       BayernOr::new(
            self,
            BayernRef::Borrowed(bayern)
        )
   }
}

impl<T> UnwrapOrByeOption<T> for Option<T> {
    fn unwrap_or_bye<FncNull>(self, fnc_null: FncNull) -> T
    where 
        FncNull: FnOnce(&mut Bayern) -> T
    {
        let mut bayern = Bayern::default();

        match self {
            Some(val) => val,
            None => fnc_null(&mut bayern)
        }
    }
}
