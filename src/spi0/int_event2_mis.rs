#[doc = "Register `INT_EVENT2_MIS` reader"]
pub type R = crate::R<IntEvent2MisSpec>;
#[doc = "Masked Transmit FIFO event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisTx {
    #[doc = "0: CLR"]
    IntEvent2MisTxClr = 0,
    #[doc = "1: SET"]
    IntEvent2MisTxSet = 1,
}
impl From<IntEvent2MisTx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_TX` reader - Masked Transmit FIFO event"]
pub type IntEvent2MisTxR = crate::BitReader<IntEvent2MisTx>;
impl IntEvent2MisTxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisTx {
        match self.bits {
            false => IntEvent2MisTx::IntEvent2MisTxClr,
            true => IntEvent2MisTx::IntEvent2MisTxSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_tx_clr(&self) -> bool {
        *self == IntEvent2MisTx::IntEvent2MisTxClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_tx_set(&self) -> bool {
        *self == IntEvent2MisTx::IntEvent2MisTxSet
    }
}
impl R {
    #[doc = "Bit 4 - Masked Transmit FIFO event"]
    #[inline(always)]
    pub fn int_event2_mis_tx(&self) -> IntEvent2MisTxR {
        IntEvent2MisTxR::new(((self.bits >> 4) & 1) != 0)
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
