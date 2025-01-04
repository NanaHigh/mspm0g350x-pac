#[doc = "Register `INT_EVENT0_ISET` writer"]
pub type W = crate::W<IntEvent0IsetSpec>;
#[doc = "AES ready interrupt, set when the selected AES operation was completed and the result can be read from AESADOUT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetAesrdy {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetAesrdyNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetAesrdySet = 1,
}
impl From<IntEvent0IsetAesrdy> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetAesrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_AESRDY` writer - AES ready interrupt, set when the selected AES operation was completed and the result can be read from AESADOUT."]
pub type IntEvent0IsetAesrdyW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetAesrdy>;
impl<'a, REG> IntEvent0IsetAesrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_aesrdy_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetAesrdy::IntEvent0IsetAesrdyNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_aesrdy_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetAesrdy::IntEvent0IsetAesrdySet)
    }
}
#[doc = "DMA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDma0 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDma0NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDma0Set = 1,
}
impl From<IntEvent0IsetDma0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDma0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DMA0` writer - DMA0"]
pub type IntEvent0IsetDma0W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDma0>;
impl<'a, REG> IntEvent0IsetDma0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dma0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDma0::IntEvent0IsetDma0NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dma0_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDma0::IntEvent0IsetDma0Set)
    }
}
#[doc = "DMA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDma1 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDma1NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDma1Set = 1,
}
impl From<IntEvent0IsetDma1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDma1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DMA1` writer - DMA1"]
pub type IntEvent0IsetDma1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDma1>;
impl<'a, REG> IntEvent0IsetDma1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dma1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDma1::IntEvent0IsetDma1NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dma1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDma1::IntEvent0IsetDma1Set)
    }
}
#[doc = "DMA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDma2 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDma2NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDma2Set = 1,
}
impl From<IntEvent0IsetDma2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDma2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DMA2` writer - DMA2"]
pub type IntEvent0IsetDma2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDma2>;
impl<'a, REG> IntEvent0IsetDma2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dma2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDma2::IntEvent0IsetDma2NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dma2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDma2::IntEvent0IsetDma2Set)
    }
}
impl W {
    #[doc = "Bit 0 - AES ready interrupt, set when the selected AES operation was completed and the result can be read from AESADOUT."]
    #[inline(always)]
    pub fn int_event0_iset_aesrdy(&mut self) -> IntEvent0IsetAesrdyW<IntEvent0IsetSpec> {
        IntEvent0IsetAesrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA0"]
    #[inline(always)]
    pub fn int_event0_iset_dma0(&mut self) -> IntEvent0IsetDma0W<IntEvent0IsetSpec> {
        IntEvent0IsetDma0W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA1"]
    #[inline(always)]
    pub fn int_event0_iset_dma1(&mut self) -> IntEvent0IsetDma1W<IntEvent0IsetSpec> {
        IntEvent0IsetDma1W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA2"]
    #[inline(always)]
    pub fn int_event0_iset_dma2(&mut self) -> IntEvent0IsetDma2W<IntEvent0IsetSpec> {
        IntEvent0IsetDma2W::new(self, 3)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event0_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0IsetSpec;
impl crate::RegisterSpec for IntEvent0IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event0_iset::W`](W) writer structure"]
impl crate::Writable for IntEvent0IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT0_ISET to value 0"]
impl crate::Resettable for IntEvent0IsetSpec {
    const RESET_VALUE: u32 = 0;
}
