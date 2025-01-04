#[doc = "Register `INT_EVENT3_IMASK` reader"]
pub type R = crate::R<IntEvent3ImaskSpec>;
#[doc = "Register `INT_EVENT3_IMASK` writer"]
pub type W = crate::W<IntEvent3ImaskSpec>;
#[doc = "DMA2 event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent3ImaskDma2 {
    #[doc = "0: CLR"]
    IntEvent3ImaskDma2Clr = 0,
    #[doc = "1: SET"]
    IntEvent3ImaskDma2Set = 1,
}
impl From<IntEvent3ImaskDma2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent3ImaskDma2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT3_IMASK_DMA2` reader - DMA2 event mask."]
pub type IntEvent3ImaskDma2R = crate::BitReader<IntEvent3ImaskDma2>;
impl IntEvent3ImaskDma2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent3ImaskDma2 {
        match self.bits {
            false => IntEvent3ImaskDma2::IntEvent3ImaskDma2Clr,
            true => IntEvent3ImaskDma2::IntEvent3ImaskDma2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event3_imask_dma2_clr(&self) -> bool {
        *self == IntEvent3ImaskDma2::IntEvent3ImaskDma2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event3_imask_dma2_set(&self) -> bool {
        *self == IntEvent3ImaskDma2::IntEvent3ImaskDma2Set
    }
}
#[doc = "Field `INT_EVENT3_IMASK_DMA2` writer - DMA2 event mask."]
pub type IntEvent3ImaskDma2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent3ImaskDma2>;
impl<'a, REG> IntEvent3ImaskDma2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event3_imask_dma2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent3ImaskDma2::IntEvent3ImaskDma2Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event3_imask_dma2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent3ImaskDma2::IntEvent3ImaskDma2Set)
    }
}
impl R {
    #[doc = "Bit 3 - DMA2 event mask."]
    #[inline(always)]
    pub fn int_event3_imask_dma2(&self) -> IntEvent3ImaskDma2R {
        IntEvent3ImaskDma2R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - DMA2 event mask."]
    #[inline(always)]
    pub fn int_event3_imask_dma2(&mut self) -> IntEvent3ImaskDma2W<IntEvent3ImaskSpec> {
        IntEvent3ImaskDma2W::new(self, 3)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event3_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event3_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent3ImaskSpec;
impl crate::RegisterSpec for IntEvent3ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event3_imask::R`](R) reader structure"]
impl crate::Readable for IntEvent3ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_event3_imask::W`](W) writer structure"]
impl crate::Writable for IntEvent3ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT3_IMASK to value 0"]
impl crate::Resettable for IntEvent3ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
