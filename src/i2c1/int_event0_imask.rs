#[doc = "Register `INT_EVENT0_IMASK` reader"]
pub type R = crate::R<IntEvent0ImaskSpec>;
#[doc = "Register `INT_EVENT0_IMASK` writer"]
pub type W = crate::W<IntEvent0ImaskSpec>;
#[doc = "Master Receive Transaction completed Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMrxdone {
    #[doc = "0: CLR"]
    IntEvent0ImaskMrxdoneClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMrxdoneSet = 1,
}
impl From<IntEvent0ImaskMrxdone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMrxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MRXDONE` reader - Master Receive Transaction completed Interrupt"]
pub type IntEvent0ImaskMrxdoneR = crate::BitReader<IntEvent0ImaskMrxdone>;
impl IntEvent0ImaskMrxdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMrxdone {
        match self.bits {
            false => IntEvent0ImaskMrxdone::IntEvent0ImaskMrxdoneClr,
            true => IntEvent0ImaskMrxdone::IntEvent0ImaskMrxdoneSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_mrxdone_clr(&self) -> bool {
        *self == IntEvent0ImaskMrxdone::IntEvent0ImaskMrxdoneClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_mrxdone_set(&self) -> bool {
        *self == IntEvent0ImaskMrxdone::IntEvent0ImaskMrxdoneSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MRXDONE` writer - Master Receive Transaction completed Interrupt"]
pub type IntEvent0ImaskMrxdoneW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMrxdone>;
impl<'a, REG> IntEvent0ImaskMrxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_mrxdone_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMrxdone::IntEvent0ImaskMrxdoneClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_mrxdone_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMrxdone::IntEvent0ImaskMrxdoneSet)
    }
}
#[doc = "Master Transmit Transaction completed Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMtxdone {
    #[doc = "0: CLR"]
    IntEvent0ImaskMtxdoneClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMtxdoneSet = 1,
}
impl From<IntEvent0ImaskMtxdone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMtxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MTXDONE` reader - Master Transmit Transaction completed Interrupt"]
pub type IntEvent0ImaskMtxdoneR = crate::BitReader<IntEvent0ImaskMtxdone>;
impl IntEvent0ImaskMtxdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMtxdone {
        match self.bits {
            false => IntEvent0ImaskMtxdone::IntEvent0ImaskMtxdoneClr,
            true => IntEvent0ImaskMtxdone::IntEvent0ImaskMtxdoneSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_mtxdone_clr(&self) -> bool {
        *self == IntEvent0ImaskMtxdone::IntEvent0ImaskMtxdoneClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_mtxdone_set(&self) -> bool {
        *self == IntEvent0ImaskMtxdone::IntEvent0ImaskMtxdoneSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MTXDONE` writer - Master Transmit Transaction completed Interrupt"]
pub type IntEvent0ImaskMtxdoneW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMtxdone>;
impl<'a, REG> IntEvent0ImaskMtxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_mtxdone_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMtxdone::IntEvent0ImaskMtxdoneClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_mtxdone_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMtxdone::IntEvent0ImaskMtxdoneSet)
    }
}
#[doc = "Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMrxfifotrg {
    #[doc = "0: CLR"]
    IntEvent0ImaskMrxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMrxfifotrgSet = 1,
}
impl From<IntEvent0ImaskMrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MRXFIFOTRG` reader - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
pub type IntEvent0ImaskMrxfifotrgR = crate::BitReader<IntEvent0ImaskMrxfifotrg>;
impl IntEvent0ImaskMrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMrxfifotrg {
        match self.bits {
            false => IntEvent0ImaskMrxfifotrg::IntEvent0ImaskMrxfifotrgClr,
            true => IntEvent0ImaskMrxfifotrg::IntEvent0ImaskMrxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_mrxfifotrg_clr(&self) -> bool {
        *self == IntEvent0ImaskMrxfifotrg::IntEvent0ImaskMrxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_mrxfifotrg_set(&self) -> bool {
        *self == IntEvent0ImaskMrxfifotrg::IntEvent0ImaskMrxfifotrgSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MRXFIFOTRG` writer - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
pub type IntEvent0ImaskMrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMrxfifotrg>;
impl<'a, REG> IntEvent0ImaskMrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_mrxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMrxfifotrg::IntEvent0ImaskMrxfifotrgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_mrxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMrxfifotrg::IntEvent0ImaskMrxfifotrgSet)
    }
}
#[doc = "Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMtxfifotrg {
    #[doc = "0: CLR"]
    IntEvent0ImaskMtxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMtxfifotrgSet = 1,
}
impl From<IntEvent0ImaskMtxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMtxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MTXFIFOTRG` reader - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
pub type IntEvent0ImaskMtxfifotrgR = crate::BitReader<IntEvent0ImaskMtxfifotrg>;
impl IntEvent0ImaskMtxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMtxfifotrg {
        match self.bits {
            false => IntEvent0ImaskMtxfifotrg::IntEvent0ImaskMtxfifotrgClr,
            true => IntEvent0ImaskMtxfifotrg::IntEvent0ImaskMtxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_mtxfifotrg_clr(&self) -> bool {
        *self == IntEvent0ImaskMtxfifotrg::IntEvent0ImaskMtxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_mtxfifotrg_set(&self) -> bool {
        *self == IntEvent0ImaskMtxfifotrg::IntEvent0ImaskMtxfifotrgSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MTXFIFOTRG` writer - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
pub type IntEvent0ImaskMtxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMtxfifotrg>;
impl<'a, REG> IntEvent0ImaskMtxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_mtxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMtxfifotrg::IntEvent0ImaskMtxfifotrgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_mtxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMtxfifotrg::IntEvent0ImaskMtxfifotrgSet)
    }
}
#[doc = "RXFIFO full event. This interrupt is set if an RX FIFO is full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMrxfifofull {
    #[doc = "0: CLR"]
    IntEvent0ImaskMrxfifofullClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMrxfifofullSet = 1,
}
impl From<IntEvent0ImaskMrxfifofull> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMrxfifofull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MRXFIFOFULL` reader - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
pub type IntEvent0ImaskMrxfifofullR = crate::BitReader<IntEvent0ImaskMrxfifofull>;
impl IntEvent0ImaskMrxfifofullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMrxfifofull {
        match self.bits {
            false => IntEvent0ImaskMrxfifofull::IntEvent0ImaskMrxfifofullClr,
            true => IntEvent0ImaskMrxfifofull::IntEvent0ImaskMrxfifofullSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_mrxfifofull_clr(&self) -> bool {
        *self == IntEvent0ImaskMrxfifofull::IntEvent0ImaskMrxfifofullClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_mrxfifofull_set(&self) -> bool {
        *self == IntEvent0ImaskMrxfifofull::IntEvent0ImaskMrxfifofullSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MRXFIFOFULL` writer - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
