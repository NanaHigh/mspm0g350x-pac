#[doc = "Register `SSR` reader"]
pub type R = crate::R<SsrSpec>;
#[doc = "Receive Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsrRreq {
    #[doc = "0: CLEARED"]
    SsrRreqCleared = 0,
    #[doc = "1: SET"]
    SsrRreqSet = 1,
}
impl From<SsrRreq> for bool {
    #[inline(always)]
    fn from(variant: SsrRreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSR_RREQ` reader - Receive Request"]
pub type SsrRreqR = crate::BitReader<SsrRreq>;
impl SsrRreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SsrRreq {
        match self.bits {
            false => SsrRreq::SsrRreqCleared,
            true => SsrRreq::SsrRreqSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_ssr_rreq_cleared(&self) -> bool {
        *self == SsrRreq::SsrRreqCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ssr_rreq_set(&self) -> bool {
        *self == SsrRreq::SsrRreqSet
    }
}
#[doc = "Transmit Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsrTreq {
    #[doc = "0: CLEARED"]
    SsrTreqCleared = 0,
    #[doc = "1: SET"]
    SsrTreqSet = 1,
}
impl From<SsrTreq> for bool {
    #[inline(always)]
    fn from(variant: SsrTreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSR_TREQ` reader - Transmit Request"]
pub type SsrTreqR = crate::BitReader<SsrTreq>;
impl SsrTreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SsrTreq {
        match self.bits {
            false => SsrTreq::SsrTreqCleared,
            true => SsrTreq::SsrTreqSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_ssr_treq_cleared(&self) -> bool {
        *self == SsrTreq::SsrTreqCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ssr_treq_set(&self) -> bool {
        *self == SsrTreq::SsrTreqSet
    }
}
#[doc = "Slave FSM is in Rx MODE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsrRxmode {
    #[doc = "0: CLEARED"]
    SsrRxmodeCleared = 0,
    #[doc = "1: SET"]
    SsrRxmodeSet = 1,
}
impl From<SsrRxmode> for bool {
    #[inline(always)]
    fn from(variant: SsrRxmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSR_RXMODE` reader - Slave FSM is in Rx MODE"]
pub type SsrRxmodeR = crate::BitReader<SsrRxmode>;
impl SsrRxmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SsrRxmode {
        match self.bits {
            false => SsrRxmode::SsrRxmodeCleared,
            true => SsrRxmode::SsrRxmodeSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_ssr_rxmode_cleared(&self) -> bool {
        *self == SsrRxmode::SsrRxmodeCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ssr_rxmode_set(&self) -> bool {
        *self == SsrRxmode::SsrRxmodeSet
    }
}
#[doc = "OAR2 Address Matched This bit gets reevaluated after every address comparison.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsrOar2sel {
    #[doc = "0: CLEARED"]
    SsrOar2selCleared = 0,
    #[doc = "1: SET"]
    SsrOar2selSet = 1,
}
impl From<SsrOar2sel> for bool {
    #[inline(always)]
    fn from(variant: SsrOar2sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSR_OAR2SEL` reader - OAR2 Address Matched This bit gets reevaluated after every address comparison."]
pub type SsrOar2selR = crate::BitReader<SsrOar2sel>;
impl SsrOar2selR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SsrOar2sel {
        match self.bits {
            false => SsrOar2sel::SsrOar2selCleared,
            true => SsrOar2sel::SsrOar2selSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_ssr_oar2sel_cleared(&self) -> bool {
        *self == SsrOar2sel::SsrOar2selCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ssr_oar2sel_set(&self) -> bool {
        *self == SsrOar2sel::SsrOar2selSet
    }
}
#[doc = "Quick Command Status Value Description: 0: The last transaction was a normal transaction or a transaction has not occurred. 1: The last transaction was a Quick Command transaction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsrQcmdst {
    #[doc = "0: CLEARED"]
    SsrQcmdstCleared = 0,
    #[doc = "1: SET"]
    SsrQcmdstSet = 1,
}
impl From<SsrQcmdst> for bool {
    #[inline(always)]
    fn from(variant: SsrQcmdst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSR_QCMDST` reader - Quick Command Status Value Description: 0: The last transaction was a normal transaction or a transaction has not occurred. 1: The last transaction was a Quick Command transaction"]
pub type SsrQcmdstR = crate::BitReader<SsrQcmdst>;
impl SsrQcmdstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SsrQcmdst {
        match self.bits {
            false => SsrQcmdst::SsrQcmdstCleared,
            true => SsrQcmdst::SsrQcmdstSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_ssr_qcmdst_cleared(&self) -> bool {
        *self == SsrQcmdst::SsrQcmdstCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ssr_qcmdst_set(&self) -> bool {
        *self == SsrQcmdst::SsrQcmdstSet
    }
}
#[doc = "Quick Command Read / Write This bit only has meaning when the QCMDST bit is set. Value Description: 0: Quick command was a write 1: Quick command was a read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsrQcmdrw {
    #[doc = "0: CLEARED"]
    SsrQcmdrwCleared = 0,
    #[doc = "1: SET"]
    SsrQcmdrwSet = 1,
}
impl From<SsrQcmdrw> for bool {
    #[inline(always)]
    fn from(variant: SsrQcmdrw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSR_QCMDRW` reader - Quick Command Read / Write This bit only has meaning when the QCMDST bit is set. Value Description: 0: Quick command was a write 1: Quick command was a read"]
pub type SsrQcmdrwR = crate::BitReader<SsrQcmdrw>;
impl SsrQcmdrwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SsrQcmdrw {
        match self.bits {
            false => SsrQcmdrw::SsrQcmdrwCleared,
            true => SsrQcmdrw::SsrQcmdrwSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_ssr_qcmdrw_cleared(&self) -> bool {
        *self == SsrQcmdrw::SsrQcmdrwCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ssr_qcmdrw_set(&self) -> bool {
        *self == SsrQcmdrw::SsrQcmdrwSet
    }
}
#[doc = "I2C bus is busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsrBusbsy {
    #[doc = "0: CLEARED"]
    SsrBusbsyCleared = 0,
    #[doc = "1: SET"]
    SsrBusbsySet = 1,
}
impl From<SsrBusbsy> for bool {
    #[inline(always)]
    fn from(variant: SsrBusbsy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSR_BUSBSY` reader - I2C bus is busy"]
pub type SsrBusbsyR = crate::BitReader<SsrBusbsy>;
impl SsrBusbsyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SsrBusbsy {
        match self.bits {
            false => SsrBusbsy::SsrBusbsyCleared,
            true => SsrBusbsy::SsrBusbsySet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_ssr_busbsy_cleared(&self) -> bool {
        *self == SsrBusbsy::SsrBusbsyCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ssr_busbsy_set(&self) -> bool {
        *self == SsrBusbsy::SsrBusbsySet
    }
}
#[doc = "Slave FSM is in TX MODE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsrTxmode {
    #[doc = "0: CLEARED"]
    SsrTxmodeCleared = 0,
    #[doc = "1: SET"]
    SsrTxmodeSet = 1,
}
impl From<SsrTxmode> for bool {
    #[inline(always)]
    fn from(variant: SsrTxmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSR_TXMODE` reader - Slave FSM is in TX MODE"]
pub type SsrTxmodeR = crate::BitReader<SsrTxmode>;
impl SsrTxmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SsrTxmode {
        match self.bits {
            false => SsrTxmode::SsrTxmodeCleared,
            true => SsrTxmode::SsrTxmodeSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_ssr_txmode_cleared(&self) -> bool {
        *self == SsrTxmode::SsrTxmodeCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ssr_txmode_set(&self) -> bool {
        *self == SsrTxmode::SsrTxmodeSet
    }
}
#[doc = "Stale Tx FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SsrStaleTxfifo {
    #[doc = "0: CLEARED"]
    SsrStaleTxfifoCleared = 0,
    #[doc = "1: SET"]
    SsrStaleTxfifoSet = 1,
}
impl From<SsrStaleTxfifo> for bool {
    #[inline(always)]
    fn from(variant: SsrStaleTxfifo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSR_STALE_TXFIFO` reader - Stale Tx FIFO"]
pub type SsrStaleTxfifoR = crate::BitReader<SsrStaleTxfifo>;
impl SsrStaleTxfifoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SsrStaleTxfifo {
        match self.bits {
            false => SsrStaleTxfifo::SsrStaleTxfifoCleared,
            true => SsrStaleTxfifo::SsrStaleTxfifoSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_ssr_stale_txfifo_cleared(&self) -> bool {
        *self == SsrStaleTxfifo::SsrStaleTxfifoCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ssr_stale_txfifo_set(&self) -> bool {
        *self == SsrStaleTxfifo::SsrStaleTxfifoSet
    }
}
#[doc = "Field `SSR_ADDRMATCH` reader - Indicates the address for which slave address match happened"]
pub type SsrAddrmatchR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Receive Request"]
    #[inline(always)]
    pub fn ssr_rreq(&self) -> SsrRreqR {
        SsrRreqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Request"]
    #[inline(always)]
    pub fn ssr_treq(&self) -> SsrTreqR {
        SsrTreqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave FSM is in Rx MODE"]
    #[inline(always)]
    pub fn ssr_rxmode(&self) -> SsrRxmodeR {
        SsrRxmodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OAR2 Address Matched This bit gets reevaluated after every address comparison."]
    #[inline(always)]
    pub fn ssr_oar2sel(&self) -> SsrOar2selR {
        SsrOar2selR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Quick Command Status Value Description: 0: The last transaction was a normal transaction or a transaction has not occurred. 1: The last transaction was a Quick Command transaction"]
    #[inline(always)]
    pub fn ssr_qcmdst(&self) -> SsrQcmdstR {
        SsrQcmdstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Quick Command Read / Write This bit only has meaning when the QCMDST bit is set. Value Description: 0: Quick command was a write 1: Quick command was a read"]
    #[inline(always)]
    pub fn ssr_qcmdrw(&self) -> SsrQcmdrwR {
        SsrQcmdrwR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C bus is busy"]
    #[inline(always)]
    pub fn ssr_busbsy(&self) -> SsrBusbsyR {
        SsrBusbsyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Slave FSM is in TX MODE"]
    #[inline(always)]
    pub fn ssr_txmode(&self) -> SsrTxmodeR {
        SsrTxmodeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stale Tx FIFO"]
    #[inline(always)]
    pub fn ssr_stale_txfifo(&self) -> SsrStaleTxfifoR {
        SsrStaleTxfifoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:18 - Indicates the address for which slave address match happened"]
    #[inline(always)]
    pub fn ssr_addrmatch(&self) -> SsrAddrmatchR {
        SsrAddrmatchR::new(((self.bits >> 9) & 0x03ff) as u16)
    }
}
#[doc = "I2C Slave Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsrSpec;
impl crate::RegisterSpec for SsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssr::R`](R) reader structure"]
impl crate::Readable for SsrSpec {}
#[doc = "`reset()` method sets SSR to value 0"]
impl crate::Resettable for SsrSpec {
    const RESET_VALUE: u32 = 0;
}
