#[doc = "Register `EVT_MODE` reader"]
pub type R = crate::R<EvtModeSpec>;
#[doc = "Register `EVT_MODE` writer"]
pub type W = crate::W<EvtModeSpec>;
#[doc = "Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EvtModeInt0Cfg {
    #[doc = "0: DISABLE"]
    EvtModeInt0CfgDisable = 0,
    #[doc = "1: SOFTWARE"]
    EvtModeInt0CfgSoftware = 1,
    #[doc = "2: HARDWARE"]
    EvtModeInt0CfgHardware = 2,
}
impl From<EvtModeInt0Cfg> for u8 {
    #[inline(always)]
    fn from(variant: EvtModeInt0Cfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EvtModeInt0Cfg {
    type Ux = u8;
}
impl crate::IsEnum for EvtModeInt0Cfg {}
#[doc = "Field `EVT_MODE_INT0_CFG` reader - Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT0\\]"]
pub type EvtModeInt0CfgR = crate::FieldReader<EvtModeInt0Cfg>;
impl EvtModeInt0CfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EvtModeInt0Cfg> {
        match self.bits {
            0 => Some(EvtModeInt0Cfg::EvtModeInt0CfgDisable),
            1 => Some(EvtModeInt0Cfg::EvtModeInt0CfgSoftware),
            2 => Some(EvtModeInt0Cfg::EvtModeInt0CfgHardware),
            _ => None,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_evt_mode_int0_cfg_disable(&self) -> bool {
        *self == EvtModeInt0Cfg::EvtModeInt0CfgDisable
    }
    #[doc = "SOFTWARE"]
    #[inline(always)]
    pub fn is_evt_mode_int0_cfg_software(&self) -> bool {
        *self == EvtModeInt0Cfg::EvtModeInt0CfgSoftware
    }
    #[doc = "HARDWARE"]
    #[inline(always)]
    pub fn is_evt_mode_int0_cfg_hardware(&self) -> bool {
        *self == EvtModeInt0Cfg::EvtModeInt0CfgHardware
    }
}
#[doc = "Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EvtModeInt1Cfg {
    #[doc = "0: DISABLE"]
    EvtModeInt1CfgDisable = 0,
    #[doc = "1: SOFTWARE"]
    EvtModeInt1CfgSoftware = 1,
    #[doc = "2: HARDWARE"]
    EvtModeInt1CfgHardware = 2,
}
impl From<EvtModeInt1Cfg> for u8 {
    #[inline(always)]
    fn from(variant: EvtModeInt1Cfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EvtModeInt1Cfg {
    type Ux = u8;
}
impl crate::IsEnum for EvtModeInt1Cfg {}
#[doc = "Field `EVT_MODE_INT1_CFG` reader - Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT1\\]"]
pub type EvtModeInt1CfgR = crate::FieldReader<EvtModeInt1Cfg>;
impl EvtModeInt1CfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EvtModeInt1Cfg> {
        match self.bits {
            0 => Some(EvtModeInt1Cfg::EvtModeInt1CfgDisable),
            1 => Some(EvtModeInt1Cfg::EvtModeInt1CfgSoftware),
            2 => Some(EvtModeInt1Cfg::EvtModeInt1CfgHardware),
            _ => None,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_evt_mode_int1_cfg_disable(&self) -> bool {
        *self == EvtModeInt1Cfg::EvtModeInt1CfgDisable
    }
    #[doc = "SOFTWARE"]
    #[inline(always)]
    pub fn is_evt_mode_int1_cfg_software(&self) -> bool {
        *self == EvtModeInt1Cfg::EvtModeInt1CfgSoftware
    }
    #[doc = "HARDWARE"]
    #[inline(always)]
    pub fn is_evt_mode_int1_cfg_hardware(&self) -> bool {
        *self == EvtModeInt1Cfg::EvtModeInt1CfgHardware
    }
}
#[doc = "Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EvtModeInt2Cfg {
    #[doc = "0: DISABLE"]
    EvtModeInt2CfgDisable = 0,
    #[doc = "1: SOFTWARE"]
    EvtModeInt2CfgSoftware = 1,
    #[doc = "2: HARDWARE"]
    EvtModeInt2CfgHardware = 2,
}
impl From<EvtModeInt2Cfg> for u8 {
    #[inline(always)]
    fn from(variant: EvtModeInt2Cfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EvtModeInt2Cfg {
    type Ux = u8;
}
impl crate::IsEnum for EvtModeInt2Cfg {}
#[doc = "Field `EVT_MODE_INT2_CFG` reader - Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT2\\]"]
pub type EvtModeInt2CfgR = crate::FieldReader<EvtModeInt2Cfg>;
impl EvtModeInt2CfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EvtModeInt2Cfg> {
        match self.bits {
            0 => Some(EvtModeInt2Cfg::EvtModeInt2CfgDisable),
            1 => Some(EvtModeInt2Cfg::EvtModeInt2CfgSoftware),
            2 => Some(EvtModeInt2Cfg::EvtModeInt2CfgHardware),
            _ => None,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_evt_mode_int2_cfg_disable(&self) -> bool {
        *self == EvtModeInt2Cfg::EvtModeInt2CfgDisable
    }
    #[doc = "SOFTWARE"]
    #[inline(always)]
    pub fn is_evt_mode_int2_cfg_software(&self) -> bool {
        *self == EvtModeInt2Cfg::EvtModeInt2CfgSoftware
    }
    #[doc = "HARDWARE"]
    #[inline(always)]
    pub fn is_evt_mode_int2_cfg_hardware(&self) -> bool {
        *self == EvtModeInt2Cfg::EvtModeInt2CfgHardware
    }
}
impl R {
    #[doc = "Bits 0:1 - Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT0\\]"]
    #[inline(always)]
    pub fn evt_mode_int0_cfg(&self) -> EvtModeInt0CfgR {
        EvtModeInt0CfgR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT1\\]"]
    #[inline(always)]
    pub fn evt_mode_int1_cfg(&self) -> EvtModeInt1CfgR {
        EvtModeInt1CfgR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT2\\]"]
    #[inline(always)]
    pub fn evt_mode_int2_cfg(&self) -> EvtModeInt2CfgR {
        EvtModeInt2CfgR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {}
#[doc = "Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtModeSpec;
impl crate::RegisterSpec for EvtModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evt_mode::R`](R) reader structure"]
impl crate::Readable for EvtModeSpec {}
#[doc = "`write(|w| ..)` method takes [`evt_mode::W`](W) writer structure"]
impl crate::Writable for EvtModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVT_MODE to value 0"]
impl crate::Resettable for EvtModeSpec {
    const RESET_VALUE: u32 = 0;
}
