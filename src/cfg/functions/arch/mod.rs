pub fn x86<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_arch = "x86")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_arch = "x86"))] {
        Cfg::none()
    }
}

pub fn x86_64<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_arch = "x86_64")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_arch = "x86_64"))] {
        Cfg::none()
    }
}

pub fn arm<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_arch = "arm")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_arch = "arm"))] {
        Cfg::none()
    }
}

pub fn aarch64<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_arch = "aarch64")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_arch = "aarch64"))] {
        Cfg::none()
    }
}
