#[doc = "Register `MCANERR_DED_STATUS` reader"]
pub type R = crate::R<McanerrDedStatusSpec>;
#[doc = "Register `MCANERR_DED_STATUS` writer"]
pub type W = crate::W<McanerrDedStatusSpec>;
#[doc = "Field `MCANERR_DED_STATUS_MSGMEM_PEND` reader - Message RAM DED Interrupt Pending 0 No DED interrupt is pending 1 DED interrupt is pending"]
pub type McanerrDedStatusMsgmemPendR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Message RAM DED Interrupt Pending 0 No DED interrupt is pending 1 DED interrupt is pending"]
    #[inline(always)]
    pub fn mcanerr_ded_status_msgmem_pend(&self) -> McanerrDedStatusMsgmemPendR {
        McanerrDedStatusMsgmemPendR::new((self.bits & 1) != 0)
    }
}
impl W {}
#[doc = "MCAN Double Error Detected Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_ded_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_ded_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanerrDedStatusSpec;
impl crate::RegisterSpec for McanerrDedStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanerr_ded_status::R`](R) reader structure"]
impl crate::Readable for McanerrDedStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanerr_ded_status::W`](W) writer structure"]
impl crate::Writable for McanerrDedStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANERR_DED_STATUS to value 0"]
impl crate::Resettable for McanerrDedStatusSpec {
    const RESET_VALUE: u32 = 0;
}
