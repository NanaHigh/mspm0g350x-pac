#[doc = "Register `INT_EVENT0_MIS` reader"]
pub type R = crate::R<IntEvent0MisSpec>;
#[doc = "Masked UARTOUT Receive Time-Out Interrupt.\n\nValue on reset: 0"]
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
#[doc = "Field `INT_EVENT0_MIS_RTOUT` reader - Masked UARTOUT Receive Time-Out Interrupt."]
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
#[doc = "Masked UART Framing Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisFrmerr {
    #[doc = "0: CLR"]
    IntEvent0MisFrmerrClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisFrmerrSet = 1,
}
impl From<IntEvent0MisFrmerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisFrmerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_FRMERR` reader - Masked UART Framing Error Interrupt."]
pub type IntEvent0MisFrmerrR = crate::BitReader<IntEvent0MisFrmerr>;
impl IntEvent0MisFrmerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisFrmerr {
        match self.bits {
            false => IntEvent0MisFrmerr::IntEvent0MisFrmerrClr,
            true => IntEvent0MisFrmerr::IntEvent0MisFrmerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_frmerr_clr(&self) -> bool {
        *self == IntEvent0MisFrmerr::IntEvent0MisFrmerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_frmerr_set(&self) -> bool {
        *self == IntEvent0MisFrmerr::IntEvent0MisFrmerrSet
    }
}
#[doc = "Masked UART Parity Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisParerr {
    #[doc = "0: CLR"]
    IntEvent0MisParerrClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisParerrSet = 1,
}
impl From<IntEvent0MisParerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisParerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_PARERR` reader - Masked UART Parity Error Interrupt."]
pub type IntEvent0MisParerrR = crate::BitReader<IntEvent0MisParerr>;
impl IntEvent0MisParerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisParerr {
        match self.bits {
            false => IntEvent0MisParerr::IntEvent0MisParerrClr,
            true => IntEvent0MisParerr::IntEvent0MisParerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_parerr_clr(&self) -> bool {
        *self == IntEvent0MisParerr::IntEvent0MisParerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_parerr_set(&self) -> bool {
        *self == IntEvent0MisParerr::IntEvent0MisParerrSet
    }
}
#[doc = "Masked UART Break Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisBrkerr {
    #[doc = "0: CLR"]
    IntEvent0MisBrkerrClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisBrkerrSet = 1,
}
impl From<IntEvent0MisBrkerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisBrkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_BRKERR` reader - Masked UART Break Error Interrupt."]
pub type IntEvent0MisBrkerrR = crate::BitReader<IntEvent0MisBrkerr>;
impl IntEvent0MisBrkerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisBrkerr {
        match self.bits {
            false => IntEvent0MisBrkerr::IntEvent0MisBrkerrClr,
            true => IntEvent0MisBrkerr::IntEvent0MisBrkerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_brkerr_clr(&self) -> bool {
        *self == IntEvent0MisBrkerr::IntEvent0MisBrkerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_brkerr_set(&self) -> bool {
        *self == IntEvent0MisBrkerr::IntEvent0MisBrkerrSet
    }
}
#[doc = "Masked UART Receive Overrun Error Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisOvrerr {
    #[doc = "0: CLR"]
    IntEvent0MisOvrerrClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisOvrerrSet = 1,
}
impl From<IntEvent0MisOvrerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisOvrerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_OVRERR` reader - Masked UART Receive Overrun Error Interrupt."]
pub type IntEvent0MisOvrerrR = crate::BitReader<IntEvent0MisOvrerr>;
impl IntEvent0MisOvrerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisOvrerr {
        match self.bits {
            false => IntEvent0MisOvrerr::IntEvent0MisOvrerrClr,
            true => IntEvent0MisOvrerr::IntEvent0MisOvrerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_ovrerr_clr(&self) -> bool {
        *self == IntEvent0MisOvrerr::IntEvent0MisOvrerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_ovrerr_set(&self) -> bool {
        *self == IntEvent0MisOvrerr::IntEvent0MisOvrerrSet
    }
}
#[doc = "Masked Negative Edge on UARTxRXD Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisRxne {
    #[doc = "0: CLR"]
    IntEvent0MisRxneClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisRxneSet = 1,
}
impl From<IntEvent0MisRxne> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisRxne) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_RXNE` reader - Masked Negative Edge on UARTxRXD Interrupt."]
pub type IntEvent0MisRxneR = crate::BitReader<IntEvent0MisRxne>;
impl IntEvent0MisRxneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisRxne {
        match self.bits {
            false => IntEvent0MisRxne::IntEvent0MisRxneClr,
            true => IntEvent0MisRxne::IntEvent0MisRxneSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_rxne_clr(&self) -> bool {
        *self == IntEvent0MisRxne::IntEvent0MisRxneClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_rxne_set(&self) -> bool {
        *self == IntEvent0MisRxne::IntEvent0MisRxneSet
    }
}
#[doc = "Masked Positive Edge on UARTxRXD Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisRxpe {
    #[doc = "0: CLR"]
    IntEvent0MisRxpeClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisRxpeSet = 1,
}
impl From<IntEvent0MisRxpe> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisRxpe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_RXPE` reader - Masked Positive Edge on UARTxRXD Interrupt."]
pub type IntEvent0MisRxpeR = crate::BitReader<IntEvent0MisRxpe>;
impl IntEvent0MisRxpeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisRxpe {
        match self.bits {
            false => IntEvent0MisRxpe::IntEvent0MisRxpeClr,
            true => IntEvent0MisRxpe::IntEvent0MisRxpeSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_rxpe_clr(&self) -> bool {
        *self == IntEvent0MisRxpe::IntEvent0MisRxpeClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_rxpe_set(&self) -> bool {
        *self == IntEvent0MisRxpe::IntEvent0MisRxpeSet
    }
}
#[doc = "Masked LIN Capture 0 / Match Interrupt .\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisLinc0 {
    #[doc = "0: CLR"]
    IntEvent0MisLinc0Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisLinc0Set = 1,
}
impl From<IntEvent0MisLinc0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisLinc0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_LINC0` reader - Masked LIN Capture 0 / Match Interrupt ."]
pub type IntEvent0MisLinc0R = crate::BitReader<IntEvent0MisLinc0>;
impl IntEvent0MisLinc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisLinc0 {
        match self.bits {
            false => IntEvent0MisLinc0::IntEvent0MisLinc0Clr,
            true => IntEvent0MisLinc0::IntEvent0MisLinc0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_linc0_clr(&self) -> bool {
        *self == IntEvent0MisLinc0::IntEvent0MisLinc0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_linc0_set(&self) -> bool {
        *self == IntEvent0MisLinc0::IntEvent0MisLinc0Set
    }
}
#[doc = "Masked LIN Capture 1 Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisLinc1 {
    #[doc = "0: CLR"]
    IntEvent0MisLinc1Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisLinc1Set = 1,
}
impl From<IntEvent0MisLinc1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisLinc1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_LINC1` reader - Masked LIN Capture 1 Interrupt."]
pub type IntEvent0MisLinc1R = crate::BitReader<IntEvent0MisLinc1>;
impl IntEvent0MisLinc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisLinc1 {
        match self.bits {
            false => IntEvent0MisLinc1::IntEvent0MisLinc1Clr,
            true => IntEvent0MisLinc1::IntEvent0MisLinc1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_linc1_clr(&self) -> bool {
        *self == IntEvent0MisLinc1::IntEvent0MisLinc1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_linc1_set(&self) -> bool {
        *self == IntEvent0MisLinc1::IntEvent0MisLinc1Set
    }
}
#[doc = "Masked LIN Hardware Counter Overflow Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisLinovf {
    #[doc = "0: CLR"]
    IntEvent0MisLinovfClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisLinovfSet = 1,
}
impl From<IntEvent0MisLinovf> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisLinovf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_LINOVF` reader - Masked LIN Hardware Counter Overflow Interrupt."]
pub type IntEvent0MisLinovfR = crate::BitReader<IntEvent0MisLinovf>;
impl IntEvent0MisLinovfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisLinovf {
        match self.bits {
            false => IntEvent0MisLinovf::IntEvent0MisLinovfClr,
            true => IntEvent0MisLinovf::IntEvent0MisLinovfSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_linovf_clr(&self) -> bool {
        *self == IntEvent0MisLinovf::IntEvent0MisLinovfClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_linovf_set(&self) -> bool {
        *self == IntEvent0MisLinovf::IntEvent0MisLinovfSet
    }
}
#[doc = "Masked UART Receive Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisRxint {
    #[doc = "0: CLR"]
    IntEvent0MisRxintClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisRxintSet = 1,
}
impl From<IntEvent0MisRxint> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisRxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_RXINT` reader - Masked UART Receive Interrupt."]
pub type IntEvent0MisRxintR = crate::BitReader<IntEvent0MisRxint>;
impl IntEvent0MisRxintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisRxint {
        match self.bits {
            false => IntEvent0MisRxint::IntEvent0MisRxintClr,
            true => IntEvent0MisRxint::IntEvent0MisRxintSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_rxint_clr(&self) -> bool {
        *self == IntEvent0MisRxint::IntEvent0MisRxintClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_rxint_set(&self) -> bool {
        *self == IntEvent0MisRxint::IntEvent0MisRxintSet
    }
}
#[doc = "Masked UART Transmit Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisTxint {
    #[doc = "0: CLR"]
    IntEvent0MisTxintClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisTxintSet = 1,
}
impl From<IntEvent0MisTxint> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisTxint) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_TXINT` reader - Masked UART Transmit Interrupt."]
pub type IntEvent0MisTxintR = crate::BitReader<IntEvent0MisTxint>;
impl IntEvent0MisTxintR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisTxint {
        match self.bits {
            false => IntEvent0MisTxint::IntEvent0MisTxintClr,
            true => IntEvent0MisTxint::IntEvent0MisTxintSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_txint_clr(&self) -> bool {
        *self == IntEvent0MisTxint::IntEvent0MisTxintClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_txint_set(&self) -> bool {
        *self == IntEvent0MisTxint::IntEvent0MisTxintSet
    }
}
#[doc = "UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisEot {
    #[doc = "0: CLR"]
    IntEvent0MisEotClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisEotSet = 1,
}
impl From<IntEvent0MisEot> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisEot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_EOT` reader - UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
pub type IntEvent0MisEotR = crate::BitReader<IntEvent0MisEot>;
impl IntEvent0MisEotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisEot {
        match self.bits {
            false => IntEvent0MisEot::IntEvent0MisEotClr,
            true => IntEvent0MisEot::IntEvent0MisEotSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_eot_clr(&self) -> bool {
        *self == IntEvent0MisEot::IntEvent0MisEotClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_eot_set(&self) -> bool {
        *self == IntEvent0MisEot::IntEvent0MisEotSet
    }
}
#[doc = "Masked Address Match Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisAddrMatch {
    #[doc = "0: CLR"]
    IntEvent0MisAddrMatchClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisAddrMatchSet = 1,
}
impl From<IntEvent0MisAddrMatch> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisAddrMatch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_ADDR_MATCH` reader - Masked Address Match Interrupt."]
pub type IntEvent0MisAddrMatchR = crate::BitReader<IntEvent0MisAddrMatch>;
impl IntEvent0MisAddrMatchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisAddrMatch {
        match self.bits {
            false => IntEvent0MisAddrMatch::IntEvent0MisAddrMatchClr,
            true => IntEvent0MisAddrMatch::IntEvent0MisAddrMatchSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_addr_match_clr(&self) -> bool {
        *self == IntEvent0MisAddrMatch::IntEvent0MisAddrMatchClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_addr_match_set(&self) -> bool {
        *self == IntEvent0MisAddrMatch::IntEvent0MisAddrMatchSet
    }
}
#[doc = "Masked UART Clear to Send Modem Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisCts {
    #[doc = "0: CLR"]
    IntEvent0MisCtsClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisCtsSet = 1,
}
impl From<IntEvent0MisCts> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisCts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_CTS` reader - Masked UART Clear to Send Modem Interrupt."]
pub type IntEvent0MisCtsR = crate::BitReader<IntEvent0MisCts>;
impl IntEvent0MisCtsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisCts {
        match self.bits {
            false => IntEvent0MisCts::IntEvent0MisCtsClr,
            true => IntEvent0MisCts::IntEvent0MisCtsSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_cts_clr(&self) -> bool {
        *self == IntEvent0MisCts::IntEvent0MisCtsClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_cts_set(&self) -> bool {
        *self == IntEvent0MisCts::IntEvent0MisCtsSet
    }
}
#[doc = "Masked DMA Done on RX Event Channel Interrupt\n\nValue on reset: 0"]
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
#[doc = "Field `INT_EVENT0_MIS_DMA_DONE_RX` reader - Masked DMA Done on RX Event Channel Interrupt"]
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
#[doc = "Masked DMA Done on TX Event Channel Interrupt\n\nValue on reset: 0"]
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
#[doc = "Field `INT_EVENT0_MIS_DMA_DONE_TX` reader - Masked DMA Done on TX Event Channel Interrupt"]
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
#[doc = "Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisNerr {
    #[doc = "0: CLR"]
    IntEvent0MisNerrClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisNerrSet = 1,
}
impl From<IntEvent0MisNerr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisNerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_NERR` reader - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
pub type IntEvent0MisNerrR = crate::BitReader<IntEvent0MisNerr>;
impl IntEvent0MisNerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisNerr {
        match self.bits {
            false => IntEvent0MisNerr::IntEvent0MisNerrClr,
            true => IntEvent0MisNerr::IntEvent0MisNerrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_nerr_clr(&self) -> bool {
        *self == IntEvent0MisNerr::IntEvent0MisNerrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_nerr_set(&self) -> bool {
        *self == IntEvent0MisNerr::IntEvent0MisNerrSet
    }
}
impl R {
    #[doc = "Bit 0 - Masked UARTOUT Receive Time-Out Interrupt."]
    #[inline(always)]
    pub fn int_event0_mis_rtout(&self) -> IntEvent0MisRtoutR {
        IntEvent0MisRtoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masked UART Framing Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_mis_frmerr(&self) -> IntEvent0MisFrmerrR {
        IntEvent0MisFrmerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masked UART Parity Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_mis_parerr(&self) -> IntEvent0MisParerrR {
        IntEvent0MisParerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masked UART Break Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_mis_brkerr(&self) -> IntEvent0MisBrkerrR {
        IntEvent0MisBrkerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Masked UART Receive Overrun Error Interrupt."]
    #[inline(always)]
    pub fn int_event0_mis_ovrerr(&self) -> IntEvent0MisOvrerrR {
        IntEvent0MisOvrerrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Masked Negative Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn int_event0_mis_rxne(&self) -> IntEvent0MisRxneR {
        IntEvent0MisRxneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Masked Positive Edge on UARTxRXD Interrupt."]
    #[inline(always)]
    pub fn int_event0_mis_rxpe(&self) -> IntEvent0MisRxpeR {
        IntEvent0MisRxpeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Masked LIN Capture 0 / Match Interrupt ."]
    #[inline(always)]
    pub fn int_event0_mis_linc0(&self) -> IntEvent0MisLinc0R {
        IntEvent0MisLinc0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Masked LIN Capture 1 Interrupt."]
    #[inline(always)]
    pub fn int_event0_mis_linc1(&self) -> IntEvent0MisLinc1R {
        IntEvent0MisLinc1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Masked LIN Hardware Counter Overflow Interrupt."]
    #[inline(always)]
    pub fn int_event0_mis_linovf(&self) -> IntEvent0MisLinovfR {
        IntEvent0MisLinovfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Masked UART Receive Interrupt."]
    #[inline(always)]
    pub fn int_event0_mis_rxint(&self) -> IntEvent0MisRxintR {
        IntEvent0MisRxintR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Masked UART Transmit Interrupt."]
    #[inline(always)]
    pub fn int_event0_mis_txint(&self) -> IntEvent0MisTxintR {
        IntEvent0MisTxintR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - UART End of Transmission Interrupt Indicates that the last bit of all transmitted data and flags has left the serializer and without any further Data in the TX Fifo or Buffer."]
    #[inline(always)]
    pub fn int_event0_mis_eot(&self) -> IntEvent0MisEotR {
        IntEvent0MisEotR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Masked Address Match Interrupt."]
    #[inline(always)]
    pub fn int_event0_mis_addr_match(&self) -> IntEvent0MisAddrMatchR {
        IntEvent0MisAddrMatchR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Masked UART Clear to Send Modem Interrupt."]
    #[inline(always)]
    pub fn int_event0_mis_cts(&self) -> IntEvent0MisCtsR {
        IntEvent0MisCtsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Masked DMA Done on RX Event Channel Interrupt"]
    #[inline(always)]
    pub fn int_event0_mis_dma_done_rx(&self) -> IntEvent0MisDmaDoneRxR {
        IntEvent0MisDmaDoneRxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Masked DMA Done on TX Event Channel Interrupt"]
    #[inline(always)]
    pub fn int_event0_mis_dma_done_tx(&self) -> IntEvent0MisDmaDoneTxR {
        IntEvent0MisDmaDoneTxR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Noise Error on triple voting. Asserted when the 3 samples of majority voting are not equal"]
    #[inline(always)]
    pub fn int_event0_mis_nerr(&self) -> IntEvent0MisNerrR {
        IntEvent0MisNerrR::new(((self.bits >> 17) & 1) != 0)
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
