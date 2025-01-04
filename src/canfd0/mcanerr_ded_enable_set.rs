#[doc = "Register `MCANERR_DED_ENABLE_SET` reader"]
pub type R = crate::R<McanerrDedEnableSetSpec>;
#[doc = "Register `MCANERR_DED_ENABLE_SET` writer"]
pub type W = crate::W<McanerrDedEnableSetSpec>;
#[doc = "Field `MCANERR_DED_ENABLE_SET_MSGMEM_ENABLE_SET` reader - Message RAM DED Interrupt Pending Enable Set. Writing a 1 to this bit enables the Message RAM DED error interrupts. Writing a 0 has no effect. Reads return the corresponding enable bit's current value."]
pub type McanerrDedEnableSetMsgmemEnableSetR = crate::BitReader;
#[doc = "Field `MCANERR_DED_ENABLE_SET_MSGMEM_ENABLE_SET` writer - Message RAM DED Interrupt Pending Enable Set. Writing a 1 to this bit enables the Message RAM DED error interrupts. Writing a 0 has no effect. Reads return the corresponding enable bit's current value."]
pub type McanerrDedEnableSetMsgmemEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Message RAM DED Interrupt Pending Enable Set. Writing a 1 to this bit enables the Message RAM DED error interrupts. Writing a 0 has no effect. Reads return the corresponding enable bit's current value."]
    #[inline(always)]
    pub fn mcanerr_ded_enable_set_msgmem_enable_set(&self) -> McanerrDedEnableSetMsgmemEnableSetR {
        McanerrDedEnableSetMsgmemEnableSetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Message RAM DED Interrupt Pending Enable Set. Writing a 1 to this bit enables the Message RAM DED error interrupts. Writing a 0 has no effect. Reads return the corresponding enable bit's current value."]
    #[inline(always)]
    pub fn mcanerr_ded_enable_set_msgmem_enable_set(
        &mut self,
    ) -> McanerrDedEnableSetMsgmemEnableSetW<McanerrDedEnableSetSpec> {
        McanerrDedEnableSetMsgmemEnableSetW::new(self, 0)
    }
}
#[doc = "MCAN Double Error Detected Interrupt Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_ded_enable_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_ded_enable_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanerrDedEnableSetSpec;
impl crate::RegisterSpec for McanerrDedEnableSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanerr_ded_enable_set::R`](R) reader structure"]
impl crate::Readable for McanerrDedEnableSetSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanerr_ded_enable_set::W`](W) writer structure"]
impl crate::Writable for McanerrDedEnableSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANERR_DED_ENABLE_SET to value 0"]
impl crate::Resettable for McanerrDedEnableSetSpec {
    const RESET_VALUE: u32 = 0;
}
