#[doc = "Register `CLKCTL` reader"]
pub type R = crate::R<ClkctlSpec>;
#[doc = "Register `CLKCTL` writer"]
pub type W = crate::W<ClkctlSpec>;
#[doc = "This bit enables the supply of the 32kHz clock to the RTC. It will not power-up the 32kHz crystal oscillator this needs to be done in the Clock System Module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkctlModclken {
    #[doc = "0: DISABLE"]
    ClkctlModclkenDisable = 0,
    #[doc = "1: ENABLE"]
    ClkctlModclkenEnable = 1,
}
impl From<ClkctlModclken> for bool {
    #[inline(always)]
    fn from(variant: ClkctlModclken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKCTL_MODCLKEN` reader - This bit enables the supply of the 32kHz clock to the RTC. It will not power-up the 32kHz crystal oscillator this needs to be done in the Clock System Module."]
pub type ClkctlModclkenR = crate::BitReader<ClkctlModclken>;
impl ClkctlModclkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkctlModclken {
        match self.bits {
            false => ClkctlModclken::ClkctlModclkenDisable,
            true => ClkctlModclken::ClkctlModclkenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_clkctl_modclken_disable(&self) -> bool {
        *self == ClkctlModclken::ClkctlModclkenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_clkctl_modclken_enable(&self) -> bool {
        *self == ClkctlModclken::ClkctlModclkenEnable
    }
}
#[doc = "Field `CLKCTL_MODCLKEN` writer - This bit enables the supply of the 32kHz clock to the RTC. It will not power-up the 32kHz crystal oscillator this needs to be done in the Clock System Module."]
pub type ClkctlModclkenW<'a, REG> = crate::BitWriter<'a, REG, ClkctlModclken>;
impl<'a, REG> ClkctlModclkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn clkctl_modclken_disable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkctlModclken::ClkctlModclkenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn clkctl_modclken_enable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkctlModclken::ClkctlModclkenEnable)
    }
}
impl R {
    #[doc = "Bit 31 - This bit enables the supply of the 32kHz clock to the RTC. It will not power-up the 32kHz crystal oscillator this needs to be done in the Clock System Module."]
    #[inline(always)]
    pub fn clkctl_modclken(&self) -> ClkctlModclkenR {
        ClkctlModclkenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - This bit enables the supply of the 32kHz clock to the RTC. It will not power-up the 32kHz crystal oscillator this needs to be done in the Clock System Module."]
    #[inline(always)]
    pub fn clkctl_modclken(&mut self) -> ClkctlModclkenW<ClkctlSpec> {
        ClkctlModclkenW::new(self, 31)
    }
}
#[doc = "RTC Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkctlSpec;
impl crate::RegisterSpec for ClkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctl::R`](R) reader structure"]
impl crate::Readable for ClkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkctl::W`](W) writer structure"]
impl crate::Writable for ClkctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCTL to value 0"]
impl crate::Resettable for ClkctlSpec {
    const RESET_VALUE: u32 = 0;
}
