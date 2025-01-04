#[doc = "Register `INT_EVENT0_RIS` reader"]
pub type R = crate::R<IntEvent0RisSpec>;
#[doc = "Master Receive Transaction completed Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMrxdone {
    #[doc = "0: CLR"]
    IntEvent0RisMrxdoneClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMrxdoneSet = 1,
}
impl From<IntEvent0RisMrxdone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMrxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MRXDONE` reader - Master Receive Transaction completed Interrupt"]
pub type IntEvent0RisMrxdoneR = crate::BitReader<IntEvent0RisMrxdone>;
impl IntEvent0RisMrxdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMrxdone {
        match self.bits {
            false => IntEvent0RisMrxdone::IntEvent0RisMrxdoneClr,
            true => IntEvent0RisMrxdone::IntEvent0RisMrxdoneSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_mrxdone_clr(&self) -> bool {
        *self == IntEvent0RisMrxdone::IntEvent0RisMrxdoneClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_mrxdone_set(&self) -> bool {
        *self == IntEvent0RisMrxdone::IntEvent0RisMrxdoneSet
    }
}
#[doc = "Master Transmit Transaction completed Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMtxdone {
    #[doc = "0: CLR"]
    IntEvent0RisMtxdoneClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMtxdoneSet = 1,
}
impl From<IntEvent0RisMtxdone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMtxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MTXDONE` reader - Master Transmit Transaction completed Interrupt"]
pub type IntEvent0RisMtxdoneR = crate::BitReader<IntEvent0RisMtxdone>;
impl IntEvent0RisMtxdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMtxdone {
        match self.bits {
            false => IntEvent0RisMtxdone::IntEvent0RisMtxdoneClr,
            true => IntEvent0RisMtxdone::IntEvent0RisMtxdoneSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_mtxdone_clr(&self) -> bool {
        *self == IntEvent0RisMtxdone::IntEvent0RisMtxdoneClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_mtxdone_set(&self) -> bool {
        *self == IntEvent0RisMtxdone::IntEvent0RisMtxdoneSet
    }
}
#[doc = "Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMrxfifotrg {
    #[doc = "0: CLR"]
    IntEvent0RisMrxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMrxfifotrgSet = 1,
}
impl From<IntEvent0RisMrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MRXFIFOTRG` reader - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
pub type IntEvent0RisMrxfifotrgR = crate::BitReader<IntEvent0RisMrxfifotrg>;
impl IntEvent0RisMrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMrxfifotrg {
        match self.bits {
            false => IntEvent0RisMrxfifotrg::IntEvent0RisMrxfifotrgClr,
            true => IntEvent0RisMrxfifotrg::IntEvent0RisMrxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_mrxfifotrg_clr(&self) -> bool {
        *self == IntEvent0RisMrxfifotrg::IntEvent0RisMrxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_mrxfifotrg_set(&self) -> bool {
        *self == IntEvent0RisMrxfifotrg::IntEvent0RisMrxfifotrgSet
    }
}
#[doc = "Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMtxfifotrg {
    #[doc = "0: CLR"]
    IntEvent0RisMtxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMtxfifotrgSet = 1,
}
impl From<IntEvent0RisMtxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMtxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MTXFIFOTRG` reader - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
pub type IntEvent0RisMtxfifotrgR = crate::BitReader<IntEvent0RisMtxfifotrg>;
impl IntEvent0RisMtxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMtxfifotrg {
        match self.bits {
            false => IntEvent0RisMtxfifotrg::IntEvent0RisMtxfifotrgClr,
            true => IntEvent0RisMtxfifotrg::IntEvent0RisMtxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_mtxfifotrg_clr(&self) -> bool {
        *self == IntEvent0RisMtxfifotrg::IntEvent0RisMtxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_mtxfifotrg_set(&self) -> bool {
        *self == IntEvent0RisMtxfifotrg::IntEvent0RisMtxfifotrgSet
    }
}
#[doc = "RXFIFO full event. This interrupt is set if an RX FIFO is full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMrxfifofull {
    #[doc = "0: CLR"]
    IntEvent0RisMrxfifofullClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMrxfifofullSet = 1,
}
impl From<IntEvent0RisMrxfifofull> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMrxfifofull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MRXFIFOFULL` reader - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
pub type IntEvent0RisMrxfifofullR = crate::BitReader<IntEvent0RisMrxfifofull>;
impl IntEvent0RisMrxfifofullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMrxfifofull {
        match self.bits {
            false => IntEvent0RisMrxfifofull::IntEvent0RisMrxfifofullClr,
            true => IntEvent0RisMrxfifofull::IntEvent0RisMrxfifofullSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_mrxfifofull_clr(&self) -> bool {
        *self == IntEvent0RisMrxfifofull::IntEvent0RisMrxfifofullClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_mrxfifofull_set(&self) -> bool {
        *self == IntEvent0RisMrxfifofull::IntEvent0RisMrxfifofullSet
    }
}
#[doc = "Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMtxempty {
    #[doc = "0: CLR"]
    IntEvent0RisMtxemptyClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMtxemptySet = 1,
}
impl From<IntEvent0RisMtxempty> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMtxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MTXEMPTY` reader - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
pub type IntEvent0RisMtxemptyR = crate::BitReader<IntEvent0RisMtxempty>;
impl IntEvent0RisMtxemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMtxempty {
        match self.bits {
            false => IntEvent0RisMtxempty::IntEvent0RisMtxemptyClr,
            true => IntEvent0RisMtxempty::IntEvent0RisMtxemptySet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_mtxempty_clr(&self) -> bool {
        *self == IntEvent0RisMtxempty::IntEvent0RisMtxemptyClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_mtxempty_set(&self) -> bool {
        *self == IntEvent0RisMtxempty::IntEvent0RisMtxemptySet
    }
}
#[doc = "Address/Data NACK Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMnack {
    #[doc = "0: CLR"]
    IntEvent0RisMnackClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMnackSet = 1,
}
impl From<IntEvent0RisMnack> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMnack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MNACK` reader - Address/Data NACK Interrupt"]
pub type IntEvent0RisMnackR = crate::BitReader<IntEvent0RisMnack>;
impl IntEvent0RisMnackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMnack {
        match self.bits {
            false => IntEvent0RisMnack::IntEvent0RisMnackClr,
            true => IntEvent0RisMnack::IntEvent0RisMnackSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_mnack_clr(&self) -> bool {
        *self == IntEvent0RisMnack::IntEvent0RisMnackClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_mnack_set(&self) -> bool {
        *self == IntEvent0RisMnack::IntEvent0RisMnackSet
    }
}
#[doc = "START Detection Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMstart {
    #[doc = "0: CLR"]
    IntEvent0RisMstartClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMstartSet = 1,
}
impl From<IntEvent0RisMstart> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MSTART` reader - START Detection Interrupt"]
pub type IntEvent0RisMstartR = crate::BitReader<IntEvent0RisMstart>;
impl IntEvent0RisMstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMstart {
        match self.bits {
            false => IntEvent0RisMstart::IntEvent0RisMstartClr,
            true => IntEvent0RisMstart::IntEvent0RisMstartSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_mstart_clr(&self) -> bool {
        *self == IntEvent0RisMstart::IntEvent0RisMstartClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_mstart_set(&self) -> bool {
        *self == IntEvent0RisMstart::IntEvent0RisMstartSet
    }
}
#[doc = "STOP Detection Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMstop {
    #[doc = "0: CLR"]
    IntEvent0RisMstopClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMstopSet = 1,
}
impl From<IntEvent0RisMstop> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MSTOP` reader - STOP Detection Interrupt"]
pub type IntEvent0RisMstopR = crate::BitReader<IntEvent0RisMstop>;
impl IntEvent0RisMstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMstop {
        match self.bits {
            false => IntEvent0RisMstop::IntEvent0RisMstopClr,
            true => IntEvent0RisMstop::IntEvent0RisMstopSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_mstop_clr(&self) -> bool {
        *self == IntEvent0RisMstop::IntEvent0RisMstopClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_mstop_set(&self) -> bool {
        *self == IntEvent0RisMstop::IntEvent0RisMstopSet
    }
}
#[doc = "Arbitration Lost Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMarblost {
    #[doc = "0: CLR"]
    IntEvent0RisMarblostClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMarblostSet = 1,
}
impl From<IntEvent0RisMarblost> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMarblost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MARBLOST` reader - Arbitration Lost Interrupt"]
pub type IntEvent0RisMarblostR = crate::BitReader<IntEvent0RisMarblost>;
impl IntEvent0RisMarblostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMarblost {
        match self.bits {
            false => IntEvent0RisMarblost::IntEvent0RisMarblostClr,
            true => IntEvent0RisMarblost::IntEvent0RisMarblostSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_marblost_clr(&self) -> bool {
        *self == IntEvent0RisMarblost::IntEvent0RisMarblostClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_marblost_set(&self) -> bool {
        *self == IntEvent0RisMarblost::IntEvent0RisMarblostSet
    }
}
#[doc = "DMA Done 1 on Event Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMdmaDone1_2 {
    #[doc = "0: CLR"]
    IntEvent0RisMdmaDone1_2Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMdmaDone1_2Set = 1,
}
impl From<IntEvent0RisMdmaDone1_2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMdmaDone1_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MDMA_DONE1_2` reader - DMA Done 1 on Event Channel 2"]
pub type IntEvent0RisMdmaDone1_2R = crate::BitReader<IntEvent0RisMdmaDone1_2>;
impl IntEvent0RisMdmaDone1_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMdmaDone1_2 {
        match self.bits {
            false => IntEvent0RisMdmaDone1_2::IntEvent0RisMdmaDone1_2Clr,
            true => IntEvent0RisMdmaDone1_2::IntEvent0RisMdmaDone1_2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_mdma_done1_2_clr(&self) -> bool {
        *self == IntEvent0RisMdmaDone1_2::IntEvent0RisMdmaDone1_2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_mdma_done1_2_set(&self) -> bool {
        *self == IntEvent0RisMdmaDone1_2::IntEvent0RisMdmaDone1_2Set
    }
}
#[doc = "DMA Done 1 on Event Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMdmaDone1_3 {
    #[doc = "0: CLR"]
    IntEvent0RisMdmaDone1_3Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMdmaDone1_3Set = 1,
}
impl From<IntEvent0RisMdmaDone1_3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMdmaDone1_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MDMA_DONE1_3` reader - DMA Done 1 on Event Channel 3"]
pub type IntEvent0RisMdmaDone1_3R = crate::BitReader<IntEvent0RisMdmaDone1_3>;
impl IntEvent0RisMdmaDone1_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMdmaDone1_3 {
        match self.bits {
            false => IntEvent0RisMdmaDone1_3::IntEvent0RisMdmaDone1_3Clr,
            true => IntEvent0RisMdmaDone1_3::IntEvent0RisMdmaDone1_3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_mdma_done1_3_clr(&self) -> bool {
        *self == IntEvent0RisMdmaDone1_3::IntEvent0RisMdmaDone1_3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_mdma_done1_3_set(&self) -> bool {
        *self == IntEvent0RisMdmaDone1_3::IntEvent0RisMdmaDone1_3Set
    }
}
#[doc = "Master RX Pec Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisMpecRxErr {
    #[doc = "0: CLR"]
    IntEvent0RisMpecRxErrClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisMpecRxErrSet = 1,
}
impl From<IntEvent0RisMpecRxErr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisMpecRxErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_MPEC_RX_ERR` reader - Master RX Pec Error Interrupt"]
pub type IntEvent0RisMpecRxErrR = crate::BitReader<IntEvent0RisMpecRxErr>;
impl IntEvent0RisMpecRxErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisMpecRxErr {
        match self.bits {
            false => IntEvent0RisMpecRxErr::IntEvent0RisMpecRxErrClr,
            true => IntEvent0RisMpecRxErr::IntEvent0RisMpecRxErrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_mpec_rx_err_clr(&self) -> bool {
        *self == IntEvent0RisMpecRxErr::IntEvent0RisMpecRxErrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_mpec_rx_err_set(&self) -> bool {
        *self == IntEvent0RisMpecRxErr::IntEvent0RisMpecRxErrSet
    }
}
#[doc = "Timeout A Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisTimeouta {
    #[doc = "0: CLR"]
    IntEvent0RisTimeoutaClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisTimeoutaSet = 1,
}
impl From<IntEvent0RisTimeouta> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisTimeouta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_TIMEOUTA` reader - Timeout A Interrupt"]
pub type IntEvent0RisTimeoutaR = crate::BitReader<IntEvent0RisTimeouta>;
impl IntEvent0RisTimeoutaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisTimeouta {
        match self.bits {
            false => IntEvent0RisTimeouta::IntEvent0RisTimeoutaClr,
            true => IntEvent0RisTimeouta::IntEvent0RisTimeoutaSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_timeouta_clr(&self) -> bool {
        *self == IntEvent0RisTimeouta::IntEvent0RisTimeoutaClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_timeouta_set(&self) -> bool {
        *self == IntEvent0RisTimeouta::IntEvent0RisTimeoutaSet
    }
}
#[doc = "Timeout B Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisTimeoutb {
    #[doc = "0: CLR"]
    IntEvent0RisTimeoutbClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisTimeoutbSet = 1,
}
impl From<IntEvent0RisTimeoutb> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisTimeoutb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_TIMEOUTB` reader - Timeout B Interrupt"]
pub type IntEvent0RisTimeoutbR = crate::BitReader<IntEvent0RisTimeoutb>;
impl IntEvent0RisTimeoutbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisTimeoutb {
        match self.bits {
            false => IntEvent0RisTimeoutb::IntEvent0RisTimeoutbClr,
            true => IntEvent0RisTimeoutb::IntEvent0RisTimeoutbSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_timeoutb_clr(&self) -> bool {
        *self == IntEvent0RisTimeoutb::IntEvent0RisTimeoutbClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_timeoutb_set(&self) -> bool {
        *self == IntEvent0RisTimeoutb::IntEvent0RisTimeoutbSet
    }
}
#[doc = "Slave Receive Data Interrupt Signals that a byte has been received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisSrxdone {
    #[doc = "0: CLR"]
    IntEvent0RisSrxdoneClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisSrxdoneSet = 1,
}
impl From<IntEvent0RisSrxdone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisSrxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_SRXDONE` reader - Slave Receive Data Interrupt Signals that a byte has been received"]
pub type IntEvent0RisSrxdoneR = crate::BitReader<IntEvent0RisSrxdone>;
impl IntEvent0RisSrxdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisSrxdone {
        match self.bits {
            false => IntEvent0RisSrxdone::IntEvent0RisSrxdoneClr,
            true => IntEvent0RisSrxdone::IntEvent0RisSrxdoneSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_srxdone_clr(&self) -> bool {
        *self == IntEvent0RisSrxdone::IntEvent0RisSrxdoneClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_srxdone_set(&self) -> bool {
        *self == IntEvent0RisSrxdone::IntEvent0RisSrxdoneSet
    }
}
#[doc = "Slave Transmit Transaction completed Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisStxdone {
    #[doc = "0: CLR"]
    IntEvent0RisStxdoneClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisStxdoneSet = 1,
}
impl From<IntEvent0RisStxdone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisStxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_STXDONE` reader - Slave Transmit Transaction completed Interrupt"]
pub type IntEvent0RisStxdoneR = crate::BitReader<IntEvent0RisStxdone>;
impl IntEvent0RisStxdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisStxdone {
        match self.bits {
            false => IntEvent0RisStxdone::IntEvent0RisStxdoneClr,
            true => IntEvent0RisStxdone::IntEvent0RisStxdoneSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_stxdone_clr(&self) -> bool {
        *self == IntEvent0RisStxdone::IntEvent0RisStxdoneClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_stxdone_set(&self) -> bool {
        *self == IntEvent0RisStxdone::IntEvent0RisStxdoneSet
    }
}
#[doc = "Slave Receive FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisSrxfifotrg {
    #[doc = "0: CLR"]
    IntEvent0RisSrxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisSrxfifotrgSet = 1,
}
impl From<IntEvent0RisSrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisSrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_SRXFIFOTRG` reader - Slave Receive FIFO Trigger"]
pub type IntEvent0RisSrxfifotrgR = crate::BitReader<IntEvent0RisSrxfifotrg>;
impl IntEvent0RisSrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisSrxfifotrg {
        match self.bits {
            false => IntEvent0RisSrxfifotrg::IntEvent0RisSrxfifotrgClr,
            true => IntEvent0RisSrxfifotrg::IntEvent0RisSrxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_srxfifotrg_clr(&self) -> bool {
        *self == IntEvent0RisSrxfifotrg::IntEvent0RisSrxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_srxfifotrg_set(&self) -> bool {
        *self == IntEvent0RisSrxfifotrg::IntEvent0RisSrxfifotrgSet
    }
}
#[doc = "Slave Transmit FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisStxfifotrg {
    #[doc = "0: CLR"]
    IntEvent0RisStxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisStxfifotrgSet = 1,
}
impl From<IntEvent0RisStxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisStxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_STXFIFOTRG` reader - Slave Transmit FIFO Trigger"]
pub type IntEvent0RisStxfifotrgR = crate::BitReader<IntEvent0RisStxfifotrg>;
impl IntEvent0RisStxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisStxfifotrg {
        match self.bits {
            false => IntEvent0RisStxfifotrg::IntEvent0RisStxfifotrgClr,
            true => IntEvent0RisStxfifotrg::IntEvent0RisStxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_stxfifotrg_clr(&self) -> bool {
        *self == IntEvent0RisStxfifotrg::IntEvent0RisStxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_stxfifotrg_set(&self) -> bool {
        *self == IntEvent0RisStxfifotrg::IntEvent0RisStxfifotrgSet
    }
}
#[doc = "RXFIFO full event. This interrupt is set if an RX FIFO is full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisSrxfifofull {
    #[doc = "0: CLR"]
    IntEvent0RisSrxfifofullClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisSrxfifofullSet = 1,
}
impl From<IntEvent0RisSrxfifofull> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisSrxfifofull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_SRXFIFOFULL` reader - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
pub type IntEvent0RisSrxfifofullR = crate::BitReader<IntEvent0RisSrxfifofull>;
impl IntEvent0RisSrxfifofullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisSrxfifofull {
        match self.bits {
            false => IntEvent0RisSrxfifofull::IntEvent0RisSrxfifofullClr,
            true => IntEvent0RisSrxfifofull::IntEvent0RisSrxfifofullSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_srxfifofull_clr(&self) -> bool {
        *self == IntEvent0RisSrxfifofull::IntEvent0RisSrxfifofullClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_srxfifofull_set(&self) -> bool {
        *self == IntEvent0RisSrxfifofull::IntEvent0RisSrxfifofullSet
    }
}
#[doc = "Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisStxempty {
    #[doc = "0: CLR"]
    IntEvent0RisStxemptyClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisStxemptySet = 1,
}
impl From<IntEvent0RisStxempty> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisStxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_STXEMPTY` reader - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
pub type IntEvent0RisStxemptyR = crate::BitReader<IntEvent0RisStxempty>;
impl IntEvent0RisStxemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisStxempty {
        match self.bits {
            false => IntEvent0RisStxempty::IntEvent0RisStxemptyClr,
            true => IntEvent0RisStxempty::IntEvent0RisStxemptySet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_stxempty_clr(&self) -> bool {
        *self == IntEvent0RisStxempty::IntEvent0RisStxemptyClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_stxempty_set(&self) -> bool {
        *self == IntEvent0RisStxempty::IntEvent0RisStxemptySet
    }
}
#[doc = "Start Condition Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisSstart {
    #[doc = "0: CLR"]
    IntEvent0RisSstartClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisSstartSet = 1,
}
impl From<IntEvent0RisSstart> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisSstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_SSTART` reader - Start Condition Interrupt"]
pub type IntEvent0RisSstartR = crate::BitReader<IntEvent0RisSstart>;
impl IntEvent0RisSstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisSstart {
        match self.bits {
            false => IntEvent0RisSstart::IntEvent0RisSstartClr,
            true => IntEvent0RisSstart::IntEvent0RisSstartSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_sstart_clr(&self) -> bool {
        *self == IntEvent0RisSstart::IntEvent0RisSstartClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_sstart_set(&self) -> bool {
        *self == IntEvent0RisSstart::IntEvent0RisSstartSet
    }
}
#[doc = "Stop Condition Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisSstop {
    #[doc = "0: CLR"]
    IntEvent0RisSstopClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisSstopSet = 1,
}
impl From<IntEvent0RisSstop> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisSstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_SSTOP` reader - Stop Condition Interrupt"]
pub type IntEvent0RisSstopR = crate::BitReader<IntEvent0RisSstop>;
impl IntEvent0RisSstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisSstop {
        match self.bits {
            false => IntEvent0RisSstop::IntEvent0RisSstopClr,
            true => IntEvent0RisSstop::IntEvent0RisSstopSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_sstop_clr(&self) -> bool {
        *self == IntEvent0RisSstop::IntEvent0RisSstopClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_sstop_set(&self) -> bool {
        *self == IntEvent0RisSstop::IntEvent0RisSstopSet
    }
}
#[doc = "General Call Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisSgencall {
    #[doc = "0: CLR"]
    IntEvent0RisSgencallClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisSgencallSet = 1,
}
impl From<IntEvent0RisSgencall> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisSgencall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_SGENCALL` reader - General Call Interrupt"]
pub type IntEvent0RisSgencallR = crate::BitReader<IntEvent0RisSgencall>;
impl IntEvent0RisSgencallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisSgencall {
        match self.bits {
            false => IntEvent0RisSgencall::IntEvent0RisSgencallClr,
            true => IntEvent0RisSgencall::IntEvent0RisSgencallSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_sgencall_clr(&self) -> bool {
        *self == IntEvent0RisSgencall::IntEvent0RisSgencallClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_sgencall_set(&self) -> bool {
        *self == IntEvent0RisSgencall::IntEvent0RisSgencallSet
    }
}
#[doc = "DMA Done 1 on Event Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisSdmaDone1_2 {
    #[doc = "0: CLR"]
    IntEvent0RisSdmaDone1_2Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisSdmaDone1_2Set = 1,
}
impl From<IntEvent0RisSdmaDone1_2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisSdmaDone1_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_SDMA_DONE1_2` reader - DMA Done 1 on Event Channel 2"]
pub type IntEvent0RisSdmaDone1_2R = crate::BitReader<IntEvent0RisSdmaDone1_2>;
impl IntEvent0RisSdmaDone1_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisSdmaDone1_2 {
        match self.bits {
            false => IntEvent0RisSdmaDone1_2::IntEvent0RisSdmaDone1_2Clr,
            true => IntEvent0RisSdmaDone1_2::IntEvent0RisSdmaDone1_2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_sdma_done1_2_clr(&self) -> bool {
        *self == IntEvent0RisSdmaDone1_2::IntEvent0RisSdmaDone1_2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_sdma_done1_2_set(&self) -> bool {
        *self == IntEvent0RisSdmaDone1_2::IntEvent0RisSdmaDone1_2Set
    }
}
#[doc = "DMA Done 1 on Event Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisSdmaDone1_3 {
    #[doc = "0: CLR"]
    IntEvent0RisSdmaDone1_3Clr = 0,
    #[doc = "1: SET"]
    IntEvent0RisSdmaDone1_3Set = 1,
}
impl From<IntEvent0RisSdmaDone1_3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisSdmaDone1_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_SDMA_DONE1_3` reader - DMA Done 1 on Event Channel 3"]
pub type IntEvent0RisSdmaDone1_3R = crate::BitReader<IntEvent0RisSdmaDone1_3>;
impl IntEvent0RisSdmaDone1_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisSdmaDone1_3 {
        match self.bits {
            false => IntEvent0RisSdmaDone1_3::IntEvent0RisSdmaDone1_3Clr,
            true => IntEvent0RisSdmaDone1_3::IntEvent0RisSdmaDone1_3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_sdma_done1_3_clr(&self) -> bool {
        *self == IntEvent0RisSdmaDone1_3::IntEvent0RisSdmaDone1_3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_sdma_done1_3_set(&self) -> bool {
        *self == IntEvent0RisSdmaDone1_3::IntEvent0RisSdmaDone1_3Set
    }
}
#[doc = "Slave RX Pec Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisSpecRxErr {
    #[doc = "0: CLR"]
    IntEvent0RisSpecRxErrClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisSpecRxErrSet = 1,
}
impl From<IntEvent0RisSpecRxErr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisSpecRxErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_SPEC_RX_ERR` reader - Slave RX Pec Error Interrupt"]
pub type IntEvent0RisSpecRxErrR = crate::BitReader<IntEvent0RisSpecRxErr>;
impl IntEvent0RisSpecRxErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisSpecRxErr {
        match self.bits {
            false => IntEvent0RisSpecRxErr::IntEvent0RisSpecRxErrClr,
            true => IntEvent0RisSpecRxErr::IntEvent0RisSpecRxErrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_spec_rx_err_clr(&self) -> bool {
        *self == IntEvent0RisSpecRxErr::IntEvent0RisSpecRxErrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_spec_rx_err_set(&self) -> bool {
        *self == IntEvent0RisSpecRxErr::IntEvent0RisSpecRxErrSet
    }
}
#[doc = "Slave TX FIFO underflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisStxUnfl {
    #[doc = "0: CLR"]
    IntEvent0RisStxUnflClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisStxUnflSet = 1,
}
impl From<IntEvent0RisStxUnfl> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisStxUnfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_STX_UNFL` reader - Slave TX FIFO underflow"]
pub type IntEvent0RisStxUnflR = crate::BitReader<IntEvent0RisStxUnfl>;
impl IntEvent0RisStxUnflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisStxUnfl {
        match self.bits {
            false => IntEvent0RisStxUnfl::IntEvent0RisStxUnflClr,
            true => IntEvent0RisStxUnfl::IntEvent0RisStxUnflSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_stx_unfl_clr(&self) -> bool {
        *self == IntEvent0RisStxUnfl::IntEvent0RisStxUnflClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_stx_unfl_set(&self) -> bool {
        *self == IntEvent0RisStxUnfl::IntEvent0RisStxUnflSet
    }
}
#[doc = "Slave RX FIFO overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisSrxOvfl {
    #[doc = "0: CLR"]
    IntEvent0RisSrxOvflClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisSrxOvflSet = 1,
}
impl From<IntEvent0RisSrxOvfl> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisSrxOvfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_SRX_OVFL` reader - Slave RX FIFO overflow"]
pub type IntEvent0RisSrxOvflR = crate::BitReader<IntEvent0RisSrxOvfl>;
impl IntEvent0RisSrxOvflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisSrxOvfl {
        match self.bits {
            false => IntEvent0RisSrxOvfl::IntEvent0RisSrxOvflClr,
            true => IntEvent0RisSrxOvfl::IntEvent0RisSrxOvflSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_srx_ovfl_clr(&self) -> bool {
        *self == IntEvent0RisSrxOvfl::IntEvent0RisSrxOvflClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_srx_ovfl_set(&self) -> bool {
        *self == IntEvent0RisSrxOvfl::IntEvent0RisSrxOvflSet
    }
}
#[doc = "Slave Arbitration Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisSarblost {
    #[doc = "0: CLR"]
    IntEvent0RisSarblostClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisSarblostSet = 1,
}
impl From<IntEvent0RisSarblost> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisSarblost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_SARBLOST` reader - Slave Arbitration Lost"]
pub type IntEvent0RisSarblostR = crate::BitReader<IntEvent0RisSarblost>;
impl IntEvent0RisSarblostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisSarblost {
        match self.bits {
            false => IntEvent0RisSarblost::IntEvent0RisSarblostClr,
            true => IntEvent0RisSarblost::IntEvent0RisSarblostSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_sarblost_clr(&self) -> bool {
        *self == IntEvent0RisSarblost::IntEvent0RisSarblostClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_sarblost_set(&self) -> bool {
        *self == IntEvent0RisSarblost::IntEvent0RisSarblostSet
    }
}
#[doc = "Interrupt overflow interrupt It is set when SSTART or SSTOP interrupts overflow i.e. occur twice without being serviced\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0RisIntrOvfl {
    #[doc = "0: CLR"]
    IntEvent0RisIntrOvflClr = 0,
    #[doc = "1: SET"]
    IntEvent0RisIntrOvflSet = 1,
}
impl From<IntEvent0RisIntrOvfl> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0RisIntrOvfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_RIS_INTR_OVFL` reader - Interrupt overflow interrupt It is set when SSTART or SSTOP interrupts overflow i.e. occur twice without being serviced"]
pub type IntEvent0RisIntrOvflR = crate::BitReader<IntEvent0RisIntrOvfl>;
impl IntEvent0RisIntrOvflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0RisIntrOvfl {
        match self.bits {
            false => IntEvent0RisIntrOvfl::IntEvent0RisIntrOvflClr,
            true => IntEvent0RisIntrOvfl::IntEvent0RisIntrOvflSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_ris_intr_ovfl_clr(&self) -> bool {
        *self == IntEvent0RisIntrOvfl::IntEvent0RisIntrOvflClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_ris_intr_ovfl_set(&self) -> bool {
        *self == IntEvent0RisIntrOvfl::IntEvent0RisIntrOvflSet
    }
}
impl R {
    #[doc = "Bit 0 - Master Receive Transaction completed Interrupt"]
    #[inline(always)]
    pub fn int_event0_ris_mrxdone(&self) -> IntEvent0RisMrxdoneR {
        IntEvent0RisMrxdoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn int_event0_ris_mtxdone(&self) -> IntEvent0RisMtxdoneR {
        IntEvent0RisMtxdoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
    #[inline(always)]
    pub fn int_event0_ris_mrxfifotrg(&self) -> IntEvent0RisMrxfifotrgR {
        IntEvent0RisMrxfifotrgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
    #[inline(always)]
    pub fn int_event0_ris_mtxfifotrg(&self) -> IntEvent0RisMtxfifotrgR {
        IntEvent0RisMtxfifotrgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
    #[inline(always)]
    pub fn int_event0_ris_mrxfifofull(&self) -> IntEvent0RisMrxfifofullR {
        IntEvent0RisMrxfifofullR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
    #[inline(always)]
    pub fn int_event0_ris_mtxempty(&self) -> IntEvent0RisMtxemptyR {
        IntEvent0RisMtxemptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Address/Data NACK Interrupt"]
    #[inline(always)]
    pub fn int_event0_ris_mnack(&self) -> IntEvent0RisMnackR {
        IntEvent0RisMnackR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - START Detection Interrupt"]
    #[inline(always)]
    pub fn int_event0_ris_mstart(&self) -> IntEvent0RisMstartR {
        IntEvent0RisMstartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STOP Detection Interrupt"]
    #[inline(always)]
    pub fn int_event0_ris_mstop(&self) -> IntEvent0RisMstopR {
        IntEvent0RisMstopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Arbitration Lost Interrupt"]
    #[inline(always)]
    pub fn int_event0_ris_marblost(&self) -> IntEvent0RisMarblostR {
        IntEvent0RisMarblostR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA Done 1 on Event Channel 2"]
    #[inline(always)]
    pub fn int_event0_ris_mdma_done1_2(&self) -> IntEvent0RisMdmaDone1_2R {
        IntEvent0RisMdmaDone1_2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA Done 1 on Event Channel 3"]
    #[inline(always)]
    pub fn int_event0_ris_mdma_done1_3(&self) -> IntEvent0RisMdmaDone1_3R {
        IntEvent0RisMdmaDone1_3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Master RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn int_event0_ris_mpec_rx_err(&self) -> IntEvent0RisMpecRxErrR {
        IntEvent0RisMpecRxErrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Timeout A Interrupt"]
    #[inline(always)]
    pub fn int_event0_ris_timeouta(&self) -> IntEvent0RisTimeoutaR {
        IntEvent0RisTimeoutaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timeout B Interrupt"]
    #[inline(always)]
    pub fn int_event0_ris_timeoutb(&self) -> IntEvent0RisTimeoutbR {
        IntEvent0RisTimeoutbR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave Receive Data Interrupt Signals that a byte has been received"]
    #[inline(always)]
    pub fn int_event0_ris_srxdone(&self) -> IntEvent0RisSrxdoneR {
        IntEvent0RisSrxdoneR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Slave Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn int_event0_ris_stxdone(&self) -> IntEvent0RisStxdoneR {
        IntEvent0RisStxdoneR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Slave Receive FIFO Trigger"]
    #[inline(always)]
    pub fn int_event0_ris_srxfifotrg(&self) -> IntEvent0RisSrxfifotrgR {
        IntEvent0RisSrxfifotrgR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Slave Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn int_event0_ris_stxfifotrg(&self) -> IntEvent0RisStxfifotrgR {
        IntEvent0RisStxfifotrgR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
    #[inline(always)]
    pub fn int_event0_ris_srxfifofull(&self) -> IntEvent0RisSrxfifofullR {
        IntEvent0RisSrxfifofullR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
    #[inline(always)]
    pub fn int_event0_ris_stxempty(&self) -> IntEvent0RisStxemptyR {
        IntEvent0RisStxemptyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Start Condition Interrupt"]
    #[inline(always)]
    pub fn int_event0_ris_sstart(&self) -> IntEvent0RisSstartR {
        IntEvent0RisSstartR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Stop Condition Interrupt"]
    #[inline(always)]
    pub fn int_event0_ris_sstop(&self) -> IntEvent0RisSstopR {
        IntEvent0RisSstopR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - General Call Interrupt"]
    #[inline(always)]
    pub fn int_event0_ris_sgencall(&self) -> IntEvent0RisSgencallR {
        IntEvent0RisSgencallR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA Done 1 on Event Channel 2"]
    #[inline(always)]
    pub fn int_event0_ris_sdma_done1_2(&self) -> IntEvent0RisSdmaDone1_2R {
        IntEvent0RisSdmaDone1_2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DMA Done 1 on Event Channel 3"]
    #[inline(always)]
    pub fn int_event0_ris_sdma_done1_3(&self) -> IntEvent0RisSdmaDone1_3R {
        IntEvent0RisSdmaDone1_3R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Slave RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn int_event0_ris_spec_rx_err(&self) -> IntEvent0RisSpecRxErrR {
        IntEvent0RisSpecRxErrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Slave TX FIFO underflow"]
    #[inline(always)]
    pub fn int_event0_ris_stx_unfl(&self) -> IntEvent0RisStxUnflR {
        IntEvent0RisStxUnflR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Slave RX FIFO overflow"]
    #[inline(always)]
    pub fn int_event0_ris_srx_ovfl(&self) -> IntEvent0RisSrxOvflR {
        IntEvent0RisSrxOvflR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Slave Arbitration Lost"]
    #[inline(always)]
    pub fn int_event0_ris_sarblost(&self) -> IntEvent0RisSarblostR {
        IntEvent0RisSarblostR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt overflow interrupt It is set when SSTART or SSTOP interrupts overflow i.e. occur twice without being serviced"]
    #[inline(always)]
    pub fn int_event0_ris_intr_ovfl(&self) -> IntEvent0RisIntrOvflR {
        IntEvent0RisIntrOvflR::new(((self.bits >> 31) & 1) != 0)
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
