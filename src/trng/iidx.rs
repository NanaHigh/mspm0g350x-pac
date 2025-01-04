#[doc = "Register `IIDX` reader"]
pub type R = crate::R<IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IidxStat {
    #[doc = "0: NO_INTR"]
    IidxStatNoIntr = 0,
    #[doc = "1: IRQ_HEALTH_FAIL"]
    IidxStatIrqHealthFail = 1,
    #[doc = "2: IRQ_CMD_FAIL"]
    IidxStatIrqCmdFail = 2,
    #[doc = "3: IRQ_CMD_DONE"]
    IidxStatIrqCmdDone = 3,
    #[doc = "4: IRQ_CAPTURED_RDY"]
    IidxStatIrqCapturedRdy = 4,
}
impl From<IidxStat> for u8 {
    #[inline(always)]
    fn from(variant: IidxStat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IidxStat {
    type Ux = u8;
}
impl crate::IsEnum for IidxStat {}
#[doc = "Field `IIDX_STAT` reader - Interrupt index status"]
pub type IidxStatR = crate::FieldReader<IidxStat>;
impl IidxStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IidxStat> {
        match self.bits {
            0 => Some(IidxStat::IidxStatNoIntr),
            1 => Some(IidxStat::IidxStatIrqHealthFail),
            2 => Some(IidxStat::IidxStatIrqCmdFail),
            3 => Some(IidxStat::IidxStatIrqCmdDone),
            4 => Some(IidxStat::IidxStatIrqCapturedRdy),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_iidx_stat_no_intr(&self) -> bool {
        *self == IidxStat::IidxStatNoIntr
    }
    #[doc = "IRQ_HEALTH_FAIL"]
    #[inline(always)]
    pub fn is_iidx_stat_irq_health_fail(&self) -> bool {
        *self == IidxStat::IidxStatIrqHealthFail
    }
    #[doc = "IRQ_CMD_FAIL"]
    #[inline(always)]
    pub fn is_iidx_stat_irq_cmd_fail(&self) -> bool {
        *self == IidxStat::IidxStatIrqCmdFail
    }
    #[doc = "IRQ_CMD_DONE"]
    #[inline(always)]
    pub fn is_iidx_stat_irq_cmd_done(&self) -> bool {
        *self == IidxStat::IidxStatIrqCmdDone
    }
    #[doc = "IRQ_CAPTURED_RDY"]
    #[inline(always)]
    pub fn is_iidx_stat_irq_captured_rdy(&self) -> bool {
        *self == IidxStat::IidxStatIrqCapturedRdy
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn iidx_stat(&self) -> IidxStatR {
        IidxStatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IidxSpec;
impl crate::RegisterSpec for IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iidx::R`](R) reader structure"]
impl crate::Readable for IidxSpec {}
#[doc = "`reset()` method sets IIDX to value 0"]
impl crate::Resettable for IidxSpec {
    const RESET_VALUE: u32 = 0;
}
