#[doc = "Register `WWDTCNTRST` reader"]
pub type R = crate::R<WwdtcntrstSpec>;
#[doc = "Register `WWDTCNTRST` writer"]
pub type W = crate::W<WwdtcntrstSpec>;
#[doc = "Field `WWDTCNTRST_RESTART` reader - Window Watchdog Timer Counter Restart Writing 00A7h to this register restarts the WWDT Counter. Writing any other value causes an error generation to the ESM. Read as 0."]
pub type WwdtcntrstRestartR = crate::FieldReader<u32>;
#[doc = "Field `WWDTCNTRST_RESTART` writer - Window Watchdog Timer Counter Restart Writing 00A7h to this register restarts the WWDT Counter. Writing any other value causes an error generation to the ESM. Read as 0."]
pub type WwdtcntrstRestartW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Window Watchdog Timer Counter Restart Writing 00A7h to this register restarts the WWDT Counter. Writing any other value causes an error generation to the ESM. Read as 0."]
    #[inline(always)]
    pub fn wwdtcntrst_restart(&self) -> WwdtcntrstRestartR {
        WwdtcntrstRestartR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Window Watchdog Timer Counter Restart Writing 00A7h to this register restarts the WWDT Counter. Writing any other value causes an error generation to the ESM. Read as 0."]
    #[inline(always)]
    pub fn wwdtcntrst_restart(&mut self) -> WwdtcntrstRestartW<WwdtcntrstSpec> {
        WwdtcntrstRestartW::new(self, 0)
    }
}
#[doc = "Window Watchdog Timer Counter Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wwdtcntrst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wwdtcntrst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WwdtcntrstSpec;
impl crate::RegisterSpec for WwdtcntrstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wwdtcntrst::R`](R) reader structure"]
impl crate::Readable for WwdtcntrstSpec {}
#[doc = "`write(|w| ..)` method takes [`wwdtcntrst::W`](W) writer structure"]
impl crate::Writable for WwdtcntrstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WWDTCNTRST to value 0"]
impl crate::Resettable for WwdtcntrstSpec {
    const RESET_VALUE: u32 = 0;
}
