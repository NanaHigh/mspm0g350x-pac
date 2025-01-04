#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Zero event generated an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisZ {
    #[doc = "0: CLR"]
    MisZClr = 0,
    #[doc = "1: SET"]
    MisZSet = 1,
}
impl From<MisZ> for bool {
    #[inline(always)]
    fn from(variant: MisZ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_Z` reader - Zero event generated an interrupt."]
pub type MisZR = crate::BitReader<MisZ>;
impl MisZR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisZ {
        match self.bits {
            false => MisZ::MisZClr,
            true => MisZ::MisZSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_z_clr(&self) -> bool {
        *self == MisZ::MisZClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_z_set(&self) -> bool {
        *self == MisZ::MisZSet
    }
}
#[doc = "Load event generated an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisL {
    #[doc = "0: CLR"]
    MisLClr = 0,
    #[doc = "1: SET"]
    MisLSet = 1,
}
impl From<MisL> for bool {
    #[inline(always)]
    fn from(variant: MisL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_L` reader - Load event generated an interrupt."]
pub type MisLR = crate::BitReader<MisL>;
impl MisLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisL {
        match self.bits {
            false => MisL::MisLClr,
            true => MisL::MisLSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_l_clr(&self) -> bool {
        *self == MisL::MisLClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_l_set(&self) -> bool {
        *self == MisL::MisLSet
    }
}
#[doc = "Capture or compare down event generated an interrupt CCP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisCcd0 {
    #[doc = "0: CLR"]
    MisCcd0Clr = 0,
    #[doc = "1: SET"]
    MisCcd0Set = 1,
}
impl From<MisCcd0> for bool {
    #[inline(always)]
    fn from(variant: MisCcd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_CCD0` reader - Capture or compare down event generated an interrupt CCP0"]
pub type MisCcd0R = crate::BitReader<MisCcd0>;
impl MisCcd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisCcd0 {
        match self.bits {
            false => MisCcd0::MisCcd0Clr,
            true => MisCcd0::MisCcd0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_ccd0_clr(&self) -> bool {
        *self == MisCcd0::MisCcd0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_ccd0_set(&self) -> bool {
        *self == MisCcd0::MisCcd0Set
    }
}
#[doc = "Capture or compare down event generated an interrupt CCP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisCcd1 {
    #[doc = "0: CLR"]
    MisCcd1Clr = 0,
    #[doc = "1: SET"]
    MisCcd1Set = 1,
}
impl From<MisCcd1> for bool {
    #[inline(always)]
    fn from(variant: MisCcd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_CCD1` reader - Capture or compare down event generated an interrupt CCP1"]
pub type MisCcd1R = crate::BitReader<MisCcd1>;
impl MisCcd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisCcd1 {
        match self.bits {
            false => MisCcd1::MisCcd1Clr,
            true => MisCcd1::MisCcd1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_ccd1_clr(&self) -> bool {
        *self == MisCcd1::MisCcd1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_ccd1_set(&self) -> bool {
        *self == MisCcd1::MisCcd1Set
    }
}
#[doc = "Capture or compare up event generated an interrupt CCP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisCcu0 {
    #[doc = "0: CLR"]
    MisCcu0Clr = 0,
    #[doc = "1: SET"]
    MisCcu0Set = 1,
}
impl From<MisCcu0> for bool {
    #[inline(always)]
    fn from(variant: MisCcu0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_CCU0` reader - Capture or compare up event generated an interrupt CCP0"]
pub type MisCcu0R = crate::BitReader<MisCcu0>;
impl MisCcu0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisCcu0 {
        match self.bits {
            false => MisCcu0::MisCcu0Clr,
            true => MisCcu0::MisCcu0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_ccu0_clr(&self) -> bool {
        *self == MisCcu0::MisCcu0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_ccu0_set(&self) -> bool {
        *self == MisCcu0::MisCcu0Set
    }
}
#[doc = "Capture or compare up event generated an interrupt CCP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisCcu1 {
    #[doc = "0: CLR"]
    MisCcu1Clr = 0,
    #[doc = "1: SET"]
    MisCcu1Set = 1,
}
impl From<MisCcu1> for bool {
    #[inline(always)]
    fn from(variant: MisCcu1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_CCU1` reader - Capture or compare up event generated an interrupt CCP1"]
pub type MisCcu1R = crate::BitReader<MisCcu1>;
impl MisCcu1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisCcu1 {
        match self.bits {
            false => MisCcu1::MisCcu1Clr,
            true => MisCcu1::MisCcu1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_ccu1_clr(&self) -> bool {
        *self == MisCcu1::MisCcu1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_ccu1_set(&self) -> bool {
        *self == MisCcu1::MisCcu1Set
    }
}
#[doc = "Trigger overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisTov {
    #[doc = "0: CLR"]
    MisTovClr = 0,
    #[doc = "1: SET"]
    MisTovSet = 1,
}
impl From<MisTov> for bool {
    #[inline(always)]
    fn from(variant: MisTov) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_TOV` reader - Trigger overflow"]
pub type MisTovR = crate::BitReader<MisTov>;
impl MisTovR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisTov {
        match self.bits {
            false => MisTov::MisTovClr,
            true => MisTov::MisTovSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_tov_clr(&self) -> bool {
        *self == MisTov::MisTovClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_tov_set(&self) -> bool {
        *self == MisTov::MisTovSet
    }
}
#[doc = "Direction Change\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisDc {
    #[doc = "0: CLR"]
    MisDcClr = 0,
    #[doc = "1: SET"]
    MisDcSet = 1,
}
impl From<MisDc> for bool {
    #[inline(always)]
    fn from(variant: MisDc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_DC` reader - Direction Change"]
pub type MisDcR = crate::BitReader<MisDc>;
impl MisDcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisDc {
        match self.bits {
            false => MisDc::MisDcClr,
            true => MisDc::MisDcSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_dc_clr(&self) -> bool {
        *self == MisDc::MisDcClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_dc_set(&self) -> bool {
        *self == MisDc::MisDcSet
    }
}
#[doc = "QEIERR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisQeierr {
    #[doc = "0: CLR"]
    MisQeierrClr = 0,
    #[doc = "1: SET"]
    MisQeierrSet = 1,
}
impl From<MisQeierr> for bool {
    #[inline(always)]
    fn from(variant: MisQeierr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_QEIERR` reader - QEIERR"]
pub type MisQeierrR = crate::BitReader<MisQeierr>;
impl MisQeierrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisQeierr {
        match self.bits {
            false => MisQeierr::MisQeierrClr,
            true => MisQeierr::MisQeierrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_qeierr_clr(&self) -> bool {
        *self == MisQeierr::MisQeierrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_qeierr_set(&self) -> bool {
        *self == MisQeierr::MisQeierrSet
    }
}
impl R {
    #[doc = "Bit 0 - Zero event generated an interrupt."]
    #[inline(always)]
    pub fn mis_z(&self) -> MisZR {
        MisZR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load event generated an interrupt."]
    #[inline(always)]
    pub fn mis_l(&self) -> MisLR {
        MisLR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture or compare down event generated an interrupt CCP0"]
    #[inline(always)]
    pub fn mis_ccd0(&self) -> MisCcd0R {
        MisCcd0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture or compare down event generated an interrupt CCP1"]
    #[inline(always)]
    pub fn mis_ccd1(&self) -> MisCcd1R {
        MisCcd1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture or compare up event generated an interrupt CCP0"]
    #[inline(always)]
    pub fn mis_ccu0(&self) -> MisCcu0R {
        MisCcu0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture or compare up event generated an interrupt CCP1"]
    #[inline(always)]
    pub fn mis_ccu1(&self) -> MisCcu1R {
        MisCcu1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 25 - Trigger overflow"]
    #[inline(always)]
    pub fn mis_tov(&self) -> MisTovR {
        MisTovR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Direction Change"]
    #[inline(always)]
    pub fn mis_dc(&self) -> MisDcR {
        MisDcR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - QEIERR"]
    #[inline(always)]
    pub fn mis_qeierr(&self) -> MisQeierrR {
        MisQeierrR::new(((self.bits >> 28) & 1) != 0)
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
