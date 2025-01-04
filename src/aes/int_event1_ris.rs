#[doc = "Register `INT_EVENT1_RIS` reader"]
pub type R = crate::R<IntEvent1RisSpec>;
#[doc = "DMA0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisDma0 {
    #[doc = "0: CLR"]
    IntEvent1RisDma0Clr = 0,
    #[doc = "1: SET"]
    IntEvent1RisDma0Set = 1,
}
impl From<IntEvent1RisDma0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisDma0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_DMA0` reader - DMA0 event"]
pub type IntEvent1RisDma0R = crate::BitReader<IntEvent1RisDma0>;
impl IntEvent1RisDma0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisDma0 {
        match self.bits {
            false => IntEvent1RisDma0::IntEvent1RisDma0Clr,
            true => IntEvent1RisDma0::IntEvent1RisDma0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_dma0_clr(&self) -> bool {
        *self == IntEvent1RisDma0::IntEvent1RisDma0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_dma0_set(&self) -> bool {
        *self == IntEvent1RisDma0::IntEvent1RisDma0Set
    }
}
impl R {
    #[doc = "Bit 1 - DMA0 event"]
    #[inline(always)]
    pub fn int_event1_ris_dma0(&self) -> IntEvent1RisDma0R {
        IntEvent1RisDma0R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1RisSpec;
impl crate::RegisterSpec for IntEvent1RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event1_ris::R`](R) reader structure"]
impl crate::Readable for IntEvent1RisSpec {}
#[doc = "`reset()` method sets INT_EVENT1_RIS to value 0"]
impl crate::Resettable for IntEvent1RisSpec {
    const RESET_VALUE: u32 = 0;
}
