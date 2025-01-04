#[doc = "Register `SHUTDNSTORE0` reader"]
pub type R = crate::R<Shutdnstore0Spec>;
#[doc = "Register `SHUTDNSTORE0` writer"]
pub type W = crate::W<Shutdnstore0Spec>;
#[doc = "Field `SHUTDNSTORE0_DATA` reader - Shutdown storage byte 0"]
pub type Shutdnstore0DataR = crate::FieldReader;
#[doc = "Field `SHUTDNSTORE0_DATA` writer - Shutdown storage byte 0"]
pub type Shutdnstore0DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Shutdown storage byte 0"]
    #[inline(always)]
    pub fn shutdnstore0_data(&self) -> Shutdnstore0DataR {
        Shutdnstore0DataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Shutdown storage byte 0"]
    #[inline(always)]
    pub fn shutdnstore0_data(&mut self) -> Shutdnstore0DataW<Shutdnstore0Spec> {
        Shutdnstore0DataW::new(self, 0)
    }
}
#[doc = "Shutdown storage memory (byte 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`shutdnstore0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shutdnstore0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Shutdnstore0Spec;
impl crate::RegisterSpec for Shutdnstore0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shutdnstore0::R`](R) reader structure"]
impl crate::Readable for Shutdnstore0Spec {}
#[doc = "`write(|w| ..)` method takes [`shutdnstore0::W`](W) writer structure"]
impl crate::Writable for Shutdnstore0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHUTDNSTORE0 to value 0"]
impl crate::Resettable for Shutdnstore0Spec {
    const RESET_VALUE: u32 = 0;
}
