#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusUf {
    #[doc = "0: NO_UNDERFLOW"]
    StatusUfNoUnderflow = 0,
    #[doc = "1: UNDERFLOW"]
    StatusUfUnderflow = 1,
}
impl From<StatusUf> for bool {
    #[inline(always)]
    fn from(variant: StatusUf) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS_UF` reader - Underflow Flag"]
pub type StatusUfR = crate::BitReader<StatusUf>;
impl StatusUfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StatusUf {
        match self.bits {
            false => StatusUf::StatusUfNoUnderflow,
            true => StatusUf::StatusUfUnderflow,
        }
    }
    #[doc = "NO_UNDERFLOW"]
    #[inline(always)]
    pub fn is_status_uf_no_underflow(&self) -> bool {
        *self == StatusUf::StatusUfNoUnderflow
    }
    #[doc = "UNDERFLOW"]
    #[inline(always)]
    pub fn is_status_uf_underflow(&self) -> bool {
        *self == StatusUf::StatusUfUnderflow
    }
}
#[doc = "Field `STATUS_OVF` reader - Overflow bit for MPY32, SQUARE32, DIV, MAC, and SAC functions This bit will be set on overflow and will retain its value until cleared by writing 1 into CLR.CLR_OVF"]
pub type StatusOvfR = crate::BitReader;
#[doc = "Incorrect inputs/outputs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum StatusErr {
    #[doc = "0: NOERROR"]
    StatusErrNoerror = 0,
    #[doc = "1: DIVBY0"]
    StatusErrDivby0 = 1,
}
impl From<StatusErr> for u8 {
    #[inline(always)]
    fn from(variant: StatusErr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for StatusErr {
    type Ux = u8;
}
impl crate::IsEnum for StatusErr {}
#[doc = "Field `STATUS_ERR` reader - Incorrect inputs/outputs."]
pub type StatusErrR = crate::FieldReader<StatusErr>;
impl StatusErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<StatusErr> {
        match self.bits {
            0 => Some(StatusErr::StatusErrNoerror),
            1 => Some(StatusErr::StatusErrDivby0),
            _ => None,
        }
    }
    #[doc = "NOERROR"]
    #[inline(always)]
    pub fn is_status_err_noerror(&self) -> bool {
        *self == StatusErr::StatusErrNoerror
    }
    #[doc = "DIVBY0"]
    #[inline(always)]
    pub fn is_status_err_divby0(&self) -> bool {
        *self == StatusErr::StatusErrDivby0
    }
}
#[doc = "MATHACL busy bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusBusy {
    #[doc = "0: DONE"]
    StatusBusyDone = 0,
    #[doc = "1: NOTDONE"]
    StatusBusyNotdone = 1,
}
impl From<StatusBusy> for bool {
    #[inline(always)]
    fn from(variant: StatusBusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STATUS_BUSY` reader - MATHACL busy bit."]
pub type StatusBusyR = crate::BitReader<StatusBusy>;
impl StatusBusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> StatusBusy {
        match self.bits {
            false => StatusBusy::StatusBusyDone,
            true => StatusBusy::StatusBusyNotdone,
        }
    }
    #[doc = "DONE"]
    #[inline(always)]
    pub fn is_status_busy_done(&self) -> bool {
        *self == StatusBusy::StatusBusyDone
    }
    #[doc = "NOTDONE"]
    #[inline(always)]
    pub fn is_status_busy_notdone(&self) -> bool {
        *self == StatusBusy::StatusBusyNotdone
    }
}
impl R {
    #[doc = "Bit 0 - Underflow Flag"]
    #[inline(always)]
    pub fn status_uf(&self) -> StatusUfR {
        StatusUfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overflow bit for MPY32, SQUARE32, DIV, MAC, and SAC functions This bit will be set on overflow and will retain its value until cleared by writing 1 into CLR.CLR_OVF"]
    #[inline(always)]
    pub fn status_ovf(&self) -> StatusOvfR {
        StatusOvfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Incorrect inputs/outputs."]
    #[inline(always)]
    pub fn status_err(&self) -> StatusErrR {
        StatusErrR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 8 - MATHACL busy bit."]
    #[inline(always)]
    pub fn status_busy(&self) -> StatusBusyR {
        StatusBusyR::new(((self.bits >> 8) & 1) != 0)
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
