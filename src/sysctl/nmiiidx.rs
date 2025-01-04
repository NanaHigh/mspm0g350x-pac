#[doc = "Register `NMIIIDX` reader"]
pub type R = crate::R<NmiiidxSpec>;
#[doc = "The NMI interrupt index (NMIIIDX) register generates a value corresponding to the highest priority pending NMI source. This value may be used as an address offset for fast, deterministic handling in the NMI service routine. A read of the NMIIIDX register will clear the corresponding interrupt status in the NMIRIS register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NmiiidxStat {
    #[doc = "0: NO_INTR"]
    NmiiidxStatNoIntr = 0,
    #[doc = "1: BORLVL"]
    NmiiidxStatBorlvl = 1,
    #[doc = "2: WWDT0"]
    NmiiidxStatWwdt0 = 2,
    #[doc = "3: WWDT1"]
    NmiiidxStatWwdt1 = 3,
    #[doc = "4: LFCLKFAIL"]
    NmiiidxStatLfclkfail = 4,
    #[doc = "5: FLASHDED"]
    NmiiidxStatFlashded = 5,
    #[doc = "6: SRAMDED"]
    NmiiidxStatSramded = 6,
}
impl From<NmiiidxStat> for u8 {
    #[inline(always)]
    fn from(variant: NmiiidxStat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for NmiiidxStat {
    type Ux = u8;
}
impl crate::IsEnum for NmiiidxStat {}
#[doc = "Field `NMIIIDX_STAT` reader - The NMI interrupt index (NMIIIDX) register generates a value corresponding to the highest priority pending NMI source. This value may be used as an address offset for fast, deterministic handling in the NMI service routine. A read of the NMIIIDX register will clear the corresponding interrupt status in the NMIRIS register."]
pub type NmiiidxStatR = crate::FieldReader<NmiiidxStat>;
impl NmiiidxStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<NmiiidxStat> {
        match self.bits {
            0 => Some(NmiiidxStat::NmiiidxStatNoIntr),
            1 => Some(NmiiidxStat::NmiiidxStatBorlvl),
            2 => Some(NmiiidxStat::NmiiidxStatWwdt0),
            3 => Some(NmiiidxStat::NmiiidxStatWwdt1),
            4 => Some(NmiiidxStat::NmiiidxStatLfclkfail),
            5 => Some(NmiiidxStat::NmiiidxStatFlashded),
            6 => Some(NmiiidxStat::NmiiidxStatSramded),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_nmiiidx_stat_no_intr(&self) -> bool {
        *self == NmiiidxStat::NmiiidxStatNoIntr
    }
    #[doc = "BORLVL"]
    #[inline(always)]
    pub fn is_nmiiidx_stat_borlvl(&self) -> bool {
        *self == NmiiidxStat::NmiiidxStatBorlvl
    }
    #[doc = "WWDT0"]
    #[inline(always)]
    pub fn is_nmiiidx_stat_wwdt0(&self) -> bool {
        *self == NmiiidxStat::NmiiidxStatWwdt0
    }
    #[doc = "WWDT1"]
    #[inline(always)]
    pub fn is_nmiiidx_stat_wwdt1(&self) -> bool {
        *self == NmiiidxStat::NmiiidxStatWwdt1
    }
    #[doc = "LFCLKFAIL"]
    #[inline(always)]
    pub fn is_nmiiidx_stat_lfclkfail(&self) -> bool {
        *self == NmiiidxStat::NmiiidxStatLfclkfail
    }
    #[doc = "FLASHDED"]
    #[inline(always)]
    pub fn is_nmiiidx_stat_flashded(&self) -> bool {
        *self == NmiiidxStat::NmiiidxStatFlashded
    }
    #[doc = "SRAMDED"]
    #[inline(always)]
    pub fn is_nmiiidx_stat_sramded(&self) -> bool {
        *self == NmiiidxStat::NmiiidxStatSramded
    }
}
impl R {
    #[doc = "Bits 0:3 - The NMI interrupt index (NMIIIDX) register generates a value corresponding to the highest priority pending NMI source. This value may be used as an address offset for fast, deterministic handling in the NMI service routine. A read of the NMIIIDX register will clear the corresponding interrupt status in the NMIRIS register."]
    #[inline(always)]
    pub fn nmiiidx_stat(&self) -> NmiiidxStatR {
        NmiiidxStatR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "NMI interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`nmiiidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmiiidxSpec;
impl crate::RegisterSpec for NmiiidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmiiidx::R`](R) reader structure"]
impl crate::Readable for NmiiidxSpec {}
#[doc = "`reset()` method sets NMIIIDX to value 0"]
impl crate::Resettable for NmiiidxSpec {
    const RESET_VALUE: u32 = 0;
}
