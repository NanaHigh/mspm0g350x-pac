#[doc = "Register `INT_EVENT0_IIDX` reader"]
pub type R = crate::R<IntEvent0IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntEvent0IidxStat {
    #[doc = "0: NO_INTR"]
    IntEvent0IidxStatNoIntr = 0,
    #[doc = "1: RTCRDY"]
    IntEvent0IidxStatRtcrdy = 1,
    #[doc = "2: RTCTEV"]
    IntEvent0IidxStatRtctev = 2,
    #[doc = "3: RTCA1"]
    IntEvent0IidxStatRtca1 = 3,
    #[doc = "4: RTCA2"]
    IntEvent0IidxStatRtca2 = 4,
    #[doc = "5: RT0PS"]
    IntEvent0IidxStatRt0ps = 5,
    #[doc = "6: RT1PS"]
    IntEvent0IidxStatRt1ps = 6,
}
impl From<IntEvent0IidxStat> for u8 {
    #[inline(always)]
    fn from(variant: IntEvent0IidxStat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntEvent0IidxStat {
    type Ux = u8;
}
impl crate::IsEnum for IntEvent0IidxStat {}
#[doc = "Field `INT_EVENT0_IIDX_STAT` reader - Interrupt index status"]
pub type IntEvent0IidxStatR = crate::FieldReader<IntEvent0IidxStat>;
impl IntEvent0IidxStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntEvent0IidxStat> {
        match self.bits {
            0 => Some(IntEvent0IidxStat::IntEvent0IidxStatNoIntr),
            1 => Some(IntEvent0IidxStat::IntEvent0IidxStatRtcrdy),
            2 => Some(IntEvent0IidxStat::IntEvent0IidxStatRtctev),
            3 => Some(IntEvent0IidxStat::IntEvent0IidxStatRtca1),
            4 => Some(IntEvent0IidxStat::IntEvent0IidxStatRtca2),
            5 => Some(IntEvent0IidxStat::IntEvent0IidxStatRt0ps),
            6 => Some(IntEvent0IidxStat::IntEvent0IidxStatRt1ps),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_no_intr(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatNoIntr
    }
    #[doc = "RTCRDY"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_rtcrdy(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatRtcrdy
    }
    #[doc = "RTCTEV"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_rtctev(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatRtctev
    }
    #[doc = "RTCA1"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_rtca1(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatRtca1
    }
    #[doc = "RTCA2"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_rtca2(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatRtca2
    }
    #[doc = "RT0PS"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_rt0ps(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatRt0ps
    }
    #[doc = "RT1PS"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_rt1ps(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatRt1ps
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn int_event0_iidx_stat(&self) -> IntEvent0IidxStatR {
        IntEvent0IidxStatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0IidxSpec;
impl crate::RegisterSpec for IntEvent0IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event0_iidx::R`](R) reader structure"]
impl crate::Readable for IntEvent0IidxSpec {}
#[doc = "`reset()` method sets INT_EVENT0_IIDX to value 0"]
impl crate::Resettable for IntEvent0IidxSpec {
    const RESET_VALUE: u32 = 0;
}
