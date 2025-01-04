#[doc = "Register `INT_EVENT0_MIS` reader"]
pub type R = crate::R<IntEvent0MisSpec>;
#[doc = "Master Receive Data Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMrxdone {
    #[doc = "0: CLR"]
    IntEvent0MisMrxdoneClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMrxdoneSet = 1,
}
impl From<IntEvent0MisMrxdone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMrxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MRXDONE` reader - Master Receive Data Interrupt"]
pub type IntEvent0MisMrxdoneR = crate::BitReader<IntEvent0MisMrxdone>;
impl IntEvent0MisMrxdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMrxdone {
        match self.bits {
            false => IntEvent0MisMrxdone::IntEvent0MisMrxdoneClr,
            true => IntEvent0MisMrxdone::IntEvent0MisMrxdoneSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_mrxdone_clr(&self) -> bool {
        *self == IntEvent0MisMrxdone::IntEvent0MisMrxdoneClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_mrxdone_set(&self) -> bool {
        *self == IntEvent0MisMrxdone::IntEvent0MisMrxdoneSet
    }
}
#[doc = "Master Transmit Transaction completed Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMtxdone {
    #[doc = "0: CLR"]
    IntEvent0MisMtxdoneClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMtxdoneSet = 1,
}
impl From<IntEvent0MisMtxdone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMtxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MTXDONE` reader - Master Transmit Transaction completed Interrupt"]
pub type IntEvent0MisMtxdoneR = crate::BitReader<IntEvent0MisMtxdone>;
impl IntEvent0MisMtxdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMtxdone {
        match self.bits {
            false => IntEvent0MisMtxdone::IntEvent0MisMtxdoneClr,
            true => IntEvent0MisMtxdone::IntEvent0MisMtxdoneSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_mtxdone_clr(&self) -> bool {
        *self == IntEvent0MisMtxdone::IntEvent0MisMtxdoneClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_mtxdone_set(&self) -> bool {
        *self == IntEvent0MisMtxdone::IntEvent0MisMtxdoneSet
    }
}
#[doc = "Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMrxfifotrg {
    #[doc = "0: CLR"]
    IntEvent0MisMrxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMrxfifotrgSet = 1,
}
impl From<IntEvent0MisMrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MRXFIFOTRG` reader - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
pub type IntEvent0MisMrxfifotrgR = crate::BitReader<IntEvent0MisMrxfifotrg>;
impl IntEvent0MisMrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMrxfifotrg {
        match self.bits {
            false => IntEvent0MisMrxfifotrg::IntEvent0MisMrxfifotrgClr,
            true => IntEvent0MisMrxfifotrg::IntEvent0MisMrxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_mrxfifotrg_clr(&self) -> bool {
        *self == IntEvent0MisMrxfifotrg::IntEvent0MisMrxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_mrxfifotrg_set(&self) -> bool {
        *self == IntEvent0MisMrxfifotrg::IntEvent0MisMrxfifotrgSet
    }
}
#[doc = "Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMtxfifotrg {
    #[doc = "0: CLR"]
    IntEvent0MisMtxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMtxfifotrgSet = 1,
}
impl From<IntEvent0MisMtxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMtxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MTXFIFOTRG` reader - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
pub type IntEvent0MisMtxfifotrgR = crate::BitReader<IntEvent0MisMtxfifotrg>;
impl IntEvent0MisMtxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMtxfifotrg {
        match self.bits {
            false => IntEvent0MisMtxfifotrg::IntEvent0MisMtxfifotrgClr,
            true => IntEvent0MisMtxfifotrg::IntEvent0MisMtxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_mtxfifotrg_clr(&self) -> bool {
        *self == IntEvent0MisMtxfifotrg::IntEvent0MisMtxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_mtxfifotrg_set(&self) -> bool {
        *self == IntEvent0MisMtxfifotrg::IntEvent0MisMtxfifotrgSet
    }
}
#[doc = "RXFIFO full event. This interrupt is set if the RX FIFO is full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMrxfifofull {
    #[doc = "0: CLR"]
    IntEvent0MisMrxfifofullClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMrxfifofullSet = 1,
}
impl From<IntEvent0MisMrxfifofull> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMrxfifofull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MRXFIFOFULL` reader - RXFIFO full event. This interrupt is set if the RX FIFO is full."]
pub type IntEvent0MisMrxfifofullR = crate::BitReader<IntEvent0MisMrxfifofull>;
impl IntEvent0MisMrxfifofullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMrxfifofull {
        match self.bits {
            false => IntEvent0MisMrxfifofull::IntEvent0MisMrxfifofullClr,
            true => IntEvent0MisMrxfifofull::IntEvent0MisMrxfifofullSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_mrxfifofull_clr(&self) -> bool {
        *self == IntEvent0MisMrxfifofull::IntEvent0MisMrxfifofullClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_mrxfifofull_set(&self) -> bool {
        *self == IntEvent0MisMrxfifofull::IntEvent0MisMrxfifofullSet
    }
}
#[doc = "Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMtxempty {
    #[doc = "0: CLR"]
    IntEvent0MisMtxemptyClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMtxemptySet = 1,
}
impl From<IntEvent0MisMtxempty> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMtxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MTXEMPTY` reader - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
pub type IntEvent0MisMtxemptyR = crate::BitReader<IntEvent0MisMtxempty>;
impl IntEvent0MisMtxemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMtxempty {
        match self.bits {
            false => IntEvent0MisMtxempty::IntEvent0MisMtxemptyClr,
            true => IntEvent0MisMtxempty::IntEvent0MisMtxemptySet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_mtxempty_clr(&self) -> bool {
        *self == IntEvent0MisMtxempty::IntEvent0MisMtxemptyClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_mtxempty_set(&self) -> bool {
        *self == IntEvent0MisMtxempty::IntEvent0MisMtxemptySet
    }
}
#[doc = "Address/Data NACK Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMnack {
    #[doc = "0: CLR"]
    IntEvent0MisMnackClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMnackSet = 1,
}
impl From<IntEvent0MisMnack> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMnack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MNACK` reader - Address/Data NACK Interrupt"]
pub type IntEvent0MisMnackR = crate::BitReader<IntEvent0MisMnack>;
impl IntEvent0MisMnackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMnack {
        match self.bits {
            false => IntEvent0MisMnack::IntEvent0MisMnackClr,
            true => IntEvent0MisMnack::IntEvent0MisMnackSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_mnack_clr(&self) -> bool {
        *self == IntEvent0MisMnack::IntEvent0MisMnackClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_mnack_set(&self) -> bool {
        *self == IntEvent0MisMnack::IntEvent0MisMnackSet
    }
}
#[doc = "START Detection Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMstart {
    #[doc = "0: CLR"]
    IntEvent0MisMstartClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMstartSet = 1,
}
impl From<IntEvent0MisMstart> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MSTART` reader - START Detection Interrupt"]
pub type IntEvent0MisMstartR = crate::BitReader<IntEvent0MisMstart>;
impl IntEvent0MisMstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMstart {
        match self.bits {
            false => IntEvent0MisMstart::IntEvent0MisMstartClr,
            true => IntEvent0MisMstart::IntEvent0MisMstartSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_mstart_clr(&self) -> bool {
        *self == IntEvent0MisMstart::IntEvent0MisMstartClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_mstart_set(&self) -> bool {
        *self == IntEvent0MisMstart::IntEvent0MisMstartSet
    }
}
#[doc = "STOP Detection Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMstop {
    #[doc = "0: CLR"]
    IntEvent0MisMstopClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMstopSet = 1,
}
impl From<IntEvent0MisMstop> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MSTOP` reader - STOP Detection Interrupt"]
pub type IntEvent0MisMstopR = crate::BitReader<IntEvent0MisMstop>;
impl IntEvent0MisMstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMstop {
        match self.bits {
            false => IntEvent0MisMstop::IntEvent0MisMstopClr,
            true => IntEvent0MisMstop::IntEvent0MisMstopSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_mstop_clr(&self) -> bool {
        *self == IntEvent0MisMstop::IntEvent0MisMstopClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_mstop_set(&self) -> bool {
        *self == IntEvent0MisMstop::IntEvent0MisMstopSet
    }
}
#[doc = "Arbitration Lost Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMarblost {
    #[doc = "0: CLR"]
    IntEvent0MisMarblostClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMarblostSet = 1,
}
impl From<IntEvent0MisMarblost> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMarblost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MARBLOST` reader - Arbitration Lost Interrupt"]
pub type IntEvent0MisMarblostR = crate::BitReader<IntEvent0MisMarblost>;
impl IntEvent0MisMarblostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMarblost {
        match self.bits {
            false => IntEvent0MisMarblost::IntEvent0MisMarblostClr,
            true => IntEvent0MisMarblost::IntEvent0MisMarblostSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_marblost_clr(&self) -> bool {
        *self == IntEvent0MisMarblost::IntEvent0MisMarblostClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_marblost_set(&self) -> bool {
        *self == IntEvent0MisMarblost::IntEvent0MisMarblostSet
    }
}
#[doc = "DMA Done 1 on Event Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMdmaDone1_2 {
    #[doc = "0: CLR"]
    IntEvent0MisMdmaDone1_2Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMdmaDone1_2Set = 1,
}
impl From<IntEvent0MisMdmaDone1_2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMdmaDone1_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MDMA_DONE1_2` reader - DMA Done 1 on Event Channel 2"]
pub type IntEvent0MisMdmaDone1_2R = crate::BitReader<IntEvent0MisMdmaDone1_2>;
impl IntEvent0MisMdmaDone1_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMdmaDone1_2 {
        match self.bits {
            false => IntEvent0MisMdmaDone1_2::IntEvent0MisMdmaDone1_2Clr,
            true => IntEvent0MisMdmaDone1_2::IntEvent0MisMdmaDone1_2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_mdma_done1_2_clr(&self) -> bool {
        *self == IntEvent0MisMdmaDone1_2::IntEvent0MisMdmaDone1_2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_mdma_done1_2_set(&self) -> bool {
        *self == IntEvent0MisMdmaDone1_2::IntEvent0MisMdmaDone1_2Set
    }
}
#[doc = "DMA Done 1 on Event Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMdmaDone1_3 {
    #[doc = "0: CLR"]
    IntEvent0MisMdmaDone1_3Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMdmaDone1_3Set = 1,
}
impl From<IntEvent0MisMdmaDone1_3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMdmaDone1_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MDMA_DONE1_3` reader - DMA Done 1 on Event Channel 3"]
pub type IntEvent0MisMdmaDone1_3R = crate::BitReader<IntEvent0MisMdmaDone1_3>;
impl IntEvent0MisMdmaDone1_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMdmaDone1_3 {
        match self.bits {
            false => IntEvent0MisMdmaDone1_3::IntEvent0MisMdmaDone1_3Clr,
            true => IntEvent0MisMdmaDone1_3::IntEvent0MisMdmaDone1_3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_mdma_done1_3_clr(&self) -> bool {
        *self == IntEvent0MisMdmaDone1_3::IntEvent0MisMdmaDone1_3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_mdma_done1_3_set(&self) -> bool {
        *self == IntEvent0MisMdmaDone1_3::IntEvent0MisMdmaDone1_3Set
    }
}
#[doc = "Master RX Pec Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisMpecRxErr {
    #[doc = "0: CLR"]
    IntEvent0MisMpecRxErrClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisMpecRxErrSet = 1,
}
impl From<IntEvent0MisMpecRxErr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisMpecRxErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_MPEC_RX_ERR` reader - Master RX Pec Error Interrupt"]
pub type IntEvent0MisMpecRxErrR = crate::BitReader<IntEvent0MisMpecRxErr>;
impl IntEvent0MisMpecRxErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisMpecRxErr {
        match self.bits {
            false => IntEvent0MisMpecRxErr::IntEvent0MisMpecRxErrClr,
            true => IntEvent0MisMpecRxErr::IntEvent0MisMpecRxErrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_mpec_rx_err_clr(&self) -> bool {
        *self == IntEvent0MisMpecRxErr::IntEvent0MisMpecRxErrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_mpec_rx_err_set(&self) -> bool {
        *self == IntEvent0MisMpecRxErr::IntEvent0MisMpecRxErrSet
    }
}
#[doc = "Timeout A Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisTimeouta {
    #[doc = "0: CLR"]
    IntEvent0MisTimeoutaClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisTimeoutaSet = 1,
}
impl From<IntEvent0MisTimeouta> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisTimeouta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_TIMEOUTA` reader - Timeout A Interrupt"]
pub type IntEvent0MisTimeoutaR = crate::BitReader<IntEvent0MisTimeouta>;
impl IntEvent0MisTimeoutaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisTimeouta {
        match self.bits {
            false => IntEvent0MisTimeouta::IntEvent0MisTimeoutaClr,
            true => IntEvent0MisTimeouta::IntEvent0MisTimeoutaSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_timeouta_clr(&self) -> bool {
        *self == IntEvent0MisTimeouta::IntEvent0MisTimeoutaClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_timeouta_set(&self) -> bool {
        *self == IntEvent0MisTimeouta::IntEvent0MisTimeoutaSet
    }
}
#[doc = "Timeout B Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisTimeoutb {
    #[doc = "0: CLR"]
    IntEvent0MisTimeoutbClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisTimeoutbSet = 1,
}
impl From<IntEvent0MisTimeoutb> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisTimeoutb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_TIMEOUTB` reader - Timeout B Interrupt"]
pub type IntEvent0MisTimeoutbR = crate::BitReader<IntEvent0MisTimeoutb>;
impl IntEvent0MisTimeoutbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisTimeoutb {
        match self.bits {
            false => IntEvent0MisTimeoutb::IntEvent0MisTimeoutbClr,
            true => IntEvent0MisTimeoutb::IntEvent0MisTimeoutbSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_timeoutb_clr(&self) -> bool {
        *self == IntEvent0MisTimeoutb::IntEvent0MisTimeoutbClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_timeoutb_set(&self) -> bool {
        *self == IntEvent0MisTimeoutb::IntEvent0MisTimeoutbSet
    }
}
#[doc = "Slave Receive Data Interrupt Signals that a byte has been received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisSrxdone {
    #[doc = "0: CLR"]
    IntEvent0MisSrxdoneClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisSrxdoneSet = 1,
}
impl From<IntEvent0MisSrxdone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisSrxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_SRXDONE` reader - Slave Receive Data Interrupt Signals that a byte has been received"]
pub type IntEvent0MisSrxdoneR = crate::BitReader<IntEvent0MisSrxdone>;
impl IntEvent0MisSrxdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisSrxdone {
        match self.bits {
            false => IntEvent0MisSrxdone::IntEvent0MisSrxdoneClr,
            true => IntEvent0MisSrxdone::IntEvent0MisSrxdoneSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_srxdone_clr(&self) -> bool {
        *self == IntEvent0MisSrxdone::IntEvent0MisSrxdoneClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_srxdone_set(&self) -> bool {
        *self == IntEvent0MisSrxdone::IntEvent0MisSrxdoneSet
    }
}
#[doc = "Slave Transmit Transaction completed Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisStxdone {
    #[doc = "0: CLR"]
    IntEvent0MisStxdoneClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisStxdoneSet = 1,
}
impl From<IntEvent0MisStxdone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisStxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_STXDONE` reader - Slave Transmit Transaction completed Interrupt"]
pub type IntEvent0MisStxdoneR = crate::BitReader<IntEvent0MisStxdone>;
impl IntEvent0MisStxdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisStxdone {
        match self.bits {
            false => IntEvent0MisStxdone::IntEvent0MisStxdoneClr,
            true => IntEvent0MisStxdone::IntEvent0MisStxdoneSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_stxdone_clr(&self) -> bool {
        *self == IntEvent0MisStxdone::IntEvent0MisStxdoneClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_stxdone_set(&self) -> bool {
        *self == IntEvent0MisStxdone::IntEvent0MisStxdoneSet
    }
}
#[doc = "Slave Receive FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisSrxfifotrg {
    #[doc = "0: CLR"]
    IntEvent0MisSrxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisSrxfifotrgSet = 1,
}
impl From<IntEvent0MisSrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisSrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_SRXFIFOTRG` reader - Slave Receive FIFO Trigger"]
pub type IntEvent0MisSrxfifotrgR = crate::BitReader<IntEvent0MisSrxfifotrg>;
impl IntEvent0MisSrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisSrxfifotrg {
        match self.bits {
            false => IntEvent0MisSrxfifotrg::IntEvent0MisSrxfifotrgClr,
            true => IntEvent0MisSrxfifotrg::IntEvent0MisSrxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_srxfifotrg_clr(&self) -> bool {
        *self == IntEvent0MisSrxfifotrg::IntEvent0MisSrxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_srxfifotrg_set(&self) -> bool {
        *self == IntEvent0MisSrxfifotrg::IntEvent0MisSrxfifotrgSet
    }
}
#[doc = "Slave Transmit FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisStxfifotrg {
    #[doc = "0: CLR"]
    IntEvent0MisStxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisStxfifotrgSet = 1,
}
impl From<IntEvent0MisStxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisStxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_STXFIFOTRG` reader - Slave Transmit FIFO Trigger"]
pub type IntEvent0MisStxfifotrgR = crate::BitReader<IntEvent0MisStxfifotrg>;
impl IntEvent0MisStxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisStxfifotrg {
        match self.bits {
            false => IntEvent0MisStxfifotrg::IntEvent0MisStxfifotrgClr,
            true => IntEvent0MisStxfifotrg::IntEvent0MisStxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_stxfifotrg_clr(&self) -> bool {
        *self == IntEvent0MisStxfifotrg::IntEvent0MisStxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_stxfifotrg_set(&self) -> bool {
        *self == IntEvent0MisStxfifotrg::IntEvent0MisStxfifotrgSet
    }
}
#[doc = "RXFIFO full event. This interrupt is set if an RX FIFO is full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisSrxfifofull {
    #[doc = "0: CLR"]
    IntEvent0MisSrxfifofullClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisSrxfifofullSet = 1,
}
impl From<IntEvent0MisSrxfifofull> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisSrxfifofull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_SRXFIFOFULL` reader - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
pub type IntEvent0MisSrxfifofullR = crate::BitReader<IntEvent0MisSrxfifofull>;
impl IntEvent0MisSrxfifofullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisSrxfifofull {
        match self.bits {
            false => IntEvent0MisSrxfifofull::IntEvent0MisSrxfifofullClr,
            true => IntEvent0MisSrxfifofull::IntEvent0MisSrxfifofullSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_srxfifofull_clr(&self) -> bool {
        *self == IntEvent0MisSrxfifofull::IntEvent0MisSrxfifofullClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_srxfifofull_set(&self) -> bool {
        *self == IntEvent0MisSrxfifofull::IntEvent0MisSrxfifofullSet
    }
}
#[doc = "Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisStxempty {
    #[doc = "0: CLR"]
    IntEvent0MisStxemptyClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisStxemptySet = 1,
}
impl From<IntEvent0MisStxempty> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisStxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_STXEMPTY` reader - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
pub type IntEvent0MisStxemptyR = crate::BitReader<IntEvent0MisStxempty>;
impl IntEvent0MisStxemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisStxempty {
        match self.bits {
            false => IntEvent0MisStxempty::IntEvent0MisStxemptyClr,
            true => IntEvent0MisStxempty::IntEvent0MisStxemptySet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_stxempty_clr(&self) -> bool {
        *self == IntEvent0MisStxempty::IntEvent0MisStxemptyClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_stxempty_set(&self) -> bool {
        *self == IntEvent0MisStxempty::IntEvent0MisStxemptySet
    }
}
#[doc = "Slave START Detection Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisSstart {
    #[doc = "0: CLR"]
    IntEvent0MisSstartClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisSstartSet = 1,
}
impl From<IntEvent0MisSstart> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisSstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_SSTART` reader - Slave START Detection Interrupt"]
pub type IntEvent0MisSstartR = crate::BitReader<IntEvent0MisSstart>;
impl IntEvent0MisSstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisSstart {
        match self.bits {
            false => IntEvent0MisSstart::IntEvent0MisSstartClr,
            true => IntEvent0MisSstart::IntEvent0MisSstartSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_sstart_clr(&self) -> bool {
        *self == IntEvent0MisSstart::IntEvent0MisSstartClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_sstart_set(&self) -> bool {
        *self == IntEvent0MisSstart::IntEvent0MisSstartSet
    }
}
#[doc = "Slave STOP Detection Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisSstop {
    #[doc = "0: CLR"]
    IntEvent0MisSstopClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisSstopSet = 1,
}
impl From<IntEvent0MisSstop> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisSstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_SSTOP` reader - Slave STOP Detection Interrupt"]
pub type IntEvent0MisSstopR = crate::BitReader<IntEvent0MisSstop>;
impl IntEvent0MisSstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisSstop {
        match self.bits {
            false => IntEvent0MisSstop::IntEvent0MisSstopClr,
            true => IntEvent0MisSstop::IntEvent0MisSstopSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_sstop_clr(&self) -> bool {
        *self == IntEvent0MisSstop::IntEvent0MisSstopClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_sstop_set(&self) -> bool {
        *self == IntEvent0MisSstop::IntEvent0MisSstopSet
    }
}
#[doc = "General Call Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisSgencall {
    #[doc = "0: CLR"]
    IntEvent0MisSgencallClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisSgencallSet = 1,
}
impl From<IntEvent0MisSgencall> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisSgencall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_SGENCALL` reader - General Call Interrupt"]
pub type IntEvent0MisSgencallR = crate::BitReader<IntEvent0MisSgencall>;
impl IntEvent0MisSgencallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisSgencall {
        match self.bits {
            false => IntEvent0MisSgencall::IntEvent0MisSgencallClr,
            true => IntEvent0MisSgencall::IntEvent0MisSgencallSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_sgencall_clr(&self) -> bool {
        *self == IntEvent0MisSgencall::IntEvent0MisSgencallClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_sgencall_set(&self) -> bool {
        *self == IntEvent0MisSgencall::IntEvent0MisSgencallSet
    }
}
#[doc = "DMA Done 1 on Event Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisSdmaDone1_2 {
    #[doc = "0: CLR"]
    IntEvent0MisSdmaDone1_2Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisSdmaDone1_2Set = 1,
}
impl From<IntEvent0MisSdmaDone1_2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisSdmaDone1_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_SDMA_DONE1_2` reader - DMA Done 1 on Event Channel 2"]
pub type IntEvent0MisSdmaDone1_2R = crate::BitReader<IntEvent0MisSdmaDone1_2>;
impl IntEvent0MisSdmaDone1_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisSdmaDone1_2 {
        match self.bits {
            false => IntEvent0MisSdmaDone1_2::IntEvent0MisSdmaDone1_2Clr,
            true => IntEvent0MisSdmaDone1_2::IntEvent0MisSdmaDone1_2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_sdma_done1_2_clr(&self) -> bool {
        *self == IntEvent0MisSdmaDone1_2::IntEvent0MisSdmaDone1_2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_sdma_done1_2_set(&self) -> bool {
        *self == IntEvent0MisSdmaDone1_2::IntEvent0MisSdmaDone1_2Set
    }
}
#[doc = "DMA Done 1 on Event Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisSdmaDone1_3 {
    #[doc = "0: CLR"]
    IntEvent0MisSdmaDone1_3Clr = 0,
    #[doc = "1: SET"]
    IntEvent0MisSdmaDone1_3Set = 1,
}
impl From<IntEvent0MisSdmaDone1_3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisSdmaDone1_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_SDMA_DONE1_3` reader - DMA Done 1 on Event Channel 3"]
pub type IntEvent0MisSdmaDone1_3R = crate::BitReader<IntEvent0MisSdmaDone1_3>;
impl IntEvent0MisSdmaDone1_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisSdmaDone1_3 {
        match self.bits {
            false => IntEvent0MisSdmaDone1_3::IntEvent0MisSdmaDone1_3Clr,
            true => IntEvent0MisSdmaDone1_3::IntEvent0MisSdmaDone1_3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_sdma_done1_3_clr(&self) -> bool {
        *self == IntEvent0MisSdmaDone1_3::IntEvent0MisSdmaDone1_3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_sdma_done1_3_set(&self) -> bool {
        *self == IntEvent0MisSdmaDone1_3::IntEvent0MisSdmaDone1_3Set
    }
}
#[doc = "Slave RX Pec Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisSpecRxErr {
    #[doc = "0: CLR"]
    IntEvent0MisSpecRxErrClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisSpecRxErrSet = 1,
}
impl From<IntEvent0MisSpecRxErr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisSpecRxErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_SPEC_RX_ERR` reader - Slave RX Pec Error Interrupt"]
pub type IntEvent0MisSpecRxErrR = crate::BitReader<IntEvent0MisSpecRxErr>;
impl IntEvent0MisSpecRxErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisSpecRxErr {
        match self.bits {
            false => IntEvent0MisSpecRxErr::IntEvent0MisSpecRxErrClr,
            true => IntEvent0MisSpecRxErr::IntEvent0MisSpecRxErrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_spec_rx_err_clr(&self) -> bool {
        *self == IntEvent0MisSpecRxErr::IntEvent0MisSpecRxErrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_spec_rx_err_set(&self) -> bool {
        *self == IntEvent0MisSpecRxErr::IntEvent0MisSpecRxErrSet
    }
}
#[doc = "Slave TX FIFO underflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisStxUnfl {
    #[doc = "0: CLR"]
    IntEvent0MisStxUnflClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisStxUnflSet = 1,
}
impl From<IntEvent0MisStxUnfl> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisStxUnfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_STX_UNFL` reader - Slave TX FIFO underflow"]
pub type IntEvent0MisStxUnflR = crate::BitReader<IntEvent0MisStxUnfl>;
impl IntEvent0MisStxUnflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisStxUnfl {
        match self.bits {
            false => IntEvent0MisStxUnfl::IntEvent0MisStxUnflClr,
            true => IntEvent0MisStxUnfl::IntEvent0MisStxUnflSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_stx_unfl_clr(&self) -> bool {
        *self == IntEvent0MisStxUnfl::IntEvent0MisStxUnflClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_stx_unfl_set(&self) -> bool {
        *self == IntEvent0MisStxUnfl::IntEvent0MisStxUnflSet
    }
}
#[doc = "Slave RX FIFO overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisSrxOvfl {
    #[doc = "0: CLR"]
    IntEvent0MisSrxOvflClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisSrxOvflSet = 1,
}
impl From<IntEvent0MisSrxOvfl> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisSrxOvfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_SRX_OVFL` reader - Slave RX FIFO overflow"]
pub type IntEvent0MisSrxOvflR = crate::BitReader<IntEvent0MisSrxOvfl>;
impl IntEvent0MisSrxOvflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisSrxOvfl {
        match self.bits {
            false => IntEvent0MisSrxOvfl::IntEvent0MisSrxOvflClr,
            true => IntEvent0MisSrxOvfl::IntEvent0MisSrxOvflSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_srx_ovfl_clr(&self) -> bool {
        *self == IntEvent0MisSrxOvfl::IntEvent0MisSrxOvflClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_srx_ovfl_set(&self) -> bool {
        *self == IntEvent0MisSrxOvfl::IntEvent0MisSrxOvflSet
    }
}
#[doc = "Slave Arbitration Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisSarblost {
    #[doc = "0: CLR"]
    IntEvent0MisSarblostClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisSarblostSet = 1,
}
impl From<IntEvent0MisSarblost> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisSarblost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_SARBLOST` reader - Slave Arbitration Lost"]
pub type IntEvent0MisSarblostR = crate::BitReader<IntEvent0MisSarblost>;
impl IntEvent0MisSarblostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisSarblost {
        match self.bits {
            false => IntEvent0MisSarblost::IntEvent0MisSarblostClr,
            true => IntEvent0MisSarblost::IntEvent0MisSarblostSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_sarblost_clr(&self) -> bool {
        *self == IntEvent0MisSarblost::IntEvent0MisSarblostClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_sarblost_set(&self) -> bool {
        *self == IntEvent0MisSarblost::IntEvent0MisSarblostSet
    }
}
#[doc = "Interrupt overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0MisIntrOvfl {
    #[doc = "0: CLR"]
    IntEvent0MisIntrOvflClr = 0,
    #[doc = "1: SET"]
    IntEvent0MisIntrOvflSet = 1,
}
impl From<IntEvent0MisIntrOvfl> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0MisIntrOvfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_MIS_INTR_OVFL` reader - Interrupt overflow"]
pub type IntEvent0MisIntrOvflR = crate::BitReader<IntEvent0MisIntrOvfl>;
impl IntEvent0MisIntrOvflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0MisIntrOvfl {
        match self.bits {
            false => IntEvent0MisIntrOvfl::IntEvent0MisIntrOvflClr,
            true => IntEvent0MisIntrOvfl::IntEvent0MisIntrOvflSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_mis_intr_ovfl_clr(&self) -> bool {
        *self == IntEvent0MisIntrOvfl::IntEvent0MisIntrOvflClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_mis_intr_ovfl_set(&self) -> bool {
        *self == IntEvent0MisIntrOvfl::IntEvent0MisIntrOvflSet
    }
}
impl R {
    #[doc = "Bit 0 - Master Receive Data Interrupt"]
    #[inline(always)]
    pub fn int_event0_mis_mrxdone(&self) -> IntEvent0MisMrxdoneR {
        IntEvent0MisMrxdoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn int_event0_mis_mtxdone(&self) -> IntEvent0MisMtxdoneR {
        IntEvent0MisMtxdoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
    #[inline(always)]
    pub fn int_event0_mis_mrxfifotrg(&self) -> IntEvent0MisMrxfifotrgR {
        IntEvent0MisMrxfifotrgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
    #[inline(always)]
    pub fn int_event0_mis_mtxfifotrg(&self) -> IntEvent0MisMtxfifotrgR {
        IntEvent0MisMtxfifotrgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFIFO full event. This interrupt is set if the RX FIFO is full."]
    #[inline(always)]
    pub fn int_event0_mis_mrxfifofull(&self) -> IntEvent0MisMrxfifofullR {
        IntEvent0MisMrxfifofullR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
    #[inline(always)]
    pub fn int_event0_mis_mtxempty(&self) -> IntEvent0MisMtxemptyR {
        IntEvent0MisMtxemptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Address/Data NACK Interrupt"]
    #[inline(always)]
    pub fn int_event0_mis_mnack(&self) -> IntEvent0MisMnackR {
        IntEvent0MisMnackR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - START Detection Interrupt"]
    #[inline(always)]
    pub fn int_event0_mis_mstart(&self) -> IntEvent0MisMstartR {
        IntEvent0MisMstartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STOP Detection Interrupt"]
    #[inline(always)]
    pub fn int_event0_mis_mstop(&self) -> IntEvent0MisMstopR {
        IntEvent0MisMstopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Arbitration Lost Interrupt"]
    #[inline(always)]
    pub fn int_event0_mis_marblost(&self) -> IntEvent0MisMarblostR {
        IntEvent0MisMarblostR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA Done 1 on Event Channel 2"]
    #[inline(always)]
    pub fn int_event0_mis_mdma_done1_2(&self) -> IntEvent0MisMdmaDone1_2R {
        IntEvent0MisMdmaDone1_2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA Done 1 on Event Channel 3"]
    #[inline(always)]
    pub fn int_event0_mis_mdma_done1_3(&self) -> IntEvent0MisMdmaDone1_3R {
        IntEvent0MisMdmaDone1_3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Master RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn int_event0_mis_mpec_rx_err(&self) -> IntEvent0MisMpecRxErrR {
        IntEvent0MisMpecRxErrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Timeout A Interrupt"]
    #[inline(always)]
    pub fn int_event0_mis_timeouta(&self) -> IntEvent0MisTimeoutaR {
        IntEvent0MisTimeoutaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timeout B Interrupt"]
    #[inline(always)]
    pub fn int_event0_mis_timeoutb(&self) -> IntEvent0MisTimeoutbR {
        IntEvent0MisTimeoutbR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave Receive Data Interrupt Signals that a byte has been received"]
    #[inline(always)]
    pub fn int_event0_mis_srxdone(&self) -> IntEvent0MisSrxdoneR {
        IntEvent0MisSrxdoneR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Slave Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn int_event0_mis_stxdone(&self) -> IntEvent0MisStxdoneR {
        IntEvent0MisStxdoneR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Slave Receive FIFO Trigger"]
    #[inline(always)]
    pub fn int_event0_mis_srxfifotrg(&self) -> IntEvent0MisSrxfifotrgR {
        IntEvent0MisSrxfifotrgR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Slave Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn int_event0_mis_stxfifotrg(&self) -> IntEvent0MisStxfifotrgR {
        IntEvent0MisStxfifotrgR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
    #[inline(always)]
    pub fn int_event0_mis_srxfifofull(&self) -> IntEvent0MisSrxfifofullR {
        IntEvent0MisSrxfifofullR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
    #[inline(always)]
    pub fn int_event0_mis_stxempty(&self) -> IntEvent0MisStxemptyR {
        IntEvent0MisStxemptyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Slave START Detection Interrupt"]
    #[inline(always)]
    pub fn int_event0_mis_sstart(&self) -> IntEvent0MisSstartR {
        IntEvent0MisSstartR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Slave STOP Detection Interrupt"]
    #[inline(always)]
    pub fn int_event0_mis_sstop(&self) -> IntEvent0MisSstopR {
        IntEvent0MisSstopR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - General Call Interrupt"]
    #[inline(always)]
    pub fn int_event0_mis_sgencall(&self) -> IntEvent0MisSgencallR {
        IntEvent0MisSgencallR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA Done 1 on Event Channel 2"]
    #[inline(always)]
    pub fn int_event0_mis_sdma_done1_2(&self) -> IntEvent0MisSdmaDone1_2R {
        IntEvent0MisSdmaDone1_2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DMA Done 1 on Event Channel 3"]
    #[inline(always)]
    pub fn int_event0_mis_sdma_done1_3(&self) -> IntEvent0MisSdmaDone1_3R {
        IntEvent0MisSdmaDone1_3R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Slave RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn int_event0_mis_spec_rx_err(&self) -> IntEvent0MisSpecRxErrR {
        IntEvent0MisSpecRxErrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Slave TX FIFO underflow"]
    #[inline(always)]
    pub fn int_event0_mis_stx_unfl(&self) -> IntEvent0MisStxUnflR {
        IntEvent0MisStxUnflR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Slave RX FIFO overflow"]
    #[inline(always)]
    pub fn int_event0_mis_srx_ovfl(&self) -> IntEvent0MisSrxOvflR {
        IntEvent0MisSrxOvflR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Slave Arbitration Lost"]
    #[inline(always)]
    pub fn int_event0_mis_sarblost(&self) -> IntEvent0MisSarblostR {
        IntEvent0MisSarblostR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt overflow"]
    #[inline(always)]
    pub fn int_event0_mis_intr_ovfl(&self) -> IntEvent0MisIntrOvflR {
        IntEvent0MisIntrOvflR::new(((self.bits >> 31) & 1) != 0)
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
