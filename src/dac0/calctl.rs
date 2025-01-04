#[doc = "Register `CALCTL` reader"]
pub type R = crate::R<CalctlSpec>;
#[doc = "Register `CALCTL` writer"]
pub type W = crate::W<CalctlSpec>;
#[doc = "This bit when set initiates the DAC offset error calibration sequence and is automatically reset when the offset error calibration completes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalctlCalon {
    #[doc = "0: INACTIVE"]
    CalctlCalonInactive = 0,
    #[doc = "1: ACTIVE"]
    CalctlCalonActive = 1,
}
impl From<CalctlCalon> for bool {
    #[inline(always)]
    fn from(variant: CalctlCalon) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALCTL_CALON` reader - This bit when set initiates the DAC offset error calibration sequence and is automatically reset when the offset error calibration completes."]
pub type CalctlCalonR = crate::BitReader<CalctlCalon>;
impl CalctlCalonR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CalctlCalon {
        match self.bits {
            false => CalctlCalon::CalctlCalonInactive,
            true => CalctlCalon::CalctlCalonActive,
        }
    }
    #[doc = "INACTIVE"]
    #[inline(always)]
    pub fn is_calctl_calon_inactive(&self) -> bool {
        *self == CalctlCalon::CalctlCalonInactive
    }
    #[doc = "ACTIVE"]
    #[inline(always)]
    pub fn is_calctl_calon_active(&self) -> bool {
        *self == CalctlCalon::CalctlCalonActive
    }
}
#[doc = "Field `CALCTL_CALON` writer - This bit when set initiates the DAC offset error calibration sequence and is automatically reset when the offset error calibration completes."]
pub type CalctlCalonW<'a, REG> = crate::BitWriter<'a, REG, CalctlCalon>;
impl<'a, REG> CalctlCalonW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "INACTIVE"]
    #[inline(always)]
    pub fn calctl_calon_inactive(self) -> &'a mut crate::W<REG> {
        self.variant(CalctlCalon::CalctlCalonInactive)
    }
    #[doc = "ACTIVE"]
    #[inline(always)]
    pub fn calctl_calon_active(self) -> &'a mut crate::W<REG> {
        self.variant(CalctlCalon::CalctlCalonActive)
    }
}
#[doc = "This bit is used to select between factory trim or self calibration trim.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CalctlCalsel {
    #[doc = "0: FACTORY_TRIM"]
    CalctlCalselFactorytrim = 0,
    #[doc = "1: SELF_CALIBRATION_TRIM"]
    CalctlCalselSelfcalibrationtrim = 1,
}
impl From<CalctlCalsel> for bool {
    #[inline(always)]
    fn from(variant: CalctlCalsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALCTL_CALSEL` reader - This bit is used to select between factory trim or self calibration trim."]
pub type CalctlCalselR = crate::BitReader<CalctlCalsel>;
impl CalctlCalselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CalctlCalsel {
        match self.bits {
            false => CalctlCalsel::CalctlCalselFactorytrim,
            true => CalctlCalsel::CalctlCalselSelfcalibrationtrim,
        }
    }
    #[doc = "FACTORY_TRIM"]
    #[inline(always)]
    pub fn is_calctl_calsel_factorytrim(&self) -> bool {
        *self == CalctlCalsel::CalctlCalselFactorytrim
    }
    #[doc = "SELF_CALIBRATION_TRIM"]
    #[inline(always)]
    pub fn is_calctl_calsel_selfcalibrationtrim(&self) -> bool {
        *self == CalctlCalsel::CalctlCalselSelfcalibrationtrim
    }
}
#[doc = "Field `CALCTL_CALSEL` writer - This bit is used to select between factory trim or self calibration trim."]
pub type CalctlCalselW<'a, REG> = crate::BitWriter<'a, REG, CalctlCalsel>;
impl<'a, REG> CalctlCalselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FACTORY_TRIM"]
    #[inline(always)]
    pub fn calctl_calsel_factorytrim(self) -> &'a mut crate::W<REG> {
        self.variant(CalctlCalsel::CalctlCalselFactorytrim)
    }
    #[doc = "SELF_CALIBRATION_TRIM"]
    #[inline(always)]
    pub fn calctl_calsel_selfcalibrationtrim(self) -> &'a mut crate::W<REG> {
        self.variant(CalctlCalsel::CalctlCalselSelfcalibrationtrim)
    }
}
impl R {
    #[doc = "Bit 0 - This bit when set initiates the DAC offset error calibration sequence and is automatically reset when the offset error calibration completes."]
    #[inline(always)]
    pub fn calctl_calon(&self) -> CalctlCalonR {
        CalctlCalonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is used to select between factory trim or self calibration trim."]
    #[inline(always)]
    pub fn calctl_calsel(&self) -> CalctlCalselR {
        CalctlCalselR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit when set initiates the DAC offset error calibration sequence and is automatically reset when the offset error calibration completes."]
    #[inline(always)]
    pub fn calctl_calon(&mut self) -> CalctlCalonW<CalctlSpec> {
        CalctlCalonW::new(self, 0)
    }
    #[doc = "Bit 1 - This bit is used to select between factory trim or self calibration trim."]
    #[inline(always)]
    pub fn calctl_calsel(&mut self) -> CalctlCalselW<CalctlSpec> {
        CalctlCalselW::new(self, 1)
    }
}
#[doc = "Calibration control\n\nYou can [`read`](crate::Reg::read) this register and get [`calctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalctlSpec;
impl crate::RegisterSpec for CalctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calctl::R`](R) reader structure"]
impl crate::Readable for CalctlSpec {}
#[doc = "`write(|w| ..)` method takes [`calctl::W`](W) writer structure"]
impl crate::Writable for CalctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CALCTL to value 0"]
impl crate::Resettable for CalctlSpec {
    const RESET_VALUE: u32 = 0;
}
