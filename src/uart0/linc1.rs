#[doc = "Register `LINC1` reader"]
pub type R = crate::R<Linc1Spec>;
#[doc = "Register `LINC1` writer"]
pub type W = crate::W<Linc1Spec>;
#[doc = "Field `LINC1_DATA` reader - 16 Bit Capture / Compare Register Captures current LINCTR value on RXD rising edge and can generate a LINC1 interrupt when capture is enabled (LINC1CAP = 1)"]
pub type Linc1DataR = crate::FieldReader<u16>;
#[doc = "Field `LINC1_DATA` writer - 16 Bit Capture / Compare Register Captures current LINCTR value on RXD rising edge and can generate a LINC1 interrupt when capture is enabled (LINC1CAP = 1)"]
pub type Linc1DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 16 Bit Capture / Compare Register Captures current LINCTR value on RXD rising edge and can generate a LINC1 interrupt when capture is enabled (LINC1CAP = 1)"]
    #[inline(always)]
    pub fn linc1_data(&self) -> Linc1DataR {
        Linc1DataR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 16 Bit Capture / Compare Register Captures current LINCTR value on RXD rising edge and can generate a LINC1 interrupt when capture is enabled (LINC1CAP = 1)"]
    #[inline(always)]
    pub fn linc1_data(&mut self) -> Linc1DataW<Linc1Spec> {
        Linc1DataW::new(self, 0)
    }
}
#[doc = "UART LIN Mode Capture 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`linc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`linc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Linc1Spec;
impl crate::RegisterSpec for Linc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`linc1::R`](R) reader structure"]
impl crate::Readable for Linc1Spec {}
#[doc = "`write(|w| ..)` method takes [`linc1::W`](W) writer structure"]
impl crate::Writable for Linc1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINC1 to value 0"]
impl crate::Resettable for Linc1Spec {
    const RESET_VALUE: u32 = 0;
}
