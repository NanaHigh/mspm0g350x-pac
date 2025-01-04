#[doc = "Register `INT_EVENT0_ISET` writer"]
pub type W = crate::W<IntEvent0IsetSpec>;
#[doc = "Set RXFIFO overflow event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetRxfifoOvf {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetRxfifoOvfNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetRxfifoOvfSet = 1,
}
impl From<IntEvent0IsetRxfifoOvf> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetRxfifoOvf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_RXFIFO_OVF` writer - Set RXFIFO overflow event."]
pub type IntEvent0IsetRxfifoOvfW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetRxfifoOvf>;
impl<'a, REG> IntEvent0IsetRxfifoOvfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_rxfifo_ovf_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRxfifoOvf::IntEvent0IsetRxfifoOvfNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_rxfifo_ovf_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRxfifoOvf::IntEvent0IsetRxfifoOvfSet)
    }
}
#[doc = "Set Parity error event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetPer {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetPerNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetPerSet = 1,
}
impl From<IntEvent0IsetPer> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetPer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_PER` writer - Set Parity error event."]
pub type IntEvent0IsetPerW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetPer>;
impl<'a, REG> IntEvent0IsetPerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_per_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetPer::IntEvent0IsetPerNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_per_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetPer::IntEvent0IsetPerSet)
    }
}
#[doc = "Set SPI Receive Time-Out Event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetRtout {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetRtoutNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetRtoutSet = 1,
}
impl From<IntEvent0IsetRtout> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetRtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_RTOUT` writer - Set SPI Receive Time-Out Event."]
pub type IntEvent0IsetRtoutW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetRtout>;
impl<'a, REG> IntEvent0IsetRtoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_rtout_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRtout::IntEvent0IsetRtoutNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_rtout_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRtout::IntEvent0IsetRtoutSet)
    }
}
#[doc = "Set Receive FIFO event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetRx {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetRxNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetRxSet = 1,
}
impl From<IntEvent0IsetRx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_RX` writer - Set Receive FIFO event."]
pub type IntEvent0IsetRxW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetRx>;
impl<'a, REG> IntEvent0IsetRxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_rx_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRx::IntEvent0IsetRxNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_rx_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRx::IntEvent0IsetRxSet)
    }
}
#[doc = "Set Transmit FIFO event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetTx {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetTxNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetTxSet = 1,
}
impl From<IntEvent0IsetTx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_TX` writer - Set Transmit FIFO event."]
pub type IntEvent0IsetTxW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetTx>;
impl<'a, REG> IntEvent0IsetTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_tx_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetTx::IntEvent0IsetTxNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_tx_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetTx::IntEvent0IsetTxSet)
    }
}
#[doc = "Set Transmit FIFO Empty event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetTxempty {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetTxemptyNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetTxemptySet = 1,
}
impl From<IntEvent0IsetTxempty> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetTxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_TXEMPTY` writer - Set Transmit FIFO Empty event."]
pub type IntEvent0IsetTxemptyW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetTxempty>;
impl<'a, REG> IntEvent0IsetTxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_txempty_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetTxempty::IntEvent0IsetTxemptyNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_txempty_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetTxempty::IntEvent0IsetTxemptySet)
    }
}
#[doc = "Set SPI IDLE mode event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetIdle {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetIdleNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetIdleSet = 1,
}
impl From<IntEvent0IsetIdle> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetIdle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_IDLE` writer - Set SPI IDLE mode event."]
pub type IntEvent0IsetIdleW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetIdle>;
impl<'a, REG> IntEvent0IsetIdleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_idle_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetIdle::IntEvent0IsetIdleNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_idle_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetIdle::IntEvent0IsetIdleSet)
    }
}
#[doc = "Set DMA Done 1 event for RX.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDmaDoneRx {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDmaDoneRxNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDmaDoneRxSet = 1,
}
impl From<IntEvent0IsetDmaDoneRx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDmaDoneRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DMA_DONE_RX` writer - Set DMA Done 1 event for RX."]
pub type IntEvent0IsetDmaDoneRxW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDmaDoneRx>;
impl<'a, REG> IntEvent0IsetDmaDoneRxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dma_done_rx_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDmaDoneRx::IntEvent0IsetDmaDoneRxNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dma_done_rx_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDmaDoneRx::IntEvent0IsetDmaDoneRxSet)
    }
}
#[doc = "Set DMA Done 1 event for TX.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDmaDoneTx {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDmaDoneTxNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDmaDoneTxSet = 1,
}
impl From<IntEvent0IsetDmaDoneTx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDmaDoneTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DMA_DONE_TX` writer - Set DMA Done 1 event for TX."]
pub type IntEvent0IsetDmaDoneTxW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDmaDoneTx>;
impl<'a, REG> IntEvent0IsetDmaDoneTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dma_done_tx_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDmaDoneTx::IntEvent0IsetDmaDoneTxNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dma_done_tx_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDmaDoneTx::IntEvent0IsetDmaDoneTxSet)
    }
}
#[doc = "Set TX FIFO Underflow Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetTxfifoUnf {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetTxfifoUnfNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetTxfifoUnfSet = 1,
}
impl From<IntEvent0IsetTxfifoUnf> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetTxfifoUnf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_TXFIFO_UNF` writer - Set TX FIFO Underflow Event"]
pub type IntEvent0IsetTxfifoUnfW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetTxfifoUnf>;
impl<'a, REG> IntEvent0IsetTxfifoUnfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_txfifo_unf_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetTxfifoUnf::IntEvent0IsetTxfifoUnfNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_txfifo_unf_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetTxfifoUnf::IntEvent0IsetTxfifoUnfSet)
    }
}
#[doc = "Set RX FIFO Full Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetRxfull {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetRxfullNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetRxfullSet = 1,
}
impl From<IntEvent0IsetRxfull> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetRxfull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_RXFULL` writer - Set RX FIFO Full Event"]
pub type IntEvent0IsetRxfullW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetRxfull>;
impl<'a, REG> IntEvent0IsetRxfullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_rxfull_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRxfull::IntEvent0IsetRxfullNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_rxfull_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRxfull::IntEvent0IsetRxfullSet)
    }
}
impl W {
    #[doc = "Bit 0 - Set RXFIFO overflow event."]
    #[inline(always)]
    pub fn int_event0_iset_rxfifo_ovf(&mut self) -> IntEvent0IsetRxfifoOvfW<IntEvent0IsetSpec> {
        IntEvent0IsetRxfifoOvfW::new(self, 0)
    }
    #[doc = "Bit 1 - Set Parity error event."]
    #[inline(always)]
    pub fn int_event0_iset_per(&mut self) -> IntEvent0IsetPerW<IntEvent0IsetSpec> {
        IntEvent0IsetPerW::new(self, 1)
    }
    #[doc = "Bit 2 - Set SPI Receive Time-Out Event."]
    #[inline(always)]
    pub fn int_event0_iset_rtout(&mut self) -> IntEvent0IsetRtoutW<IntEvent0IsetSpec> {
        IntEvent0IsetRtoutW::new(self, 2)
    }
    #[doc = "Bit 3 - Set Receive FIFO event."]
    #[inline(always)]
    pub fn int_event0_iset_rx(&mut self) -> IntEvent0IsetRxW<IntEvent0IsetSpec> {
        IntEvent0IsetRxW::new(self, 3)
    }
    #[doc = "Bit 4 - Set Transmit FIFO event."]
    #[inline(always)]
    pub fn int_event0_iset_tx(&mut self) -> IntEvent0IsetTxW<IntEvent0IsetSpec> {
        IntEvent0IsetTxW::new(self, 4)
    }
    #[doc = "Bit 5 - Set Transmit FIFO Empty event."]
    #[inline(always)]
    pub fn int_event0_iset_txempty(&mut self) -> IntEvent0IsetTxemptyW<IntEvent0IsetSpec> {
        IntEvent0IsetTxemptyW::new(self, 5)
    }
    #[doc = "Bit 6 - Set SPI IDLE mode event."]
    #[inline(always)]
    pub fn int_event0_iset_idle(&mut self) -> IntEvent0IsetIdleW<IntEvent0IsetSpec> {
        IntEvent0IsetIdleW::new(self, 6)
    }
    #[doc = "Bit 7 - Set DMA Done 1 event for RX."]
    #[inline(always)]
    pub fn int_event0_iset_dma_done_rx(&mut self) -> IntEvent0IsetDmaDoneRxW<IntEvent0IsetSpec> {
        IntEvent0IsetDmaDoneRxW::new(self, 7)
    }
    #[doc = "Bit 8 - Set DMA Done 1 event for TX."]
    #[inline(always)]
    pub fn int_event0_iset_dma_done_tx(&mut self) -> IntEvent0IsetDmaDoneTxW<IntEvent0IsetSpec> {
        IntEvent0IsetDmaDoneTxW::new(self, 8)
    }
    #[doc = "Bit 9 - Set TX FIFO Underflow Event"]
    #[inline(always)]
    pub fn int_event0_iset_txfifo_unf(&mut self) -> IntEvent0IsetTxfifoUnfW<IntEvent0IsetSpec> {
        IntEvent0IsetTxfifoUnfW::new(self, 9)
    }
    #[doc = "Bit 10 - Set RX FIFO Full Event"]
    #[inline(always)]
    pub fn int_event0_iset_rxfull(&mut self) -> IntEvent0IsetRxfullW<IntEvent0IsetSpec> {
        IntEvent0IsetRxfullW::new(self, 10)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event0_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0IsetSpec;
impl crate::RegisterSpec for IntEvent0IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event0_iset::W`](W) writer structure"]
impl crate::Writable for IntEvent0IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT0_ISET to value 0"]
impl crate::Resettable for IntEvent0IsetSpec {
    const RESET_VALUE: u32 = 0;
}
