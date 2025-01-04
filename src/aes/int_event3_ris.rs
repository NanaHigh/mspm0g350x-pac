#[doc = "Register `INT_EVENT3_RIS` reader"]
pub type R = crate::R<IntEvent3RisSpec>;
#[doc = "DMA2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent3RisDma2 {
    #[doc = "0: CLR"]
    IntEvent3RisDma2Clr = 0,
    #[doc = "1: SET"]
    IntEvent3RisDma2Set = 1,
}
impl From<IntEvent3RisDma2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent3RisDma2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT3_RIS_DMA2` reader - DMA2 event"]
pub type IntEvent3RisDma2R = crate::BitReader<IntEvent3RisDma2>;
impl IntEvent3RisDma2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent3RisDma2 {
        match self.bits {
            false => IntEvent3RisDma2::IntEvent3RisDma2Clr,
            true => IntEvent3RisDma2::IntEvent3RisDma2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event3_ris_dma2_clr(&self) -> bool {
        *self == IntEvent3RisDma2::IntEvent3RisDma2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event3_ris_dma2_set(&self) -> bool {
        *self == IntEvent3RisDma2::IntEvent3RisDma2Set
    }
}
impl R {
    #[doc = "Bit 3 - DMA2 event"]
    #[inline(always)]
    pub fn int_event3_ris_dma2(&self) -> IntEvent3RisDma2R {
        IntEvent3RisDma2R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event3_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent3RisSpec;
impl crate::RegisterSpec for IntEvent3RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event3_ris::R`](R) reader structure"]
impl crate::Readable for IntEvent3RisSpec {}
#[doc = "`reset()` method sets INT_EVENT3_RIS to value 0"]
impl crate::Resettable for IntEvent3RisSpec {
    const RESET_VALUE: u32 = 0;
}
