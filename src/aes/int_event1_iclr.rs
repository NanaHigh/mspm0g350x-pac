#[doc = "Register `INT_EVENT1_ICLR` writer"]
pub type W = crate::W<IntEvent1IclrSpec>;
#[doc = "DMA0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrDma0 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrDma0NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrDma0Clr = 1,
}
impl From<IntEvent1IclrDma0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrDma0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_DMA0` writer - DMA0 event"]
pub type IntEvent1IclrDma0W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrDma0>;
impl<'a, REG> IntEvent1IclrDma0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_dma0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDma0::IntEvent1IclrDma0NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_dma0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDma0::IntEvent1IclrDma0Clr)
    }
}
impl W {
    #[doc = "Bit 1 - DMA0 event"]
    #[inline(always)]
    pub fn int_event1_iclr_dma0(&mut self) -> IntEvent1IclrDma0W<IntEvent1IclrSpec> {
        IntEvent1IclrDma0W::new(self, 1)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event1_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1IclrSpec;
impl crate::RegisterSpec for IntEvent1IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event1_iclr::W`](W) writer structure"]
impl crate::Writable for IntEvent1IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT1_ICLR to value 0"]
impl crate::Resettable for IntEvent1IclrSpec {
    const RESET_VALUE: u32 = 0;
}
