#[doc = "Register `INT_EVENT2_RIS` reader"]
pub type R = crate::R<IntEvent2RisSpec>;
#[doc = "Transmit FIFO event: A read returns the current mask for transmit FIFO interrupt. On a write of 1, the mask for transmit FIFO interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisTx {
    #[doc = "0: CLR"]
    IntEvent2RisTxClr = 0,
    #[doc = "1: SET"]
    IntEvent2RisTxSet = 1,
}
impl From<IntEvent2RisTx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_TX` reader - Transmit FIFO event: A read returns the current mask for transmit FIFO interrupt. On a write of 1, the mask for transmit FIFO interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
pub type IntEvent2RisTxR = crate::BitReader<IntEvent2RisTx>;
impl IntEvent2RisTxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisTx {
        match self.bits {
            false => IntEvent2RisTx::IntEvent2RisTxClr,
            true => IntEvent2RisTx::IntEvent2RisTxSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_tx_clr(&self) -> bool {
        *self == IntEvent2RisTx::IntEvent2RisTxClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_tx_set(&self) -> bool {
        *self == IntEvent2RisTx::IntEvent2RisTxSet
    }
}
impl R {
    #[doc = "Bit 4 - Transmit FIFO event: A read returns the current mask for transmit FIFO interrupt. On a write of 1, the mask for transmit FIFO interrupt is set which means the interrupt state will be reflected in MIS.TXMIS. A write of 0 clears the mask which means MIS.TXMIS will not reflect the interrupt."]
    #[inline(always)]
    pub fn int_event2_ris_tx(&self) -> IntEvent2RisTxR {
        IntEvent2RisTxR::new(((self.bits >> 4) & 1) != 0)
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
