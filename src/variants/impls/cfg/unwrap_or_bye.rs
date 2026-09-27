use crate::{
    cc::Cfg,
    bayern::{
        Bayern,
        BayernOr,
        BayernRef,

        UnwrapOrBayern,
    }, 
};


impl<'a, T> UnwrapOrBayern<'a, T> for Cfg<T> {
    fn unwrap_or_bayern(self) -> BayernOr<'a, T> {
        BayernOr::new(self.option,BayernRef::Owned(Bayern::default()))
    }
    fn unwrap_or_bayern_from(self, bayern: &'a mut Bayern) -> BayernOr<'a, T> {
        BayernOr::new(self.option, BayernRef::Borrowed(bayern))
    }
}
