#[doc = "Register `CLKOVR` reader"]
pub type R = crate::R<ClkovrSpec>;
#[doc = "Register `CLKOVR` writer"]
pub type W = crate::W<ClkovrSpec>;
#[doc = "Unlocks the functionality of \\[RUN_STOP\\]
to override the automatic peripheral clock request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkovrOverride {
    #[doc = "0: DISABLED"]
    ClkovrOverrideDisabled = 0,
    #[doc = "1: ENABLED"]
    ClkovrOverrideEnabled = 1,
}
impl From<ClkovrOverride> for bool {
    #[inline(always)]
    fn from(variant: ClkovrOverride) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKOVR_OVERRIDE` reader - Unlocks the functionality of \\[RUN_STOP\\]
to override the automatic peripheral clock request"]
pub type ClkovrOverrideR = crate::BitReader<ClkovrOverride>;
impl ClkovrOverrideR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkovrOverride {
        match self.bits {
            false => ClkovrOverride::ClkovrOverrideDisabled,
            true => ClkovrOverride::ClkovrOverrideEnabled,
        }
    }
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn is_clkovr_override_disabled(&self) -> bool {
        *self == ClkovrOverride::ClkovrOverrideDisabled
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn is_clkovr_override_enabled(&self) -> bool {
        *self == ClkovrOverride::ClkovrOverrideEnabled
    }
}
#[doc = "Field `CLKOVR_OVERRIDE` writer - Unlocks the functionality of \\[RUN_STOP\\]
to override the automatic peripheral clock request"]
pub type ClkovrOverrideW<'a, REG> = crate::BitWriter<'a, REG, ClkovrOverride>;
impl<'a, REG> ClkovrOverrideW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLED"]
    #[inline(always)]
    pub fn clkovr_override_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ClkovrOverride::ClkovrOverrideDisabled)
    }
    #[doc = "ENABLED"]
    #[inline(always)]
    pub fn clkovr_override_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ClkovrOverride::ClkovrOverrideEnabled)
    }
}
#[doc = "If \\[OVERRIDE\\]
is enabled, this register is used to manually control the peripheral's clock request to the system\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkovrRunStop {
    #[doc = "0: RUN"]
    ClkovrRunStopRun = 0,
    #[doc = "1: STOP"]
    ClkovrRunStopStop = 1,
}
impl From<ClkovrRunStop> for bool {
    #[inline(always)]
    fn from(variant: ClkovrRunStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKOVR_RUN_STOP` reader - If \\[OVERRIDE\\]
is enabled, this register is used to manually control the peripheral's clock request to the system"]
pub type ClkovrRunStopR = crate::BitReader<ClkovrRunStop>;
impl ClkovrRunStopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkovrRunStop {
        match self.bits {
            false => ClkovrRunStop::ClkovrRunStopRun,
            true => ClkovrRunStop::ClkovrRunStopStop,
        }
    }
    #[doc = "RUN"]
    #[inline(always)]
    pub fn is_clkovr_run_stop_run(&self) -> bool {
        *self == ClkovrRunStop::ClkovrRunStopRun
    }
    #[doc = "STOP"]
    #[inline(always)]
    pub fn is_clkovr_run_stop_stop(&self) -> bool {
        *self == ClkovrRunStop::ClkovrRunStopStop
    }
}
#[doc = "Field `CLKOVR_RUN_STOP` writer - If \\[OVERRIDE\\]
is enabled, this register is used to manually control the peripheral's clock request to the system"]
pub type ClkovrRunStopW<'a, REG> = crate::BitWriter<'a, REG, ClkovrRunStop>;
impl<'a, REG> ClkovrRunStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RUN"]
    #[inline(always)]
    pub fn clkovr_run_stop_run(self) -> &'a mut crate::W<REG> {
        self.variant(ClkovrRunStop::ClkovrRunStopRun)
    }
    #[doc = "STOP"]
    #[inline(always)]
    pub fn clkovr_run_stop_stop(self) -> &'a mut crate::W<REG> {
        self.variant(ClkovrRunStop::ClkovrRunStopStop)
    }
}
impl R {
    #[doc = "Bit 0 - Unlocks the functionality of \\[RUN_STOP\\]
to override the automatic peripheral clock request"]
    #[inline(always)]
    pub fn clkovr_override(&self) -> ClkovrOverrideR {
        ClkovrOverrideR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - If \\[OVERRIDE\\]
is enabled, this register is used to manually control the peripheral's clock request to the system"]
    #[inline(always)]
    pub fn clkovr_run_stop(&self) -> ClkovrRunStopR {
        ClkovrRunStopR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Unlocks the functionality of \\[RUN_STOP\\]
to override the automatic peripheral clock request"]
    #[inline(always)]
    pub fn clkovr_override(&mut self) -> ClkovrOverrideW<ClkovrSpec> {
        ClkovrOverrideW::new(self, 0)
    }
    #[doc = "Bit 1 - If \\[OVERRIDE\\]
is enabled, this register is used to manually control the peripheral's clock request to the system"]
    #[inline(always)]
    pub fn clkovr_run_stop(&mut self) -> ClkovrRunStopW<ClkovrSpec> {
        ClkovrRunStopW::new(self, 1)
    }
}
#[doc = "Clock Override\n\nYou can [`read`](crate::Reg::read) this register and get [`clkovr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkovr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkovrSpec;
impl crate::RegisterSpec for ClkovrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkovr::R`](R) reader structure"]
impl crate::Readable for ClkovrSpec {}
#[doc = "`write(|w| ..)` method takes [`clkovr::W`](W) writer structure"]
impl crate::Writable for ClkovrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKOVR to value 0"]
impl crate::Resettable for ClkovrSpec {
    const RESET_VALUE: u32 = 0;
}
