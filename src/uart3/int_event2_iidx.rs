#[doc = "Register `INT_EVENT2_IIDX` reader"]
pub type R = crate::R<IntEvent2IidxSpec>;
#[doc = "UART Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in UARTRIS and UARTMISC. 15h-1Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntEvent2IidxStat {
    #[doc = "0: NO_INTR"]
    IntEvent2IidxStatNoIntr = 0,
    #[doc = "12: TXIFG"]
    IntEvent2IidxStatTxifg = 12,
}
impl From<IntEvent2IidxStat> for u8 {
    #[inline(always)]
    fn from(variant: IntEvent2IidxStat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntEvent2IidxStat {
    type Ux = u8;
}
impl crate::IsEnum for IntEvent2IidxStat {}
#[doc = "Field `INT_EVENT2_IIDX_STAT` reader - UART Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in UARTRIS and UARTMISC. 15h-1Fh = Reserved"]
pub type IntEvent2IidxStatR = crate::FieldReader<IntEvent2IidxStat>;
impl IntEvent2IidxStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntEvent2IidxStat> {
        match self.bits {
            0 => Some(IntEvent2IidxStat::IntEvent2IidxStatNoIntr),
            12 => Some(IntEvent2IidxStat::IntEvent2IidxStatTxifg),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_no_intr(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatNoIntr
    }
    #[doc = "TXIFG"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_txifg(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatTxifg
    }
}
impl R {
    #[doc = "Bits 0:7 - UART Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in UARTRIS and UARTMISC. 15h-1Fh = Reserved"]
    #[inline(always)]
    pub fn int_event2_iidx_stat(&self) -> IntEvent2IidxStatR {
        IntEvent2IidxStatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2IidxSpec;
impl crate::RegisterSpec for IntEvent2IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event2_iidx::R`](R) reader structure"]
impl crate::Readable for IntEvent2IidxSpec {}
#[doc = "`reset()` method sets INT_EVENT2_IIDX to value 0"]
impl crate::Resettable for IntEvent2IidxSpec {
    const RESET_VALUE: u32 = 0;
}
