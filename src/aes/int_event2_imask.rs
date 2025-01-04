#[doc = "Register `INT_EVENT2_IMASK` reader"]
pub type R = crate::R<IntEvent2ImaskSpec>;
#[doc = "Register `INT_EVENT2_IMASK` writer"]
pub type W = crate::W<IntEvent2ImaskSpec>;
#[doc = "DMA1 event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskDma1 {
    #[doc = "0: CLR"]
    IntEvent2ImaskDma1Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskDma1Set = 1,
}
impl From<IntEvent2ImaskDma1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskDma1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DMA1` reader - DMA1 event mask."]
pub type IntEvent2ImaskDma1R = crate::BitReader<IntEvent2ImaskDma1>;
impl IntEvent2ImaskDma1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskDma1 {
        match self.bits {
            false => IntEvent2ImaskDma1::IntEvent2ImaskDma1Clr,
            true => IntEvent2ImaskDma1::IntEvent2ImaskDma1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_dma1_clr(&self) -> bool {
        *self == IntEvent2ImaskDma1::IntEvent2ImaskDma1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_dma1_set(&self) -> bool {
        *self == IntEvent2ImaskDma1::IntEvent2ImaskDma1Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DMA1` writer - DMA1 event mask."]
pub type IntEvent2ImaskDma1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskDma1>;
impl<'a, REG> IntEvent2ImaskDma1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_dma1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDma1::IntEvent2ImaskDma1Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_dma1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDma1::IntEvent2ImaskDma1Set)
    }
}
impl R {
    #[doc = "Bit 2 - DMA1 event mask."]
    #[inline(always)]
    pub fn int_event2_imask_dma1(&self) -> IntEvent2ImaskDma1R {
        IntEvent2ImaskDma1R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - DMA1 event mask."]
    #[inline(always)]
    pub fn int_event2_imask_dma1(&mut self) -> IntEvent2ImaskDma1W<IntEvent2ImaskSpec> {
        IntEvent2ImaskDma1W::new(self, 2)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event2_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2ImaskSpec;
impl crate::RegisterSpec for IntEvent2ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event2_imask::R`](R) reader structure"]
impl crate::Readable for IntEvent2ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_event2_imask::W`](W) writer structure"]
impl crate::Writable for IntEvent2ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT2_IMASK to value 0"]
impl crate::Resettable for IntEvent2ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