pub type IntEvent0ImaskMrxfifofullW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMrxfifofull>;
impl<'a, REG> IntEvent0ImaskMrxfifofullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_mrxfifofull_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMrxfifofull::IntEvent0ImaskMrxfifofullClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_mrxfifofull_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMrxfifofull::IntEvent0ImaskMrxfifofullSet)
    }
}
#[doc = "Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMtxempty {
    #[doc = "0: CLR"]
    IntEvent0ImaskMtxemptyClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMtxemptySet = 1,
}
impl From<IntEvent0ImaskMtxempty> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMtxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MTXEMPTY` reader - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
pub type IntEvent0ImaskMtxemptyR = crate::BitReader<IntEvent0ImaskMtxempty>;
impl IntEvent0ImaskMtxemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMtxempty {
        match self.bits {
            false => IntEvent0ImaskMtxempty::IntEvent0ImaskMtxemptyClr,
            true => IntEvent0ImaskMtxempty::IntEvent0ImaskMtxemptySet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_mtxempty_clr(&self) -> bool {
        *self == IntEvent0ImaskMtxempty::IntEvent0ImaskMtxemptyClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_mtxempty_set(&self) -> bool {
        *self == IntEvent0ImaskMtxempty::IntEvent0ImaskMtxemptySet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MTXEMPTY` writer - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
pub type IntEvent0ImaskMtxemptyW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMtxempty>;
impl<'a, REG> IntEvent0ImaskMtxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_mtxempty_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMtxempty::IntEvent0ImaskMtxemptyClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_mtxempty_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMtxempty::IntEvent0ImaskMtxemptySet)
    }
}
#[doc = "Address/Data NACK Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMnack {
    #[doc = "0: CLR"]
    IntEvent0ImaskMnackClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMnackSet = 1,
}
impl From<IntEvent0ImaskMnack> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMnack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MNACK` reader - Address/Data NACK Interrupt"]
pub type IntEvent0ImaskMnackR = crate::BitReader<IntEvent0ImaskMnack>;
impl IntEvent0ImaskMnackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMnack {
        match self.bits {
            false => IntEvent0ImaskMnack::IntEvent0ImaskMnackClr,
            true => IntEvent0ImaskMnack::IntEvent0ImaskMnackSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_mnack_clr(&self) -> bool {
        *self == IntEvent0ImaskMnack::IntEvent0ImaskMnackClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_mnack_set(&self) -> bool {
        *self == IntEvent0ImaskMnack::IntEvent0ImaskMnackSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MNACK` writer - Address/Data NACK Interrupt"]
pub type IntEvent0ImaskMnackW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMnack>;
impl<'a, REG> IntEvent0ImaskMnackW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_mnack_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMnack::IntEvent0ImaskMnackClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_mnack_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMnack::IntEvent0ImaskMnackSet)
    }
}
#[doc = "START Detection Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMstart {
    #[doc = "0: CLR"]
    IntEvent0ImaskMstartClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMstartSet = 1,
}
impl From<IntEvent0ImaskMstart> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MSTART` reader - START Detection Interrupt"]
pub type IntEvent0ImaskMstartR = crate::BitReader<IntEvent0ImaskMstart>;
impl IntEvent0ImaskMstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMstart {
        match self.bits {
            false => IntEvent0ImaskMstart::IntEvent0ImaskMstartClr,
            true => IntEvent0ImaskMstart::IntEvent0ImaskMstartSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_mstart_clr(&self) -> bool {
        *self == IntEvent0ImaskMstart::IntEvent0ImaskMstartClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_mstart_set(&self) -> bool {
        *self == IntEvent0ImaskMstart::IntEvent0ImaskMstartSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MSTART` writer - START Detection Interrupt"]
pub type IntEvent0ImaskMstartW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMstart>;
impl<'a, REG> IntEvent0ImaskMstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_mstart_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMstart::IntEvent0ImaskMstartClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_mstart_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMstart::IntEvent0ImaskMstartSet)
    }
}
#[doc = "STOP Detection Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMstop {
    #[doc = "0: CLR"]
    IntEvent0ImaskMstopClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMstopSet = 1,
}
impl From<IntEvent0ImaskMstop> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MSTOP` reader - STOP Detection Interrupt"]
pub type IntEvent0ImaskMstopR = crate::BitReader<IntEvent0ImaskMstop>;
impl IntEvent0ImaskMstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMstop {
        match self.bits {
            false => IntEvent0ImaskMstop::IntEvent0ImaskMstopClr,
            true => IntEvent0ImaskMstop::IntEvent0ImaskMstopSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_mstop_clr(&self) -> bool {
        *self == IntEvent0ImaskMstop::IntEvent0ImaskMstopClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_mstop_set(&self) -> bool {
        *self == IntEvent0ImaskMstop::IntEvent0ImaskMstopSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MSTOP` writer - STOP Detection Interrupt"]
pub type IntEvent0ImaskMstopW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMstop>;
impl<'a, REG> IntEvent0ImaskMstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_mstop_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMstop::IntEvent0ImaskMstopClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_mstop_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMstop::IntEvent0ImaskMstopSet)
    }
}
#[doc = "Arbitration Lost Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMarblost {
    #[doc = "0: CLR"]
    IntEvent0ImaskMarblostClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMarblostSet = 1,
}
impl From<IntEvent0ImaskMarblost> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMarblost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MARBLOST` reader - Arbitration Lost Interrupt"]
pub type IntEvent0ImaskMarblostR = crate::BitReader<IntEvent0ImaskMarblost>;
impl IntEvent0ImaskMarblostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMarblost {
        match self.bits {
            false => IntEvent0ImaskMarblost::IntEvent0ImaskMarblostClr,
            true => IntEvent0ImaskMarblost::IntEvent0ImaskMarblostSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_marblost_clr(&self) -> bool {
        *self == IntEvent0ImaskMarblost::IntEvent0ImaskMarblostClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_marblost_set(&self) -> bool {
        *self == IntEvent0ImaskMarblost::IntEvent0ImaskMarblostSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MARBLOST` writer - Arbitration Lost Interrupt"]
pub type IntEvent0ImaskMarblostW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMarblost>;
impl<'a, REG> IntEvent0ImaskMarblostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_marblost_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMarblost::IntEvent0ImaskMarblostClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_marblost_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMarblost::IntEvent0ImaskMarblostSet)
    }
}
#[doc = "DMA Done 1 on Event Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMdmaDone1_2 {
    #[doc = "0: CLR"]
    IntEvent0ImaskMdmaDone1_2Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMdmaDone1_2Set = 1,
}
impl From<IntEvent0ImaskMdmaDone1_2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMdmaDone1_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MDMA_DONE1_2` reader - DMA Done 1 on Event Channel 2"]
pub type IntEvent0ImaskMdmaDone1_2R = crate::BitReader<IntEvent0ImaskMdmaDone1_2>;
impl IntEvent0ImaskMdmaDone1_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMdmaDone1_2 {
        match self.bits {
            false => IntEvent0ImaskMdmaDone1_2::IntEvent0ImaskMdmaDone1_2Clr,
            true => IntEvent0ImaskMdmaDone1_2::IntEvent0ImaskMdmaDone1_2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_mdma_done1_2_clr(&self) -> bool {
        *self == IntEvent0ImaskMdmaDone1_2::IntEvent0ImaskMdmaDone1_2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_mdma_done1_2_set(&self) -> bool {
        *self == IntEvent0ImaskMdmaDone1_2::IntEvent0ImaskMdmaDone1_2Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MDMA_DONE1_2` writer - DMA Done 1 on Event Channel 2"]
