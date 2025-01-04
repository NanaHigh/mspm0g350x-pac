#[doc = "Register `INT_EVENT0_IMASK` reader"]
pub type R = crate::R<IntEvent0ImaskSpec>;
#[doc = "Register `INT_EVENT0_IMASK` writer"]
pub type W = crate::W<IntEvent0ImaskSpec>;
#[doc = "RXFIFO overflow event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskRxfifoOvf {
    #[doc = "0: CLR"]
    IntEvent0ImaskRxfifoOvfClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskRxfifoOvfSet = 1,
}
impl From<IntEvent0ImaskRxfifoOvf> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskRxfifoOvf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RXFIFO_OVF` reader - RXFIFO overflow event mask."]
pub type IntEvent0ImaskRxfifoOvfR = crate::BitReader<IntEvent0ImaskRxfifoOvf>;
impl IntEvent0ImaskRxfifoOvfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskRxfifoOvf {
        match self.bits {
            false => IntEvent0ImaskRxfifoOvf::IntEvent0ImaskRxfifoOvfClr,
            true => IntEvent0ImaskRxfifoOvf::IntEvent0ImaskRxfifoOvfSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_rxfifo_ovf_clr(&self) -> bool {
        *self == IntEvent0ImaskRxfifoOvf::IntEvent0ImaskRxfifoOvfClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_rxfifo_ovf_set(&self) -> bool {
        *self == IntEvent0ImaskRxfifoOvf::IntEvent0ImaskRxfifoOvfSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RXFIFO_OVF` writer - RXFIFO overflow event mask."]
pub type IntEvent0ImaskRxfifoOvfW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskRxfifoOvf>;
impl<'a, REG> IntEvent0ImaskRxfifoOvfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_rxfifo_ovf_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRxfifoOvf::IntEvent0ImaskRxfifoOvfClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_rxfifo_ovf_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRxfifoOvf::IntEvent0ImaskRxfifoOvfSet)
    }
}
#[doc = "Parity error event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskPer {
    #[doc = "0: CLR"]
    IntEvent0ImaskPerClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskPerSet = 1,
}
impl From<IntEvent0ImaskPer> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskPer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_PER` reader - Parity error event mask."]
pub type IntEvent0ImaskPerR = crate::BitReader<IntEvent0ImaskPer>;
impl IntEvent0ImaskPerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskPer {
        match self.bits {
            false => IntEvent0ImaskPer::IntEvent0ImaskPerClr,
            true => IntEvent0ImaskPer::IntEvent0ImaskPerSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_per_clr(&self) -> bool {
        *self == IntEvent0ImaskPer::IntEvent0ImaskPerClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_per_set(&self) -> bool {
        *self == IntEvent0ImaskPer::IntEvent0ImaskPerSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_PER` writer - Parity error event mask."]
pub type IntEvent0ImaskPerW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskPer>;
impl<'a, REG> IntEvent0ImaskPerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_per_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskPer::IntEvent0ImaskPerClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_per_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskPer::IntEvent0ImaskPerSet)
    }
}
#[doc = "Enable SPI Receive Time-Out event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskRtout {
    #[doc = "0: CLR"]
    IntEvent0ImaskRtoutClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskRtoutSet = 1,
}
impl From<IntEvent0ImaskRtout> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskRtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RTOUT` reader - Enable SPI Receive Time-Out event mask."]
pub type IntEvent0ImaskRtoutR = crate::BitReader<IntEvent0ImaskRtout>;
impl IntEvent0ImaskRtoutR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskRtout {
        match self.bits {
            false => IntEvent0ImaskRtout::IntEvent0ImaskRtoutClr,
            true => IntEvent0ImaskRtout::IntEvent0ImaskRtoutSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_rtout_clr(&self) -> bool {
        *self == IntEvent0ImaskRtout::IntEvent0ImaskRtoutClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_rtout_set(&self) -> bool {
        *self == IntEvent0ImaskRtout::IntEvent0ImaskRtoutSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RTOUT` writer - Enable SPI Receive Time-Out event mask."]
