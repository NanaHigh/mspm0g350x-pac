#[doc = "Register `INT_EVENT0_RIS` reader"]
pub type R = crate::R<IntEvent0RisSpec>;
#[doc = "RXFIFO overflow event. This interrupt is set if an RX FIFO overflow has been detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisRxfifoOvf {
    #[doc = "0: CLR"]
    IntEvent0RisRxfifoOvfClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisRxfifoOvfSet = 1,
}
impl From<IntEvent0RisRxfifoOvf> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisRxfifoOvf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_RXFIFO_OVF` reader - RXFIFO overflow event. This interrupt is set if an RX FIFO overflow has been detected."]
pub type IntEvent0RisRxfifoOvfR = crate::BitReader<IntEvent0RisRxfifoOvf>;
impl IntEvent0RisRxfifoOvfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisRxfifoOvf {
        match self.bits {
            false => IntEvent0RisRxfifoOvf::IntEvent0RisRxfifoOvfClr,
            true => IntEvent0RisRxfifoOvf::IntEvent0RisRxfifoOvfSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_rxfifo_ovf_clr(&self) -> bool {
        *self == IntEvent0RisRxfifoOvf::IntEvent0RisRxfifoOvfClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_rxfifo_ovf_set(&self) -> bool {
        *self == IntEvent0RisRxfifoOvf::IntEvent0RisRxfifoOvfSet
    }
}
#[doc = "Parity error event: this bit is set if a Parity error has been detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisPer {
    #[doc = "0: CLR"]
    IntEvent0RisPerClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisPerSet = 1,
}
impl From<IntEvent0RisPer> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisPer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_PER` reader - Parity error event: this bit is set if a Parity error has been detected"]
pub type IntEvent0RisPerR = crate::BitReader<IntEvent0RisPer>;
impl IntEvent0RisPerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisPer {
        match self.bits {
            false => IntEvent0RisPer::IntEvent0RisPerClr,
            true => IntEvent0RisPer::IntEvent0RisPerSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_per_clr(&self) -> bool {
        *self == IntEvent0RisPer::IntEvent0RisPerClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_per_set(&self) -> bool {
        *self == IntEvent0RisPer::IntEvent0RisPerSet
    }
}
#[doc = "SPI Receive Time-Out event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisRtout {
    #[doc = "0: CLR"]
    IntEvent0RisRtoutClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisRtoutSet = 1,
}
impl From<IntEvent0RisRtout> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisRtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_RTOUT` reader - SPI Receive Time-Out event."]
pub type IntEvent0RisRtoutR = crate::BitReader<IntEvent0RisRtout>;
impl IntEvent0RisRtoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisRtout {
        match self.bits {
            false => IntEvent0RisRtout::IntEvent0RisRtoutClr,
            true => IntEvent0RisRtout::IntEvent0RisRtoutSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_rtout_clr(&self) -> bool {
        *self == IntEvent0RisRtout::IntEvent0RisRtoutClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_rtout_set(&self) -> bool {
        *self == IntEvent0RisRtout::IntEvent0RisRtoutSet
    }
}
#[doc = "Receive FIFO event.This interrupt is set if the selected Receive FIFO level has been reached\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisRx {
    #[doc = "0: CLR"]
    IntEvent0RisRxClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisRxSet = 1,
}
impl From<IntEvent0RisRx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_RX` reader - Receive FIFO event.This interrupt is set if the selected Receive FIFO level has been reached"]
pub type IntEvent0RisRxR = crate::BitReader<IntEvent0RisRx>;
impl IntEvent0RisRxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisRx {
        match self.bits {
            false => IntEvent0RisRx::IntEvent0RisRxClr,
            true => IntEvent0RisRx::IntEvent0RisRxSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_rx_clr(&self) -> bool {
        *self == IntEvent0RisRx::IntEvent0RisRxClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_rx_set(&self) -> bool {
        *self == IntEvent0RisRx::IntEvent0RisRxSet
    }
}
#[doc = "Transmit FIFO event..This interrupt is set if the selected Transmit FIFO level has been reached.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisTx {
    #[doc = "0: CLR"]
    IntEvent0RisTxClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisTxSet = 1,
}
impl From<IntEvent0RisTx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_TX` reader - Transmit FIFO event..This interrupt is set if the selected Transmit FIFO level has been reached."]
pub type IntEvent0RisTxR = crate::BitReader<IntEvent0RisTx>;
impl IntEvent0RisTxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisTx {
        match self.bits {
            false => IntEvent0RisTx::IntEvent0RisTxClr,
            true => IntEvent0RisTx::IntEvent0RisTxSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_tx_clr(&self) -> bool {
        *self == IntEvent0RisTx::IntEvent0RisTxClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_tx_set(&self) -> bool {
        *self == IntEvent0RisTx::IntEvent0RisTxSet
    }
}
#[doc = "Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been move to the shift register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisTxempty {
    #[doc = "0: CLR"]
    IntEvent0RisTxemptyClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisTxemptySet = 1,
}
impl From<IntEvent0RisTxempty> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisTxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_TXEMPTY` reader - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been move to the shift register."]
pub type IntEvent0RisTxemptyR = crate::BitReader<IntEvent0RisTxempty>;
impl IntEvent0RisTxemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisTxempty {
        match self.bits {
            false => IntEvent0RisTxempty::IntEvent0RisTxemptyClr,
            true => IntEvent0RisTxempty::IntEvent0RisTxemptySet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_txempty_clr(&self) -> bool {
        *self == IntEvent0RisTxempty::IntEvent0RisTxemptyClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_txempty_set(&self) -> bool {
        *self == IntEvent0RisTxempty::IntEvent0RisTxemptySet
    }
}
#[doc = "SPI has done finished transfers and changed into IDLE mode. This bit is set when BUSY goes low.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisIdle {
    #[doc = "0: CLR"]
    IntEvent0RisIdleClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisIdleSet = 1,
}
impl From<IntEvent0RisIdle> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisIdle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_IDLE` reader - SPI has done finished transfers and changed into IDLE mode. This bit is set when BUSY goes low."]
pub type IntEvent0RisIdleR = crate::BitReader<IntEvent0RisIdle>;
impl IntEvent0RisIdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisIdle {
        match self.bits {
            false => IntEvent0RisIdle::IntEvent0RisIdleClr,
            true => IntEvent0RisIdle::IntEvent0RisIdleSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_idle_clr(&self) -> bool {
        *self == IntEvent0RisIdle::IntEvent0RisIdleClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_idle_set(&self) -> bool {
        *self == IntEvent0RisIdle::IntEvent0RisIdleSet
    }
}
#[doc = "DMA Done 1 event for RX. This interrupt is set if the RX DMA channel sends the DONE signal. This allows the handling of the DMA event inside the mapped peripheral.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDmaDoneRx {
    #[doc = "0: CLR"]
    IntEvent0RisDmaDoneRxClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDmaDoneRxSet = 1,
}
impl From<IntEvent0RisDmaDoneRx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDmaDoneRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DMA_DONE_RX` reader - DMA Done 1 event for RX. This interrupt is set if the RX DMA channel sends the DONE signal. This allows the handling of the DMA event inside the mapped peripheral."]
pub type IntEvent0RisDmaDoneRxR = crate::BitReader<IntEvent0RisDmaDoneRx>;
impl IntEvent0RisDmaDoneRxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDmaDoneRx {
        match self.bits {
            false => IntEvent0RisDmaDoneRx::IntEvent0RisDmaDoneRxClr,
            true => IntEvent0RisDmaDoneRx::IntEvent0RisDmaDoneRxSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dma_done_rx_clr(&self) -> bool {
        *self == IntEvent0RisDmaDoneRx::IntEvent0RisDmaDoneRxClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dma_done_rx_set(&self) -> bool {
        *self == IntEvent0RisDmaDoneRx::IntEvent0RisDmaDoneRxSet
    }
}
#[doc = "DMA Done 1 event for TX. This interrupt is set if the TX DMA channel sends the DONE signal. This allows the handling of the DMA event inside the mapped peripheral.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisDmaDoneTx {
    #[doc = "0: CLR"]
    IntEvent0RisDmaDoneTxClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisDmaDoneTxSet = 1,
}
impl From<IntEvent0RisDmaDoneTx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisDmaDoneTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_DMA_DONE_TX` reader - DMA Done 1 event for TX. This interrupt is set if the TX DMA channel sends the DONE signal. This allows the handling of the DMA event inside the mapped peripheral."]
pub type IntEvent0RisDmaDoneTxR = crate::BitReader<IntEvent0RisDmaDoneTx>;
impl IntEvent0RisDmaDoneTxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisDmaDoneTx {
        match self.bits {
            false => IntEvent0RisDmaDoneTx::IntEvent0RisDmaDoneTxClr,
            true => IntEvent0RisDmaDoneTx::IntEvent0RisDmaDoneTxSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_dma_done_tx_clr(&self) -> bool {
        *self == IntEvent0RisDmaDoneTx::IntEvent0RisDmaDoneTxClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_dma_done_tx_set(&self) -> bool {
        *self == IntEvent0RisDmaDoneTx::IntEvent0RisDmaDoneTxSet
    }
}
#[doc = "TX FIFO Underflow Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisTxfifoUnf {
    #[doc = "0: CLR"]
    IntEvent0RisTxfifoUnfClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisTxfifoUnfSet = 1,
}
impl From<IntEvent0RisTxfifoUnf> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisTxfifoUnf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_TXFIFO_UNF` reader - TX FIFO Underflow Interrupt"]
pub type IntEvent0RisTxfifoUnfR = crate::BitReader<IntEvent0RisTxfifoUnf>;
impl IntEvent0RisTxfifoUnfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisTxfifoUnf {
        match self.bits {
            false => IntEvent0RisTxfifoUnf::IntEvent0RisTxfifoUnfClr,
            true => IntEvent0RisTxfifoUnf::IntEvent0RisTxfifoUnfSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_txfifo_unf_clr(&self) -> bool {
        *self == IntEvent0RisTxfifoUnf::IntEvent0RisTxfifoUnfClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_txfifo_unf_set(&self) -> bool {
        *self == IntEvent0RisTxfifoUnf::IntEvent0RisTxfifoUnfSet
    }
}
#[doc = "RX FIFO Full Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisRxfull {
    #[doc = "0: CLR"]
    IntEvent0RisRxfullClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisRxfullSet = 1,
}
impl From<IntEvent0RisRxfull> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisRxfull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_RXFULL` reader - RX FIFO Full Interrupt"]
pub type IntEvent0RisRxfullR = crate::BitReader<IntEvent0RisRxfull>;
impl IntEvent0RisRxfullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisRxfull {
        match self.bits {
            false => IntEvent0RisRxfull::IntEvent0RisRxfullClr,
            true => IntEvent0RisRxfull::IntEvent0RisRxfullSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_rxfull_clr(&self) -> bool {
        *self == IntEvent0RisRxfull::IntEvent0RisRxfullClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_rxfull_set(&self) -> bool {
        *self == IntEvent0RisRxfull::IntEvent0RisRxfullSet
    }
}
impl R {
    #[doc = "Bit 0 - RXFIFO overflow event. This interrupt is set if an RX FIFO overflow has been detected."]
    #[inline(always)]
    pub fn int_event0_ris_rxfifo_ovf(&self) -> IntEvent0RisRxfifoOvfR {
        IntEvent0RisRxfifoOvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity error event: this bit is set if a Parity error has been detected"]
    #[inline(always)]
    pub fn int_event0_ris_per(&self) -> IntEvent0RisPerR {
        IntEvent0RisPerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI Receive Time-Out event."]
    #[inline(always)]
    pub fn int_event0_ris_rtout(&self) -> IntEvent0RisRtoutR {
        IntEvent0RisRtoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO event.This interrupt is set if the selected Receive FIFO level has been reached"]
    #[inline(always)]
    pub fn int_event0_ris_rx(&self) -> IntEvent0RisRxR {
        IntEvent0RisRxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO event..This interrupt is set if the selected Transmit FIFO level has been reached."]
    #[inline(always)]
    pub fn int_event0_ris_tx(&self) -> IntEvent0RisTxR {
        IntEvent0RisTxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been move to the shift register."]
    #[inline(always)]
    pub fn int_event0_ris_txempty(&self) -> IntEvent0RisTxemptyR {
        IntEvent0RisTxemptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI has done finished transfers and changed into IDLE mode. This bit is set when BUSY goes low."]
    #[inline(always)]
    pub fn int_event0_ris_idle(&self) -> IntEvent0RisIdleR {
        IntEvent0RisIdleR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Done 1 event for RX. This interrupt is set if the RX DMA channel sends the DONE signal. This allows the handling of the DMA event inside the mapped peripheral."]
    #[inline(always)]
    pub fn int_event0_ris_dma_done_rx(&self) -> IntEvent0RisDmaDoneRxR {
        IntEvent0RisDmaDoneRxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA Done 1 event for TX. This interrupt is set if the TX DMA channel sends the DONE signal. This allows the handling of the DMA event inside the mapped peripheral."]
    #[inline(always)]
    pub fn int_event0_ris_dma_done_tx(&self) -> IntEvent0RisDmaDoneTxR {
        IntEvent0RisDmaDoneTxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TX FIFO Underflow Interrupt"]
    #[inline(always)]
    pub fn int_event0_ris_txfifo_unf(&self) -> IntEvent0RisTxfifoUnfR {
        IntEvent0RisTxfifoUnfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RX FIFO Full Interrupt"]
    #[inline(always)]
    pub fn int_event0_ris_rxfull(&self) -> IntEvent0RisRxfullR {
        IntEvent0RisRxfullR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0RisSpec;
impl crate::RegisterSpec for IntEvent0RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event0_ris::R`](R) reader structure"]
impl crate::Readable for IntEvent0RisSpec {}
#[doc = "`reset()` method sets INT_EVENT0_RIS to value 0"]
impl crate::Resettable for IntEvent0RisSpec {
    const RESET_VALUE: u32 = 0;
}
