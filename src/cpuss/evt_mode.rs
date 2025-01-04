#[doc = "Register `EVT_MODE` reader"]
pub type R = crate::R<EvtModeSpec>;
#[doc = "Event line mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EvtModeIntCfg {
    #[doc = "0: DISABLE"]
    EvtModeIntCfgDisable = 0,
    #[doc = "1: SOFTWARE"]
    EvtModeIntCfgSoftware = 1,
    #[doc = "2: HARDWARE"]
    EvtModeIntCfgHardware = 2,
}
impl From<EvtModeIntCfg> for u8 {
    #[inline(always)]
    fn from(variant: EvtModeIntCfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EvtModeIntCfg {
    type Ux = u8;
}
impl crate::IsEnum for EvtModeIntCfg {}
#[doc = "Field `EVT_MODE_INT_CFG` reader - Event line mode select"]
pub type EvtModeIntCfgR = crate::FieldReader<EvtModeIntCfg>;
impl EvtModeIntCfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EvtModeIntCfg> {
        match self.bits {
            0 => Some(EvtModeIntCfg::EvtModeIntCfgDisable),
            1 => Some(EvtModeIntCfg::EvtModeIntCfgSoftware),
            2 => Some(EvtModeIntCfg::EvtModeIntCfgHardware),
            _ => None,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_evt_mode_int_cfg_disable(&self) -> bool {
        *self == EvtModeIntCfg::EvtModeIntCfgDisable
    }
    #[doc = "SOFTWARE"]
    #[inline(always)]
    pub fn is_evt_mode_int_cfg_software(&self) -> bool {
        *self == EvtModeIntCfg::EvtModeIntCfgSoftware
    }
    #[doc = "HARDWARE"]
    #[inline(always)]
    pub fn is_evt_mode_int_cfg_hardware(&self) -> bool {
        *self == EvtModeIntCfg::EvtModeIntCfgHardware
    }
}
impl R {
    #[doc = "Bits 0:1 - Event line mode select"]
    #[inline(always)]
    pub fn evt_mode_int_cfg(&self) -> EvtModeIntCfgR {
        EvtModeIntCfgR::new((self.bits & 3) as u8)
    }
}
#[doc = "Event Mode\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_mode::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvtModeSpec;
impl crate::RegisterSpec for EvtModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evt_mode::R`](R) reader structure"]
impl crate::Readable for EvtModeSpec {}
#[doc = "`reset()` method sets EVT_MODE to value 0"]
impl crate::Resettable for EvtModeSpec {
    const RESET_VALUE: u32 = 0;
}
