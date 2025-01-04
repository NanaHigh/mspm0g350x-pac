#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "This bit reflects the value of the comparator output. Writing to this bit has no effect on the comparator output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatOut {
    #[doc = "0: LOW"]
    StatOutLow = 0,
    #[doc = "1: HIGH"]
    StatOutHigh = 1,
}
impl From<StatOut> for bool {
    #[inline(always)]
    fn from(variant: StatOut) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAT_OUT` reader - This bit reflects the value of the comparator output. Writing to this bit has no effect on the comparator output."]
pub type StatOutR = crate::BitReader<StatOut>;
impl StatOutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StatOut {
        match self.bits {
            false => StatOut::StatOutLow,
            true => StatOut::StatOutHigh,
        }
    }
    #[doc = "LOW"]
    #[inline(always)]
    pub fn is_stat_out_low(&self) -> bool {
        *self == StatOut::StatOutLow
    }
    #[doc = "HIGH"]
    #[inline(always)]
    pub fn is_stat_out_high(&self) -> bool {
        *self == StatOut::StatOutHigh
    }
}
impl R {
    #[doc = "Bit 0 - This bit reflects the value of the comparator output. Writing to this bit has no effect on the comparator output."]
    #[inline(always)]
    pub fn stat_out(&self) -> StatOutR {
        StatOutR::new((self.bits & 1) != 0)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
