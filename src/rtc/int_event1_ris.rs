#[doc = "Register `INT_EVENT1_RIS` reader"]
pub type R = crate::R<IntEvent1RisSpec>;
#[doc = "Raw RTC-Ready interrupts status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisRtcrdy {
    #[doc = "0: CLR"]
    IntEvent1RisRtcrdyClr = 0,
    #[doc = "1: SET"]
    IntEvent1RisRtcrdySet = 1,
}
impl From<IntEvent1RisRtcrdy> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisRtcrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_RTCRDY` reader - Raw RTC-Ready interrupts status"]
pub type IntEvent1RisRtcrdyR = crate::BitReader<IntEvent1RisRtcrdy>;
impl IntEvent1RisRtcrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisRtcrdy {
        match self.bits {
            false => IntEvent1RisRtcrdy::IntEvent1RisRtcrdyClr,
            true => IntEvent1RisRtcrdy::IntEvent1RisRtcrdySet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_rtcrdy_clr(&self) -> bool {
        *self == IntEvent1RisRtcrdy::IntEvent1RisRtcrdyClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_rtcrdy_set(&self) -> bool {
        *self == IntEvent1RisRtcrdy::IntEvent1RisRtcrdySet
    }
}
#[doc = "Raw Time-Event interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisRtctev {
    #[doc = "0: CLR"]
    IntEvent1RisRtctevClr = 0,
    #[doc = "1: SET"]
    IntEvent1RisRtctevSet = 1,
}
impl From<IntEvent1RisRtctev> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisRtctev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_RTCTEV` reader - Raw Time-Event interrupt status"]
pub type IntEvent1RisRtctevR = crate::BitReader<IntEvent1RisRtctev>;
impl IntEvent1RisRtctevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisRtctev {
        match self.bits {
            false => IntEvent1RisRtctev::IntEvent1RisRtctevClr,
            true => IntEvent1RisRtctev::IntEvent1RisRtctevSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_rtctev_clr(&self) -> bool {
        *self == IntEvent1RisRtctev::IntEvent1RisRtctevClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_rtctev_set(&self) -> bool {
        *self == IntEvent1RisRtctev::IntEvent1RisRtctevSet
    }
}
#[doc = "Raw Alarm-1 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisRtca1 {
    #[doc = "0: CLR"]
    IntEvent1RisRtca1Clr = 0,
    #[doc = "1: SET"]
    IntEvent1RisRtca1Set = 1,
}
impl From<IntEvent1RisRtca1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisRtca1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_RTCA1` reader - Raw Alarm-1 interrupt status"]
pub type IntEvent1RisRtca1R = crate::BitReader<IntEvent1RisRtca1>;
impl IntEvent1RisRtca1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisRtca1 {
        match self.bits {
            false => IntEvent1RisRtca1::IntEvent1RisRtca1Clr,
            true => IntEvent1RisRtca1::IntEvent1RisRtca1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_rtca1_clr(&self) -> bool {
        *self == IntEvent1RisRtca1::IntEvent1RisRtca1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_rtca1_set(&self) -> bool {
        *self == IntEvent1RisRtca1::IntEvent1RisRtca1Set
    }
}
#[doc = "Raw Alarm-2 interrupts status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisRtca2 {
    #[doc = "0: CLR"]
    IntEvent1RisRtca2Clr = 0,
    #[doc = "1: SET"]
    IntEvent1RisRtca2Set = 1,
}
impl From<IntEvent1RisRtca2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisRtca2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_RTCA2` reader - Raw Alarm-2 interrupts status"]
pub type IntEvent1RisRtca2R = crate::BitReader<IntEvent1RisRtca2>;
impl IntEvent1RisRtca2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisRtca2 {
        match self.bits {
            false => IntEvent1RisRtca2::IntEvent1RisRtca2Clr,
            true => IntEvent1RisRtca2::IntEvent1RisRtca2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_rtca2_clr(&self) -> bool {
        *self == IntEvent1RisRtca2::IntEvent1RisRtca2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_rtca2_set(&self) -> bool {
        *self == IntEvent1RisRtca2::IntEvent1RisRtca2Set
    }
}
#[doc = "Raw Prescaler-0 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisRt0ps {
    #[doc = "0: CLR"]
    IntEvent1RisRt0psClr = 0,
    #[doc = "1: SET"]
    IntEvent1RisRt0psSet = 1,
}
impl From<IntEvent1RisRt0ps> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisRt0ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_RT0PS` reader - Raw Prescaler-0 interrupt status"]
pub type IntEvent1RisRt0psR = crate::BitReader<IntEvent1RisRt0ps>;
impl IntEvent1RisRt0psR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisRt0ps {
        match self.bits {
            false => IntEvent1RisRt0ps::IntEvent1RisRt0psClr,
            true => IntEvent1RisRt0ps::IntEvent1RisRt0psSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_rt0ps_clr(&self) -> bool {
        *self == IntEvent1RisRt0ps::IntEvent1RisRt0psClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_rt0ps_set(&self) -> bool {
        *self == IntEvent1RisRt0ps::IntEvent1RisRt0psSet
    }
}
#[doc = "Raw Prescaler-1 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisRt1ps {
    #[doc = "0: CLR"]
    IntEvent1RisRt1psClr = 0,
    #[doc = "1: SET"]
    IntEvent1RisRt1psSet = 1,
}
impl From<IntEvent1RisRt1ps> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisRt1ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_RT1PS` reader - Raw Prescaler-1 interrupt status"]
pub type IntEvent1RisRt1psR = crate::BitReader<IntEvent1RisRt1ps>;
impl IntEvent1RisRt1psR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisRt1ps {
        match self.bits {
            false => IntEvent1RisRt1ps::IntEvent1RisRt1psClr,
            true => IntEvent1RisRt1ps::IntEvent1RisRt1psSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_rt1ps_clr(&self) -> bool {
        *self == IntEvent1RisRt1ps::IntEvent1RisRt1psClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_rt1ps_set(&self) -> bool {
        *self == IntEvent1RisRt1ps::IntEvent1RisRt1psSet
    }
}
impl R {
    #[doc = "Bit 0 - Raw RTC-Ready interrupts status"]
    #[inline(always)]
    pub fn int_event1_ris_rtcrdy(&self) -> IntEvent1RisRtcrdyR {
        IntEvent1RisRtcrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Raw Time-Event interrupt status"]
    #[inline(always)]
    pub fn int_event1_ris_rtctev(&self) -> IntEvent1RisRtctevR {
        IntEvent1RisRtctevR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw Alarm-1 interrupt status"]
    #[inline(always)]
    pub fn int_event1_ris_rtca1(&self) -> IntEvent1RisRtca1R {
        IntEvent1RisRtca1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Raw Alarm-2 interrupts status"]
    #[inline(always)]
    pub fn int_event1_ris_rtca2(&self) -> IntEvent1RisRtca2R {
        IntEvent1RisRtca2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Raw Prescaler-0 interrupt status"]
    #[inline(always)]
    pub fn int_event1_ris_rt0ps(&self) -> IntEvent1RisRt0psR {
        IntEvent1RisRt0psR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Raw Prescaler-1 interrupt status"]
    #[inline(always)]
    pub fn int_event1_ris_rt1ps(&self) -> IntEvent1RisRt1psR {
        IntEvent1RisRt1psR::new(((self.bits >> 5) & 1) != 0)
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
