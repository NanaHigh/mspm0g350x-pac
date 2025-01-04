#[doc = "Register `MCANERR_SEC_STATUS` reader"]
pub type R = crate::R<McanerrSecStatusSpec>;
#[doc = "Register `MCANERR_SEC_STATUS` writer"]
pub type W = crate::W<McanerrSecStatusSpec>;
#[doc = "Field `MCANERR_SEC_STATUS_MSGMEM_PEND` reader - Message RAM SEC Interrupt Pending 0 No SEC interrupt is pending 1 SEC interrupt is pending"]
pub type McanerrSecStatusMsgmemPendR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Message RAM SEC Interrupt Pending 0 No SEC interrupt is pending 1 SEC interrupt is pending"]
    #[inline(always)]
    pub fn mcanerr_sec_status_msgmem_pend(&self) -> McanerrSecStatusMsgmemPendR {
        McanerrSecStatusMsgmemPendR::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "MCAN Single Error Corrected Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_sec_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_sec_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanerrSecStatusSpec;
impl crate::RegisterSpec for McanerrSecStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanerr_sec_status::R`](R) reader structure"]
impl crate::Readable for McanerrSecStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanerr_sec_status::W`](W) writer structure"]
impl crate::Writable for McanerrSecStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANERR_SEC_STATUS to value 0"]
impl crate::Resettable for McanerrSecStatusSpec {
    const RESET_VALUE: u32 = 0;
}
