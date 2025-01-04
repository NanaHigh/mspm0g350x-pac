#[doc = "Register `INT_EVENT1_MIS` reader"]
pub type R = crate::R<IntEvent1MisSpec>;
#[doc = "Masked UARTOUT Receive Time-Out Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisRtout {
    #[doc = "0: CLR"]
    IntEvent1MisRtoutClr = 0,
    #[doc = "1: SET"]
    IntEvent1MisRtoutSet = 1,
}
impl From<IntEvent1MisRtout> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisRtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_RTOUT` reader - Masked UARTOUT Receive Time-Out Interrupt."]
pub type IntEvent1MisRtoutR = crate::BitReader<IntEvent1MisRtout>;
impl IntEvent1MisRtoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisRtout {
        match self.bits {
            false => IntEvent1MisRtout::IntEvent1MisRtoutClr,
            true => IntEvent1MisRtout::IntEvent1MisRtoutSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_rtout_clr(&self) -> bool {
        *self == IntEvent1MisRtout::IntEvent1MisRtoutClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_rtout_set(&self) -> bool {
        *self == IntEvent1MisRtout::IntEvent1MisRtoutSet
    }
}
#[doc = "Masked UART Receive Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1MisRxint {
    #[doc = "0: CLR"]
    IntEvent1MisRxintClr = 0,
    #[doc = "1: SET"]
    IntEvent1MisRxintSet = 1,
}
impl From<IntEvent1MisRxint> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1MisRxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_MIS_RXINT` reader - Masked UART Receive Interrupt."]
pub type IntEvent1MisRxintR = crate::BitReader<IntEvent1MisRxint>;
impl IntEvent1MisRxintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1MisRxint {
        match self.bits {
            false => IntEvent1MisRxint::IntEvent1MisRxintClr,
            true => IntEvent1MisRxint::IntEvent1MisRxintSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_mis_rxint_clr(&self) -> bool {
        *self == IntEvent1MisRxint::IntEvent1MisRxintClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_mis_rxint_set(&self) -> bool {
        *self == IntEvent1MisRxint::IntEvent1MisRxintSet
    }
}
impl R {
    #[doc = "Bit 0 - Masked UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn int_event1_mis_rtout(&self) -> IntEvent1MisRtoutR {
        IntEvent1MisRtoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - Masked UART Receive Interrupt."]
    #[inline(always)]
    pub fn int_event1_mis_rxint(&self) -> IntEvent1MisRxintR {
        IntEvent1MisRxintR::new(((self.bits >> 10) & 1) != 0)
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
