#[doc = "Register `RES1` reader"]
pub type R = crate::R<Res1Spec>;
#[doc = "Register `RES1` writer"]
pub type W = crate::W<Res1Spec>;
#[doc = "Field `RES1_DATA` reader - Result 1 register"]
pub type Res1DataR = crate::FieldReader<u32>;
#[doc = "Field `RES1_DATA` writer - Result 1 register"]
pub type Res1DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Result 1 register"]
    #[inline(always)]
    pub fn res1_data(&self) -> Res1DataR {
        Res1DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Result 1 register"]
    #[inline(always)]
    pub fn res1_data(&mut self) -> Res1DataW<Res1Spec> {
        Res1DataW::new(self, 0)
    }
}
#[doc = "Result 1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`res1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res1Spec;
impl crate::RegisterSpec for Res1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res1::R`](R) reader structure"]
impl crate::Readable for Res1Spec {}
#[doc = "`write(|w| ..)` method takes [`res1::W`](W) writer structure"]
impl crate::Writable for Res1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES1 to value 0"]
impl crate::Resettable for Res1Spec {
    const RESET_VALUE: u32 = 0;
}
