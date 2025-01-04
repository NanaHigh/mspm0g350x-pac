#[doc = "Register `SYSTEMCFG` reader"]
pub type R = crate::R<SystemcfgSpec>;
#[doc = "Register `SYSTEMCFG` writer"]
pub type W = crate::W<SystemcfgSpec>;
#[doc = "WWDTLP0RSTDIS specifies whether a WWDT Error Event will trigger a BOOTRST or an NMI.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SystemcfgWwdtlp0rstdis {
    #[doc = "0: FALSE"]
    SystemcfgWwdtlp0rstdisFalse = 0,
    #[doc = "1: TRUE"]
    SystemcfgWwdtlp0rstdisTrue = 1,
}
impl From<SystemcfgWwdtlp0rstdis> for bool {
    #[inline(always)]
    fn from(variant: SystemcfgWwdtlp0rstdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSTEMCFG_WWDTLP0RSTDIS` reader - WWDTLP0RSTDIS specifies whether a WWDT Error Event will trigger a BOOTRST or an NMI."]
pub type SystemcfgWwdtlp0rstdisR = crate::BitReader<SystemcfgWwdtlp0rstdis>;
impl SystemcfgWwdtlp0rstdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SystemcfgWwdtlp0rstdis {
        match self.bits {
            false => SystemcfgWwdtlp0rstdis::SystemcfgWwdtlp0rstdisFalse,
            true => SystemcfgWwdtlp0rstdis::SystemcfgWwdtlp0rstdisTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_systemcfg_wwdtlp0rstdis_false(&self) -> bool {
        *self == SystemcfgWwdtlp0rstdis::SystemcfgWwdtlp0rstdisFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_systemcfg_wwdtlp0rstdis_true(&self) -> bool {
        *self == SystemcfgWwdtlp0rstdis::SystemcfgWwdtlp0rstdisTrue
    }
}
#[doc = "Field `SYSTEMCFG_WWDTLP0RSTDIS` writer - WWDTLP0RSTDIS specifies whether a WWDT Error Event will trigger a BOOTRST or an NMI."]
pub type SystemcfgWwdtlp0rstdisW<'a, REG> = crate::BitWriter<'a, REG, SystemcfgWwdtlp0rstdis>;
impl<'a, REG> SystemcfgWwdtlp0rstdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn systemcfg_wwdtlp0rstdis_false(self) -> &'a mut crate::W<REG> {
        self.variant(SystemcfgWwdtlp0rstdis::SystemcfgWwdtlp0rstdisFalse)
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn systemcfg_wwdtlp0rstdis_true(self) -> &'a mut crate::W<REG> {
        self.variant(SystemcfgWwdtlp0rstdis::SystemcfgWwdtlp0rstdisTrue)
    }
}
#[doc = "WWDTLP1RSTDIS specifies whether a WWDT Error Event will trigger a SYSRST or an NMI.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SystemcfgWwdtlp1rstdis {
    #[doc = "0: FALSE"]
    SystemcfgWwdtlp1rstdisFalse = 0,
    #[doc = "1: TRUE"]
    SystemcfgWwdtlp1rstdisTrue = 1,
}
impl From<SystemcfgWwdtlp1rstdis> for bool {
    #[inline(always)]
    fn from(variant: SystemcfgWwdtlp1rstdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSTEMCFG_WWDTLP1RSTDIS` reader - WWDTLP1RSTDIS specifies whether a WWDT Error Event will trigger a SYSRST or an NMI."]
pub type SystemcfgWwdtlp1rstdisR = crate::BitReader<SystemcfgWwdtlp1rstdis>;
impl SystemcfgWwdtlp1rstdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SystemcfgWwdtlp1rstdis {
        match self.bits {
            false => SystemcfgWwdtlp1rstdis::SystemcfgWwdtlp1rstdisFalse,
            true => SystemcfgWwdtlp1rstdis::SystemcfgWwdtlp1rstdisTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_systemcfg_wwdtlp1rstdis_false(&self) -> bool {
        *self == SystemcfgWwdtlp1rstdis::SystemcfgWwdtlp1rstdisFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_systemcfg_wwdtlp1rstdis_true(&self) -> bool {
        *self == SystemcfgWwdtlp1rstdis::SystemcfgWwdtlp1rstdisTrue
    }
}
#[doc = "Field `SYSTEMCFG_WWDTLP1RSTDIS` writer - WWDTLP1RSTDIS specifies whether a WWDT Error Event will trigger a SYSRST or an NMI."]
pub type SystemcfgWwdtlp1rstdisW<'a, REG> = crate::BitWriter<'a, REG, SystemcfgWwdtlp1rstdis>;
impl<'a, REG> SystemcfgWwdtlp1rstdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn systemcfg_wwdtlp1rstdis_false(self) -> &'a mut crate::W<REG> {
        self.variant(SystemcfgWwdtlp1rstdis::SystemcfgWwdtlp1rstdisFalse)
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn systemcfg_wwdtlp1rstdis_true(self) -> &'a mut crate::W<REG> {
        self.variant(SystemcfgWwdtlp1rstdis::SystemcfgWwdtlp1rstdisTrue)
    }
}
#[doc = "FLASHECCRSTDIS specifies whether a flash ECC double error detect (DED) will trigger a SYSRST or an NMI.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SystemcfgFlasheccrstdis {
    #[doc = "0: FALSE"]
    SystemcfgFlasheccrstdisFalse = 0,
    #[doc = "1: TRUE"]
    SystemcfgFlasheccrstdisTrue = 1,
}
impl From<SystemcfgFlasheccrstdis> for bool {
    #[inline(always)]
    fn from(variant: SystemcfgFlasheccrstdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSTEMCFG_FLASHECCRSTDIS` reader - FLASHECCRSTDIS specifies whether a flash ECC double error detect (DED) will trigger a SYSRST or an NMI."]
pub type SystemcfgFlasheccrstdisR = crate::BitReader<SystemcfgFlasheccrstdis>;
impl SystemcfgFlasheccrstdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SystemcfgFlasheccrstdis {
        match self.bits {
            false => SystemcfgFlasheccrstdis::SystemcfgFlasheccrstdisFalse,
            true => SystemcfgFlasheccrstdis::SystemcfgFlasheccrstdisTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_systemcfg_flasheccrstdis_false(&self) -> bool {
        *self == SystemcfgFlasheccrstdis::SystemcfgFlasheccrstdisFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_systemcfg_flasheccrstdis_true(&self) -> bool {
        *self == SystemcfgFlasheccrstdis::SystemcfgFlasheccrstdisTrue
    }
}
#[doc = "Field `SYSTEMCFG_FLASHECCRSTDIS` writer - FLASHECCRSTDIS specifies whether a flash ECC double error detect (DED) will trigger a SYSRST or an NMI."]
pub type SystemcfgFlasheccrstdisW<'a, REG> = crate::BitWriter<'a, REG, SystemcfgFlasheccrstdis>;
impl<'a, REG> SystemcfgFlasheccrstdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn systemcfg_flasheccrstdis_false(self) -> &'a mut crate::W<REG> {
        self.variant(SystemcfgFlasheccrstdis::SystemcfgFlasheccrstdisFalse)
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn systemcfg_flasheccrstdis_true(self) -> &'a mut crate::W<REG> {
        self.variant(SystemcfgFlasheccrstdis::SystemcfgFlasheccrstdisTrue)
    }
}
#[doc = "The key value of 1Bh (27) must be written to KEY together with contents to be updated. Reads as 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SystemcfgKey {
    #[doc = "27: VALUE"]
    SystemcfgKeyValue = 27,
}
impl From<SystemcfgKey> for u8 {
    #[inline(always)]
    fn from(variant: SystemcfgKey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SystemcfgKey {
    type Ux = u8;
}
impl crate::IsEnum for SystemcfgKey {}
#[doc = "Field `SYSTEMCFG_KEY` writer - The key value of 1Bh (27) must be written to KEY together with contents to be updated. Reads as 0"]
pub type SystemcfgKeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, SystemcfgKey>;
impl<'a, REG> SystemcfgKeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VALUE"]
    #[inline(always)]
    pub fn systemcfg_key_value(self) -> &'a mut crate::W<REG> {
        self.variant(SystemcfgKey::SystemcfgKeyValue)
    }
}
impl R {
    #[doc = "Bit 0 - WWDTLP0RSTDIS specifies whether a WWDT Error Event will trigger a BOOTRST or an NMI."]
    #[inline(always)]
    pub fn systemcfg_wwdtlp0rstdis(&self) -> SystemcfgWwdtlp0rstdisR {
        SystemcfgWwdtlp0rstdisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WWDTLP1RSTDIS specifies whether a WWDT Error Event will trigger a SYSRST or an NMI."]
    #[inline(always)]
    pub fn systemcfg_wwdtlp1rstdis(&self) -> SystemcfgWwdtlp1rstdisR {
        SystemcfgWwdtlp1rstdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FLASHECCRSTDIS specifies whether a flash ECC double error detect (DED) will trigger a SYSRST or an NMI."]
    #[inline(always)]
    pub fn systemcfg_flasheccrstdis(&self) -> SystemcfgFlasheccrstdisR {
        SystemcfgFlasheccrstdisR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WWDTLP0RSTDIS specifies whether a WWDT Error Event will trigger a BOOTRST or an NMI."]
    #[inline(always)]
    pub fn systemcfg_wwdtlp0rstdis(&mut self) -> SystemcfgWwdtlp0rstdisW<SystemcfgSpec> {
        SystemcfgWwdtlp0rstdisW::new(self, 0)
    }
    #[doc = "Bit 1 - WWDTLP1RSTDIS specifies whether a WWDT Error Event will trigger a SYSRST or an NMI."]
    #[inline(always)]
    pub fn systemcfg_wwdtlp1rstdis(&mut self) -> SystemcfgWwdtlp1rstdisW<SystemcfgSpec> {
        SystemcfgWwdtlp1rstdisW::new(self, 1)
    }
    #[doc = "Bit 2 - FLASHECCRSTDIS specifies whether a flash ECC double error detect (DED) will trigger a SYSRST or an NMI."]
    #[inline(always)]
    pub fn systemcfg_flasheccrstdis(&mut self) -> SystemcfgFlasheccrstdisW<SystemcfgSpec> {
        SystemcfgFlasheccrstdisW::new(self, 2)
    }
    #[doc = "Bits 24:31 - The key value of 1Bh (27) must be written to KEY together with contents to be updated. Reads as 0"]
    #[inline(always)]
    pub fn systemcfg_key(&mut self) -> SystemcfgKeyW<SystemcfgSpec> {
        SystemcfgKeyW::new(self, 24)
    }
}
#[doc = "System configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`systemcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`systemcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SystemcfgSpec;
impl crate::RegisterSpec for SystemcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`systemcfg::R`](R) reader structure"]
impl crate::Readable for SystemcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`systemcfg::W`](W) writer structure"]
impl crate::Writable for SystemcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSTEMCFG to value 0x06"]
impl crate::Resettable for SystemcfgSpec {
    const RESET_VALUE: u32 = 0x06;
}
