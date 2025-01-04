#[doc = "Register `INT_EVENT0_ICLR` writer"]
pub type W = crate::W<IntEvent0IclrSpec>;
#[doc = "Master Receive Data Interrupt Signals that a byte has been received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMrxdone {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMrxdoneNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMrxdoneClr = 1,
}
impl From<IntEvent0IclrMrxdone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMrxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MRXDONE` writer - Master Receive Data Interrupt Signals that a byte has been received"]
pub type IntEvent0IclrMrxdoneW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMrxdone>;
impl<'a, REG> IntEvent0IclrMrxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_mrxdone_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMrxdone::IntEvent0IclrMrxdoneNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_mrxdone_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMrxdone::IntEvent0IclrMrxdoneClr)
    }
}
#[doc = "Master Transmit Transaction completed Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMtxdone {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMtxdoneNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMtxdoneClr = 1,
}
impl From<IntEvent0IclrMtxdone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMtxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MTXDONE` writer - Master Transmit Transaction completed Interrupt"]
pub type IntEvent0IclrMtxdoneW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMtxdone>;
impl<'a, REG> IntEvent0IclrMtxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_mtxdone_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMtxdone::IntEvent0IclrMtxdoneNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_mtxdone_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMtxdone::IntEvent0IclrMtxdoneClr)
    }
}
#[doc = "Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMrxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMrxfifotrgNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMrxfifotrgClr = 1,
}
impl From<IntEvent0IclrMrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MRXFIFOTRG` writer - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
pub type IntEvent0IclrMrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMrxfifotrg>;
impl<'a, REG> IntEvent0IclrMrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_mrxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMrxfifotrg::IntEvent0IclrMrxfifotrgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_mrxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMrxfifotrg::IntEvent0IclrMrxfifotrgClr)
    }
}
#[doc = "Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMtxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMtxfifotrgNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMtxfifotrgClr = 1,
}
impl From<IntEvent0IclrMtxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMtxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MTXFIFOTRG` writer - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
pub type IntEvent0IclrMtxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMtxfifotrg>;
impl<'a, REG> IntEvent0IclrMtxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_mtxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMtxfifotrg::IntEvent0IclrMtxfifotrgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_mtxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMtxfifotrg::IntEvent0IclrMtxfifotrgClr)
    }
}
#[doc = "RXFIFO full event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMrxfifofull {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMrxfifofullNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMrxfifofullClr = 1,
}
impl From<IntEvent0IclrMrxfifofull> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMrxfifofull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MRXFIFOFULL` writer - RXFIFO full event."]
pub type IntEvent0IclrMrxfifofullW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMrxfifofull>;
impl<'a, REG> IntEvent0IclrMrxfifofullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_mrxfifofull_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMrxfifofull::IntEvent0IclrMrxfifofullNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_mrxfifofull_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMrxfifofull::IntEvent0IclrMrxfifofullClr)
    }
}
#[doc = "Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMtxempty {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMtxemptyNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMtxemptyClr = 1,
}
impl From<IntEvent0IclrMtxempty> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMtxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MTXEMPTY` writer - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
pub type IntEvent0IclrMtxemptyW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMtxempty>;
impl<'a, REG> IntEvent0IclrMtxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_mtxempty_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMtxempty::IntEvent0IclrMtxemptyNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_mtxempty_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMtxempty::IntEvent0IclrMtxemptyClr)
    }
}
#[doc = "Address/Data NACK Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMnack {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMnackNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMnackClr = 1,
}
impl From<IntEvent0IclrMnack> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMnack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MNACK` writer - Address/Data NACK Interrupt"]
pub type IntEvent0IclrMnackW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMnack>;
impl<'a, REG> IntEvent0IclrMnackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_mnack_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMnack::IntEvent0IclrMnackNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_mnack_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMnack::IntEvent0IclrMnackClr)
    }
}
#[doc = "START Detection Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMstart {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMstartNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMstartClr = 1,
}
impl From<IntEvent0IclrMstart> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MSTART` writer - START Detection Interrupt"]
pub type IntEvent0IclrMstartW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMstart>;
impl<'a, REG> IntEvent0IclrMstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_mstart_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMstart::IntEvent0IclrMstartNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_mstart_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMstart::IntEvent0IclrMstartClr)
    }
}
#[doc = "STOP Detection Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMstop {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMstopNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMstopClr = 1,
}
impl From<IntEvent0IclrMstop> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MSTOP` writer - STOP Detection Interrupt"]
pub type IntEvent0IclrMstopW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMstop>;
impl<'a, REG> IntEvent0IclrMstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_mstop_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMstop::IntEvent0IclrMstopNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_mstop_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMstop::IntEvent0IclrMstopClr)
    }
}
#[doc = "Arbitration Lost Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMarblost {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMarblostNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMarblostClr = 1,
}
impl From<IntEvent0IclrMarblost> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMarblost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MARBLOST` writer - Arbitration Lost Interrupt"]
pub type IntEvent0IclrMarblostW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMarblost>;
impl<'a, REG> IntEvent0IclrMarblostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_marblost_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMarblost::IntEvent0IclrMarblostNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_marblost_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMarblost::IntEvent0IclrMarblostClr)
    }
}
#[doc = "DMA Done 1 on Event Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMdmaDone1_2 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMdmaDone1_2NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMdmaDone1_2Clr = 1,
}
impl From<IntEvent0IclrMdmaDone1_2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMdmaDone1_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MDMA_DONE1_2` writer - DMA Done 1 on Event Channel 2"]
pub type IntEvent0IclrMdmaDone1_2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMdmaDone1_2>;
impl<'a, REG> IntEvent0IclrMdmaDone1_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_mdma_done1_2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMdmaDone1_2::IntEvent0IclrMdmaDone1_2NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_mdma_done1_2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMdmaDone1_2::IntEvent0IclrMdmaDone1_2Clr)
    }
}
#[doc = "DMA Done 1 on Event Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMdmaDone1_3 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMdmaDone1_3NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMdmaDone1_3Clr = 1,
}
impl From<IntEvent0IclrMdmaDone1_3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMdmaDone1_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MDMA_DONE1_3` writer - DMA Done 1 on Event Channel 3"]
pub type IntEvent0IclrMdmaDone1_3W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMdmaDone1_3>;
impl<'a, REG> IntEvent0IclrMdmaDone1_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_mdma_done1_3_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMdmaDone1_3::IntEvent0IclrMdmaDone1_3NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_mdma_done1_3_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMdmaDone1_3::IntEvent0IclrMdmaDone1_3Clr)
    }
}
#[doc = "Master RX Pec Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrMpecRxErr {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrMpecRxErrNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrMpecRxErrClr = 1,
}
impl From<IntEvent0IclrMpecRxErr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrMpecRxErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_MPEC_RX_ERR` writer - Master RX Pec Error Interrupt"]
pub type IntEvent0IclrMpecRxErrW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrMpecRxErr>;
impl<'a, REG> IntEvent0IclrMpecRxErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_mpec_rx_err_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMpecRxErr::IntEvent0IclrMpecRxErrNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_mpec_rx_err_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrMpecRxErr::IntEvent0IclrMpecRxErrClr)
    }
}
#[doc = "Timeout A interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrTimeouta {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrTimeoutaNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrTimeoutaClr = 1,
}
impl From<IntEvent0IclrTimeouta> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrTimeouta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_TIMEOUTA` writer - Timeout A interrupt"]
pub type IntEvent0IclrTimeoutaW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrTimeouta>;
impl<'a, REG> IntEvent0IclrTimeoutaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_timeouta_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrTimeouta::IntEvent0IclrTimeoutaNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_timeouta_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrTimeouta::IntEvent0IclrTimeoutaClr)
    }
}
#[doc = "Timeout B Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrTimeoutb {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrTimeoutbNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrTimeoutbClr = 1,
}
impl From<IntEvent0IclrTimeoutb> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrTimeoutb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_TIMEOUTB` writer - Timeout B Interrupt"]
pub type IntEvent0IclrTimeoutbW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrTimeoutb>;
impl<'a, REG> IntEvent0IclrTimeoutbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_timeoutb_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrTimeoutb::IntEvent0IclrTimeoutbNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_timeoutb_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrTimeoutb::IntEvent0IclrTimeoutbClr)
    }
}
#[doc = "Slave Receive Data Interrupt Signals that a byte has been received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrSrxdone {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrSrxdoneNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrSrxdoneClr = 1,
}
impl From<IntEvent0IclrSrxdone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrSrxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_SRXDONE` writer - Slave Receive Data Interrupt Signals that a byte has been received"]
pub type IntEvent0IclrSrxdoneW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrSrxdone>;
impl<'a, REG> IntEvent0IclrSrxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_srxdone_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSrxdone::IntEvent0IclrSrxdoneNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_srxdone_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSrxdone::IntEvent0IclrSrxdoneClr)
    }
}
#[doc = "Slave Transmit Transaction completed Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrStxdone {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrStxdoneNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrStxdoneClr = 1,
}
impl From<IntEvent0IclrStxdone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrStxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_STXDONE` writer - Slave Transmit Transaction completed Interrupt"]
pub type IntEvent0IclrStxdoneW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrStxdone>;
impl<'a, REG> IntEvent0IclrStxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_stxdone_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrStxdone::IntEvent0IclrStxdoneNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_stxdone_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrStxdone::IntEvent0IclrStxdoneClr)
    }
}
#[doc = "Slave Receive FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrSrxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrSrxfifotrgNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrSrxfifotrgClr = 1,
}
impl From<IntEvent0IclrSrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrSrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_SRXFIFOTRG` writer - Slave Receive FIFO Trigger"]
pub type IntEvent0IclrSrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrSrxfifotrg>;
impl<'a, REG> IntEvent0IclrSrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_srxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSrxfifotrg::IntEvent0IclrSrxfifotrgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_srxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSrxfifotrg::IntEvent0IclrSrxfifotrgClr)
    }
}
#[doc = "Slave Transmit FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrStxfifotrg {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrStxfifotrgNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrStxfifotrgClr = 1,
}
impl From<IntEvent0IclrStxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrStxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_STXFIFOTRG` writer - Slave Transmit FIFO Trigger"]
pub type IntEvent0IclrStxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrStxfifotrg>;
impl<'a, REG> IntEvent0IclrStxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_stxfifotrg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrStxfifotrg::IntEvent0IclrStxfifotrgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_stxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrStxfifotrg::IntEvent0IclrStxfifotrgClr)
    }
}
#[doc = "RXFIFO full event. This interrupt is set if an RX FIFO is full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrSrxfifofull {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrSrxfifofullNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrSrxfifofullClr = 1,
}
impl From<IntEvent0IclrSrxfifofull> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrSrxfifofull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_SRXFIFOFULL` writer - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
pub type IntEvent0IclrSrxfifofullW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrSrxfifofull>;
impl<'a, REG> IntEvent0IclrSrxfifofullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_srxfifofull_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSrxfifofull::IntEvent0IclrSrxfifofullNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_srxfifofull_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSrxfifofull::IntEvent0IclrSrxfifofullClr)
    }
}
#[doc = "Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrStxempty {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrStxemptyNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrStxemptyClr = 1,
}
impl From<IntEvent0IclrStxempty> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrStxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_STXEMPTY` writer - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
pub type IntEvent0IclrStxemptyW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrStxempty>;
impl<'a, REG> IntEvent0IclrStxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_stxempty_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrStxempty::IntEvent0IclrStxemptyNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_stxempty_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrStxempty::IntEvent0IclrStxemptyClr)
    }
}
#[doc = "Slave START Detection Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrSstart {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrSstartNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrSstartClr = 1,
}
impl From<IntEvent0IclrSstart> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrSstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_SSTART` writer - Slave START Detection Interrupt"]
pub type IntEvent0IclrSstartW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrSstart>;
impl<'a, REG> IntEvent0IclrSstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_sstart_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSstart::IntEvent0IclrSstartNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_sstart_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSstart::IntEvent0IclrSstartClr)
    }
}
#[doc = "Slave STOP Detection Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrSstop {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrSstopNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrSstopClr = 1,
}
impl From<IntEvent0IclrSstop> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrSstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_SSTOP` writer - Slave STOP Detection Interrupt"]
pub type IntEvent0IclrSstopW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrSstop>;
impl<'a, REG> IntEvent0IclrSstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_sstop_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSstop::IntEvent0IclrSstopNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_sstop_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSstop::IntEvent0IclrSstopClr)
    }
}
#[doc = "General Call Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrSgencall {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrSgencallNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrSgencallClr = 1,
}
impl From<IntEvent0IclrSgencall> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrSgencall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_SGENCALL` writer - General Call Interrupt"]
pub type IntEvent0IclrSgencallW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrSgencall>;
impl<'a, REG> IntEvent0IclrSgencallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_sgencall_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSgencall::IntEvent0IclrSgencallNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_sgencall_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSgencall::IntEvent0IclrSgencallClr)
    }
}
#[doc = "DMA Done 1 on Event Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrSdmaDone1_2 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrSdmaDone1_2NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrSdmaDone1_2Clr = 1,
}
impl From<IntEvent0IclrSdmaDone1_2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrSdmaDone1_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_SDMA_DONE1_2` writer - DMA Done 1 on Event Channel 2"]
pub type IntEvent0IclrSdmaDone1_2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrSdmaDone1_2>;
impl<'a, REG> IntEvent0IclrSdmaDone1_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_sdma_done1_2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSdmaDone1_2::IntEvent0IclrSdmaDone1_2NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_sdma_done1_2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSdmaDone1_2::IntEvent0IclrSdmaDone1_2Clr)
    }
}
#[doc = "DMA Done 1 on Event Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrSdmaDone1_3 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrSdmaDone1_3NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrSdmaDone1_3Clr = 1,
}
impl From<IntEvent0IclrSdmaDone1_3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrSdmaDone1_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_SDMA_DONE1_3` writer - DMA Done 1 on Event Channel 3"]
pub type IntEvent0IclrSdmaDone1_3W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrSdmaDone1_3>;
impl<'a, REG> IntEvent0IclrSdmaDone1_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_sdma_done1_3_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSdmaDone1_3::IntEvent0IclrSdmaDone1_3NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_sdma_done1_3_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSdmaDone1_3::IntEvent0IclrSdmaDone1_3Clr)
    }
}
#[doc = "Slave RX Pec Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrSpecRxErr {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrSpecRxErrNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrSpecRxErrClr = 1,
}
impl From<IntEvent0IclrSpecRxErr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrSpecRxErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_SPEC_RX_ERR` writer - Slave RX Pec Error Interrupt"]
pub type IntEvent0IclrSpecRxErrW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrSpecRxErr>;
impl<'a, REG> IntEvent0IclrSpecRxErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_spec_rx_err_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSpecRxErr::IntEvent0IclrSpecRxErrNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_spec_rx_err_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSpecRxErr::IntEvent0IclrSpecRxErrClr)
    }
}
#[doc = "Slave TX FIFO underflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrStxUnfl {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrStxUnflNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrStxUnflClr = 1,
}
impl From<IntEvent0IclrStxUnfl> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrStxUnfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_STX_UNFL` writer - Slave TX FIFO underflow"]
pub type IntEvent0IclrStxUnflW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrStxUnfl>;
impl<'a, REG> IntEvent0IclrStxUnflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_stx_unfl_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrStxUnfl::IntEvent0IclrStxUnflNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_stx_unfl_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrStxUnfl::IntEvent0IclrStxUnflClr)
    }
}
#[doc = "Slave RX FIFO overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrSrxOvfl {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrSrxOvflNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrSrxOvflClr = 1,
}
impl From<IntEvent0IclrSrxOvfl> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrSrxOvfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_SRX_OVFL` writer - Slave RX FIFO overflow"]
pub type IntEvent0IclrSrxOvflW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrSrxOvfl>;
impl<'a, REG> IntEvent0IclrSrxOvflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_srx_ovfl_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSrxOvfl::IntEvent0IclrSrxOvflNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_srx_ovfl_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSrxOvfl::IntEvent0IclrSrxOvflClr)
    }
}
#[doc = "Slave Arbitration Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrSarblost {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrSarblostNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrSarblostClr = 1,
}
impl From<IntEvent0IclrSarblost> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrSarblost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_SARBLOST` writer - Slave Arbitration Lost"]
pub type IntEvent0IclrSarblostW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrSarblost>;
impl<'a, REG> IntEvent0IclrSarblostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_sarblost_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSarblost::IntEvent0IclrSarblostNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_sarblost_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrSarblost::IntEvent0IclrSarblostClr)
    }
}
#[doc = "Interrupt overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrIntrOvfl {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrIntrOvflNoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrIntrOvflClr = 1,
}
impl From<IntEvent0IclrIntrOvfl> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrIntrOvfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_INTR_OVFL` writer - Interrupt overflow"]
pub type IntEvent0IclrIntrOvflW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrIntrOvfl>;
impl<'a, REG> IntEvent0IclrIntrOvflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_intr_ovfl_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrIntrOvfl::IntEvent0IclrIntrOvflNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_intr_ovfl_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrIntrOvfl::IntEvent0IclrIntrOvflClr)
    }
}
impl W {
    #[doc = "Bit 0 - Master Receive Data Interrupt Signals that a byte has been received"]
    #[inline(always)]
    pub fn int_event0_iclr_mrxdone(&mut self) -> IntEvent0IclrMrxdoneW<IntEvent0IclrSpec> {
        IntEvent0IclrMrxdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Master Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn int_event0_iclr_mtxdone(&mut self) -> IntEvent0IclrMtxdoneW<IntEvent0IclrSpec> {
        IntEvent0IclrMtxdoneW::new(self, 1)
    }
    #[doc = "Bit 2 - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
    #[inline(always)]
    pub fn int_event0_iclr_mrxfifotrg(&mut self) -> IntEvent0IclrMrxfifotrgW<IntEvent0IclrSpec> {
        IntEvent0IclrMrxfifotrgW::new(self, 2)
    }
    #[doc = "Bit 3 - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
    #[inline(always)]
    pub fn int_event0_iclr_mtxfifotrg(&mut self) -> IntEvent0IclrMtxfifotrgW<IntEvent0IclrSpec> {
        IntEvent0IclrMtxfifotrgW::new(self, 3)
    }
    #[doc = "Bit 4 - RXFIFO full event."]
    #[inline(always)]
    pub fn int_event0_iclr_mrxfifofull(&mut self) -> IntEvent0IclrMrxfifofullW<IntEvent0IclrSpec> {
        IntEvent0IclrMrxfifofullW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
    #[inline(always)]
    pub fn int_event0_iclr_mtxempty(&mut self) -> IntEvent0IclrMtxemptyW<IntEvent0IclrSpec> {
        IntEvent0IclrMtxemptyW::new(self, 5)
    }
    #[doc = "Bit 7 - Address/Data NACK Interrupt"]
    #[inline(always)]
    pub fn int_event0_iclr_mnack(&mut self) -> IntEvent0IclrMnackW<IntEvent0IclrSpec> {
        IntEvent0IclrMnackW::new(self, 7)
    }
    #[doc = "Bit 8 - START Detection Interrupt"]
    #[inline(always)]
    pub fn int_event0_iclr_mstart(&mut self) -> IntEvent0IclrMstartW<IntEvent0IclrSpec> {
        IntEvent0IclrMstartW::new(self, 8)
    }
    #[doc = "Bit 9 - STOP Detection Interrupt"]
    #[inline(always)]
    pub fn int_event0_iclr_mstop(&mut self) -> IntEvent0IclrMstopW<IntEvent0IclrSpec> {
        IntEvent0IclrMstopW::new(self, 9)
    }
    #[doc = "Bit 10 - Arbitration Lost Interrupt"]
    #[inline(always)]
    pub fn int_event0_iclr_marblost(&mut self) -> IntEvent0IclrMarblostW<IntEvent0IclrSpec> {
        IntEvent0IclrMarblostW::new(self, 10)
    }
    #[doc = "Bit 11 - DMA Done 1 on Event Channel 2"]
    #[inline(always)]
    pub fn int_event0_iclr_mdma_done1_2(&mut self) -> IntEvent0IclrMdmaDone1_2W<IntEvent0IclrSpec> {
        IntEvent0IclrMdmaDone1_2W::new(self, 11)
    }
    #[doc = "Bit 12 - DMA Done 1 on Event Channel 3"]
    #[inline(always)]
    pub fn int_event0_iclr_mdma_done1_3(&mut self) -> IntEvent0IclrMdmaDone1_3W<IntEvent0IclrSpec> {
        IntEvent0IclrMdmaDone1_3W::new(self, 12)
    }
    #[doc = "Bit 13 - Master RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn int_event0_iclr_mpec_rx_err(&mut self) -> IntEvent0IclrMpecRxErrW<IntEvent0IclrSpec> {
        IntEvent0IclrMpecRxErrW::new(self, 13)
    }
    #[doc = "Bit 14 - Timeout A interrupt"]
    #[inline(always)]
    pub fn int_event0_iclr_timeouta(&mut self) -> IntEvent0IclrTimeoutaW<IntEvent0IclrSpec> {
        IntEvent0IclrTimeoutaW::new(self, 14)
    }
    #[doc = "Bit 15 - Timeout B Interrupt"]
    #[inline(always)]
    pub fn int_event0_iclr_timeoutb(&mut self) -> IntEvent0IclrTimeoutbW<IntEvent0IclrSpec> {
        IntEvent0IclrTimeoutbW::new(self, 15)
    }
    #[doc = "Bit 16 - Slave Receive Data Interrupt Signals that a byte has been received"]
    #[inline(always)]
    pub fn int_event0_iclr_srxdone(&mut self) -> IntEvent0IclrSrxdoneW<IntEvent0IclrSpec> {
        IntEvent0IclrSrxdoneW::new(self, 16)
    }
    #[doc = "Bit 17 - Slave Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn int_event0_iclr_stxdone(&mut self) -> IntEvent0IclrStxdoneW<IntEvent0IclrSpec> {
        IntEvent0IclrStxdoneW::new(self, 17)
    }
    #[doc = "Bit 18 - Slave Receive FIFO Trigger"]
    #[inline(always)]
    pub fn int_event0_iclr_srxfifotrg(&mut self) -> IntEvent0IclrSrxfifotrgW<IntEvent0IclrSpec> {
        IntEvent0IclrSrxfifotrgW::new(self, 18)
    }
    #[doc = "Bit 19 - Slave Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn int_event0_iclr_stxfifotrg(&mut self) -> IntEvent0IclrStxfifotrgW<IntEvent0IclrSpec> {
        IntEvent0IclrStxfifotrgW::new(self, 19)
    }
    #[doc = "Bit 20 - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
    #[inline(always)]
    pub fn int_event0_iclr_srxfifofull(&mut self) -> IntEvent0IclrSrxfifofullW<IntEvent0IclrSpec> {
        IntEvent0IclrSrxfifofullW::new(self, 20)
    }
    #[doc = "Bit 21 - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
    #[inline(always)]
    pub fn int_event0_iclr_stxempty(&mut self) -> IntEvent0IclrStxemptyW<IntEvent0IclrSpec> {
        IntEvent0IclrStxemptyW::new(self, 21)
    }
    #[doc = "Bit 22 - Slave START Detection Interrupt"]
    #[inline(always)]
    pub fn int_event0_iclr_sstart(&mut self) -> IntEvent0IclrSstartW<IntEvent0IclrSpec> {
        IntEvent0IclrSstartW::new(self, 22)
    }
    #[doc = "Bit 23 - Slave STOP Detection Interrupt"]
    #[inline(always)]
    pub fn int_event0_iclr_sstop(&mut self) -> IntEvent0IclrSstopW<IntEvent0IclrSpec> {
        IntEvent0IclrSstopW::new(self, 23)
    }
    #[doc = "Bit 24 - General Call Interrupt"]
    #[inline(always)]
    pub fn int_event0_iclr_sgencall(&mut self) -> IntEvent0IclrSgencallW<IntEvent0IclrSpec> {
        IntEvent0IclrSgencallW::new(self, 24)
    }
    #[doc = "Bit 25 - DMA Done 1 on Event Channel 2"]
    #[inline(always)]
    pub fn int_event0_iclr_sdma_done1_2(&mut self) -> IntEvent0IclrSdmaDone1_2W<IntEvent0IclrSpec> {
        IntEvent0IclrSdmaDone1_2W::new(self, 25)
    }
    #[doc = "Bit 26 - DMA Done 1 on Event Channel 3"]
    #[inline(always)]
    pub fn int_event0_iclr_sdma_done1_3(&mut self) -> IntEvent0IclrSdmaDone1_3W<IntEvent0IclrSpec> {
        IntEvent0IclrSdmaDone1_3W::new(self, 26)
    }
    #[doc = "Bit 27 - Slave RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn int_event0_iclr_spec_rx_err(&mut self) -> IntEvent0IclrSpecRxErrW<IntEvent0IclrSpec> {
        IntEvent0IclrSpecRxErrW::new(self, 27)
    }
    #[doc = "Bit 28 - Slave TX FIFO underflow"]
    #[inline(always)]
    pub fn int_event0_iclr_stx_unfl(&mut self) -> IntEvent0IclrStxUnflW<IntEvent0IclrSpec> {
        IntEvent0IclrStxUnflW::new(self, 28)
    }
    #[doc = "Bit 29 - Slave RX FIFO overflow"]
    #[inline(always)]
    pub fn int_event0_iclr_srx_ovfl(&mut self) -> IntEvent0IclrSrxOvflW<IntEvent0IclrSpec> {
        IntEvent0IclrSrxOvflW::new(self, 29)
    }
    #[doc = "Bit 30 - Slave Arbitration Lost"]
    #[inline(always)]
    pub fn int_event0_iclr_sarblost(&mut self) -> IntEvent0IclrSarblostW<IntEvent0IclrSpec> {
        IntEvent0IclrSarblostW::new(self, 30)
    }
    #[doc = "Bit 31 - Interrupt overflow"]
    #[inline(always)]
    pub fn int_event0_iclr_intr_ovfl(&mut self) -> IntEvent0IclrIntrOvflW<IntEvent0IclrSpec> {
        IntEvent0IclrIntrOvflW::new(self, 31)
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
