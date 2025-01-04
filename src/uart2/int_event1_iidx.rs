#[doc = "Register `INT_EVENT1_IIDX` reader"]
pub type R = crate::R<IntEvent1IidxSpec>;
#[doc = "UART Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in UARTRIS and UARTMISC. 15h-1Fh = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntEvent1IidxStat {
    #[doc = "0: NO_INTR"]
    IntEvent1IidxStatNoIntr = 0,
    #[doc = "1: RTFG"]
    IntEvent1IidxStatRtfg = 1,
    #[doc = "11: RXIFG"]
    IntEvent1IidxStatRxifg = 11,
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
#[doc = "Field `INT_EVENT1_IIDX_STAT` reader - UART Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in UARTRIS and UARTMISC. 15h-1Fh = Reserved"]
pub type IntEvent1IidxStatR = crate::FieldReader<IntEvent1IidxStat>;
impl IntEvent1IidxStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntEvent1IidxStat> {
        match self.bits {
            0 => Some(IntEvent1IidxStat::IntEvent1IidxStatNoIntr),
            1 => Some(IntEvent1IidxStat::IntEvent1IidxStatRtfg),
            11 => Some(IntEvent1IidxStat::IntEvent1IidxStatRxifg),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_no_intr(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatNoIntr
    }
    #[doc = "RTFG"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_rtfg(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatRtfg
    }
    #[doc = "RXIFG"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_rxifg(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatRxifg
    }
}
impl R {
    #[doc = "Bits 0:7 - UART Module Interrupt Vector Value. This register provides the highes priority interrupt index. A read clears the corresponding interrupt flag in UARTRIS and UARTMISC. 15h-1Fh = Reserved"]
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
