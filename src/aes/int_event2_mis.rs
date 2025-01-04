#[doc = "Register `INT_EVENT2_MIS` reader"]
pub type R = crate::R<IntEvent2MisSpec>;
#[doc = "DMA1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisDma1 {
    #[doc = "0: CLR"]
    IntEvent2MisDma1Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisDma1Set = 1,
}
impl From<IntEvent2MisDma1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisDma1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_DMA1` reader - DMA1 event"]
pub type IntEvent2MisDma1R = crate::BitReader<IntEvent2MisDma1>;
impl IntEvent2MisDma1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisDma1 {
        match self.bits {
            false => IntEvent2MisDma1::IntEvent2MisDma1Clr,
            true => IntEvent2MisDma1::IntEvent2MisDma1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_dma1_clr(&self) -> bool {
        *self == IntEvent2MisDma1::IntEvent2MisDma1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_dma1_set(&self) -> bool {
        *self == IntEvent2MisDma1::IntEvent2MisDma1Set
    }
}
impl R {
    #[doc = "Bit 2 - DMA1 event"]
    #[inline(always)]
    pub fn int_event2_mis_dma1(&self) -> IntEvent2MisDma1R {
        IntEvent2MisDma1R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2MisSpec;
impl crate::RegisterSpec for IntEvent2MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event2_mis::R`](R) reader structure"]
impl crate::Readable for IntEvent2MisSpec {}
#[doc = "`reset()` method sets INT_EVENT2_MIS to value 0"]
impl crate::Resettable for IntEvent2MisSpec {
    const RESET_VALUE: u32 = 0;
}
