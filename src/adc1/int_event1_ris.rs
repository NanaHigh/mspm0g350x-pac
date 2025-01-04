#[doc = "Register `INT_EVENT1_RIS` reader"]
pub type R = crate::R<IntEvent1RisSpec>;
#[doc = "Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisHighifg {
    #[doc = "0: CLR"]
    IntEvent1RisHighifgClr = 0,
    #[doc = "1: SET"]
    IntEvent1RisHighifgSet = 1,
}
impl From<IntEvent1RisHighifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisHighifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_HIGHIFG` reader - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent1RisHighifgR = crate::BitReader<IntEvent1RisHighifg>;
impl IntEvent1RisHighifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisHighifg {
        match self.bits {
            false => IntEvent1RisHighifg::IntEvent1RisHighifgClr,
            true => IntEvent1RisHighifg::IntEvent1RisHighifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_highifg_clr(&self) -> bool {
        *self == IntEvent1RisHighifg::IntEvent1RisHighifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_highifg_set(&self) -> bool {
        *self == IntEvent1RisHighifg::IntEvent1RisHighifgSet
    }
}
#[doc = "Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisLowifg {
    #[doc = "0: CLR"]
    IntEvent1RisLowifgClr = 0,
    #[doc = "1: SET"]
    IntEvent1RisLowifgSet = 1,
}
impl From<IntEvent1RisLowifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisLowifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_LOWIFG` reader - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent1RisLowifgR = crate::BitReader<IntEvent1RisLowifg>;
impl IntEvent1RisLowifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisLowifg {
        match self.bits {
            false => IntEvent1RisLowifg::IntEvent1RisLowifgClr,
            true => IntEvent1RisLowifg::IntEvent1RisLowifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_lowifg_clr(&self) -> bool {
        *self == IntEvent1RisLowifg::IntEvent1RisLowifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_lowifg_set(&self) -> bool {
        *self == IntEvent1RisLowifg::IntEvent1RisLowifgSet
    }
}
#[doc = "Mask INIFG in MIS_EX register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisInifg {
    #[doc = "0: CLR"]
    IntEvent1RisInifgClr = 0,
    #[doc = "1: SET"]
    IntEvent1RisInifgSet = 1,
}
impl From<IntEvent1RisInifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisInifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_INIFG` reader - Mask INIFG in MIS_EX register."]
pub type IntEvent1RisInifgR = crate::BitReader<IntEvent1RisInifg>;
impl IntEvent1RisInifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisInifg {
        match self.bits {
            false => IntEvent1RisInifg::IntEvent1RisInifgClr,
            true => IntEvent1RisInifg::IntEvent1RisInifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_inifg_clr(&self) -> bool {
        *self == IntEvent1RisInifg::IntEvent1RisInifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_inifg_set(&self) -> bool {
        *self == IntEvent1RisInifg::IntEvent1RisInifgSet
    }
}
#[doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisMemresifg0 {
    #[doc = "0: CLR"]
    IntEvent1RisMemresifg0Clr = 0,
    #[doc = "1: SET"]
    IntEvent1RisMemresifg0Set = 1,
}
impl From<IntEvent1RisMemresifg0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisMemresifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_MEMRESIFG0` reader - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent1RisMemresifg0R = crate::BitReader<IntEvent1RisMemresifg0>;
impl IntEvent1RisMemresifg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisMemresifg0 {
        match self.bits {
            false => IntEvent1RisMemresifg0::IntEvent1RisMemresifg0Clr,
            true => IntEvent1RisMemresifg0::IntEvent1RisMemresifg0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_memresifg0_clr(&self) -> bool {
        *self == IntEvent1RisMemresifg0::IntEvent1RisMemresifg0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_memresifg0_set(&self) -> bool {
        *self == IntEvent1RisMemresifg0::IntEvent1RisMemresifg0Set
    }
}
impl R {
    #[doc = "Bit 2 - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event1_ris_highifg(&self) -> IntEvent1RisHighifgR {
        IntEvent1RisHighifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event1_ris_lowifg(&self) -> IntEvent1RisLowifgR {
        IntEvent1RisLowifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask INIFG in MIS_EX register."]
    #[inline(always)]
    pub fn int_event1_ris_inifg(&self) -> IntEvent1RisInifgR {
        IntEvent1RisInifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event1_ris_memresifg0(&self) -> IntEvent1RisMemresifg0R {
        IntEvent1RisMemresifg0R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1RisSpec;
impl crate::RegisterSpec for IntEvent1RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event1_ris::R`](R) reader structure"]
impl crate::Readable for IntEvent1RisSpec {}
#[doc = "`reset()` method sets INT_EVENT1_RIS to value 0"]
impl crate::Resettable for IntEvent1RisSpec {
    const RESET_VALUE: u32 = 0;
}
