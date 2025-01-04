#[doc = "Register `INT_EVENT0_RIS` reader"]
pub type R = crate::R<IntEvent0RisSpec>;
#[doc = "Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisOvifg {
    #[doc = "0: CLR"]
    IntEvent0RisOvifgClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisOvifgSet = 1,
}
impl From<IntEvent0RisOvifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisOvifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_OVIFG` reader - Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0RisOvifgR = crate::BitReader<IntEvent0RisOvifg>;
impl IntEvent0RisOvifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisOvifg {
        match self.bits {
            false => IntEvent0RisOvifg::IntEvent0RisOvifgClr,
            true => IntEvent0RisOvifg::IntEvent0RisOvifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_ovifg_clr(&self) -> bool {
        *self == IntEvent0RisOvifg::IntEvent0RisOvifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_ovifg_set(&self) -> bool {
        *self == IntEvent0RisOvifg::IntEvent0RisOvifgSet
    }
}
#[doc = "Raw interrupt flag for sequence conversion trigger overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisTovifg {
    #[doc = "0: CLR"]
    IntEvent0RisTovifgClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisTovifgSet = 1,
}
impl From<IntEvent0RisTovifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisTovifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_TOVIFG` reader - Raw interrupt flag for sequence conversion trigger overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0RisTovifgR = crate::BitReader<IntEvent0RisTovifg>;
impl IntEvent0RisTovifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisTovifg {
        match self.bits {
            false => IntEvent0RisTovifg::IntEvent0RisTovifgClr,
            true => IntEvent0RisTovifg::IntEvent0RisTovifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_tovifg_clr(&self) -> bool {
        *self == IntEvent0RisTovifg::IntEvent0RisTovifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_tovifg_set(&self) -> bool {
        *self == IntEvent0RisTovifg::IntEvent0RisTovifgSet
    }
}
#[doc = "Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisHighifg {
    #[doc = "0: CLR"]
    IntEvent0RisHighifgClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisHighifgSet = 1,
}
impl From<IntEvent0RisHighifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisHighifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_HIGHIFG` reader - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0RisHighifgR = crate::BitReader<IntEvent0RisHighifg>;
impl IntEvent0RisHighifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisHighifg {
        match self.bits {
            false => IntEvent0RisHighifg::IntEvent0RisHighifgClr,
            true => IntEvent0RisHighifg::IntEvent0RisHighifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_highifg_clr(&self) -> bool {
        *self == IntEvent0RisHighifg::IntEvent0RisHighifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_highifg_set(&self) -> bool {
        *self == IntEvent0RisHighifg::IntEvent0RisHighifgSet
    }
}
#[doc = "Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisLowifg {
    #[doc = "0: CLR"]
    IntEvent0RisLowifgClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisLowifgSet = 1,
}
impl From<IntEvent0RisLowifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisLowifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_LOWIFG` reader - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0RisLowifgR = crate::BitReader<IntEvent0RisLowifg>;
impl IntEvent0RisLowifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisLowifg {
        match self.bits {
            false => IntEvent0RisLowifg::IntEvent0RisLowifgClr,
            true => IntEvent0RisLowifg::IntEvent0RisLowifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_lowifg_clr(&self) -> bool {
        *self == IntEvent0RisLowifg::IntEvent0RisLowifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_lowifg_set(&self) -> bool {
        *self == IntEvent0RisLowifg::IntEvent0RisLowifgSet
    }
}
#[doc = "Mask INIFG in MIS_EX register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisInifg {
    #[doc = "0: CLR"]
    IntEvent0RisInifgClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisInifgSet = 1,
}
impl From<IntEvent0RisInifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisInifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_INIFG` reader - Mask INIFG in MIS_EX register."]
pub type IntEvent0RisInifgR = crate::BitReader<IntEvent0RisInifg>;
impl IntEvent0RisInifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisInifg {
        match self.bits {
            false => IntEvent0RisInifg::IntEvent0RisInifgClr,
            true => IntEvent0RisInifg::IntEvent0RisInifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_inifg_clr(&self) -> bool {
        *self == IntEvent0RisInifg::IntEvent0RisInifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_inifg_set(&self) -> bool {
        *self == IntEvent0RisInifg::IntEvent0RisInifgSet
    }
}
#[doc = "Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDmadone {
    #[doc = "0: CLR"]
    IntEvent0RisDmadoneClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDmadoneSet = 1,
}
impl From<IntEvent0RisDmadone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDmadone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DMADONE` reader - Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0RisDmadoneR = crate::BitReader<IntEvent0RisDmadone>;
impl IntEvent0RisDmadoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDmadone {
        match self.bits {
            false => IntEvent0RisDmadone::IntEvent0RisDmadoneClr,
            true => IntEvent0RisDmadone::IntEvent0RisDmadoneSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dmadone_clr(&self) -> bool {
        *self == IntEvent0RisDmadone::IntEvent0RisDmadoneClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dmadone_set(&self) -> bool {
        *self == IntEvent0RisDmadone::IntEvent0RisDmadoneSet
    }
}
#[doc = "Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisUvifg {
    #[doc = "0: CLR"]
    IntEvent0RisUvifgClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisUvifgSet = 1,
}
impl From<IntEvent0RisUvifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisUvifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_UVIFG` reader - Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR is set to 1."]
pub type IntEvent0RisUvifgR = crate::BitReader<IntEvent0RisUvifg>;
impl IntEvent0RisUvifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisUvifg {
        match self.bits {
            false => IntEvent0RisUvifg::IntEvent0RisUvifgClr,
            true => IntEvent0RisUvifg::IntEvent0RisUvifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_uvifg_clr(&self) -> bool {
        *self == IntEvent0RisUvifg::IntEvent0RisUvifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_uvifg_set(&self) -> bool {
        *self == IntEvent0RisUvifg::IntEvent0RisUvifgSet
    }
}
#[doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMemresifg0 {
    #[doc = "0: CLR"]
    IntEvent0RisMemresifg0Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMemresifg0Set = 1,
}
impl From<IntEvent0RisMemresifg0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMemresifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MEMRESIFG0` reader - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0RisMemresifg0R = crate::BitReader<IntEvent0RisMemresifg0>;
impl IntEvent0RisMemresifg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMemresifg0 {
        match self.bits {
            false => IntEvent0RisMemresifg0::IntEvent0RisMemresifg0Clr,
            true => IntEvent0RisMemresifg0::IntEvent0RisMemresifg0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg0_clr(&self) -> bool {
        *self == IntEvent0RisMemresifg0::IntEvent0RisMemresifg0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg0_set(&self) -> bool {
        *self == IntEvent0RisMemresifg0::IntEvent0RisMemresifg0Set
    }
}
#[doc = "Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMemresifg1 {
    #[doc = "0: CLR"]
    IntEvent0RisMemresifg1Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMemresifg1Set = 1,
}
impl From<IntEvent0RisMemresifg1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMemresifg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MEMRESIFG1` reader - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0RisMemresifg1R = crate::BitReader<IntEvent0RisMemresifg1>;
impl IntEvent0RisMemresifg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMemresifg1 {
        match self.bits {
            false => IntEvent0RisMemresifg1::IntEvent0RisMemresifg1Clr,
            true => IntEvent0RisMemresifg1::IntEvent0RisMemresifg1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg1_clr(&self) -> bool {
        *self == IntEvent0RisMemresifg1::IntEvent0RisMemresifg1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg1_set(&self) -> bool {
        *self == IntEvent0RisMemresifg1::IntEvent0RisMemresifg1Set
    }
}
#[doc = "Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMemresifg2 {
    #[doc = "0: CLR"]
    IntEvent0RisMemresifg2Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMemresifg2Set = 1,
}
impl From<IntEvent0RisMemresifg2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMemresifg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MEMRESIFG2` reader - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0RisMemresifg2R = crate::BitReader<IntEvent0RisMemresifg2>;
impl IntEvent0RisMemresifg2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMemresifg2 {
        match self.bits {
            false => IntEvent0RisMemresifg2::IntEvent0RisMemresifg2Clr,
            true => IntEvent0RisMemresifg2::IntEvent0RisMemresifg2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg2_clr(&self) -> bool {
        *self == IntEvent0RisMemresifg2::IntEvent0RisMemresifg2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg2_set(&self) -> bool {
        *self == IntEvent0RisMemresifg2::IntEvent0RisMemresifg2Set
    }
}
#[doc = "Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMemresifg3 {
    #[doc = "0: CLR"]
    IntEvent0RisMemresifg3Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMemresifg3Set = 1,
}
impl From<IntEvent0RisMemresifg3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMemresifg3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MEMRESIFG3` reader - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0RisMemresifg3R = crate::BitReader<IntEvent0RisMemresifg3>;
impl IntEvent0RisMemresifg3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMemresifg3 {
        match self.bits {
            false => IntEvent0RisMemresifg3::IntEvent0RisMemresifg3Clr,
            true => IntEvent0RisMemresifg3::IntEvent0RisMemresifg3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg3_clr(&self) -> bool {
        *self == IntEvent0RisMemresifg3::IntEvent0RisMemresifg3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg3_set(&self) -> bool {
        *self == IntEvent0RisMemresifg3::IntEvent0RisMemresifg3Set
    }
}
#[doc = "Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMemresifg4 {
    #[doc = "0: CLR"]
    IntEvent0RisMemresifg4Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMemresifg4Set = 1,
}
impl From<IntEvent0RisMemresifg4> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMemresifg4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MEMRESIFG4` reader - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0RisMemresifg4R = crate::BitReader<IntEvent0RisMemresifg4>;
impl IntEvent0RisMemresifg4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMemresifg4 {
        match self.bits {
            false => IntEvent0RisMemresifg4::IntEvent0RisMemresifg4Clr,
            true => IntEvent0RisMemresifg4::IntEvent0RisMemresifg4Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg4_clr(&self) -> bool {
        *self == IntEvent0RisMemresifg4::IntEvent0RisMemresifg4Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg4_set(&self) -> bool {
        *self == IntEvent0RisMemresifg4::IntEvent0RisMemresifg4Set
    }
}
#[doc = "Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMemresifg5 {
    #[doc = "0: CLR"]
    IntEvent0RisMemresifg5Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMemresifg5Set = 1,
}
impl From<IntEvent0RisMemresifg5> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMemresifg5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MEMRESIFG5` reader - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0RisMemresifg5R = crate::BitReader<IntEvent0RisMemresifg5>;
impl IntEvent0RisMemresifg5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMemresifg5 {
        match self.bits {
            false => IntEvent0RisMemresifg5::IntEvent0RisMemresifg5Clr,
            true => IntEvent0RisMemresifg5::IntEvent0RisMemresifg5Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg5_clr(&self) -> bool {
        *self == IntEvent0RisMemresifg5::IntEvent0RisMemresifg5Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg5_set(&self) -> bool {
        *self == IntEvent0RisMemresifg5::IntEvent0RisMemresifg5Set
    }
}
#[doc = "Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMemresifg6 {
    #[doc = "0: CLR"]
    IntEvent0RisMemresifg6Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMemresifg6Set = 1,
}
impl From<IntEvent0RisMemresifg6> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMemresifg6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MEMRESIFG6` reader - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0RisMemresifg6R = crate::BitReader<IntEvent0RisMemresifg6>;
impl IntEvent0RisMemresifg6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMemresifg6 {
        match self.bits {
            false => IntEvent0RisMemresifg6::IntEvent0RisMemresifg6Clr,
            true => IntEvent0RisMemresifg6::IntEvent0RisMemresifg6Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg6_clr(&self) -> bool {
        *self == IntEvent0RisMemresifg6::IntEvent0RisMemresifg6Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg6_set(&self) -> bool {
        *self == IntEvent0RisMemresifg6::IntEvent0RisMemresifg6Set
    }
}
#[doc = "Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMemresifg7 {
    #[doc = "0: CLR"]
    IntEvent0RisMemresifg7Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMemresifg7Set = 1,
}
impl From<IntEvent0RisMemresifg7> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMemresifg7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MEMRESIFG7` reader - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0RisMemresifg7R = crate::BitReader<IntEvent0RisMemresifg7>;
impl IntEvent0RisMemresifg7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMemresifg7 {
        match self.bits {
            false => IntEvent0RisMemresifg7::IntEvent0RisMemresifg7Clr,
            true => IntEvent0RisMemresifg7::IntEvent0RisMemresifg7Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg7_clr(&self) -> bool {
        *self == IntEvent0RisMemresifg7::IntEvent0RisMemresifg7Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg7_set(&self) -> bool {
        *self == IntEvent0RisMemresifg7::IntEvent0RisMemresifg7Set
    }
}
#[doc = "Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMemresifg8 {
    #[doc = "0: CLR"]
    IntEvent0RisMemresifg8Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMemresifg8Set = 1,
}
impl From<IntEvent0RisMemresifg8> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMemresifg8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MEMRESIFG8` reader - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0RisMemresifg8R = crate::BitReader<IntEvent0RisMemresifg8>;
impl IntEvent0RisMemresifg8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMemresifg8 {
        match self.bits {
            false => IntEvent0RisMemresifg8::IntEvent0RisMemresifg8Clr,
            true => IntEvent0RisMemresifg8::IntEvent0RisMemresifg8Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg8_clr(&self) -> bool {
        *self == IntEvent0RisMemresifg8::IntEvent0RisMemresifg8Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg8_set(&self) -> bool {
        *self == IntEvent0RisMemresifg8::IntEvent0RisMemresifg8Set
    }
}
#[doc = "Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMemresifg9 {
    #[doc = "0: CLR"]
    IntEvent0RisMemresifg9Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMemresifg9Set = 1,
}
impl From<IntEvent0RisMemresifg9> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMemresifg9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MEMRESIFG9` reader - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0RisMemresifg9R = crate::BitReader<IntEvent0RisMemresifg9>;
impl IntEvent0RisMemresifg9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMemresifg9 {
        match self.bits {
            false => IntEvent0RisMemresifg9::IntEvent0RisMemresifg9Clr,
            true => IntEvent0RisMemresifg9::IntEvent0RisMemresifg9Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg9_clr(&self) -> bool {
        *self == IntEvent0RisMemresifg9::IntEvent0RisMemresifg9Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg9_set(&self) -> bool {
        *self == IntEvent0RisMemresifg9::IntEvent0RisMemresifg9Set
    }
}
#[doc = "Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMemresifg10 {
    #[doc = "0: CLR"]
    IntEvent0RisMemresifg10Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMemresifg10Set = 1,
}
impl From<IntEvent0RisMemresifg10> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMemresifg10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MEMRESIFG10` reader - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0RisMemresifg10R = crate::BitReader<IntEvent0RisMemresifg10>;
impl IntEvent0RisMemresifg10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMemresifg10 {
        match self.bits {
            false => IntEvent0RisMemresifg10::IntEvent0RisMemresifg10Clr,
            true => IntEvent0RisMemresifg10::IntEvent0RisMemresifg10Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg10_clr(&self) -> bool {
        *self == IntEvent0RisMemresifg10::IntEvent0RisMemresifg10Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg10_set(&self) -> bool {
        *self == IntEvent0RisMemresifg10::IntEvent0RisMemresifg10Set
    }
}
#[doc = "Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMemresifg11 {
    #[doc = "0: CLR"]
    IntEvent0RisMemresifg11Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMemresifg11Set = 1,
}
impl From<IntEvent0RisMemresifg11> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMemresifg11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MEMRESIFG11` reader - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0RisMemresifg11R = crate::BitReader<IntEvent0RisMemresifg11>;
impl IntEvent0RisMemresifg11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMemresifg11 {
        match self.bits {
            false => IntEvent0RisMemresifg11::IntEvent0RisMemresifg11Clr,
            true => IntEvent0RisMemresifg11::IntEvent0RisMemresifg11Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg11_clr(&self) -> bool {
        *self == IntEvent0RisMemresifg11::IntEvent0RisMemresifg11Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_memresifg11_set(&self) -> bool {
        *self == IntEvent0RisMemresifg11::IntEvent0RisMemresifg11Set
    }
}
impl R {
    #[doc = "Bit 0 - Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_ris_ovifg(&self) -> IntEvent0RisOvifgR {
        IntEvent0RisOvifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw interrupt flag for sequence conversion trigger overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_ris_tovifg(&self) -> IntEvent0RisTovifgR {
        IntEvent0RisTovifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_ris_highifg(&self) -> IntEvent0RisHighifgR {
        IntEvent0RisHighifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_ris_lowifg(&self) -> IntEvent0RisLowifgR {
        IntEvent0RisLowifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask INIFG in MIS_EX register."]
    #[inline(always)]
    pub fn int_event0_ris_inifg(&self) -> IntEvent0RisInifgR {
        IntEvent0RisInifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_ris_dmadone(&self) -> IntEvent0RisDmadoneR {
        IntEvent0RisDmadoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR is set to 1."]
    #[inline(always)]
    pub fn int_event0_ris_uvifg(&self) -> IntEvent0RisUvifgR {
        IntEvent0RisUvifgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_ris_memresifg0(&self) -> IntEvent0RisMemresifg0R {
        IntEvent0RisMemresifg0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_ris_memresifg1(&self) -> IntEvent0RisMemresifg1R {
        IntEvent0RisMemresifg1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_ris_memresifg2(&self) -> IntEvent0RisMemresifg2R {
        IntEvent0RisMemresifg2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_ris_memresifg3(&self) -> IntEvent0RisMemresifg3R {
        IntEvent0RisMemresifg3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_ris_memresifg4(&self) -> IntEvent0RisMemresifg4R {
        IntEvent0RisMemresifg4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_ris_memresifg5(&self) -> IntEvent0RisMemresifg5R {
        IntEvent0RisMemresifg5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_ris_memresifg6(&self) -> IntEvent0RisMemresifg6R {
        IntEvent0RisMemresifg6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_ris_memresifg7(&self) -> IntEvent0RisMemresifg7R {
        IntEvent0RisMemresifg7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_ris_memresifg8(&self) -> IntEvent0RisMemresifg8R {
        IntEvent0RisMemresifg8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_ris_memresifg9(&self) -> IntEvent0RisMemresifg9R {
        IntEvent0RisMemresifg9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_ris_memresifg10(&self) -> IntEvent0RisMemresifg10R {
        IntEvent0RisMemresifg10R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_ris_memresifg11(&self) -> IntEvent0RisMemresifg11R {
        IntEvent0RisMemresifg11R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0RisSpec;
impl crate::RegisterSpec for IntEvent0RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event0_ris::R`](R) reader structure"]
impl crate::Readable for IntEvent0RisSpec {}
#[doc = "`reset()` method sets INT_EVENT0_RIS to value 0"]
impl crate::Resettable for IntEvent0RisSpec {
    const RESET_VALUE: u32 = 0;
}
