#[doc = "Register `EVT_MODE` reader"]
pub type R = crate::R<EvtModeSpec>;
#[doc = "Register `EVT_MODE` writer"]
pub type W = crate::W<EvtModeSpec>;
#[doc = "Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT\\]\\[0\\]\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EvtModeEvt0Cfg {
    #[doc = "0: DISABLE"]
    EvtModeEvt0CfgDisable = 0,
    #[doc = "1: SOFTWARE"]
    EvtModeEvt0CfgSoftware = 1,
    #[doc = "2: HARDWARE"]
    EvtModeEvt0CfgHardware = 2,
}
impl From<EvtModeEvt0Cfg> for u8 {
    #[inline(always)]
    fn from(variant: EvtModeEvt0Cfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EvtModeEvt0Cfg {
    type Ux = u8;
}
impl crate::IsEnum for EvtModeEvt0Cfg {}
#[doc = "Field `EVT_MODE_EVT0_CFG` reader - Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT\\]\\[0\\]"]
pub type EvtModeEvt0CfgR = crate::FieldReader<EvtModeEvt0Cfg>;
impl EvtModeEvt0CfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EvtModeEvt0Cfg> {
        match self.bits {
            0 => Some(EvtModeEvt0Cfg::EvtModeEvt0CfgDisable),
            1 => Some(EvtModeEvt0Cfg::EvtModeEvt0CfgSoftware),
            2 => Some(EvtModeEvt0Cfg::EvtModeEvt0CfgHardware),
            _ => None,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_evt_mode_evt0_cfg_disable(&self) -> bool {
        *self == EvtModeEvt0Cfg::EvtModeEvt0CfgDisable
    }
    #[doc = "SOFTWARE"]
    #[inline(always)]
    pub fn is_evt_mode_evt0_cfg_software(&self) -> bool {
        *self == EvtModeEvt0Cfg::EvtModeEvt0CfgSoftware
    }
    #[doc = "HARDWARE"]
    #[inline(always)]
    pub fn is_evt_mode_evt0_cfg_hardware(&self) -> bool {
        *self == EvtModeEvt0Cfg::EvtModeEvt0CfgHardware
    }
}
#[doc = "Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT\\]\\[1\\]\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EvtModeEvt1Cfg {
    #[doc = "0: DISABLE"]
    EvtModeEvt1CfgDisable = 0,
    #[doc = "1: SOFTWARE"]
    EvtModeEvt1CfgSoftware = 1,
    #[doc = "2: HARDWARE"]
    EvtModeEvt1CfgHardware = 2,
}
impl From<EvtModeEvt1Cfg> for u8 {
    #[inline(always)]
    fn from(variant: EvtModeEvt1Cfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EvtModeEvt1Cfg {
    type Ux = u8;
}
impl crate::IsEnum for EvtModeEvt1Cfg {}
#[doc = "Field `EVT_MODE_EVT1_CFG` reader - Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT\\]\\[1\\]"]
pub type EvtModeEvt1CfgR = crate::FieldReader<EvtModeEvt1Cfg>;
impl EvtModeEvt1CfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EvtModeEvt1Cfg> {
        match self.bits {
            0 => Some(EvtModeEvt1Cfg::EvtModeEvt1CfgDisable),
            1 => Some(EvtModeEvt1Cfg::EvtModeEvt1CfgSoftware),
            2 => Some(EvtModeEvt1Cfg::EvtModeEvt1CfgHardware),
            _ => None,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_evt_mode_evt1_cfg_disable(&self) -> bool {
        *self == EvtModeEvt1Cfg::EvtModeEvt1CfgDisable
    }
    #[doc = "SOFTWARE"]
    #[inline(always)]
    pub fn is_evt_mode_evt1_cfg_software(&self) -> bool {
        *self == EvtModeEvt1Cfg::EvtModeEvt1CfgSoftware
    }
    #[doc = "HARDWARE"]
    #[inline(always)]
    pub fn is_evt_mode_evt1_cfg_hardware(&self) -> bool {
        *self == EvtModeEvt1Cfg::EvtModeEvt1CfgHardware
    }
}
#[doc = "Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT\\]\\[1\\]\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EvtModeEvt2Cfg {
    #[doc = "0: DISABLE"]
    EvtModeEvt2CfgDisable = 0,
    #[doc = "1: SOFTWARE"]
    EvtModeEvt2CfgSoftware = 1,
    #[doc = "2: HARDWARE"]
    EvtModeEvt2CfgHardware = 2,
}
impl From<EvtModeEvt2Cfg> for u8 {
    #[inline(always)]
    fn from(variant: EvtModeEvt2Cfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EvtModeEvt2Cfg {
    type Ux = u8;
}
impl crate::IsEnum for EvtModeEvt2Cfg {}
#[doc = "Field `EVT_MODE_EVT2_CFG` reader - Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT\\]\\[1\\]"]
pub type EvtModeEvt2CfgR = crate::FieldReader<EvtModeEvt2Cfg>;
impl EvtModeEvt2CfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EvtModeEvt2Cfg> {
        match self.bits {
            0 => Some(EvtModeEvt2Cfg::EvtModeEvt2CfgDisable),
            1 => Some(EvtModeEvt2Cfg::EvtModeEvt2CfgSoftware),
            2 => Some(EvtModeEvt2Cfg::EvtModeEvt2CfgHardware),
            _ => None,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_evt_mode_evt2_cfg_disable(&self) -> bool {
        *self == EvtModeEvt2Cfg::EvtModeEvt2CfgDisable
    }
    #[doc = "SOFTWARE"]
    #[inline(always)]
    pub fn is_evt_mode_evt2_cfg_software(&self) -> bool {
        *self == EvtModeEvt2Cfg::EvtModeEvt2CfgSoftware
    }
    #[doc = "HARDWARE"]
    #[inline(always)]
    pub fn is_evt_mode_evt2_cfg_hardware(&self) -> bool {
        *self == EvtModeEvt2Cfg::EvtModeEvt2CfgHardware
    }
}
impl R {
    #[doc = "Bits 0:1 - Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT\\]\\[0\\]"]
    #[inline(always)]
    pub fn evt_mode_evt0_cfg(&self) -> EvtModeEvt0CfgR {
        EvtModeEvt0CfgR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT\\]\\[1\\]"]
    #[inline(always)]
    pub fn evt_mode_evt1_cfg(&self) -> EvtModeEvt1CfgR {
        EvtModeEvt1CfgR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Event line mode select for event corresponding to \\[IPSTANDARD.INT_EVENT\\]\\[1\\]"]
    #[inline(always)]
    pub fn evt_mode_evt2_cfg(&self) -> EvtModeEvt2CfgR {
        EvtModeEvt2CfgR::new(((self.bits >> 4) & 3) as u8)
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
#[doc = "`reset()` method sets EVT_MODE to value 0x29"]
impl crate::Resettable for EvtModeSpec {
    const RESET_VALUE: u32 = 0x29;
}
