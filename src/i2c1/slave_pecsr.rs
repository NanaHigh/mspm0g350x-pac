#[doc = "Register `SLAVE_PECSR` reader"]
pub type R = crate::R<SlavePecsrSpec>;
#[doc = "Field `SLAVE_PECSR_PECBYTECNT` reader - This is the current PEC Byte Count of the Slave State Machine."]
pub type SlavePecsrPecbytecntR = crate::FieldReader<u16>;
#[doc = "This status bit indicates if the PEC was checked in the transaction that occurred before the last Stop. Latched on Stop.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlavePecsrPecstsCheck {
    #[doc = "0: CLEARED"]
    SlavePecsrPecstsCheckCleared = 0,
    #[doc = "1: SET"]
    SlavePecsrPecstsCheckSet = 1,
}
impl From<SlavePecsrPecstsCheck> for bool {
    #[inline(always)]
    fn from(variant: SlavePecsrPecstsCheck) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAVE_PECSR_PECSTS_CHECK` reader - This status bit indicates if the PEC was checked in the transaction that occurred before the last Stop. Latched on Stop."]
pub type SlavePecsrPecstsCheckR = crate::BitReader<SlavePecsrPecstsCheck>;
impl SlavePecsrPecstsCheckR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SlavePecsrPecstsCheck {
        match self.bits {
            false => SlavePecsrPecstsCheck::SlavePecsrPecstsCheckCleared,
            true => SlavePecsrPecstsCheck::SlavePecsrPecstsCheckSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_slave_pecsr_pecsts_check_cleared(&self) -> bool {
        *self == SlavePecsrPecstsCheck::SlavePecsrPecstsCheckCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_slave_pecsr_pecsts_check_set(&self) -> bool {
        *self == SlavePecsrPecstsCheck::SlavePecsrPecstsCheckSet
    }
}
#[doc = "This status bit indicates if a PEC check error occurred in the transaction that occurred before the last Stop. Latched on Stop.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlavePecsrPecstsError {
    #[doc = "0: CLEARED"]
    SlavePecsrPecstsErrorCleared = 0,
    #[doc = "1: SET"]
    SlavePecsrPecstsErrorSet = 1,
}
impl From<SlavePecsrPecstsError> for bool {
    #[inline(always)]
    fn from(variant: SlavePecsrPecstsError) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLAVE_PECSR_PECSTS_ERROR` reader - This status bit indicates if a PEC check error occurred in the transaction that occurred before the last Stop. Latched on Stop."]
pub type SlavePecsrPecstsErrorR = crate::BitReader<SlavePecsrPecstsError>;
impl SlavePecsrPecstsErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SlavePecsrPecstsError {
        match self.bits {
            false => SlavePecsrPecstsError::SlavePecsrPecstsErrorCleared,
            true => SlavePecsrPecstsError::SlavePecsrPecstsErrorSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_slave_pecsr_pecsts_error_cleared(&self) -> bool {
        *self == SlavePecsrPecstsError::SlavePecsrPecstsErrorCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_slave_pecsr_pecsts_error_set(&self) -> bool {
        *self == SlavePecsrPecstsError::SlavePecsrPecstsErrorSet
    }
}
impl R {
    #[doc = "Bits 0:8 - This is the current PEC Byte Count of the Slave State Machine."]
    #[inline(always)]
    pub fn slave_pecsr_pecbytecnt(&self) -> SlavePecsrPecbytecntR {
        SlavePecsrPecbytecntR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - This status bit indicates if the PEC was checked in the transaction that occurred before the last Stop. Latched on Stop."]
    #[inline(always)]
    pub fn slave_pecsr_pecsts_check(&self) -> SlavePecsrPecstsCheckR {
        SlavePecsrPecstsCheckR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This status bit indicates if a PEC check error occurred in the transaction that occurred before the last Stop. Latched on Stop."]
    #[inline(always)]
    pub fn slave_pecsr_pecsts_error(&self) -> SlavePecsrPecstsErrorR {
        SlavePecsrPecstsErrorR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "I2C slave PEC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`slave_pecsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SlavePecsrSpec;
impl crate::RegisterSpec for SlavePecsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slave_pecsr::R`](R) reader structure"]
impl crate::Readable for SlavePecsrSpec {}
#[doc = "`reset()` method sets SLAVE_PECSR to value 0"]
impl crate::Resettable for SlavePecsrSpec {
    const RESET_VALUE: u32 = 0;
}
