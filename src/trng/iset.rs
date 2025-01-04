#[doc = "Register `ISET` writer"]
pub type W = crate::W<IsetSpec>;
#[doc = "Indicates to the CPU that any of the health tests have failed. Reading the IIDX or DATA_CAPTURE registers will clear this interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetIrqHealthFail {
    #[doc = "0: NO_EFFECT"]
    IsetIrqHealthFailNoEffect = 0,
    #[doc = "1: SET"]
    IsetIrqHealthFailSet = 1,
}
impl From<IsetIrqHealthFail> for bool {
    #[inline(always)]
    fn from(variant: IsetIrqHealthFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_IRQ_HEALTH_FAIL` writer - Indicates to the CPU that any of the health tests have failed. Reading the IIDX or DATA_CAPTURE registers will clear this interrupt."]
pub type IsetIrqHealthFailW<'a, REG> = crate::BitWriter<'a, REG, IsetIrqHealthFail>;
impl<'a, REG> IsetIrqHealthFailW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_irq_health_fail_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetIrqHealthFail::IsetIrqHealthFailNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_irq_health_fail_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetIrqHealthFail::IsetIrqHealthFailSet)
    }
}
#[doc = "Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetIrqCmdFail {
    #[doc = "0: NO_EFFECT"]
    IsetIrqCmdFailNoEffect = 0,
    #[doc = "1: SET"]
    IsetIrqCmdFailSet = 1,
}
impl From<IsetIrqCmdFail> for bool {
    #[inline(always)]
    fn from(variant: IsetIrqCmdFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_IRQ_CMD_FAIL` writer - Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected."]
pub type IsetIrqCmdFailW<'a, REG> = crate::BitWriter<'a, REG, IsetIrqCmdFail>;
impl<'a, REG> IsetIrqCmdFailW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_irq_cmd_fail_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetIrqCmdFail::IsetIrqCmdFailNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_irq_cmd_fail_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetIrqCmdFail::IsetIrqCmdFailSet)
    }
}
#[doc = "Write to turn on CMD_DONE IRQ. Indicates that the last issued TRNG command has finished.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetIrqCmdDone {
    #[doc = "0: NO_EFFECT"]
    IsetIrqCmdDoneNoEffect = 0,
    #[doc = "1: SET"]
    IsetIrqCmdDoneSet = 1,
}
impl From<IsetIrqCmdDone> for bool {
    #[inline(always)]
    fn from(variant: IsetIrqCmdDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_IRQ_CMD_DONE` writer - Write to turn on CMD_DONE IRQ. Indicates that the last issued TRNG command has finished."]
pub type IsetIrqCmdDoneW<'a, REG> = crate::BitWriter<'a, REG, IsetIrqCmdDone>;
impl<'a, REG> IsetIrqCmdDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_irq_cmd_done_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetIrqCmdDone::IsetIrqCmdDoneNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_irq_cmd_done_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetIrqCmdDone::IsetIrqCmdDoneSet)
    }
}
#[doc = "Indicates to the CPU that the Captured Word is ready to be read. Reading the IIDX or DATA_CAPTURE registers will clear this interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetIrqCapturedRdy {
    #[doc = "0: NO_EFFECT"]
    IsetIrqCapturedRdyNoEffect = 0,
    #[doc = "1: SET"]
    IsetIrqCapturedRdySet = 1,
}
impl From<IsetIrqCapturedRdy> for bool {
    #[inline(always)]
    fn from(variant: IsetIrqCapturedRdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_IRQ_CAPTURED_RDY` writer - Indicates to the CPU that the Captured Word is ready to be read. Reading the IIDX or DATA_CAPTURE registers will clear this interrupt."]
pub type IsetIrqCapturedRdyW<'a, REG> = crate::BitWriter<'a, REG, IsetIrqCapturedRdy>;
impl<'a, REG> IsetIrqCapturedRdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_irq_captured_rdy_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetIrqCapturedRdy::IsetIrqCapturedRdyNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_irq_captured_rdy_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetIrqCapturedRdy::IsetIrqCapturedRdySet)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates to the CPU that any of the health tests have failed. Reading the IIDX or DATA_CAPTURE registers will clear this interrupt."]
    #[inline(always)]
    pub fn iset_irq_health_fail(&mut self) -> IsetIrqHealthFailW<IsetSpec> {
        IsetIrqHealthFailW::new(self, 0)
    }
    #[doc = "Bit 1 - Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected."]
    #[inline(always)]
    pub fn iset_irq_cmd_fail(&mut self) -> IsetIrqCmdFailW<IsetSpec> {
        IsetIrqCmdFailW::new(self, 1)
    }
    #[doc = "Bit 2 - Write to turn on CMD_DONE IRQ. Indicates that the last issued TRNG command has finished."]
    #[inline(always)]
    pub fn iset_irq_cmd_done(&mut self) -> IsetIrqCmdDoneW<IsetSpec> {
        IsetIrqCmdDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - Indicates to the CPU that the Captured Word is ready to be read. Reading the IIDX or DATA_CAPTURE registers will clear this interrupt."]
    #[inline(always)]
    pub fn iset_irq_captured_rdy(&mut self) -> IsetIrqCapturedRdyW<IsetSpec> {
        IsetIrqCapturedRdyW::new(self, 3)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsetSpec;
impl crate::RegisterSpec for IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iset::W`](W) writer structure"]
impl crate::Writable for IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISET to value 0"]
impl crate::Resettable for IsetSpec {
    const RESET_VALUE: u32 = 0;
}
