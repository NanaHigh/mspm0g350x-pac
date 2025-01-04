#[doc = "Register `RESETLEVEL` reader"]
pub type R = crate::R<ResetlevelSpec>;
#[doc = "Register `RESETLEVEL` writer"]
pub type W = crate::W<ResetlevelSpec>;
#[doc = "LEVEL is used to specify the type of reset to be issued when RESETCMD is set to generate a software triggered reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ResetlevelLevel {
    #[doc = "0: CPU"]
    ResetlevelLevelCpu = 0,
    #[doc = "1: BOOT"]
    ResetlevelLevelBoot = 1,
    #[doc = "2: BOOTLOADERENTRY"]
    ResetlevelLevelBootloaderentry = 2,
    #[doc = "3: POR"]
    ResetlevelLevelPor = 3,
    #[doc = "4: BOOTLOADEREXIT"]
    ResetlevelLevelBootloaderexit = 4,
}
impl From<ResetlevelLevel> for u8 {
    #[inline(always)]
    fn from(variant: ResetlevelLevel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ResetlevelLevel {
    type Ux = u8;
}
impl crate::IsEnum for ResetlevelLevel {}
#[doc = "Field `RESETLEVEL_LEVEL` reader - LEVEL is used to specify the type of reset to be issued when RESETCMD is set to generate a software triggered reset."]
pub type ResetlevelLevelR = crate::FieldReader<ResetlevelLevel>;
impl ResetlevelLevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ResetlevelLevel> {
        match self.bits {
            0 => Some(ResetlevelLevel::ResetlevelLevelCpu),
            1 => Some(ResetlevelLevel::ResetlevelLevelBoot),
            2 => Some(ResetlevelLevel::ResetlevelLevelBootloaderentry),
            3 => Some(ResetlevelLevel::ResetlevelLevelPor),
            4 => Some(ResetlevelLevel::ResetlevelLevelBootloaderexit),
            _ => None,
        }
    }
    #[doc = "CPU"]
    #[inline(always)]
    pub fn is_resetlevel_level_cpu(&self) -> bool {
        *self == ResetlevelLevel::ResetlevelLevelCpu
    }
    #[doc = "BOOT"]
    #[inline(always)]
    pub fn is_resetlevel_level_boot(&self) -> bool {
        *self == ResetlevelLevel::ResetlevelLevelBoot
    }
    #[doc = "BOOTLOADERENTRY"]
    #[inline(always)]
    pub fn is_resetlevel_level_bootloaderentry(&self) -> bool {
        *self == ResetlevelLevel::ResetlevelLevelBootloaderentry
    }
    #[doc = "POR"]
    #[inline(always)]
    pub fn is_resetlevel_level_por(&self) -> bool {
        *self == ResetlevelLevel::ResetlevelLevelPor
    }
    #[doc = "BOOTLOADEREXIT"]
    #[inline(always)]
    pub fn is_resetlevel_level_bootloaderexit(&self) -> bool {
        *self == ResetlevelLevel::ResetlevelLevelBootloaderexit
    }
}
#[doc = "Field `RESETLEVEL_LEVEL` writer - LEVEL is used to specify the type of reset to be issued when RESETCMD is set to generate a software triggered reset."]
pub type ResetlevelLevelW<'a, REG> = crate::FieldWriter<'a, REG, 3, ResetlevelLevel>;
impl<'a, REG> ResetlevelLevelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CPU"]
    #[inline(always)]
    pub fn resetlevel_level_cpu(self) -> &'a mut crate::W<REG> {
        self.variant(ResetlevelLevel::ResetlevelLevelCpu)
    }
    #[doc = "BOOT"]
    #[inline(always)]
    pub fn resetlevel_level_boot(self) -> &'a mut crate::W<REG> {
        self.variant(ResetlevelLevel::ResetlevelLevelBoot)
    }
    #[doc = "BOOTLOADERENTRY"]
    #[inline(always)]
    pub fn resetlevel_level_bootloaderentry(self) -> &'a mut crate::W<REG> {
        self.variant(ResetlevelLevel::ResetlevelLevelBootloaderentry)
    }
    #[doc = "POR"]
    #[inline(always)]
    pub fn resetlevel_level_por(self) -> &'a mut crate::W<REG> {
        self.variant(ResetlevelLevel::ResetlevelLevelPor)
    }
    #[doc = "BOOTLOADEREXIT"]
    #[inline(always)]
    pub fn resetlevel_level_bootloaderexit(self) -> &'a mut crate::W<REG> {
        self.variant(ResetlevelLevel::ResetlevelLevelBootloaderexit)
    }
}
impl R {
    #[doc = "Bits 0:2 - LEVEL is used to specify the type of reset to be issued when RESETCMD is set to generate a software triggered reset."]
    #[inline(always)]
    pub fn resetlevel_level(&self) -> ResetlevelLevelR {
        ResetlevelLevelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - LEVEL is used to specify the type of reset to be issued when RESETCMD is set to generate a software triggered reset."]
    #[inline(always)]
    pub fn resetlevel_level(&mut self) -> ResetlevelLevelW<ResetlevelSpec> {
        ResetlevelLevelW::new(self, 0)
    }
}
#[doc = "Reset level for application-triggered reset command\n\nYou can [`read`](crate::Reg::read) this register and get [`resetlevel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`resetlevel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetlevelSpec;
impl crate::RegisterSpec for ResetlevelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`resetlevel::R`](R) reader structure"]
impl crate::Readable for ResetlevelSpec {}
#[doc = "`write(|w| ..)` method takes [`resetlevel::W`](W) writer structure"]
impl crate::Writable for ResetlevelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESETLEVEL to value 0"]
impl crate::Resettable for ResetlevelSpec {
    const RESET_VALUE: u32 = 0;
}
