#[doc = "Register `MCANERR_SEC_ENABLE_CLR` reader"]
pub type R = crate::R<McanerrSecEnableClrSpec>;
#[doc = "Register `MCANERR_SEC_ENABLE_CLR` writer"]
pub type W = crate::W<McanerrSecEnableClrSpec>;
#[doc = "Field `MCANERR_SEC_ENABLE_CLR_MSGMEM_ENABLE_CLR` reader - Message RAM SEC Interrupt Pending Enable Clear. Writing a 1 to this bit disables the Message RAM SEC error interrupts. Writing a 0 has no effect. Reads return the corresponding enable bit's current value."]
pub type McanerrSecEnableClrMsgmemEnableClrR = crate::BitReader;
#[doc = "Field `MCANERR_SEC_ENABLE_CLR_MSGMEM_ENABLE_CLR` writer - Message RAM SEC Interrupt Pending Enable Clear. Writing a 1 to this bit disables the Message RAM SEC error interrupts. Writing a 0 has no effect. Reads return the corresponding enable bit's current value."]
pub type McanerrSecEnableClrMsgmemEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Message RAM SEC Interrupt Pending Enable Clear. Writing a 1 to this bit disables the Message RAM SEC error interrupts. Writing a 0 has no effect. Reads return the corresponding enable bit's current value."]
    #[inline(always)]
    pub fn mcanerr_sec_enable_clr_msgmem_enable_clr(&self) -> McanerrSecEnableClrMsgmemEnableClrR {
        McanerrSecEnableClrMsgmemEnableClrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Message RAM SEC Interrupt Pending Enable Clear. Writing a 1 to this bit disables the Message RAM SEC error interrupts. Writing a 0 has no effect. Reads return the corresponding enable bit's current value."]
    #[inline(always)]
    pub fn mcanerr_sec_enable_clr_msgmem_enable_clr(
        &mut self,
    ) -> McanerrSecEnableClrMsgmemEnableClrW<McanerrSecEnableClrSpec> {
        McanerrSecEnableClrMsgmemEnableClrW::new(self, 0)
    }
}
#[doc = "MCAN Single Error Corrected Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_sec_enable_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_sec_enable_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanerrSecEnableClrSpec;
impl crate::RegisterSpec for McanerrSecEnableClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanerr_sec_enable_clr::R`](R) reader structure"]
impl crate::Readable for McanerrSecEnableClrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanerr_sec_enable_clr::W`](W) writer structure"]
impl crate::Writable for McanerrSecEnableClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANERR_SEC_ENABLE_CLR to value 0"]
impl crate::Resettable for McanerrSecEnableClrSpec {
    const RESET_VALUE: u32 = 0;
}
