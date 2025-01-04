#[doc = "Register `CLKCFG` reader"]
pub type R = crate::R<ClkcfgSpec>;
#[doc = "Register `CLKCFG` writer"]
pub type W = crate::W<ClkcfgSpec>;
#[doc = "ADC sample clock source selection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkcfgSampclk {
    #[doc = "0: SYSOSC"]
    ClkcfgSampclkSysosc = 0,
    #[doc = "1: ULPCLK"]
    ClkcfgSampclkUlpclk = 1,
    #[doc = "2: HFCLK"]
    ClkcfgSampclkHfclk = 2,
}
impl From<ClkcfgSampclk> for u8 {
    #[inline(always)]
    fn from(variant: ClkcfgSampclk) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkcfgSampclk {
    type Ux = u8;
}
impl crate::IsEnum for ClkcfgSampclk {}
#[doc = "Field `CLKCFG_SAMPCLK` reader - ADC sample clock source selection."]
pub type ClkcfgSampclkR = crate::FieldReader<ClkcfgSampclk>;
impl ClkcfgSampclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkcfgSampclk> {
        match self.bits {
            0 => Some(ClkcfgSampclk::ClkcfgSampclkSysosc),
            1 => Some(ClkcfgSampclk::ClkcfgSampclkUlpclk),
            2 => Some(ClkcfgSampclk::ClkcfgSampclkHfclk),
            _ => None,
        }
    }
    #[doc = "SYSOSC"]
    #[inline(always)]
    pub fn is_clkcfg_sampclk_sysosc(&self) -> bool {
        *self == ClkcfgSampclk::ClkcfgSampclkSysosc
    }
    #[doc = "ULPCLK"]
    #[inline(always)]
    pub fn is_clkcfg_sampclk_ulpclk(&self) -> bool {
        *self == ClkcfgSampclk::ClkcfgSampclkUlpclk
    }
    #[doc = "HFCLK"]
    #[inline(always)]
    pub fn is_clkcfg_sampclk_hfclk(&self) -> bool {
        *self == ClkcfgSampclk::ClkcfgSampclkHfclk
    }
}
#[doc = "Field `CLKCFG_SAMPCLK` writer - ADC sample clock source selection."]
pub type ClkcfgSampclkW<'a, REG> = crate::FieldWriter<'a, REG, 2, ClkcfgSampclk>;
impl<'a, REG> ClkcfgSampclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SYSOSC"]
    #[inline(always)]
    pub fn clkcfg_sampclk_sysosc(self) -> &'a mut crate::W<REG> {
        self.variant(ClkcfgSampclk::ClkcfgSampclkSysosc)
    }
    #[doc = "ULPCLK"]
    #[inline(always)]
    pub fn clkcfg_sampclk_ulpclk(self) -> &'a mut crate::W<REG> {
        self.variant(ClkcfgSampclk::ClkcfgSampclkUlpclk)
    }
    #[doc = "HFCLK"]
    #[inline(always)]
    pub fn clkcfg_sampclk_hfclk(self) -> &'a mut crate::W<REG> {
        self.variant(ClkcfgSampclk::ClkcfgSampclkHfclk)
    }
}
#[doc = "CCONRUN: Forces SYSOSC to run at base frequency when device is in RUN mode which can be used as ADC sample or conversion clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkcfgCconrun {
    #[doc = "0: DISABLE"]
    ClkcfgCconrunDisable = 0,
    #[doc = "1: ENABLE"]
    ClkcfgCconrunEnable = 1,
}
impl From<ClkcfgCconrun> for bool {
    #[inline(always)]
    fn from(variant: ClkcfgCconrun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKCFG_CCONRUN` reader - CCONRUN: Forces SYSOSC to run at base frequency when device is in RUN mode which can be used as ADC sample or conversion clock source."]
pub type ClkcfgCconrunR = crate::BitReader<ClkcfgCconrun>;
impl ClkcfgCconrunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkcfgCconrun {
        match self.bits {
            false => ClkcfgCconrun::ClkcfgCconrunDisable,
            true => ClkcfgCconrun::ClkcfgCconrunEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_clkcfg_cconrun_disable(&self) -> bool {
        *self == ClkcfgCconrun::ClkcfgCconrunDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_clkcfg_cconrun_enable(&self) -> bool {
        *self == ClkcfgCconrun::ClkcfgCconrunEnable
    }
}
#[doc = "Field `CLKCFG_CCONRUN` writer - CCONRUN: Forces SYSOSC to run at base frequency when device is in RUN mode which can be used as ADC sample or conversion clock source."]
pub type ClkcfgCconrunW<'a, REG> = crate::BitWriter<'a, REG, ClkcfgCconrun>;
impl<'a, REG> ClkcfgCconrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn clkcfg_cconrun_disable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkcfgCconrun::ClkcfgCconrunDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn clkcfg_cconrun_enable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkcfgCconrun::ClkcfgCconrunEnable)
    }
}
#[doc = "CCONSTOP: Forces SYSOSC to run at base frequency when device is in STOP mode which can be used as ADC sample or conversion clock source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkcfgCconstop {
    #[doc = "0: DISABLE"]
    ClkcfgCconstopDisable = 0,
    #[doc = "1: ENABLE"]
    ClkcfgCconstopEnable = 1,
}
impl From<ClkcfgCconstop> for bool {
    #[inline(always)]
    fn from(variant: ClkcfgCconstop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKCFG_CCONSTOP` reader - CCONSTOP: Forces SYSOSC to run at base frequency when device is in STOP mode which can be used as ADC sample or conversion clock source."]
pub type ClkcfgCconstopR = crate::BitReader<ClkcfgCconstop>;
impl ClkcfgCconstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkcfgCconstop {
        match self.bits {
            false => ClkcfgCconstop::ClkcfgCconstopDisable,
            true => ClkcfgCconstop::ClkcfgCconstopEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_clkcfg_cconstop_disable(&self) -> bool {
        *self == ClkcfgCconstop::ClkcfgCconstopDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_clkcfg_cconstop_enable(&self) -> bool {
        *self == ClkcfgCconstop::ClkcfgCconstopEnable
    }
}
#[doc = "Field `CLKCFG_CCONSTOP` writer - CCONSTOP: Forces SYSOSC to run at base frequency when device is in STOP mode which can be used as ADC sample or conversion clock source."]
pub type ClkcfgCconstopW<'a, REG> = crate::BitWriter<'a, REG, ClkcfgCconstop>;
impl<'a, REG> ClkcfgCconstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn clkcfg_cconstop_disable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkcfgCconstop::ClkcfgCconstopDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn clkcfg_cconstop_enable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkcfgCconstop::ClkcfgCconstopEnable)
    }
}
#[doc = "Unlock key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkcfgKey {
    #[doc = "169: _TO_UNLOCK_W_"]
    ClkcfgKeyUnlockW = 169,
}
impl From<ClkcfgKey> for u8 {
    #[inline(always)]
    fn from(variant: ClkcfgKey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkcfgKey {
    type Ux = u8;
}
impl crate::IsEnum for ClkcfgKey {}
#[doc = "Field `CLKCFG_KEY` writer - Unlock key"]
pub type ClkcfgKeyW<'a, REG> = crate::FieldWriter<'a, REG, 8, ClkcfgKey>;
impl<'a, REG> ClkcfgKeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "_TO_UNLOCK_W_"]
    #[inline(always)]
    pub fn clkcfg_key_unlock_w(self) -> &'a mut crate::W<REG> {
        self.variant(ClkcfgKey::ClkcfgKeyUnlockW)
    }
}
impl R {
    #[doc = "Bits 0:1 - ADC sample clock source selection."]
    #[inline(always)]
    pub fn clkcfg_sampclk(&self) -> ClkcfgSampclkR {
        ClkcfgSampclkR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - CCONRUN: Forces SYSOSC to run at base frequency when device is in RUN mode which can be used as ADC sample or conversion clock source."]
    #[inline(always)]
    pub fn clkcfg_cconrun(&self) -> ClkcfgCconrunR {
        ClkcfgCconrunR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CCONSTOP: Forces SYSOSC to run at base frequency when device is in STOP mode which can be used as ADC sample or conversion clock source."]
    #[inline(always)]
    pub fn clkcfg_cconstop(&self) -> ClkcfgCconstopR {
        ClkcfgCconstopR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC sample clock source selection."]
    #[inline(always)]
    pub fn clkcfg_sampclk(&mut self) -> ClkcfgSampclkW<ClkcfgSpec> {
        ClkcfgSampclkW::new(self, 0)
    }
    #[doc = "Bit 4 - CCONRUN: Forces SYSOSC to run at base frequency when device is in RUN mode which can be used as ADC sample or conversion clock source."]
    #[inline(always)]
    pub fn clkcfg_cconrun(&mut self) -> ClkcfgCconrunW<ClkcfgSpec> {
        ClkcfgCconrunW::new(self, 4)
    }
    #[doc = "Bit 5 - CCONSTOP: Forces SYSOSC to run at base frequency when device is in STOP mode which can be used as ADC sample or conversion clock source."]
    #[inline(always)]
    pub fn clkcfg_cconstop(&mut self) -> ClkcfgCconstopW<ClkcfgSpec> {
        ClkcfgCconstopW::new(self, 5)
    }
    #[doc = "Bits 24:31 - Unlock key"]
    #[inline(always)]
    pub fn clkcfg_key(&mut self) -> ClkcfgKeyW<ClkcfgSpec> {
        ClkcfgKeyW::new(self, 24)
    }
}
#[doc = "ADC clock configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkcfgSpec;
impl crate::RegisterSpec for ClkcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkcfg::R`](R) reader structure"]
impl crate::Readable for ClkcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`clkcfg::W`](W) writer structure"]
impl crate::Writable for ClkcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCFG to value 0"]
impl crate::Resettable for ClkcfgSpec {
    const RESET_VALUE: u32 = 0;
}
