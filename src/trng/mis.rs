#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Masked interrupt result for HEALTH_FAIL. Indicates to the CPU that any of the health tests have failed for the latest 1024-bit window.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisIrqHealthFail {
    #[doc = "0: CLR"]
    MisIrqHealthFailClr = 0,
    #[doc = "1: SET"]
    MisIrqHealthFailSet = 1,
}
impl From<MisIrqHealthFail> for bool {
    #[inline(always)]
    fn from(variant: MisIrqHealthFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_IRQ_HEALTH_FAIL` reader - Masked interrupt result for HEALTH_FAIL. Indicates to the CPU that any of the health tests have failed for the latest 1024-bit window."]
pub type MisIrqHealthFailR = crate::BitReader<MisIrqHealthFail>;
impl MisIrqHealthFailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisIrqHealthFail {
        match self.bits {
            false => MisIrqHealthFail::MisIrqHealthFailClr,
            true => MisIrqHealthFail::MisIrqHealthFailSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_irq_health_fail_clr(&self) -> bool {
        *self == MisIrqHealthFail::MisIrqHealthFailClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_irq_health_fail_set(&self) -> bool {
        *self == MisIrqHealthFail::MisIrqHealthFailSet
    }
}
#[doc = "Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisIrqCmdFail {
    #[doc = "0: CLR"]
    MisIrqCmdFailClr = 0,
    #[doc = "1: SET"]
    MisIrqCmdFailSet = 1,
}
impl From<MisIrqCmdFail> for bool {
    #[inline(always)]
    fn from(variant: MisIrqCmdFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_IRQ_CMD_FAIL` reader - Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected."]
pub type MisIrqCmdFailR = crate::BitReader<MisIrqCmdFail>;
impl MisIrqCmdFailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisIrqCmdFail {
        match self.bits {
            false => MisIrqCmdFail::MisIrqCmdFailClr,
            true => MisIrqCmdFail::MisIrqCmdFailSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_irq_cmd_fail_clr(&self) -> bool {
        *self == MisIrqCmdFail::MisIrqCmdFailClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_irq_cmd_fail_set(&self) -> bool {
        *self == MisIrqCmdFail::MisIrqCmdFailSet
    }
}
#[doc = "Masked interrupt source for IRQ_CMD_DONE. Indicates that the issued command/mode has completed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisIrqCmdDone {
    #[doc = "0: CLR"]
    MisIrqCmdDoneClr = 0,
    #[doc = "1: SET"]
    MisIrqCmdDoneSet = 1,
}
impl From<MisIrqCmdDone> for bool {
    #[inline(always)]
    fn from(variant: MisIrqCmdDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_IRQ_CMD_DONE` reader - Masked interrupt source for IRQ_CMD_DONE. Indicates that the issued command/mode has completed."]
pub type MisIrqCmdDoneR = crate::BitReader<MisIrqCmdDone>;
impl MisIrqCmdDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisIrqCmdDone {
        match self.bits {
            false => MisIrqCmdDone::MisIrqCmdDoneClr,
            true => MisIrqCmdDone::MisIrqCmdDoneSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_irq_cmd_done_clr(&self) -> bool {
        *self == MisIrqCmdDone::MisIrqCmdDoneClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_irq_cmd_done_set(&self) -> bool {
        *self == MisIrqCmdDone::MisIrqCmdDoneSet
    }
}
#[doc = "Masked interrupt result for CAPTURED_READY. Indicates to the CPU that the Captured Word is ready to be read. Reading the IIDX will clear this interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisIrqCapturedRdy {
    #[doc = "0: CLR"]
    MisIrqCapturedRdyClr = 0,
    #[doc = "1: SET"]
    MisIrqCapturedRdySet = 1,
}
impl From<MisIrqCapturedRdy> for bool {
    #[inline(always)]
    fn from(variant: MisIrqCapturedRdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_IRQ_CAPTURED_RDY` reader - Masked interrupt result for CAPTURED_READY. Indicates to the CPU that the Captured Word is ready to be read. Reading the IIDX will clear this interrupt."]
pub type MisIrqCapturedRdyR = crate::BitReader<MisIrqCapturedRdy>;
impl MisIrqCapturedRdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisIrqCapturedRdy {
        match self.bits {
            false => MisIrqCapturedRdy::MisIrqCapturedRdyClr,
            true => MisIrqCapturedRdy::MisIrqCapturedRdySet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_irq_captured_rdy_clr(&self) -> bool {
        *self == MisIrqCapturedRdy::MisIrqCapturedRdyClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_irq_captured_rdy_set(&self) -> bool {
        *self == MisIrqCapturedRdy::MisIrqCapturedRdySet
    }
}
impl R {
    #[doc = "Bit 0 - Masked interrupt result for HEALTH_FAIL. Indicates to the CPU that any of the health tests have failed for the latest 1024-bit window."]
    #[inline(always)]
    pub fn mis_irq_health_fail(&self) -> MisIrqHealthFailR {
        MisIrqHealthFailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected."]
    #[inline(always)]
    pub fn mis_irq_cmd_fail(&self) -> MisIrqCmdFailR {
        MisIrqCmdFailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masked interrupt source for IRQ_CMD_DONE. Indicates that the issued command/mode has completed."]
    #[inline(always)]
    pub fn mis_irq_cmd_done(&self) -> MisIrqCmdDoneR {
        MisIrqCmdDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masked interrupt result for CAPTURED_READY. Indicates to the CPU that the Captured Word is ready to be read. Reading the IIDX will clear this interrupt."]
    #[inline(always)]
    pub fn mis_irq_captured_rdy(&self) -> MisIrqCapturedRdyR {
        MisIrqCapturedRdyR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {
    const RESET_VALUE: u32 = 0;
}
