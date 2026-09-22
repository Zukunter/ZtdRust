use crate::cfg::Cfg;

pub fn not_android<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "android"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "android")] {
        Cfg::none()
    }
}
pub fn not_cygwin<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "cygwin"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "cygwin")] {
        Cfg::none()
    }
}
pub fn not_espidf<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "espidf"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "espidf")] {
        Cfg::none()
    }
}
pub fn not_helenos<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "helenos"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "helenos")] {
        Cfg::none()
    }
}
pub fn not_horizon<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "horizon"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "horizon")] {
        Cfg::none()
    }
}
pub fn not_hurd<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "hurd"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "hurd")] {
        Cfg::none()
    }
}
pub fn not_lynxos178<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "lynxos178"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "lynxos178")] {
        Cfg::none()
    }
}
pub fn not_managarm<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "managarm"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "managarm")] {
        Cfg::none()
    }
}
pub fn not_motor<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "motor"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "motor")] {
        Cfg::none()
    }
}
pub fn not_nto<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "nto"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "nto")] {
        Cfg::none()
    }
}
pub fn not_nuttx<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "nuttx"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "nuttx")] {
        Cfg::none()
    }
}
pub fn not_psx<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "psx"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "psx")] {
        Cfg::none()
    }
}
pub fn not_qurt<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "qurt"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "qurt")] {
        Cfg::none()
    }
}
pub fn not_rtems<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "rtems"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "rtems")] {
        Cfg::none()
    }
}
pub fn not_solid_asp3<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "solid_asp3"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "solid_asp3")] {
        Cfg::none()
    }
}
pub fn not_teeos<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "teeos"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "teeos")] {
        Cfg::none()
    }
}
pub fn not_trusty<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "trusty"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "trusty")] {
        Cfg::none()
    }
}
pub fn not_vexos<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "vexos"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "vexos")] {
        Cfg::none()
    }
}
pub fn not_vita<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "vita"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "vita")] {
        Cfg::none()
    }
}
pub fn not_vxworks<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "vxworks"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "vxworks")] {
        Cfg::none()
    }
}
pub fn not_wasi<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "wasi"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "wasi")] {
        Cfg::none()
    }
}
pub fn not_xous<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "xous"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "xous")] {
        Cfg::none()
    }
}
pub fn not_zkvm<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "zkvm"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "zkvm")] {
        Cfg::none()
    }
}
pub fn not_windows<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "windows"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "windows")] {
        Cfg::none()
    }
}
pub fn not_linux<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "linux"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "linux")] {
        Cfg::none()
    }
}
pub fn not_freebsd<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "freebsd"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "freebsd")] {
        Cfg::none()
    }
}
pub fn not_fuchsia<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "fuchsia"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "fuchsia")] {
        Cfg::none()
    }
}
pub fn not_solaris<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "solaris"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "solaris")] {
        Cfg::none()
    }
}
pub fn not_dragonfly<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "dragonfly"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "dragonfly")] {
        Cfg::none()
    }
}
pub fn not_l4re<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "l4re"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "l4re")] {
        Cfg::none()
    }
}
pub fn not_psp<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "psp"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "psp")] {
        Cfg::none()
    }
}
pub fn not_openbsd<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "openbsd"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "openbsd")] {
        Cfg::none()
    }
}
pub fn not_redox<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "redox"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "redox")] {
        Cfg::none()
    }
}
pub fn not_netbsd<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "netbsd"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "netbsd")] {
        Cfg::none()
    }
}
pub fn not_hermit<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "hermit"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "hermit")] {
        Cfg::none()
    }
}
pub fn not_haiku<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "haiku"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "haiku")] {
        Cfg::none()
    }
}
pub fn not_emscripten<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "emscripten"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "emscripten")] {
        Cfg::none()
    }
}
pub fn not_uefi<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "uefi"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "uefi")] {
        Cfg::none()
    }
}
pub fn not_illumos<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "illumos"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "illumos")] {
        Cfg::none()
    }
}
pub fn not_cuda<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "cuda"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "cuda")] {
        Cfg::none()
    }
}
pub fn not_macos<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "macos"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "macos")] {
        Cfg::none()
    }
}
pub fn not_ios<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "ios"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "ios")] {
        Cfg::none()
    }
}
pub fn not_tvos<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "tvos"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "tvos")] {
        Cfg::none()
    }
}
pub fn not_watchos<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "watchos"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "watchos")] {
        Cfg::none()
    }
}
pub fn not_aix<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "aix"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "aix")] {
        Cfg::none()
    }
}
pub fn not_amdhsa<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "amdhsa"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "amdhsa")] {
        Cfg::none()
    }
}
pub fn not_visionos<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "visionos"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "visionos")] {
        Cfg::none()
    }
}
pub fn not_os_none<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "none"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "none")] {
        Cfg::none()
    }
}
pub fn not_os_unknown<Fnc, T>(_fnc: Fnc) -> Cfg<T>
where
    Fnc: FnOnce() -> T
{
    #[cfg(not(target_os = "unknown"))] {
        let t = _fnc();
        Cfg::some(t)
    }

    #[cfg(target_os = "unknown")] {
        Cfg::none()
    }
}
