#[doc = "Register `INT_EVENT0_ISET` writer"]
pub type W = crate::W<IntEvent0IsetSpec>;
#[doc = "Master Receive Data Interrupt Signals that a byte has been received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMrxdone {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMrxdoneNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMrxdoneSet = 1,
}
impl From<IntEvent0IsetMrxdone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMrxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MRXDONE` writer - Master Receive Data Interrupt Signals that a byte has been received"]
pub type IntEvent0IsetMrxdoneW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMrxdone>;
impl<'a, REG> IntEvent0IsetMrxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_mrxdone_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMrxdone::IntEvent0IsetMrxdoneNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_mrxdone_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMrxdone::IntEvent0IsetMrxdoneSet)
    }
}
#[doc = "Master Transmit Transaction completed Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMtxdone {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMtxdoneNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMtxdoneSet = 1,
}
impl From<IntEvent0IsetMtxdone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMtxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MTXDONE` writer - Master Transmit Transaction completed Interrupt"]
pub type IntEvent0IsetMtxdoneW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMtxdone>;
impl<'a, REG> IntEvent0IsetMtxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_mtxdone_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMtxdone::IntEvent0IsetMtxdoneNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_mtxdone_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMtxdone::IntEvent0IsetMtxdoneSet)
    }
}
#[doc = "Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMrxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMrxfifotrgNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMrxfifotrgSet = 1,
}
impl From<IntEvent0IsetMrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MRXFIFOTRG` writer - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
pub type IntEvent0IsetMrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMrxfifotrg>;
impl<'a, REG> IntEvent0IsetMrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_mrxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMrxfifotrg::IntEvent0IsetMrxfifotrgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_mrxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMrxfifotrg::IntEvent0IsetMrxfifotrgSet)
    }
}
#[doc = "Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMtxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMtxfifotrgNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMtxfifotrgSet = 1,
}
impl From<IntEvent0IsetMtxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMtxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MTXFIFOTRG` writer - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
pub type IntEvent0IsetMtxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMtxfifotrg>;
impl<'a, REG> IntEvent0IsetMtxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_mtxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMtxfifotrg::IntEvent0IsetMtxfifotrgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_mtxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMtxfifotrg::IntEvent0IsetMtxfifotrgSet)
    }
}
#[doc = "RXFIFO full event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMrxfifofull {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMrxfifofullNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMrxfifofullSet = 1,
}
impl From<IntEvent0IsetMrxfifofull> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMrxfifofull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MRXFIFOFULL` writer - RXFIFO full event."]
pub type IntEvent0IsetMrxfifofullW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMrxfifofull>;
impl<'a, REG> IntEvent0IsetMrxfifofullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_mrxfifofull_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMrxfifofull::IntEvent0IsetMrxfifofullNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_mrxfifofull_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMrxfifofull::IntEvent0IsetMrxfifofullSet)
    }
}
#[doc = "Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMtxempty {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMtxemptyNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMtxemptySet = 1,
}
impl From<IntEvent0IsetMtxempty> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMtxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MTXEMPTY` writer - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
pub type IntEvent0IsetMtxemptyW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMtxempty>;
impl<'a, REG> IntEvent0IsetMtxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_mtxempty_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMtxempty::IntEvent0IsetMtxemptyNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_mtxempty_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMtxempty::IntEvent0IsetMtxemptySet)
    }
}
#[doc = "Address/Data NACK Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMnack {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMnackNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMnackSet = 1,
}
impl From<IntEvent0IsetMnack> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMnack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MNACK` writer - Address/Data NACK Interrupt"]
pub type IntEvent0IsetMnackW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMnack>;
impl<'a, REG> IntEvent0IsetMnackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_mnack_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMnack::IntEvent0IsetMnackNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_mnack_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMnack::IntEvent0IsetMnackSet)
    }
}
#[doc = "START Detection Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMstart {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMstartNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMstartSet = 1,
}
impl From<IntEvent0IsetMstart> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MSTART` writer - START Detection Interrupt"]
pub type IntEvent0IsetMstartW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMstart>;
impl<'a, REG> IntEvent0IsetMstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_mstart_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMstart::IntEvent0IsetMstartNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_mstart_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMstart::IntEvent0IsetMstartSet)
    }
}
#[doc = "STOP Detection Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMstop {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMstopNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMstopSet = 1,
}
impl From<IntEvent0IsetMstop> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MSTOP` writer - STOP Detection Interrupt"]
pub type IntEvent0IsetMstopW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMstop>;
impl<'a, REG> IntEvent0IsetMstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_mstop_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMstop::IntEvent0IsetMstopNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_mstop_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMstop::IntEvent0IsetMstopSet)
    }
}
#[doc = "Arbitration Lost Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMarblost {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMarblostNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMarblostSet = 1,
}
impl From<IntEvent0IsetMarblost> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMarblost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MARBLOST` writer - Arbitration Lost Interrupt"]
pub type IntEvent0IsetMarblostW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMarblost>;
impl<'a, REG> IntEvent0IsetMarblostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_marblost_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMarblost::IntEvent0IsetMarblostNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_marblost_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMarblost::IntEvent0IsetMarblostSet)
    }
}
#[doc = "DMA Done 1 on Event Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMdmaDone1_2 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMdmaDone1_2NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMdmaDone1_2Set = 1,
}
impl From<IntEvent0IsetMdmaDone1_2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMdmaDone1_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MDMA_DONE1_2` writer - DMA Done 1 on Event Channel 2"]
pub type IntEvent0IsetMdmaDone1_2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMdmaDone1_2>;
impl<'a, REG> IntEvent0IsetMdmaDone1_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_mdma_done1_2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMdmaDone1_2::IntEvent0IsetMdmaDone1_2NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_mdma_done1_2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMdmaDone1_2::IntEvent0IsetMdmaDone1_2Set)
    }
}
#[doc = "DMA Done 1 on Event Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMdmaDone1_3 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMdmaDone1_3NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMdmaDone1_3Set = 1,
}
impl From<IntEvent0IsetMdmaDone1_3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMdmaDone1_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MDMA_DONE1_3` writer - DMA Done 1 on Event Channel 3"]
pub type IntEvent0IsetMdmaDone1_3W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMdmaDone1_3>;
impl<'a, REG> IntEvent0IsetMdmaDone1_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_mdma_done1_3_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMdmaDone1_3::IntEvent0IsetMdmaDone1_3NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_mdma_done1_3_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMdmaDone1_3::IntEvent0IsetMdmaDone1_3Set)
    }
}
#[doc = "Master RX Pec Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetMpecRxErr {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetMpecRxErrNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetMpecRxErrSet = 1,
}
impl From<IntEvent0IsetMpecRxErr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetMpecRxErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_MPEC_RX_ERR` writer - Master RX Pec Error Interrupt"]
pub type IntEvent0IsetMpecRxErrW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetMpecRxErr>;
impl<'a, REG> IntEvent0IsetMpecRxErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_mpec_rx_err_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMpecRxErr::IntEvent0IsetMpecRxErrNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_mpec_rx_err_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetMpecRxErr::IntEvent0IsetMpecRxErrSet)
    }
}
#[doc = "Timeout A interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetTimeouta {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetTimeoutaNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetTimeoutaSet = 1,
}
impl From<IntEvent0IsetTimeouta> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetTimeouta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_TIMEOUTA` writer - Timeout A interrupt"]
pub type IntEvent0IsetTimeoutaW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetTimeouta>;
impl<'a, REG> IntEvent0IsetTimeoutaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_timeouta_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetTimeouta::IntEvent0IsetTimeoutaNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_timeouta_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetTimeouta::IntEvent0IsetTimeoutaSet)
    }
}
#[doc = "Timeout B Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetTimeoutb {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetTimeoutbNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetTimeoutbSet = 1,
}
impl From<IntEvent0IsetTimeoutb> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetTimeoutb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_TIMEOUTB` writer - Timeout B Interrupt"]
pub type IntEvent0IsetTimeoutbW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetTimeoutb>;
impl<'a, REG> IntEvent0IsetTimeoutbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_timeoutb_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetTimeoutb::IntEvent0IsetTimeoutbNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_timeoutb_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetTimeoutb::IntEvent0IsetTimeoutbSet)
    }
}
#[doc = "Slave Receive Data Interrupt Signals that a byte has been received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetSrxdone {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetSrxdoneNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetSrxdoneSet = 1,
}
impl From<IntEvent0IsetSrxdone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetSrxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_SRXDONE` writer - Slave Receive Data Interrupt Signals that a byte has been received"]
pub type IntEvent0IsetSrxdoneW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetSrxdone>;
impl<'a, REG> IntEvent0IsetSrxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_srxdone_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSrxdone::IntEvent0IsetSrxdoneNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_srxdone_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSrxdone::IntEvent0IsetSrxdoneSet)
    }
}
#[doc = "Slave Transmit Transaction completed Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetStxdone {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetStxdoneNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetStxdoneSet = 1,
}
impl From<IntEvent0IsetStxdone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetStxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_STXDONE` writer - Slave Transmit Transaction completed Interrupt"]
pub type IntEvent0IsetStxdoneW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetStxdone>;
impl<'a, REG> IntEvent0IsetStxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_stxdone_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetStxdone::IntEvent0IsetStxdoneNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_stxdone_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetStxdone::IntEvent0IsetStxdoneSet)
    }
}
#[doc = "Slave Receive FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetSrxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetSrxfifotrgNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetSrxfifotrgSet = 1,
}
impl From<IntEvent0IsetSrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetSrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_SRXFIFOTRG` writer - Slave Receive FIFO Trigger"]
pub type IntEvent0IsetSrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetSrxfifotrg>;
impl<'a, REG> IntEvent0IsetSrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_srxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSrxfifotrg::IntEvent0IsetSrxfifotrgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_srxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSrxfifotrg::IntEvent0IsetSrxfifotrgSet)
    }
}
#[doc = "Slave Transmit FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetStxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetStxfifotrgNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetStxfifotrgSet = 1,
}
impl From<IntEvent0IsetStxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetStxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_STXFIFOTRG` writer - Slave Transmit FIFO Trigger"]
pub type IntEvent0IsetStxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetStxfifotrg>;
impl<'a, REG> IntEvent0IsetStxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_stxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetStxfifotrg::IntEvent0IsetStxfifotrgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_stxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetStxfifotrg::IntEvent0IsetStxfifotrgSet)
    }
}
#[doc = "RXFIFO full event. This interrupt is set if an RX FIFO is full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetSrxfifofull {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetSrxfifofullNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetSrxfifofullSet = 1,
}
impl From<IntEvent0IsetSrxfifofull> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetSrxfifofull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_SRXFIFOFULL` writer - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
pub type IntEvent0IsetSrxfifofullW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetSrxfifofull>;
impl<'a, REG> IntEvent0IsetSrxfifofullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_srxfifofull_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSrxfifofull::IntEvent0IsetSrxfifofullNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_srxfifofull_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSrxfifofull::IntEvent0IsetSrxfifofullSet)
    }
}
#[doc = "Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetStxempty {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetStxemptyNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetStxemptySet = 1,
}
impl From<IntEvent0IsetStxempty> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetStxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_STXEMPTY` writer - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
pub type IntEvent0IsetStxemptyW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetStxempty>;
impl<'a, REG> IntEvent0IsetStxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_stxempty_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetStxempty::IntEvent0IsetStxemptyNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_stxempty_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetStxempty::IntEvent0IsetStxemptySet)
    }
}
#[doc = "Start Condition Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetSstart {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetSstartNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetSstartSet = 1,
}
impl From<IntEvent0IsetSstart> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetSstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_SSTART` writer - Start Condition Interrupt"]
pub type IntEvent0IsetSstartW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetSstart>;
impl<'a, REG> IntEvent0IsetSstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_sstart_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSstart::IntEvent0IsetSstartNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_sstart_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSstart::IntEvent0IsetSstartSet)
    }
}
#[doc = "Stop Condition Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetSstop {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetSstopNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetSstopSet = 1,
}
impl From<IntEvent0IsetSstop> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetSstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_SSTOP` writer - Stop Condition Interrupt"]
pub type IntEvent0IsetSstopW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetSstop>;
impl<'a, REG> IntEvent0IsetSstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_sstop_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSstop::IntEvent0IsetSstopNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_sstop_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSstop::IntEvent0IsetSstopSet)
    }
}
#[doc = "General Call Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetSgencall {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetSgencallNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetSgencallSet = 1,
}
impl From<IntEvent0IsetSgencall> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetSgencall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_SGENCALL` writer - General Call Interrupt"]
pub type IntEvent0IsetSgencallW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetSgencall>;
impl<'a, REG> IntEvent0IsetSgencallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_sgencall_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSgencall::IntEvent0IsetSgencallNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_sgencall_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSgencall::IntEvent0IsetSgencallSet)
    }
}
#[doc = "DMA Done 1 on Event Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetSdmaDone1_2 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetSdmaDone1_2NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetSdmaDone1_2Set = 1,
}
impl From<IntEvent0IsetSdmaDone1_2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetSdmaDone1_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_SDMA_DONE1_2` writer - DMA Done 1 on Event Channel 2"]
pub type IntEvent0IsetSdmaDone1_2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetSdmaDone1_2>;
impl<'a, REG> IntEvent0IsetSdmaDone1_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_sdma_done1_2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSdmaDone1_2::IntEvent0IsetSdmaDone1_2NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_sdma_done1_2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSdmaDone1_2::IntEvent0IsetSdmaDone1_2Set)
    }
}
#[doc = "DMA Done 1 on Event Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetSdmaDone1_3 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetSdmaDone1_3NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetSdmaDone1_3Set = 1,
}
impl From<IntEvent0IsetSdmaDone1_3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetSdmaDone1_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_SDMA_DONE1_3` writer - DMA Done 1 on Event Channel 3"]
pub type IntEvent0IsetSdmaDone1_3W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetSdmaDone1_3>;
impl<'a, REG> IntEvent0IsetSdmaDone1_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_sdma_done1_3_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSdmaDone1_3::IntEvent0IsetSdmaDone1_3NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_sdma_done1_3_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSdmaDone1_3::IntEvent0IsetSdmaDone1_3Set)
    }
}
#[doc = "Slave RX Pec Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetSpecRxErr {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetSpecRxErrNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetSpecRxErrSet = 1,
}
impl From<IntEvent0IsetSpecRxErr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetSpecRxErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_SPEC_RX_ERR` writer - Slave RX Pec Error Interrupt"]
pub type IntEvent0IsetSpecRxErrW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetSpecRxErr>;
impl<'a, REG> IntEvent0IsetSpecRxErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_spec_rx_err_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSpecRxErr::IntEvent0IsetSpecRxErrNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_spec_rx_err_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSpecRxErr::IntEvent0IsetSpecRxErrSet)
    }
}
#[doc = "Slave TX FIFO underflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetStxUnfl {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetStxUnflNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetStxUnflSet = 1,
}
impl From<IntEvent0IsetStxUnfl> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetStxUnfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_STX_UNFL` writer - Slave TX FIFO underflow"]
pub type IntEvent0IsetStxUnflW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetStxUnfl>;
impl<'a, REG> IntEvent0IsetStxUnflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_stx_unfl_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetStxUnfl::IntEvent0IsetStxUnflNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_stx_unfl_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetStxUnfl::IntEvent0IsetStxUnflSet)
    }
}
#[doc = "Slave RX FIFO overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetSrxOvfl {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetSrxOvflNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetSrxOvflSet = 1,
}
impl From<IntEvent0IsetSrxOvfl> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetSrxOvfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_SRX_OVFL` writer - Slave RX FIFO overflow"]
pub type IntEvent0IsetSrxOvflW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetSrxOvfl>;
impl<'a, REG> IntEvent0IsetSrxOvflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_srx_ovfl_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSrxOvfl::IntEvent0IsetSrxOvflNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_srx_ovfl_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSrxOvfl::IntEvent0IsetSrxOvflSet)
    }
}
#[doc = "Slave Arbitration Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetSarblost {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetSarblostNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetSarblostSet = 1,
}
impl From<IntEvent0IsetSarblost> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetSarblost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_SARBLOST` writer - Slave Arbitration Lost"]
pub type IntEvent0IsetSarblostW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetSarblost>;
impl<'a, REG> IntEvent0IsetSarblostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_sarblost_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSarblost::IntEvent0IsetSarblostNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_sarblost_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetSarblost::IntEvent0IsetSarblostSet)
    }
}
#[doc = "Interrupt overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetIntrOvfl {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetIntrOvflNoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetIntrOvflSet = 1,
}
impl From<IntEvent0IsetIntrOvfl> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetIntrOvfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_INTR_OVFL` writer - Interrupt overflow"]
pub type IntEvent0IsetIntrOvflW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetIntrOvfl>;
impl<'a, REG> IntEvent0IsetIntrOvflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_intr_ovfl_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetIntrOvfl::IntEvent0IsetIntrOvflNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_intr_ovfl_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetIntrOvfl::IntEvent0IsetIntrOvflSet)
    }
}
impl W {
    #[doc = "Bit 0 - Master Receive Data Interrupt Signals that a byte has been received"]
    #[inline(always)]
    pub fn int_event0_iset_mrxdone(&mut self) -> IntEvent0IsetMrxdoneW<IntEvent0IsetSpec> {
        IntEvent0IsetMrxdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Master Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn int_event0_iset_mtxdone(&mut self) -> IntEvent0IsetMtxdoneW<IntEvent0IsetSpec> {
        IntEvent0IsetMtxdoneW::new(self, 1)
    }
    #[doc = "Bit 2 - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
    #[inline(always)]
    pub fn int_event0_iset_mrxfifotrg(&mut self) -> IntEvent0IsetMrxfifotrgW<IntEvent0IsetSpec> {
        IntEvent0IsetMrxfifotrgW::new(self, 2)
    }
    #[doc = "Bit 3 - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
    #[inline(always)]
    pub fn int_event0_iset_mtxfifotrg(&mut self) -> IntEvent0IsetMtxfifotrgW<IntEvent0IsetSpec> {
        IntEvent0IsetMtxfifotrgW::new(self, 3)
    }
    #[doc = "Bit 4 - RXFIFO full event."]
    #[inline(always)]
    pub fn int_event0_iset_mrxfifofull(&mut self) -> IntEvent0IsetMrxfifofullW<IntEvent0IsetSpec> {
        IntEvent0IsetMrxfifofullW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
    #[inline(always)]
    pub fn int_event0_iset_mtxempty(&mut self) -> IntEvent0IsetMtxemptyW<IntEvent0IsetSpec> {
        IntEvent0IsetMtxemptyW::new(self, 5)
    }
    #[doc = "Bit 7 - Address/Data NACK Interrupt"]
    #[inline(always)]
    pub fn int_event0_iset_mnack(&mut self) -> IntEvent0IsetMnackW<IntEvent0IsetSpec> {
        IntEvent0IsetMnackW::new(self, 7)
    }
    #[doc = "Bit 8 - START Detection Interrupt"]
    #[inline(always)]
    pub fn int_event0_iset_mstart(&mut self) -> IntEvent0IsetMstartW<IntEvent0IsetSpec> {
        IntEvent0IsetMstartW::new(self, 8)
    }
    #[doc = "Bit 9 - STOP Detection Interrupt"]
    #[inline(always)]
    pub fn int_event0_iset_mstop(&mut self) -> IntEvent0IsetMstopW<IntEvent0IsetSpec> {
        IntEvent0IsetMstopW::new(self, 9)
    }
    #[doc = "Bit 10 - Arbitration Lost Interrupt"]
    #[inline(always)]
    pub fn int_event0_iset_marblost(&mut self) -> IntEvent0IsetMarblostW<IntEvent0IsetSpec> {
        IntEvent0IsetMarblostW::new(self, 10)
    }
    #[doc = "Bit 11 - DMA Done 1 on Event Channel 2"]
    #[inline(always)]
    pub fn int_event0_iset_mdma_done1_2(&mut self) -> IntEvent0IsetMdmaDone1_2W<IntEvent0IsetSpec> {
        IntEvent0IsetMdmaDone1_2W::new(self, 11)
    }
    #[doc = "Bit 12 - DMA Done 1 on Event Channel 3"]
    #[inline(always)]
    pub fn int_event0_iset_mdma_done1_3(&mut self) -> IntEvent0IsetMdmaDone1_3W<IntEvent0IsetSpec> {
        IntEvent0IsetMdmaDone1_3W::new(self, 12)
    }
    #[doc = "Bit 13 - Master RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn int_event0_iset_mpec_rx_err(&mut self) -> IntEvent0IsetMpecRxErrW<IntEvent0IsetSpec> {
        IntEvent0IsetMpecRxErrW::new(self, 13)
    }
    #[doc = "Bit 14 - Timeout A interrupt"]
    #[inline(always)]
    pub fn int_event0_iset_timeouta(&mut self) -> IntEvent0IsetTimeoutaW<IntEvent0IsetSpec> {
        IntEvent0IsetTimeoutaW::new(self, 14)
    }
    #[doc = "Bit 15 - Timeout B Interrupt"]
    #[inline(always)]
    pub fn int_event0_iset_timeoutb(&mut self) -> IntEvent0IsetTimeoutbW<IntEvent0IsetSpec> {
        IntEvent0IsetTimeoutbW::new(self, 15)
    }
    #[doc = "Bit 16 - Slave Receive Data Interrupt Signals that a byte has been received"]
    #[inline(always)]
    pub fn int_event0_iset_srxdone(&mut self) -> IntEvent0IsetSrxdoneW<IntEvent0IsetSpec> {
        IntEvent0IsetSrxdoneW::new(self, 16)
    }
    #[doc = "Bit 17 - Slave Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn int_event0_iset_stxdone(&mut self) -> IntEvent0IsetStxdoneW<IntEvent0IsetSpec> {
        IntEvent0IsetStxdoneW::new(self, 17)
    }
    #[doc = "Bit 18 - Slave Receive FIFO Trigger"]
    #[inline(always)]
    pub fn int_event0_iset_srxfifotrg(&mut self) -> IntEvent0IsetSrxfifotrgW<IntEvent0IsetSpec> {
        IntEvent0IsetSrxfifotrgW::new(self, 18)
    }
    #[doc = "Bit 19 - Slave Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn int_event0_iset_stxfifotrg(&mut self) -> IntEvent0IsetStxfifotrgW<IntEvent0IsetSpec> {
        IntEvent0IsetStxfifotrgW::new(self, 19)
    }
    #[doc = "Bit 20 - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
    #[inline(always)]
    pub fn int_event0_iset_srxfifofull(&mut self) -> IntEvent0IsetSrxfifofullW<IntEvent0IsetSpec> {
        IntEvent0IsetSrxfifofullW::new(self, 20)
    }
    #[doc = "Bit 21 - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
    #[inline(always)]
    pub fn int_event0_iset_stxempty(&mut self) -> IntEvent0IsetStxemptyW<IntEvent0IsetSpec> {
        IntEvent0IsetStxemptyW::new(self, 21)
    }
    #[doc = "Bit 22 - Start Condition Interrupt"]
    #[inline(always)]
    pub fn int_event0_iset_sstart(&mut self) -> IntEvent0IsetSstartW<IntEvent0IsetSpec> {
        IntEvent0IsetSstartW::new(self, 22)
    }
    #[doc = "Bit 23 - Stop Condition Interrupt"]
    #[inline(always)]
    pub fn int_event0_iset_sstop(&mut self) -> IntEvent0IsetSstopW<IntEvent0IsetSpec> {
        IntEvent0IsetSstopW::new(self, 23)
    }
    #[doc = "Bit 24 - General Call Interrupt"]
    #[inline(always)]
    pub fn int_event0_iset_sgencall(&mut self) -> IntEvent0IsetSgencallW<IntEvent0IsetSpec> {
        IntEvent0IsetSgencallW::new(self, 24)
    }
    #[doc = "Bit 25 - DMA Done 1 on Event Channel 2"]
    #[inline(always)]
    pub fn int_event0_iset_sdma_done1_2(&mut self) -> IntEvent0IsetSdmaDone1_2W<IntEvent0IsetSpec> {
        IntEvent0IsetSdmaDone1_2W::new(self, 25)
    }
    #[doc = "Bit 26 - DMA Done 1 on Event Channel 3"]
    #[inline(always)]
    pub fn int_event0_iset_sdma_done1_3(&mut self) -> IntEvent0IsetSdmaDone1_3W<IntEvent0IsetSpec> {
        IntEvent0IsetSdmaDone1_3W::new(self, 26)
    }
    #[doc = "Bit 27 - Slave RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn int_event0_iset_spec_rx_err(&mut self) -> IntEvent0IsetSpecRxErrW<IntEvent0IsetSpec> {
        IntEvent0IsetSpecRxErrW::new(self, 27)
    }
    #[doc = "Bit 28 - Slave TX FIFO underflow"]
    #[inline(always)]
    pub fn int_event0_iset_stx_unfl(&mut self) -> IntEvent0IsetStxUnflW<IntEvent0IsetSpec> {
        IntEvent0IsetStxUnflW::new(self, 28)
    }
    #[doc = "Bit 29 - Slave RX FIFO overflow"]
    #[inline(always)]
    pub fn int_event0_iset_srx_ovfl(&mut self) -> IntEvent0IsetSrxOvflW<IntEvent0IsetSpec> {
        IntEvent0IsetSrxOvflW::new(self, 29)
    }
    #[doc = "Bit 30 - Slave Arbitration Lost"]
    #[inline(always)]
    pub fn int_event0_iset_sarblost(&mut self) -> IntEvent0IsetSarblostW<IntEvent0IsetSpec> {
        IntEvent0IsetSarblostW::new(self, 30)
    }
    #[doc = "Bit 31 - Interrupt overflow"]
    #[inline(always)]
    pub fn int_event0_iset_intr_ovfl(&mut self) -> IntEvent0IsetIntrOvflW<IntEvent0IsetSpec> {
        IntEvent0IsetIntrOvflW::new(self, 31)
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
