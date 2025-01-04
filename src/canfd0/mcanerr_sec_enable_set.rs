#[doc = "Register `MCANERR_SEC_ENABLE_SET` reader"]
pub type R = crate::R<McanerrSecEnableSetSpec>;
#[doc = "Register `MCANERR_SEC_ENABLE_SET` writer"]
pub type W = crate::W<McanerrSecEnableSetSpec>;
#[doc = "Field `MCANERR_SEC_ENABLE_SET_MSGMEM_ENABLE_SET` reader - Message RAM SEC Interrupt Pending Enable Set. Writing a 1 to this bit enables the Message RAM SEC error interrupts. Writing a 0 has no effect. Reads return the corresponding enable bit's current value."]
pub type McanerrSecEnableSetMsgmemEnableSetR = crate::BitReader;
#[doc = "Field `MCANERR_SEC_ENABLE_SET_MSGMEM_ENABLE_SET` writer - Message RAM SEC Interrupt Pending Enable Set. Writing a 1 to this bit enables the Message RAM SEC error interrupts. Writing a 0 has no effect. Reads return the corresponding enable bit's current value."]
pub type McanerrSecEnableSetMsgmemEnableSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Message RAM SEC Interrupt Pending Enable Set. Writing a 1 to this bit enables the Message RAM SEC error interrupts. Writing a 0 has no effect. Reads return the corresponding enable bit's current value."]
    #[inline(always)]
    pub fn mcanerr_sec_enable_set_msgmem_enable_set(&self) -> McanerrSecEnableSetMsgmemEnableSetR {
        McanerrSecEnableSetMsgmemEnableSetR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Message RAM SEC Interrupt Pending Enable Set. Writing a 1 to this bit enables the Message RAM SEC error interrupts. Writing a 0 has no effect. Reads return the corresponding enable bit's current value."]
    #[inline(always)]
    pub fn mcanerr_sec_enable_set_msgmem_enable_set(
        &mut self,
    ) -> McanerrSecEnableSetMsgmemEnableSetW<McanerrSecEnableSetSpec> {
        McanerrSecEnableSetMsgmemEnableSetW::new(self, 0)
    }
}
#[doc = "MCAN Single Error Corrected Interrupt Enable Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_sec_enable_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanerr_sec_enable_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanerrSecEnableSetSpec;
impl crate::RegisterSpec for McanerrSecEnableSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanerr_sec_enable_set::R`](R) reader structure"]
impl crate::Readable for McanerrSecEnableSetSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanerr_sec_enable_set::W`](W) writer structure"]
impl crate::Writable for McanerrSecEnableSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANERR_SEC_ENABLE_SET to value 0"]
impl crate::Resettable for McanerrSecEnableSetSpec {
    const RESET_VALUE: u32 = 0;
}