pub type IntEvent0ImaskRtoutW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskRtout>;
impl<'a, REG> IntEvent0ImaskRtoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_rtout_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRtout::IntEvent0ImaskRtoutClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_rtout_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRtout::IntEvent0ImaskRtoutSet)
    }
}
#[doc = "Receive FIFO event.This interrupt is set if the selected Receive FIFO level has been reached\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskRx {
    #[doc = "0: CLR"]
    IntEvent0ImaskRxClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskRxSet = 1,
}
impl From<IntEvent0ImaskRx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RX` reader - Receive FIFO event.This interrupt is set if the selected Receive FIFO level has been reached"]
pub type IntEvent0ImaskRxR = crate::BitReader<IntEvent0ImaskRx>;
impl IntEvent0ImaskRxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskRx {
        match self.bits {
            false => IntEvent0ImaskRx::IntEvent0ImaskRxClr,
            true => IntEvent0ImaskRx::IntEvent0ImaskRxSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_rx_clr(&self) -> bool {
        *self == IntEvent0ImaskRx::IntEvent0ImaskRxClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_rx_set(&self) -> bool {
        *self == IntEvent0ImaskRx::IntEvent0ImaskRxSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RX` writer - Receive FIFO event.This interrupt is set if the selected Receive FIFO level has been reached"]
pub type IntEvent0ImaskRxW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskRx>;
impl<'a, REG> IntEvent0ImaskRxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_rx_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRx::IntEvent0ImaskRxClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_rx_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRx::IntEvent0ImaskRxSet)
    }
}
#[doc = "Transmit FIFO event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskTx {
    #[doc = "0: CLR"]
    IntEvent0ImaskTxClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskTxSet = 1,
}
impl From<IntEvent0ImaskTx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_TX` reader - Transmit FIFO event mask."]
pub type IntEvent0ImaskTxR = crate::BitReader<IntEvent0ImaskTx>;
impl IntEvent0ImaskTxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskTx {
        match self.bits {
            false => IntEvent0ImaskTx::IntEvent0ImaskTxClr,
            true => IntEvent0ImaskTx::IntEvent0ImaskTxSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_tx_clr(&self) -> bool {
        *self == IntEvent0ImaskTx::IntEvent0ImaskTxClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_tx_set(&self) -> bool {
        *self == IntEvent0ImaskTx::IntEvent0ImaskTxSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_TX` writer - Transmit FIFO event mask."]
pub type IntEvent0ImaskTxW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskTx>;
impl<'a, REG> IntEvent0ImaskTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_tx_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskTx::IntEvent0ImaskTxClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_tx_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskTx::IntEvent0ImaskTxSet)
    }
}
#[doc = "Transmit FIFO Empty event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskTxempty {
    #[doc = "0: CLR"]
    IntEvent0ImaskTxemptyClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskTxemptySet = 1,
}
impl From<IntEvent0ImaskTxempty> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskTxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_TXEMPTY` reader - Transmit FIFO Empty event mask."]
pub type IntEvent0ImaskTxemptyR = crate::BitReader<IntEvent0ImaskTxempty>;
impl IntEvent0ImaskTxemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskTxempty {
        match self.bits {
            false => IntEvent0ImaskTxempty::IntEvent0ImaskTxemptyClr,
            true => IntEvent0ImaskTxempty::IntEvent0ImaskTxemptySet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_txempty_clr(&self) -> bool {
        *self == IntEvent0ImaskTxempty::IntEvent0ImaskTxemptyClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_txempty_set(&self) -> bool {
        *self == IntEvent0ImaskTxempty::IntEvent0ImaskTxemptySet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_TXEMPTY` writer - Transmit FIFO Empty event mask."]
pub type IntEvent0ImaskTxemptyW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskTxempty>;
impl<'a, REG> IntEvent0ImaskTxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_txempty_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskTxempty::IntEvent0ImaskTxemptyClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_txempty_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskTxempty::IntEvent0ImaskTxemptySet)
    }
}
#[doc = "SPI Idle event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskIdle {
    #[doc = "0: CLR"]
    IntEvent0ImaskIdleClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskIdleSet = 1,
}
impl From<IntEvent0ImaskIdle> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskIdle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_IDLE` reader - SPI Idle event mask."]
pub type IntEvent0ImaskIdleR = crate::BitReader<IntEvent0ImaskIdle>;
impl IntEvent0ImaskIdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskIdle {
        match self.bits {
            false => IntEvent0ImaskIdle::IntEvent0ImaskIdleClr,
            true => IntEvent0ImaskIdle::IntEvent0ImaskIdleSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_idle_clr(&self) -> bool {
        *self == IntEvent0ImaskIdle::IntEvent0ImaskIdleClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_idle_set(&self) -> bool {
        *self == IntEvent0ImaskIdle::IntEvent0ImaskIdleSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_IDLE` writer - SPI Idle event mask."]
