#[doc = "Register `INT_EVENT3_IIDX` reader"]
pub type R = crate::R<IntEvent3IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntEvent3IidxStat {
    #[doc = "0: NO_INTR"]
    IntEvent3IidxStatNoIntr = 0,
    #[doc = "4: DMA2"]
    IntEvent3IidxStatDma2 = 4,
}
impl From<IntEvent3IidxStat> for u8 {
    #[inline(always)]
    fn from(variant: IntEvent3IidxStat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntEvent3IidxStat {
    type Ux = u8;
}
impl crate::IsEnum for IntEvent3IidxStat {}
#[doc = "Field `INT_EVENT3_IIDX_STAT` reader - Interrupt index status"]
pub type IntEvent3IidxStatR = crate::FieldReader<IntEvent3IidxStat>;
impl IntEvent3IidxStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntEvent3IidxStat> {
        match self.bits {
            0 => Some(IntEvent3IidxStat::IntEvent3IidxStatNoIntr),
            4 => Some(IntEvent3IidxStat::IntEvent3IidxStatDma2),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_int_event3_iidx_stat_no_intr(&self) -> bool {
        *self == IntEvent3IidxStat::IntEvent3IidxStatNoIntr
    }
    #[doc = "DMA2"]
    #[inline(always)]
    pub fn is_int_event3_iidx_stat_dma2(&self) -> bool {
        *self == IntEvent3IidxStat::IntEvent3IidxStatDma2
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn int_event3_iidx_stat(&self) -> IntEvent3IidxStatR {
        IntEvent3IidxStatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event3_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent3IidxSpec;
impl crate::RegisterSpec for IntEvent3IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event3_iidx::R`](R) reader structure"]
impl crate::Readable for IntEvent3IidxSpec {}
#[doc = "`reset()` method sets INT_EVENT3_IIDX to value 0"]
impl crate::Resettable for IntEvent3IidxSpec {
    const RESET_VALUE: u32 = 0;
}
