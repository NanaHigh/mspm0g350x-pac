#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "OAxn Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtlEnable {
    #[doc = "0: OFF"]
    CtlEnableOff = 0,
    #[doc = "1: ON"]
    CtlEnableOn = 1,
}
impl From<CtlEnable> for bool {
    #[inline(always)]
    fn from(variant: CtlEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL_ENABLE` reader - OAxn Enable."]
pub type CtlEnableR = crate::BitReader<CtlEnable>;
impl CtlEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtlEnable {
        match self.bits {
            false => CtlEnable::CtlEnableOff,
            true => CtlEnable::CtlEnableOn,
        }
    }
    #[doc = "OFF"]
    #[inline(always)]
    pub fn is_ctl_enable_off(&self) -> bool {
        *self == CtlEnable::CtlEnableOff
    }
    #[doc = "ON"]
    #[inline(always)]
    pub fn is_ctl_enable_on(&self) -> bool {
        *self == CtlEnable::CtlEnableOn
    }
}
#[doc = "Field `CTL_ENABLE` writer - OAxn Enable."]
pub type CtlEnableW<'a, REG> = crate::BitWriter<'a, REG, CtlEnable>;
impl<'a, REG> CtlEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OFF"]
    #[inline(always)]
    pub fn ctl_enable_off(self) -> &'a mut crate::W<REG> {
        self.variant(CtlEnable::CtlEnableOff)
    }
    #[doc = "ON"]
    #[inline(always)]
    pub fn ctl_enable_on(self) -> &'a mut crate::W<REG> {
        self.variant(CtlEnable::CtlEnableOn)
    }
}
impl R {
    #[doc = "Bit 0 - OAxn Enable."]
    #[inline(always)]
    pub fn ctl_enable(&self) -> CtlEnableR {
        CtlEnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OAxn Enable."]
    #[inline(always)]
    pub fn ctl_enable(&mut self) -> CtlEnableW<CtlSpec> {
        CtlEnableW::new(self, 0)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
