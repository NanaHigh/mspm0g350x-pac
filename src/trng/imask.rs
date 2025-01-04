#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "Mask for IRQ_HEALTH_FAIL. Indicates that a health test has failed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskIrqHealthFail {
    #[doc = "0: DISABLED"]
    ImaskIrqHealthFailDisabled = 0,
    #[doc = "1: ENABLED"]
    ImaskIrqHealthFailEnabled = 1,
}
impl From<ImaskIrqHealthFail> for bool {
    #[inline(always)]
    fn from(variant: ImaskIrqHealthFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_IRQ_HEALTH_FAIL` reader - Mask for IRQ_HEALTH_FAIL. Indicates that a health test has failed."]
pub type ImaskIrqHealthFailR = crate::BitReader<ImaskIrqHealthFail>;
impl ImaskIrqHealthFailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskIrqHealthFail {
        match self.bits {
            false => ImaskIrqHealthFail::ImaskIrqHealthFailDisabled,
            true => ImaskIrqHealthFail::ImaskIrqHealthFailEnabled,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_imask_irq_health_fail_disabled(&self) -> bool {
        *self == ImaskIrqHealthFail::ImaskIrqHealthFailDisabled
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn is_imask_irq_health_fail_enabled(&self) -> bool {
        *self == ImaskIrqHealthFail::ImaskIrqHealthFailEnabled
    }
}
#[doc = "Field `IMASK_IRQ_HEALTH_FAIL` writer - Mask for IRQ_HEALTH_FAIL. Indicates that a health test has failed."]
pub type ImaskIrqHealthFailW<'a, REG> = crate::BitWriter<'a, REG, ImaskIrqHealthFail>;
impl<'a, REG> ImaskIrqHealthFailW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn imask_irq_health_fail_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskIrqHealthFail::ImaskIrqHealthFailDisabled)
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn imask_irq_health_fail_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskIrqHealthFail::ImaskIrqHealthFailEnabled)
    }
}
#[doc = "Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskIrqCmdFail {
    #[doc = "0: DISABLED"]
    ImaskIrqCmdFailDisabled = 0,
    #[doc = "1: ENABLED"]
    ImaskIrqCmdFailEnabled = 1,
}
impl From<ImaskIrqCmdFail> for bool {
    #[inline(always)]
    fn from(variant: ImaskIrqCmdFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_IRQ_CMD_FAIL` reader - Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected."]
pub type ImaskIrqCmdFailR = crate::BitReader<ImaskIrqCmdFail>;
impl ImaskIrqCmdFailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskIrqCmdFail {
        match self.bits {
            false => ImaskIrqCmdFail::ImaskIrqCmdFailDisabled,
            true => ImaskIrqCmdFail::ImaskIrqCmdFailEnabled,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_imask_irq_cmd_fail_disabled(&self) -> bool {
        *self == ImaskIrqCmdFail::ImaskIrqCmdFailDisabled
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn is_imask_irq_cmd_fail_enabled(&self) -> bool {
        *self == ImaskIrqCmdFail::ImaskIrqCmdFailEnabled
    }
}
#[doc = "Field `IMASK_IRQ_CMD_FAIL` writer - Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected."]
pub type ImaskIrqCmdFailW<'a, REG> = crate::BitWriter<'a, REG, ImaskIrqCmdFail>;
impl<'a, REG> ImaskIrqCmdFailW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn imask_irq_cmd_fail_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskIrqCmdFail::ImaskIrqCmdFailDisabled)
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn imask_irq_cmd_fail_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskIrqCmdFail::ImaskIrqCmdFailEnabled)
    }
}
#[doc = "Mask for IRQ_CMD_DONE. Indicates that a command has finished\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskIrqCmdDone {
    #[doc = "0: DISABLED"]
    ImaskIrqCmdDoneDisabled = 0,
    #[doc = "1: ENABLED"]
    ImaskIrqCmdDoneEnabled = 1,
}
impl From<ImaskIrqCmdDone> for bool {
    #[inline(always)]
    fn from(variant: ImaskIrqCmdDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_IRQ_CMD_DONE` reader - Mask for IRQ_CMD_DONE. Indicates that a command has finished"]
pub type ImaskIrqCmdDoneR = crate::BitReader<ImaskIrqCmdDone>;
impl ImaskIrqCmdDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskIrqCmdDone {
        match self.bits {
            false => ImaskIrqCmdDone::ImaskIrqCmdDoneDisabled,
            true => ImaskIrqCmdDone::ImaskIrqCmdDoneEnabled,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_imask_irq_cmd_done_disabled(&self) -> bool {
        *self == ImaskIrqCmdDone::ImaskIrqCmdDoneDisabled
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn is_imask_irq_cmd_done_enabled(&self) -> bool {
        *self == ImaskIrqCmdDone::ImaskIrqCmdDoneEnabled
    }
}
#[doc = "Field `IMASK_IRQ_CMD_DONE` writer - Mask for IRQ_CMD_DONE. Indicates that a command has finished"]
pub type ImaskIrqCmdDoneW<'a, REG> = crate::BitWriter<'a, REG, ImaskIrqCmdDone>;
impl<'a, REG> ImaskIrqCmdDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn imask_irq_cmd_done_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskIrqCmdDone::ImaskIrqCmdDoneDisabled)
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn imask_irq_cmd_done_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskIrqCmdDone::ImaskIrqCmdDoneEnabled)
    }
}
#[doc = "Mask for IRQ_CAPTURED_RDY. Indicates to the CPU that the Captured Word is ready to be read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskIrqCapturedRdy {
    #[doc = "0: DISABLED"]
    ImaskIrqCapturedRdyDisabled = 0,
    #[doc = "1: ENABLED"]
    ImaskIrqCapturedRdyEnabled = 1,
}
impl From<ImaskIrqCapturedRdy> for bool {
    #[inline(always)]
    fn from(variant: ImaskIrqCapturedRdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_IRQ_CAPTURED_RDY` reader - Mask for IRQ_CAPTURED_RDY. Indicates to the CPU that the Captured Word is ready to be read."]
pub type ImaskIrqCapturedRdyR = crate::BitReader<ImaskIrqCapturedRdy>;
impl ImaskIrqCapturedRdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskIrqCapturedRdy {
        match self.bits {
            false => ImaskIrqCapturedRdy::ImaskIrqCapturedRdyDisabled,
            true => ImaskIrqCapturedRdy::ImaskIrqCapturedRdyEnabled,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_imask_irq_captured_rdy_disabled(&self) -> bool {
        *self == ImaskIrqCapturedRdy::ImaskIrqCapturedRdyDisabled
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn is_imask_irq_captured_rdy_enabled(&self) -> bool {
        *self == ImaskIrqCapturedRdy::ImaskIrqCapturedRdyEnabled
    }
}
#[doc = "Field `IMASK_IRQ_CAPTURED_RDY` writer - Mask for IRQ_CAPTURED_RDY. Indicates to the CPU that the Captured Word is ready to be read."]
pub type ImaskIrqCapturedRdyW<'a, REG> = crate::BitWriter<'a, REG, ImaskIrqCapturedRdy>;
impl<'a, REG> ImaskIrqCapturedRdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn imask_irq_captured_rdy_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskIrqCapturedRdy::ImaskIrqCapturedRdyDisabled)
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn imask_irq_captured_rdy_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskIrqCapturedRdy::ImaskIrqCapturedRdyEnabled)
    }
}
impl R {
    #[doc = "Bit 0 - Mask for IRQ_HEALTH_FAIL. Indicates that a health test has failed."]
    #[inline(always)]
    pub fn imask_irq_health_fail(&self) -> ImaskIrqHealthFailR {
        ImaskIrqHealthFailR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected."]
    #[inline(always)]
    pub fn imask_irq_cmd_fail(&self) -> ImaskIrqCmdFailR {
        ImaskIrqCmdFailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mask for IRQ_CMD_DONE. Indicates that a command has finished"]
    #[inline(always)]
    pub fn imask_irq_cmd_done(&self) -> ImaskIrqCmdDoneR {
        ImaskIrqCmdDoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mask for IRQ_CAPTURED_RDY. Indicates to the CPU that the Captured Word is ready to be read."]
    #[inline(always)]
    pub fn imask_irq_captured_rdy(&self) -> ImaskIrqCapturedRdyR {
        ImaskIrqCapturedRdyR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mask for IRQ_HEALTH_FAIL. Indicates that a health test has failed."]
    #[inline(always)]
    pub fn imask_irq_health_fail(&mut self) -> ImaskIrqHealthFailW<ImaskSpec> {
        ImaskIrqHealthFailW::new(self, 0)
    }
    #[doc = "Bit 1 - Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected."]
    #[inline(always)]
    pub fn imask_irq_cmd_fail(&mut self) -> ImaskIrqCmdFailW<ImaskSpec> {
        ImaskIrqCmdFailW::new(self, 1)
    }
    #[doc = "Bit 2 - Mask for IRQ_CMD_DONE. Indicates that a command has finished"]
    #[inline(always)]
    pub fn imask_irq_cmd_done(&mut self) -> ImaskIrqCmdDoneW<ImaskSpec> {
        ImaskIrqCmdDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - Mask for IRQ_CAPTURED_RDY. Indicates to the CPU that the Captured Word is ready to be read."]
    #[inline(always)]
    pub fn imask_irq_captured_rdy(&mut self) -> ImaskIrqCapturedRdyW<ImaskSpec> {
        ImaskIrqCapturedRdyW::new(self, 3)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImaskSpec;
impl crate::RegisterSpec for ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask::R`](R) reader structure"]
impl crate::Readable for ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`imask::W`](W) writer structure"]
impl crate::Writable for ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMASK to value 0"]
impl crate::Resettable for ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
