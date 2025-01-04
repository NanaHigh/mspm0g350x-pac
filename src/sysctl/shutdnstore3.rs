#[doc = "Register `SHUTDNSTORE3` reader"]
pub type R = crate::R<Shutdnstore3Spec>;
#[doc = "Register `SHUTDNSTORE3` writer"]
pub type W = crate::W<Shutdnstore3Spec>;
#[doc = "Field `SHUTDNSTORE3_DATA` reader - Shutdown storage byte 3"]
pub type Shutdnstore3DataR = crate::FieldReader;
#[doc = "Field `SHUTDNSTORE3_DATA` writer - Shutdown storage byte 3"]
pub type Shutdnstore3DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Shutdown storage byte 3"]
    #[inline(always)]
    pub fn shutdnstore3_data(&self) -> Shutdnstore3DataR {
        Shutdnstore3DataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Shutdown storage byte 3"]
    #[inline(always)]
    pub fn shutdnstore3_data(&mut self) -> Shutdnstore3DataW<Shutdnstore3Spec> {
        Shutdnstore3DataW::new(self, 0)
    }
}
#[doc = "Shutdown storage memory (byte 3)\n\nYou can [`read`](crate::Reg::read) this register and get [`shutdnstore3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shutdnstore3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Shutdnstore3Spec;
impl crate::RegisterSpec for Shutdnstore3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shutdnstore3::R`](R) reader structure"]
impl crate::Readable for Shutdnstore3Spec {}
#[doc = "`write(|w| ..)` method takes [`shutdnstore3::W`](W) writer structure"]
impl crate::Writable for Shutdnstore3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHUTDNSTORE3 to value 0"]
impl crate::Resettable for Shutdnstore3Spec {
    const RESET_VALUE: u32 = 0;
}
