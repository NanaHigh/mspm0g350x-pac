#[doc = "Register `RES2` reader"]
pub type R = crate::R<Res2Spec>;
#[doc = "Register `RES2` writer"]
pub type W = crate::W<Res2Spec>;
#[doc = "Field `RES2_DATA` reader - Result 2 register"]
pub type Res2DataR = crate::FieldReader<u32>;
#[doc = "Field `RES2_DATA` writer - Result 2 register"]
pub type Res2DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Result 2 register"]
    #[inline(always)]
    pub fn res2_data(&self) -> Res2DataR {
        Res2DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Result 2 register"]
    #[inline(always)]
    pub fn res2_data(&mut self) -> Res2DataW<Res2Spec> {
        Res2DataW::new(self, 0)
    }
}
#[doc = "Result 2 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`res2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`res2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Res2Spec;
impl crate::RegisterSpec for Res2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res2::R`](R) reader structure"]
impl crate::Readable for Res2Spec {}
#[doc = "`write(|w| ..)` method takes [`res2::W`](W) writer structure"]
impl crate::Writable for Res2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RES2 to value 0"]
impl crate::Resettable for Res2Spec {
    const RESET_VALUE: u32 = 0;
}
