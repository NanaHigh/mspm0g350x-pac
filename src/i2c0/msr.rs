#[doc = "Register `MSR` reader"]
pub type R = crate::R<MsrSpec>;
#[doc = "I2C Master FSM Busy The BUSY bit is set during an ongoing transaction, so is set during the transmit/receive of the amount of data set in MBLEN including START, RESTART, Address and STOP signal generation when required for the current transaction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsrBusy {
    #[doc = "0: CLEARED"]
    MsrBusyCleared = 0,
    #[doc = "1: SET"]
    MsrBusySet = 1,
}
impl From<MsrBusy> for bool {
    #[inline(always)]
    fn from(variant: MsrBusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSR_BUSY` reader - I2C Master FSM Busy The BUSY bit is set during an ongoing transaction, so is set during the transmit/receive of the amount of data set in MBLEN including START, RESTART, Address and STOP signal generation when required for the current transaction."]
pub type MsrBusyR = crate::BitReader<MsrBusy>;
impl MsrBusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MsrBusy {
        match self.bits {
            false => MsrBusy::MsrBusyCleared,
            true => MsrBusy::MsrBusySet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_msr_busy_cleared(&self) -> bool {
        *self == MsrBusy::MsrBusyCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_msr_busy_set(&self) -> bool {
        *self == MsrBusy::MsrBusySet
    }
}
#[doc = "Error The error can be from the slave address not being acknowledged or the transmit data not being acknowledged.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsrErr {
    #[doc = "0: CLEARED"]
    MsrErrCleared = 0,
    #[doc = "1: SET"]
    MsrErrSet = 1,
}
impl From<MsrErr> for bool {
    #[inline(always)]
    fn from(variant: MsrErr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSR_ERR` reader - Error The error can be from the slave address not being acknowledged or the transmit data not being acknowledged."]
pub type MsrErrR = crate::BitReader<MsrErr>;
impl MsrErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MsrErr {
        match self.bits {
            false => MsrErr::MsrErrCleared,
            true => MsrErr::MsrErrSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_msr_err_cleared(&self) -> bool {
        *self == MsrErr::MsrErrCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_msr_err_set(&self) -> bool {
        *self == MsrErr::MsrErrSet
    }
}
#[doc = "Acknowledge Address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsrAdrack {
    #[doc = "0: CLEARED"]
    MsrAdrackCleared = 0,
    #[doc = "1: SET"]
    MsrAdrackSet = 1,
}
impl From<MsrAdrack> for bool {
    #[inline(always)]
    fn from(variant: MsrAdrack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSR_ADRACK` reader - Acknowledge Address"]
pub type MsrAdrackR = crate::BitReader<MsrAdrack>;
impl MsrAdrackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MsrAdrack {
        match self.bits {
            false => MsrAdrack::MsrAdrackCleared,
            true => MsrAdrack::MsrAdrackSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_msr_adrack_cleared(&self) -> bool {
        *self == MsrAdrack::MsrAdrackCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_msr_adrack_set(&self) -> bool {
        *self == MsrAdrack::MsrAdrackSet
    }
}
#[doc = "Acknowledge Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsrDatack {
    #[doc = "0: CLEARED"]
    MsrDatackCleared = 0,
    #[doc = "1: SET"]
    MsrDatackSet = 1,
}
impl From<MsrDatack> for bool {
    #[inline(always)]
    fn from(variant: MsrDatack) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSR_DATACK` reader - Acknowledge Data"]
pub type MsrDatackR = crate::BitReader<MsrDatack>;
impl MsrDatackR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MsrDatack {
        match self.bits {
            false => MsrDatack::MsrDatackCleared,
            true => MsrDatack::MsrDatackSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_msr_datack_cleared(&self) -> bool {
        *self == MsrDatack::MsrDatackCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_msr_datack_set(&self) -> bool {
        *self == MsrDatack::MsrDatackSet
    }
}
#[doc = "Arbitration Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsrArblst {
    #[doc = "0: CLEARED"]
    MsrArblstCleared = 0,
    #[doc = "1: SET"]
    MsrArblstSet = 1,
}
impl From<MsrArblst> for bool {
    #[inline(always)]
    fn from(variant: MsrArblst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSR_ARBLST` reader - Arbitration Lost"]
pub type MsrArblstR = crate::BitReader<MsrArblst>;
impl MsrArblstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MsrArblst {
        match self.bits {
            false => MsrArblst::MsrArblstCleared,
            true => MsrArblst::MsrArblstSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_msr_arblst_cleared(&self) -> bool {
        *self == MsrArblst::MsrArblstCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_msr_arblst_set(&self) -> bool {
        *self == MsrArblst::MsrArblstSet
    }
}
#[doc = "I2C Idle\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsrIdle {
    #[doc = "0: CLEARED"]
    MsrIdleCleared = 0,
    #[doc = "1: SET"]
    MsrIdleSet = 1,
}
impl From<MsrIdle> for bool {
    #[inline(always)]
    fn from(variant: MsrIdle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSR_IDLE` reader - I2C Idle"]
pub type MsrIdleR = crate::BitReader<MsrIdle>;
impl MsrIdleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MsrIdle {
        match self.bits {
            false => MsrIdle::MsrIdleCleared,
            true => MsrIdle::MsrIdleSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_msr_idle_cleared(&self) -> bool {
        *self == MsrIdle::MsrIdleCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_msr_idle_set(&self) -> bool {
        *self == MsrIdle::MsrIdleSet
    }
}
#[doc = "I2C Bus is Busy Master State Machine will wait until this bit is cleared before starting a transaction. When first enabling the Master in multi master environments, FW should wait for one I2C clock period after setting ACTIVE high before writing to the MTCR register to start the transaction so that if SCL goes low it will trigger the BUSBSY.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MsrBusbsy {
    #[doc = "0: CLEARED"]
    MsrBusbsyCleared = 0,
    #[doc = "1: SET"]
    MsrBusbsySet = 1,
}
impl From<MsrBusbsy> for bool {
    #[inline(always)]
    fn from(variant: MsrBusbsy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSR_BUSBSY` reader - I2C Bus is Busy Master State Machine will wait until this bit is cleared before starting a transaction. When first enabling the Master in multi master environments, FW should wait for one I2C clock period after setting ACTIVE high before writing to the MTCR register to start the transaction so that if SCL goes low it will trigger the BUSBSY."]
pub type MsrBusbsyR = crate::BitReader<MsrBusbsy>;
impl MsrBusbsyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MsrBusbsy {
        match self.bits {
            false => MsrBusbsy::MsrBusbsyCleared,
            true => MsrBusbsy::MsrBusbsySet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_msr_busbsy_cleared(&self) -> bool {
        *self == MsrBusbsy::MsrBusbsyCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_msr_busbsy_set(&self) -> bool {
        *self == MsrBusbsy::MsrBusbsySet
    }
}
#[doc = "Field `MSR_MBCNT` reader - I2C Master Transaction Count This field contains the current count-down value of the transaction."]
pub type MsrMbcntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - I2C Master FSM Busy The BUSY bit is set during an ongoing transaction, so is set during the transmit/receive of the amount of data set in MBLEN including START, RESTART, Address and STOP signal generation when required for the current transaction."]
    #[inline(always)]
    pub fn msr_busy(&self) -> MsrBusyR {
        MsrBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Error The error can be from the slave address not being acknowledged or the transmit data not being acknowledged."]
    #[inline(always)]
    pub fn msr_err(&self) -> MsrErrR {
        MsrErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Acknowledge Address"]
    #[inline(always)]
    pub fn msr_adrack(&self) -> MsrAdrackR {
        MsrAdrackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Acknowledge Data"]
    #[inline(always)]
    pub fn msr_datack(&self) -> MsrDatackR {
        MsrDatackR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Arbitration Lost"]
    #[inline(always)]
    pub fn msr_arblst(&self) -> MsrArblstR {
        MsrArblstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C Idle"]
    #[inline(always)]
    pub fn msr_idle(&self) -> MsrIdleR {
        MsrIdleR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C Bus is Busy Master State Machine will wait until this bit is cleared before starting a transaction. When first enabling the Master in multi master environments, FW should wait for one I2C clock period after setting ACTIVE high before writing to the MTCR register to start the transaction so that if SCL goes low it will trigger the BUSBSY."]
    #[inline(always)]
    pub fn msr_busbsy(&self) -> MsrBusbsyR {
        MsrBusbsyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:27 - I2C Master Transaction Count This field contains the current count-down value of the transaction."]
    #[inline(always)]
    pub fn msr_mbcnt(&self) -> MsrMbcntR {
        MsrMbcntR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "I2C Master Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsrSpec;
impl crate::RegisterSpec for MsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MsrSpec {}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MsrSpec {
    const RESET_VALUE: u32 = 0;
}
