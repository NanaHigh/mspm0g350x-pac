#[doc = "Register `INT_EVENT0_ICLR` writer"]
pub type W = crate::W<IntEvent0IclrSpec>;
#[doc = "Clear UARTOUT Receive Time-Out Interrupt.\n\nValue on reset: 0"]
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
#[doc = "Field `INT_EVENT0_ICLR_RTOUT` writer - Clear UARTOUT Receive Time-Out Interrupt."]
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
#[doc = "Clear UART Framing Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrFrmerr {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrFrmerrNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrFrmerrClr = 1,
}
impl From<IntEvent0IclrFrmerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrFrmerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_FRMERR` writer - Clear UART Framing Error Interrupt."]
pub type IntEvent0IclrFrmerrW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrFrmerr>;
impl<'a, REG> IntEvent0IclrFrmerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_frmerr_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrFrmerr::IntEvent0IclrFrmerrNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_frmerr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrFrmerr::IntEvent0IclrFrmerrClr)
    }
}
#[doc = "Clear UART Parity Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrParerr {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrParerrNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrParerrClr = 1,
}
impl From<IntEvent0IclrParerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrParerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_PARERR` writer - Clear UART Parity Error Interrupt."]
pub type IntEvent0IclrParerrW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrParerr>;
impl<'a, REG> IntEvent0IclrParerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_parerr_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrParerr::IntEvent0IclrParerrNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_parerr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrParerr::IntEvent0IclrParerrClr)
    }
}
#[doc = "Clear UART Break Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrBrkerr {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrBrkerrNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrBrkerrClr = 1,
}
impl From<IntEvent0IclrBrkerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrBrkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_BRKERR` writer - Clear UART Break Error Interrupt."]
pub type IntEvent0IclrBrkerrW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrBrkerr>;
impl<'a, REG> IntEvent0IclrBrkerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_brkerr_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrBrkerr::IntEvent0IclrBrkerrNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_brkerr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrBrkerr::IntEvent0IclrBrkerrClr)
    }
}
#[doc = "Clear UART Receive Overrun Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrOvrerr {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrOvrerrNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrOvrerrClr = 1,
}
impl From<IntEvent0IclrOvrerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrOvrerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_OVRERR` writer - Clear UART Receive Overrun Error Interrupt."]
pub type IntEvent0IclrOvrerrW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrOvrerr>;
impl<'a, REG> IntEvent0IclrOvrerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_ovrerr_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrOvrerr::IntEvent0IclrOvrerrNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_ovrerr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrOvrerr::IntEvent0IclrOvrerrClr)
    }
}
#[doc = "Clear Negative Edge on UARTxRXD Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrRxne {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrRxneNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrRxneClr = 1,
}
impl From<IntEvent0IclrRxne> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrRxne) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_RXNE` writer - Clear Negative Edge on UARTxRXD Interrupt."]
pub type IntEvent0IclrRxneW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrRxne>;
impl<'a, REG> IntEvent0IclrRxneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_rxne_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRxne::IntEvent0IclrRxneNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_rxne_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRxne::IntEvent0IclrRxneClr)
    }
}
#[doc = "Clear Positive Edge on UARTxRXD Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrRxpe {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrRxpeNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrRxpeClr = 1,
}
impl From<IntEvent0IclrRxpe> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrRxpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_RXPE` writer - Clear Positive Edge on UARTxRXD Interrupt."]
pub type IntEvent0IclrRxpeW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrRxpe>;
impl<'a, REG> IntEvent0IclrRxpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_rxpe_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRxpe::IntEvent0IclrRxpeNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_rxpe_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRxpe::IntEvent0IclrRxpeClr)
    }
}
#[doc = "Clear UART Receive Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrRxint {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrRxintNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrRxintClr = 1,
}
impl From<IntEvent0IclrRxint> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrRxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_RXINT` writer - Clear UART Receive Interrupt."]
pub type IntEvent0IclrRxintW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrRxint>;
impl<'a, REG> IntEvent0IclrRxintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_rxint_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRxint::IntEvent0IclrRxintNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_rxint_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrRxint::IntEvent0IclrRxintClr)
    }
}
#[doc = "Clear UART Transmit Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrTxint {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrTxintNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrTxintClr = 1,
}
impl From<IntEvent0IclrTxint> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrTxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_TXINT` writer - Clear UART Transmit Interrupt."]
pub type IntEvent0IclrTxintW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrTxint>;
impl<'a, REG> IntEvent0IclrTxintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_txint_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrTxint::IntEvent0IclrTxintNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_txint_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrTxint::IntEvent0IclrTxintClr)
    }
}
#[doc = "Clear UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrEot {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrEotNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrEotClr = 1,
}
impl From<IntEvent0IclrEot> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrEot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_EOT` writer - Clear UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
pub type IntEvent0IclrEotW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrEot>;
impl<'a, REG> IntEvent0IclrEotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_eot_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrEot::IntEvent0IclrEotNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_eot_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrEot::IntEvent0IclrEotClr)
    }
}
#[doc = "Clear Address Match Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrAddrMatch {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrAddrMatchNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrAddrMatchClr = 1,
}
impl From<IntEvent0IclrAddrMatch> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrAddrMatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_ADDR_MATCH` writer - Clear Address Match Interrupt."]
pub type IntEvent0IclrAddrMatchW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrAddrMatch>;
impl<'a, REG> IntEvent0IclrAddrMatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_addr_match_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrAddrMatch::IntEvent0IclrAddrMatchNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_addr_match_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrAddrMatch::IntEvent0IclrAddrMatchClr)
    }
}
#[doc = "Clear UART Clear to Send Modem Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrCts {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrCtsNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrCtsClr = 1,
}
impl From<IntEvent0IclrCts> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrCts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_CTS` writer - Clear UART Clear to Send Modem Interrupt."]
pub type IntEvent0IclrCtsW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrCts>;
impl<'a, REG> IntEvent0IclrCtsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_cts_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrCts::IntEvent0IclrCtsNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_cts_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrCts::IntEvent0IclrCtsClr)
    }
}
#[doc = "Clear DMA Done on RX Event Channel Interrupt\n\nValue on reset: 0"]
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
#[doc = "Field `INT_EVENT0_ICLR_DMA_DONE_RX` writer - Clear DMA Done on RX Event Channel Interrupt"]
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
#[doc = "Clear DMA Done on TX Event Channel Interrupt\n\nValue on reset: 0"]
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
#[doc = "Field `INT_EVENT0_ICLR_DMA_DONE_TX` writer - Clear DMA Done on TX Event Channel Interrupt"]
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
#[doc = "Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrNerr {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrNerrNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrNerrClr = 1,
}
impl From<IntEvent0IclrNerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrNerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_NERR` writer - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
pub type IntEvent0IclrNerrW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrNerr>;
impl<'a, REG> IntEvent0IclrNerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_nerr_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrNerr::IntEvent0IclrNerrNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_nerr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrNerr::IntEvent0IclrNerrClr)
    }
}
impl W {
    #[doc = "Bit 0 - Clear UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn int_event0_iclr_rtout(&mut self) -> IntEvent0IclrRtoutW<IntEvent0IclrSpec> {
        IntEvent0IclrRtoutW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear UART Framing Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_iclr_frmerr(&mut self) -> IntEvent0IclrFrmerrW<IntEvent0IclrSpec> {
        IntEvent0IclrFrmerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear UART Parity Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_iclr_parerr(&mut self) -> IntEvent0IclrParerrW<IntEvent0IclrSpec> {
        IntEvent0IclrParerrW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear UART Break Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_iclr_brkerr(&mut self) -> IntEvent0IclrBrkerrW<IntEvent0IclrSpec> {
        IntEvent0IclrBrkerrW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear UART Receive Overrun Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_iclr_ovrerr(&mut self) -> IntEvent0IclrOvrerrW<IntEvent0IclrSpec> {
        IntEvent0IclrOvrerrW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear Negative Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn int_event0_iclr_rxne(&mut self) -> IntEvent0IclrRxneW<IntEvent0IclrSpec> {
        IntEvent0IclrRxneW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear Positive Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn int_event0_iclr_rxpe(&mut self) -> IntEvent0IclrRxpeW<IntEvent0IclrSpec> {
        IntEvent0IclrRxpeW::new(self, 6)
    }
    #[doc = "Bit 10 - Clear UART Receive Interrupt."]
    #[inline(always)]
    pub fn int_event0_iclr_rxint(&mut self) -> IntEvent0IclrRxintW<IntEvent0IclrSpec> {
        IntEvent0IclrRxintW::new(self, 10)
    }
    #[doc = "Bit 11 - Clear UART Transmit Interrupt."]
    #[inline(always)]
    pub fn int_event0_iclr_txint(&mut self) -> IntEvent0IclrTxintW<IntEvent0IclrSpec> {
        IntEvent0IclrTxintW::new(self, 11)
    }
    #[doc = "Bit 12 - Clear UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
    #[inline(always)]
    pub fn int_event0_iclr_eot(&mut self) -> IntEvent0IclrEotW<IntEvent0IclrSpec> {
        IntEvent0IclrEotW::new(self, 12)
    }
    #[doc = "Bit 13 - Clear Address Match Interrupt."]
    #[inline(always)]
    pub fn int_event0_iclr_addr_match(&mut self) -> IntEvent0IclrAddrMatchW<IntEvent0IclrSpec> {
        IntEvent0IclrAddrMatchW::new(self, 13)
    }
    #[doc = "Bit 14 - Clear UART Clear to Send Modem Interrupt."]
    #[inline(always)]
    pub fn int_event0_iclr_cts(&mut self) -> IntEvent0IclrCtsW<IntEvent0IclrSpec> {
        IntEvent0IclrCtsW::new(self, 14)
    }
    #[doc = "Bit 15 - Clear DMA Done on RX Event Channel Interrupt"]
    #[inline(always)]
    pub fn int_event0_iclr_dma_done_rx(&mut self) -> IntEvent0IclrDmaDoneRxW<IntEvent0IclrSpec> {
        IntEvent0IclrDmaDoneRxW::new(self, 15)
    }
    #[doc = "Bit 16 - Clear DMA Done on TX Event Channel Interrupt"]
    #[inline(always)]
    pub fn int_event0_iclr_dma_done_tx(&mut self) -> IntEvent0IclrDmaDoneTxW<IntEvent0IclrSpec> {
        IntEvent0IclrDmaDoneTxW::new(self, 16)
    }
    #[doc = "Bit 17 - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
    #[inline(always)]
    pub fn int_event0_iclr_nerr(&mut self) -> IntEvent0IclrNerrW<IntEvent0IclrSpec> {
        IntEvent0IclrNerrW::new(self, 17)
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
