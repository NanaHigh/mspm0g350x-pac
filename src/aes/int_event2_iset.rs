#[doc = "Register `INT_EVENT2_ISET` writer"]
pub type W = crate::W<IntEvent2IsetSpec>;
#[doc = "DMA1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetDma1 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetDma1NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetDma1Set = 1,
}
impl From<IntEvent2IsetDma1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetDma1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_DMA1` writer - DMA1 event"]
pub type IntEvent2IsetDma1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetDma1>;
impl<'a, REG> IntEvent2IsetDma1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_dma1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDma1::IntEvent2IsetDma1NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_dma1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDma1::IntEvent2IsetDma1Set)
    }
}
impl W {
    #[doc = "Bit 2 - DMA1 event"]
    #[inline(always)]
    pub fn int_event2_iset_dma1(&mut self) -> IntEvent2IsetDma1W<IntEvent2IsetSpec> {
        IntEvent2IsetDma1W::new(self, 2)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event2_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2IsetSpec;
impl crate::RegisterSpec for IntEvent2IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event2_iset::W`](W) writer structure"]
impl crate::Writable for IntEvent2IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT2_ISET to value 0"]
impl crate::Resettable for IntEvent2IsetSpec {
    const RESET_VALUE: u32 = 0;
}