pub type IntEvent0ImaskMdmaDone1_2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMdmaDone1_2>;
impl<'a, REG> IntEvent0ImaskMdmaDone1_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_mdma_done1_2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMdmaDone1_2::IntEvent0ImaskMdmaDone1_2Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_mdma_done1_2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMdmaDone1_2::IntEvent0ImaskMdmaDone1_2Set)
    }
}
#[doc = "DMA Done 1 on Event Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMdmaDone1_3 {
    #[doc = "0: CLR"]
    IntEvent0ImaskMdmaDone1_3Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMdmaDone1_3Set = 1,
}
impl From<IntEvent0ImaskMdmaDone1_3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMdmaDone1_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MDMA_DONE1_3` reader - DMA Done 1 on Event Channel 3"]
pub type IntEvent0ImaskMdmaDone1_3R = crate::BitReader<IntEvent0ImaskMdmaDone1_3>;
impl IntEvent0ImaskMdmaDone1_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMdmaDone1_3 {
        match self.bits {
            false => IntEvent0ImaskMdmaDone1_3::IntEvent0ImaskMdmaDone1_3Clr,
            true => IntEvent0ImaskMdmaDone1_3::IntEvent0ImaskMdmaDone1_3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_mdma_done1_3_clr(&self) -> bool {
        *self == IntEvent0ImaskMdmaDone1_3::IntEvent0ImaskMdmaDone1_3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_mdma_done1_3_set(&self) -> bool {
        *self == IntEvent0ImaskMdmaDone1_3::IntEvent0ImaskMdmaDone1_3Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MDMA_DONE1_3` writer - DMA Done 1 on Event Channel 3"]
pub type IntEvent0ImaskMdmaDone1_3W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMdmaDone1_3>;
impl<'a, REG> IntEvent0ImaskMdmaDone1_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_mdma_done1_3_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMdmaDone1_3::IntEvent0ImaskMdmaDone1_3Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_mdma_done1_3_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMdmaDone1_3::IntEvent0ImaskMdmaDone1_3Set)
    }
}
#[doc = "Master RX Pec Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskMpecRxErr {
    #[doc = "0: CLR"]
    IntEvent0ImaskMpecRxErrClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskMpecRxErrSet = 1,
}
impl From<IntEvent0ImaskMpecRxErr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskMpecRxErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MPEC_RX_ERR` reader - Master RX Pec Error Interrupt"]
pub type IntEvent0ImaskMpecRxErrR = crate::BitReader<IntEvent0ImaskMpecRxErr>;
impl IntEvent0ImaskMpecRxErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskMpecRxErr {
        match self.bits {
            false => IntEvent0ImaskMpecRxErr::IntEvent0ImaskMpecRxErrClr,
            true => IntEvent0ImaskMpecRxErr::IntEvent0ImaskMpecRxErrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_mpec_rx_err_clr(&self) -> bool {
        *self == IntEvent0ImaskMpecRxErr::IntEvent0ImaskMpecRxErrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_mpec_rx_err_set(&self) -> bool {
        *self == IntEvent0ImaskMpecRxErr::IntEvent0ImaskMpecRxErrSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_MPEC_RX_ERR` writer - Master RX Pec Error Interrupt"]
pub type IntEvent0ImaskMpecRxErrW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskMpecRxErr>;
impl<'a, REG> IntEvent0ImaskMpecRxErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_mpec_rx_err_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMpecRxErr::IntEvent0ImaskMpecRxErrClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_mpec_rx_err_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskMpecRxErr::IntEvent0ImaskMpecRxErrSet)
    }
}
#[doc = "Timeout A Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskTimeouta {
    #[doc = "0: CLR"]
    IntEvent0ImaskTimeoutaClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskTimeoutaSet = 1,
}
impl From<IntEvent0ImaskTimeouta> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskTimeouta) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_TIMEOUTA` reader - Timeout A Interrupt"]
pub type IntEvent0ImaskTimeoutaR = crate::BitReader<IntEvent0ImaskTimeouta>;
impl IntEvent0ImaskTimeoutaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskTimeouta {
        match self.bits {
            false => IntEvent0ImaskTimeouta::IntEvent0ImaskTimeoutaClr,
            true => IntEvent0ImaskTimeouta::IntEvent0ImaskTimeoutaSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_timeouta_clr(&self) -> bool {
        *self == IntEvent0ImaskTimeouta::IntEvent0ImaskTimeoutaClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_timeouta_set(&self) -> bool {
        *self == IntEvent0ImaskTimeouta::IntEvent0ImaskTimeoutaSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_TIMEOUTA` writer - Timeout A Interrupt"]
pub type IntEvent0ImaskTimeoutaW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskTimeouta>;
impl<'a, REG> IntEvent0ImaskTimeoutaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_timeouta_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskTimeouta::IntEvent0ImaskTimeoutaClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_timeouta_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskTimeouta::IntEvent0ImaskTimeoutaSet)
    }
}
#[doc = "Timeout B Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskTimeoutb {
    #[doc = "0: CLR"]
    IntEvent0ImaskTimeoutbClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskTimeoutbSet = 1,
}
impl From<IntEvent0ImaskTimeoutb> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskTimeoutb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_TIMEOUTB` reader - Timeout B Interrupt"]
pub type IntEvent0ImaskTimeoutbR = crate::BitReader<IntEvent0ImaskTimeoutb>;
impl IntEvent0ImaskTimeoutbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskTimeoutb {
        match self.bits {
            false => IntEvent0ImaskTimeoutb::IntEvent0ImaskTimeoutbClr,
            true => IntEvent0ImaskTimeoutb::IntEvent0ImaskTimeoutbSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_timeoutb_clr(&self) -> bool {
        *self == IntEvent0ImaskTimeoutb::IntEvent0ImaskTimeoutbClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_timeoutb_set(&self) -> bool {
        *self == IntEvent0ImaskTimeoutb::IntEvent0ImaskTimeoutbSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_TIMEOUTB` writer - Timeout B Interrupt"]
pub type IntEvent0ImaskTimeoutbW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskTimeoutb>;
impl<'a, REG> IntEvent0ImaskTimeoutbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_timeoutb_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskTimeoutb::IntEvent0ImaskTimeoutbClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_timeoutb_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskTimeoutb::IntEvent0ImaskTimeoutbSet)
    }
}
#[doc = "Slave Receive Data Interrupt Signals that a byte has been received\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskSrxdone {
    #[doc = "0: CLR"]
    IntEvent0ImaskSrxdoneClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskSrxdoneSet = 1,
}
impl From<IntEvent0ImaskSrxdone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskSrxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SRXDONE` reader - Slave Receive Data Interrupt Signals that a byte has been received"]
pub type IntEvent0ImaskSrxdoneR = crate::BitReader<IntEvent0ImaskSrxdone>;
impl IntEvent0ImaskSrxdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskSrxdone {
        match self.bits {
            false => IntEvent0ImaskSrxdone::IntEvent0ImaskSrxdoneClr,
            true => IntEvent0ImaskSrxdone::IntEvent0ImaskSrxdoneSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_srxdone_clr(&self) -> bool {
        *self == IntEvent0ImaskSrxdone::IntEvent0ImaskSrxdoneClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_srxdone_set(&self) -> bool {
        *self == IntEvent0ImaskSrxdone::IntEvent0ImaskSrxdoneSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SRXDONE` writer - Slave Receive Data Interrupt Signals that a byte has been received"]
