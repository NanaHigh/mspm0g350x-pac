#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "OA ready status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatRdy {
    #[doc = "0: FALSE"]
    StatRdyFalse = 0,
    #[doc = "1: TRUE"]
    StatRdyTrue = 1,
}
impl From<StatRdy> for bool {
    #[inline(always)]
    fn from(variant: StatRdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAT_RDY` reader - OA ready status."]
pub type StatRdyR = crate::BitReader<StatRdy>;
impl StatRdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StatRdy {
        match self.bits {
            false => StatRdy::StatRdyFalse,
            true => StatRdy::StatRdyTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_stat_rdy_false(&self) -> bool {
        *self == StatRdy::StatRdyFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_stat_rdy_true(&self) -> bool {
        *self == StatRdy::StatRdyTrue
    }
}
impl R {
    #[doc = "Bit 0 - OA ready status."]
    #[inline(always)]
    pub fn stat_rdy(&self) -> StatRdyR {
        StatRdyR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
