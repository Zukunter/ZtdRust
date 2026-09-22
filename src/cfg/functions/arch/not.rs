use crate::cfg::Cfg;

pub fn not_x86<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_arch = "x86"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_arch = "x86")] {
        Cfg::none()
    }
}
pub fn not_x86_64<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_arch = "x86_64"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_arch = "x86_64")] {
        Cfg::none()
    }
}
pub fn not_arm<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_arch = "arm"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_arch = "arm")] {
        Cfg::none()
    }
}
pub fn not_aarch64<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_arch = "aarch64"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_arch = "aarch64")] {
        Cfg::none()
    }
}
