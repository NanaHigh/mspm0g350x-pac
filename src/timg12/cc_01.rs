#[doc = "Register `CC_01[%s]` reader"]
pub type R = crate::R<Cc01Spec>;
#[doc = "Register `CC_01[%s]` writer"]
pub type W = crate::W<Cc01Spec>;
#[doc = "Field `CC_01_CCVAL` reader - Capture or compare value"]
pub type Cc01CcvalR = crate::FieldReader<u32>;
#[doc = "Field `CC_01_CCVAL` writer - Capture or compare value"]
pub type Cc01CcvalW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Capture or compare value"]
    #[inline(always)]
    pub fn cc_01_ccval(&self) -> Cc01CcvalR {
        Cc01CcvalR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Capture or compare value"]
    #[inline(always)]
    pub fn cc_01_ccval(&mut self) -> Cc01CcvalW<Cc01Spec> {
        Cc01CcvalW::new(self, 0)
    }
}
#[doc = "Capture or Compare Register 0 to Capture or Compare Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc01Spec;
impl crate::RegisterSpec for Cc01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc_01::R`](R) reader structure"]
impl crate::Readable for Cc01Spec {}
#[doc = "`write(|w| ..)` method takes [`cc_01::W`](W) writer structure"]
impl crate::Writable for Cc01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC_01[%s]
to value 0"]
impl crate::Resettable for Cc01Spec {
    const RESET_VALUE: u32 = 0;
}
