#[doc = "Register `INT_EVENT2_RIS` reader"]
pub type R = crate::R<IntEvent2RisSpec>;
#[doc = "DMA1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisDma1 {
    #[doc = "0: CLR"]
    IntEvent2RisDma1Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisDma1Set = 1,
}
impl From<IntEvent2RisDma1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisDma1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_DMA1` reader - DMA1 event"]
pub type IntEvent2RisDma1R = crate::BitReader<IntEvent2RisDma1>;
impl IntEvent2RisDma1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisDma1 {
        match self.bits {
            false => IntEvent2RisDma1::IntEvent2RisDma1Clr,
            true => IntEvent2RisDma1::IntEvent2RisDma1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_dma1_clr(&self) -> bool {
        *self == IntEvent2RisDma1::IntEvent2RisDma1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_dma1_set(&self) -> bool {
        *self == IntEvent2RisDma1::IntEvent2RisDma1Set
    }
}
impl R {
    #[doc = "Bit 2 - DMA1 event"]
    #[inline(always)]
    pub fn int_event2_ris_dma1(&self) -> IntEvent2RisDma1R {
        IntEvent2RisDma1R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2RisSpec;
impl crate::RegisterSpec for IntEvent2RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event2_ris::R`](R) reader structure"]
impl crate::Readable for IntEvent2RisSpec {}
#[doc = "`reset()` method sets INT_EVENT2_RIS to value 0"]
impl crate::Resettable for IntEvent2RisSpec {
    const RESET_VALUE: u32 = 0;
}
