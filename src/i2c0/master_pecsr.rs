#[doc = "Register `MASTER_PECSR` reader"]
pub type R = crate::R<MasterPecsrSpec>;
#[doc = "Field `MASTER_PECSR_PECBYTECNT` reader - PEC Byte Count This is the current PEC Byte Count of the Master State Machine."]
pub type MasterPecsrPecbytecntR = crate::FieldReader<u16>;
#[doc = "This status bit indicates if the PEC was checked in the transaction that occurred before the last Stop. Latched on Stop.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MasterPecsrPecstsCheck {
    #[doc = "0: CLEARED"]
    MasterPecsrPecstsCheckCleared = 0,
    #[doc = "1: SET"]
    MasterPecsrPecstsCheckSet = 1,
}
impl From<MasterPecsrPecstsCheck> for bool {
    #[inline(always)]
    fn from(variant: MasterPecsrPecstsCheck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASTER_PECSR_PECSTS_CHECK` reader - This status bit indicates if the PEC was checked in the transaction that occurred before the last Stop. Latched on Stop."]
pub type MasterPecsrPecstsCheckR = crate::BitReader<MasterPecsrPecstsCheck>;
impl MasterPecsrPecstsCheckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MasterPecsrPecstsCheck {
        match self.bits {
            false => MasterPecsrPecstsCheck::MasterPecsrPecstsCheckCleared,
            true => MasterPecsrPecstsCheck::MasterPecsrPecstsCheckSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_master_pecsr_pecsts_check_cleared(&self) -> bool {
        *self == MasterPecsrPecstsCheck::MasterPecsrPecstsCheckCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_master_pecsr_pecsts_check_set(&self) -> bool {
        *self == MasterPecsrPecstsCheck::MasterPecsrPecstsCheckSet
    }
}
#[doc = "This status bit indicates if a PEC check error occurred in the transaction that occurred before the last Stop. Latched on Stop.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MasterPecsrPecstsError {
    #[doc = "0: CLEARED"]
    MasterPecsrPecstsErrorCleared = 0,
    #[doc = "1: SET"]
    MasterPecsrPecstsErrorSet = 1,
}
impl From<MasterPecsrPecstsError> for bool {
    #[inline(always)]
    fn from(variant: MasterPecsrPecstsError) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASTER_PECSR_PECSTS_ERROR` reader - This status bit indicates if a PEC check error occurred in the transaction that occurred before the last Stop. Latched on Stop."]
pub type MasterPecsrPecstsErrorR = crate::BitReader<MasterPecsrPecstsError>;
impl MasterPecsrPecstsErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MasterPecsrPecstsError {
        match self.bits {
            false => MasterPecsrPecstsError::MasterPecsrPecstsErrorCleared,
            true => MasterPecsrPecstsError::MasterPecsrPecstsErrorSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_master_pecsr_pecsts_error_cleared(&self) -> bool {
        *self == MasterPecsrPecstsError::MasterPecsrPecstsErrorCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_master_pecsr_pecsts_error_set(&self) -> bool {
        *self == MasterPecsrPecstsError::MasterPecsrPecstsErrorSet
    }
}
impl R {
    #[doc = "Bits 0:8 - PEC Byte Count This is the current PEC Byte Count of the Master State Machine."]
    #[inline(always)]
    pub fn master_pecsr_pecbytecnt(&self) -> MasterPecsrPecbytecntR {
        MasterPecsrPecbytecntR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - This status bit indicates if the PEC was checked in the transaction that occurred before the last Stop. Latched on Stop."]
    #[inline(always)]
    pub fn master_pecsr_pecsts_check(&self) -> MasterPecsrPecstsCheckR {
        MasterPecsrPecstsCheckR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This status bit indicates if a PEC check error occurred in the transaction that occurred before the last Stop. Latched on Stop."]
    #[inline(always)]
    pub fn master_pecsr_pecsts_error(&self) -> MasterPecsrPecstsErrorR {
        MasterPecsrPecstsErrorR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "I2C master PEC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`master_pecsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MasterPecsrSpec;
impl crate::RegisterSpec for MasterPecsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`master_pecsr::R`](R) reader structure"]
impl crate::Readable for MasterPecsrSpec {}
#[doc = "`reset()` method sets MASTER_PECSR to value 0"]
impl crate::Resettable for MasterPecsrSpec {
    const RESET_VALUE: u32 = 0;
}
