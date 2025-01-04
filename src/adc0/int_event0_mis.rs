#[doc = "Register `INT_EVENT0_MIS` reader"]
pub type R = crate::R<IntEvent0MisSpec>;
#[doc = "Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisOvifg {
    #[doc = "0: CLR"]
    IntEvent0MisOvifgClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisOvifgSet = 1,
}
impl From<IntEvent0MisOvifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisOvifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_OVIFG` reader - Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0MisOvifgR = crate::BitReader<IntEvent0MisOvifg>;
impl IntEvent0MisOvifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisOvifg {
        match self.bits {
            false => IntEvent0MisOvifg::IntEvent0MisOvifgClr,
            true => IntEvent0MisOvifg::IntEvent0MisOvifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_ovifg_clr(&self) -> bool {
        *self == IntEvent0MisOvifg::IntEvent0MisOvifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_ovifg_set(&self) -> bool {
        *self == IntEvent0MisOvifg::IntEvent0MisOvifgSet
    }
}
#[doc = "Raw interrupt flag for sequence conversion timeout overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisTovifg {
    #[doc = "0: CLR"]
    IntEvent0MisTovifgClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisTovifgSet = 1,
}
impl From<IntEvent0MisTovifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisTovifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_TOVIFG` reader - Raw interrupt flag for sequence conversion timeout overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0MisTovifgR = crate::BitReader<IntEvent0MisTovifg>;
impl IntEvent0MisTovifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisTovifg {
        match self.bits {
            false => IntEvent0MisTovifg::IntEvent0MisTovifgClr,
            true => IntEvent0MisTovifg::IntEvent0MisTovifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_tovifg_clr(&self) -> bool {
        *self == IntEvent0MisTovifg::IntEvent0MisTovifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_tovifg_set(&self) -> bool {
        *self == IntEvent0MisTovifg::IntEvent0MisTovifgSet
    }
}
#[doc = "Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisHighifg {
    #[doc = "0: CLR"]
    IntEvent0MisHighifgClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisHighifgSet = 1,
}
impl From<IntEvent0MisHighifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisHighifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_HIGHIFG` reader - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0MisHighifgR = crate::BitReader<IntEvent0MisHighifg>;
impl IntEvent0MisHighifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisHighifg {
        match self.bits {
            false => IntEvent0MisHighifg::IntEvent0MisHighifgClr,
            true => IntEvent0MisHighifg::IntEvent0MisHighifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_highifg_clr(&self) -> bool {
        *self == IntEvent0MisHighifg::IntEvent0MisHighifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_highifg_set(&self) -> bool {
        *self == IntEvent0MisHighifg::IntEvent0MisHighifgSet
    }
}
#[doc = "Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisLowifg {
    #[doc = "0: CLR"]
    IntEvent0MisLowifgClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisLowifgSet = 1,
}
impl From<IntEvent0MisLowifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisLowifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_LOWIFG` reader - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0MisLowifgR = crate::BitReader<IntEvent0MisLowifg>;
impl IntEvent0MisLowifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisLowifg {
        match self.bits {
            false => IntEvent0MisLowifg::IntEvent0MisLowifgClr,
            true => IntEvent0MisLowifg::IntEvent0MisLowifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_lowifg_clr(&self) -> bool {
        *self == IntEvent0MisLowifg::IntEvent0MisLowifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_lowifg_set(&self) -> bool {
        *self == IntEvent0MisLowifg::IntEvent0MisLowifgSet
    }
}
#[doc = "Mask INIFG in MIS_EX register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisInifg {
    #[doc = "0: CLR"]
    IntEvent0MisInifgClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisInifgSet = 1,
}
impl From<IntEvent0MisInifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisInifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_INIFG` reader - Mask INIFG in MIS_EX register."]
pub type IntEvent0MisInifgR = crate::BitReader<IntEvent0MisInifg>;
impl IntEvent0MisInifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisInifg {
        match self.bits {
            false => IntEvent0MisInifg::IntEvent0MisInifgClr,
            true => IntEvent0MisInifg::IntEvent0MisInifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_inifg_clr(&self) -> bool {
        *self == IntEvent0MisInifg::IntEvent0MisInifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_inifg_set(&self) -> bool {
        *self == IntEvent0MisInifg::IntEvent0MisInifgSet
    }
}
#[doc = "Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDmadone {
    #[doc = "0: CLR"]
    IntEvent0MisDmadoneClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDmadoneSet = 1,
}
impl From<IntEvent0MisDmadone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDmadone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DMADONE` reader - Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent0MisDmadoneR = crate::BitReader<IntEvent0MisDmadone>;
impl IntEvent0MisDmadoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDmadone {
        match self.bits {
            false => IntEvent0MisDmadone::IntEvent0MisDmadoneClr,
            true => IntEvent0MisDmadone::IntEvent0MisDmadoneSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dmadone_clr(&self) -> bool {
        *self == IntEvent0MisDmadone::IntEvent0MisDmadoneClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dmadone_set(&self) -> bool {
        *self == IntEvent0MisDmadone::IntEvent0MisDmadoneSet
    }
}
#[doc = "Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisUvifg {
    #[doc = "0: CLR"]
    IntEvent0MisUvifgClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisUvifgSet = 1,
}
impl From<IntEvent0MisUvifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisUvifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_UVIFG` reader - Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR is set to 1."]
pub type IntEvent0MisUvifgR = crate::BitReader<IntEvent0MisUvifg>;
impl IntEvent0MisUvifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisUvifg {
        match self.bits {
            false => IntEvent0MisUvifg::IntEvent0MisUvifgClr,
            true => IntEvent0MisUvifg::IntEvent0MisUvifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_uvifg_clr(&self) -> bool {
        *self == IntEvent0MisUvifg::IntEvent0MisUvifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_uvifg_set(&self) -> bool {
        *self == IntEvent0MisUvifg::IntEvent0MisUvifgSet
    }
}
#[doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMemresifg0 {
    #[doc = "0: CLR"]
    IntEvent0MisMemresifg0Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMemresifg0Set = 1,
}
impl From<IntEvent0MisMemresifg0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMemresifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MEMRESIFG0` reader - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0MisMemresifg0R = crate::BitReader<IntEvent0MisMemresifg0>;
impl IntEvent0MisMemresifg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMemresifg0 {
        match self.bits {
            false => IntEvent0MisMemresifg0::IntEvent0MisMemresifg0Clr,
            true => IntEvent0MisMemresifg0::IntEvent0MisMemresifg0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg0_clr(&self) -> bool {
        *self == IntEvent0MisMemresifg0::IntEvent0MisMemresifg0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg0_set(&self) -> bool {
        *self == IntEvent0MisMemresifg0::IntEvent0MisMemresifg0Set
    }
}
#[doc = "Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMemresifg1 {
    #[doc = "0: CLR"]
    IntEvent0MisMemresifg1Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMemresifg1Set = 1,
}
impl From<IntEvent0MisMemresifg1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMemresifg1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MEMRESIFG1` reader - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0MisMemresifg1R = crate::BitReader<IntEvent0MisMemresifg1>;
impl IntEvent0MisMemresifg1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMemresifg1 {
        match self.bits {
            false => IntEvent0MisMemresifg1::IntEvent0MisMemresifg1Clr,
            true => IntEvent0MisMemresifg1::IntEvent0MisMemresifg1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg1_clr(&self) -> bool {
        *self == IntEvent0MisMemresifg1::IntEvent0MisMemresifg1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg1_set(&self) -> bool {
        *self == IntEvent0MisMemresifg1::IntEvent0MisMemresifg1Set
    }
}
#[doc = "Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMemresifg2 {
    #[doc = "0: CLR"]
    IntEvent0MisMemresifg2Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMemresifg2Set = 1,
}
impl From<IntEvent0MisMemresifg2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMemresifg2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MEMRESIFG2` reader - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0MisMemresifg2R = crate::BitReader<IntEvent0MisMemresifg2>;
impl IntEvent0MisMemresifg2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMemresifg2 {
        match self.bits {
            false => IntEvent0MisMemresifg2::IntEvent0MisMemresifg2Clr,
            true => IntEvent0MisMemresifg2::IntEvent0MisMemresifg2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg2_clr(&self) -> bool {
        *self == IntEvent0MisMemresifg2::IntEvent0MisMemresifg2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg2_set(&self) -> bool {
        *self == IntEvent0MisMemresifg2::IntEvent0MisMemresifg2Set
    }
}
#[doc = "Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMemresifg3 {
    #[doc = "0: CLR"]
    IntEvent0MisMemresifg3Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMemresifg3Set = 1,
}
impl From<IntEvent0MisMemresifg3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMemresifg3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MEMRESIFG3` reader - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0MisMemresifg3R = crate::BitReader<IntEvent0MisMemresifg3>;
impl IntEvent0MisMemresifg3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMemresifg3 {
        match self.bits {
            false => IntEvent0MisMemresifg3::IntEvent0MisMemresifg3Clr,
            true => IntEvent0MisMemresifg3::IntEvent0MisMemresifg3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg3_clr(&self) -> bool {
        *self == IntEvent0MisMemresifg3::IntEvent0MisMemresifg3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg3_set(&self) -> bool {
        *self == IntEvent0MisMemresifg3::IntEvent0MisMemresifg3Set
    }
}
#[doc = "Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMemresifg4 {
    #[doc = "0: CLR"]
    IntEvent0MisMemresifg4Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMemresifg4Set = 1,
}
impl From<IntEvent0MisMemresifg4> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMemresifg4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MEMRESIFG4` reader - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0MisMemresifg4R = crate::BitReader<IntEvent0MisMemresifg4>;
impl IntEvent0MisMemresifg4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMemresifg4 {
        match self.bits {
            false => IntEvent0MisMemresifg4::IntEvent0MisMemresifg4Clr,
            true => IntEvent0MisMemresifg4::IntEvent0MisMemresifg4Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg4_clr(&self) -> bool {
        *self == IntEvent0MisMemresifg4::IntEvent0MisMemresifg4Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg4_set(&self) -> bool {
        *self == IntEvent0MisMemresifg4::IntEvent0MisMemresifg4Set
    }
}
#[doc = "Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMemresifg5 {
    #[doc = "0: CLR"]
    IntEvent0MisMemresifg5Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMemresifg5Set = 1,
}
impl From<IntEvent0MisMemresifg5> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMemresifg5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MEMRESIFG5` reader - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0MisMemresifg5R = crate::BitReader<IntEvent0MisMemresifg5>;
impl IntEvent0MisMemresifg5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMemresifg5 {
        match self.bits {
            false => IntEvent0MisMemresifg5::IntEvent0MisMemresifg5Clr,
            true => IntEvent0MisMemresifg5::IntEvent0MisMemresifg5Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg5_clr(&self) -> bool {
        *self == IntEvent0MisMemresifg5::IntEvent0MisMemresifg5Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg5_set(&self) -> bool {
        *self == IntEvent0MisMemresifg5::IntEvent0MisMemresifg5Set
    }
}
#[doc = "Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMemresifg6 {
    #[doc = "0: CLR"]
    IntEvent0MisMemresifg6Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMemresifg6Set = 1,
}
impl From<IntEvent0MisMemresifg6> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMemresifg6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MEMRESIFG6` reader - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0MisMemresifg6R = crate::BitReader<IntEvent0MisMemresifg6>;
impl IntEvent0MisMemresifg6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMemresifg6 {
        match self.bits {
            false => IntEvent0MisMemresifg6::IntEvent0MisMemresifg6Clr,
            true => IntEvent0MisMemresifg6::IntEvent0MisMemresifg6Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg6_clr(&self) -> bool {
        *self == IntEvent0MisMemresifg6::IntEvent0MisMemresifg6Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg6_set(&self) -> bool {
        *self == IntEvent0MisMemresifg6::IntEvent0MisMemresifg6Set
    }
}
#[doc = "Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMemresifg7 {
    #[doc = "0: CLR"]
    IntEvent0MisMemresifg7Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMemresifg7Set = 1,
}
impl From<IntEvent0MisMemresifg7> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMemresifg7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MEMRESIFG7` reader - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0MisMemresifg7R = crate::BitReader<IntEvent0MisMemresifg7>;
impl IntEvent0MisMemresifg7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMemresifg7 {
        match self.bits {
            false => IntEvent0MisMemresifg7::IntEvent0MisMemresifg7Clr,
            true => IntEvent0MisMemresifg7::IntEvent0MisMemresifg7Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg7_clr(&self) -> bool {
        *self == IntEvent0MisMemresifg7::IntEvent0MisMemresifg7Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg7_set(&self) -> bool {
        *self == IntEvent0MisMemresifg7::IntEvent0MisMemresifg7Set
    }
}
#[doc = "Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMemresifg8 {
    #[doc = "0: CLR"]
    IntEvent0MisMemresifg8Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMemresifg8Set = 1,
}
impl From<IntEvent0MisMemresifg8> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMemresifg8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MEMRESIFG8` reader - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0MisMemresifg8R = crate::BitReader<IntEvent0MisMemresifg8>;
impl IntEvent0MisMemresifg8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMemresifg8 {
        match self.bits {
            false => IntEvent0MisMemresifg8::IntEvent0MisMemresifg8Clr,
            true => IntEvent0MisMemresifg8::IntEvent0MisMemresifg8Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg8_clr(&self) -> bool {
        *self == IntEvent0MisMemresifg8::IntEvent0MisMemresifg8Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg8_set(&self) -> bool {
        *self == IntEvent0MisMemresifg8::IntEvent0MisMemresifg8Set
    }
}
#[doc = "Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMemresifg9 {
    #[doc = "0: CLR"]
    IntEvent0MisMemresifg9Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMemresifg9Set = 1,
}
impl From<IntEvent0MisMemresifg9> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMemresifg9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MEMRESIFG9` reader - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0MisMemresifg9R = crate::BitReader<IntEvent0MisMemresifg9>;
impl IntEvent0MisMemresifg9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMemresifg9 {
        match self.bits {
            false => IntEvent0MisMemresifg9::IntEvent0MisMemresifg9Clr,
            true => IntEvent0MisMemresifg9::IntEvent0MisMemresifg9Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg9_clr(&self) -> bool {
        *self == IntEvent0MisMemresifg9::IntEvent0MisMemresifg9Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg9_set(&self) -> bool {
        *self == IntEvent0MisMemresifg9::IntEvent0MisMemresifg9Set
    }
}
#[doc = "Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMemresifg10 {
    #[doc = "0: CLR"]
    IntEvent0MisMemresifg10Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMemresifg10Set = 1,
}
impl From<IntEvent0MisMemresifg10> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMemresifg10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MEMRESIFG10` reader - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0MisMemresifg10R = crate::BitReader<IntEvent0MisMemresifg10>;
impl IntEvent0MisMemresifg10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMemresifg10 {
        match self.bits {
            false => IntEvent0MisMemresifg10::IntEvent0MisMemresifg10Clr,
            true => IntEvent0MisMemresifg10::IntEvent0MisMemresifg10Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg10_clr(&self) -> bool {
        *self == IntEvent0MisMemresifg10::IntEvent0MisMemresifg10Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg10_set(&self) -> bool {
        *self == IntEvent0MisMemresifg10::IntEvent0MisMemresifg10Set
    }
}
#[doc = "Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMemresifg11 {
    #[doc = "0: CLR"]
    IntEvent0MisMemresifg11Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMemresifg11Set = 1,
}
impl From<IntEvent0MisMemresifg11> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMemresifg11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MEMRESIFG11` reader - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent0MisMemresifg11R = crate::BitReader<IntEvent0MisMemresifg11>;
impl IntEvent0MisMemresifg11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMemresifg11 {
        match self.bits {
            false => IntEvent0MisMemresifg11::IntEvent0MisMemresifg11Clr,
            true => IntEvent0MisMemresifg11::IntEvent0MisMemresifg11Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg11_clr(&self) -> bool {
        *self == IntEvent0MisMemresifg11::IntEvent0MisMemresifg11Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_memresifg11_set(&self) -> bool {
        *self == IntEvent0MisMemresifg11::IntEvent0MisMemresifg11Set
    }
}
impl R {
    #[doc = "Bit 0 - Raw interrupt flag for MEMRESx overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_mis_ovifg(&self) -> IntEvent0MisOvifgR {
        IntEvent0MisOvifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw interrupt flag for sequence conversion timeout overflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_mis_tovifg(&self) -> IntEvent0MisTovifgR {
        IntEvent0MisTovifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_mis_highifg(&self) -> IntEvent0MisHighifgR {
        IntEvent0MisHighifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_mis_lowifg(&self) -> IntEvent0MisLowifgR {
        IntEvent0MisLowifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask INIFG in MIS_EX register."]
    #[inline(always)]
    pub fn int_event0_mis_inifg(&self) -> IntEvent0MisInifgR {
        IntEvent0MisInifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Raw interrupt flag for DMADONE. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event0_mis_dmadone(&self) -> IntEvent0MisDmadoneR {
        IntEvent0MisDmadoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Raw interrupt flag for MEMRESx underflow. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR is set to 1."]
    #[inline(always)]
    pub fn int_event0_mis_uvifg(&self) -> IntEvent0MisUvifgR {
        IntEvent0MisUvifgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_mis_memresifg0(&self) -> IntEvent0MisMemresifg0R {
        IntEvent0MisMemresifg0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Raw interrupt status for MEMRES1. This bit is set to 1 when MEMRES1 is loaded with a new conversion result. Reading MEMRES1 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_mis_memresifg1(&self) -> IntEvent0MisMemresifg1R {
        IntEvent0MisMemresifg1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Raw interrupt status for MEMRES2. This bit is set to 1 when MEMRES2 is loaded with a new conversion result. Reading MEMRES2 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_mis_memresifg2(&self) -> IntEvent0MisMemresifg2R {
        IntEvent0MisMemresifg2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Raw interrupt status for MEMRES3. This bit is set to 1 when MEMRES3 is loaded with a new conversion result. Reading MEMRES3 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_mis_memresifg3(&self) -> IntEvent0MisMemresifg3R {
        IntEvent0MisMemresifg3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Raw interrupt status for MEMRES4. This bit is set to 1 when MEMRES4 is loaded with a new conversion result. Reading MEMRES4 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_mis_memresifg4(&self) -> IntEvent0MisMemresifg4R {
        IntEvent0MisMemresifg4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Raw interrupt status for MEMRES5. This bit is set to 1 when MEMRES5 is loaded with a new conversion result. Reading MEMRES5 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_mis_memresifg5(&self) -> IntEvent0MisMemresifg5R {
        IntEvent0MisMemresifg5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Raw interrupt status for MEMRES6. This bit is set to 1 when MEMRES6 is loaded with a new conversion result. Reading MEMRES6 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_mis_memresifg6(&self) -> IntEvent0MisMemresifg6R {
        IntEvent0MisMemresifg6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Raw interrupt status for MEMRES7. This bit is set to 1 when MEMRES7 is loaded with a new conversion result. Reading MEMRES7 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_mis_memresifg7(&self) -> IntEvent0MisMemresifg7R {
        IntEvent0MisMemresifg7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Raw interrupt status for MEMRES8. This bit is set to 1 when MEMRES8 is loaded with a new conversion result. Reading MEMRES8 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_mis_memresifg8(&self) -> IntEvent0MisMemresifg8R {
        IntEvent0MisMemresifg8R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Raw interrupt status for MEMRES9. This bit is set to 1 when MEMRES9 is loaded with a new conversion result. Reading MEMRES9 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_mis_memresifg9(&self) -> IntEvent0MisMemresifg9R {
        IntEvent0MisMemresifg9R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Raw interrupt status for MEMRES10. This bit is set to 1 when MEMRES10 is loaded with a new conversion result. Reading MEMRES10 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_mis_memresifg10(&self) -> IntEvent0MisMemresifg10R {
        IntEvent0MisMemresifg10R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Raw interrupt status for MEMRES11. This bit is set to 1 when MEMRES11 is loaded with a new conversion result. Reading MEMRES11 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event0_mis_memresifg11(&self) -> IntEvent0MisMemresifg11R {
        IntEvent0MisMemresifg11R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0MisSpec;
impl crate::RegisterSpec for IntEvent0MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event0_mis::R`](R) reader structure"]
impl crate::Readable for IntEvent0MisSpec {}
#[doc = "`reset()` method sets INT_EVENT0_MIS to value 0"]
impl crate::Resettable for IntEvent0MisSpec {
    const RESET_VALUE: u32 = 0;
}
