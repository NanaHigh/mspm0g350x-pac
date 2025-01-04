#[doc = "Register `INT_EVENT1_MIS` reader"]
pub type R = crate::R<IntEvent1MisSpec>;
#[doc = "Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisHighifg {
    #[doc = "0: CLR"]
    IntEvent1MisHighifgClr = 0,
    #[doc = "1: SET"]
    IntEvent1MisHighifgSet = 1,
}
impl From<IntEvent1MisHighifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisHighifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_HIGHIFG` reader - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent1MisHighifgR = crate::BitReader<IntEvent1MisHighifg>;
impl IntEvent1MisHighifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisHighifg {
        match self.bits {
            false => IntEvent1MisHighifg::IntEvent1MisHighifgClr,
            true => IntEvent1MisHighifg::IntEvent1MisHighifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_highifg_clr(&self) -> bool {
        *self == IntEvent1MisHighifg::IntEvent1MisHighifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_highifg_set(&self) -> bool {
        *self == IntEvent1MisHighifg::IntEvent1MisHighifgSet
    }
}
#[doc = "Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisLowifg {
    #[doc = "0: CLR"]
    IntEvent1MisLowifgClr = 0,
    #[doc = "1: SET"]
    IntEvent1MisLowifgSet = 1,
}
impl From<IntEvent1MisLowifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisLowifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_LOWIFG` reader - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
pub type IntEvent1MisLowifgR = crate::BitReader<IntEvent1MisLowifg>;
impl IntEvent1MisLowifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisLowifg {
        match self.bits {
            false => IntEvent1MisLowifg::IntEvent1MisLowifgClr,
            true => IntEvent1MisLowifg::IntEvent1MisLowifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_lowifg_clr(&self) -> bool {
        *self == IntEvent1MisLowifg::IntEvent1MisLowifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_lowifg_set(&self) -> bool {
        *self == IntEvent1MisLowifg::IntEvent1MisLowifgSet
    }
}
#[doc = "Mask INIFG in MIS_EX register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisInifg {
    #[doc = "0: CLR"]
    IntEvent1MisInifgClr = 0,
    #[doc = "1: SET"]
    IntEvent1MisInifgSet = 1,
}
impl From<IntEvent1MisInifg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisInifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_INIFG` reader - Mask INIFG in MIS_EX register."]
pub type IntEvent1MisInifgR = crate::BitReader<IntEvent1MisInifg>;
impl IntEvent1MisInifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisInifg {
        match self.bits {
            false => IntEvent1MisInifg::IntEvent1MisInifgClr,
            true => IntEvent1MisInifg::IntEvent1MisInifgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_inifg_clr(&self) -> bool {
        *self == IntEvent1MisInifg::IntEvent1MisInifgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_inifg_set(&self) -> bool {
        *self == IntEvent1MisInifg::IntEvent1MisInifgSet
    }
}
#[doc = "Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisMemresifg0 {
    #[doc = "0: CLR"]
    IntEvent1MisMemresifg0Clr = 0,
    #[doc = "1: SET"]
    IntEvent1MisMemresifg0Set = 1,
}
impl From<IntEvent1MisMemresifg0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisMemresifg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_MEMRESIFG0` reader - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
pub type IntEvent1MisMemresifg0R = crate::BitReader<IntEvent1MisMemresifg0>;
impl IntEvent1MisMemresifg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisMemresifg0 {
        match self.bits {
            false => IntEvent1MisMemresifg0::IntEvent1MisMemresifg0Clr,
            true => IntEvent1MisMemresifg0::IntEvent1MisMemresifg0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_memresifg0_clr(&self) -> bool {
        *self == IntEvent1MisMemresifg0::IntEvent1MisMemresifg0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_memresifg0_set(&self) -> bool {
        *self == IntEvent1MisMemresifg0::IntEvent1MisMemresifg0Set
    }
}
impl R {
    #[doc = "Bit 2 - Raw interrupt flag for the MEMRESx result register being higher than the WCHIGHx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event1_mis_highifg(&self) -> IntEvent1MisHighifgR {
        IntEvent1MisHighifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw interrupt flag for the MEMRESx result register being below than the WCLOWx threshold of the window comparator. This bit is reset to 0 by IIDX read or when corresponding bit in ICLR_EX is set to 1."]
    #[inline(always)]
    pub fn int_event1_mis_lowifg(&self) -> IntEvent1MisLowifgR {
        IntEvent1MisLowifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Mask INIFG in MIS_EX register."]
    #[inline(always)]
    pub fn int_event1_mis_inifg(&self) -> IntEvent1MisInifgR {
        IntEvent1MisInifgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Raw interrupt status for MEMRES0. This bit is set to 1 when MEMRES0 is loaded with a new conversion result. Reading MEMRES0 register will clear this bit, or when the corresponding bit in ICLR is set to 1"]
    #[inline(always)]
    pub fn int_event1_mis_memresifg0(&self) -> IntEvent1MisMemresifg0R {
        IntEvent1MisMemresifg0R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1MisSpec;
impl crate::RegisterSpec for IntEvent1MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event1_mis::R`](R) reader structure"]
impl crate::Readable for IntEvent1MisSpec {}
#[doc = "`reset()` method sets INT_EVENT1_MIS to value 0"]
impl crate::Resettable for IntEvent1MisSpec {
    const RESET_VALUE: u32 = 0;
}
