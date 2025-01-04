#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisDmach0 {
    #[doc = "0: CLR"]
    MisDmach0Clr = 0,
    #[doc = "1: SET"]
    MisDmach0Set = 1,
}
impl From<MisDmach0> for bool {
    #[inline(always)]
    fn from(variant: MisDmach0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_DMACH0` reader - DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type MisDmach0R = crate::BitReader<MisDmach0>;
impl MisDmach0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisDmach0 {
        match self.bits {
            false => MisDmach0::MisDmach0Clr,
            true => MisDmach0::MisDmach0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_dmach0_clr(&self) -> bool {
        *self == MisDmach0::MisDmach0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_dmach0_set(&self) -> bool {
        *self == MisDmach0::MisDmach0Set
    }
}
#[doc = "DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisDmach1 {
    #[doc = "0: CLR"]
    MisDmach1Clr = 0,
    #[doc = "1: SET"]
    MisDmach1Set = 1,
}
impl From<MisDmach1> for bool {
    #[inline(always)]
    fn from(variant: MisDmach1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_DMACH1` reader - DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type MisDmach1R = crate::BitReader<MisDmach1>;
impl MisDmach1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisDmach1 {
        match self.bits {
            false => MisDmach1::MisDmach1Clr,
            true => MisDmach1::MisDmach1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_dmach1_clr(&self) -> bool {
        *self == MisDmach1::MisDmach1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_dmach1_set(&self) -> bool {
        *self == MisDmach1::MisDmach1Set
    }
}
#[doc = "DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisDmach2 {
    #[doc = "0: CLR"]
    MisDmach2Clr = 0,
    #[doc = "1: SET"]
    MisDmach2Set = 1,
}
impl From<MisDmach2> for bool {
    #[inline(always)]
    fn from(variant: MisDmach2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_DMACH2` reader - DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type MisDmach2R = crate::BitReader<MisDmach2>;
impl MisDmach2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisDmach2 {
        match self.bits {
            false => MisDmach2::MisDmach2Clr,
            true => MisDmach2::MisDmach2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_dmach2_clr(&self) -> bool {
        *self == MisDmach2::MisDmach2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_dmach2_set(&self) -> bool {
        *self == MisDmach2::MisDmach2Set
    }
}
#[doc = "DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisDmach3 {
    #[doc = "0: CLR"]
    MisDmach3Clr = 0,
    #[doc = "1: SET"]
    MisDmach3Set = 1,
}
impl From<MisDmach3> for bool {
    #[inline(always)]
    fn from(variant: MisDmach3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_DMACH3` reader - DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type MisDmach3R = crate::BitReader<MisDmach3>;
impl MisDmach3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisDmach3 {
        match self.bits {
            false => MisDmach3::MisDmach3Clr,
            true => MisDmach3::MisDmach3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_dmach3_clr(&self) -> bool {
        *self == MisDmach3::MisDmach3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_dmach3_set(&self) -> bool {
        *self == MisDmach3::MisDmach3Set
    }
}
#[doc = "DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisDmach4 {
    #[doc = "0: CLR"]
    MisDmach4Clr = 0,
    #[doc = "1: SET"]
    MisDmach4Set = 1,
}
impl From<MisDmach4> for bool {
    #[inline(always)]
    fn from(variant: MisDmach4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_DMACH4` reader - DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type MisDmach4R = crate::BitReader<MisDmach4>;
impl MisDmach4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisDmach4 {
        match self.bits {
            false => MisDmach4::MisDmach4Clr,
            true => MisDmach4::MisDmach4Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_dmach4_clr(&self) -> bool {
        *self == MisDmach4::MisDmach4Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_dmach4_set(&self) -> bool {
        *self == MisDmach4::MisDmach4Set
    }
}
#[doc = "DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisDmach5 {
    #[doc = "0: CLR"]
    MisDmach5Clr = 0,
    #[doc = "1: SET"]
    MisDmach5Set = 1,
}
impl From<MisDmach5> for bool {
    #[inline(always)]
    fn from(variant: MisDmach5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_DMACH5` reader - DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type MisDmach5R = crate::BitReader<MisDmach5>;
impl MisDmach5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisDmach5 {
        match self.bits {
            false => MisDmach5::MisDmach5Clr,
            true => MisDmach5::MisDmach5Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_dmach5_clr(&self) -> bool {
        *self == MisDmach5::MisDmach5Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_dmach5_set(&self) -> bool {
        *self == MisDmach5::MisDmach5Set
    }
}
#[doc = "DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisDmach6 {
    #[doc = "0: CLR"]
    MisDmach6Clr = 0,
    #[doc = "1: SET"]
    MisDmach6Set = 1,
}
impl From<MisDmach6> for bool {
    #[inline(always)]
    fn from(variant: MisDmach6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_DMACH6` reader - DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type MisDmach6R = crate::BitReader<MisDmach6>;
impl MisDmach6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisDmach6 {
        match self.bits {
            false => MisDmach6::MisDmach6Clr,
            true => MisDmach6::MisDmach6Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_dmach6_clr(&self) -> bool {
        *self == MisDmach6::MisDmach6Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_dmach6_set(&self) -> bool {
        *self == MisDmach6::MisDmach6Set
    }
}
#[doc = "Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisPreirqch0 {
    #[doc = "0: CLR"]
    MisPreirqch0Clr = 0,
    #[doc = "1: SET"]
    MisPreirqch0Set = 1,
}
impl From<MisPreirqch0> for bool {
    #[inline(always)]
    fn from(variant: MisPreirqch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_PREIRQCH0` reader - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
pub type MisPreirqch0R = crate::BitReader<MisPreirqch0>;
impl MisPreirqch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisPreirqch0 {
        match self.bits {
            false => MisPreirqch0::MisPreirqch0Clr,
            true => MisPreirqch0::MisPreirqch0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_preirqch0_clr(&self) -> bool {
        *self == MisPreirqch0::MisPreirqch0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_preirqch0_set(&self) -> bool {
        *self == MisPreirqch0::MisPreirqch0Set
    }
}
#[doc = "Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisPreirqch1 {
    #[doc = "0: CLR"]
    MisPreirqch1Clr = 0,
    #[doc = "1: SET"]
    MisPreirqch1Set = 1,
}
impl From<MisPreirqch1> for bool {
    #[inline(always)]
    fn from(variant: MisPreirqch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_PREIRQCH1` reader - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
pub type MisPreirqch1R = crate::BitReader<MisPreirqch1>;
impl MisPreirqch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisPreirqch1 {
        match self.bits {
            false => MisPreirqch1::MisPreirqch1Clr,
            true => MisPreirqch1::MisPreirqch1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_preirqch1_clr(&self) -> bool {
        *self == MisPreirqch1::MisPreirqch1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_preirqch1_set(&self) -> bool {
        *self == MisPreirqch1::MisPreirqch1Set
    }
}
#[doc = "Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisPreirqch2 {
    #[doc = "0: CLR"]
    MisPreirqch2Clr = 0,
    #[doc = "1: SET"]
    MisPreirqch2Set = 1,
}
impl From<MisPreirqch2> for bool {
    #[inline(always)]
    fn from(variant: MisPreirqch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_PREIRQCH2` reader - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
pub type MisPreirqch2R = crate::BitReader<MisPreirqch2>;
impl MisPreirqch2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisPreirqch2 {
        match self.bits {
            false => MisPreirqch2::MisPreirqch2Clr,
            true => MisPreirqch2::MisPreirqch2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_preirqch2_clr(&self) -> bool {
        *self == MisPreirqch2::MisPreirqch2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_preirqch2_set(&self) -> bool {
        *self == MisPreirqch2::MisPreirqch2Set
    }
}
#[doc = "DMA address error, SRC address not reachable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisAddrerr {
    #[doc = "0: CLR"]
    MisAddrerrClr = 0,
    #[doc = "1: SET"]
    MisAddrerrSet = 1,
}
impl From<MisAddrerr> for bool {
    #[inline(always)]
    fn from(variant: MisAddrerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_ADDRERR` reader - DMA address error, SRC address not reachable."]
pub type MisAddrerrR = crate::BitReader<MisAddrerr>;
impl MisAddrerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisAddrerr {
        match self.bits {
            false => MisAddrerr::MisAddrerrClr,
            true => MisAddrerr::MisAddrerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_addrerr_clr(&self) -> bool {
        *self == MisAddrerr::MisAddrerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_addrerr_set(&self) -> bool {
        *self == MisAddrerr::MisAddrerrSet
    }
}
#[doc = "DMA data error, SRC data might be corrupted (PAR or ECC error).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisDataerr {
    #[doc = "0: CLR"]
    MisDataerrClr = 0,
    #[doc = "1: SET"]
    MisDataerrSet = 1,
}
impl From<MisDataerr> for bool {
    #[inline(always)]
    fn from(variant: MisDataerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_DATAERR` reader - DMA data error, SRC data might be corrupted (PAR or ECC error)."]
pub type MisDataerrR = crate::BitReader<MisDataerr>;
impl MisDataerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisDataerr {
        match self.bits {
            false => MisDataerr::MisDataerrClr,
            true => MisDataerr::MisDataerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_dataerr_clr(&self) -> bool {
        *self == MisDataerr::MisDataerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_dataerr_set(&self) -> bool {
        *self == MisDataerr::MisDataerrSet
    }
}
impl R {
    #[doc = "Bit 0 - DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn mis_dmach0(&self) -> MisDmach0R {
        MisDmach0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn mis_dmach1(&self) -> MisDmach1R {
        MisDmach1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn mis_dmach2(&self) -> MisDmach2R {
        MisDmach2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn mis_dmach3(&self) -> MisDmach3R {
        MisDmach3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn mis_dmach4(&self) -> MisDmach4R {
        MisDmach4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn mis_dmach5(&self) -> MisDmach5R {
        MisDmach5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn mis_dmach6(&self) -> MisDmach6R {
        MisDmach6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 16 - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn mis_preirqch0(&self) -> MisPreirqch0R {
        MisPreirqch0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn mis_preirqch1(&self) -> MisPreirqch1R {
        MisPreirqch1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn mis_preirqch2(&self) -> MisPreirqch2R {
        MisPreirqch2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA address error, SRC address not reachable."]
    #[inline(always)]
    pub fn mis_addrerr(&self) -> MisAddrerrR {
        MisAddrerrR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA data error, SRC data might be corrupted (PAR or ECC error)."]
    #[inline(always)]
    pub fn mis_dataerr(&self) -> MisDataerrR {
        MisDataerrR::new(((self.bits >> 25) & 1) != 0)
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
