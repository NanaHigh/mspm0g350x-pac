#[doc = "Register `PDBGCTL` reader"]
pub type R = crate::R<PdbgctlSpec>;
#[doc = "Register `PDBGCTL` writer"]
pub type W = crate::W<PdbgctlSpec>;
#[doc = "Free run control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdbgctlFree {
    #[doc = "0: STOP"]
    PdbgctlFreeStop = 0,
    #[doc = "1: RUN"]
    PdbgctlFreeRun = 1,
}
impl From<PdbgctlFree> for bool {
    #[inline(always)]
    fn from(variant: PdbgctlFree) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDBGCTL_FREE` reader - Free run control"]
pub type PdbgctlFreeR = crate::BitReader<PdbgctlFree>;
impl PdbgctlFreeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdbgctlFree {
        match self.bits {
            false => PdbgctlFree::PdbgctlFreeStop,
            true => PdbgctlFree::PdbgctlFreeRun,
        }
    }
    #[doc = "STOP"]
    #[inline(always)]
    pub fn is_pdbgctl_free_stop(&self) -> bool {
        *self == PdbgctlFree::PdbgctlFreeStop
    }
    #[doc = "RUN"]
    #[inline(always)]
    pub fn is_pdbgctl_free_run(&self) -> bool {
        *self == PdbgctlFree::PdbgctlFreeRun
    }
}
#[doc = "Field `PDBGCTL_FREE` writer - Free run control"]
pub type PdbgctlFreeW<'a, REG> = crate::BitWriter<'a, REG, PdbgctlFree>;
impl<'a, REG> PdbgctlFreeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "STOP"]
    #[inline(always)]
    pub fn pdbgctl_free_stop(self) -> &'a mut crate::W<REG> {
        self.variant(PdbgctlFree::PdbgctlFreeStop)
    }
    #[doc = "RUN"]
    #[inline(always)]
    pub fn pdbgctl_free_run(self) -> &'a mut crate::W<REG> {
        self.variant(PdbgctlFree::PdbgctlFreeRun)
    }
}
#[doc = "Soft halt boundary control. This function is only available, if \\[FREE\\]
is set to 'STOP'\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdbgctlSoft {
    #[doc = "0: IMMEDIATE"]
    PdbgctlSoftImmediate = 0,
    #[doc = "1: DELAYED"]
    PdbgctlSoftDelayed = 1,
}
impl From<PdbgctlSoft> for bool {
    #[inline(always)]
    fn from(variant: PdbgctlSoft) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDBGCTL_SOFT` reader - Soft halt boundary control. This function is only available, if \\[FREE\\]
is set to 'STOP'"]
pub type PdbgctlSoftR = crate::BitReader<PdbgctlSoft>;
impl PdbgctlSoftR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PdbgctlSoft {
        match self.bits {
            false => PdbgctlSoft::PdbgctlSoftImmediate,
            true => PdbgctlSoft::PdbgctlSoftDelayed,
        }
    }
    #[doc = "IMMEDIATE"]
    #[inline(always)]
    pub fn is_pdbgctl_soft_immediate(&self) -> bool {
        *self == PdbgctlSoft::PdbgctlSoftImmediate
    }
    #[doc = "DELAYED"]
    #[inline(always)]
    pub fn is_pdbgctl_soft_delayed(&self) -> bool {
        *self == PdbgctlSoft::PdbgctlSoftDelayed
    }
}
#[doc = "Field `PDBGCTL_SOFT` writer - Soft halt boundary control. This function is only available, if \\[FREE\\]
is set to 'STOP'"]
pub type PdbgctlSoftW<'a, REG> = crate::BitWriter<'a, REG, PdbgctlSoft>;
impl<'a, REG> PdbgctlSoftW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IMMEDIATE"]
    #[inline(always)]
    pub fn pdbgctl_soft_immediate(self) -> &'a mut crate::W<REG> {
        self.variant(PdbgctlSoft::PdbgctlSoftImmediate)
    }
    #[doc = "DELAYED"]
    #[inline(always)]
    pub fn pdbgctl_soft_delayed(self) -> &'a mut crate::W<REG> {
        self.variant(PdbgctlSoft::PdbgctlSoftDelayed)
    }
}
impl R {
    #[doc = "Bit 0 - Free run control"]
    #[inline(always)]
    pub fn pdbgctl_free(&self) -> PdbgctlFreeR {
        PdbgctlFreeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Soft halt boundary control. This function is only available, if \\[FREE\\]
is set to 'STOP'"]
    #[inline(always)]
    pub fn pdbgctl_soft(&self) -> PdbgctlSoftR {
        PdbgctlSoftR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Free run control"]
    #[inline(always)]
    pub fn pdbgctl_free(&mut self) -> PdbgctlFreeW<PdbgctlSpec> {
        PdbgctlFreeW::new(self, 0)
    }
    #[doc = "Bit 1 - Soft halt boundary control. This function is only available, if \\[FREE\\]
is set to 'STOP'"]
    #[inline(always)]
    pub fn pdbgctl_soft(&mut self) -> PdbgctlSoftW<PdbgctlSpec> {
        PdbgctlSoftW::new(self, 1)
    }
}
#[doc = "Peripheral Debug Control\n\nYou can [`read`](crate::Reg::read) this register and get [`pdbgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdbgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdbgctlSpec;
impl crate::RegisterSpec for PdbgctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdbgctl::R`](R) reader structure"]
impl crate::Readable for PdbgctlSpec {}
#[doc = "`write(|w| ..)` method takes [`pdbgctl::W`](W) writer structure"]
impl crate::Writable for PdbgctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDBGCTL to value 0"]
impl crate::Resettable for PdbgctlSpec {
    const RESET_VALUE: u32 = 0;
}
