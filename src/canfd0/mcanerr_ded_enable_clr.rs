#[doc = "Register `MCANERR_DED_ENABLE_CLR` reader"]
pub type R = crate::R<McanerrDedEnableClrSpec>;
#[doc = "Register `MCANERR_DED_ENABLE_CLR` writer"]
pub type W = crate::W<McanerrDedEnableClrSpec>;
#[doc = "Field `MCANERR_DED_ENABLE_CLR_MSGMEM_ENABLE_CLR` reader - Message RAM DED Interrupt Pending Enable Clear. Writing a 1 to this bit disables the Message RAM DED error interrupts. Writing a 0 has no effect. Reads return the corresponding enable bit's current value."]
pub type McanerrDedEnableClrMsgmemEnableClrR = crate::BitReader;
#[doc = "Field `MCANERR_DED_ENABLE_CLR_MSGMEM_ENABLE_CLR` writer - Message RAM DED Interrupt Pending Enable Clear. Writing a 1 to this bit disables the Message RAM DED error interrupts. Writing a 0 has no effect. Reads return the corresponding enable bit's current value."]
pub type McanerrDedEnableClrMsgmemEnableClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Message RAM DED Interrupt Pending Enable Clear. Writing a 1 to this bit disables the Message RAM DED error interrupts. Writing a 0 has no effect. Reads return the corresponding enable bit's current value."]
    #[inline(always)]
    pub fn mcanerr_ded_enable_clr_msgmem_enable_clr(&self) -> McanerrDedEnableClrMsgmemEnableClrR {
        McanerrDedEnableClrMsgmemEnableClrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Message RAM DED Interrupt Pending Enable Clear. Writing a 1 to this bit disables the Message RAM DED error interrupts. Writing a 0 has no effect. Reads return the corresponding enable bit's current value."]
    #[inline(always)]
    pub fn mcanerr_ded_enable_clr_msgmem_enable_clr(
        &mut self,
    ) -> McanerrDedEnableClrMsgmemEnableClrW<McanerrDedEnableClrSpec> {
        McanerrDedEnableClrMsgmemEnableClrW::new(self, 0)
    }
}
#[doc = "MCAN Double Error Detected Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_ded_enable_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_ded_enable_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanerrDedEnableClrSpec;
impl crate::RegisterSpec for McanerrDedEnableClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanerr_ded_enable_clr::R`](R) reader structure"]
impl crate::Readable for McanerrDedEnableClrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanerr_ded_enable_clr::W`](W) writer structure"]
impl crate::Writable for McanerrDedEnableClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANERR_DED_ENABLE_CLR to value 0"]
impl crate::Resettable for McanerrDedEnableClrSpec {
    const RESET_VALUE: u32 = 0;
}
