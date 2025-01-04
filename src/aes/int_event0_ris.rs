#[doc = "Register `INT_EVENT0_RIS` reader"]
pub type R = crate::R<IntEvent0RisSpec>;
#[doc = "AES ready interrupt, set when the selected AES operation was completed and the result can be read from AESADOUT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisAesrdy {
    #[doc = "0: CLR"]
    IntEvent0RisAesrdyClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisAesrdySet = 1,
}
impl From<IntEvent0RisAesrdy> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisAesrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_AESRDY` reader - AES ready interrupt, set when the selected AES operation was completed and the result can be read from AESADOUT."]
pub type IntEvent0RisAesrdyR = crate::BitReader<IntEvent0RisAesrdy>;
impl IntEvent0RisAesrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisAesrdy {
        match self.bits {
            false => IntEvent0RisAesrdy::IntEvent0RisAesrdyClr,
            true => IntEvent0RisAesrdy::IntEvent0RisAesrdySet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_aesrdy_clr(&self) -> bool {
        *self == IntEvent0RisAesrdy::IntEvent0RisAesrdyClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_aesrdy_set(&self) -> bool {
        *self == IntEvent0RisAesrdy::IntEvent0RisAesrdySet
    }
}
#[doc = "DMA0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDma0 {
    #[doc = "0: CLR"]
    IntEvent0RisDma0Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDma0Set = 1,
}
impl From<IntEvent0RisDma0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDma0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DMA0` reader - DMA0 event"]
pub type IntEvent0RisDma0R = crate::BitReader<IntEvent0RisDma0>;
impl IntEvent0RisDma0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDma0 {
        match self.bits {
            false => IntEvent0RisDma0::IntEvent0RisDma0Clr,
            true => IntEvent0RisDma0::IntEvent0RisDma0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dma0_clr(&self) -> bool {
        *self == IntEvent0RisDma0::IntEvent0RisDma0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dma0_set(&self) -> bool {
        *self == IntEvent0RisDma0::IntEvent0RisDma0Set
    }
}
#[doc = "DMA1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDma1 {
    #[doc = "0: CLR"]
    IntEvent0RisDma1Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDma1Set = 1,
}
impl From<IntEvent0RisDma1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDma1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DMA1` reader - DMA1 event"]
pub type IntEvent0RisDma1R = crate::BitReader<IntEvent0RisDma1>;
impl IntEvent0RisDma1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDma1 {
        match self.bits {
            false => IntEvent0RisDma1::IntEvent0RisDma1Clr,
            true => IntEvent0RisDma1::IntEvent0RisDma1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dma1_clr(&self) -> bool {
        *self == IntEvent0RisDma1::IntEvent0RisDma1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dma1_set(&self) -> bool {
        *self == IntEvent0RisDma1::IntEvent0RisDma1Set
    }
}
#[doc = "DMA2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDma2 {
    #[doc = "0: CLR"]
    IntEvent0RisDma2Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDma2Set = 1,
}
impl From<IntEvent0RisDma2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDma2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DMA2` reader - DMA2 event"]
pub type IntEvent0RisDma2R = crate::BitReader<IntEvent0RisDma2>;
impl IntEvent0RisDma2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDma2 {
        match self.bits {
            false => IntEvent0RisDma2::IntEvent0RisDma2Clr,
            true => IntEvent0RisDma2::IntEvent0RisDma2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dma2_clr(&self) -> bool {
        *self == IntEvent0RisDma2::IntEvent0RisDma2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dma2_set(&self) -> bool {
        *self == IntEvent0RisDma2::IntEvent0RisDma2Set
    }
}
impl R {
    #[doc = "Bit 0 - AES ready interrupt, set when the selected AES operation was completed and the result can be read from AESADOUT."]
    #[inline(always)]
    pub fn int_event0_ris_aesrdy(&self) -> IntEvent0RisAesrdyR {
        IntEvent0RisAesrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA0 event"]
    #[inline(always)]
    pub fn int_event0_ris_dma0(&self) -> IntEvent0RisDma0R {
        IntEvent0RisDma0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA1 event"]
    #[inline(always)]
    pub fn int_event0_ris_dma1(&self) -> IntEvent0RisDma1R {
        IntEvent0RisDma1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA2 event"]
    #[inline(always)]
    pub fn int_event0_ris_dma2(&self) -> IntEvent0RisDma2R {
        IntEvent0RisDma2R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0RisSpec;
impl crate::RegisterSpec for IntEvent0RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event0_ris::R`](R) reader structure"]
impl crate::Readable for IntEvent0RisSpec {}
#[doc = "`reset()` method sets INT_EVENT0_RIS to value 0"]
impl crate::Resettable for IntEvent0RisSpec {
    const RESET_VALUE: u32 = 0;
}