pub type IntEvent0ImaskIdleW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskIdle>;
impl<'a, REG> IntEvent0ImaskIdleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_idle_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskIdle::IntEvent0ImaskIdleClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_idle_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskIdle::IntEvent0ImaskIdleSet)
    }
}
#[doc = "DMA Done 1 event for RX event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDmaDoneRx {
    #[doc = "0: CLR"]
    IntEvent0ImaskDmaDoneRxClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDmaDoneRxSet = 1,
}
impl From<IntEvent0ImaskDmaDoneRx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDmaDoneRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DMA_DONE_RX` reader - DMA Done 1 event for RX event mask."]
pub type IntEvent0ImaskDmaDoneRxR = crate::BitReader<IntEvent0ImaskDmaDoneRx>;
impl IntEvent0ImaskDmaDoneRxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDmaDoneRx {
        match self.bits {
            false => IntEvent0ImaskDmaDoneRx::IntEvent0ImaskDmaDoneRxClr,
            true => IntEvent0ImaskDmaDoneRx::IntEvent0ImaskDmaDoneRxSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dma_done_rx_clr(&self) -> bool {
        *self == IntEvent0ImaskDmaDoneRx::IntEvent0ImaskDmaDoneRxClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dma_done_rx_set(&self) -> bool {
        *self == IntEvent0ImaskDmaDoneRx::IntEvent0ImaskDmaDoneRxSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DMA_DONE_RX` writer - DMA Done 1 event for RX event mask."]
pub type IntEvent0ImaskDmaDoneRxW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDmaDoneRx>;
impl<'a, REG> IntEvent0ImaskDmaDoneRxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dma_done_rx_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDmaDoneRx::IntEvent0ImaskDmaDoneRxClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dma_done_rx_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDmaDoneRx::IntEvent0ImaskDmaDoneRxSet)
    }
}
#[doc = "DMA Done 1 event for TX event mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDmaDoneTx {
    #[doc = "0: CLR"]
    IntEvent0ImaskDmaDoneTxClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDmaDoneTxSet = 1,
}
impl From<IntEvent0ImaskDmaDoneTx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDmaDoneTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DMA_DONE_TX` reader - DMA Done 1 event for TX event mask."]
pub type IntEvent0ImaskDmaDoneTxR = crate::BitReader<IntEvent0ImaskDmaDoneTx>;
impl IntEvent0ImaskDmaDoneTxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDmaDoneTx {
        match self.bits {
            false => IntEvent0ImaskDmaDoneTx::IntEvent0ImaskDmaDoneTxClr,
            true => IntEvent0ImaskDmaDoneTx::IntEvent0ImaskDmaDoneTxSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dma_done_tx_clr(&self) -> bool {
        *self == IntEvent0ImaskDmaDoneTx::IntEvent0ImaskDmaDoneTxClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dma_done_tx_set(&self) -> bool {
        *self == IntEvent0ImaskDmaDoneTx::IntEvent0ImaskDmaDoneTxSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DMA_DONE_TX` writer - DMA Done 1 event for TX event mask."]
