#[doc = "Register `MBMON` reader"]
pub type R = crate::R<MbmonSpec>;
#[doc = "I2C SCL Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MbmonScl {
    #[doc = "0: CLEARED"]
    MbmonSclCleared = 0,
    #[doc = "1: SET"]
    MbmonSclSet = 1,
}
impl From<MbmonScl> for bool {
    #[inline(always)]
    fn from(variant: MbmonScl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MBMON_SCL` reader - I2C SCL Status"]
pub type MbmonSclR = crate::BitReader<MbmonScl>;
impl MbmonSclR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MbmonScl {
        match self.bits {
            false => MbmonScl::MbmonSclCleared,
            true => MbmonScl::MbmonSclSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_mbmon_scl_cleared(&self) -> bool {
        *self == MbmonScl::MbmonSclCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mbmon_scl_set(&self) -> bool {
        *self == MbmonScl::MbmonSclSet
    }
}
#[doc = "I2C SDA Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MbmonSda {
    #[doc = "0: CLEARED"]
    MbmonSdaCleared = 0,
    #[doc = "1: SET"]
    MbmonSdaSet = 1,
}
impl From<MbmonSda> for bool {
    #[inline(always)]
    fn from(variant: MbmonSda) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MBMON_SDA` reader - I2C SDA Status"]
pub type MbmonSdaR = crate::BitReader<MbmonSda>;
impl MbmonSdaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MbmonSda {
        match self.bits {
            false => MbmonSda::MbmonSdaCleared,
            true => MbmonSda::MbmonSdaSet,
        }
    }
    #[doc = "CLEARED"]
    #[inline(always)]
    pub fn is_mbmon_sda_cleared(&self) -> bool {
        *self == MbmonSda::MbmonSdaCleared
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mbmon_sda_set(&self) -> bool {
        *self == MbmonSda::MbmonSdaSet
    }
}
impl R {
    #[doc = "Bit 0 - I2C SCL Status"]
    #[inline(always)]
    pub fn mbmon_scl(&self) -> MbmonSclR {
        MbmonSclR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C SDA Status"]
    #[inline(always)]
    pub fn mbmon_sda(&self) -> MbmonSdaR {
        MbmonSdaR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "I2C Master Bus Monitor\n\nYou can [`read`](crate::Reg::read) this register and get [`mbmon::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbmonSpec;
impl crate::RegisterSpec for MbmonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbmon::R`](R) reader structure"]
impl crate::Readable for MbmonSpec {}
#[doc = "`reset()` method sets MBMON to value 0x03"]
impl crate::Resettable for MbmonSpec {
    const RESET_VALUE: u32 = 0x03;
}
