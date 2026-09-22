use crate::cfg::Cfg;

pub fn android<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "android")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "android"))] {
        Cfg::none()
    }
}

pub fn cygwin<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "cygwin")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "cygwin"))] {
        Cfg::none()
    }
}

pub fn espidf<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "espidf")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "espidf"))] {
        Cfg::none()
    }
}

pub fn helenos<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "helenos")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "helenos"))] {
        Cfg::none()
    }
}

pub fn horizon<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "horizon")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "horizon"))] {
        Cfg::none()
    }
}

pub fn hurd<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "hurd")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "hurd"))] {
        Cfg::none()
    }
}

pub fn lynxos178<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "lynxos178")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "lynxos178"))] {
        Cfg::none()
    }
}

pub fn managarm<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "managarm")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "managarm"))] {
        Cfg::none()
    }
}

pub fn motor<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "motor")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "motor"))] {
        Cfg::none()
    }
}

pub fn nto<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "nto")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "nto"))] {
        Cfg::none()
    }
}

pub fn nuttx<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "nuttx")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "nuttx"))] {
        Cfg::none()
    }
}

pub fn psx<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "psx")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "psx"))] {
        Cfg::none()
    }
}

pub fn qurt<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "qurt")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "qurt"))] {
        Cfg::none()
    }
}

pub fn rtems<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "rtems")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "rtems"))] {
        Cfg::none()
    }
}

pub fn solid_asp3<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "solid_asp3")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "solid_asp3"))] {
        Cfg::none()
    }
}

pub fn teeos<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "teeos")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "teeos"))] {
        Cfg::none()
    }
}

pub fn trusty<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "trusty")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "trusty"))] {
        Cfg::none()
    }
}

pub fn vexos<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "vexos")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "vexos"))] {
        Cfg::none()
    }
}

pub fn vita<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "vita")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "vita"))] {
        Cfg::none()
    }
}

pub fn vxworks<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "vxworks")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "vxworks"))] {
        Cfg::none()
    }
}

pub fn wasi<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "wasi")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "wasi"))] {
        Cfg::none()
    }
}

pub fn xous<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "xous")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "xous"))] {
        Cfg::none()
    }
}

pub fn zkvm<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "zkvm")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "zkvm"))] {
        Cfg::none()
    }
}

pub fn windows<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "windows")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "windows"))] {
        Cfg::none()
    }
}

pub fn linux<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "linux")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "linux"))] {
        Cfg::none()
    }
}

pub fn freebsd<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "freebsd")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "freebsd"))] {
        Cfg::none()
    }
}

pub fn fuchsia<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "fuchsia")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "fuchsia"))] {
        Cfg::none()
    }
}

pub fn solaris<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "solaris")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "solaris"))] {
        Cfg::none()
    }
}

pub fn dragonfly<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "dragonfly")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "dragonfly"))] {
        Cfg::none()
    }
}

pub fn l4re<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "l4re")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "l4re"))] {
        Cfg::none()
    }
}

pub fn psp<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "psp")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "psp"))] {
        Cfg::none()
    }
}

pub fn openbsd<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "openbsd")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "openbsd"))] {
        Cfg::none()
    }
}

pub fn redox<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "redox")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "redox"))] {
        Cfg::none()
    }
}

pub fn netbsd<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "netbsd")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "netbsd"))] {
        Cfg::none()
    }
}

pub fn hermit<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "hermit")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "hermit"))] {
        Cfg::none()
    }
}

pub fn haiku<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "haiku")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "haiku"))] {
        Cfg::none()
    }
}

pub fn emscripten<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "emscripten")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "emscripten"))] {
        Cfg::none()
    }
}

pub fn uefi<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "uefi")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "uefi"))] {
        Cfg::none()
    }
}

pub fn illumos<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "illumos")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "illumos"))] {
        Cfg::none()
    }
}

pub fn cuda<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "cuda")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "cuda"))] {
        Cfg::none()
    }
}

pub fn macos<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "macos")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "macos"))] {
        Cfg::none()
    }
}

pub fn ios<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "ios")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "ios"))] {
        Cfg::none()
    }
}

pub fn tvos<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "tvos")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "tvos"))] {
        Cfg::none()
    }
}

pub fn watchos<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "watchos")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "watchos"))] {
        Cfg::none()
    }
}

pub fn aix<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "aix")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "aix"))] {
        Cfg::none()
    }
}

pub fn amdhsa<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "amdhsa")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "amdhsa"))] {
        Cfg::none()
    }
}

pub fn visionos<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "visionos")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "visionos"))] {
        Cfg::none()
    }
}

pub fn os_none<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "none")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "none"))] {
        Cfg::none()
    }
}

pub fn os_unknown<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(target_os = "unknown")] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(not(target_os = "unknown"))] {
        Cfg::none()
    }
}