pub type IntEvent0ImaskDmaDoneTxW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDmaDoneTx>;
impl<'a, REG> IntEvent0ImaskDmaDoneTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dma_done_tx_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDmaDoneTx::IntEvent0ImaskDmaDoneTxClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dma_done_tx_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDmaDoneTx::IntEvent0ImaskDmaDoneTxSet)
    }
}
#[doc = "TX FIFO underflow interrupt mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskTxfifoUnf {
    #[doc = "0: CLR"]
    IntEvent0ImaskTxfifoUnfClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskTxfifoUnfSet = 1,
}
impl From<IntEvent0ImaskTxfifoUnf> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskTxfifoUnf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_TXFIFO_UNF` reader - TX FIFO underflow interrupt mask"]
pub type IntEvent0ImaskTxfifoUnfR = crate::BitReader<IntEvent0ImaskTxfifoUnf>;
impl IntEvent0ImaskTxfifoUnfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskTxfifoUnf {
        match self.bits {
            false => IntEvent0ImaskTxfifoUnf::IntEvent0ImaskTxfifoUnfClr,
            true => IntEvent0ImaskTxfifoUnf::IntEvent0ImaskTxfifoUnfSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_txfifo_unf_clr(&self) -> bool {
        *self == IntEvent0ImaskTxfifoUnf::IntEvent0ImaskTxfifoUnfClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_txfifo_unf_set(&self) -> bool {
        *self == IntEvent0ImaskTxfifoUnf::IntEvent0ImaskTxfifoUnfSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_TXFIFO_UNF` writer - TX FIFO underflow interrupt mask"]
pub type IntEvent0ImaskTxfifoUnfW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskTxfifoUnf>;
impl<'a, REG> IntEvent0ImaskTxfifoUnfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_txfifo_unf_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskTxfifoUnf::IntEvent0ImaskTxfifoUnfClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_txfifo_unf_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskTxfifoUnf::IntEvent0ImaskTxfifoUnfSet)
    }
}
#[doc = "RX FIFO Full Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskRxfull {
    #[doc = "0: CLR"]
    IntEvent0ImaskRxfullClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskRxfullSet = 1,
}
impl From<IntEvent0ImaskRxfull> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskRxfull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RXFULL` reader - RX FIFO Full Interrupt Mask"]
pub type IntEvent0ImaskRxfullR = crate::BitReader<IntEvent0ImaskRxfull>;
impl IntEvent0ImaskRxfullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskRxfull {
        match self.bits {
            false => IntEvent0ImaskRxfull::IntEvent0ImaskRxfullClr,
            true => IntEvent0ImaskRxfull::IntEvent0ImaskRxfullSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_rxfull_clr(&self) -> bool {
        *self == IntEvent0ImaskRxfull::IntEvent0ImaskRxfullClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_rxfull_set(&self) -> bool {
        *self == IntEvent0ImaskRxfull::IntEvent0ImaskRxfullSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_RXFULL` writer - RX FIFO Full Interrupt Mask"]
