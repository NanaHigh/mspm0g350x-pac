#[doc = "Register `INT_EVENT1_IMASK` reader"]
pub type R = crate::R<IntEvent1ImaskSpec>;
#[doc = "Register `INT_EVENT1_IMASK` writer"]
pub type W = crate::W<IntEvent1ImaskSpec>;
#[doc = "DMA0 event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskDma0 {
    #[doc = "0: CLR"]
    IntEvent1ImaskDma0Clr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskDma0Set = 1,
}
impl From<IntEvent1ImaskDma0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskDma0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DMA0` reader - DMA0 event mask."]
pub type IntEvent1ImaskDma0R = crate::BitReader<IntEvent1ImaskDma0>;
impl IntEvent1ImaskDma0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskDma0 {
        match self.bits {
            false => IntEvent1ImaskDma0::IntEvent1ImaskDma0Clr,
            true => IntEvent1ImaskDma0::IntEvent1ImaskDma0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_dma0_clr(&self) -> bool {
        *self == IntEvent1ImaskDma0::IntEvent1ImaskDma0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_dma0_set(&self) -> bool {
        *self == IntEvent1ImaskDma0::IntEvent1ImaskDma0Set
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DMA0` writer - DMA0 event mask."]
pub type IntEvent1ImaskDma0W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskDma0>;
impl<'a, REG> IntEvent1ImaskDma0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_dma0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDma0::IntEvent1ImaskDma0Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_dma0_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDma0::IntEvent1ImaskDma0Set)
    }
}
impl R {
    #[doc = "Bit 1 - DMA0 event mask."]
    #[inline(always)]
    pub fn int_event1_imask_dma0(&self) -> IntEvent1ImaskDma0R {
        IntEvent1ImaskDma0R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DMA0 event mask."]
    #[inline(always)]
    pub fn int_event1_imask_dma0(&mut self) -> IntEvent1ImaskDma0W<IntEvent1ImaskSpec> {
        IntEvent1ImaskDma0W::new(self, 1)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event1_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1ImaskSpec;
impl crate::RegisterSpec for IntEvent1ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event1_imask::R`](R) reader structure"]
impl crate::Readable for IntEvent1ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_event1_imask::W`](W) writer structure"]
impl crate::Writable for IntEvent1ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT1_IMASK to value 0"]
impl crate::Resettable for IntEvent1ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
