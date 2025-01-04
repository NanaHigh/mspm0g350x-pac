#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Masked interrupt status for COMPIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisCompifg {
    #[doc = "0: CLR"]
    MisCompifgClr = 0,
    #[doc = "1: SET"]
    MisCompifgSet = 1,
}
impl From<MisCompifg> for bool {
    #[inline(always)]
    fn from(variant: MisCompifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_COMPIFG` reader - Masked interrupt status for COMPIFG"]
pub type MisCompifgR = crate::BitReader<MisCompifg>;
impl MisCompifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisCompifg {
        match self.bits {
            false => MisCompifg::MisCompifgClr,
            true => MisCompifg::MisCompifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_compifg_clr(&self) -> bool {
        *self == MisCompifg::MisCompifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_compifg_set(&self) -> bool {
        *self == MisCompifg::MisCompifgSet
    }
}
#[doc = "Masked interrupt status for COMPINVIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisCompinvifg {
    #[doc = "0: CLR"]
    MisCompinvifgClr = 0,
    #[doc = "1: SET"]
    MisCompinvifgSet = 1,
}
impl From<MisCompinvifg> for bool {
    #[inline(always)]
    fn from(variant: MisCompinvifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_COMPINVIFG` reader - Masked interrupt status for COMPINVIFG"]
pub type MisCompinvifgR = crate::BitReader<MisCompinvifg>;
impl MisCompinvifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisCompinvifg {
        match self.bits {
            false => MisCompinvifg::MisCompinvifgClr,
            true => MisCompinvifg::MisCompinvifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_compinvifg_clr(&self) -> bool {
        *self == MisCompinvifg::MisCompinvifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_compinvifg_set(&self) -> bool {
        *self == MisCompinvifg::MisCompinvifgSet
    }
}
#[doc = "Masked interrupt status for OUTRDYIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisOutrdyifg {
    #[doc = "0: CLR"]
    MisOutrdyifgClr = 0,
    #[doc = "1: SET"]
    MisOutrdyifgSet = 1,
}
impl From<MisOutrdyifg> for bool {
    #[inline(always)]
    fn from(variant: MisOutrdyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_OUTRDYIFG` reader - Masked interrupt status for OUTRDYIFG"]
pub type MisOutrdyifgR = crate::BitReader<MisOutrdyifg>;
impl MisOutrdyifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisOutrdyifg {
        match self.bits {
            false => MisOutrdyifg::MisOutrdyifgClr,
            true => MisOutrdyifg::MisOutrdyifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_outrdyifg_clr(&self) -> bool {
        *self == MisOutrdyifg::MisOutrdyifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_outrdyifg_set(&self) -> bool {
        *self == MisOutrdyifg::MisOutrdyifgSet
    }
}
impl R {
    #[doc = "Bit 1 - Masked interrupt status for COMPIFG"]
    #[inline(always)]
    pub fn mis_compifg(&self) -> MisCompifgR {
        MisCompifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masked interrupt status for COMPINVIFG"]
    #[inline(always)]
    pub fn mis_compinvifg(&self) -> MisCompinvifgR {
        MisCompinvifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masked interrupt status for OUTRDYIFG"]
    #[inline(always)]
    pub fn mis_outrdyifg(&self) -> MisOutrdyifgR {
        MisOutrdyifgR::new(((self.bits >> 3) & 1) != 0)
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
