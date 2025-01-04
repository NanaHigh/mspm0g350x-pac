#[doc = "Register `INT_EVENT1_RIS` reader"]
pub type R = crate::R<IntEvent1RisSpec>;
#[doc = "UARTOUT Receive Time-Out Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisRtout {
    #[doc = "0: CLR"]
    IntEvent1RisRtoutClr = 0,
    #[doc = "1: SET"]
    IntEvent1RisRtoutSet = 1,
}
impl From<IntEvent1RisRtout> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisRtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_RTOUT` reader - UARTOUT Receive Time-Out Interrupt."]
pub type IntEvent1RisRtoutR = crate::BitReader<IntEvent1RisRtout>;
impl IntEvent1RisRtoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisRtout {
        match self.bits {
            false => IntEvent1RisRtout::IntEvent1RisRtoutClr,
            true => IntEvent1RisRtout::IntEvent1RisRtoutSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_rtout_clr(&self) -> bool {
        *self == IntEvent1RisRtout::IntEvent1RisRtoutClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_rtout_set(&self) -> bool {
        *self == IntEvent1RisRtout::IntEvent1RisRtoutSet
    }
}
#[doc = "UART Receive Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1RisRxint {
    #[doc = "0: CLR"]
    IntEvent1RisRxintClr = 0,
    #[doc = "1: SET"]
    IntEvent1RisRxintSet = 1,
}
impl From<IntEvent1RisRxint> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1RisRxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_RIS_RXINT` reader - UART Receive Interrupt."]
pub type IntEvent1RisRxintR = crate::BitReader<IntEvent1RisRxint>;
impl IntEvent1RisRxintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1RisRxint {
        match self.bits {
            false => IntEvent1RisRxint::IntEvent1RisRxintClr,
            true => IntEvent1RisRxint::IntEvent1RisRxintSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_ris_rxint_clr(&self) -> bool {
        *self == IntEvent1RisRxint::IntEvent1RisRxintClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_ris_rxint_set(&self) -> bool {
        *self == IntEvent1RisRxint::IntEvent1RisRxintSet
    }
}
impl R {
    #[doc = "Bit 0 - UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn int_event1_ris_rtout(&self) -> IntEvent1RisRtoutR {
        IntEvent1RisRtoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - UART Receive Interrupt."]
    #[inline(always)]
    pub fn int_event1_ris_rxint(&self) -> IntEvent1RisRxintR {
        IntEvent1RisRxintR::new(((self.bits >> 10) & 1) != 0)
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
