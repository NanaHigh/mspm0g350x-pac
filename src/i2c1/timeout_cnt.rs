#[doc = "Register `TIMEOUT_CNT` reader"]
pub type R = crate::R<TimeoutCntSpec>;
#[doc = "Field `TIMEOUT_CNT_TCNTA` reader - Timeout Count A Current Count: This field contains the upper 8 bits of a 12-bit current counter for timeout counter A"]
pub type TimeoutCntTcntaR = crate::FieldReader;
#[doc = "Field `TIMEOUT_CNT_TCNTB` reader - Timeout Count B Current Count: This field contains the upper 8 bits of a 12-bit current counter for timeout counter B"]
pub type TimeoutCntTcntbR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Timeout Count A Current Count: This field contains the upper 8 bits of a 12-bit current counter for timeout counter A"]
    #[inline(always)]
    pub fn timeout_cnt_tcnta(&self) -> TimeoutCntTcntaR {
        TimeoutCntTcntaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Timeout Count B Current Count: This field contains the upper 8 bits of a 12-bit current counter for timeout counter B"]
    #[inline(always)]
    pub fn timeout_cnt_tcntb(&self) -> TimeoutCntTcntbR {
        TimeoutCntTcntbR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "I2C Timeout Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout_cnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeoutCntSpec;
impl crate::RegisterSpec for TimeoutCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout_cnt::R`](R) reader structure"]
impl crate::Readable for TimeoutCntSpec {}
#[doc = "`reset()` method sets TIMEOUT_CNT to value 0x0002_0002"]
impl crate::Resettable for TimeoutCntSpec {
    const RESET_VALUE: u32 = 0x0002_0002;
}
