#[doc = "Register `SPECIAL_AUTH` reader"]
pub type R = crate::R<SpecialAuthSpec>;
#[doc = "An active high input. When asserted (and SWD access is also permitted), the debug tools can use the Security-AP to communicate with security control logic. When deasserted, a DAPBUS firewall will isolate the AP and prevent access to the Security-AP.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpecialAuthSecapen {
    #[doc = "0: DISABLE"]
    SpecialAuthSecapenDisable = 0,
    #[doc = "1: ENABLE"]
    SpecialAuthSecapenEnable = 1,
}
impl From<SpecialAuthSecapen> for bool {
    #[inline(always)]
    fn from(variant: SpecialAuthSecapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPECIAL_AUTH_SECAPEN` reader - An active high input. When asserted (and SWD access is also permitted), the debug tools can use the Security-AP to communicate with security control logic. When deasserted, a DAPBUS firewall will isolate the AP and prevent access to the Security-AP."]
pub type SpecialAuthSecapenR = crate::BitReader<SpecialAuthSecapen>;
impl SpecialAuthSecapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpecialAuthSecapen {
        match self.bits {
            false => SpecialAuthSecapen::SpecialAuthSecapenDisable,
            true => SpecialAuthSecapen::SpecialAuthSecapenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_special_auth_secapen_disable(&self) -> bool {
        *self == SpecialAuthSecapen::SpecialAuthSecapenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_special_auth_secapen_enable(&self) -> bool {
        *self == SpecialAuthSecapen::SpecialAuthSecapenEnable
    }
}
#[doc = "When asserted, the SW-DP functions normally. When deasserted, the SW-DP effectively disables all external debug access.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpecialAuthSwdporten {
    #[doc = "0: DISABLE"]
    SpecialAuthSwdportenDisable = 0,
    #[doc = "1: ENABLE"]
    SpecialAuthSwdportenEnable = 1,
}
impl From<SpecialAuthSwdporten> for bool {
    #[inline(always)]
    fn from(variant: SpecialAuthSwdporten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPECIAL_AUTH_SWDPORTEN` reader - When asserted, the SW-DP functions normally. When deasserted, the SW-DP effectively disables all external debug access."]
pub type SpecialAuthSwdportenR = crate::BitReader<SpecialAuthSwdporten>;
impl SpecialAuthSwdportenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpecialAuthSwdporten {
        match self.bits {
            false => SpecialAuthSwdporten::SpecialAuthSwdportenDisable,
            true => SpecialAuthSwdporten::SpecialAuthSwdportenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_special_auth_swdporten_disable(&self) -> bool {
        *self == SpecialAuthSwdporten::SpecialAuthSwdportenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_special_auth_swdporten_enable(&self) -> bool {
        *self == SpecialAuthSwdporten::SpecialAuthSwdportenEnable
    }
}
#[doc = "An active high input. When asserted (and SWD access is also permitted), the debug tools can then access the DFT-AP external to the DebugSS lite. When deasserted, a DAPBUS firewall will isolate the AP and prevent access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpecialAuthDftapen {
    #[doc = "0: DISABLE"]
    SpecialAuthDftapenDisable = 0,
    #[doc = "1: ENABLE"]
    SpecialAuthDftapenEnable = 1,
}
impl From<SpecialAuthDftapen> for bool {
    #[inline(always)]
    fn from(variant: SpecialAuthDftapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPECIAL_AUTH_DFTAPEN` reader - An active high input. When asserted (and SWD access is also permitted), the debug tools can then access the DFT-AP external to the DebugSS lite. When deasserted, a DAPBUS firewall will isolate the AP and prevent access."]
pub type SpecialAuthDftapenR = crate::BitReader<SpecialAuthDftapen>;
impl SpecialAuthDftapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpecialAuthDftapen {
        match self.bits {
            false => SpecialAuthDftapen::SpecialAuthDftapenDisable,
            true => SpecialAuthDftapen::SpecialAuthDftapenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_special_auth_dftapen_disable(&self) -> bool {
        *self == SpecialAuthDftapen::SpecialAuthDftapenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_special_auth_dftapen_enable(&self) -> bool {
        *self == SpecialAuthDftapen::SpecialAuthDftapenEnable
    }
}
#[doc = "An active high input. When asserted (and SWD access is also permitted), the debug tools can then access an ET-AP external to the DebugSS lite. When deasserted, a DAPBUS firewall will isolate the AP and prevent access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpecialAuthEtapen {
    #[doc = "0: DISABLE"]
    SpecialAuthEtapenDisable = 0,
    #[doc = "1: ENABLE"]
    SpecialAuthEtapenEnable = 1,
}
impl From<SpecialAuthEtapen> for bool {
    #[inline(always)]
    fn from(variant: SpecialAuthEtapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPECIAL_AUTH_ETAPEN` reader - An active high input. When asserted (and SWD access is also permitted), the debug tools can then access an ET-AP external to the DebugSS lite. When deasserted, a DAPBUS firewall will isolate the AP and prevent access."]
pub type SpecialAuthEtapenR = crate::BitReader<SpecialAuthEtapen>;
impl SpecialAuthEtapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpecialAuthEtapen {
        match self.bits {
            false => SpecialAuthEtapen::SpecialAuthEtapenDisable,
            true => SpecialAuthEtapen::SpecialAuthEtapenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_special_auth_etapen_disable(&self) -> bool {
        *self == SpecialAuthEtapen::SpecialAuthEtapenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_special_auth_etapen_enable(&self) -> bool {
        *self == SpecialAuthEtapen::SpecialAuthEtapenEnable
    }
}
#[doc = "An active high input. When asserted (and SWD access is also permitted), the debug tools can use the Config-AP to read device configuration information. When deasserted, a DAPBUS firewall will isolate the AP and prevent access to the Config-AP.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpecialAuthCfgapen {
    #[doc = "0: DISABLE"]
    SpecialAuthCfgapenDisable = 0,
    #[doc = "1: ENABLE"]
    SpecialAuthCfgapenEnable = 1,
}
impl From<SpecialAuthCfgapen> for bool {
    #[inline(always)]
    fn from(variant: SpecialAuthCfgapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPECIAL_AUTH_CFGAPEN` reader - An active high input. When asserted (and SWD access is also permitted), the debug tools can use the Config-AP to read device configuration information. When deasserted, a DAPBUS firewall will isolate the AP and prevent access to the Config-AP."]
pub type SpecialAuthCfgapenR = crate::BitReader<SpecialAuthCfgapen>;
impl SpecialAuthCfgapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpecialAuthCfgapen {
        match self.bits {
            false => SpecialAuthCfgapen::SpecialAuthCfgapenDisable,
            true => SpecialAuthCfgapen::SpecialAuthCfgapenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_special_auth_cfgapen_disable(&self) -> bool {
        *self == SpecialAuthCfgapen::SpecialAuthCfgapenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_special_auth_cfgapen_enable(&self) -> bool {
        *self == SpecialAuthCfgapen::SpecialAuthCfgapenEnable
    }
}
#[doc = "Disabling / enabling debug access to the M0+ Core via the AHB-AP DAP bus isolation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpecialAuthAhbapen {
    #[doc = "0: DISABLE"]
    SpecialAuthAhbapenDisable = 0,
    #[doc = "1: ENABLE"]
    SpecialAuthAhbapenEnable = 1,
}
impl From<SpecialAuthAhbapen> for bool {
    #[inline(always)]
    fn from(variant: SpecialAuthAhbapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPECIAL_AUTH_AHBAPEN` reader - Disabling / enabling debug access to the M0+ Core via the AHB-AP DAP bus isolation."]
pub type SpecialAuthAhbapenR = crate::BitReader<SpecialAuthAhbapen>;
impl SpecialAuthAhbapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpecialAuthAhbapen {
        match self.bits {
            false => SpecialAuthAhbapen::SpecialAuthAhbapenDisable,
            true => SpecialAuthAhbapen::SpecialAuthAhbapenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_special_auth_ahbapen_disable(&self) -> bool {
        *self == SpecialAuthAhbapen::SpecialAuthAhbapenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_special_auth_ahbapen_enable(&self) -> bool {
        *self == SpecialAuthAhbapen::SpecialAuthAhbapenEnable
    }
}
#[doc = "An active high input. When asserted (and SWD access is also permitted), the debug tools can then access the PWR-AP to power and reset state of the CPU. When deasserted, a DAPBUS firewall will isolate the AP and prevent access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpecialAuthPwrapen {
    #[doc = "0: DISABLE"]
    SpecialAuthPwrapenDisable = 0,
    #[doc = "1: ENABLE"]
    SpecialAuthPwrapenEnable = 1,
}
impl From<SpecialAuthPwrapen> for bool {
    #[inline(always)]
    fn from(variant: SpecialAuthPwrapen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPECIAL_AUTH_PWRAPEN` reader - An active high input. When asserted (and SWD access is also permitted), the debug tools can then access the PWR-AP to power and reset state of the CPU. When deasserted, a DAPBUS firewall will isolate the AP and prevent access."]
pub type SpecialAuthPwrapenR = crate::BitReader<SpecialAuthPwrapen>;
impl SpecialAuthPwrapenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpecialAuthPwrapen {
        match self.bits {
            false => SpecialAuthPwrapen::SpecialAuthPwrapenDisable,
            true => SpecialAuthPwrapen::SpecialAuthPwrapenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_special_auth_pwrapen_disable(&self) -> bool {
        *self == SpecialAuthPwrapen::SpecialAuthPwrapenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_special_auth_pwrapen_enable(&self) -> bool {
        *self == SpecialAuthPwrapen::SpecialAuthPwrapenEnable
    }
}
impl R {
    #[doc = "Bit 0 - An active high input. When asserted (and SWD access is also permitted), the debug tools can use the Security-AP to communicate with security control logic. When deasserted, a DAPBUS firewall will isolate the AP and prevent access to the Security-AP."]
    #[inline(always)]
    pub fn special_auth_secapen(&self) -> SpecialAuthSecapenR {
        SpecialAuthSecapenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When asserted, the SW-DP functions normally. When deasserted, the SW-DP effectively disables all external debug access."]
    #[inline(always)]
    pub fn special_auth_swdporten(&self) -> SpecialAuthSwdportenR {
        SpecialAuthSwdportenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - An active high input. When asserted (and SWD access is also permitted), the debug tools can then access the DFT-AP external to the DebugSS lite. When deasserted, a DAPBUS firewall will isolate the AP and prevent access."]
    #[inline(always)]
    pub fn special_auth_dftapen(&self) -> SpecialAuthDftapenR {
        SpecialAuthDftapenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - An active high input. When asserted (and SWD access is also permitted), the debug tools can then access an ET-AP external to the DebugSS lite. When deasserted, a DAPBUS firewall will isolate the AP and prevent access."]
    #[inline(always)]
    pub fn special_auth_etapen(&self) -> SpecialAuthEtapenR {
        SpecialAuthEtapenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - An active high input. When asserted (and SWD access is also permitted), the debug tools can use the Config-AP to read device configuration information. When deasserted, a DAPBUS firewall will isolate the AP and prevent access to the Config-AP."]
    #[inline(always)]
    pub fn special_auth_cfgapen(&self) -> SpecialAuthCfgapenR {
        SpecialAuthCfgapenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disabling / enabling debug access to the M0+ Core via the AHB-AP DAP bus isolation."]
    #[inline(always)]
    pub fn special_auth_ahbapen(&self) -> SpecialAuthAhbapenR {
        SpecialAuthAhbapenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - An active high input. When asserted (and SWD access is also permitted), the debug tools can then access the PWR-AP to power and reset state of the CPU. When deasserted, a DAPBUS firewall will isolate the AP and prevent access."]
    #[inline(always)]
    pub fn special_auth_pwrapen(&self) -> SpecialAuthPwrapenR {
        SpecialAuthPwrapenR::new(((self.bits >> 6) & 1) != 0)
    }
}
#[doc = "Special enable authorization register\n\nYou can [`read`](crate::Reg::read) this register and get [`special_auth::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpecialAuthSpec;
impl crate::RegisterSpec for SpecialAuthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`special_auth::R`](R) reader structure"]
impl crate::Readable for SpecialAuthSpec {}
#[doc = "`reset()` method sets SPECIAL_AUTH to value 0x13"]
impl crate::Resettable for SpecialAuthSpec {
    const RESET_VALUE: u32 = 0x13;
}
