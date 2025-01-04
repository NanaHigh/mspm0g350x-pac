#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Zero event generated an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisZ {
    #[doc = "0: CLR"]
    RisZClr = 0,
    #[doc = "1: SET"]
    RisZSet = 1,
}
impl From<RisZ> for bool {
    #[inline(always)]
    fn from(variant: RisZ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_Z` reader - Zero event generated an interrupt."]
pub type RisZR = crate::BitReader<RisZ>;
impl RisZR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisZ {
        match self.bits {
            false => RisZ::RisZClr,
            true => RisZ::RisZSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_z_clr(&self) -> bool {
        *self == RisZ::RisZClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_z_set(&self) -> bool {
        *self == RisZ::RisZSet
    }
}
#[doc = "Load event generated an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisL {
    #[doc = "0: CLR"]
    RisLClr = 0,
    #[doc = "1: SET"]
    RisLSet = 1,
}
impl From<RisL> for bool {
    #[inline(always)]
    fn from(variant: RisL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_L` reader - Load event generated an interrupt."]
pub type RisLR = crate::BitReader<RisL>;
impl RisLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisL {
        match self.bits {
            false => RisL::RisLClr,
            true => RisL::RisLSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_l_clr(&self) -> bool {
        *self == RisL::RisLClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_l_set(&self) -> bool {
        *self == RisL::RisLSet
    }
}
#[doc = "Capture or compare down event generated an interrupt CCP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisCcd0 {
    #[doc = "0: CLR"]
    RisCcd0Clr = 0,
    #[doc = "1: SET"]
    RisCcd0Set = 1,
}
impl From<RisCcd0> for bool {
    #[inline(always)]
    fn from(variant: RisCcd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_CCD0` reader - Capture or compare down event generated an interrupt CCP0"]
pub type RisCcd0R = crate::BitReader<RisCcd0>;
impl RisCcd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisCcd0 {
        match self.bits {
            false => RisCcd0::RisCcd0Clr,
            true => RisCcd0::RisCcd0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_ccd0_clr(&self) -> bool {
        *self == RisCcd0::RisCcd0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_ccd0_set(&self) -> bool {
        *self == RisCcd0::RisCcd0Set
    }
}
#[doc = "Capture or compare down event generated an interrupt CCP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisCcd1 {
    #[doc = "0: CLR"]
    RisCcd1Clr = 0,
    #[doc = "1: SET"]
    RisCcd1Set = 1,
}
impl From<RisCcd1> for bool {
    #[inline(always)]
    fn from(variant: RisCcd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_CCD1` reader - Capture or compare down event generated an interrupt CCP1"]
pub type RisCcd1R = crate::BitReader<RisCcd1>;
impl RisCcd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisCcd1 {
        match self.bits {
            false => RisCcd1::RisCcd1Clr,
            true => RisCcd1::RisCcd1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_ccd1_clr(&self) -> bool {
        *self == RisCcd1::RisCcd1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_ccd1_set(&self) -> bool {
        *self == RisCcd1::RisCcd1Set
    }
}
#[doc = "Capture or compare up event generated an interrupt CCP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisCcu0 {
    #[doc = "0: CLR"]
    RisCcu0Clr = 0,
    #[doc = "1: SET"]
    RisCcu0Set = 1,
}
impl From<RisCcu0> for bool {
    #[inline(always)]
    fn from(variant: RisCcu0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_CCU0` reader - Capture or compare up event generated an interrupt CCP0"]
pub type RisCcu0R = crate::BitReader<RisCcu0>;
impl RisCcu0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisCcu0 {
        match self.bits {
            false => RisCcu0::RisCcu0Clr,
            true => RisCcu0::RisCcu0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_ccu0_clr(&self) -> bool {
        *self == RisCcu0::RisCcu0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_ccu0_set(&self) -> bool {
        *self == RisCcu0::RisCcu0Set
    }
}
#[doc = "Capture or compare up event generated an interrupt CCP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisCcu1 {
    #[doc = "0: CLR"]
    RisCcu1Clr = 0,
    #[doc = "1: SET"]
    RisCcu1Set = 1,
}
impl From<RisCcu1> for bool {
    #[inline(always)]
    fn from(variant: RisCcu1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_CCU1` reader - Capture or compare up event generated an interrupt CCP1"]
pub type RisCcu1R = crate::BitReader<RisCcu1>;
impl RisCcu1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisCcu1 {
        match self.bits {
            false => RisCcu1::RisCcu1Clr,
            true => RisCcu1::RisCcu1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_ccu1_clr(&self) -> bool {
        *self == RisCcu1::RisCcu1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_ccu1_set(&self) -> bool {
        *self == RisCcu1::RisCcu1Set
    }
}
#[doc = "Trigger overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisTov {
    #[doc = "0: CLR"]
    RisTovClr = 0,
    #[doc = "1: SET"]
    RisTovSet = 1,
}
impl From<RisTov> for bool {
    #[inline(always)]
    fn from(variant: RisTov) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_TOV` reader - Trigger overflow"]
pub type RisTovR = crate::BitReader<RisTov>;
impl RisTovR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisTov {
        match self.bits {
            false => RisTov::RisTovClr,
            true => RisTov::RisTovSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_tov_clr(&self) -> bool {
        *self == RisTov::RisTovClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_tov_set(&self) -> bool {
        *self == RisTov::RisTovSet
    }
}
#[doc = "Direction Change\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisDc {
    #[doc = "0: CLR"]
    RisDcClr = 0,
    #[doc = "1: SET"]
    RisDcSet = 1,
}
impl From<RisDc> for bool {
    #[inline(always)]
    fn from(variant: RisDc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_DC` reader - Direction Change"]
pub type RisDcR = crate::BitReader<RisDc>;
impl RisDcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisDc {
        match self.bits {
            false => RisDc::RisDcClr,
            true => RisDc::RisDcSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_dc_clr(&self) -> bool {
        *self == RisDc::RisDcClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_dc_set(&self) -> bool {
        *self == RisDc::RisDcSet
    }
}
#[doc = "QEIERR, set on an incorrect state transition on the encoder interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisQeierr {
    #[doc = "0: CLR"]
    RisQeierrClr = 0,
    #[doc = "1: SET"]
    RisQeierrSet = 1,
}
impl From<RisQeierr> for bool {
    #[inline(always)]
    fn from(variant: RisQeierr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_QEIERR` reader - QEIERR, set on an incorrect state transition on the encoder interface."]
pub type RisQeierrR = crate::BitReader<RisQeierr>;
impl RisQeierrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisQeierr {
        match self.bits {
            false => RisQeierr::RisQeierrClr,
            true => RisQeierr::RisQeierrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_qeierr_clr(&self) -> bool {
        *self == RisQeierr::RisQeierrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_qeierr_set(&self) -> bool {
        *self == RisQeierr::RisQeierrSet
    }
}
impl R {
    #[doc = "Bit 0 - Zero event generated an interrupt."]
    #[inline(always)]
    pub fn ris_z(&self) -> RisZR {
        RisZR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load event generated an interrupt."]
    #[inline(always)]
    pub fn ris_l(&self) -> RisLR {
        RisLR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture or compare down event generated an interrupt CCP0"]
    #[inline(always)]
    pub fn ris_ccd0(&self) -> RisCcd0R {
        RisCcd0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture or compare down event generated an interrupt CCP1"]
    #[inline(always)]
    pub fn ris_ccd1(&self) -> RisCcd1R {
        RisCcd1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture or compare up event generated an interrupt CCP0"]
    #[inline(always)]
    pub fn ris_ccu0(&self) -> RisCcu0R {
        RisCcu0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture or compare up event generated an interrupt CCP1"]
    #[inline(always)]
    pub fn ris_ccu1(&self) -> RisCcu1R {
        RisCcu1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 25 - Trigger overflow"]
    #[inline(always)]
    pub fn ris_tov(&self) -> RisTovR {
        RisTovR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Direction Change"]
    #[inline(always)]
    pub fn ris_dc(&self) -> RisDcR {
        RisDcR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - QEIERR, set on an incorrect state transition on the encoder interface."]
    #[inline(always)]
    pub fn ris_qeierr(&self) -> RisQeierrR {
        RisQeierrR::new(((self.bits >> 28) & 1) != 0)
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
