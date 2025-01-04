#[doc = "Register `INT_EVENT0_MIS` reader"]
pub type R = crate::R<IntEvent0MisSpec>;
#[doc = "Masked RTC-Ready interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisRtcrdy {
    #[doc = "0: CLR"]
    IntEvent0MisRtcrdyClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisRtcrdySet = 1,
}
impl From<IntEvent0MisRtcrdy> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisRtcrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_RTCRDY` reader - Masked RTC-Ready interrupt status"]
pub type IntEvent0MisRtcrdyR = crate::BitReader<IntEvent0MisRtcrdy>;
impl IntEvent0MisRtcrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisRtcrdy {
        match self.bits {
            false => IntEvent0MisRtcrdy::IntEvent0MisRtcrdyClr,
            true => IntEvent0MisRtcrdy::IntEvent0MisRtcrdySet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_rtcrdy_clr(&self) -> bool {
        *self == IntEvent0MisRtcrdy::IntEvent0MisRtcrdyClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_rtcrdy_set(&self) -> bool {
        *self == IntEvent0MisRtcrdy::IntEvent0MisRtcrdySet
    }
}
#[doc = "Masked Time-Event interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisRtctev {
    #[doc = "0: CLR"]
    IntEvent0MisRtctevClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisRtctevSet = 1,
}
impl From<IntEvent0MisRtctev> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisRtctev) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_RTCTEV` reader - Masked Time-Event interrupt status"]
pub type IntEvent0MisRtctevR = crate::BitReader<IntEvent0MisRtctev>;
impl IntEvent0MisRtctevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisRtctev {
        match self.bits {
            false => IntEvent0MisRtctev::IntEvent0MisRtctevClr,
            true => IntEvent0MisRtctev::IntEvent0MisRtctevSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_rtctev_clr(&self) -> bool {
        *self == IntEvent0MisRtctev::IntEvent0MisRtctevClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_rtctev_set(&self) -> bool {
        *self == IntEvent0MisRtctev::IntEvent0MisRtctevSet
    }
}
#[doc = "Masked Alarm-1 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisRtca1 {
    #[doc = "0: CLR"]
    IntEvent0MisRtca1Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisRtca1Set = 1,
}
impl From<IntEvent0MisRtca1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisRtca1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_RTCA1` reader - Masked Alarm-1 interrupt status"]
pub type IntEvent0MisRtca1R = crate::BitReader<IntEvent0MisRtca1>;
impl IntEvent0MisRtca1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisRtca1 {
        match self.bits {
            false => IntEvent0MisRtca1::IntEvent0MisRtca1Clr,
            true => IntEvent0MisRtca1::IntEvent0MisRtca1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_rtca1_clr(&self) -> bool {
        *self == IntEvent0MisRtca1::IntEvent0MisRtca1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_rtca1_set(&self) -> bool {
        *self == IntEvent0MisRtca1::IntEvent0MisRtca1Set
    }
}
#[doc = "Masked Alarm-2 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisRtca2 {
    #[doc = "0: CLR"]
    IntEvent0MisRtca2Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisRtca2Set = 1,
}
impl From<IntEvent0MisRtca2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisRtca2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_RTCA2` reader - Masked Alarm-2 interrupt status"]
pub type IntEvent0MisRtca2R = crate::BitReader<IntEvent0MisRtca2>;
impl IntEvent0MisRtca2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisRtca2 {
        match self.bits {
            false => IntEvent0MisRtca2::IntEvent0MisRtca2Clr,
            true => IntEvent0MisRtca2::IntEvent0MisRtca2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_rtca2_clr(&self) -> bool {
        *self == IntEvent0MisRtca2::IntEvent0MisRtca2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_rtca2_set(&self) -> bool {
        *self == IntEvent0MisRtca2::IntEvent0MisRtca2Set
    }
}
#[doc = "Masked Prescaler-0 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisRt0ps {
    #[doc = "0: CLR"]
    IntEvent0MisRt0psClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisRt0psSet = 1,
}
impl From<IntEvent0MisRt0ps> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisRt0ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_RT0PS` reader - Masked Prescaler-0 interrupt status"]
pub type IntEvent0MisRt0psR = crate::BitReader<IntEvent0MisRt0ps>;
impl IntEvent0MisRt0psR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisRt0ps {
        match self.bits {
            false => IntEvent0MisRt0ps::IntEvent0MisRt0psClr,
            true => IntEvent0MisRt0ps::IntEvent0MisRt0psSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_rt0ps_clr(&self) -> bool {
        *self == IntEvent0MisRt0ps::IntEvent0MisRt0psClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_rt0ps_set(&self) -> bool {
        *self == IntEvent0MisRt0ps::IntEvent0MisRt0psSet
    }
}
#[doc = "Masked Prescaler-1 interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisRt1ps {
    #[doc = "0: CLR"]
    IntEvent0MisRt1psClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisRt1psSet = 1,
}
impl From<IntEvent0MisRt1ps> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisRt1ps) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_RT1PS` reader - Masked Prescaler-1 interrupt status"]
pub type IntEvent0MisRt1psR = crate::BitReader<IntEvent0MisRt1ps>;
impl IntEvent0MisRt1psR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisRt1ps {
        match self.bits {
            false => IntEvent0MisRt1ps::IntEvent0MisRt1psClr,
            true => IntEvent0MisRt1ps::IntEvent0MisRt1psSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_rt1ps_clr(&self) -> bool {
        *self == IntEvent0MisRt1ps::IntEvent0MisRt1psClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_rt1ps_set(&self) -> bool {
        *self == IntEvent0MisRt1ps::IntEvent0MisRt1psSet
    }
}
impl R {
    #[doc = "Bit 0 - Masked RTC-Ready interrupt status"]
    #[inline(always)]
    pub fn int_event0_mis_rtcrdy(&self) -> IntEvent0MisRtcrdyR {
        IntEvent0MisRtcrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masked Time-Event interrupt status"]
    #[inline(always)]
    pub fn int_event0_mis_rtctev(&self) -> IntEvent0MisRtctevR {
        IntEvent0MisRtctevR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masked Alarm-1 interrupt status"]
    #[inline(always)]
    pub fn int_event0_mis_rtca1(&self) -> IntEvent0MisRtca1R {
        IntEvent0MisRtca1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masked Alarm-2 interrupt status"]
    #[inline(always)]
    pub fn int_event0_mis_rtca2(&self) -> IntEvent0MisRtca2R {
        IntEvent0MisRtca2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Masked Prescaler-0 interrupt status"]
    #[inline(always)]
    pub fn int_event0_mis_rt0ps(&self) -> IntEvent0MisRt0psR {
        IntEvent0MisRt0psR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Masked Prescaler-1 interrupt status"]
    #[inline(always)]
    pub fn int_event0_mis_rt1ps(&self) -> IntEvent0MisRt1psR {
        IntEvent0MisRt1psR::new(((self.bits >> 5) & 1) != 0)
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
