#[doc = "Register `INT_EVENT0_ISET` writer"]
pub type W = crate::W<IntEvent0IsetSpec>;
#[doc = "Set UARTOUT Receive Time-Out Interrupt.\n\nValue on reset: 0"]
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
#[doc = "Field `INT_EVENT0_ISET_RTOUT` writer - Set UARTOUT Receive Time-Out Interrupt."]
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
#[doc = "Set UART Framing Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetFrmerr {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetFrmerrNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetFrmerrSet = 1,
}
impl From<IntEvent0IsetFrmerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetFrmerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_FRMERR` writer - Set UART Framing Error Interrupt."]
pub type IntEvent0IsetFrmerrW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetFrmerr>;
impl<'a, REG> IntEvent0IsetFrmerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_frmerr_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetFrmerr::IntEvent0IsetFrmerrNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_frmerr_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetFrmerr::IntEvent0IsetFrmerrSet)
    }
}
#[doc = "Set UART Parity Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetParerr {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetParerrNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetParerrSet = 1,
}
impl From<IntEvent0IsetParerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetParerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_PARERR` writer - Set UART Parity Error Interrupt."]
pub type IntEvent0IsetParerrW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetParerr>;
impl<'a, REG> IntEvent0IsetParerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_parerr_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetParerr::IntEvent0IsetParerrNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_parerr_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetParerr::IntEvent0IsetParerrSet)
    }
}
#[doc = "Set UART Break Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetBrkerr {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetBrkerrNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetBrkerrSet = 1,
}
impl From<IntEvent0IsetBrkerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetBrkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_BRKERR` writer - Set UART Break Error Interrupt."]
pub type IntEvent0IsetBrkerrW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetBrkerr>;
impl<'a, REG> IntEvent0IsetBrkerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_brkerr_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetBrkerr::IntEvent0IsetBrkerrNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_brkerr_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetBrkerr::IntEvent0IsetBrkerrSet)
    }
}
#[doc = "Set UART Receive Overrun Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetOvrerr {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetOvrerrNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetOvrerrSet = 1,
}
impl From<IntEvent0IsetOvrerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetOvrerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_OVRERR` writer - Set UART Receive Overrun Error Interrupt."]
pub type IntEvent0IsetOvrerrW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetOvrerr>;
impl<'a, REG> IntEvent0IsetOvrerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_ovrerr_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetOvrerr::IntEvent0IsetOvrerrNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_ovrerr_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetOvrerr::IntEvent0IsetOvrerrSet)
    }
}
#[doc = "Set Negative Edge on UARTxRXD Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetRxne {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetRxneNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetRxneSet = 1,
}
impl From<IntEvent0IsetRxne> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetRxne) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_RXNE` writer - Set Negative Edge on UARTxRXD Interrupt."]
pub type IntEvent0IsetRxneW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetRxne>;
impl<'a, REG> IntEvent0IsetRxneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_rxne_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRxne::IntEvent0IsetRxneNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_rxne_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRxne::IntEvent0IsetRxneSet)
    }
}
#[doc = "Set Positive Edge on UARTxRXD Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetRxpe {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetRxpeNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetRxpeSet = 1,
}
impl From<IntEvent0IsetRxpe> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetRxpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_RXPE` writer - Set Positive Edge on UARTxRXD Interrupt."]
pub type IntEvent0IsetRxpeW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetRxpe>;
impl<'a, REG> IntEvent0IsetRxpeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_rxpe_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRxpe::IntEvent0IsetRxpeNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_rxpe_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRxpe::IntEvent0IsetRxpeSet)
    }
}
#[doc = "Set UART Receive Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetRxint {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetRxintNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetRxintSet = 1,
}
impl From<IntEvent0IsetRxint> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetRxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_RXINT` writer - Set UART Receive Interrupt."]
pub type IntEvent0IsetRxintW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetRxint>;
impl<'a, REG> IntEvent0IsetRxintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_rxint_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRxint::IntEvent0IsetRxintNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_rxint_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetRxint::IntEvent0IsetRxintSet)
    }
}
#[doc = "Set UART Transmit Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetTxint {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetTxintNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetTxintSet = 1,
}
impl From<IntEvent0IsetTxint> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetTxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_TXINT` writer - Set UART Transmit Interrupt."]
pub type IntEvent0IsetTxintW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetTxint>;
impl<'a, REG> IntEvent0IsetTxintW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_txint_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetTxint::IntEvent0IsetTxintNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_txint_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetTxint::IntEvent0IsetTxintSet)
    }
}
#[doc = "Set UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetEot {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetEotNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetEotSet = 1,
}
impl From<IntEvent0IsetEot> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetEot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_EOT` writer - Set UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
pub type IntEvent0IsetEotW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetEot>;
impl<'a, REG> IntEvent0IsetEotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_eot_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetEot::IntEvent0IsetEotNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_eot_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetEot::IntEvent0IsetEotSet)
    }
}
#[doc = "Set Address Match Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetAddrMatch {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetAddrMatchNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetAddrMatchSet = 1,
}
impl From<IntEvent0IsetAddrMatch> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetAddrMatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_ADDR_MATCH` writer - Set Address Match Interrupt."]
pub type IntEvent0IsetAddrMatchW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetAddrMatch>;
impl<'a, REG> IntEvent0IsetAddrMatchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_addr_match_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetAddrMatch::IntEvent0IsetAddrMatchNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_addr_match_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetAddrMatch::IntEvent0IsetAddrMatchSet)
    }
}
#[doc = "Set UART Clear to Send Modem Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetCts {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetCtsNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetCtsSet = 1,
}
impl From<IntEvent0IsetCts> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetCts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_CTS` writer - Set UART Clear to Send Modem Interrupt."]
pub type IntEvent0IsetCtsW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetCts>;
impl<'a, REG> IntEvent0IsetCtsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_cts_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetCts::IntEvent0IsetCtsNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_cts_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetCts::IntEvent0IsetCtsSet)
    }
}
#[doc = "Set DMA Done on RX Event Channel Interrupt\n\nValue on reset: 0"]
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
#[doc = "Field `INT_EVENT0_ISET_DMA_DONE_RX` writer - Set DMA Done on RX Event Channel Interrupt"]
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
#[doc = "Set DMA Done on TX Event Channel Interrupt\n\nValue on reset: 0"]
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
#[doc = "Field `INT_EVENT0_ISET_DMA_DONE_TX` writer - Set DMA Done on TX Event Channel Interrupt"]
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
#[doc = "Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetNerr {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetNerrNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetNerrSet = 1,
}
impl From<IntEvent0IsetNerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetNerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_NERR` writer - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
pub type IntEvent0IsetNerrW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetNerr>;
impl<'a, REG> IntEvent0IsetNerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_nerr_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetNerr::IntEvent0IsetNerrNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_nerr_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetNerr::IntEvent0IsetNerrSet)
    }
}
impl W {
    #[doc = "Bit 0 - Set UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn int_event0_iset_rtout(&mut self) -> IntEvent0IsetRtoutW<IntEvent0IsetSpec> {
        IntEvent0IsetRtoutW::new(self, 0)
    }
    #[doc = "Bit 1 - Set UART Framing Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_iset_frmerr(&mut self) -> IntEvent0IsetFrmerrW<IntEvent0IsetSpec> {
        IntEvent0IsetFrmerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Set UART Parity Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_iset_parerr(&mut self) -> IntEvent0IsetParerrW<IntEvent0IsetSpec> {
        IntEvent0IsetParerrW::new(self, 2)
    }
    #[doc = "Bit 3 - Set UART Break Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_iset_brkerr(&mut self) -> IntEvent0IsetBrkerrW<IntEvent0IsetSpec> {
        IntEvent0IsetBrkerrW::new(self, 3)
    }
    #[doc = "Bit 4 - Set UART Receive Overrun Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_iset_ovrerr(&mut self) -> IntEvent0IsetOvrerrW<IntEvent0IsetSpec> {
        IntEvent0IsetOvrerrW::new(self, 4)
    }
    #[doc = "Bit 5 - Set Negative Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn int_event0_iset_rxne(&mut self) -> IntEvent0IsetRxneW<IntEvent0IsetSpec> {
        IntEvent0IsetRxneW::new(self, 5)
    }
    #[doc = "Bit 6 - Set Positive Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn int_event0_iset_rxpe(&mut self) -> IntEvent0IsetRxpeW<IntEvent0IsetSpec> {
        IntEvent0IsetRxpeW::new(self, 6)
    }
    #[doc = "Bit 10 - Set UART Receive Interrupt."]
    #[inline(always)]
    pub fn int_event0_iset_rxint(&mut self) -> IntEvent0IsetRxintW<IntEvent0IsetSpec> {
        IntEvent0IsetRxintW::new(self, 10)
    }
    #[doc = "Bit 11 - Set UART Transmit Interrupt."]
    #[inline(always)]
    pub fn int_event0_iset_txint(&mut self) -> IntEvent0IsetTxintW<IntEvent0IsetSpec> {
        IntEvent0IsetTxintW::new(self, 11)
    }
    #[doc = "Bit 12 - Set UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
    #[inline(always)]
    pub fn int_event0_iset_eot(&mut self) -> IntEvent0IsetEotW<IntEvent0IsetSpec> {
        IntEvent0IsetEotW::new(self, 12)
    }
    #[doc = "Bit 13 - Set Address Match Interrupt."]
    #[inline(always)]
    pub fn int_event0_iset_addr_match(&mut self) -> IntEvent0IsetAddrMatchW<IntEvent0IsetSpec> {
        IntEvent0IsetAddrMatchW::new(self, 13)
    }
    #[doc = "Bit 14 - Set UART Clear to Send Modem Interrupt."]
    #[inline(always)]
    pub fn int_event0_iset_cts(&mut self) -> IntEvent0IsetCtsW<IntEvent0IsetSpec> {
        IntEvent0IsetCtsW::new(self, 14)
    }
    #[doc = "Bit 15 - Set DMA Done on RX Event Channel Interrupt"]
    #[inline(always)]
    pub fn int_event0_iset_dma_done_rx(&mut self) -> IntEvent0IsetDmaDoneRxW<IntEvent0IsetSpec> {
        IntEvent0IsetDmaDoneRxW::new(self, 15)
    }
    #[doc = "Bit 16 - Set DMA Done on TX Event Channel Interrupt"]
    #[inline(always)]
    pub fn int_event0_iset_dma_done_tx(&mut self) -> IntEvent0IsetDmaDoneTxW<IntEvent0IsetSpec> {
        IntEvent0IsetDmaDoneTxW::new(self, 16)
    }
    #[doc = "Bit 17 - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
    #[inline(always)]
    pub fn int_event0_iset_nerr(&mut self) -> IntEvent0IsetNerrW<IntEvent0IsetSpec> {
        IntEvent0IsetNerrW::new(self, 17)
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
