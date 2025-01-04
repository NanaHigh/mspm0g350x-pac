#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Busy. This bit indicates that an active ADC sample or conversion operation is in progress.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusBusy {
    #[doc = "0: IDLE"]
    StatusBusyIdle = 0,
    #[doc = "1: ACTIVE"]
    StatusBusyActive = 1,
}
impl From<StatusBusy> for bool {
    #[inline(always)]
    fn from(variant: StatusBusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS_BUSY` reader - Busy. This bit indicates that an active ADC sample or conversion operation is in progress."]
pub type StatusBusyR = crate::BitReader<StatusBusy>;
impl StatusBusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StatusBusy {
        match self.bits {
            false => StatusBusy::StatusBusyIdle,
            true => StatusBusy::StatusBusyActive,
        }
    }
    #[doc = "IDLE"]
    #[inline(always)]
    pub fn is_status_busy_idle(&self) -> bool {
        *self == StatusBusy::StatusBusyIdle
    }
    #[doc = "ACTIVE"]
    #[inline(always)]
    pub fn is_status_busy_active(&self) -> bool {
        *self == StatusBusy::StatusBusyActive
    }
}
#[doc = "Indicates reference buffer is powered up and ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusRefbufrdy {
    #[doc = "0: NOTREADY"]
    StatusRefbufrdyNotready = 0,
    #[doc = "1: READY"]
    StatusRefbufrdyReady = 1,
}
impl From<StatusRefbufrdy> for bool {
    #[inline(always)]
    fn from(variant: StatusRefbufrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS_REFBUFRDY` reader - Indicates reference buffer is powered up and ready."]
pub type StatusRefbufrdyR = crate::BitReader<StatusRefbufrdy>;
impl StatusRefbufrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StatusRefbufrdy {
        match self.bits {
            false => StatusRefbufrdy::StatusRefbufrdyNotready,
            true => StatusRefbufrdy::StatusRefbufrdyReady,
        }
    }
    #[doc = "NOTREADY"]
    #[inline(always)]
    pub fn is_status_refbufrdy_notready(&self) -> bool {
        *self == StatusRefbufrdy::StatusRefbufrdyNotready
    }
    #[doc = "READY"]
    #[inline(always)]
    pub fn is_status_refbufrdy_ready(&self) -> bool {
        *self == StatusRefbufrdy::StatusRefbufrdyReady
    }
}
impl R {
    #[doc = "Bit 0 - Busy. This bit indicates that an active ADC sample or conversion operation is in progress."]
    #[inline(always)]
    pub fn status_busy(&self) -> StatusBusyR {
        StatusBusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates reference buffer is powered up and ready."]
    #[inline(always)]
    pub fn status_refbufrdy(&self) -> StatusRefbufrdyR {
        StatusRefbufrdyR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
