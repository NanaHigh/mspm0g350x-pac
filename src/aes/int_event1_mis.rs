#[doc = "Register `INT_EVENT1_MIS` reader"]
pub type R = crate::R<IntEvent1MisSpec>;
#[doc = "DMA0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisDma0 {
    #[doc = "0: CLR"]
    IntEvent1MisDma0Clr = 0,
    #[doc = "1: SET"]
    IntEvent1MisDma0Set = 1,
}
impl From<IntEvent1MisDma0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisDma0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_DMA0` reader - DMA0 event"]
pub type IntEvent1MisDma0R = crate::BitReader<IntEvent1MisDma0>;
impl IntEvent1MisDma0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisDma0 {
        match self.bits {
            false => IntEvent1MisDma0::IntEvent1MisDma0Clr,
            true => IntEvent1MisDma0::IntEvent1MisDma0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_dma0_clr(&self) -> bool {
        *self == IntEvent1MisDma0::IntEvent1MisDma0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_dma0_set(&self) -> bool {
        *self == IntEvent1MisDma0::IntEvent1MisDma0Set
    }
}
impl R {
    #[doc = "Bit 1 - DMA0 event"]
    #[inline(always)]
    pub fn int_event1_mis_dma0(&self) -> IntEvent1MisDma0R {
        IntEvent1MisDma0R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1MisSpec;
impl crate::RegisterSpec for IntEvent1MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event1_mis::R`](R) reader structure"]
impl crate::Readable for IntEvent1MisSpec {}
#[doc = "`reset()` method sets INT_EVENT1_MIS to value 0"]
impl crate::Resettable for IntEvent1MisSpec {
    const RESET_VALUE: u32 = 0;
}
