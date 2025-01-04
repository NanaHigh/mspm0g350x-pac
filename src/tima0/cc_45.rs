#[doc = "Register `CC_45[%s]` reader"]
pub type R = crate::R<Cc45Spec>;
#[doc = "Register `CC_45[%s]` writer"]
pub type W = crate::W<Cc45Spec>;
#[doc = "Field `CC_45_CCVAL` reader - Capture or compare value"]
pub type Cc45CcvalR = crate::FieldReader<u16>;
#[doc = "Field `CC_45_CCVAL` writer - Capture or compare value"]
pub type Cc45CcvalW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Capture or compare value"]
    #[inline(always)]
    pub fn cc_45_ccval(&self) -> Cc45CcvalR {
        Cc45CcvalR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Capture or compare value"]
    #[inline(always)]
    pub fn cc_45_ccval(&mut self) -> Cc45CcvalW<Cc45Spec> {
        Cc45CcvalW::new(self, 0)
    }
}
#[doc = "Compare Register 4 to Compare Register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`cc_45::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc_45::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc45Spec;
impl crate::RegisterSpec for Cc45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc_45::R`](R) reader structure"]
impl crate::Readable for Cc45Spec {}
#[doc = "`write(|w| ..)` method takes [`cc_45::W`](W) writer structure"]
impl crate::Writable for Cc45Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC_45[%s]
to value 0"]
impl crate::Resettable for Cc45Spec {
    const RESET_VALUE: u32 = 0;
}
