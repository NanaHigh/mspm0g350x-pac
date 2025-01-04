#[doc = "Register `SYSOSCTRIMUSER` reader"]
pub type R = crate::R<SysosctrimuserSpec>;
#[doc = "Register `SYSOSCTRIMUSER` writer"]
pub type W = crate::W<SysosctrimuserSpec>;
#[doc = "FREQ specifies the target user-trimmed frequency for SYSOSC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SysosctrimuserFreq {
    #[doc = "1: SYSOSC16M"]
    SysosctrimuserFreqSysosc16m = 1,
    #[doc = "2: SYSOSC24M"]
    SysosctrimuserFreqSysosc24m = 2,
}
impl From<SysosctrimuserFreq> for u8 {
    #[inline(always)]
    fn from(variant: SysosctrimuserFreq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SysosctrimuserFreq {
    type Ux = u8;
}
impl crate::IsEnum for SysosctrimuserFreq {}
#[doc = "Field `SYSOSCTRIMUSER_FREQ` reader - FREQ specifies the target user-trimmed frequency for SYSOSC."]
pub type SysosctrimuserFreqR = crate::FieldReader<SysosctrimuserFreq>;
impl SysosctrimuserFreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SysosctrimuserFreq> {
        match self.bits {
            1 => Some(SysosctrimuserFreq::SysosctrimuserFreqSysosc16m),
            2 => Some(SysosctrimuserFreq::SysosctrimuserFreqSysosc24m),
            _ => None,
        }
    }
    #[doc = "SYSOSC16M"]
    #[inline(always)]
    pub fn is_sysosctrimuser_freq_sysosc16m(&self) -> bool {
        *self == SysosctrimuserFreq::SysosctrimuserFreqSysosc16m
    }
    #[doc = "SYSOSC24M"]
    #[inline(always)]
    pub fn is_sysosctrimuser_freq_sysosc24m(&self) -> bool {
        *self == SysosctrimuserFreq::SysosctrimuserFreqSysosc24m
    }
}
#[doc = "Field `SYSOSCTRIMUSER_FREQ` writer - FREQ specifies the target user-trimmed frequency for SYSOSC."]
pub type SysosctrimuserFreqW<'a, REG> = crate::FieldWriter<'a, REG, 2, SysosctrimuserFreq>;
impl<'a, REG> SysosctrimuserFreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SYSOSC16M"]
    #[inline(always)]
    pub fn sysosctrimuser_freq_sysosc16m(self) -> &'a mut crate::W<REG> {
        self.variant(SysosctrimuserFreq::SysosctrimuserFreqSysosc16m)
    }
    #[doc = "SYSOSC24M"]
    #[inline(always)]
    pub fn sysosctrimuser_freq_sysosc24m(self) -> &'a mut crate::W<REG> {
        self.variant(SysosctrimuserFreq::SysosctrimuserFreqSysosc24m)
    }
}
#[doc = "Field `SYSOSCTRIMUSER_CAP` reader - CAP specifies the SYSOSC capacitor trim. This value changes with the target frequency."]
pub type SysosctrimuserCapR = crate::FieldReader;
#[doc = "Field `SYSOSCTRIMUSER_CAP` writer - CAP specifies the SYSOSC capacitor trim. This value changes with the target frequency."]
pub type SysosctrimuserCapW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SYSOSCTRIMUSER_RESCOARSE` reader - RESCOARSE specifies the resister coarse trim. This value changes with the target frequency."]
pub type SysosctrimuserRescoarseR = crate::FieldReader;
#[doc = "Field `SYSOSCTRIMUSER_RESCOARSE` writer - RESCOARSE specifies the resister coarse trim. This value changes with the target frequency."]
pub type SysosctrimuserRescoarseW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SYSOSCTRIMUSER_RESFINE` reader - RESFINE specifies the resister fine trim. This value changes with the target frequency."]
pub type SysosctrimuserResfineR = crate::FieldReader;
#[doc = "Field `SYSOSCTRIMUSER_RESFINE` writer - RESFINE specifies the resister fine trim. This value changes with the target frequency."]
pub type SysosctrimuserResfineW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SYSOSCTRIMUSER_RDIV` reader - RDIV specifies the frequency correction loop (FCL) resistor trim. This value changes with the target frequency."]
pub type SysosctrimuserRdivR = crate::FieldReader<u16>;
#[doc = "Field `SYSOSCTRIMUSER_RDIV` writer - RDIV specifies the frequency correction loop (FCL) resistor trim. This value changes with the target frequency."]
pub type SysosctrimuserRdivW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:1 - FREQ specifies the target user-trimmed frequency for SYSOSC."]
    #[inline(always)]
    pub fn sysosctrimuser_freq(&self) -> SysosctrimuserFreqR {
        SysosctrimuserFreqR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - CAP specifies the SYSOSC capacitor trim. This value changes with the target frequency."]
    #[inline(always)]
    pub fn sysosctrimuser_cap(&self) -> SysosctrimuserCapR {
        SysosctrimuserCapR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:13 - RESCOARSE specifies the resister coarse trim. This value changes with the target frequency."]
    #[inline(always)]
    pub fn sysosctrimuser_rescoarse(&self) -> SysosctrimuserRescoarseR {
        SysosctrimuserRescoarseR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - RESFINE specifies the resister fine trim. This value changes with the target frequency."]
    #[inline(always)]
    pub fn sysosctrimuser_resfine(&self) -> SysosctrimuserResfineR {
        SysosctrimuserResfineR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:28 - RDIV specifies the frequency correction loop (FCL) resistor trim. This value changes with the target frequency."]
    #[inline(always)]
    pub fn sysosctrimuser_rdiv(&self) -> SysosctrimuserRdivR {
        SysosctrimuserRdivR::new(((self.bits >> 20) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - FREQ specifies the target user-trimmed frequency for SYSOSC."]
    #[inline(always)]
    pub fn sysosctrimuser_freq(&mut self) -> SysosctrimuserFreqW<SysosctrimuserSpec> {
        SysosctrimuserFreqW::new(self, 0)
    }
    #[doc = "Bits 4:6 - CAP specifies the SYSOSC capacitor trim. This value changes with the target frequency."]
    #[inline(always)]
    pub fn sysosctrimuser_cap(&mut self) -> SysosctrimuserCapW<SysosctrimuserSpec> {
        SysosctrimuserCapW::new(self, 4)
    }
    #[doc = "Bits 8:13 - RESCOARSE specifies the resister coarse trim. This value changes with the target frequency."]
    #[inline(always)]
    pub fn sysosctrimuser_rescoarse(&mut self) -> SysosctrimuserRescoarseW<SysosctrimuserSpec> {
        SysosctrimuserRescoarseW::new(self, 8)
    }
    #[doc = "Bits 16:19 - RESFINE specifies the resister fine trim. This value changes with the target frequency."]
    #[inline(always)]
    pub fn sysosctrimuser_resfine(&mut self) -> SysosctrimuserResfineW<SysosctrimuserSpec> {
        SysosctrimuserResfineW::new(self, 16)
    }
    #[doc = "Bits 20:28 - RDIV specifies the frequency correction loop (FCL) resistor trim. This value changes with the target frequency."]
    #[inline(always)]
    pub fn sysosctrimuser_rdiv(&mut self) -> SysosctrimuserRdivW<SysosctrimuserSpec> {
        SysosctrimuserRdivW::new(self, 20)
    }
}
#[doc = "SYSOSC user-specified trim\n\nYou can [`read`](crate::Reg::read) this register and get [`sysosctrimuser::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysosctrimuser::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysosctrimuserSpec;
impl crate::RegisterSpec for SysosctrimuserSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sysosctrimuser::R`](R) reader structure"]
impl crate::Readable for SysosctrimuserSpec {}
#[doc = "`write(|w| ..)` method takes [`sysosctrimuser::W`](W) writer structure"]
impl crate::Writable for SysosctrimuserSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SYSOSCTRIMUSER to value 0"]
impl crate::Resettable for SysosctrimuserSpec {
    const RESET_VALUE: u32 = 0;
}
