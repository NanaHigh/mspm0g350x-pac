#[doc = "Register `INT_EVENT1_IIDX` reader"]
pub type R = crate::R<IntEvent1IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntEvent1IidxStat {
    #[doc = "0: NO_INTR"]
    IntEvent1IidxStatNoIntr = 0,
    #[doc = "1: RTCRDY"]
    IntEvent1IidxStatRtcrdy = 1,
    #[doc = "2: RTCTEV"]
    IntEvent1IidxStatRtctev = 2,
    #[doc = "3: RTCA1"]
    IntEvent1IidxStatRtca1 = 3,
    #[doc = "4: RTCA2"]
    IntEvent1IidxStatRtca2 = 4,
    #[doc = "5: RT0PS"]
    IntEvent1IidxStatRt0ps = 5,
    #[doc = "6: RT1PS"]
    IntEvent1IidxStatRt1ps = 6,
}
impl From<IntEvent1IidxStat> for u8 {
    #[inline(always)]
    fn from(variant: IntEvent1IidxStat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntEvent1IidxStat {
    type Ux = u8;
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
            1 => Some(IntEvent1IidxStat::IntEvent1IidxStatRtcrdy),
            2 => Some(IntEvent1IidxStat::IntEvent1IidxStatRtctev),
            3 => Some(IntEvent1IidxStat::IntEvent1IidxStatRtca1),
            4 => Some(IntEvent1IidxStat::IntEvent1IidxStatRtca2),
            5 => Some(IntEvent1IidxStat::IntEvent1IidxStatRt0ps),
            6 => Some(IntEvent1IidxStat::IntEvent1IidxStatRt1ps),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_no_intr(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatNoIntr
    }
    #[doc = "RTCRDY"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_rtcrdy(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatRtcrdy
    }
    #[doc = "RTCTEV"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_rtctev(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatRtctev
    }
    #[doc = "RTCA1"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_rtca1(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatRtca1
    }
    #[doc = "RTCA2"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_rtca2(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatRtca2
    }
    #[doc = "RT0PS"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_rt0ps(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatRt0ps
    }
    #[doc = "RT1PS"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_rt1ps(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatRt1ps
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn int_event1_iidx_stat(&self) -> IntEvent1IidxStatR {
        IntEvent1IidxStatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