pub type IntEvent0ImaskSrxdoneW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskSrxdone>;
impl<'a, REG> IntEvent0ImaskSrxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_srxdone_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSrxdone::IntEvent0ImaskSrxdoneClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_srxdone_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSrxdone::IntEvent0ImaskSrxdoneSet)
    }
}
#[doc = "Slave Transmit Transaction completed Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskStxdone {
    #[doc = "0: CLR"]
    IntEvent0ImaskStxdoneClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskStxdoneSet = 1,
}
impl From<IntEvent0ImaskStxdone> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskStxdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_STXDONE` reader - Slave Transmit Transaction completed Interrupt"]
pub type IntEvent0ImaskStxdoneR = crate::BitReader<IntEvent0ImaskStxdone>;
impl IntEvent0ImaskStxdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskStxdone {
        match self.bits {
            false => IntEvent0ImaskStxdone::IntEvent0ImaskStxdoneClr,
            true => IntEvent0ImaskStxdone::IntEvent0ImaskStxdoneSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_stxdone_clr(&self) -> bool {
        *self == IntEvent0ImaskStxdone::IntEvent0ImaskStxdoneClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_stxdone_set(&self) -> bool {
        *self == IntEvent0ImaskStxdone::IntEvent0ImaskStxdoneSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_STXDONE` writer - Slave Transmit Transaction completed Interrupt"]
pub type IntEvent0ImaskStxdoneW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskStxdone>;
impl<'a, REG> IntEvent0ImaskStxdoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_stxdone_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskStxdone::IntEvent0ImaskStxdoneClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_stxdone_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskStxdone::IntEvent0ImaskStxdoneSet)
    }
}
#[doc = "Slave Receive FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskSrxfifotrg {
    #[doc = "0: CLR"]
    IntEvent0ImaskSrxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskSrxfifotrgSet = 1,
}
impl From<IntEvent0ImaskSrxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskSrxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SRXFIFOTRG` reader - Slave Receive FIFO Trigger"]
pub type IntEvent0ImaskSrxfifotrgR = crate::BitReader<IntEvent0ImaskSrxfifotrg>;
impl IntEvent0ImaskSrxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskSrxfifotrg {
        match self.bits {
            false => IntEvent0ImaskSrxfifotrg::IntEvent0ImaskSrxfifotrgClr,
            true => IntEvent0ImaskSrxfifotrg::IntEvent0ImaskSrxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_srxfifotrg_clr(&self) -> bool {
        *self == IntEvent0ImaskSrxfifotrg::IntEvent0ImaskSrxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_srxfifotrg_set(&self) -> bool {
        *self == IntEvent0ImaskSrxfifotrg::IntEvent0ImaskSrxfifotrgSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SRXFIFOTRG` writer - Slave Receive FIFO Trigger"]
pub type IntEvent0ImaskSrxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskSrxfifotrg>;
impl<'a, REG> IntEvent0ImaskSrxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_srxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSrxfifotrg::IntEvent0ImaskSrxfifotrgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_srxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSrxfifotrg::IntEvent0ImaskSrxfifotrgSet)
    }
}
#[doc = "Slave Transmit FIFO Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskStxfifotrg {
    #[doc = "0: CLR"]
    IntEvent0ImaskStxfifotrgClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskStxfifotrgSet = 1,
}
impl From<IntEvent0ImaskStxfifotrg> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskStxfifotrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_STXFIFOTRG` reader - Slave Transmit FIFO Trigger"]
pub type IntEvent0ImaskStxfifotrgR = crate::BitReader<IntEvent0ImaskStxfifotrg>;
impl IntEvent0ImaskStxfifotrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskStxfifotrg {
        match self.bits {
            false => IntEvent0ImaskStxfifotrg::IntEvent0ImaskStxfifotrgClr,
            true => IntEvent0ImaskStxfifotrg::IntEvent0ImaskStxfifotrgSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_stxfifotrg_clr(&self) -> bool {
        *self == IntEvent0ImaskStxfifotrg::IntEvent0ImaskStxfifotrgClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_stxfifotrg_set(&self) -> bool {
        *self == IntEvent0ImaskStxfifotrg::IntEvent0ImaskStxfifotrgSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_STXFIFOTRG` writer - Slave Transmit FIFO Trigger"]
pub type IntEvent0ImaskStxfifotrgW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskStxfifotrg>;
impl<'a, REG> IntEvent0ImaskStxfifotrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_stxfifotrg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskStxfifotrg::IntEvent0ImaskStxfifotrgClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_stxfifotrg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskStxfifotrg::IntEvent0ImaskStxfifotrgSet)
    }
}
#[doc = "RXFIFO full event. This interrupt is set if an Slave RX FIFO is full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskSrxfifofull {
    #[doc = "0: CLR"]
    IntEvent0ImaskSrxfifofullClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskSrxfifofullSet = 1,
}
impl From<IntEvent0ImaskSrxfifofull> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskSrxfifofull) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SRXFIFOFULL` reader - RXFIFO full event. This interrupt is set if an Slave RX FIFO is full."]
pub type IntEvent0ImaskSrxfifofullR = crate::BitReader<IntEvent0ImaskSrxfifofull>;
impl IntEvent0ImaskSrxfifofullR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskSrxfifofull {
        match self.bits {
            false => IntEvent0ImaskSrxfifofull::IntEvent0ImaskSrxfifofullClr,
            true => IntEvent0ImaskSrxfifofull::IntEvent0ImaskSrxfifofullSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_srxfifofull_clr(&self) -> bool {
        *self == IntEvent0ImaskSrxfifofull::IntEvent0ImaskSrxfifofullClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_srxfifofull_set(&self) -> bool {
        *self == IntEvent0ImaskSrxfifofull::IntEvent0ImaskSrxfifofullSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SRXFIFOFULL` writer - RXFIFO full event. This interrupt is set if an Slave RX FIFO is full."]
pub type IntEvent0ImaskSrxfifofullW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskSrxfifofull>;
impl<'a, REG> IntEvent0ImaskSrxfifofullW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_srxfifofull_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSrxfifofull::IntEvent0ImaskSrxfifofullClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_srxfifofull_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSrxfifofull::IntEvent0ImaskSrxfifofullSet)
    }
}
#[doc = "Slave Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskStxempty {
    #[doc = "0: CLR"]
    IntEvent0ImaskStxemptyClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskStxemptySet = 1,
}
impl From<IntEvent0ImaskStxempty> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskStxempty) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_STXEMPTY` reader - Slave Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
pub type IntEvent0ImaskStxemptyR = crate::BitReader<IntEvent0ImaskStxempty>;
impl IntEvent0ImaskStxemptyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskStxempty {
        match self.bits {
            false => IntEvent0ImaskStxempty::IntEvent0ImaskStxemptyClr,
            true => IntEvent0ImaskStxempty::IntEvent0ImaskStxemptySet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_stxempty_clr(&self) -> bool {
        *self == IntEvent0ImaskStxempty::IntEvent0ImaskStxemptyClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_stxempty_set(&self) -> bool {
        *self == IntEvent0ImaskStxempty::IntEvent0ImaskStxemptySet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_STXEMPTY` writer - Slave Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
