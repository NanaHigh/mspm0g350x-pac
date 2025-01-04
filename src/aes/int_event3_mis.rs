#[doc = "Register `INT_EVENT3_MIS` reader"]
pub type R = crate::R<IntEvent3MisSpec>;
#[doc = "DMA2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent3MisDma2 {
    #[doc = "0: CLR"]
    IntEvent3MisDma2Clr = 0,
    #[doc = "1: SET"]
    IntEvent3MisDma2Set = 1,
}
impl From<IntEvent3MisDma2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent3MisDma2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT3_MIS_DMA2` reader - DMA2 event"]
pub type IntEvent3MisDma2R = crate::BitReader<IntEvent3MisDma2>;
impl IntEvent3MisDma2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent3MisDma2 {
        match self.bits {
            false => IntEvent3MisDma2::IntEvent3MisDma2Clr,
            true => IntEvent3MisDma2::IntEvent3MisDma2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event3_mis_dma2_clr(&self) -> bool {
        *self == IntEvent3MisDma2::IntEvent3MisDma2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event3_mis_dma2_set(&self) -> bool {
        *self == IntEvent3MisDma2::IntEvent3MisDma2Set
    }
}
impl R {
    #[doc = "Bit 3 - DMA2 event"]
    #[inline(always)]
    pub fn int_event3_mis_dma2(&self) -> IntEvent3MisDma2R {
        IntEvent3MisDma2R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event3_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent3MisSpec;
impl crate::RegisterSpec for IntEvent3MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event3_mis::R`](R) reader structure"]
impl crate::Readable for IntEvent3MisSpec {}
#[doc = "`reset()` method sets INT_EVENT3_MIS to value 0"]
impl crate::Resettable for IntEvent3MisSpec {
    const RESET_VALUE: u32 = 0;
}
