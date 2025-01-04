#[doc = "Register `INT_EVENT0_ICLR` writer"]
pub type W = crate::W<IntEvent0IclrSpec>;
#[doc = "AES ready interrupt, set when the selected AES operation was completed and the result can be read from AESADOUT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrAesrdy {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrAesrdyNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrAesrdyClr = 1,
}
impl From<IntEvent0IclrAesrdy> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrAesrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_AESRDY` writer - AES ready interrupt, set when the selected AES operation was completed and the result can be read from AESADOUT."]
pub type IntEvent0IclrAesrdyW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrAesrdy>;
impl<'a, REG> IntEvent0IclrAesrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_aesrdy_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrAesrdy::IntEvent0IclrAesrdyNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_aesrdy_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrAesrdy::IntEvent0IclrAesrdyClr)
    }
}
#[doc = "DMA0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDma0 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDma0NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDma0Clr = 1,
}
impl From<IntEvent0IclrDma0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDma0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DMA0` writer - DMA0"]
pub type IntEvent0IclrDma0W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDma0>;
impl<'a, REG> IntEvent0IclrDma0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dma0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDma0::IntEvent0IclrDma0NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dma0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDma0::IntEvent0IclrDma0Clr)
    }
}
#[doc = "DMA1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDma1 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDma1NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDma1Clr = 1,
}
impl From<IntEvent0IclrDma1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDma1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DMA1` writer - DMA1"]
pub type IntEvent0IclrDma1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDma1>;
impl<'a, REG> IntEvent0IclrDma1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dma1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDma1::IntEvent0IclrDma1NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dma1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDma1::IntEvent0IclrDma1Clr)
    }
}
#[doc = "DMA2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDma2 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDma2NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDma2Clr = 1,
}
impl From<IntEvent0IclrDma2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDma2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DMA2` writer - DMA2"]
pub type IntEvent0IclrDma2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDma2>;
impl<'a, REG> IntEvent0IclrDma2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dma2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDma2::IntEvent0IclrDma2NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dma2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDma2::IntEvent0IclrDma2Clr)
    }
}
impl W {
    #[doc = "Bit 0 - AES ready interrupt, set when the selected AES operation was completed and the result can be read from AESADOUT."]
    #[inline(always)]
    pub fn int_event0_iclr_aesrdy(&mut self) -> IntEvent0IclrAesrdyW<IntEvent0IclrSpec> {
        IntEvent0IclrAesrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA0"]
    #[inline(always)]
    pub fn int_event0_iclr_dma0(&mut self) -> IntEvent0IclrDma0W<IntEvent0IclrSpec> {
        IntEvent0IclrDma0W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA1"]
    #[inline(always)]
    pub fn int_event0_iclr_dma1(&mut self) -> IntEvent0IclrDma1W<IntEvent0IclrSpec> {
        IntEvent0IclrDma1W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA2"]
    #[inline(always)]
    pub fn int_event0_iclr_dma2(&mut self) -> IntEvent0IclrDma2W<IntEvent0IclrSpec> {
        IntEvent0IclrDma2W::new(self, 3)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event0_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0IclrSpec;
impl crate::RegisterSpec for IntEvent0IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event0_iclr::W`](W) writer structure"]
impl crate::Writable for IntEvent0IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT0_ICLR to value 0"]
impl crate::Resettable for IntEvent0IclrSpec {
    const RESET_VALUE: u32 = 0;
}