pub type IntEvent0ImaskStxemptyW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskStxempty>;
impl<'a, REG> IntEvent0ImaskStxemptyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_stxempty_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskStxempty::IntEvent0ImaskStxemptyClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_stxempty_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskStxempty::IntEvent0ImaskStxemptySet)
    }
}
#[doc = "Start Condition Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskSstart {
    #[doc = "0: CLR"]
    IntEvent0ImaskSstartClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskSstartSet = 1,
}
impl From<IntEvent0ImaskSstart> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskSstart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SSTART` reader - Start Condition Interrupt"]
pub type IntEvent0ImaskSstartR = crate::BitReader<IntEvent0ImaskSstart>;
impl IntEvent0ImaskSstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskSstart {
        match self.bits {
            false => IntEvent0ImaskSstart::IntEvent0ImaskSstartClr,
            true => IntEvent0ImaskSstart::IntEvent0ImaskSstartSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_sstart_clr(&self) -> bool {
        *self == IntEvent0ImaskSstart::IntEvent0ImaskSstartClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_sstart_set(&self) -> bool {
        *self == IntEvent0ImaskSstart::IntEvent0ImaskSstartSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SSTART` writer - Start Condition Interrupt"]
pub type IntEvent0ImaskSstartW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskSstart>;
impl<'a, REG> IntEvent0ImaskSstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_sstart_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSstart::IntEvent0ImaskSstartClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_sstart_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSstart::IntEvent0ImaskSstartSet)
    }
}
#[doc = "Stop Condition Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskSstop {
    #[doc = "0: CLR"]
    IntEvent0ImaskSstopClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskSstopSet = 1,
}
impl From<IntEvent0ImaskSstop> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskSstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SSTOP` reader - Stop Condition Interrupt"]
pub type IntEvent0ImaskSstopR = crate::BitReader<IntEvent0ImaskSstop>;
impl IntEvent0ImaskSstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskSstop {
        match self.bits {
            false => IntEvent0ImaskSstop::IntEvent0ImaskSstopClr,
            true => IntEvent0ImaskSstop::IntEvent0ImaskSstopSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_sstop_clr(&self) -> bool {
        *self == IntEvent0ImaskSstop::IntEvent0ImaskSstopClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_sstop_set(&self) -> bool {
        *self == IntEvent0ImaskSstop::IntEvent0ImaskSstopSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SSTOP` writer - Stop Condition Interrupt"]
pub type IntEvent0ImaskSstopW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskSstop>;
impl<'a, REG> IntEvent0ImaskSstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_sstop_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSstop::IntEvent0ImaskSstopClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_sstop_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSstop::IntEvent0ImaskSstopSet)
    }
}
#[doc = "General Call Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskSgencall {
    #[doc = "0: CLR"]
    IntEvent0ImaskSgencallClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskSgencallSet = 1,
}
impl From<IntEvent0ImaskSgencall> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskSgencall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SGENCALL` reader - General Call Interrupt"]
pub type IntEvent0ImaskSgencallR = crate::BitReader<IntEvent0ImaskSgencall>;
impl IntEvent0ImaskSgencallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskSgencall {
        match self.bits {
            false => IntEvent0ImaskSgencall::IntEvent0ImaskSgencallClr,
            true => IntEvent0ImaskSgencall::IntEvent0ImaskSgencallSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_sgencall_clr(&self) -> bool {
        *self == IntEvent0ImaskSgencall::IntEvent0ImaskSgencallClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_sgencall_set(&self) -> bool {
        *self == IntEvent0ImaskSgencall::IntEvent0ImaskSgencallSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SGENCALL` writer - General Call Interrupt"]
pub type IntEvent0ImaskSgencallW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskSgencall>;
impl<'a, REG> IntEvent0ImaskSgencallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_sgencall_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSgencall::IntEvent0ImaskSgencallClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_sgencall_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSgencall::IntEvent0ImaskSgencallSet)
    }
}
#[doc = "Slave DMA Done 1 on Event Channel 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskSdmaDone1_2 {
    #[doc = "0: CLR"]
    IntEvent0ImaskSdmaDone1_2Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskSdmaDone1_2Set = 1,
}
impl From<IntEvent0ImaskSdmaDone1_2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskSdmaDone1_2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SDMA_DONE1_2` reader - Slave DMA Done 1 on Event Channel 2"]
pub type IntEvent0ImaskSdmaDone1_2R = crate::BitReader<IntEvent0ImaskSdmaDone1_2>;
impl IntEvent0ImaskSdmaDone1_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskSdmaDone1_2 {
        match self.bits {
            false => IntEvent0ImaskSdmaDone1_2::IntEvent0ImaskSdmaDone1_2Clr,
            true => IntEvent0ImaskSdmaDone1_2::IntEvent0ImaskSdmaDone1_2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_sdma_done1_2_clr(&self) -> bool {
        *self == IntEvent0ImaskSdmaDone1_2::IntEvent0ImaskSdmaDone1_2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_sdma_done1_2_set(&self) -> bool {
        *self == IntEvent0ImaskSdmaDone1_2::IntEvent0ImaskSdmaDone1_2Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SDMA_DONE1_2` writer - Slave DMA Done 1 on Event Channel 2"]
pub type IntEvent0ImaskSdmaDone1_2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskSdmaDone1_2>;
impl<'a, REG> IntEvent0ImaskSdmaDone1_2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_sdma_done1_2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSdmaDone1_2::IntEvent0ImaskSdmaDone1_2Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_sdma_done1_2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSdmaDone1_2::IntEvent0ImaskSdmaDone1_2Set)
    }
}
#[doc = "Slave DMA Done 1 on Event Channel 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskSdmaDone1_3 {
    #[doc = "0: CLR"]
    IntEvent0ImaskSdmaDone1_3Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskSdmaDone1_3Set = 1,
}
impl From<IntEvent0ImaskSdmaDone1_3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskSdmaDone1_3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SDMA_DONE1_3` reader - Slave DMA Done 1 on Event Channel 3"]
pub type IntEvent0ImaskSdmaDone1_3R = crate::BitReader<IntEvent0ImaskSdmaDone1_3>;
impl IntEvent0ImaskSdmaDone1_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskSdmaDone1_3 {
        match self.bits {
            false => IntEvent0ImaskSdmaDone1_3::IntEvent0ImaskSdmaDone1_3Clr,
            true => IntEvent0ImaskSdmaDone1_3::IntEvent0ImaskSdmaDone1_3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_sdma_done1_3_clr(&self) -> bool {
        *self == IntEvent0ImaskSdmaDone1_3::IntEvent0ImaskSdmaDone1_3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_sdma_done1_3_set(&self) -> bool {
        *self == IntEvent0ImaskSdmaDone1_3::IntEvent0ImaskSdmaDone1_3Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SDMA_DONE1_3` writer - Slave DMA Done 1 on Event Channel 3"]
pub type IntEvent0ImaskSdmaDone1_3W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskSdmaDone1_3>;
impl<'a, REG> IntEvent0ImaskSdmaDone1_3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_sdma_done1_3_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSdmaDone1_3::IntEvent0ImaskSdmaDone1_3Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_sdma_done1_3_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSdmaDone1_3::IntEvent0ImaskSdmaDone1_3Set)
    }
}
#[doc = "Slave RX Pec Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskSpecRxErr {
    #[doc = "0: CLR"]
    IntEvent0ImaskSpecRxErrClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskSpecRxErrSet = 1,
}
impl From<IntEvent0ImaskSpecRxErr> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskSpecRxErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SPEC_RX_ERR` reader - Slave RX Pec Error Interrupt"]
pub type IntEvent0ImaskSpecRxErrR = crate::BitReader<IntEvent0ImaskSpecRxErr>;
impl IntEvent0ImaskSpecRxErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskSpecRxErr {
        match self.bits {
            false => IntEvent0ImaskSpecRxErr::IntEvent0ImaskSpecRxErrClr,
            true => IntEvent0ImaskSpecRxErr::IntEvent0ImaskSpecRxErrSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_spec_rx_err_clr(&self) -> bool {
        *self == IntEvent0ImaskSpecRxErr::IntEvent0ImaskSpecRxErrClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_spec_rx_err_set(&self) -> bool {
        *self == IntEvent0ImaskSpecRxErr::IntEvent0ImaskSpecRxErrSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SPEC_RX_ERR` writer - Slave RX Pec Error Interrupt"]
