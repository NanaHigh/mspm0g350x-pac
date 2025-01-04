#[doc = "Register `INT_EVENT1_IIDX` reader"]
pub type R = crate::R<IntEvent1IidxSpec>;
#[doc = "I2C Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MISC. 15h-1Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntEvent1IidxStat {
    #[doc = "0: NO_INTR"]
    IntEvent1IidxStatNoIntr = 0,
    #[doc = "1: MRXFIFOTRG"]
    IntEvent1IidxStatMrxfifotrg = 1,
    #[doc = "2: MTXFIFOTRG"]
    IntEvent1IidxStatMtxfifotrg = 2,
    #[doc = "3: SRXFIFOTRG"]
    IntEvent1IidxStatSrxfifotrg = 3,
    #[doc = "4: STXFIFOTRG"]
    IntEvent1IidxStatStxfifotrg = 4,
}
impl From<IntEvent1IidxStat> for u8 {
    #[inline(always)]
    fn from(variant: IntEvent1IidxStat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntEvent1IidxStat {
    type Ux = u8;
}
impl crate::IsEnum for IntEvent1IidxStat {}
#[doc = "Field `INT_EVENT1_IIDX_STAT` reader - I2C Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MISC. 15h-1Fh = Reserved"]
pub type IntEvent1IidxStatR = crate::FieldReader<IntEvent1IidxStat>;
impl IntEvent1IidxStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntEvent1IidxStat> {
        match self.bits {
            0 => Some(IntEvent1IidxStat::IntEvent1IidxStatNoIntr),
            1 => Some(IntEvent1IidxStat::IntEvent1IidxStatMrxfifotrg),
            2 => Some(IntEvent1IidxStat::IntEvent1IidxStatMtxfifotrg),
            3 => Some(IntEvent1IidxStat::IntEvent1IidxStatSrxfifotrg),
            4 => Some(IntEvent1IidxStat::IntEvent1IidxStatStxfifotrg),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_no_intr(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatNoIntr
    }
    #[doc = "MRXFIFOTRG"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_mrxfifotrg(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatMrxfifotrg
    }
    #[doc = "MTXFIFOTRG"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_mtxfifotrg(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatMtxfifotrg
    }
    #[doc = "SRXFIFOTRG"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_srxfifotrg(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatSrxfifotrg
    }
    #[doc = "STXFIFOTRG"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_stxfifotrg(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatStxfifotrg
    }
}
impl R {
    #[doc = "Bits 0:7 - I2C Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in RIS and MISC. 15h-1Fh = Reserved"]
    #[inline(always)]
    pub fn int_event1_iidx_stat(&self) -> IntEvent1IidxStatR {
        IntEvent1IidxStatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1IidxSpec;
impl crate::RegisterSpec for IntEvent1IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event1_iidx::R`](R) reader structure"]
impl crate::Readable for IntEvent1IidxSpec {}
#[doc = "`reset()` method sets INT_EVENT1_IIDX to value 0"]
impl crate::Resettable for IntEvent1IidxSpec {
    const RESET_VALUE: u32 = 0;
}
