#[doc = "Register `INT_EVENT2_RIS` reader"]
pub type R = crate::R<IntEvent2RisSpec>;
#[doc = "UART Transmit Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisTxint {
    #[doc = "0: CLR"]
    IntEvent2RisTxintClr = 0,
    #[doc = "1: SET"]
    IntEvent2RisTxintSet = 1,
}
impl From<IntEvent2RisTxint> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisTxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_TXINT` reader - UART Transmit Interrupt."]
pub type IntEvent2RisTxintR = crate::BitReader<IntEvent2RisTxint>;
impl IntEvent2RisTxintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisTxint {
        match self.bits {
            false => IntEvent2RisTxint::IntEvent2RisTxintClr,
            true => IntEvent2RisTxint::IntEvent2RisTxintSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_txint_clr(&self) -> bool {
        *self == IntEvent2RisTxint::IntEvent2RisTxintClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_txint_set(&self) -> bool {
        *self == IntEvent2RisTxint::IntEvent2RisTxintSet
    }
}
impl R {
    #[doc = "Bit 11 - UART Transmit Interrupt."]
    #[inline(always)]
    pub fn int_event2_ris_txint(&self) -> IntEvent2RisTxintR {
        IntEvent2RisTxintR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2RisSpec;
impl crate::RegisterSpec for IntEvent2RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event2_ris::R`](R) reader structure"]
impl crate::Readable for IntEvent2RisSpec {}
#[doc = "`reset()` method sets INT_EVENT2_RIS to value 0"]
impl crate::Resettable for IntEvent2RisSpec {
    const RESET_VALUE: u32 = 0;
}
