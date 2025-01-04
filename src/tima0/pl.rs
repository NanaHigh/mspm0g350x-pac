#[doc = "Register `PL` reader"]
pub type R = crate::R<PlSpec>;
#[doc = "Register `PL` writer"]
pub type W = crate::W<PlSpec>;
#[doc = "Field `PL_PHASE` reader - Phase Load value"]
pub type PlPhaseR = crate::FieldReader<u16>;
#[doc = "Field `PL_PHASE` writer - Phase Load value"]
pub type PlPhaseW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Phase Load value"]
    #[inline(always)]
    pub fn pl_phase(&self) -> PlPhaseR {
        PlPhaseR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Phase Load value"]
    #[inline(always)]
    pub fn pl_phase(&mut self) -> PlPhaseW<PlSpec> {
        PlPhaseW::new(self, 0)
    }
}
#[doc = "Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PlSpec;
impl crate::RegisterSpec for PlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pl::R`](R) reader structure"]
impl crate::Readable for PlSpec {}
#[doc = "`write(|w| ..)` method takes [`pl::W`](W) writer structure"]
impl crate::Writable for PlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PL to value 0"]
impl crate::Resettable for PlSpec {
    const RESET_VALUE: u32 = 0;
}
