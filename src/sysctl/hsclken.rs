#[doc = "Register `HSCLKEN` reader"]
pub type R = crate::R<HsclkenSpec>;
#[doc = "Register `HSCLKEN` writer"]
pub type W = crate::W<HsclkenSpec>;
#[doc = "HFXTEN enables or disables the high frequency crystal oscillator (HFXT).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsclkenHfxten {
    #[doc = "0: DISABLE"]
    HsclkenHfxtenDisable = 0,
    #[doc = "1: ENABLE"]
    HsclkenHfxtenEnable = 1,
}
impl From<HsclkenHfxten> for bool {
    #[inline(always)]
    fn from(variant: HsclkenHfxten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSCLKEN_HFXTEN` reader - HFXTEN enables or disables the high frequency crystal oscillator (HFXT)."]
pub type HsclkenHfxtenR = crate::BitReader<HsclkenHfxten>;
impl HsclkenHfxtenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsclkenHfxten {
        match self.bits {
            false => HsclkenHfxten::HsclkenHfxtenDisable,
            true => HsclkenHfxten::HsclkenHfxtenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_hsclken_hfxten_disable(&self) -> bool {
        *self == HsclkenHfxten::HsclkenHfxtenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_hsclken_hfxten_enable(&self) -> bool {
        *self == HsclkenHfxten::HsclkenHfxtenEnable
    }
}
#[doc = "Field `HSCLKEN_HFXTEN` writer - HFXTEN enables or disables the high frequency crystal oscillator (HFXT)."]
pub type HsclkenHfxtenW<'a, REG> = crate::BitWriter<'a, REG, HsclkenHfxten>;
impl<'a, REG> HsclkenHfxtenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn hsclken_hfxten_disable(self) -> &'a mut crate::W<REG> {
        self.variant(HsclkenHfxten::HsclkenHfxtenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn hsclken_hfxten_enable(self) -> &'a mut crate::W<REG> {
        self.variant(HsclkenHfxten::HsclkenHfxtenEnable)
    }
}
#[doc = "SYSPLLEN enables or disables the system phase-lock loop (SYSPLL).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsclkenSyspllen {
    #[doc = "0: DISABLE"]
    HsclkenSyspllenDisable = 0,
    #[doc = "1: ENABLE"]
    HsclkenSyspllenEnable = 1,
}
impl From<HsclkenSyspllen> for bool {
    #[inline(always)]
    fn from(variant: HsclkenSyspllen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSCLKEN_SYSPLLEN` reader - SYSPLLEN enables or disables the system phase-lock loop (SYSPLL)."]
pub type HsclkenSyspllenR = crate::BitReader<HsclkenSyspllen>;
impl HsclkenSyspllenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsclkenSyspllen {
        match self.bits {
            false => HsclkenSyspllen::HsclkenSyspllenDisable,
            true => HsclkenSyspllen::HsclkenSyspllenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_hsclken_syspllen_disable(&self) -> bool {
        *self == HsclkenSyspllen::HsclkenSyspllenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_hsclken_syspllen_enable(&self) -> bool {
        *self == HsclkenSyspllen::HsclkenSyspllenEnable
    }
}
#[doc = "Field `HSCLKEN_SYSPLLEN` writer - SYSPLLEN enables or disables the system phase-lock loop (SYSPLL)."]
pub type HsclkenSyspllenW<'a, REG> = crate::BitWriter<'a, REG, HsclkenSyspllen>;
impl<'a, REG> HsclkenSyspllenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn hsclken_syspllen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(HsclkenSyspllen::HsclkenSyspllenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn hsclken_syspllen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(HsclkenSyspllen::HsclkenSyspllenEnable)
    }
}
#[doc = "USEEXTHFCLK selects the HFCLK_IN digital clock input to be the source for HFCLK. When disabled, HFXT is the HFCLK source and HFXTEN may be set. Do not set HFXTEN and USEEXTHFCLK simultaneously.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsclkenUseexthfclk {
    #[doc = "0: DISABLE"]
    HsclkenUseexthfclkDisable = 0,
    #[doc = "1: ENABLE"]
    HsclkenUseexthfclkEnable = 1,
}
impl From<HsclkenUseexthfclk> for bool {
    #[inline(always)]
    fn from(variant: HsclkenUseexthfclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSCLKEN_USEEXTHFCLK` reader - USEEXTHFCLK selects the HFCLK_IN digital clock input to be the source for HFCLK. When disabled, HFXT is the HFCLK source and HFXTEN may be set. Do not set HFXTEN and USEEXTHFCLK simultaneously."]
pub type HsclkenUseexthfclkR = crate::BitReader<HsclkenUseexthfclk>;
impl HsclkenUseexthfclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsclkenUseexthfclk {
        match self.bits {
            false => HsclkenUseexthfclk::HsclkenUseexthfclkDisable,
            true => HsclkenUseexthfclk::HsclkenUseexthfclkEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_hsclken_useexthfclk_disable(&self) -> bool {
        *self == HsclkenUseexthfclk::HsclkenUseexthfclkDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_hsclken_useexthfclk_enable(&self) -> bool {
        *self == HsclkenUseexthfclk::HsclkenUseexthfclkEnable
    }
}
#[doc = "Field `HSCLKEN_USEEXTHFCLK` writer - USEEXTHFCLK selects the HFCLK_IN digital clock input to be the source for HFCLK. When disabled, HFXT is the HFCLK source and HFXTEN may be set. Do not set HFXTEN and USEEXTHFCLK simultaneously."]
pub type HsclkenUseexthfclkW<'a, REG> = crate::BitWriter<'a, REG, HsclkenUseexthfclk>;
impl<'a, REG> HsclkenUseexthfclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn hsclken_useexthfclk_disable(self) -> &'a mut crate::W<REG> {
        self.variant(HsclkenUseexthfclk::HsclkenUseexthfclkDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn hsclken_useexthfclk_enable(self) -> &'a mut crate::W<REG> {
        self.variant(HsclkenUseexthfclk::HsclkenUseexthfclkEnable)
    }
}
impl R {
    #[doc = "Bit 0 - HFXTEN enables or disables the high frequency crystal oscillator (HFXT)."]
    #[inline(always)]
    pub fn hsclken_hfxten(&self) -> HsclkenHfxtenR {
        HsclkenHfxtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - SYSPLLEN enables or disables the system phase-lock loop (SYSPLL)."]
    #[inline(always)]
    pub fn hsclken_syspllen(&self) -> HsclkenSyspllenR {
        HsclkenSyspllenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - USEEXTHFCLK selects the HFCLK_IN digital clock input to be the source for HFCLK. When disabled, HFXT is the HFCLK source and HFXTEN may be set. Do not set HFXTEN and USEEXTHFCLK simultaneously."]
    #[inline(always)]
    pub fn hsclken_useexthfclk(&self) -> HsclkenUseexthfclkR {
        HsclkenUseexthfclkR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFXTEN enables or disables the high frequency crystal oscillator (HFXT)."]
    #[inline(always)]
    pub fn hsclken_hfxten(&mut self) -> HsclkenHfxtenW<HsclkenSpec> {
        HsclkenHfxtenW::new(self, 0)
    }
    #[doc = "Bit 8 - SYSPLLEN enables or disables the system phase-lock loop (SYSPLL)."]
    #[inline(always)]
    pub fn hsclken_syspllen(&mut self) -> HsclkenSyspllenW<HsclkenSpec> {
        HsclkenSyspllenW::new(self, 8)
    }
    #[doc = "Bit 16 - USEEXTHFCLK selects the HFCLK_IN digital clock input to be the source for HFCLK. When disabled, HFXT is the HFCLK source and HFXTEN may be set. Do not set HFXTEN and USEEXTHFCLK simultaneously."]
    #[inline(always)]
    pub fn hsclken_useexthfclk(&mut self) -> HsclkenUseexthfclkW<HsclkenSpec> {
        HsclkenUseexthfclkW::new(self, 16)
    }
}
#[doc = "High-speed clock (HSCLK) source enable/disable\n\nYou can [`read`](crate::Reg::read) this register and get [`hsclken::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsclken::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsclkenSpec;
impl crate::RegisterSpec for HsclkenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsclken::R`](R) reader structure"]
impl crate::Readable for HsclkenSpec {}
#[doc = "`write(|w| ..)` method takes [`hsclken::W`](W) writer structure"]
impl crate::Writable for HsclkenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSCLKEN to value 0"]
impl crate::Resettable for HsclkenSpec {
    const RESET_VALUE: u32 = 0;
}
