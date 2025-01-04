#[doc = "Register `CC_23[%s]` reader"]
pub type R = crate::R<Cc23Spec>;
#[doc = "Register `CC_23[%s]` writer"]
pub type W = crate::W<Cc23Spec>;
#[doc = "Field `CC_23_CCVAL` reader - Capture or compare value"]
pub type Cc23CcvalR = crate::FieldReader<u16>;
#[doc = "Field `CC_23_CCVAL` writer - Capture or compare value"]
pub type Cc23CcvalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture or compare value"]
    #[inline(always)]
    pub fn cc_23_ccval(&self) -> Cc23CcvalR {
        Cc23CcvalR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture or compare value"]
    #[inline(always)]
    pub fn cc_23_ccval(&mut self) -> Cc23CcvalW<Cc23Spec> {
        Cc23CcvalW::new(self, 0)
    }
}
#[doc = "Capture or Compare Register 0 to Capture or Compare Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc23Spec;
impl crate::RegisterSpec for Cc23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc_23::R`](R) reader structure"]
impl crate::Readable for Cc23Spec {}
#[doc = "`write(|w| ..)` method takes [`cc_23::W`](W) writer structure"]
impl crate::Writable for Cc23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC_23[%s]
to value 0"]
impl crate::Resettable for Cc23Spec {
    const RESET_VALUE: u32 = 0;
}
