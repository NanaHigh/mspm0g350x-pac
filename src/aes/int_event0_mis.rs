#[doc = "Register `INT_EVENT0_MIS` reader"]
pub type R = crate::R<IntEvent0MisSpec>;
#[doc = "AES ready interrupt, set when the selected AES operation was completed and the result can be read from AESADOUT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisAesrdy {
    #[doc = "0: CLR"]
    IntEvent0MisAesrdyClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisAesrdySet = 1,
}
impl From<IntEvent0MisAesrdy> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisAesrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_AESRDY` reader - AES ready interrupt, set when the selected AES operation was completed and the result can be read from AESADOUT."]
pub type IntEvent0MisAesrdyR = crate::BitReader<IntEvent0MisAesrdy>;
impl IntEvent0MisAesrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisAesrdy {
        match self.bits {
            false => IntEvent0MisAesrdy::IntEvent0MisAesrdyClr,
            true => IntEvent0MisAesrdy::IntEvent0MisAesrdySet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_aesrdy_clr(&self) -> bool {
        *self == IntEvent0MisAesrdy::IntEvent0MisAesrdyClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_aesrdy_set(&self) -> bool {
        *self == IntEvent0MisAesrdy::IntEvent0MisAesrdySet
    }
}
#[doc = "DMA0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDma0 {
    #[doc = "0: CLR"]
    IntEvent0MisDma0Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDma0Set = 1,
}
impl From<IntEvent0MisDma0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDma0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DMA0` reader - DMA0 event"]
pub type IntEvent0MisDma0R = crate::BitReader<IntEvent0MisDma0>;
impl IntEvent0MisDma0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDma0 {
        match self.bits {
            false => IntEvent0MisDma0::IntEvent0MisDma0Clr,
            true => IntEvent0MisDma0::IntEvent0MisDma0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dma0_clr(&self) -> bool {
        *self == IntEvent0MisDma0::IntEvent0MisDma0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dma0_set(&self) -> bool {
        *self == IntEvent0MisDma0::IntEvent0MisDma0Set
    }
}
#[doc = "DMA1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDma1 {
    #[doc = "0: CLR"]
    IntEvent0MisDma1Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDma1Set = 1,
}
impl From<IntEvent0MisDma1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDma1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DMA1` reader - DMA1 event"]
pub type IntEvent0MisDma1R = crate::BitReader<IntEvent0MisDma1>;
impl IntEvent0MisDma1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDma1 {
        match self.bits {
            false => IntEvent0MisDma1::IntEvent0MisDma1Clr,
            true => IntEvent0MisDma1::IntEvent0MisDma1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dma1_clr(&self) -> bool {
        *self == IntEvent0MisDma1::IntEvent0MisDma1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dma1_set(&self) -> bool {
        *self == IntEvent0MisDma1::IntEvent0MisDma1Set
    }
}
#[doc = "DMA2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDma2 {
    #[doc = "0: CLR"]
    IntEvent0MisDma2Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDma2Set = 1,
}
impl From<IntEvent0MisDma2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDma2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DMA2` reader - DMA2 event"]
pub type IntEvent0MisDma2R = crate::BitReader<IntEvent0MisDma2>;
impl IntEvent0MisDma2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDma2 {
        match self.bits {
            false => IntEvent0MisDma2::IntEvent0MisDma2Clr,
            true => IntEvent0MisDma2::IntEvent0MisDma2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dma2_clr(&self) -> bool {
        *self == IntEvent0MisDma2::IntEvent0MisDma2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dma2_set(&self) -> bool {
        *self == IntEvent0MisDma2::IntEvent0MisDma2Set
    }
}
impl R {
    #[doc = "Bit 0 - AES ready interrupt, set when the selected AES operation was completed and the result can be read from AESADOUT."]
    #[inline(always)]
    pub fn int_event0_mis_aesrdy(&self) -> IntEvent0MisAesrdyR {
        IntEvent0MisAesrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA0 event"]
    #[inline(always)]
    pub fn int_event0_mis_dma0(&self) -> IntEvent0MisDma0R {
        IntEvent0MisDma0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA1 event"]
    #[inline(always)]
    pub fn int_event0_mis_dma1(&self) -> IntEvent0MisDma1R {
        IntEvent0MisDma1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA2 event"]
    #[inline(always)]
    pub fn int_event0_mis_dma2(&self) -> IntEvent0MisDma2R {
        IntEvent0MisDma2R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0MisSpec;
impl crate::RegisterSpec for IntEvent0MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event0_mis::R`](R) reader structure"]
impl crate::Readable for IntEvent0MisSpec {}
#[doc = "`reset()` method sets INT_EVENT0_MIS to value 0"]
impl crate::Resettable for IntEvent0MisSpec {
    const RESET_VALUE: u32 = 0;
}
