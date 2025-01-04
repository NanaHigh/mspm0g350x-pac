#[doc = "Register `INT_EVENT1_IIDX` reader"]
pub type R = crate::R<IntEvent1IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum IntEvent1IidxStat {
    #[doc = "0: NO_INTR"]
    IntEvent1IidxStatNoIntr = 0,
    #[doc = "3: HIGHIFG"]
    IntEvent1IidxStatHighifg = 3,
    #[doc = "4: LOWIFG"]
    IntEvent1IidxStatLowifg = 4,
    #[doc = "5: INIFG"]
    IntEvent1IidxStatInifg = 5,
    #[doc = "9: MEMRESIFG0"]
    IntEvent1IidxStatMemresifg0 = 9,
}
impl From<IntEvent1IidxStat> for u16 {
    #[inline(always)]
    fn from(variant: IntEvent1IidxStat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntEvent1IidxStat {
    type Ux = u16;
}
impl crate::IsEnum for IntEvent1IidxStat {}
#[doc = "Field `INT_EVENT1_IIDX_STAT` reader - Interrupt index status"]
pub type IntEvent1IidxStatR = crate::FieldReader<IntEvent1IidxStat>;
impl IntEvent1IidxStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntEvent1IidxStat> {
        match self.bits {
            0 => Some(IntEvent1IidxStat::IntEvent1IidxStatNoIntr),
            3 => Some(IntEvent1IidxStat::IntEvent1IidxStatHighifg),
            4 => Some(IntEvent1IidxStat::IntEvent1IidxStatLowifg),
            5 => Some(IntEvent1IidxStat::IntEvent1IidxStatInifg),
            9 => Some(IntEvent1IidxStat::IntEvent1IidxStatMemresifg0),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_no_intr(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatNoIntr
    }
    #[doc = "HIGHIFG"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_highifg(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatHighifg
    }
    #[doc = "LOWIFG"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_lowifg(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatLowifg
    }
    #[doc = "INIFG"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_inifg(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatInifg
    }
    #[doc = "MEMRESIFG0"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_memresifg0(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatMemresifg0
    }
}
impl R {
    #[doc = "Bits 0:9 - Interrupt index status"]
    #[inline(always)]
    pub fn int_event1_iidx_stat(&self) -> IntEvent1IidxStatR {
        IntEvent1IidxStatR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1IidxSpec;
impl crate::RegisterSpec for IntEvent1IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event1_iidx::R`](R) reader structure"]
impl crate::Readable for IntEvent1IidxSpec {}
#[doc = "`reset()` method sets INT_EVENT1_IIDX to value 0"]
impl crate::Resettable for IntEvent1IidxSpec {
    const RESET_VALUE: u32 = 0;
}
