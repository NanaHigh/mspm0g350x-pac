#[doc = "Register `INT_EVENT3_ISET` writer"]
pub type W = crate::W<IntEvent3IsetSpec>;
#[doc = "DMA2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent3IsetDma2 {
    #[doc = "0: NO_EFFECT"]
    IntEvent3IsetDma2NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent3IsetDma2Set = 1,
}
impl From<IntEvent3IsetDma2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent3IsetDma2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT3_ISET_DMA2` writer - DMA2 event"]
pub type IntEvent3IsetDma2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent3IsetDma2>;
impl<'a, REG> IntEvent3IsetDma2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event3_iset_dma2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent3IsetDma2::IntEvent3IsetDma2NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event3_iset_dma2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent3IsetDma2::IntEvent3IsetDma2Set)
    }
}
impl W {
    #[doc = "Bit 3 - DMA2 event"]
    #[inline(always)]
    pub fn int_event3_iset_dma2(&mut self) -> IntEvent3IsetDma2W<IntEvent3IsetSpec> {
        IntEvent3IsetDma2W::new(self, 3)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event3_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent3IsetSpec;
impl crate::RegisterSpec for IntEvent3IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event3_iset::W`](W) writer structure"]
impl crate::Writable for IntEvent3IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT3_ISET to value 0"]
impl crate::Resettable for IntEvent3IsetSpec {
    const RESET_VALUE: u32 = 0;
}
