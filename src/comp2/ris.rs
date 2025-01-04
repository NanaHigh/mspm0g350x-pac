#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Raw interrupt status for comparator output interrupt flag. The IES bit defines the transition of the comparator output setting this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisCompifg {
    #[doc = "0: CLR"]
    RisCompifgClr = 0,
    #[doc = "1: SET"]
    RisCompifgSet = 1,
}
impl From<RisCompifg> for bool {
    #[inline(always)]
    fn from(variant: RisCompifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_COMPIFG` reader - Raw interrupt status for comparator output interrupt flag. The IES bit defines the transition of the comparator output setting this bit."]
pub type RisCompifgR = crate::BitReader<RisCompifg>;
impl RisCompifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisCompifg {
        match self.bits {
            false => RisCompifg::RisCompifgClr,
            true => RisCompifg::RisCompifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_compifg_clr(&self) -> bool {
        *self == RisCompifg::RisCompifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_compifg_set(&self) -> bool {
        *self == RisCompifg::RisCompifgSet
    }
}
#[doc = "Raw interrupt status for comparator output inverted interrupt flag. The IES bit defines the transition of the comparator output setting this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisCompinvifg {
    #[doc = "0: CLR"]
    RisCompinvifgClr = 0,
    #[doc = "1: SET"]
    RisCompinvifgSet = 1,
}
impl From<RisCompinvifg> for bool {
    #[inline(always)]
    fn from(variant: RisCompinvifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_COMPINVIFG` reader - Raw interrupt status for comparator output inverted interrupt flag. The IES bit defines the transition of the comparator output setting this bit."]
pub type RisCompinvifgR = crate::BitReader<RisCompinvifg>;
impl RisCompinvifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisCompinvifg {
        match self.bits {
            false => RisCompinvifg::RisCompinvifgClr,
            true => RisCompinvifg::RisCompinvifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_compinvifg_clr(&self) -> bool {
        *self == RisCompinvifg::RisCompinvifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_compinvifg_set(&self) -> bool {
        *self == RisCompinvifg::RisCompinvifgSet
    }
}
#[doc = "Raw interrupt status for comparator output ready interrupt flag. This bit is set when the comparator output is valid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisOutrdyifg {
    #[doc = "0: CLR"]
    RisOutrdyifgClr = 0,
    #[doc = "1: SET"]
    RisOutrdyifgSet = 1,
}
impl From<RisOutrdyifg> for bool {
    #[inline(always)]
    fn from(variant: RisOutrdyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_OUTRDYIFG` reader - Raw interrupt status for comparator output ready interrupt flag. This bit is set when the comparator output is valid."]
pub type RisOutrdyifgR = crate::BitReader<RisOutrdyifg>;
impl RisOutrdyifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisOutrdyifg {
        match self.bits {
            false => RisOutrdyifg::RisOutrdyifgClr,
            true => RisOutrdyifg::RisOutrdyifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_outrdyifg_clr(&self) -> bool {
        *self == RisOutrdyifg::RisOutrdyifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_outrdyifg_set(&self) -> bool {
        *self == RisOutrdyifg::RisOutrdyifgSet
    }
}
impl R {
    #[doc = "Bit 1 - Raw interrupt status for comparator output interrupt flag. The IES bit defines the transition of the comparator output setting this bit."]
    #[inline(always)]
    pub fn ris_compifg(&self) -> RisCompifgR {
        RisCompifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw interrupt status for comparator output inverted interrupt flag. The IES bit defines the transition of the comparator output setting this bit."]
    #[inline(always)]
    pub fn ris_compinvifg(&self) -> RisCompinvifgR {
        RisCompinvifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw interrupt status for comparator output ready interrupt flag. This bit is set when the comparator output is valid."]
    #[inline(always)]
    pub fn ris_outrdyifg(&self) -> RisOutrdyifgR {
        RisOutrdyifgR::new(((self.bits >> 3) & 1) != 0)
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
