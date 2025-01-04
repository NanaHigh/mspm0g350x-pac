#[doc = "Register `SHUTDNSTORE2` reader"]
pub type R = crate::R<Shutdnstore2Spec>;
#[doc = "Register `SHUTDNSTORE2` writer"]
pub type W = crate::W<Shutdnstore2Spec>;
#[doc = "Field `SHUTDNSTORE2_DATA` reader - Shutdown storage byte 2"]
pub type Shutdnstore2DataR = crate::FieldReader;
#[doc = "Field `SHUTDNSTORE2_DATA` writer - Shutdown storage byte 2"]
pub type Shutdnstore2DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Shutdown storage byte 2"]
    #[inline(always)]
    pub fn shutdnstore2_data(&self) -> Shutdnstore2DataR {
        Shutdnstore2DataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Shutdown storage byte 2"]
    #[inline(always)]
    pub fn shutdnstore2_data(&mut self) -> Shutdnstore2DataW<Shutdnstore2Spec> {
        Shutdnstore2DataW::new(self, 0)
    }
}
#[doc = "Shutdown storage memory (byte 2)\n\nYou can [`read`](crate::Reg::read) this register and get [`shutdnstore2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shutdnstore2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Shutdnstore2Spec;
impl crate::RegisterSpec for Shutdnstore2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shutdnstore2::R`](R) reader structure"]
impl crate::Readable for Shutdnstore2Spec {}
#[doc = "`write(|w| ..)` method takes [`shutdnstore2::W`](W) writer structure"]
impl crate::Writable for Shutdnstore2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHUTDNSTORE2 to value 0"]
impl crate::Resettable for Shutdnstore2Spec {
    const RESET_VALUE: u32 = 0;
}
