#[doc = "Register `INT_EVENT0_RIS` reader"]
pub type R = crate::R<IntEvent0RisSpec>;
#[doc = "Raw RTC-Ready interrupts status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisRtcrdy {
    #[doc = "0: CLR"]
    IntEvent0RisRtcrdyClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisRtcrdySet = 1,
}
impl From<IntEvent0RisRtcrdy> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisRtcrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_RTCRDY` reader - Raw RTC-Ready interrupts status"]
pub type IntEvent0RisRtcrdyR = crate::BitReader<IntEvent0RisRtcrdy>;
impl IntEvent0RisRtcrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisRtcrdy {
        match self.bits {
            false => IntEvent0RisRtcrdy::IntEvent0RisRtcrdyClr,
            true => IntEvent0RisRtcrdy::IntEvent0RisRtcrdySet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_rtcrdy_clr(&self) -> bool {
        *self == IntEvent0RisRtcrdy::IntEvent0RisRtcrdyClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_rtcrdy_set(&self) -> bool {
        *self == IntEvent0RisRtcrdy::IntEvent0RisRtcrdySet
    }
}
#[doc = "Raw Time-Event interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisRtctev {
    #[doc = "0: CLR"]
    IntEvent0RisRtctevClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisRtctevSet = 1,
}
impl From<IntEvent0RisRtctev> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisRtctev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_RTCTEV` reader - Raw Time-Event interrupt status"]
pub type IntEvent0RisRtctevR = crate::BitReader<IntEvent0RisRtctev>;
impl IntEvent0RisRtctevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisRtctev {
        match self.bits {
            false => IntEvent0RisRtctev::IntEvent0RisRtctevClr,
            true => IntEvent0RisRtctev::IntEvent0RisRtctevSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_rtctev_clr(&self) -> bool {
        *self == IntEvent0RisRtctev::IntEvent0RisRtctevClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_rtctev_set(&self) -> bool {
        *self == IntEvent0RisRtctev::IntEvent0RisRtctevSet
    }
}
#[doc = "Raw Alarm-1 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisRtca1 {
    #[doc = "0: CLR"]
    IntEvent0RisRtca1Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisRtca1Set = 1,
}
impl From<IntEvent0RisRtca1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisRtca1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_RTCA1` reader - Raw Alarm-1 interrupt status"]
pub type IntEvent0RisRtca1R = crate::BitReader<IntEvent0RisRtca1>;
impl IntEvent0RisRtca1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisRtca1 {
        match self.bits {
            false => IntEvent0RisRtca1::IntEvent0RisRtca1Clr,
            true => IntEvent0RisRtca1::IntEvent0RisRtca1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_rtca1_clr(&self) -> bool {
        *self == IntEvent0RisRtca1::IntEvent0RisRtca1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_rtca1_set(&self) -> bool {
        *self == IntEvent0RisRtca1::IntEvent0RisRtca1Set
    }
}
#[doc = "Raw Alarm-2 interrupts status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisRtca2 {
    #[doc = "0: CLR"]
    IntEvent0RisRtca2Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisRtca2Set = 1,
}
impl From<IntEvent0RisRtca2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisRtca2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_RTCA2` reader - Raw Alarm-2 interrupts status"]
pub type IntEvent0RisRtca2R = crate::BitReader<IntEvent0RisRtca2>;
impl IntEvent0RisRtca2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisRtca2 {
        match self.bits {
            false => IntEvent0RisRtca2::IntEvent0RisRtca2Clr,
            true => IntEvent0RisRtca2::IntEvent0RisRtca2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_rtca2_clr(&self) -> bool {
        *self == IntEvent0RisRtca2::IntEvent0RisRtca2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_rtca2_set(&self) -> bool {
        *self == IntEvent0RisRtca2::IntEvent0RisRtca2Set
    }
}
#[doc = "Raw Prescaler-0 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisRt0ps {
    #[doc = "0: CLR"]
    IntEvent0RisRt0psClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisRt0psSet = 1,
}
impl From<IntEvent0RisRt0ps> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisRt0ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_RT0PS` reader - Raw Prescaler-0 interrupt status"]
pub type IntEvent0RisRt0psR = crate::BitReader<IntEvent0RisRt0ps>;
impl IntEvent0RisRt0psR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisRt0ps {
        match self.bits {
            false => IntEvent0RisRt0ps::IntEvent0RisRt0psClr,
            true => IntEvent0RisRt0ps::IntEvent0RisRt0psSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_rt0ps_clr(&self) -> bool {
        *self == IntEvent0RisRt0ps::IntEvent0RisRt0psClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_rt0ps_set(&self) -> bool {
        *self == IntEvent0RisRt0ps::IntEvent0RisRt0psSet
    }
}
#[doc = "Raw Prescaler-1 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisRt1ps {
    #[doc = "0: CLR"]
    IntEvent0RisRt1psClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisRt1psSet = 1,
}
impl From<IntEvent0RisRt1ps> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisRt1ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_RT1PS` reader - Raw Prescaler-1 interrupt status"]
pub type IntEvent0RisRt1psR = crate::BitReader<IntEvent0RisRt1ps>;
impl IntEvent0RisRt1psR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisRt1ps {
        match self.bits {
            false => IntEvent0RisRt1ps::IntEvent0RisRt1psClr,
            true => IntEvent0RisRt1ps::IntEvent0RisRt1psSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_rt1ps_clr(&self) -> bool {
        *self == IntEvent0RisRt1ps::IntEvent0RisRt1psClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_rt1ps_set(&self) -> bool {
        *self == IntEvent0RisRt1ps::IntEvent0RisRt1psSet
    }
}
impl R {
    #[doc = "Bit 0 - Raw RTC-Ready interrupts status"]
    #[inline(always)]
    pub fn int_event0_ris_rtcrdy(&self) -> IntEvent0RisRtcrdyR {
        IntEvent0RisRtcrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw Time-Event interrupt status"]
    #[inline(always)]
    pub fn int_event0_ris_rtctev(&self) -> IntEvent0RisRtctevR {
        IntEvent0RisRtctevR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw Alarm-1 interrupt status"]
    #[inline(always)]
    pub fn int_event0_ris_rtca1(&self) -> IntEvent0RisRtca1R {
        IntEvent0RisRtca1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw Alarm-2 interrupts status"]
    #[inline(always)]
    pub fn int_event0_ris_rtca2(&self) -> IntEvent0RisRtca2R {
        IntEvent0RisRtca2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Raw Prescaler-0 interrupt status"]
    #[inline(always)]
    pub fn int_event0_ris_rt0ps(&self) -> IntEvent0RisRt0psR {
        IntEvent0RisRt0psR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Raw Prescaler-1 interrupt status"]
    #[inline(always)]
    pub fn int_event0_ris_rt1ps(&self) -> IntEvent0RisRt1psR {
        IntEvent0RisRt1psR::new(((self.bits >> 5) & 1) != 0)
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
