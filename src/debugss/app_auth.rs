#[doc = "Register `APP_AUTH` reader"]
pub type R = crate::R<AppAuthSpec>;
#[doc = "Controls invasive debug enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppAuthDbgen {
    #[doc = "0: DISABLE"]
    AppAuthDbgenDisable = 0,
    #[doc = "1: ENABLE"]
    AppAuthDbgenEnable = 1,
}
impl From<AppAuthDbgen> for bool {
    #[inline(always)]
    fn from(variant: AppAuthDbgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APP_AUTH_DBGEN` reader - Controls invasive debug enable."]
pub type AppAuthDbgenR = crate::BitReader<AppAuthDbgen>;
impl AppAuthDbgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AppAuthDbgen {
        match self.bits {
            false => AppAuthDbgen::AppAuthDbgenDisable,
            true => AppAuthDbgen::AppAuthDbgenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_app_auth_dbgen_disable(&self) -> bool {
        *self == AppAuthDbgen::AppAuthDbgenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_app_auth_dbgen_enable(&self) -> bool {
        *self == AppAuthDbgen::AppAuthDbgenEnable
    }
}
#[doc = "Controls non-invasive debug enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppAuthNiden {
    #[doc = "0: DISABLE"]
    AppAuthNidenDisable = 0,
    #[doc = "1: ENABLE"]
    AppAuthNidenEnable = 1,
}
impl From<AppAuthNiden> for bool {
    #[inline(always)]
    fn from(variant: AppAuthNiden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APP_AUTH_NIDEN` reader - Controls non-invasive debug enable."]
pub type AppAuthNidenR = crate::BitReader<AppAuthNiden>;
impl AppAuthNidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AppAuthNiden {
        match self.bits {
            false => AppAuthNiden::AppAuthNidenDisable,
            true => AppAuthNiden::AppAuthNidenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_app_auth_niden_disable(&self) -> bool {
        *self == AppAuthNiden::AppAuthNidenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_app_auth_niden_enable(&self) -> bool {
        *self == AppAuthNiden::AppAuthNidenEnable
    }
}
#[doc = "Secure invasive debug enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppAuthSpiden {
    #[doc = "0: DISABLE"]
    AppAuthSpidenDisable = 0,
    #[doc = "1: ENABLE"]
    AppAuthSpidenEnable = 1,
}
impl From<AppAuthSpiden> for bool {
    #[inline(always)]
    fn from(variant: AppAuthSpiden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APP_AUTH_SPIDEN` reader - Secure invasive debug enable."]
pub type AppAuthSpidenR = crate::BitReader<AppAuthSpiden>;
impl AppAuthSpidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AppAuthSpiden {
        match self.bits {
            false => AppAuthSpiden::AppAuthSpidenDisable,
            true => AppAuthSpiden::AppAuthSpidenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_app_auth_spiden_disable(&self) -> bool {
        *self == AppAuthSpiden::AppAuthSpidenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_app_auth_spiden_enable(&self) -> bool {
        *self == AppAuthSpiden::AppAuthSpidenEnable
    }
}
#[doc = "Secure non-invasive debug enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppAuthSpniden {
    #[doc = "0: DISABLE"]
    AppAuthSpnidenDisable = 0,
    #[doc = "1: ENABLE"]
    AppAuthSpnidenEnable = 1,
}
impl From<AppAuthSpniden> for bool {
    #[inline(always)]
    fn from(variant: AppAuthSpniden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `APP_AUTH_SPNIDEN` reader - Secure non-invasive debug enable."]
pub type AppAuthSpnidenR = crate::BitReader<AppAuthSpniden>;
impl AppAuthSpnidenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AppAuthSpniden {
        match self.bits {
            false => AppAuthSpniden::AppAuthSpnidenDisable,
            true => AppAuthSpniden::AppAuthSpnidenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_app_auth_spniden_disable(&self) -> bool {
        *self == AppAuthSpniden::AppAuthSpnidenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_app_auth_spniden_enable(&self) -> bool {
        *self == AppAuthSpniden::AppAuthSpnidenEnable
    }
}
impl R {
    #[doc = "Bit 0 - Controls invasive debug enable."]
    #[inline(always)]
    pub fn app_auth_dbgen(&self) -> AppAuthDbgenR {
        AppAuthDbgenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Controls non-invasive debug enable."]
    #[inline(always)]
    pub fn app_auth_niden(&self) -> AppAuthNidenR {
        AppAuthNidenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Secure invasive debug enable."]
    #[inline(always)]
    pub fn app_auth_spiden(&self) -> AppAuthSpidenR {
        AppAuthSpidenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Secure non-invasive debug enable."]
    #[inline(always)]
    pub fn app_auth_spniden(&self) -> AppAuthSpnidenR {
        AppAuthSpnidenR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Application CPU0 authorization register\n\nYou can [`read`](crate::Reg::read) this register and get [`app_auth::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppAuthSpec;
impl crate::RegisterSpec for AppAuthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`app_auth::R`](R) reader structure"]
impl crate::Readable for AppAuthSpec {}
#[doc = "`reset()` method sets APP_AUTH to value 0"]
impl crate::Resettable for AppAuthSpec {
    const RESET_VALUE: u32 = 0;
}
