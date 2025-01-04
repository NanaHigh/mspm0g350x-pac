#[doc = "Register `INT_EVENT2_ICLR` writer"]
pub type W = crate::W<IntEvent2IclrSpec>;
#[doc = "DMA1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrDma1 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrDma1NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrDma1Clr = 1,
}
impl From<IntEvent2IclrDma1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrDma1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_DMA1` writer - DMA1 event"]
pub type IntEvent2IclrDma1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrDma1>;
impl<'a, REG> IntEvent2IclrDma1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_dma1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDma1::IntEvent2IclrDma1NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_dma1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDma1::IntEvent2IclrDma1Clr)
    }
}
impl W {
    #[doc = "Bit 2 - DMA1 event"]
    #[inline(always)]
    pub fn int_event2_iclr_dma1(&mut self) -> IntEvent2IclrDma1W<IntEvent2IclrSpec> {
        IntEvent2IclrDma1W::new(self, 2)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event2_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2IclrSpec;
impl crate::RegisterSpec for IntEvent2IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event2_iclr::W`](W) writer structure"]
impl crate::Writable for IntEvent2IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT2_ICLR to value 0"]
impl crate::Resettable for IntEvent2IclrSpec {
    const RESET_VALUE: u32 = 0;
}