pub type IntEvent0ImaskRxfullW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskRxfull>;
impl<'a, REG> IntEvent0ImaskRxfullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_rxfull_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRxfull::IntEvent0ImaskRxfullClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_rxfull_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskRxfull::IntEvent0ImaskRxfullSet)
    }
}
impl R {
    #[doc = "Bit 0 - RXFIFO overflow event mask."]
    #[inline(always)]
    pub fn int_event0_imask_rxfifo_ovf(&self) -> IntEvent0ImaskRxfifoOvfR {
        IntEvent0ImaskRxfifoOvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity error event mask."]
    #[inline(always)]
    pub fn int_event0_imask_per(&self) -> IntEvent0ImaskPerR {
        IntEvent0ImaskPerR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable SPI Receive Time-Out event mask."]
    #[inline(always)]
    pub fn int_event0_imask_rtout(&self) -> IntEvent0ImaskRtoutR {
        IntEvent0ImaskRtoutR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO event.This interrupt is set if the selected Receive FIFO level has been reached"]
    #[inline(always)]
    pub fn int_event0_imask_rx(&self) -> IntEvent0ImaskRxR {
        IntEvent0ImaskRxR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO event mask."]
    #[inline(always)]
    pub fn int_event0_imask_tx(&self) -> IntEvent0ImaskTxR {
        IntEvent0ImaskTxR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO Empty event mask."]
    #[inline(always)]
    pub fn int_event0_imask_txempty(&self) -> IntEvent0ImaskTxemptyR {
        IntEvent0ImaskTxemptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI Idle event mask."]
    #[inline(always)]
    pub fn int_event0_imask_idle(&self) -> IntEvent0ImaskIdleR {
        IntEvent0ImaskIdleR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Done 1 event for RX event mask."]
    #[inline(always)]
    pub fn int_event0_imask_dma_done_rx(&self) -> IntEvent0ImaskDmaDoneRxR {
        IntEvent0ImaskDmaDoneRxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA Done 1 event for TX event mask."]
    #[inline(always)]
    pub fn int_event0_imask_dma_done_tx(&self) -> IntEvent0ImaskDmaDoneTxR {
        IntEvent0ImaskDmaDoneTxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TX FIFO underflow interrupt mask"]
    #[inline(always)]
    pub fn int_event0_imask_txfifo_unf(&self) -> IntEvent0ImaskTxfifoUnfR {
        IntEvent0ImaskTxfifoUnfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RX FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn int_event0_imask_rxfull(&self) -> IntEvent0ImaskRxfullR {
        IntEvent0ImaskRxfullR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXFIFO overflow event mask."]
    #[inline(always)]
    pub fn int_event0_imask_rxfifo_ovf(&mut self) -> IntEvent0ImaskRxfifoOvfW<IntEvent0ImaskSpec> {
        IntEvent0ImaskRxfifoOvfW::new(self, 0)
    }
    #[doc = "Bit 1 - Parity error event mask."]
    #[inline(always)]
    pub fn int_event0_imask_per(&mut self) -> IntEvent0ImaskPerW<IntEvent0ImaskSpec> {
        IntEvent0ImaskPerW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable SPI Receive Time-Out event mask."]
    #[inline(always)]
    pub fn int_event0_imask_rtout(&mut self) -> IntEvent0ImaskRtoutW<IntEvent0ImaskSpec> {
        IntEvent0ImaskRtoutW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive FIFO event.This interrupt is set if the selected Receive FIFO level has been reached"]
    #[inline(always)]
    pub fn int_event0_imask_rx(&mut self) -> IntEvent0ImaskRxW<IntEvent0ImaskSpec> {
        IntEvent0ImaskRxW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit FIFO event mask."]
    #[inline(always)]
    pub fn int_event0_imask_tx(&mut self) -> IntEvent0ImaskTxW<IntEvent0ImaskSpec> {
        IntEvent0ImaskTxW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit FIFO Empty event mask."]
    #[inline(always)]
    pub fn int_event0_imask_txempty(&mut self) -> IntEvent0ImaskTxemptyW<IntEvent0ImaskSpec> {
        IntEvent0ImaskTxemptyW::new(self, 5)
    }
    #[doc = "Bit 6 - SPI Idle event mask."]
    #[inline(always)]
    pub fn int_event0_imask_idle(&mut self) -> IntEvent0ImaskIdleW<IntEvent0ImaskSpec> {
        IntEvent0ImaskIdleW::new(self, 6)
    }
    #[doc = "Bit 7 - DMA Done 1 event for RX event mask."]
    #[inline(always)]
    pub fn int_event0_imask_dma_done_rx(&mut self) -> IntEvent0ImaskDmaDoneRxW<IntEvent0ImaskSpec> {
        IntEvent0ImaskDmaDoneRxW::new(self, 7)
    }
    #[doc = "Bit 8 - DMA Done 1 event for TX event mask."]
    #[inline(always)]
    pub fn int_event0_imask_dma_done_tx(&mut self) -> IntEvent0ImaskDmaDoneTxW<IntEvent0ImaskSpec> {
        IntEvent0ImaskDmaDoneTxW::new(self, 8)
    }
    #[doc = "Bit 9 - TX FIFO underflow interrupt mask"]
    #[inline(always)]
    pub fn int_event0_imask_txfifo_unf(&mut self) -> IntEvent0ImaskTxfifoUnfW<IntEvent0ImaskSpec> {
        IntEvent0ImaskTxfifoUnfW::new(self, 9)
    }
    #[doc = "Bit 10 - RX FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn int_event0_imask_rxfull(&mut self) -> IntEvent0ImaskRxfullW<IntEvent0ImaskSpec> {
        IntEvent0ImaskRxfullW::new(self, 10)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event0_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0ImaskSpec;
impl crate::RegisterSpec for IntEvent0ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event0_imask::R`](R) reader structure"]
impl crate::Readable for IntEvent0ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_event0_imask::W`](W) writer structure"]
impl crate::Writable for IntEvent0ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT0_IMASK to value 0"]
impl crate::Resettable for IntEvent0ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
