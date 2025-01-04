#[doc = "Register `INT_EVENT1_MIS` reader"]
pub type R = crate::R<IntEvent1MisSpec>;
#[doc = "Masked RTC-Ready interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisRtcrdy {
    #[doc = "0: CLR"]
    IntEvent1MisRtcrdyClr = 0,
    #[doc = "1: SET"]
    IntEvent1MisRtcrdySet = 1,
}
impl From<IntEvent1MisRtcrdy> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisRtcrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_RTCRDY` reader - Masked RTC-Ready interrupt status"]
pub type IntEvent1MisRtcrdyR = crate::BitReader<IntEvent1MisRtcrdy>;
impl IntEvent1MisRtcrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisRtcrdy {
        match self.bits {
            false => IntEvent1MisRtcrdy::IntEvent1MisRtcrdyClr,
            true => IntEvent1MisRtcrdy::IntEvent1MisRtcrdySet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_rtcrdy_clr(&self) -> bool {
        *self == IntEvent1MisRtcrdy::IntEvent1MisRtcrdyClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_rtcrdy_set(&self) -> bool {
        *self == IntEvent1MisRtcrdy::IntEvent1MisRtcrdySet
    }
}
#[doc = "Masked Time-Event interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisRtctev {
    #[doc = "0: CLR"]
    IntEvent1MisRtctevClr = 0,
    #[doc = "1: SET"]
    IntEvent1MisRtctevSet = 1,
}
impl From<IntEvent1MisRtctev> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisRtctev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_RTCTEV` reader - Masked Time-Event interrupt status"]
pub type IntEvent1MisRtctevR = crate::BitReader<IntEvent1MisRtctev>;
impl IntEvent1MisRtctevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisRtctev {
        match self.bits {
            false => IntEvent1MisRtctev::IntEvent1MisRtctevClr,
            true => IntEvent1MisRtctev::IntEvent1MisRtctevSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_rtctev_clr(&self) -> bool {
        *self == IntEvent1MisRtctev::IntEvent1MisRtctevClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_rtctev_set(&self) -> bool {
        *self == IntEvent1MisRtctev::IntEvent1MisRtctevSet
    }
}
#[doc = "Masked Alarm-1 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisRtca1 {
    #[doc = "0: CLR"]
    IntEvent1MisRtca1Clr = 0,
    #[doc = "1: SET"]
    IntEvent1MisRtca1Set = 1,
}
impl From<IntEvent1MisRtca1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisRtca1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_RTCA1` reader - Masked Alarm-1 interrupt status"]
pub type IntEvent1MisRtca1R = crate::BitReader<IntEvent1MisRtca1>;
impl IntEvent1MisRtca1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisRtca1 {
        match self.bits {
            false => IntEvent1MisRtca1::IntEvent1MisRtca1Clr,
            true => IntEvent1MisRtca1::IntEvent1MisRtca1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_rtca1_clr(&self) -> bool {
        *self == IntEvent1MisRtca1::IntEvent1MisRtca1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_rtca1_set(&self) -> bool {
        *self == IntEvent1MisRtca1::IntEvent1MisRtca1Set
    }
}
#[doc = "Masked Alarm-2 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisRtca2 {
    #[doc = "0: CLR"]
    IntEvent1MisRtca2Clr = 0,
    #[doc = "1: SET"]
    IntEvent1MisRtca2Set = 1,
}
impl From<IntEvent1MisRtca2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisRtca2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_RTCA2` reader - Masked Alarm-2 interrupt status"]
pub type IntEvent1MisRtca2R = crate::BitReader<IntEvent1MisRtca2>;
impl IntEvent1MisRtca2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisRtca2 {
        match self.bits {
            false => IntEvent1MisRtca2::IntEvent1MisRtca2Clr,
            true => IntEvent1MisRtca2::IntEvent1MisRtca2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_rtca2_clr(&self) -> bool {
        *self == IntEvent1MisRtca2::IntEvent1MisRtca2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_rtca2_set(&self) -> bool {
        *self == IntEvent1MisRtca2::IntEvent1MisRtca2Set
    }
}
#[doc = "Masked Prescaler-0 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisRt0ps {
    #[doc = "0: CLR"]
    IntEvent1MisRt0psClr = 0,
    #[doc = "1: SET"]
    IntEvent1MisRt0psSet = 1,
}
impl From<IntEvent1MisRt0ps> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisRt0ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_RT0PS` reader - Masked Prescaler-0 interrupt status"]
pub type IntEvent1MisRt0psR = crate::BitReader<IntEvent1MisRt0ps>;
impl IntEvent1MisRt0psR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisRt0ps {
        match self.bits {
            false => IntEvent1MisRt0ps::IntEvent1MisRt0psClr,
            true => IntEvent1MisRt0ps::IntEvent1MisRt0psSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_rt0ps_clr(&self) -> bool {
        *self == IntEvent1MisRt0ps::IntEvent1MisRt0psClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_rt0ps_set(&self) -> bool {
        *self == IntEvent1MisRt0ps::IntEvent1MisRt0psSet
    }
}
#[doc = "Masked Prescaler-1 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisRt1ps {
    #[doc = "0: CLR"]
    IntEvent1MisRt1psClr = 0,
    #[doc = "1: SET"]
    IntEvent1MisRt1psSet = 1,
}
impl From<IntEvent1MisRt1ps> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisRt1ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_RT1PS` reader - Masked Prescaler-1 interrupt status"]
pub type IntEvent1MisRt1psR = crate::BitReader<IntEvent1MisRt1ps>;
impl IntEvent1MisRt1psR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisRt1ps {
        match self.bits {
            false => IntEvent1MisRt1ps::IntEvent1MisRt1psClr,
            true => IntEvent1MisRt1ps::IntEvent1MisRt1psSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_rt1ps_clr(&self) -> bool {
        *self == IntEvent1MisRt1ps::IntEvent1MisRt1psClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_rt1ps_set(&self) -> bool {
        *self == IntEvent1MisRt1ps::IntEvent1MisRt1psSet
    }
}
impl R {
    #[doc = "Bit 0 - Masked RTC-Ready interrupt status"]
    #[inline(always)]
    pub fn int_event1_mis_rtcrdy(&self) -> IntEvent1MisRtcrdyR {
        IntEvent1MisRtcrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masked Time-Event interrupt status"]
    #[inline(always)]
    pub fn int_event1_mis_rtctev(&self) -> IntEvent1MisRtctevR {
        IntEvent1MisRtctevR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masked Alarm-1 interrupt status"]
    #[inline(always)]
    pub fn int_event1_mis_rtca1(&self) -> IntEvent1MisRtca1R {
        IntEvent1MisRtca1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masked Alarm-2 interrupt status"]
    #[inline(always)]
    pub fn int_event1_mis_rtca2(&self) -> IntEvent1MisRtca2R {
        IntEvent1MisRtca2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Masked Prescaler-0 interrupt status"]
    #[inline(always)]
    pub fn int_event1_mis_rt0ps(&self) -> IntEvent1MisRt0psR {
        IntEvent1MisRt0psR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Masked Prescaler-1 interrupt status"]
    #[inline(always)]
    pub fn int_event1_mis_rt1ps(&self) -> IntEvent1MisRt1psR {
        IntEvent1MisRt1psR::new(((self.bits >> 5) & 1) != 0)
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
