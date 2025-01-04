#[doc = "Register `INT_EVENT3_ICLR` writer"]
pub type W = crate::W<IntEvent3IclrSpec>;
#[doc = "DMA2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent3IclrDma2 {
    #[doc = "0: NO_EFFECT"]
    IntEvent3IclrDma2NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent3IclrDma2Clr = 1,
}
impl From<IntEvent3IclrDma2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent3IclrDma2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT3_ICLR_DMA2` writer - DMA2 event"]
pub type IntEvent3IclrDma2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent3IclrDma2>;
impl<'a, REG> IntEvent3IclrDma2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event3_iclr_dma2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent3IclrDma2::IntEvent3IclrDma2NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event3_iclr_dma2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent3IclrDma2::IntEvent3IclrDma2Clr)
    }
}
impl W {
    #[doc = "Bit 3 - DMA2 event"]
    #[inline(always)]
    pub fn int_event3_iclr_dma2(&mut self) -> IntEvent3IclrDma2W<IntEvent3IclrSpec> {
        IntEvent3IclrDma2W::new(self, 3)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event3_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent3IclrSpec;
impl crate::RegisterSpec for IntEvent3IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event3_iclr::W`](W) writer structure"]
impl crate::Writable for IntEvent3IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT3_ICLR to value 0"]
impl crate::Resettable for IntEvent3IclrSpec {
    const RESET_VALUE: u32 = 0;
}
