#[doc = "Register `INT_EVENT0_IMASK` reader"]
pub type R = crate::R<IntEvent0ImaskSpec>;
#[doc = "Register `INT_EVENT0_IMASK` writer"]
pub type W = crate::W<IntEvent0ImaskSpec>;
#[doc = "AES ready interrupt, set when the selected AES operation was completed and the result can be read from AESADOUT.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskAesrdy {
    #[doc = "0: CLR"]
    IntEvent0ImaskAesrdyClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskAesrdySet = 1,
}
impl From<IntEvent0ImaskAesrdy> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskAesrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_AESRDY` reader - AES ready interrupt, set when the selected AES operation was completed and the result can be read from AESADOUT."]
pub type IntEvent0ImaskAesrdyR = crate::BitReader<IntEvent0ImaskAesrdy>;
impl IntEvent0ImaskAesrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskAesrdy {
        match self.bits {
            false => IntEvent0ImaskAesrdy::IntEvent0ImaskAesrdyClr,
            true => IntEvent0ImaskAesrdy::IntEvent0ImaskAesrdySet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_aesrdy_clr(&self) -> bool {
        *self == IntEvent0ImaskAesrdy::IntEvent0ImaskAesrdyClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_aesrdy_set(&self) -> bool {
        *self == IntEvent0ImaskAesrdy::IntEvent0ImaskAesrdySet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_AESRDY` writer - AES ready interrupt, set when the selected AES operation was completed and the result can be read from AESADOUT."]
pub type IntEvent0ImaskAesrdyW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskAesrdy>;
impl<'a, REG> IntEvent0ImaskAesrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_aesrdy_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskAesrdy::IntEvent0ImaskAesrdyClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_aesrdy_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskAesrdy::IntEvent0ImaskAesrdySet)
    }
}
#[doc = "DMA0 event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDma0 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDma0Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDma0Set = 1,
}
impl From<IntEvent0ImaskDma0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDma0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DMA0` reader - DMA0 event mask."]
pub type IntEvent0ImaskDma0R = crate::BitReader<IntEvent0ImaskDma0>;
impl IntEvent0ImaskDma0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDma0 {
        match self.bits {
            false => IntEvent0ImaskDma0::IntEvent0ImaskDma0Clr,
            true => IntEvent0ImaskDma0::IntEvent0ImaskDma0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dma0_clr(&self) -> bool {
        *self == IntEvent0ImaskDma0::IntEvent0ImaskDma0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dma0_set(&self) -> bool {
        *self == IntEvent0ImaskDma0::IntEvent0ImaskDma0Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DMA0` writer - DMA0 event mask."]
pub type IntEvent0ImaskDma0W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDma0>;
impl<'a, REG> IntEvent0ImaskDma0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dma0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDma0::IntEvent0ImaskDma0Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dma0_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDma0::IntEvent0ImaskDma0Set)
    }
}
#[doc = "DMA1 event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDma1 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDma1Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDma1Set = 1,
}
impl From<IntEvent0ImaskDma1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDma1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DMA1` reader - DMA1 event mask."]
pub type IntEvent0ImaskDma1R = crate::BitReader<IntEvent0ImaskDma1>;
impl IntEvent0ImaskDma1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDma1 {
        match self.bits {
            false => IntEvent0ImaskDma1::IntEvent0ImaskDma1Clr,
            true => IntEvent0ImaskDma1::IntEvent0ImaskDma1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dma1_clr(&self) -> bool {
        *self == IntEvent0ImaskDma1::IntEvent0ImaskDma1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dma1_set(&self) -> bool {
        *self == IntEvent0ImaskDma1::IntEvent0ImaskDma1Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DMA1` writer - DMA1 event mask."]
pub type IntEvent0ImaskDma1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDma1>;
impl<'a, REG> IntEvent0ImaskDma1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dma1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDma1::IntEvent0ImaskDma1Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dma1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDma1::IntEvent0ImaskDma1Set)
    }
}
#[doc = "DMA2 event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDma2 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDma2Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDma2Set = 1,
}
impl From<IntEvent0ImaskDma2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDma2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DMA2` reader - DMA2 event mask."]
pub type IntEvent0ImaskDma2R = crate::BitReader<IntEvent0ImaskDma2>;
impl IntEvent0ImaskDma2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDma2 {
        match self.bits {
            false => IntEvent0ImaskDma2::IntEvent0ImaskDma2Clr,
            true => IntEvent0ImaskDma2::IntEvent0ImaskDma2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dma2_clr(&self) -> bool {
        *self == IntEvent0ImaskDma2::IntEvent0ImaskDma2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dma2_set(&self) -> bool {
        *self == IntEvent0ImaskDma2::IntEvent0ImaskDma2Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DMA2` writer - DMA2 event mask."]
pub type IntEvent0ImaskDma2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDma2>;
impl<'a, REG> IntEvent0ImaskDma2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dma2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDma2::IntEvent0ImaskDma2Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dma2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDma2::IntEvent0ImaskDma2Set)
    }
}
impl R {
    #[doc = "Bit 0 - AES ready interrupt, set when the selected AES operation was completed and the result can be read from AESADOUT."]
    #[inline(always)]
    pub fn int_event0_imask_aesrdy(&self) -> IntEvent0ImaskAesrdyR {
        IntEvent0ImaskAesrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA0 event mask."]
    #[inline(always)]
    pub fn int_event0_imask_dma0(&self) -> IntEvent0ImaskDma0R {
        IntEvent0ImaskDma0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA1 event mask."]
    #[inline(always)]
    pub fn int_event0_imask_dma1(&self) -> IntEvent0ImaskDma1R {
        IntEvent0ImaskDma1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA2 event mask."]
    #[inline(always)]
    pub fn int_event0_imask_dma2(&self) -> IntEvent0ImaskDma2R {
        IntEvent0ImaskDma2R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES ready interrupt, set when the selected AES operation was completed and the result can be read from AESADOUT."]
    #[inline(always)]
    pub fn int_event0_imask_aesrdy(&mut self) -> IntEvent0ImaskAesrdyW<IntEvent0ImaskSpec> {
        IntEvent0ImaskAesrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA0 event mask."]
    #[inline(always)]
    pub fn int_event0_imask_dma0(&mut self) -> IntEvent0ImaskDma0W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDma0W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA1 event mask."]
    #[inline(always)]
    pub fn int_event0_imask_dma1(&mut self) -> IntEvent0ImaskDma1W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDma1W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA2 event mask."]
    #[inline(always)]
    pub fn int_event0_imask_dma2(&mut self) -> IntEvent0ImaskDma2W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDma2W::new(self, 3)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event0_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0ImaskSpec;
impl crate::RegisterSpec for IntEvent0ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event0_imask::R`](R) reader structure"]
impl crate::Readable for IntEvent0ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_event0_imask::W`](W) writer structure"]
impl crate::Writable for IntEvent0ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT0_IMASK to value 0"]
impl crate::Resettable for IntEvent0ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
