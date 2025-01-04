#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "Indicates to the CPU that any of the health tests have failed. Reading the IIDX or DATA_CAPTURE registers will clear this interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrIrqHealthFail {
    #[doc = "0: NO_EFFECT"]
    IclrIrqHealthFailNoEffect = 0,
    #[doc = "1: CLR"]
    IclrIrqHealthFailClr = 1,
}
impl From<IclrIrqHealthFail> for bool {
    #[inline(always)]
    fn from(variant: IclrIrqHealthFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_IRQ_HEALTH_FAIL` writer - Indicates to the CPU that any of the health tests have failed. Reading the IIDX or DATA_CAPTURE registers will clear this interrupt."]
pub type IclrIrqHealthFailW<'a, REG> = crate::BitWriter<'a, REG, IclrIrqHealthFail>;
impl<'a, REG> IclrIrqHealthFailW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_irq_health_fail_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrIrqHealthFail::IclrIrqHealthFailNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_irq_health_fail_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrIrqHealthFail::IclrIrqHealthFailClr)
    }
}
#[doc = "Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrIrqCmdFail {
    #[doc = "0: NO_EFFECT"]
    IclrIrqCmdFailNoEffect = 0,
    #[doc = "1: CLR"]
    IclrIrqCmdFailClr = 1,
}
impl From<IclrIrqCmdFail> for bool {
    #[inline(always)]
    fn from(variant: IclrIrqCmdFail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_IRQ_CMD_FAIL` writer - Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected."]
pub type IclrIrqCmdFailW<'a, REG> = crate::BitWriter<'a, REG, IclrIrqCmdFail>;
impl<'a, REG> IclrIrqCmdFailW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_irq_cmd_fail_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrIrqCmdFail::IclrIrqCmdFailNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_irq_cmd_fail_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrIrqCmdFail::IclrIrqCmdFailClr)
    }
}
#[doc = "Write to turn off CMD_DONE IRQ. Indicates that the last issued TRNG command has finished.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrIrqCmdDone {
    #[doc = "0: NO_EFFECT"]
    IclrIrqCmdDoneNoEffect = 0,
    #[doc = "1: CLR"]
    IclrIrqCmdDoneClr = 1,
}
impl From<IclrIrqCmdDone> for bool {
    #[inline(always)]
    fn from(variant: IclrIrqCmdDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_IRQ_CMD_DONE` writer - Write to turn off CMD_DONE IRQ. Indicates that the last issued TRNG command has finished."]
pub type IclrIrqCmdDoneW<'a, REG> = crate::BitWriter<'a, REG, IclrIrqCmdDone>;
impl<'a, REG> IclrIrqCmdDoneW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_irq_cmd_done_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrIrqCmdDone::IclrIrqCmdDoneNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_irq_cmd_done_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrIrqCmdDone::IclrIrqCmdDoneClr)
    }
}
#[doc = "Indicates to the CPU that the Captured Word is ready to be read. Reading the IIDX or DATA_CAPTURE registers will clear this interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrIrqCapturedRdy {
    #[doc = "0: NO_EFFECT"]
    IclrIrqCapturedRdyNoEffect = 0,
    #[doc = "1: CLR"]
    IclrIrqCapturedRdyClr = 1,
}
impl From<IclrIrqCapturedRdy> for bool {
    #[inline(always)]
    fn from(variant: IclrIrqCapturedRdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_IRQ_CAPTURED_RDY` writer - Indicates to the CPU that the Captured Word is ready to be read. Reading the IIDX or DATA_CAPTURE registers will clear this interrupt."]
pub type IclrIrqCapturedRdyW<'a, REG> = crate::BitWriter<'a, REG, IclrIrqCapturedRdy>;
impl<'a, REG> IclrIrqCapturedRdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_irq_captured_rdy_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrIrqCapturedRdy::IclrIrqCapturedRdyNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_irq_captured_rdy_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrIrqCapturedRdy::IclrIrqCapturedRdyClr)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates to the CPU that any of the health tests have failed. Reading the IIDX or DATA_CAPTURE registers will clear this interrupt."]
    #[inline(always)]
    pub fn iclr_irq_health_fail(&mut self) -> IclrIrqHealthFailW<IclrSpec> {
        IclrIrqHealthFailW::new(self, 0)
    }
    #[doc = "Bit 1 - Masked interrupt source for IRQ_CMD_FAIL. Indicates that the just issued command/mode has been rejected."]
    #[inline(always)]
    pub fn iclr_irq_cmd_fail(&mut self) -> IclrIrqCmdFailW<IclrSpec> {
        IclrIrqCmdFailW::new(self, 1)
    }
    #[doc = "Bit 2 - Write to turn off CMD_DONE IRQ. Indicates that the last issued TRNG command has finished."]
    #[inline(always)]
    pub fn iclr_irq_cmd_done(&mut self) -> IclrIrqCmdDoneW<IclrSpec> {
        IclrIrqCmdDoneW::new(self, 2)
    }
    #[doc = "Bit 3 - Indicates to the CPU that the Captured Word is ready to be read. Reading the IIDX or DATA_CAPTURE registers will clear this interrupt."]
    #[inline(always)]
    pub fn iclr_irq_captured_rdy(&mut self) -> IclrIrqCapturedRdyW<IclrSpec> {
        IclrIrqCapturedRdyW::new(self, 3)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IclrSpec;
impl crate::RegisterSpec for IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iclr::W`](W) writer structure"]
impl crate::Writable for IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for IclrSpec {
    const RESET_VALUE: u32 = 0;
}
