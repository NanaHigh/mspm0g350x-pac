#[doc = "Register `INT_EVENT1_ISET` writer"]
pub type W = crate::W<IntEvent1IsetSpec>;
#[doc = "DMA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetDma0 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetDma0NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetDma0Set = 1,
}
impl From<IntEvent1IsetDma0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetDma0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_DMA0` writer - DMA0"]
pub type IntEvent1IsetDma0W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetDma0>;
impl<'a, REG> IntEvent1IsetDma0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_dma0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDma0::IntEvent1IsetDma0NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_dma0_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDma0::IntEvent1IsetDma0Set)
    }
}
impl W {
    #[doc = "Bit 1 - DMA0"]
    #[inline(always)]
    pub fn int_event1_iset_dma0(&mut self) -> IntEvent1IsetDma0W<IntEvent1IsetSpec> {
        IntEvent1IsetDma0W::new(self, 1)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event1_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1IsetSpec;
impl crate::RegisterSpec for IntEvent1IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event1_iset::W`](W) writer structure"]
impl crate::Writable for IntEvent1IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT1_ISET to value 0"]
impl crate::Resettable for IntEvent1IsetSpec {
    const RESET_VALUE: u32 = 0;
}
