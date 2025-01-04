#[doc = "Register `INT_EVENT2_MIS` reader"]
pub type R = crate::R<IntEvent2MisSpec>;
#[doc = "Masked UART Transmit Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisTxint {
    #[doc = "0: CLR"]
    IntEvent2MisTxintClr = 0,
    #[doc = "1: SET"]
    IntEvent2MisTxintSet = 1,
}
impl From<IntEvent2MisTxint> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisTxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_TXINT` reader - Masked UART Transmit Interrupt."]
pub type IntEvent2MisTxintR = crate::BitReader<IntEvent2MisTxint>;
impl IntEvent2MisTxintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisTxint {
        match self.bits {
            false => IntEvent2MisTxint::IntEvent2MisTxintClr,
            true => IntEvent2MisTxint::IntEvent2MisTxintSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_txint_clr(&self) -> bool {
        *self == IntEvent2MisTxint::IntEvent2MisTxintClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_txint_set(&self) -> bool {
        *self == IntEvent2MisTxint::IntEvent2MisTxintSet
    }
}
impl R {
    #[doc = "Bit 11 - Masked UART Transmit Interrupt."]
    #[inline(always)]
    pub fn int_event2_mis_txint(&self) -> IntEvent2MisTxintR {
        IntEvent2MisTxintR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2MisSpec;
impl crate::RegisterSpec for IntEvent2MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event2_mis::R`](R) reader structure"]
impl crate::Readable for IntEvent2MisSpec {}
#[doc = "`reset()` method sets INT_EVENT2_MIS to value 0"]
impl crate::Resettable for IntEvent2MisSpec {
    const RESET_VALUE: u32 = 0;
}
