#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Interval Timer Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisInttim {
    #[doc = "0: CLR"]
    MisInttimClr = 0,
    #[doc = "1: SET"]
    MisInttimSet = 1,
}
impl From<MisInttim> for bool {
    #[inline(always)]
    fn from(variant: MisInttim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_INTTIM` reader - Interval Timer Interrupt."]
pub type MisInttimR = crate::BitReader<MisInttim>;
impl MisInttimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisInttim {
        match self.bits {
            false => MisInttim::MisInttimClr,
            true => MisInttim::MisInttimSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_inttim_clr(&self) -> bool {
        *self == MisInttim::MisInttimClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_inttim_set(&self) -> bool {
        *self == MisInttim::MisInttimSet
    }
}
impl R {
    #[doc = "Bit 0 - Interval Timer Interrupt."]
    #[inline(always)]
    pub fn mis_inttim(&self) -> MisInttimR {
        MisInttimR::new((self.bits & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {
    const RESET_VALUE: u32 = 0;
}