pub type IntEvent0ImaskSpecRxErrW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskSpecRxErr>;
impl<'a, REG> IntEvent0ImaskSpecRxErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_spec_rx_err_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSpecRxErr::IntEvent0ImaskSpecRxErrClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_spec_rx_err_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSpecRxErr::IntEvent0ImaskSpecRxErrSet)
    }
}
#[doc = "Slave TX FIFO underflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskStxUnfl {
    #[doc = "0: CLR"]
    IntEvent0ImaskStxUnflClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskStxUnflSet = 1,
}
impl From<IntEvent0ImaskStxUnfl> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskStxUnfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_STX_UNFL` reader - Slave TX FIFO underflow"]
pub type IntEvent0ImaskStxUnflR = crate::BitReader<IntEvent0ImaskStxUnfl>;
impl IntEvent0ImaskStxUnflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskStxUnfl {
        match self.bits {
            false => IntEvent0ImaskStxUnfl::IntEvent0ImaskStxUnflClr,
            true => IntEvent0ImaskStxUnfl::IntEvent0ImaskStxUnflSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_stx_unfl_clr(&self) -> bool {
        *self == IntEvent0ImaskStxUnfl::IntEvent0ImaskStxUnflClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_stx_unfl_set(&self) -> bool {
        *self == IntEvent0ImaskStxUnfl::IntEvent0ImaskStxUnflSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_STX_UNFL` writer - Slave TX FIFO underflow"]
pub type IntEvent0ImaskStxUnflW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskStxUnfl>;
impl<'a, REG> IntEvent0ImaskStxUnflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_stx_unfl_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskStxUnfl::IntEvent0ImaskStxUnflClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_stx_unfl_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskStxUnfl::IntEvent0ImaskStxUnflSet)
    }
}
#[doc = "Slave RX FIFO overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskSrxOvfl {
    #[doc = "0: CLR"]
    IntEvent0ImaskSrxOvflClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskSrxOvflSet = 1,
}
impl From<IntEvent0ImaskSrxOvfl> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskSrxOvfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SRX_OVFL` reader - Slave RX FIFO overflow"]
pub type IntEvent0ImaskSrxOvflR = crate::BitReader<IntEvent0ImaskSrxOvfl>;
impl IntEvent0ImaskSrxOvflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskSrxOvfl {
        match self.bits {
            false => IntEvent0ImaskSrxOvfl::IntEvent0ImaskSrxOvflClr,
            true => IntEvent0ImaskSrxOvfl::IntEvent0ImaskSrxOvflSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_srx_ovfl_clr(&self) -> bool {
        *self == IntEvent0ImaskSrxOvfl::IntEvent0ImaskSrxOvflClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_srx_ovfl_set(&self) -> bool {
        *self == IntEvent0ImaskSrxOvfl::IntEvent0ImaskSrxOvflSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SRX_OVFL` writer - Slave RX FIFO overflow"]
pub type IntEvent0ImaskSrxOvflW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskSrxOvfl>;
impl<'a, REG> IntEvent0ImaskSrxOvflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_srx_ovfl_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSrxOvfl::IntEvent0ImaskSrxOvflClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_srx_ovfl_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSrxOvfl::IntEvent0ImaskSrxOvflSet)
    }
}
#[doc = "Slave Arbitration Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskSarblost {
    #[doc = "0: CLR"]
    IntEvent0ImaskSarblostClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskSarblostSet = 1,
}
impl From<IntEvent0ImaskSarblost> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskSarblost) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SARBLOST` reader - Slave Arbitration Lost"]
pub type IntEvent0ImaskSarblostR = crate::BitReader<IntEvent0ImaskSarblost>;
impl IntEvent0ImaskSarblostR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskSarblost {
        match self.bits {
            false => IntEvent0ImaskSarblost::IntEvent0ImaskSarblostClr,
            true => IntEvent0ImaskSarblost::IntEvent0ImaskSarblostSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_sarblost_clr(&self) -> bool {
        *self == IntEvent0ImaskSarblost::IntEvent0ImaskSarblostClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_sarblost_set(&self) -> bool {
        *self == IntEvent0ImaskSarblost::IntEvent0ImaskSarblostSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_SARBLOST` writer - Slave Arbitration Lost"]
