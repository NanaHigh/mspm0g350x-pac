#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Indicates to the CPU that any of the health tests have failed. Reading the IIDX will clear this interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisIrqHealthFail {
    #[doc = "0: CLR"]
    RisIrqHealthFailClr = 0,
    #[doc = "1: SET"]
    RisIrqHealthFailSet = 1,
}
impl From<RisIrqHealthFail> for bool {
    #[inline(always)]
    fn from(variant: RisIrqHealthFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_IRQ_HEALTH_FAIL` reader - Indicates to the CPU that any of the health tests have failed. Reading the IIDX will clear this interrupt."]
pub type RisIrqHealthFailR = crate::BitReader<RisIrqHealthFail>;
impl RisIrqHealthFailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisIrqHealthFail {
        match self.bits {
            false => RisIrqHealthFail::RisIrqHealthFailClr,
            true => RisIrqHealthFail::RisIrqHealthFailSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_irq_health_fail_clr(&self) -> bool {
        *self == RisIrqHealthFail::RisIrqHealthFailClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_irq_health_fail_set(&self) -> bool {
        *self == RisIrqHealthFail::RisIrqHealthFailSet
    }
}
#[doc = "Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisIrqCmdFail {
    #[doc = "0: CLR"]
    RisIrqCmdFailClr = 0,
    #[doc = "1: SET"]
    RisIrqCmdFailSet = 1,
}
impl From<RisIrqCmdFail> for bool {
    #[inline(always)]
    fn from(variant: RisIrqCmdFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_IRQ_CMD_FAIL` reader - Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected."]
pub type RisIrqCmdFailR = crate::BitReader<RisIrqCmdFail>;
impl RisIrqCmdFailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisIrqCmdFail {
        match self.bits {
            false => RisIrqCmdFail::RisIrqCmdFailClr,
            true => RisIrqCmdFail::RisIrqCmdFailSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_irq_cmd_fail_clr(&self) -> bool {
        *self == RisIrqCmdFail::RisIrqCmdFailClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_irq_cmd_fail_set(&self) -> bool {
        *self == RisIrqCmdFail::RisIrqCmdFailSet
    }
}
#[doc = "Raw interrupt source for IRQ_CMD_DONE. Indicates that the issued command/mode has completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisIrqCmdDone {
    #[doc = "0: CLR"]
    RisIrqCmdDoneClr = 0,
    #[doc = "1: SET"]
    RisIrqCmdDoneSet = 1,
}
impl From<RisIrqCmdDone> for bool {
    #[inline(always)]
    fn from(variant: RisIrqCmdDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_IRQ_CMD_DONE` reader - Raw interrupt source for IRQ_CMD_DONE. Indicates that the issued command/mode has completed."]
pub type RisIrqCmdDoneR = crate::BitReader<RisIrqCmdDone>;
impl RisIrqCmdDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisIrqCmdDone {
        match self.bits {
            false => RisIrqCmdDone::RisIrqCmdDoneClr,
            true => RisIrqCmdDone::RisIrqCmdDoneSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_irq_cmd_done_clr(&self) -> bool {
        *self == RisIrqCmdDone::RisIrqCmdDoneClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_irq_cmd_done_set(&self) -> bool {
        *self == RisIrqCmdDone::RisIrqCmdDoneSet
    }
}
#[doc = "Indicates to the CPU that the Captured Word is ready to be read. Reading the IIDX will clear this interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisIrqCapturedRdy {
    #[doc = "0: CLR"]
    RisIrqCapturedRdyClr = 0,
    #[doc = "1: SET"]
    RisIrqCapturedRdySet = 1,
}
impl From<RisIrqCapturedRdy> for bool {
    #[inline(always)]
    fn from(variant: RisIrqCapturedRdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_IRQ_CAPTURED_RDY` reader - Indicates to the CPU that the Captured Word is ready to be read. Reading the IIDX will clear this interrupt."]
pub type RisIrqCapturedRdyR = crate::BitReader<RisIrqCapturedRdy>;
impl RisIrqCapturedRdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisIrqCapturedRdy {
        match self.bits {
            false => RisIrqCapturedRdy::RisIrqCapturedRdyClr,
            true => RisIrqCapturedRdy::RisIrqCapturedRdySet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_irq_captured_rdy_clr(&self) -> bool {
        *self == RisIrqCapturedRdy::RisIrqCapturedRdyClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_irq_captured_rdy_set(&self) -> bool {
        *self == RisIrqCapturedRdy::RisIrqCapturedRdySet
    }
}
impl R {
    #[doc = "Bit 0 - Indicates to the CPU that any of the health tests have failed. Reading the IIDX will clear this interrupt."]
    #[inline(always)]
    pub fn ris_irq_health_fail(&self) -> RisIrqHealthFailR {
        RisIrqHealthFailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected."]
    #[inline(always)]
    pub fn ris_irq_cmd_fail(&self) -> RisIrqCmdFailR {
        RisIrqCmdFailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Raw interrupt source for IRQ_CMD_DONE. Indicates that the issued command/mode has completed."]
    #[inline(always)]
    pub fn ris_irq_cmd_done(&self) -> RisIrqCmdDoneR {
        RisIrqCmdDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates to the CPU that the Captured Word is ready to be read. Reading the IIDX will clear this interrupt."]
    #[inline(always)]
    pub fn ris_irq_captured_rdy(&self) -> RisIrqCapturedRdyR {
        RisIrqCapturedRdyR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0;
}
