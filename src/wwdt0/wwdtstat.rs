#[doc = "Register `WWDTSTAT` reader"]
pub type R = crate::R<WwdtstatSpec>;
#[doc = "Watchdog running status flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WwdtstatRun {
    #[doc = "0: OFF"]
    WwdtstatRunOff = 0,
    #[doc = "1: ON"]
    WwdtstatRunOn = 1,
}
impl From<WwdtstatRun> for bool {
    #[inline(always)]
    fn from(variant: WwdtstatRun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WWDTSTAT_RUN` reader - Watchdog running status flag."]
pub type WwdtstatRunR = crate::BitReader<WwdtstatRun>;
impl WwdtstatRunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WwdtstatRun {
        match self.bits {
            false => WwdtstatRun::WwdtstatRunOff,
            true => WwdtstatRun::WwdtstatRunOn,
        }
    }
    #[doc = "OFF"]
    #[inline(always)]
    pub fn is_wwdtstat_run_off(&self) -> bool {
        *self == WwdtstatRun::WwdtstatRunOff
    }
    #[doc = "ON"]
    #[inline(always)]
    pub fn is_wwdtstat_run_on(&self) -> bool {
        *self == WwdtstatRun::WwdtstatRunOn
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog running status flag."]
    #[inline(always)]
    pub fn wwdtstat_run(&self) -> WwdtstatRunR {
        WwdtstatRunR::new((self.bits & 1) != 0)
    }
}
#[doc = "Window Watchdog Timer Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdtstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WwdtstatSpec;
impl crate::RegisterSpec for WwdtstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdtstat::R`](R) reader structure"]
impl crate::Readable for WwdtstatSpec {}
#[doc = "`reset()` method sets WWDTSTAT to value 0"]
impl crate::Resettable for WwdtstatSpec {
    const RESET_VALUE: u32 = 0;
}
