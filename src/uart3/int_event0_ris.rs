#[doc = "Register `INT_EVENT0_RIS` reader"]
pub type R = crate::R<IntEvent0RisSpec>;
#[doc = "UARTOUT Receive Time-Out Interrupt.\n\nValue on reset: 0"]
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
#[doc = "Field `INT_EVENT0_RIS_RTOUT` reader - UARTOUT Receive Time-Out Interrupt."]
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
#[doc = "UART Framing Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisFrmerr {
    #[doc = "0: CLR"]
    IntEvent0RisFrmerrClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisFrmerrSet = 1,
}
impl From<IntEvent0RisFrmerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisFrmerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_FRMERR` reader - UART Framing Error Interrupt."]
pub type IntEvent0RisFrmerrR = crate::BitReader<IntEvent0RisFrmerr>;
impl IntEvent0RisFrmerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisFrmerr {
        match self.bits {
            false => IntEvent0RisFrmerr::IntEvent0RisFrmerrClr,
            true => IntEvent0RisFrmerr::IntEvent0RisFrmerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_frmerr_clr(&self) -> bool {
        *self == IntEvent0RisFrmerr::IntEvent0RisFrmerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_frmerr_set(&self) -> bool {
        *self == IntEvent0RisFrmerr::IntEvent0RisFrmerrSet
    }
}
#[doc = "UART Parity Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisParerr {
    #[doc = "0: CLR"]
    IntEvent0RisParerrClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisParerrSet = 1,
}
impl From<IntEvent0RisParerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisParerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_PARERR` reader - UART Parity Error Interrupt."]
pub type IntEvent0RisParerrR = crate::BitReader<IntEvent0RisParerr>;
impl IntEvent0RisParerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisParerr {
        match self.bits {
            false => IntEvent0RisParerr::IntEvent0RisParerrClr,
            true => IntEvent0RisParerr::IntEvent0RisParerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_parerr_clr(&self) -> bool {
        *self == IntEvent0RisParerr::IntEvent0RisParerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_parerr_set(&self) -> bool {
        *self == IntEvent0RisParerr::IntEvent0RisParerrSet
    }
}
#[doc = "UART Break Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisBrkerr {
    #[doc = "0: CLR"]
    IntEvent0RisBrkerrClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisBrkerrSet = 1,
}
impl From<IntEvent0RisBrkerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisBrkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_BRKERR` reader - UART Break Error Interrupt."]
pub type IntEvent0RisBrkerrR = crate::BitReader<IntEvent0RisBrkerr>;
impl IntEvent0RisBrkerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisBrkerr {
        match self.bits {
            false => IntEvent0RisBrkerr::IntEvent0RisBrkerrClr,
            true => IntEvent0RisBrkerr::IntEvent0RisBrkerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_brkerr_clr(&self) -> bool {
        *self == IntEvent0RisBrkerr::IntEvent0RisBrkerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_brkerr_set(&self) -> bool {
        *self == IntEvent0RisBrkerr::IntEvent0RisBrkerrSet
    }
}
#[doc = "UART Receive Overrun Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisOvrerr {
    #[doc = "0: CLR"]
    IntEvent0RisOvrerrClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisOvrerrSet = 1,
}
impl From<IntEvent0RisOvrerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisOvrerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_OVRERR` reader - UART Receive Overrun Error Interrupt."]
pub type IntEvent0RisOvrerrR = crate::BitReader<IntEvent0RisOvrerr>;
impl IntEvent0RisOvrerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisOvrerr {
        match self.bits {
            false => IntEvent0RisOvrerr::IntEvent0RisOvrerrClr,
            true => IntEvent0RisOvrerr::IntEvent0RisOvrerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_ovrerr_clr(&self) -> bool {
        *self == IntEvent0RisOvrerr::IntEvent0RisOvrerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_ovrerr_set(&self) -> bool {
        *self == IntEvent0RisOvrerr::IntEvent0RisOvrerrSet
    }
}
#[doc = "Negative Edge on UARTxRXD Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisRxne {
    #[doc = "0: CLR"]
    IntEvent0RisRxneClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisRxneSet = 1,
}
impl From<IntEvent0RisRxne> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisRxne) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_RXNE` reader - Negative Edge on UARTxRXD Interrupt."]
pub type IntEvent0RisRxneR = crate::BitReader<IntEvent0RisRxne>;
impl IntEvent0RisRxneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisRxne {
        match self.bits {
            false => IntEvent0RisRxne::IntEvent0RisRxneClr,
            true => IntEvent0RisRxne::IntEvent0RisRxneSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_rxne_clr(&self) -> bool {
        *self == IntEvent0RisRxne::IntEvent0RisRxneClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_rxne_set(&self) -> bool {
        *self == IntEvent0RisRxne::IntEvent0RisRxneSet
    }
}
#[doc = "Positive Edge on UARTxRXD Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisRxpe {
    #[doc = "0: CLR"]
    IntEvent0RisRxpeClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisRxpeSet = 1,
}
impl From<IntEvent0RisRxpe> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisRxpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_RXPE` reader - Positive Edge on UARTxRXD Interrupt."]
pub type IntEvent0RisRxpeR = crate::BitReader<IntEvent0RisRxpe>;
impl IntEvent0RisRxpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisRxpe {
        match self.bits {
            false => IntEvent0RisRxpe::IntEvent0RisRxpeClr,
            true => IntEvent0RisRxpe::IntEvent0RisRxpeSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_rxpe_clr(&self) -> bool {
        *self == IntEvent0RisRxpe::IntEvent0RisRxpeClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_rxpe_set(&self) -> bool {
        *self == IntEvent0RisRxpe::IntEvent0RisRxpeSet
    }
}
#[doc = "UART Receive Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisRxint {
    #[doc = "0: CLR"]
    IntEvent0RisRxintClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisRxintSet = 1,
}
impl From<IntEvent0RisRxint> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisRxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_RXINT` reader - UART Receive Interrupt."]
pub type IntEvent0RisRxintR = crate::BitReader<IntEvent0RisRxint>;
impl IntEvent0RisRxintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisRxint {
        match self.bits {
            false => IntEvent0RisRxint::IntEvent0RisRxintClr,
            true => IntEvent0RisRxint::IntEvent0RisRxintSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_rxint_clr(&self) -> bool {
        *self == IntEvent0RisRxint::IntEvent0RisRxintClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_rxint_set(&self) -> bool {
        *self == IntEvent0RisRxint::IntEvent0RisRxintSet
    }
}
#[doc = "UART Transmit Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisTxint {
    #[doc = "0: CLR"]
    IntEvent0RisTxintClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisTxintSet = 1,
}
impl From<IntEvent0RisTxint> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisTxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_TXINT` reader - UART Transmit Interrupt."]
pub type IntEvent0RisTxintR = crate::BitReader<IntEvent0RisTxint>;
impl IntEvent0RisTxintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisTxint {
        match self.bits {
            false => IntEvent0RisTxint::IntEvent0RisTxintClr,
            true => IntEvent0RisTxint::IntEvent0RisTxintSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_txint_clr(&self) -> bool {
        *self == IntEvent0RisTxint::IntEvent0RisTxintClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_txint_set(&self) -> bool {
        *self == IntEvent0RisTxint::IntEvent0RisTxintSet
    }
}
#[doc = "UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisEot {
    #[doc = "0: CLR"]
    IntEvent0RisEotClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisEotSet = 1,
}
impl From<IntEvent0RisEot> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisEot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_EOT` reader - UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
pub type IntEvent0RisEotR = crate::BitReader<IntEvent0RisEot>;
impl IntEvent0RisEotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisEot {
        match self.bits {
            false => IntEvent0RisEot::IntEvent0RisEotClr,
            true => IntEvent0RisEot::IntEvent0RisEotSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_eot_clr(&self) -> bool {
        *self == IntEvent0RisEot::IntEvent0RisEotClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_eot_set(&self) -> bool {
        *self == IntEvent0RisEot::IntEvent0RisEotSet
    }
}
#[doc = "Address Match Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisAddrMatch {
    #[doc = "0: CLR"]
    IntEvent0RisAddrMatchClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisAddrMatchSet = 1,
}
impl From<IntEvent0RisAddrMatch> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisAddrMatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_ADDR_MATCH` reader - Address Match Interrupt."]
pub type IntEvent0RisAddrMatchR = crate::BitReader<IntEvent0RisAddrMatch>;
impl IntEvent0RisAddrMatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisAddrMatch {
        match self.bits {
            false => IntEvent0RisAddrMatch::IntEvent0RisAddrMatchClr,
            true => IntEvent0RisAddrMatch::IntEvent0RisAddrMatchSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_addr_match_clr(&self) -> bool {
        *self == IntEvent0RisAddrMatch::IntEvent0RisAddrMatchClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_addr_match_set(&self) -> bool {
        *self == IntEvent0RisAddrMatch::IntEvent0RisAddrMatchSet
    }
}
#[doc = "UART Clear to Send Modem Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisCts {
    #[doc = "0: CLR"]
    IntEvent0RisCtsClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisCtsSet = 1,
}
impl From<IntEvent0RisCts> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisCts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_CTS` reader - UART Clear to Send Modem Interrupt."]
pub type IntEvent0RisCtsR = crate::BitReader<IntEvent0RisCts>;
impl IntEvent0RisCtsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisCts {
        match self.bits {
            false => IntEvent0RisCts::IntEvent0RisCtsClr,
            true => IntEvent0RisCts::IntEvent0RisCtsSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_cts_clr(&self) -> bool {
        *self == IntEvent0RisCts::IntEvent0RisCtsClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_cts_set(&self) -> bool {
        *self == IntEvent0RisCts::IntEvent0RisCtsSet
    }
}
#[doc = "DMA Done on RX Event Channel Interrupt\n\nValue on reset: 0"]
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
#[doc = "Field `INT_EVENT0_RIS_DMA_DONE_RX` reader - DMA Done on RX Event Channel Interrupt"]
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
#[doc = "DMA Done on TX Event Channel Interrupt\n\nValue on reset: 0"]
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
#[doc = "Field `INT_EVENT0_RIS_DMA_DONE_TX` reader - DMA Done on TX Event Channel Interrupt"]
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
#[doc = "Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisNerr {
    #[doc = "0: CLR"]
    IntEvent0RisNerrClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisNerrSet = 1,
}
impl From<IntEvent0RisNerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisNerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_NERR` reader - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
pub type IntEvent0RisNerrR = crate::BitReader<IntEvent0RisNerr>;
impl IntEvent0RisNerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisNerr {
        match self.bits {
            false => IntEvent0RisNerr::IntEvent0RisNerrClr,
            true => IntEvent0RisNerr::IntEvent0RisNerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_nerr_clr(&self) -> bool {
        *self == IntEvent0RisNerr::IntEvent0RisNerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_nerr_set(&self) -> bool {
        *self == IntEvent0RisNerr::IntEvent0RisNerrSet
    }
}
impl R {
    #[doc = "Bit 0 - UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn int_event0_ris_rtout(&self) -> IntEvent0RisRtoutR {
        IntEvent0RisRtoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART Framing Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_ris_frmerr(&self) -> IntEvent0RisFrmerrR {
        IntEvent0RisFrmerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - UART Parity Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_ris_parerr(&self) -> IntEvent0RisParerrR {
        IntEvent0RisParerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UART Break Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_ris_brkerr(&self) -> IntEvent0RisBrkerrR {
        IntEvent0RisBrkerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - UART Receive Overrun Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_ris_ovrerr(&self) -> IntEvent0RisOvrerrR {
        IntEvent0RisOvrerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Negative Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn int_event0_ris_rxne(&self) -> IntEvent0RisRxneR {
        IntEvent0RisRxneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Positive Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn int_event0_ris_rxpe(&self) -> IntEvent0RisRxpeR {
        IntEvent0RisRxpeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - UART Receive Interrupt."]
    #[inline(always)]
    pub fn int_event0_ris_rxint(&self) -> IntEvent0RisRxintR {
        IntEvent0RisRxintR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UART Transmit Interrupt."]
    #[inline(always)]
    pub fn int_event0_ris_txint(&self) -> IntEvent0RisTxintR {
        IntEvent0RisTxintR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
    #[inline(always)]
    pub fn int_event0_ris_eot(&self) -> IntEvent0RisEotR {
        IntEvent0RisEotR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Address Match Interrupt."]
    #[inline(always)]
    pub fn int_event0_ris_addr_match(&self) -> IntEvent0RisAddrMatchR {
        IntEvent0RisAddrMatchR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - UART Clear to Send Modem Interrupt."]
    #[inline(always)]
    pub fn int_event0_ris_cts(&self) -> IntEvent0RisCtsR {
        IntEvent0RisCtsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA Done on RX Event Channel Interrupt"]
    #[inline(always)]
    pub fn int_event0_ris_dma_done_rx(&self) -> IntEvent0RisDmaDoneRxR {
        IntEvent0RisDmaDoneRxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA Done on TX Event Channel Interrupt"]
    #[inline(always)]
    pub fn int_event0_ris_dma_done_tx(&self) -> IntEvent0RisDmaDoneTxR {
        IntEvent0RisDmaDoneTxR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
    #[inline(always)]
    pub fn int_event0_ris_nerr(&self) -> IntEvent0RisNerrR {
        IntEvent0RisNerrR::new(((self.bits >> 17) & 1) != 0)
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
