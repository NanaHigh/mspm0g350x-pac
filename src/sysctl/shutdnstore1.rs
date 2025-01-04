#[doc = "Register `SHUTDNSTORE1` reader"]
pub type R = crate::R<Shutdnstore1Spec>;
#[doc = "Register `SHUTDNSTORE1` writer"]
pub type W = crate::W<Shutdnstore1Spec>;
#[doc = "Field `SHUTDNSTORE1_DATA` reader - Shutdown storage byte 1"]
pub type Shutdnstore1DataR = crate::FieldReader;
#[doc = "Field `SHUTDNSTORE1_DATA` writer - Shutdown storage byte 1"]
pub type Shutdnstore1DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Shutdown storage byte 1"]
    #[inline(always)]
    pub fn shutdnstore1_data(&self) -> Shutdnstore1DataR {
        Shutdnstore1DataR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Shutdown storage byte 1"]
    #[inline(always)]
    pub fn shutdnstore1_data(&mut self) -> Shutdnstore1DataW<Shutdnstore1Spec> {
        Shutdnstore1DataW::new(self, 0)
    }
}
#[doc = "Shutdown storage memory (byte 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`shutdnstore1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shutdnstore1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Shutdnstore1Spec;
impl crate::RegisterSpec for Shutdnstore1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shutdnstore1::R`](R) reader structure"]
impl crate::Readable for Shutdnstore1Spec {}
#[doc = "`write(|w| ..)` method takes [`shutdnstore1::W`](W) writer structure"]
impl crate::Writable for Shutdnstore1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHUTDNSTORE1 to value 0"]
impl crate::Resettable for Shutdnstore1Spec {
    const RESET_VALUE: u32 = 0;
}
