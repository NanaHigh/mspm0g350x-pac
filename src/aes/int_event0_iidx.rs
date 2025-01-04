#[doc = "Register `INT_EVENT0_IIDX` reader"]
pub type R = crate::R<IntEvent0IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntEvent0IidxStat {
    #[doc = "0: NO_INTR"]
    IntEvent0IidxStatNoIntr = 0,
    #[doc = "1: AESRDY"]
    IntEvent0IidxStatAesrdy = 1,
    #[doc = "2: DMA0"]
    IntEvent0IidxStatDma0 = 2,
    #[doc = "3: DMA1"]
    IntEvent0IidxStatDma1 = 3,
    #[doc = "4: DMA2"]
    IntEvent0IidxStatDma2 = 4,
}
impl From<IntEvent0IidxStat> for u8 {
    #[inline(always)]
    fn from(variant: IntEvent0IidxStat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntEvent0IidxStat {
    type Ux = u8;
}
impl crate::IsEnum for IntEvent0IidxStat {}
#[doc = "Field `INT_EVENT0_IIDX_STAT` reader - Interrupt index status"]
pub type IntEvent0IidxStatR = crate::FieldReader<IntEvent0IidxStat>;
impl IntEvent0IidxStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntEvent0IidxStat> {
        match self.bits {
            0 => Some(IntEvent0IidxStat::IntEvent0IidxStatNoIntr),
            1 => Some(IntEvent0IidxStat::IntEvent0IidxStatAesrdy),
            2 => Some(IntEvent0IidxStat::IntEvent0IidxStatDma0),
            3 => Some(IntEvent0IidxStat::IntEvent0IidxStatDma1),
            4 => Some(IntEvent0IidxStat::IntEvent0IidxStatDma2),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_no_intr(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatNoIntr
    }
    #[doc = "AESRDY"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_aesrdy(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatAesrdy
    }
    #[doc = "DMA0"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dma0(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDma0
    }
    #[doc = "DMA1"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dma1(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDma1
    }
    #[doc = "DMA2"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dma2(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDma2
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn int_event0_iidx_stat(&self) -> IntEvent0IidxStatR {
        IntEvent0IidxStatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0IidxSpec;
impl crate::RegisterSpec for IntEvent0IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event0_iidx::R`](R) reader structure"]
impl crate::Readable for IntEvent0IidxSpec {}
#[doc = "`reset()` method sets INT_EVENT0_IIDX to value 0"]
impl crate::Resettable for IntEvent0IidxSpec {
    const RESET_VALUE: u32 = 0;
}
