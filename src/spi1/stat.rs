#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Transmit FIFO empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatTfe {
    #[doc = "0: NOT_EMPTY"]
    StatTfeNotEmpty = 0,
    #[doc = "1: EMPTY"]
    StatTfeEmpty = 1,
}
impl From<StatTfe> for bool {
    #[inline(always)]
    fn from(variant: StatTfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAT_TFE` reader - Transmit FIFO empty."]
pub type StatTfeR = crate::BitReader<StatTfe>;
impl StatTfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StatTfe {
        match self.bits {
            false => StatTfe::StatTfeNotEmpty,
            true => StatTfe::StatTfeEmpty,
        }
    }
    #[doc = "NOT_EMPTY"]
    #[inline(always)]
    pub fn is_stat_tfe_not_empty(&self) -> bool {
        *self == StatTfe::StatTfeNotEmpty
    }
    #[doc = "EMPTY"]
    #[inline(always)]
    pub fn is_stat_tfe_empty(&self) -> bool {
        *self == StatTfe::StatTfeEmpty
    }
}
#[doc = "Transmit FIFO not full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatTnf {
    #[doc = "0: FULL"]
    StatTnfFull = 0,
    #[doc = "1: NOT_FULL"]
    StatTnfNotFull = 1,
}
impl From<StatTnf> for bool {
    #[inline(always)]
    fn from(variant: StatTnf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAT_TNF` reader - Transmit FIFO not full"]
pub type StatTnfR = crate::BitReader<StatTnf>;
impl StatTnfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StatTnf {
        match self.bits {
            false => StatTnf::StatTnfFull,
            true => StatTnf::StatTnfNotFull,
        }
    }
    #[doc = "FULL"]
    #[inline(always)]
    pub fn is_stat_tnf_full(&self) -> bool {
        *self == StatTnf::StatTnfFull
    }
    #[doc = "NOT_FULL"]
    #[inline(always)]
    pub fn is_stat_tnf_not_full(&self) -> bool {
        *self == StatTnf::StatTnfNotFull
    }
}
#[doc = "Receive FIFO empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatRfe {
    #[doc = "0: NOT_EMPTY"]
    StatRfeNotEmpty = 0,
    #[doc = "1: EMPTY"]
    StatRfeEmpty = 1,
}
impl From<StatRfe> for bool {
    #[inline(always)]
    fn from(variant: StatRfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAT_RFE` reader - Receive FIFO empty."]
pub type StatRfeR = crate::BitReader<StatRfe>;
impl StatRfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StatRfe {
        match self.bits {
            false => StatRfe::StatRfeNotEmpty,
            true => StatRfe::StatRfeEmpty,
        }
    }
    #[doc = "NOT_EMPTY"]
    #[inline(always)]
    pub fn is_stat_rfe_not_empty(&self) -> bool {
        *self == StatRfe::StatRfeNotEmpty
    }
    #[doc = "EMPTY"]
    #[inline(always)]
    pub fn is_stat_rfe_empty(&self) -> bool {
        *self == StatRfe::StatRfeEmpty
    }
}
#[doc = "Receive FIFO not full\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatRnf {
    #[doc = "0: FULL"]
    StatRnfFull = 0,
    #[doc = "1: NOT_FULL"]
    StatRnfNotFull = 1,
}
impl From<StatRnf> for bool {
    #[inline(always)]
    fn from(variant: StatRnf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAT_RNF` reader - Receive FIFO not full"]
pub type StatRnfR = crate::BitReader<StatRnf>;
impl StatRnfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StatRnf {
        match self.bits {
            false => StatRnf::StatRnfFull,
            true => StatRnf::StatRnfNotFull,
        }
    }
    #[doc = "FULL"]
    #[inline(always)]
    pub fn is_stat_rnf_full(&self) -> bool {
        *self == StatRnf::StatRnfFull
    }
    #[doc = "NOT_FULL"]
    #[inline(always)]
    pub fn is_stat_rnf_not_full(&self) -> bool {
        *self == StatRnf::StatRnfNotFull
    }
}
#[doc = "Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatBusy {
    #[doc = "0: IDLE"]
    StatBusyIdle = 0,
    #[doc = "1: ACTIVE"]
    StatBusyActive = 1,
}
impl From<StatBusy> for bool {
    #[inline(always)]
    fn from(variant: StatBusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STAT_BUSY` reader - Busy"]
pub type StatBusyR = crate::BitReader<StatBusy>;
impl StatBusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StatBusy {
        match self.bits {
            false => StatBusy::StatBusyIdle,
            true => StatBusy::StatBusyActive,
        }
    }
    #[doc = "IDLE"]
    #[inline(always)]
    pub fn is_stat_busy_idle(&self) -> bool {
        *self == StatBusy::StatBusyIdle
    }
    #[doc = "ACTIVE"]
    #[inline(always)]
    pub fn is_stat_busy_active(&self) -> bool {
        *self == StatBusy::StatBusyActive
    }
}
impl R {
    #[doc = "Bit 0 - Transmit FIFO empty."]
    #[inline(always)]
    pub fn stat_tfe(&self) -> StatTfeR {
        StatTfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO not full"]
    #[inline(always)]
    pub fn stat_tnf(&self) -> StatTnfR {
        StatTnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO empty."]
    #[inline(always)]
    pub fn stat_rfe(&self) -> StatRfeR {
        StatRfeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO not full"]
    #[inline(always)]
    pub fn stat_rnf(&self) -> StatRnfR {
        StatRnfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Busy"]
    #[inline(always)]
    pub fn stat_busy(&self) -> StatBusyR {
        StatBusyR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0;
}
