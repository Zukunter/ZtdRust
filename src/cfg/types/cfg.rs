use crate::bayern::Bayern;

pub struct Cfg<T> {
    pub(self) internal: Option<T>
}

impl<T> Cfg<T> {
    pub(crate) fn some(initial: T) -> Self {
        Self {
            internal: Some(initial)
        }
    }
    pub(crate) fn none() -> Self {
        Self {
            internal: None
        }
    }
    
    pub fn otherwisse<Fnc>(self, _fnc: Fnc) -> T 
    where 
        Fnc: FnOnce() -> T
    {
        match self.internal {
            Some(int) => int,
            None => _fnc()
        }
    }
    pub fn otherwisse_bye<Fnc>(self, _fnc: Fnc) -> T
    where 
        Fnc: FnOnce(&mut Bayern) -> T
    {
        let mut bayern = Bayern::new();
        match self.internal {
            Some(int) => int,
            None => _fnc(&mut bayern)
        }
    }

    /* Os */
        pub fn android<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "android")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "android"))] {
                return self;
            }   
        }
        pub fn cygwin<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "cygwin")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "cygwin"))] {
                return self;
            }   
        }
        pub fn espidf<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "espidf")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "espidf"))] {
                return self;
            }   
        }
        pub fn helenos<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "helenos")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "helenos"))] {
                return self;
            }   
        }
        pub fn horizon<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "horizon")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "horizon"))] {
                return self;
            }   
        }
        pub fn hurd<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "hurd")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "hurd"))] {
                return self;
            }   
        }
        pub fn lynxos178<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "lynxos178")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "lynxos178"))] {
                return self;
            }   
        }
        pub fn managarm<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "managarm")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "managarm"))] {
                return self;
            }   
        }
        pub fn motor<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "motor")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "motor"))] {
                return self;
            }   
        }
        pub fn nto<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "nto")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "nto"))] {
                return self;
            }   
        }
        pub fn nuttx<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "nuttx")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "nuttx"))] {
                return self;
            }   
        }
        pub fn psx<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "psx")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "psx"))] {
                return self;
            }   
        }
        pub fn qurt<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "qurt")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "qurt"))] {
                return self;
            }   
        }
        pub fn rtems<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "rtems")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "rtems"))] {
                return self;
            }   
        }
        pub fn solid_asp3<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "solid_asp3")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "solid_asp3"))] {
                return self;
            }   
        }
        pub fn teeos<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "teeos")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "teeos"))] {
                return self;
            }   
        }
        pub fn trusty<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "trusty")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "trusty"))] {
                return self;
            }   
        }
        pub fn vexos<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "vexos")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "vexos"))] {
                return self;
            }   
        }
        pub fn vita<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "vita")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "vita"))] {
                return self;
            }   
        }
        pub fn vxworks<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "vxworks")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "vxworks"))] {
                return self;
            }   
        }
        pub fn wasi<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "wasi")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "wasi"))] {
                return self;
            }   
        }
        pub fn xous<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "xous")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "xous"))] {
                return self;
            }   
        }
        pub fn zkvm<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "zkvm")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "zkvm"))] {
                return self;
            }   
        }
        pub fn windows<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "windows")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "windows"))] {
                return self;
            }
        }
        pub fn linux<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "linux")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "linux"))] {
                return self;
            }   
        }
        pub fn freebsd<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "freebsd")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "freebsd"))] {
                return self;
            }   
        }
        pub fn fuchsia<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "fuchsia")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "fuchsia"))] {
                return self;
            }   
        }
        pub fn solaris<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "solaris")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "solaris"))] {
                return self;
            }   
        }
        pub fn dragonfly<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "dragonfly")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "dragonfly"))] {
                return self;
            }   
        }
        pub fn l4re<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "l4re")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "l4re"))] {
                return self;
            }   
        }
        pub fn psp<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "psp")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "psp"))] {
                return self;
            }   
        }
        pub fn openbsd<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "openbsd")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "openbsd"))] {
                return self;
            }   
        }
        pub fn redox<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "redox")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "redox"))] {
                return self;
            }   
        }
        pub fn netbsd<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "netbsd")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "netbsd"))] {
                return self;
            }   
        }
        pub fn hermit<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "hermit")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "hermit"))] {
                return self;
            }   
        }
        pub fn haiku<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "haiku")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "haiku"))] {
                return self;
            }   
        }
        pub fn emscripten<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "emscripten")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "emscripten"))] {
                return self;
            }   
        }
        pub fn uefi<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "uefi")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "uefi"))] {
                return self;
            }   
        }
        pub fn illumos<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "illumos")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "illumos"))] {
                return self;
            }   
        }
        pub fn cuda<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "cuda")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "cuda"))] {
                return self;
            }   
        }
        pub fn macos<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "macos")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "macos"))] {
                return self;
            }   
        }
        pub fn ios<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "ios")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "ios"))] {
                return self;
            }   
        }
        pub fn tvos<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "tvos")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "tvos"))] {
                return self;
            }   
        }
        pub fn watchos<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "watchos")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "watchos"))] {
                return self;
            }   
        }
        pub fn aix<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "aix")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "aix"))] {
                return self;
            }   
        }
        pub fn amdhsa<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "amdhsa")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "amdhsa"))] {
                return self;
            }   
        }
        pub fn visionos<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "visionos")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "visionos"))] {
                return self;
            }   
        }
        pub fn os_none<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "none")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "none"))] {
                return self;
            }   
        }
        pub fn os_unknown<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_os = "unknown")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_os = "unknown"))] {
                return self;
            }   
        }
    /* Not Os */
        pub fn not_android<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "android"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "android")] {
                return self;
            }
        }
        pub fn not_cygwin<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "cygwin"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "cygwin")] {
                return self;
            }
        }
        pub fn not_espidf<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "espidf"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "espidf")] {
                return self;
            }
        }
        pub fn not_helenos<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "helenos"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "helenos")] {
                return self;
            }
        }
        pub fn not_horizon<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "horizon"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "horizon")] {
                return self;
            }
        }
        pub fn not_hurd<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "hurd"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "hurd")] {
                return self;
            }
        }
        pub fn not_lynxos178<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "lynxos178"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "lynxos178")] {
                return self;
            }
        }
        pub fn not_managarm<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "managarm"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "managarm")] {
                return self;
            }
        }
        pub fn not_motor<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "motor"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "motor")] {
                return self;
            }
        }
        pub fn not_nto<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "nto"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "nto")] {
                return self;
            }
        }
        pub fn not_nuttx<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "nuttx"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "nuttx")] {
                return self;
            }
        }
        pub fn not_psx<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "psx"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "psx")] {
                return self;
            }
        }
        pub fn not_qurt<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "qurt"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "qurt")] {
                return self;
            }
        }
        pub fn not_rtems<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "rtems"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "rtems")] {
                return self;
            }
        }
        pub fn not_solid_asp3<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "solid_asp3"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "solid_asp3")] {
                return self;
            }
        }
        pub fn not_teeos<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "teeos"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "teeos")] {
                return self;
            }
        }
        pub fn not_trusty<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "trusty"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "trusty")] {
                return self;
            }
        }
        pub fn not_vexos<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "vexos"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "vexos")] {
                return self;
            }
        }
        pub fn not_vita<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "vita"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "vita")] {
                return self;
            }
        }
        pub fn not_vxworks<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "vxworks"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "vxworks")] {
                return self;
            }
        }
        pub fn not_wasi<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "wasi"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "wasi")] {
                return self;
            }
        }
        pub fn not_xous<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "xous"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "xous")] {
                return self;
            }
        }
        pub fn not_zkvm<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "zkvm"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "zkvm")] {
                return self;
            }
        }
        pub fn not_windows<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "windows"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "windows")] {
                return self;
            }
        }
        pub fn not_linux<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "linux"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "linux")] {
                return self;
            }
        }
        pub fn not_freebsd<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "freebsd"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "freebsd")] {
                return self;
            }
        }
        pub fn not_fuchsia<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "fuchsia"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "fuchsia")] {
                return self;
            }
        }
        pub fn not_solaris<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "solaris"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "solaris")] {
                return self;
            }
        }
        pub fn not_dragonfly<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "dragonfly"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "dragonfly")] {
                return self;
            }
        }
        pub fn not_l4re<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "l4re"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "l4re")] {
                return self;
            }
        }
        pub fn not_psp<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "psp"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "psp")] {
                return self;
            }
        }
        pub fn not_openbsd<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "openbsd"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "openbsd")] {
                return self;
            }
        }
        pub fn not_redox<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "redox"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "redox")] {
                return self;
            }
        }
        pub fn not_netbsd<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "netbsd"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "netbsd")] {
                return self;
            }
        }
        pub fn not_hermit<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "hermit"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "hermit")] {
                return self;
            }
        }
        pub fn not_haiku<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "haiku"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "haiku")] {
                return self;
            }
        }
        pub fn not_emscripten<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "emscripten"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "emscripten")] {
                return self;
            }
        }
        pub fn not_uefi<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "uefi"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "uefi")] {
                return self;
            }
        }
        pub fn not_illumos<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "illumos"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "illumos")] {
                return self;
            }
        }
        pub fn not_cuda<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "cuda"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "cuda")] {
                return self;
            }
        }
        pub fn not_macos<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "macos"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "macos")] {
                return self;
            }
        }
        pub fn not_ios<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "ios"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "ios")] {
                return self;
            }
        }
        pub fn not_tvos<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "tvos"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "tvos")] {
                return self;
            }
        }
        pub fn not_watchos<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "watchos"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "watchos")] {
                return self;
            }
        }
        pub fn not_aix<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "aix"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "aix")] {
                return self;
            }
        }
        pub fn not_amdhsa<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "amdhsa"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "amdhsa")] {
                return self;
            }
        }
        pub fn not_visionos<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "visionos"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "visionos")] {
                return self;
            }
        }
        pub fn not_os_none<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "none"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "none")] {
                return self;
            }
        }
        pub fn not_os_unknown<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_os = "unknown"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_os = "unknown")] {
                return self;
            }
        }
    /* Arch */
        pub fn x86<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_arch = "x86")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_arch = "x86"))] {
                return self;
            }   
        }
        pub fn x86_64<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_arch = "x86_64")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_arch = "x86_64"))] {
                return self;
            }   
        }
        pub fn arm<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_arch = "arm")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_arch = "arm"))] {
                return self;
            }   
        }
        pub fn aarch64<Fnc>(self, _fnc: Fnc) -> Self
        where 
            Fnc: FnOnce() -> T 
        {
            #[cfg(target_arch = "aarch64")] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(not(target_arch = "aarch64"))] {
                return self;
            }   
        }
    /* Not Arch */
        pub fn not_x86<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_arch = "x86"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_arch = "x86")] {
                return self;
            }
        }
        pub fn not_x86_64<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_arch = "x86_64"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_arch = "x86_64")] {
                return self;
            }
        }
        pub fn not_arm<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_arch = "arm"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_arch = "arm")] {
                return self;
            }
        }
        pub fn not_aarch64<Fnc>(self, _fnc: Fnc) -> Self
        where
            Fnc: FnOnce() -> T,
        {
            #[cfg(not(target_arch = "aarch64"))] {
                let t = _fnc();
                return Self::some(t);
            }

            #[cfg(target_arch = "aarch64")] {
                return self;
            }
        }
}
