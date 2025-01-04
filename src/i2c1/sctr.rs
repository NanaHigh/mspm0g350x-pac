#[doc = "Register `SCTR` reader"]
pub type R = crate::R<SctrSpec>;
#[doc = "Register `SCTR` writer"]
pub type W = crate::W<SctrSpec>;
#[doc = "Device Active. Setting this bit enables the slave functionality.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SctrActive {
    #[doc = "0: DISABLE"]
    SctrActiveDisable = 0,
    #[doc = "1: ENABLE"]
    SctrActiveEnable = 1,
}
impl From<SctrActive> for bool {
    #[inline(always)]
    fn from(variant: SctrActive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCTR_ACTIVE` reader - Device Active. Setting this bit enables the slave functionality."]
pub type SctrActiveR = crate::BitReader<SctrActive>;
impl SctrActiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SctrActive {
        match self.bits {
            false => SctrActive::SctrActiveDisable,
            true => SctrActive::SctrActiveEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_sctr_active_disable(&self) -> bool {
        *self == SctrActive::SctrActiveDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_sctr_active_enable(&self) -> bool {
        *self == SctrActive::SctrActiveEnable
    }
}
#[doc = "Field `SCTR_ACTIVE` writer - Device Active. Setting this bit enables the slave functionality."]
pub type SctrActiveW<'a, REG> = crate::BitWriter<'a, REG, SctrActive>;
impl<'a, REG> SctrActiveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn sctr_active_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrActive::SctrActiveDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn sctr_active_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrActive::SctrActiveEnable)
    }
}
#[doc = "General call response enable. This bit is only available in UCBxI2COA0. Modify only when UCSWRST = 1. 0b = Do not respond to a general call 1b = Respond to a general call\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SctrGencall {
    #[doc = "0: DISABLE"]
    SctrGencallDisable = 0,
    #[doc = "1: ENABLE"]
    SctrGencallEnable = 1,
}
impl From<SctrGencall> for bool {
    #[inline(always)]
    fn from(variant: SctrGencall) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCTR_GENCALL` reader - General call response enable. This bit is only available in UCBxI2COA0. Modify only when UCSWRST = 1. 0b = Do not respond to a general call 1b = Respond to a general call"]
pub type SctrGencallR = crate::BitReader<SctrGencall>;
impl SctrGencallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SctrGencall {
        match self.bits {
            false => SctrGencall::SctrGencallDisable,
            true => SctrGencall::SctrGencallEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_sctr_gencall_disable(&self) -> bool {
        *self == SctrGencall::SctrGencallDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_sctr_gencall_enable(&self) -> bool {
        *self == SctrGencall::SctrGencallEnable
    }
}
#[doc = "Field `SCTR_GENCALL` writer - General call response enable. This bit is only available in UCBxI2COA0. Modify only when UCSWRST = 1. 0b = Do not respond to a general call 1b = Respond to a general call"]
pub type SctrGencallW<'a, REG> = crate::BitWriter<'a, REG, SctrGencall>;
impl<'a, REG> SctrGencallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn sctr_gencall_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrGencall::SctrGencallDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn sctr_gencall_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrGencall::SctrGencallEnable)
    }
}
#[doc = "Slave Clock Stretch Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SctrSclkstretch {
    #[doc = "0: DISABLE"]
    SctrSclkstretchDisable = 0,
    #[doc = "1: ENABLE"]
    SctrSclkstretchEnable = 1,
}
impl From<SctrSclkstretch> for bool {
    #[inline(always)]
    fn from(variant: SctrSclkstretch) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCTR_SCLKSTRETCH` reader - Slave Clock Stretch Enable"]
pub type SctrSclkstretchR = crate::BitReader<SctrSclkstretch>;
impl SctrSclkstretchR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SctrSclkstretch {
        match self.bits {
            false => SctrSclkstretch::SctrSclkstretchDisable,
            true => SctrSclkstretch::SctrSclkstretchEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_sctr_sclkstretch_disable(&self) -> bool {
        *self == SctrSclkstretch::SctrSclkstretchDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_sctr_sclkstretch_enable(&self) -> bool {
        *self == SctrSclkstretch::SctrSclkstretchEnable
    }
}
#[doc = "Field `SCTR_SCLKSTRETCH` writer - Slave Clock Stretch Enable"]
pub type SctrSclkstretchW<'a, REG> = crate::BitWriter<'a, REG, SctrSclkstretch>;
impl<'a, REG> SctrSclkstretchW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn sctr_sclkstretch_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrSclkstretch::SctrSclkstretchDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn sctr_sclkstretch_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrSclkstretch::SctrSclkstretchEnable)
    }
}
#[doc = "Tx Empty Interrupt on TREQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SctrTxemptyOnTreq {
    #[doc = "0: DISABLE"]
    SctrTxemptyOnTreqDisable = 0,
    #[doc = "1: ENABLE"]
    SctrTxemptyOnTreqEnable = 1,
}
impl From<SctrTxemptyOnTreq> for bool {
    #[inline(always)]
    fn from(variant: SctrTxemptyOnTreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCTR_TXEMPTY_ON_TREQ` reader - Tx Empty Interrupt on TREQ"]
pub type SctrTxemptyOnTreqR = crate::BitReader<SctrTxemptyOnTreq>;
impl SctrTxemptyOnTreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SctrTxemptyOnTreq {
        match self.bits {
            false => SctrTxemptyOnTreq::SctrTxemptyOnTreqDisable,
            true => SctrTxemptyOnTreq::SctrTxemptyOnTreqEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_sctr_txempty_on_treq_disable(&self) -> bool {
        *self == SctrTxemptyOnTreq::SctrTxemptyOnTreqDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_sctr_txempty_on_treq_enable(&self) -> bool {
        *self == SctrTxemptyOnTreq::SctrTxemptyOnTreqEnable
    }
}
#[doc = "Field `SCTR_TXEMPTY_ON_TREQ` writer - Tx Empty Interrupt on TREQ"]
pub type SctrTxemptyOnTreqW<'a, REG> = crate::BitWriter<'a, REG, SctrTxemptyOnTreq>;
impl<'a, REG> SctrTxemptyOnTreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn sctr_txempty_on_treq_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrTxemptyOnTreq::SctrTxemptyOnTreqDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn sctr_txempty_on_treq_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrTxemptyOnTreq::SctrTxemptyOnTreqEnable)
    }
}
#[doc = "Tx Trigger when slave FSM is in Tx Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SctrTxtrigTxmode {
    #[doc = "0: DISABLE"]
    SctrTxtrigTxmodeDisable = 0,
    #[doc = "1: ENABLE"]
    SctrTxtrigTxmodeEnable = 1,
}
impl From<SctrTxtrigTxmode> for bool {
    #[inline(always)]
    fn from(variant: SctrTxtrigTxmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCTR_TXTRIG_TXMODE` reader - Tx Trigger when slave FSM is in Tx Mode"]
pub type SctrTxtrigTxmodeR = crate::BitReader<SctrTxtrigTxmode>;
impl SctrTxtrigTxmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SctrTxtrigTxmode {
        match self.bits {
            false => SctrTxtrigTxmode::SctrTxtrigTxmodeDisable,
            true => SctrTxtrigTxmode::SctrTxtrigTxmodeEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_sctr_txtrig_txmode_disable(&self) -> bool {
        *self == SctrTxtrigTxmode::SctrTxtrigTxmodeDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_sctr_txtrig_txmode_enable(&self) -> bool {
        *self == SctrTxtrigTxmode::SctrTxtrigTxmodeEnable
    }
}
#[doc = "Field `SCTR_TXTRIG_TXMODE` writer - Tx Trigger when slave FSM is in Tx Mode"]
pub type SctrTxtrigTxmodeW<'a, REG> = crate::BitWriter<'a, REG, SctrTxtrigTxmode>;
impl<'a, REG> SctrTxtrigTxmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn sctr_txtrig_txmode_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrTxtrigTxmode::SctrTxtrigTxmodeDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn sctr_txtrig_txmode_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrTxtrigTxmode::SctrTxtrigTxmodeEnable)
    }
}
#[doc = "Tx transfer waits when stale data in Tx FIFO. This prevents stale bytes left in the TX FIFO from automatically being sent on the next I2C packet. Note: this should be used with TXEMPTY_ON_TREQ set to prevent the Slave State Machine from waiting for TX FIFO data without an interrupt notification when the FIFO data is stale.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SctrTxwaitStaleTxfifo {
    #[doc = "0: DISABLE"]
    SctrTxwaitStaleTxfifoDisable = 0,
    #[doc = "1: ENABLE"]
    SctrTxwaitStaleTxfifoEnable = 1,
}
impl From<SctrTxwaitStaleTxfifo> for bool {
    #[inline(always)]
    fn from(variant: SctrTxwaitStaleTxfifo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCTR_TXWAIT_STALE_TXFIFO` reader - Tx transfer waits when stale data in Tx FIFO. This prevents stale bytes left in the TX FIFO from automatically being sent on the next I2C packet. Note: this should be used with TXEMPTY_ON_TREQ set to prevent the Slave State Machine from waiting for TX FIFO data without an interrupt notification when the FIFO data is stale."]
pub type SctrTxwaitStaleTxfifoR = crate::BitReader<SctrTxwaitStaleTxfifo>;
impl SctrTxwaitStaleTxfifoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SctrTxwaitStaleTxfifo {
        match self.bits {
            false => SctrTxwaitStaleTxfifo::SctrTxwaitStaleTxfifoDisable,
            true => SctrTxwaitStaleTxfifo::SctrTxwaitStaleTxfifoEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_sctr_txwait_stale_txfifo_disable(&self) -> bool {
        *self == SctrTxwaitStaleTxfifo::SctrTxwaitStaleTxfifoDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_sctr_txwait_stale_txfifo_enable(&self) -> bool {
        *self == SctrTxwaitStaleTxfifo::SctrTxwaitStaleTxfifoEnable
    }
}
#[doc = "Field `SCTR_TXWAIT_STALE_TXFIFO` writer - Tx transfer waits when stale data in Tx FIFO. This prevents stale bytes left in the TX FIFO from automatically being sent on the next I2C packet. Note: this should be used with TXEMPTY_ON_TREQ set to prevent the Slave State Machine from waiting for TX FIFO data without an interrupt notification when the FIFO data is stale."]
pub type SctrTxwaitStaleTxfifoW<'a, REG> = crate::BitWriter<'a, REG, SctrTxwaitStaleTxfifo>;
impl<'a, REG> SctrTxwaitStaleTxfifoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn sctr_txwait_stale_txfifo_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrTxwaitStaleTxfifo::SctrTxwaitStaleTxfifoDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn sctr_txwait_stale_txfifo_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrTxwaitStaleTxfifo::SctrTxwaitStaleTxfifoEnable)
    }
}
#[doc = "Rx full interrupt generated on RREQ condition as indicated in SSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SctrRxfullOnRreq {
    #[doc = "0: DISABLE"]
    SctrRxfullOnRreqDisable = 0,
    #[doc = "1: ENABLE"]
    SctrRxfullOnRreqEnable = 1,
}
impl From<SctrRxfullOnRreq> for bool {
    #[inline(always)]
    fn from(variant: SctrRxfullOnRreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCTR_RXFULL_ON_RREQ` reader - Rx full interrupt generated on RREQ condition as indicated in SSR"]
pub type SctrRxfullOnRreqR = crate::BitReader<SctrRxfullOnRreq>;
impl SctrRxfullOnRreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SctrRxfullOnRreq {
        match self.bits {
            false => SctrRxfullOnRreq::SctrRxfullOnRreqDisable,
            true => SctrRxfullOnRreq::SctrRxfullOnRreqEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_sctr_rxfull_on_rreq_disable(&self) -> bool {
        *self == SctrRxfullOnRreq::SctrRxfullOnRreqDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_sctr_rxfull_on_rreq_enable(&self) -> bool {
        *self == SctrRxfullOnRreq::SctrRxfullOnRreqEnable
    }
}
#[doc = "Field `SCTR_RXFULL_ON_RREQ` writer - Rx full interrupt generated on RREQ condition as indicated in SSR"]
pub type SctrRxfullOnRreqW<'a, REG> = crate::BitWriter<'a, REG, SctrRxfullOnRreq>;
impl<'a, REG> SctrRxfullOnRreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn sctr_rxfull_on_rreq_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrRxfullOnRreq::SctrRxfullOnRreqDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn sctr_rxfull_on_rreq_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrRxfullOnRreq::SctrRxfullOnRreqEnable)
    }
}
#[doc = "Enable Default Host Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SctrEnDefhostadr {
    #[doc = "0: DISABLE"]
    SctrEnDefhostadrDisable = 0,
    #[doc = "1: ENABLE"]
    SctrEnDefhostadrEnable = 1,
}
impl From<SctrEnDefhostadr> for bool {
    #[inline(always)]
    fn from(variant: SctrEnDefhostadr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCTR_EN_DEFHOSTADR` reader - Enable Default Host Address"]
pub type SctrEnDefhostadrR = crate::BitReader<SctrEnDefhostadr>;
impl SctrEnDefhostadrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SctrEnDefhostadr {
        match self.bits {
            false => SctrEnDefhostadr::SctrEnDefhostadrDisable,
            true => SctrEnDefhostadr::SctrEnDefhostadrEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_sctr_en_defhostadr_disable(&self) -> bool {
        *self == SctrEnDefhostadr::SctrEnDefhostadrDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_sctr_en_defhostadr_enable(&self) -> bool {
        *self == SctrEnDefhostadr::SctrEnDefhostadrEnable
    }
}
#[doc = "Field `SCTR_EN_DEFHOSTADR` writer - Enable Default Host Address"]
pub type SctrEnDefhostadrW<'a, REG> = crate::BitWriter<'a, REG, SctrEnDefhostadr>;
impl<'a, REG> SctrEnDefhostadrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn sctr_en_defhostadr_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrEnDefhostadr::SctrEnDefhostadrDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn sctr_en_defhostadr_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrEnDefhostadr::SctrEnDefhostadrEnable)
    }
}
#[doc = "Enable Alert Response Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SctrEnAlrespadr {
    #[doc = "0: DISABLE"]
    SctrEnAlrespadrDisable = 0,
    #[doc = "1: ENABLE"]
    SctrEnAlrespadrEnable = 1,
}
impl From<SctrEnAlrespadr> for bool {
    #[inline(always)]
    fn from(variant: SctrEnAlrespadr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCTR_EN_ALRESPADR` reader - Enable Alert Response Address"]
pub type SctrEnAlrespadrR = crate::BitReader<SctrEnAlrespadr>;
impl SctrEnAlrespadrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SctrEnAlrespadr {
        match self.bits {
            false => SctrEnAlrespadr::SctrEnAlrespadrDisable,
            true => SctrEnAlrespadr::SctrEnAlrespadrEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_sctr_en_alrespadr_disable(&self) -> bool {
        *self == SctrEnAlrespadr::SctrEnAlrespadrDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_sctr_en_alrespadr_enable(&self) -> bool {
        *self == SctrEnAlrespadr::SctrEnAlrespadrEnable
    }
}
#[doc = "Field `SCTR_EN_ALRESPADR` writer - Enable Alert Response Address"]
pub type SctrEnAlrespadrW<'a, REG> = crate::BitWriter<'a, REG, SctrEnAlrespadr>;
impl<'a, REG> SctrEnAlrespadrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn sctr_en_alrespadr_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrEnAlrespadr::SctrEnAlrespadrDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn sctr_en_alrespadr_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrEnAlrespadr::SctrEnAlrespadrEnable)
    }
}
#[doc = "Enable Deault device address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SctrEnDefdevadr {
    #[doc = "0: DISABLE"]
    SctrEnDefdevadrDisable = 0,
    #[doc = "1: ENABLE"]
    SctrEnDefdevadrEnable = 1,
}
impl From<SctrEnDefdevadr> for bool {
    #[inline(always)]
    fn from(variant: SctrEnDefdevadr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCTR_EN_DEFDEVADR` reader - Enable Deault device address"]
pub type SctrEnDefdevadrR = crate::BitReader<SctrEnDefdevadr>;
impl SctrEnDefdevadrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SctrEnDefdevadr {
        match self.bits {
            false => SctrEnDefdevadr::SctrEnDefdevadrDisable,
            true => SctrEnDefdevadr::SctrEnDefdevadrEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_sctr_en_defdevadr_disable(&self) -> bool {
        *self == SctrEnDefdevadr::SctrEnDefdevadrDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_sctr_en_defdevadr_enable(&self) -> bool {
        *self == SctrEnDefdevadr::SctrEnDefdevadrEnable
    }
}
#[doc = "Field `SCTR_EN_DEFDEVADR` writer - Enable Deault device address"]
pub type SctrEnDefdevadrW<'a, REG> = crate::BitWriter<'a, REG, SctrEnDefdevadr>;
impl<'a, REG> SctrEnDefdevadrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn sctr_en_defdevadr_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrEnDefdevadr::SctrEnDefdevadrDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn sctr_en_defdevadr_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrEnDefdevadr::SctrEnDefdevadrEnable)
    }
}
#[doc = "Slave Wakeup Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SctrSwuen {
    #[doc = "0: DISABLE"]
    SctrSwuenDisable = 0,
    #[doc = "1: ENABLE"]
    SctrSwuenEnable = 1,
}
impl From<SctrSwuen> for bool {
    #[inline(always)]
    fn from(variant: SctrSwuen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCTR_SWUEN` reader - Slave Wakeup Enable"]
pub type SctrSwuenR = crate::BitReader<SctrSwuen>;
impl SctrSwuenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SctrSwuen {
        match self.bits {
            false => SctrSwuen::SctrSwuenDisable,
            true => SctrSwuen::SctrSwuenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_sctr_swuen_disable(&self) -> bool {
        *self == SctrSwuen::SctrSwuenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_sctr_swuen_enable(&self) -> bool {
        *self == SctrSwuen::SctrSwuenEnable
    }
}
#[doc = "Field `SCTR_SWUEN` writer - Slave Wakeup Enable"]
pub type SctrSwuenW<'a, REG> = crate::BitWriter<'a, REG, SctrSwuen>;
impl<'a, REG> SctrSwuenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn sctr_swuen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrSwuen::SctrSwuenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn sctr_swuen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(SctrSwuen::SctrSwuenEnable)
    }
}
impl R {
    #[doc = "Bit 0 - Device Active. Setting this bit enables the slave functionality."]
    #[inline(always)]
    pub fn sctr_active(&self) -> SctrActiveR {
        SctrActiveR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - General call response enable. This bit is only available in UCBxI2COA0. Modify only when UCSWRST = 1. 0b = Do not respond to a general call 1b = Respond to a general call"]
    #[inline(always)]
    pub fn sctr_gencall(&self) -> SctrGencallR {
        SctrGencallR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave Clock Stretch Enable"]
    #[inline(always)]
    pub fn sctr_sclkstretch(&self) -> SctrSclkstretchR {
        SctrSclkstretchR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tx Empty Interrupt on TREQ"]
    #[inline(always)]
    pub fn sctr_txempty_on_treq(&self) -> SctrTxemptyOnTreqR {
        SctrTxemptyOnTreqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx Trigger when slave FSM is in Tx Mode"]
    #[inline(always)]
    pub fn sctr_txtrig_txmode(&self) -> SctrTxtrigTxmodeR {
        SctrTxtrigTxmodeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tx transfer waits when stale data in Tx FIFO. This prevents stale bytes left in the TX FIFO from automatically being sent on the next I2C packet. Note: this should be used with TXEMPTY_ON_TREQ set to prevent the Slave State Machine from waiting for TX FIFO data without an interrupt notification when the FIFO data is stale."]
    #[inline(always)]
    pub fn sctr_txwait_stale_txfifo(&self) -> SctrTxwaitStaleTxfifoR {
        SctrTxwaitStaleTxfifoR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx full interrupt generated on RREQ condition as indicated in SSR"]
    #[inline(always)]
    pub fn sctr_rxfull_on_rreq(&self) -> SctrRxfullOnRreqR {
        SctrRxfullOnRreqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Default Host Address"]
    #[inline(always)]
    pub fn sctr_en_defhostadr(&self) -> SctrEnDefhostadrR {
        SctrEnDefhostadrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Alert Response Address"]
    #[inline(always)]
    pub fn sctr_en_alrespadr(&self) -> SctrEnAlrespadrR {
        SctrEnAlrespadrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Deault device address"]
    #[inline(always)]
    pub fn sctr_en_defdevadr(&self) -> SctrEnDefdevadrR {
        SctrEnDefdevadrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Slave Wakeup Enable"]
    #[inline(always)]
    pub fn sctr_swuen(&self) -> SctrSwuenR {
        SctrSwuenR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device Active. Setting this bit enables the slave functionality."]
    #[inline(always)]
    pub fn sctr_active(&mut self) -> SctrActiveW<SctrSpec> {
        SctrActiveW::new(self, 0)
    }
    #[doc = "Bit 1 - General call response enable. This bit is only available in UCBxI2COA0. Modify only when UCSWRST = 1. 0b = Do not respond to a general call 1b = Respond to a general call"]
    #[inline(always)]
    pub fn sctr_gencall(&mut self) -> SctrGencallW<SctrSpec> {
        SctrGencallW::new(self, 1)
    }
    #[doc = "Bit 2 - Slave Clock Stretch Enable"]
    #[inline(always)]
    pub fn sctr_sclkstretch(&mut self) -> SctrSclkstretchW<SctrSpec> {
        SctrSclkstretchW::new(self, 2)
    }
    #[doc = "Bit 3 - Tx Empty Interrupt on TREQ"]
    #[inline(always)]
    pub fn sctr_txempty_on_treq(&mut self) -> SctrTxemptyOnTreqW<SctrSpec> {
        SctrTxemptyOnTreqW::new(self, 3)
    }
    #[doc = "Bit 4 - Tx Trigger when slave FSM is in Tx Mode"]
    #[inline(always)]
    pub fn sctr_txtrig_txmode(&mut self) -> SctrTxtrigTxmodeW<SctrSpec> {
        SctrTxtrigTxmodeW::new(self, 4)
    }
    #[doc = "Bit 5 - Tx transfer waits when stale data in Tx FIFO. This prevents stale bytes left in the TX FIFO from automatically being sent on the next I2C packet. Note: this should be used with TXEMPTY_ON_TREQ set to prevent the Slave State Machine from waiting for TX FIFO data without an interrupt notification when the FIFO data is stale."]
    #[inline(always)]
    pub fn sctr_txwait_stale_txfifo(&mut self) -> SctrTxwaitStaleTxfifoW<SctrSpec> {
        SctrTxwaitStaleTxfifoW::new(self, 5)
    }
    #[doc = "Bit 6 - Rx full interrupt generated on RREQ condition as indicated in SSR"]
    #[inline(always)]
    pub fn sctr_rxfull_on_rreq(&mut self) -> SctrRxfullOnRreqW<SctrSpec> {
        SctrRxfullOnRreqW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Default Host Address"]
    #[inline(always)]
    pub fn sctr_en_defhostadr(&mut self) -> SctrEnDefhostadrW<SctrSpec> {
        SctrEnDefhostadrW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Alert Response Address"]
    #[inline(always)]
    pub fn sctr_en_alrespadr(&mut self) -> SctrEnAlrespadrW<SctrSpec> {
        SctrEnAlrespadrW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Deault device address"]
    #[inline(always)]
    pub fn sctr_en_defdevadr(&mut self) -> SctrEnDefdevadrW<SctrSpec> {
        SctrEnDefdevadrW::new(self, 9)
    }
    #[doc = "Bit 10 - Slave Wakeup Enable"]
    #[inline(always)]
    pub fn sctr_swuen(&mut self) -> SctrSwuenW<SctrSpec> {
        SctrSwuenW::new(self, 10)
    }
}
#[doc = "I2C Slave Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sctr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sctr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SctrSpec;
impl crate::RegisterSpec for SctrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sctr::R`](R) reader structure"]
impl crate::Readable for SctrSpec {}
#[doc = "`write(|w| ..)` method takes [`sctr::W`](W) writer structure"]
impl crate::Writable for SctrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCTR to value 0x0404"]
impl crate::Resettable for SctrSpec {
    const RESET_VALUE: u32 = 0x0404;
}
