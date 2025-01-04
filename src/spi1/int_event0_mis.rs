#[doc = "Register `INT_EVENT0_MIS` reader"]
pub type R = crate::R<IntEvent0MisSpec>;
#[doc = "Masked RXFIFO overflow event. This interrupt is set if an RX FIFO overflow has been detected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisRxfifoOvf {
    #[doc = "0: CLR"]
    IntEvent0MisRxfifoOvfClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisRxfifoOvfSet = 1,
}
impl From<IntEvent0MisRxfifoOvf> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisRxfifoOvf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_RXFIFO_OVF` reader - Masked RXFIFO overflow event. This interrupt is set if an RX FIFO overflow has been detected."]
pub type IntEvent0MisRxfifoOvfR = crate::BitReader<IntEvent0MisRxfifoOvf>;
impl IntEvent0MisRxfifoOvfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisRxfifoOvf {
        match self.bits {
            false => IntEvent0MisRxfifoOvf::IntEvent0MisRxfifoOvfClr,
            true => IntEvent0MisRxfifoOvf::IntEvent0MisRxfifoOvfSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_rxfifo_ovf_clr(&self) -> bool {
        *self == IntEvent0MisRxfifoOvf::IntEvent0MisRxfifoOvfClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_rxfifo_ovf_set(&self) -> bool {
        *self == IntEvent0MisRxfifoOvf::IntEvent0MisRxfifoOvfSet
    }
}
#[doc = "Masked Parity error event: this bit if a Parity error has been detected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisPer {
    #[doc = "0: CLR"]
    IntEvent0MisPerClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisPerSet = 1,
}
impl From<IntEvent0MisPer> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisPer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_PER` reader - Masked Parity error event: this bit if a Parity error has been detected"]
pub type IntEvent0MisPerR = crate::BitReader<IntEvent0MisPer>;
impl IntEvent0MisPerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisPer {
        match self.bits {
            false => IntEvent0MisPer::IntEvent0MisPerClr,
            true => IntEvent0MisPer::IntEvent0MisPerSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_per_clr(&self) -> bool {
        *self == IntEvent0MisPer::IntEvent0MisPerClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_per_set(&self) -> bool {
        *self == IntEvent0MisPer::IntEvent0MisPerSet
    }
}
#[doc = "Masked SPI Receive Time-Out Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisRtout {
    #[doc = "0: CLR"]
    IntEvent0MisRtoutClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisRtoutSet = 1,
}
impl From<IntEvent0MisRtout> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisRtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_RTOUT` reader - Masked SPI Receive Time-Out Interrupt."]
pub type IntEvent0MisRtoutR = crate::BitReader<IntEvent0MisRtout>;
impl IntEvent0MisRtoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisRtout {
        match self.bits {
            false => IntEvent0MisRtout::IntEvent0MisRtoutClr,
            true => IntEvent0MisRtout::IntEvent0MisRtoutSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_rtout_clr(&self) -> bool {
        *self == IntEvent0MisRtout::IntEvent0MisRtoutClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_rtout_set(&self) -> bool {
        *self == IntEvent0MisRtout::IntEvent0MisRtoutSet
    }
}
#[doc = "Masked receive FIFO event.This interrupt is set if the selected Receive FIFO level has been reached\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisRx {
    #[doc = "0: CLR"]
    IntEvent0MisRxClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisRxSet = 1,
}
impl From<IntEvent0MisRx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_RX` reader - Masked receive FIFO event.This interrupt is set if the selected Receive FIFO level has been reached"]
pub type IntEvent0MisRxR = crate::BitReader<IntEvent0MisRx>;
impl IntEvent0MisRxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisRx {
        match self.bits {
            false => IntEvent0MisRx::IntEvent0MisRxClr,
            true => IntEvent0MisRx::IntEvent0MisRxSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_rx_clr(&self) -> bool {
        *self == IntEvent0MisRx::IntEvent0MisRxClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_rx_set(&self) -> bool {
        *self == IntEvent0MisRx::IntEvent0MisRxSet
    }
}
#[doc = "Masked Transmit FIFO event. This interrupt is set if the selected Transmit FIFO level has been reached.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisTx {
    #[doc = "0: CLR"]
    IntEvent0MisTxClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisTxSet = 1,
}
impl From<IntEvent0MisTx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_TX` reader - Masked Transmit FIFO event. This interrupt is set if the selected Transmit FIFO level has been reached."]
pub type IntEvent0MisTxR = crate::BitReader<IntEvent0MisTx>;
impl IntEvent0MisTxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisTx {
        match self.bits {
            false => IntEvent0MisTx::IntEvent0MisTxClr,
            true => IntEvent0MisTx::IntEvent0MisTxSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_tx_clr(&self) -> bool {
        *self == IntEvent0MisTx::IntEvent0MisTxClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_tx_set(&self) -> bool {
        *self == IntEvent0MisTx::IntEvent0MisTxSet
    }
}
#[doc = "Masked Transmit FIFO Empty event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisTxempty {
    #[doc = "0: CLR"]
    IntEvent0MisTxemptyClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisTxemptySet = 1,
}
impl From<IntEvent0MisTxempty> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisTxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_TXEMPTY` reader - Masked Transmit FIFO Empty event."]
pub type IntEvent0MisTxemptyR = crate::BitReader<IntEvent0MisTxempty>;
impl IntEvent0MisTxemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisTxempty {
        match self.bits {
            false => IntEvent0MisTxempty::IntEvent0MisTxemptyClr,
            true => IntEvent0MisTxempty::IntEvent0MisTxemptySet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_txempty_clr(&self) -> bool {
        *self == IntEvent0MisTxempty::IntEvent0MisTxemptyClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_txempty_set(&self) -> bool {
        *self == IntEvent0MisTxempty::IntEvent0MisTxemptySet
    }
}
#[doc = "Masked SPI IDLE mode event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisIdle {
    #[doc = "0: CLR"]
    IntEvent0MisIdleClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisIdleSet = 1,
}
impl From<IntEvent0MisIdle> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisIdle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_IDLE` reader - Masked SPI IDLE mode event."]
pub type IntEvent0MisIdleR = crate::BitReader<IntEvent0MisIdle>;
impl IntEvent0MisIdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisIdle {
        match self.bits {
            false => IntEvent0MisIdle::IntEvent0MisIdleClr,
            true => IntEvent0MisIdle::IntEvent0MisIdleSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_idle_clr(&self) -> bool {
        *self == IntEvent0MisIdle::IntEvent0MisIdleClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_idle_set(&self) -> bool {
        *self == IntEvent0MisIdle::IntEvent0MisIdleSet
    }
}
#[doc = "Masked DMA Done 1 event for RX.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDmaDoneRx {
    #[doc = "0: CLR"]
    IntEvent0MisDmaDoneRxClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDmaDoneRxSet = 1,
}
impl From<IntEvent0MisDmaDoneRx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDmaDoneRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DMA_DONE_RX` reader - Masked DMA Done 1 event for RX."]
pub type IntEvent0MisDmaDoneRxR = crate::BitReader<IntEvent0MisDmaDoneRx>;
impl IntEvent0MisDmaDoneRxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDmaDoneRx {
        match self.bits {
            false => IntEvent0MisDmaDoneRx::IntEvent0MisDmaDoneRxClr,
            true => IntEvent0MisDmaDoneRx::IntEvent0MisDmaDoneRxSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dma_done_rx_clr(&self) -> bool {
        *self == IntEvent0MisDmaDoneRx::IntEvent0MisDmaDoneRxClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dma_done_rx_set(&self) -> bool {
        *self == IntEvent0MisDmaDoneRx::IntEvent0MisDmaDoneRxSet
    }
}
#[doc = "Masked DMA Done 1 event for TX.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisDmaDoneTx {
    #[doc = "0: CLR"]
    IntEvent0MisDmaDoneTxClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisDmaDoneTxSet = 1,
}
impl From<IntEvent0MisDmaDoneTx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisDmaDoneTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_DMA_DONE_TX` reader - Masked DMA Done 1 event for TX."]
pub type IntEvent0MisDmaDoneTxR = crate::BitReader<IntEvent0MisDmaDoneTx>;
impl IntEvent0MisDmaDoneTxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisDmaDoneTx {
        match self.bits {
            false => IntEvent0MisDmaDoneTx::IntEvent0MisDmaDoneTxClr,
            true => IntEvent0MisDmaDoneTx::IntEvent0MisDmaDoneTxSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_dma_done_tx_clr(&self) -> bool {
        *self == IntEvent0MisDmaDoneTx::IntEvent0MisDmaDoneTxClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_dma_done_tx_set(&self) -> bool {
        *self == IntEvent0MisDmaDoneTx::IntEvent0MisDmaDoneTxSet
    }
}
#[doc = "TX FIFO underflow interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisTxfifoUnf {
    #[doc = "0: CLR"]
    IntEvent0MisTxfifoUnfClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisTxfifoUnfSet = 1,
}
impl From<IntEvent0MisTxfifoUnf> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisTxfifoUnf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_TXFIFO_UNF` reader - TX FIFO underflow interrupt"]
pub type IntEvent0MisTxfifoUnfR = crate::BitReader<IntEvent0MisTxfifoUnf>;
impl IntEvent0MisTxfifoUnfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisTxfifoUnf {
        match self.bits {
            false => IntEvent0MisTxfifoUnf::IntEvent0MisTxfifoUnfClr,
            true => IntEvent0MisTxfifoUnf::IntEvent0MisTxfifoUnfSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_txfifo_unf_clr(&self) -> bool {
        *self == IntEvent0MisTxfifoUnf::IntEvent0MisTxfifoUnfClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_txfifo_unf_set(&self) -> bool {
        *self == IntEvent0MisTxfifoUnf::IntEvent0MisTxfifoUnfSet
    }
}
#[doc = "RX FIFO Full Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisRxfull {
    #[doc = "0: CLR"]
    IntEvent0MisRxfullClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisRxfullSet = 1,
}
impl From<IntEvent0MisRxfull> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisRxfull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_RXFULL` reader - RX FIFO Full Interrupt"]
pub type IntEvent0MisRxfullR = crate::BitReader<IntEvent0MisRxfull>;
impl IntEvent0MisRxfullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisRxfull {
        match self.bits {
            false => IntEvent0MisRxfull::IntEvent0MisRxfullClr,
            true => IntEvent0MisRxfull::IntEvent0MisRxfullSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_rxfull_clr(&self) -> bool {
        *self == IntEvent0MisRxfull::IntEvent0MisRxfullClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_rxfull_set(&self) -> bool {
        *self == IntEvent0MisRxfull::IntEvent0MisRxfullSet
    }
}
impl R {
    #[doc = "Bit 0 - Masked RXFIFO overflow event. This interrupt is set if an RX FIFO overflow has been detected."]
    #[inline(always)]
    pub fn int_event0_mis_rxfifo_ovf(&self) -> IntEvent0MisRxfifoOvfR {
        IntEvent0MisRxfifoOvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masked Parity error event: this bit if a Parity error has been detected"]
    #[inline(always)]
    pub fn int_event0_mis_per(&self) -> IntEvent0MisPerR {
        IntEvent0MisPerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masked SPI Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn int_event0_mis_rtout(&self) -> IntEvent0MisRtoutR {
        IntEvent0MisRtoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masked receive FIFO event.This interrupt is set if the selected Receive FIFO level has been reached"]
    #[inline(always)]
    pub fn int_event0_mis_rx(&self) -> IntEvent0MisRxR {
        IntEvent0MisRxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Masked Transmit FIFO event. This interrupt is set if the selected Transmit FIFO level has been reached."]
    #[inline(always)]
    pub fn int_event0_mis_tx(&self) -> IntEvent0MisTxR {
        IntEvent0MisTxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Masked Transmit FIFO Empty event."]
    #[inline(always)]
    pub fn int_event0_mis_txempty(&self) -> IntEvent0MisTxemptyR {
        IntEvent0MisTxemptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Masked SPI IDLE mode event."]
    #[inline(always)]
    pub fn int_event0_mis_idle(&self) -> IntEvent0MisIdleR {
        IntEvent0MisIdleR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Masked DMA Done 1 event for RX."]
    #[inline(always)]
    pub fn int_event0_mis_dma_done_rx(&self) -> IntEvent0MisDmaDoneRxR {
        IntEvent0MisDmaDoneRxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Masked DMA Done 1 event for TX."]
    #[inline(always)]
    pub fn int_event0_mis_dma_done_tx(&self) -> IntEvent0MisDmaDoneTxR {
        IntEvent0MisDmaDoneTxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TX FIFO underflow interrupt"]
    #[inline(always)]
    pub fn int_event0_mis_txfifo_unf(&self) -> IntEvent0MisTxfifoUnfR {
        IntEvent0MisTxfifoUnfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RX FIFO Full Interrupt"]
    #[inline(always)]
    pub fn int_event0_mis_rxfull(&self) -> IntEvent0MisRxfullR {
        IntEvent0MisRxfullR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0MisSpec;
impl crate::RegisterSpec for IntEvent0MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event0_mis::R`](R) reader structure"]
impl crate::Readable for IntEvent0MisSpec {}
#[doc = "`reset()` method sets INT_EVENT0_MIS to value 0"]
impl crate::Resettable for IntEvent0MisSpec {
    const RESET_VALUE: u32 = 0;
}
