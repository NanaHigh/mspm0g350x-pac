#[doc = "Register `INT_EVENT0_ICLR` writer"]
pub type W = crate::W<IntEvent0IclrSpec>;
#[doc = "Clear RXFIFO overflow event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrRxfifoOvf {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrRxfifoOvfNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrRxfifoOvfClr = 1,
}
impl From<IntEvent0IclrRxfifoOvf> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrRxfifoOvf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_RXFIFO_OVF` writer - Clear RXFIFO overflow event."]
pub type IntEvent0IclrRxfifoOvfW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrRxfifoOvf>;
impl<'a, REG> IntEvent0IclrRxfifoOvfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_rxfifo_ovf_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRxfifoOvf::IntEvent0IclrRxfifoOvfNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_rxfifo_ovf_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRxfifoOvf::IntEvent0IclrRxfifoOvfClr)
    }
}
#[doc = "Clear Parity error event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrPer {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrPerNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrPerClr = 1,
}
impl From<IntEvent0IclrPer> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrPer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_PER` writer - Clear Parity error event."]
pub type IntEvent0IclrPerW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrPer>;
impl<'a, REG> IntEvent0IclrPerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_per_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrPer::IntEvent0IclrPerNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_per_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrPer::IntEvent0IclrPerClr)
    }
}
#[doc = "Clear SPI Receive Time-Out Event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrRtout {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrRtoutNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrRtoutClr = 1,
}
impl From<IntEvent0IclrRtout> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrRtout) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_RTOUT` writer - Clear SPI Receive Time-Out Event."]
pub type IntEvent0IclrRtoutW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrRtout>;
impl<'a, REG> IntEvent0IclrRtoutW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_rtout_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRtout::IntEvent0IclrRtoutNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_rtout_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRtout::IntEvent0IclrRtoutClr)
    }
}
#[doc = "Clear Receive FIFO event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrRx {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrRxNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrRxClr = 1,
}
impl From<IntEvent0IclrRx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_RX` writer - Clear Receive FIFO event."]
pub type IntEvent0IclrRxW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrRx>;
impl<'a, REG> IntEvent0IclrRxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_rx_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRx::IntEvent0IclrRxNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_rx_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRx::IntEvent0IclrRxClr)
    }
}
#[doc = "Clear Transmit FIFO event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrTx {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrTxNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrTxClr = 1,
}
impl From<IntEvent0IclrTx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_TX` writer - Clear Transmit FIFO event."]
pub type IntEvent0IclrTxW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrTx>;
impl<'a, REG> IntEvent0IclrTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_tx_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrTx::IntEvent0IclrTxNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_tx_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrTx::IntEvent0IclrTxClr)
    }
}
#[doc = "Clear Transmit FIFO Empty event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrTxempty {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrTxemptyNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrTxemptyClr = 1,
}
impl From<IntEvent0IclrTxempty> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrTxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_TXEMPTY` writer - Clear Transmit FIFO Empty event."]
pub type IntEvent0IclrTxemptyW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrTxempty>;
impl<'a, REG> IntEvent0IclrTxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_txempty_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrTxempty::IntEvent0IclrTxemptyNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_txempty_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrTxempty::IntEvent0IclrTxemptyClr)
    }
}
#[doc = "Clear SPI IDLE mode event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrIdle {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrIdleNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrIdleClr = 1,
}
impl From<IntEvent0IclrIdle> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrIdle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_IDLE` writer - Clear SPI IDLE mode event."]
pub type IntEvent0IclrIdleW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrIdle>;
impl<'a, REG> IntEvent0IclrIdleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_idle_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrIdle::IntEvent0IclrIdleNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_idle_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrIdle::IntEvent0IclrIdleClr)
    }
}
#[doc = "Clear DMA Done 1 event for RX.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDmaDoneRx {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDmaDoneRxNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDmaDoneRxClr = 1,
}
impl From<IntEvent0IclrDmaDoneRx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDmaDoneRx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DMA_DONE_RX` writer - Clear DMA Done 1 event for RX."]
pub type IntEvent0IclrDmaDoneRxW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDmaDoneRx>;
impl<'a, REG> IntEvent0IclrDmaDoneRxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dma_done_rx_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDmaDoneRx::IntEvent0IclrDmaDoneRxNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dma_done_rx_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDmaDoneRx::IntEvent0IclrDmaDoneRxClr)
    }
}
#[doc = "Clear DMA Done 1 event for TX.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDmaDoneTx {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDmaDoneTxNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDmaDoneTxClr = 1,
}
impl From<IntEvent0IclrDmaDoneTx> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDmaDoneTx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DMA_DONE_TX` writer - Clear DMA Done 1 event for TX."]
pub type IntEvent0IclrDmaDoneTxW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDmaDoneTx>;
impl<'a, REG> IntEvent0IclrDmaDoneTxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dma_done_tx_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDmaDoneTx::IntEvent0IclrDmaDoneTxNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dma_done_tx_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDmaDoneTx::IntEvent0IclrDmaDoneTxClr)
    }
}
#[doc = "Clear TXFIFO underflow event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrTxfifoUnf {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrTxfifoUnfNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrTxfifoUnfClr = 1,
}
impl From<IntEvent0IclrTxfifoUnf> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrTxfifoUnf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_TXFIFO_UNF` writer - Clear TXFIFO underflow event"]
pub type IntEvent0IclrTxfifoUnfW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrTxfifoUnf>;
impl<'a, REG> IntEvent0IclrTxfifoUnfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_txfifo_unf_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrTxfifoUnf::IntEvent0IclrTxfifoUnfNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_txfifo_unf_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrTxfifoUnf::IntEvent0IclrTxfifoUnfClr)
    }
}
#[doc = "Clear RX FIFO underflow event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrRxfull {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrRxfullNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrRxfullClr = 1,
}
impl From<IntEvent0IclrRxfull> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrRxfull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_RXFULL` writer - Clear RX FIFO underflow event"]
pub type IntEvent0IclrRxfullW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrRxfull>;
impl<'a, REG> IntEvent0IclrRxfullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_rxfull_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRxfull::IntEvent0IclrRxfullNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_rxfull_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRxfull::IntEvent0IclrRxfullClr)
    }
}
impl W {
    #[doc = "Bit 0 - Clear RXFIFO overflow event."]
    #[inline(always)]
    pub fn int_event0_iclr_rxfifo_ovf(&mut self) -> IntEvent0IclrRxfifoOvfW<IntEvent0IclrSpec> {
        IntEvent0IclrRxfifoOvfW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Parity error event."]
    #[inline(always)]
    pub fn int_event0_iclr_per(&mut self) -> IntEvent0IclrPerW<IntEvent0IclrSpec> {
        IntEvent0IclrPerW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear SPI Receive Time-Out Event."]
    #[inline(always)]
    pub fn int_event0_iclr_rtout(&mut self) -> IntEvent0IclrRtoutW<IntEvent0IclrSpec> {
        IntEvent0IclrRtoutW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Receive FIFO event."]
    #[inline(always)]
    pub fn int_event0_iclr_rx(&mut self) -> IntEvent0IclrRxW<IntEvent0IclrSpec> {
        IntEvent0IclrRxW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear Transmit FIFO event."]
    #[inline(always)]
    pub fn int_event0_iclr_tx(&mut self) -> IntEvent0IclrTxW<IntEvent0IclrSpec> {
        IntEvent0IclrTxW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear Transmit FIFO Empty event."]
    #[inline(always)]
    pub fn int_event0_iclr_txempty(&mut self) -> IntEvent0IclrTxemptyW<IntEvent0IclrSpec> {
        IntEvent0IclrTxemptyW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear SPI IDLE mode event."]
    #[inline(always)]
    pub fn int_event0_iclr_idle(&mut self) -> IntEvent0IclrIdleW<IntEvent0IclrSpec> {
        IntEvent0IclrIdleW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear DMA Done 1 event for RX."]
    #[inline(always)]
    pub fn int_event0_iclr_dma_done_rx(&mut self) -> IntEvent0IclrDmaDoneRxW<IntEvent0IclrSpec> {
        IntEvent0IclrDmaDoneRxW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear DMA Done 1 event for TX."]
    #[inline(always)]
    pub fn int_event0_iclr_dma_done_tx(&mut self) -> IntEvent0IclrDmaDoneTxW<IntEvent0IclrSpec> {
        IntEvent0IclrDmaDoneTxW::new(self, 8)
    }
    #[doc = "Bit 9 - Clear TXFIFO underflow event"]
    #[inline(always)]
    pub fn int_event0_iclr_txfifo_unf(&mut self) -> IntEvent0IclrTxfifoUnfW<IntEvent0IclrSpec> {
        IntEvent0IclrTxfifoUnfW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear RX FIFO underflow event"]
    #[inline(always)]
    pub fn int_event0_iclr_rxfull(&mut self) -> IntEvent0IclrRxfullW<IntEvent0IclrSpec> {
        IntEvent0IclrRxfullW::new(self, 10)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event0_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0IclrSpec;
impl crate::RegisterSpec for IntEvent0IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event0_iclr::W`](W) writer structure"]
impl crate::Writable for IntEvent0IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT0_ICLR to value 0"]
impl crate::Resettable for IntEvent0IclrSpec {
    const RESET_VALUE: u32 = 0;
}