pub type IntEvent0ImaskSarblostW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskSarblost>;
impl<'a, REG> IntEvent0ImaskSarblostW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_sarblost_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSarblost::IntEvent0ImaskSarblostClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_sarblost_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskSarblost::IntEvent0ImaskSarblostSet)
    }
}
#[doc = "Interrupt Overflow Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskIntrOvfl {
    #[doc = "0: CLR"]
    IntEvent0ImaskIntrOvflClr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskIntrOvflSet = 1,
}
impl From<IntEvent0ImaskIntrOvfl> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskIntrOvfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_INTR_OVFL` reader - Interrupt Overflow Interrupt Mask"]
pub type IntEvent0ImaskIntrOvflR = crate::BitReader<IntEvent0ImaskIntrOvfl>;
impl IntEvent0ImaskIntrOvflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskIntrOvfl {
        match self.bits {
            false => IntEvent0ImaskIntrOvfl::IntEvent0ImaskIntrOvflClr,
            true => IntEvent0ImaskIntrOvfl::IntEvent0ImaskIntrOvflSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_intr_ovfl_clr(&self) -> bool {
        *self == IntEvent0ImaskIntrOvfl::IntEvent0ImaskIntrOvflClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_intr_ovfl_set(&self) -> bool {
        *self == IntEvent0ImaskIntrOvfl::IntEvent0ImaskIntrOvflSet
    }
}
#[doc = "Field `INT_EVENT0_IMASK_INTR_OVFL` writer - Interrupt Overflow Interrupt Mask"]
pub type IntEvent0ImaskIntrOvflW<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskIntrOvfl>;
impl<'a, REG> IntEvent0ImaskIntrOvflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_intr_ovfl_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskIntrOvfl::IntEvent0ImaskIntrOvflClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_intr_ovfl_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskIntrOvfl::IntEvent0ImaskIntrOvflSet)
    }
}
impl R {
    #[doc = "Bit 0 - Master Receive Transaction completed Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_mrxdone(&self) -> IntEvent0ImaskMrxdoneR {
        IntEvent0ImaskMrxdoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_mtxdone(&self) -> IntEvent0ImaskMtxdoneR {
        IntEvent0ImaskMtxdoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
    #[inline(always)]
    pub fn int_event0_imask_mrxfifotrg(&self) -> IntEvent0ImaskMrxfifotrgR {
        IntEvent0ImaskMrxfifotrgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
    #[inline(always)]
    pub fn int_event0_imask_mtxfifotrg(&self) -> IntEvent0ImaskMtxfifotrgR {
        IntEvent0ImaskMtxfifotrgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
    #[inline(always)]
    pub fn int_event0_imask_mrxfifofull(&self) -> IntEvent0ImaskMrxfifofullR {
        IntEvent0ImaskMrxfifofullR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
    #[inline(always)]
    pub fn int_event0_imask_mtxempty(&self) -> IntEvent0ImaskMtxemptyR {
        IntEvent0ImaskMtxemptyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Address/Data NACK Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_mnack(&self) -> IntEvent0ImaskMnackR {
        IntEvent0ImaskMnackR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - START Detection Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_mstart(&self) -> IntEvent0ImaskMstartR {
        IntEvent0ImaskMstartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STOP Detection Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_mstop(&self) -> IntEvent0ImaskMstopR {
        IntEvent0ImaskMstopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Arbitration Lost Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_marblost(&self) -> IntEvent0ImaskMarblostR {
        IntEvent0ImaskMarblostR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA Done 1 on Event Channel 2"]
    #[inline(always)]
    pub fn int_event0_imask_mdma_done1_2(&self) -> IntEvent0ImaskMdmaDone1_2R {
        IntEvent0ImaskMdmaDone1_2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA Done 1 on Event Channel 3"]
    #[inline(always)]
    pub fn int_event0_imask_mdma_done1_3(&self) -> IntEvent0ImaskMdmaDone1_3R {
        IntEvent0ImaskMdmaDone1_3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Master RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_mpec_rx_err(&self) -> IntEvent0ImaskMpecRxErrR {
        IntEvent0ImaskMpecRxErrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Timeout A Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_timeouta(&self) -> IntEvent0ImaskTimeoutaR {
        IntEvent0ImaskTimeoutaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timeout B Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_timeoutb(&self) -> IntEvent0ImaskTimeoutbR {
        IntEvent0ImaskTimeoutbR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave Receive Data Interrupt Signals that a byte has been received"]
    #[inline(always)]
    pub fn int_event0_imask_srxdone(&self) -> IntEvent0ImaskSrxdoneR {
        IntEvent0ImaskSrxdoneR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Slave Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_stxdone(&self) -> IntEvent0ImaskStxdoneR {
        IntEvent0ImaskStxdoneR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Slave Receive FIFO Trigger"]
    #[inline(always)]
    pub fn int_event0_imask_srxfifotrg(&self) -> IntEvent0ImaskSrxfifotrgR {
        IntEvent0ImaskSrxfifotrgR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Slave Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn int_event0_imask_stxfifotrg(&self) -> IntEvent0ImaskStxfifotrgR {
        IntEvent0ImaskStxfifotrgR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RXFIFO full event. This interrupt is set if an Slave RX FIFO is full."]
    #[inline(always)]
    pub fn int_event0_imask_srxfifofull(&self) -> IntEvent0ImaskSrxfifofullR {
        IntEvent0ImaskSrxfifofullR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Slave Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
    #[inline(always)]
    pub fn int_event0_imask_stxempty(&self) -> IntEvent0ImaskStxemptyR {
        IntEvent0ImaskStxemptyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Start Condition Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_sstart(&self) -> IntEvent0ImaskSstartR {
        IntEvent0ImaskSstartR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Stop Condition Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_sstop(&self) -> IntEvent0ImaskSstopR {
        IntEvent0ImaskSstopR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - General Call Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_sgencall(&self) -> IntEvent0ImaskSgencallR {
        IntEvent0ImaskSgencallR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Slave DMA Done 1 on Event Channel 2"]
    #[inline(always)]
    pub fn int_event0_imask_sdma_done1_2(&self) -> IntEvent0ImaskSdmaDone1_2R {
        IntEvent0ImaskSdmaDone1_2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Slave DMA Done 1 on Event Channel 3"]
    #[inline(always)]
    pub fn int_event0_imask_sdma_done1_3(&self) -> IntEvent0ImaskSdmaDone1_3R {
        IntEvent0ImaskSdmaDone1_3R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Slave RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_spec_rx_err(&self) -> IntEvent0ImaskSpecRxErrR {
        IntEvent0ImaskSpecRxErrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Slave TX FIFO underflow"]
    #[inline(always)]
    pub fn int_event0_imask_stx_unfl(&self) -> IntEvent0ImaskStxUnflR {
        IntEvent0ImaskStxUnflR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Slave RX FIFO overflow"]
    #[inline(always)]
    pub fn int_event0_imask_srx_ovfl(&self) -> IntEvent0ImaskSrxOvflR {
        IntEvent0ImaskSrxOvflR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Slave Arbitration Lost"]
    #[inline(always)]
    pub fn int_event0_imask_sarblost(&self) -> IntEvent0ImaskSarblostR {
        IntEvent0ImaskSarblostR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Overflow Interrupt Mask"]
    #[inline(always)]
    pub fn int_event0_imask_intr_ovfl(&self) -> IntEvent0ImaskIntrOvflR {
        IntEvent0ImaskIntrOvflR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Receive Transaction completed Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_mrxdone(&mut self) -> IntEvent0ImaskMrxdoneW<IntEvent0ImaskSpec> {
        IntEvent0ImaskMrxdoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Master Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_mtxdone(&mut self) -> IntEvent0ImaskMtxdoneW<IntEvent0ImaskSpec> {
        IntEvent0ImaskMtxdoneW::new(self, 1)
    }
    #[doc = "Bit 2 - Master Receive FIFO Trigger Trigger when RX FIFO contains &gt;= defined bytes"]
    #[inline(always)]
    pub fn int_event0_imask_mrxfifotrg(&mut self) -> IntEvent0ImaskMrxfifotrgW<IntEvent0ImaskSpec> {
        IntEvent0ImaskMrxfifotrgW::new(self, 2)
    }
    #[doc = "Bit 3 - Master Transmit FIFO Trigger Trigger when Transmit FIFO contains &lt;= defined bytes"]
    #[inline(always)]
    pub fn int_event0_imask_mtxfifotrg(&mut self) -> IntEvent0ImaskMtxfifotrgW<IntEvent0ImaskSpec> {
        IntEvent0ImaskMtxfifotrgW::new(self, 3)
    }
    #[doc = "Bit 4 - RXFIFO full event. This interrupt is set if an RX FIFO is full."]
    #[inline(always)]
    pub fn int_event0_imask_mrxfifofull(
        &mut self,
    ) -> IntEvent0ImaskMrxfifofullW<IntEvent0ImaskSpec> {
        IntEvent0ImaskMrxfifofullW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
    #[inline(always)]
    pub fn int_event0_imask_mtxempty(&mut self) -> IntEvent0ImaskMtxemptyW<IntEvent0ImaskSpec> {
        IntEvent0ImaskMtxemptyW::new(self, 5)
    }
    #[doc = "Bit 7 - Address/Data NACK Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_mnack(&mut self) -> IntEvent0ImaskMnackW<IntEvent0ImaskSpec> {
        IntEvent0ImaskMnackW::new(self, 7)
    }
    #[doc = "Bit 8 - START Detection Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_mstart(&mut self) -> IntEvent0ImaskMstartW<IntEvent0ImaskSpec> {
        IntEvent0ImaskMstartW::new(self, 8)
    }
    #[doc = "Bit 9 - STOP Detection Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_mstop(&mut self) -> IntEvent0ImaskMstopW<IntEvent0ImaskSpec> {
        IntEvent0ImaskMstopW::new(self, 9)
    }
    #[doc = "Bit 10 - Arbitration Lost Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_marblost(&mut self) -> IntEvent0ImaskMarblostW<IntEvent0ImaskSpec> {
        IntEvent0ImaskMarblostW::new(self, 10)
    }
    #[doc = "Bit 11 - DMA Done 1 on Event Channel 2"]
    #[inline(always)]
    pub fn int_event0_imask_mdma_done1_2(
        &mut self,
    ) -> IntEvent0ImaskMdmaDone1_2W<IntEvent0ImaskSpec> {
        IntEvent0ImaskMdmaDone1_2W::new(self, 11)
    }
    #[doc = "Bit 12 - DMA Done 1 on Event Channel 3"]
    #[inline(always)]
    pub fn int_event0_imask_mdma_done1_3(
        &mut self,
    ) -> IntEvent0ImaskMdmaDone1_3W<IntEvent0ImaskSpec> {
        IntEvent0ImaskMdmaDone1_3W::new(self, 12)
    }
    #[doc = "Bit 13 - Master RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_mpec_rx_err(&mut self) -> IntEvent0ImaskMpecRxErrW<IntEvent0ImaskSpec> {
        IntEvent0ImaskMpecRxErrW::new(self, 13)
    }
    #[doc = "Bit 14 - Timeout A Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_timeouta(&mut self) -> IntEvent0ImaskTimeoutaW<IntEvent0ImaskSpec> {
        IntEvent0ImaskTimeoutaW::new(self, 14)
    }
    #[doc = "Bit 15 - Timeout B Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_timeoutb(&mut self) -> IntEvent0ImaskTimeoutbW<IntEvent0ImaskSpec> {
        IntEvent0ImaskTimeoutbW::new(self, 15)
    }
    #[doc = "Bit 16 - Slave Receive Data Interrupt Signals that a byte has been received"]
    #[inline(always)]
    pub fn int_event0_imask_srxdone(&mut self) -> IntEvent0ImaskSrxdoneW<IntEvent0ImaskSpec> {
        IntEvent0ImaskSrxdoneW::new(self, 16)
    }
    #[doc = "Bit 17 - Slave Transmit Transaction completed Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_stxdone(&mut self) -> IntEvent0ImaskStxdoneW<IntEvent0ImaskSpec> {
        IntEvent0ImaskStxdoneW::new(self, 17)
    }
    #[doc = "Bit 18 - Slave Receive FIFO Trigger"]
    #[inline(always)]
    pub fn int_event0_imask_srxfifotrg(&mut self) -> IntEvent0ImaskSrxfifotrgW<IntEvent0ImaskSpec> {
        IntEvent0ImaskSrxfifotrgW::new(self, 18)
    }
    #[doc = "Bit 19 - Slave Transmit FIFO Trigger"]
    #[inline(always)]
    pub fn int_event0_imask_stxfifotrg(&mut self) -> IntEvent0ImaskStxfifotrgW<IntEvent0ImaskSpec> {
        IntEvent0ImaskStxfifotrgW::new(self, 19)
    }
    #[doc = "Bit 20 - RXFIFO full event. This interrupt is set if an Slave RX FIFO is full."]
    #[inline(always)]
    pub fn int_event0_imask_srxfifofull(
        &mut self,
    ) -> IntEvent0ImaskSrxfifofullW<IntEvent0ImaskSpec> {
        IntEvent0ImaskSrxfifofullW::new(self, 20)
    }
    #[doc = "Bit 21 - Slave Transmit FIFO Empty interrupt mask. This interrupt is set if all data in the Transmit FIFO have been shifted out and the transmit goes into idle mode."]
    #[inline(always)]
    pub fn int_event0_imask_stxempty(&mut self) -> IntEvent0ImaskStxemptyW<IntEvent0ImaskSpec> {
        IntEvent0ImaskStxemptyW::new(self, 21)
    }
    #[doc = "Bit 22 - Start Condition Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_sstart(&mut self) -> IntEvent0ImaskSstartW<IntEvent0ImaskSpec> {
        IntEvent0ImaskSstartW::new(self, 22)
    }
    #[doc = "Bit 23 - Stop Condition Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_sstop(&mut self) -> IntEvent0ImaskSstopW<IntEvent0ImaskSpec> {
        IntEvent0ImaskSstopW::new(self, 23)
    }
    #[doc = "Bit 24 - General Call Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_sgencall(&mut self) -> IntEvent0ImaskSgencallW<IntEvent0ImaskSpec> {
        IntEvent0ImaskSgencallW::new(self, 24)
    }
    #[doc = "Bit 25 - Slave DMA Done 1 on Event Channel 2"]
    #[inline(always)]
    pub fn int_event0_imask_sdma_done1_2(
        &mut self,
    ) -> IntEvent0ImaskSdmaDone1_2W<IntEvent0ImaskSpec> {
        IntEvent0ImaskSdmaDone1_2W::new(self, 25)
    }
    #[doc = "Bit 26 - Slave DMA Done 1 on Event Channel 3"]
    #[inline(always)]
    pub fn int_event0_imask_sdma_done1_3(
        &mut self,
    ) -> IntEvent0ImaskSdmaDone1_3W<IntEvent0ImaskSpec> {
        IntEvent0ImaskSdmaDone1_3W::new(self, 26)
    }
    #[doc = "Bit 27 - Slave RX Pec Error Interrupt"]
    #[inline(always)]
    pub fn int_event0_imask_spec_rx_err(&mut self) -> IntEvent0ImaskSpecRxErrW<IntEvent0ImaskSpec> {
        IntEvent0ImaskSpecRxErrW::new(self, 27)
    }
    #[doc = "Bit 28 - Slave TX FIFO underflow"]
    #[inline(always)]
    pub fn int_event0_imask_stx_unfl(&mut self) -> IntEvent0ImaskStxUnflW<IntEvent0ImaskSpec> {
        IntEvent0ImaskStxUnflW::new(self, 28)
    }
    #[doc = "Bit 29 - Slave RX FIFO overflow"]
    #[inline(always)]
    pub fn int_event0_imask_srx_ovfl(&mut self) -> IntEvent0ImaskSrxOvflW<IntEvent0ImaskSpec> {
        IntEvent0ImaskSrxOvflW::new(self, 29)
    }
    #[doc = "Bit 30 - Slave Arbitration Lost"]
    #[inline(always)]
    pub fn int_event0_imask_sarblost(&mut self) -> IntEvent0ImaskSarblostW<IntEvent0ImaskSpec> {
        IntEvent0ImaskSarblostW::new(self, 30)
    }
    #[doc = "Bit 31 - Interrupt Overflow Interrupt Mask"]
    #[inline(always)]
    pub fn int_event0_imask_intr_ovfl(&mut self) -> IntEvent0ImaskIntrOvflW<IntEvent0ImaskSpec> {
        IntEvent0ImaskIntrOvflW::new(self, 31)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event0_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0ImaskSpec;
impl crate::RegisterSpec for IntEvent0ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event0_imask::R`](R) reader structure"]
impl crate::Readable for IntEvent0ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_event0_imask::W`](W) writer structure"]
impl crate::Writable for IntEvent0ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT0_IMASK to value 0"]
impl crate::Resettable for IntEvent0ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
