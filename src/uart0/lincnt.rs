#[doc = "Register `LINCNT` reader"]
pub type R = crate::R<LincntSpec>;
#[doc = "Register `LINCNT` writer"]
pub type W = crate::W<LincntSpec>;
#[doc = "Field `LINCNT_VALUE` reader - 16 bit up counter clocked by the functional clock of the UART."]
pub type LincntValueR = crate::FieldReader<u16>;
#[doc = "Field `LINCNT_VALUE` writer - 16 bit up counter clocked by the functional clock of the UART."]
pub type LincntValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 16 bit up counter clocked by the functional clock of the UART."]
    #[inline(always)]
    pub fn lincnt_value(&self) -> LincntValueR {
        LincntValueR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 16 bit up counter clocked by the functional clock of the UART."]
    #[inline(always)]
    pub fn lincnt_value(&mut self) -> LincntValueW<LincntSpec> {
        LincntValueW::new(self, 0)
    }
}
#[doc = "UART LIN Mode Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lincnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lincnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LincntSpec;
impl crate::RegisterSpec for LincntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lincnt::R`](R) reader structure"]
impl crate::Readable for LincntSpec {}
#[doc = "`write(|w| ..)` method takes [`lincnt::W`](W) writer structure"]
impl crate::Writable for LincntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINCNT to value 0"]
impl crate::Resettable for LincntSpec {
    const RESET_VALUE: u32 = 0;
}
