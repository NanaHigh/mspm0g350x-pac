#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Interval Timer Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisInttim {
    #[doc = "0: CLR"]
    RisInttimClr = 0,
    #[doc = "1: SET"]
    RisInttimSet = 1,
}
impl From<RisInttim> for bool {
    #[inline(always)]
    fn from(variant: RisInttim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_INTTIM` reader - Interval Timer Interrupt."]
pub type RisInttimR = crate::BitReader<RisInttim>;
impl RisInttimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisInttim {
        match self.bits {
            false => RisInttim::RisInttimClr,
            true => RisInttim::RisInttimSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_inttim_clr(&self) -> bool {
        *self == RisInttim::RisInttimClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_inttim_set(&self) -> bool {
        *self == RisInttim::RisInttimSet
    }
}
impl R {
    #[doc = "Bit 0 - Interval Timer Interrupt."]
    #[inline(always)]
    pub fn ris_inttim(&self) -> RisInttimR {
        RisInttimR::new((self.bits & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0;
}
