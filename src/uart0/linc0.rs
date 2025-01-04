#[doc = "Register `LINC0` reader"]
pub type R = crate::R<Linc0Spec>;
#[doc = "Register `LINC0` writer"]
pub type W = crate::W<Linc0Spec>;
#[doc = "Field `LINC0_DATA` reader - 16 Bit Capture / Compare Register Captures current LINCTR value on RXD falling edge and can generate a LINC0 interrupt when capture is enabled (LINC0CAP = 1). If compare mode is enabled (LINC0_MATCH = 1), a counter match can generate a LINC0 interrupt."]
pub type Linc0DataR = crate::FieldReader<u16>;
#[doc = "Field `LINC0_DATA` writer - 16 Bit Capture / Compare Register Captures current LINCTR value on RXD falling edge and can generate a LINC0 interrupt when capture is enabled (LINC0CAP = 1). If compare mode is enabled (LINC0_MATCH = 1), a counter match can generate a LINC0 interrupt."]
pub type Linc0DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 16 Bit Capture / Compare Register Captures current LINCTR value on RXD falling edge and can generate a LINC0 interrupt when capture is enabled (LINC0CAP = 1). If compare mode is enabled (LINC0_MATCH = 1), a counter match can generate a LINC0 interrupt."]
    #[inline(always)]
    pub fn linc0_data(&self) -> Linc0DataR {
        Linc0DataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 16 Bit Capture / Compare Register Captures current LINCTR value on RXD falling edge and can generate a LINC0 interrupt when capture is enabled (LINC0CAP = 1). If compare mode is enabled (LINC0_MATCH = 1), a counter match can generate a LINC0 interrupt."]
    #[inline(always)]
    pub fn linc0_data(&mut self) -> Linc0DataW<Linc0Spec> {
        Linc0DataW::new(self, 0)
    }
}
#[doc = "UART LIN Mode Capture 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`linc0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linc0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Linc0Spec;
impl crate::RegisterSpec for Linc0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`linc0::R`](R) reader structure"]
impl crate::Readable for Linc0Spec {}
#[doc = "`write(|w| ..)` method takes [`linc0::W`](W) writer structure"]
impl crate::Writable for Linc0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINC0 to value 0"]
impl crate::Resettable for Linc0Spec {
    const RESET_VALUE: u32 = 0;
}
