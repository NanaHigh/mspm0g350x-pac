#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisDmach0 {
    #[doc = "0: CLR"]
    RisDmach0Clr = 0,
    #[doc = "1: SET"]
    RisDmach0Set = 1,
}
impl From<RisDmach0> for bool {
    #[inline(always)]
    fn from(variant: RisDmach0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_DMACH0` reader - DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type RisDmach0R = crate::BitReader<RisDmach0>;
impl RisDmach0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisDmach0 {
        match self.bits {
            false => RisDmach0::RisDmach0Clr,
            true => RisDmach0::RisDmach0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_dmach0_clr(&self) -> bool {
        *self == RisDmach0::RisDmach0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_dmach0_set(&self) -> bool {
        *self == RisDmach0::RisDmach0Set
    }
}
#[doc = "DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisDmach1 {
    #[doc = "0: CLR"]
    RisDmach1Clr = 0,
    #[doc = "1: SET"]
    RisDmach1Set = 1,
}
impl From<RisDmach1> for bool {
    #[inline(always)]
    fn from(variant: RisDmach1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_DMACH1` reader - DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type RisDmach1R = crate::BitReader<RisDmach1>;
impl RisDmach1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisDmach1 {
        match self.bits {
            false => RisDmach1::RisDmach1Clr,
            true => RisDmach1::RisDmach1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_dmach1_clr(&self) -> bool {
        *self == RisDmach1::RisDmach1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_dmach1_set(&self) -> bool {
        *self == RisDmach1::RisDmach1Set
    }
}
#[doc = "DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisDmach2 {
    #[doc = "0: CLR"]
    RisDmach2Clr = 0,
    #[doc = "1: SET"]
    RisDmach2Set = 1,
}
impl From<RisDmach2> for bool {
    #[inline(always)]
    fn from(variant: RisDmach2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_DMACH2` reader - DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type RisDmach2R = crate::BitReader<RisDmach2>;
impl RisDmach2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisDmach2 {
        match self.bits {
            false => RisDmach2::RisDmach2Clr,
            true => RisDmach2::RisDmach2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_dmach2_clr(&self) -> bool {
        *self == RisDmach2::RisDmach2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_dmach2_set(&self) -> bool {
        *self == RisDmach2::RisDmach2Set
    }
}
#[doc = "DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisDmach3 {
    #[doc = "0: CLR"]
    RisDmach3Clr = 0,
    #[doc = "1: SET"]
    RisDmach3Set = 1,
}
impl From<RisDmach3> for bool {
    #[inline(always)]
    fn from(variant: RisDmach3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_DMACH3` reader - DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type RisDmach3R = crate::BitReader<RisDmach3>;
impl RisDmach3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisDmach3 {
        match self.bits {
            false => RisDmach3::RisDmach3Clr,
            true => RisDmach3::RisDmach3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_dmach3_clr(&self) -> bool {
        *self == RisDmach3::RisDmach3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_dmach3_set(&self) -> bool {
        *self == RisDmach3::RisDmach3Set
    }
}
#[doc = "DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisDmach4 {
    #[doc = "0: CLR"]
    RisDmach4Clr = 0,
    #[doc = "1: SET"]
    RisDmach4Set = 1,
}
impl From<RisDmach4> for bool {
    #[inline(always)]
    fn from(variant: RisDmach4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_DMACH4` reader - DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type RisDmach4R = crate::BitReader<RisDmach4>;
impl RisDmach4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisDmach4 {
        match self.bits {
            false => RisDmach4::RisDmach4Clr,
            true => RisDmach4::RisDmach4Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_dmach4_clr(&self) -> bool {
        *self == RisDmach4::RisDmach4Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_dmach4_set(&self) -> bool {
        *self == RisDmach4::RisDmach4Set
    }
}
#[doc = "DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisDmach5 {
    #[doc = "0: CLR"]
    RisDmach5Clr = 0,
    #[doc = "1: SET"]
    RisDmach5Set = 1,
}
impl From<RisDmach5> for bool {
    #[inline(always)]
    fn from(variant: RisDmach5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_DMACH5` reader - DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type RisDmach5R = crate::BitReader<RisDmach5>;
impl RisDmach5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisDmach5 {
        match self.bits {
            false => RisDmach5::RisDmach5Clr,
            true => RisDmach5::RisDmach5Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_dmach5_clr(&self) -> bool {
        *self == RisDmach5::RisDmach5Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_dmach5_set(&self) -> bool {
        *self == RisDmach5::RisDmach5Set
    }
}
#[doc = "DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisDmach6 {
    #[doc = "0: CLR"]
    RisDmach6Clr = 0,
    #[doc = "1: SET"]
    RisDmach6Set = 1,
}
impl From<RisDmach6> for bool {
    #[inline(always)]
    fn from(variant: RisDmach6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_DMACH6` reader - DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type RisDmach6R = crate::BitReader<RisDmach6>;
impl RisDmach6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisDmach6 {
        match self.bits {
            false => RisDmach6::RisDmach6Clr,
            true => RisDmach6::RisDmach6Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_dmach6_clr(&self) -> bool {
        *self == RisDmach6::RisDmach6Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_dmach6_set(&self) -> bool {
        *self == RisDmach6::RisDmach6Set
    }
}
#[doc = "Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisPreirqch0 {
    #[doc = "0: CLR"]
    RisPreirqch0Clr = 0,
    #[doc = "1: SET"]
    RisPreirqch0Set = 1,
}
impl From<RisPreirqch0> for bool {
    #[inline(always)]
    fn from(variant: RisPreirqch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_PREIRQCH0` reader - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
pub type RisPreirqch0R = crate::BitReader<RisPreirqch0>;
impl RisPreirqch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisPreirqch0 {
        match self.bits {
            false => RisPreirqch0::RisPreirqch0Clr,
            true => RisPreirqch0::RisPreirqch0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_preirqch0_clr(&self) -> bool {
        *self == RisPreirqch0::RisPreirqch0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_preirqch0_set(&self) -> bool {
        *self == RisPreirqch0::RisPreirqch0Set
    }
}
#[doc = "Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisPreirqch1 {
    #[doc = "0: CLR"]
    RisPreirqch1Clr = 0,
    #[doc = "1: SET"]
    RisPreirqch1Set = 1,
}
impl From<RisPreirqch1> for bool {
    #[inline(always)]
    fn from(variant: RisPreirqch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_PREIRQCH1` reader - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
pub type RisPreirqch1R = crate::BitReader<RisPreirqch1>;
impl RisPreirqch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisPreirqch1 {
        match self.bits {
            false => RisPreirqch1::RisPreirqch1Clr,
            true => RisPreirqch1::RisPreirqch1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_preirqch1_clr(&self) -> bool {
        *self == RisPreirqch1::RisPreirqch1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_preirqch1_set(&self) -> bool {
        *self == RisPreirqch1::RisPreirqch1Set
    }
}
#[doc = "Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisPreirqch2 {
    #[doc = "0: CLR"]
    RisPreirqch2Clr = 0,
    #[doc = "1: SET"]
    RisPreirqch2Set = 1,
}
impl From<RisPreirqch2> for bool {
    #[inline(always)]
    fn from(variant: RisPreirqch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_PREIRQCH2` reader - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
pub type RisPreirqch2R = crate::BitReader<RisPreirqch2>;
impl RisPreirqch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisPreirqch2 {
        match self.bits {
            false => RisPreirqch2::RisPreirqch2Clr,
            true => RisPreirqch2::RisPreirqch2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_preirqch2_clr(&self) -> bool {
        *self == RisPreirqch2::RisPreirqch2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_preirqch2_set(&self) -> bool {
        *self == RisPreirqch2::RisPreirqch2Set
    }
}
#[doc = "DMA address error, SRC address not reachable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisAddrerr {
    #[doc = "0: CLR"]
    RisAddrerrClr = 0,
    #[doc = "1: SET"]
    RisAddrerrSet = 1,
}
impl From<RisAddrerr> for bool {
    #[inline(always)]
    fn from(variant: RisAddrerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_ADDRERR` reader - DMA address error, SRC address not reachable."]
pub type RisAddrerrR = crate::BitReader<RisAddrerr>;
impl RisAddrerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisAddrerr {
        match self.bits {
            false => RisAddrerr::RisAddrerrClr,
            true => RisAddrerr::RisAddrerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_addrerr_clr(&self) -> bool {
        *self == RisAddrerr::RisAddrerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_addrerr_set(&self) -> bool {
        *self == RisAddrerr::RisAddrerrSet
    }
}
#[doc = "DMA data error, SRC data might be corrupted (PAR or ECC error).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisDataerr {
    #[doc = "0: CLR"]
    RisDataerrClr = 0,
    #[doc = "1: SET"]
    RisDataerrSet = 1,
}
impl From<RisDataerr> for bool {
    #[inline(always)]
    fn from(variant: RisDataerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_DATAERR` reader - DMA data error, SRC data might be corrupted (PAR or ECC error)."]
pub type RisDataerrR = crate::BitReader<RisDataerr>;
impl RisDataerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisDataerr {
        match self.bits {
            false => RisDataerr::RisDataerrClr,
            true => RisDataerr::RisDataerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_dataerr_clr(&self) -> bool {
        *self == RisDataerr::RisDataerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_dataerr_set(&self) -> bool {
        *self == RisDataerr::RisDataerrSet
    }
}
impl R {
    #[doc = "Bit 0 - DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn ris_dmach0(&self) -> RisDmach0R {
        RisDmach0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn ris_dmach1(&self) -> RisDmach1R {
        RisDmach1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn ris_dmach2(&self) -> RisDmach2R {
        RisDmach2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn ris_dmach3(&self) -> RisDmach3R {
        RisDmach3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn ris_dmach4(&self) -> RisDmach4R {
        RisDmach4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn ris_dmach5(&self) -> RisDmach5R {
        RisDmach5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn ris_dmach6(&self) -> RisDmach6R {
        RisDmach6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn ris_preirqch0(&self) -> RisPreirqch0R {
        RisPreirqch0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn ris_preirqch1(&self) -> RisPreirqch1R {
        RisPreirqch1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn ris_preirqch2(&self) -> RisPreirqch2R {
        RisPreirqch2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA address error, SRC address not reachable."]
    #[inline(always)]
    pub fn ris_addrerr(&self) -> RisAddrerrR {
        RisAddrerrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA data error, SRC data might be corrupted (PAR or ECC error)."]
    #[inline(always)]
    pub fn ris_dataerr(&self) -> RisDataerrR {
        RisDataerrR::new(((self.bits >> 25) & 1) != 0)
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
