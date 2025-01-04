#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "MCAN Interrupt Line 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisIntl0 {
    #[doc = "0: CLR"]
    RisIntl0Clr = 0,
    #[doc = "1: SET"]
    RisIntl0Set = 1,
}
impl From<RisIntl0> for bool {
    #[inline(always)]
    fn from(variant: RisIntl0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_INTL0` reader - MCAN Interrupt Line 0."]
pub type RisIntl0R = crate::BitReader<RisIntl0>;
impl RisIntl0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisIntl0 {
        match self.bits {
            false => RisIntl0::RisIntl0Clr,
            true => RisIntl0::RisIntl0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_intl0_clr(&self) -> bool {
        *self == RisIntl0::RisIntl0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_intl0_set(&self) -> bool {
        *self == RisIntl0::RisIntl0Set
    }
}
#[doc = "MCAN Interrupt Line 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisIntl1 {
    #[doc = "0: CLR"]
    RisIntl1Clr = 0,
    #[doc = "1: SET"]
    RisIntl1Set = 1,
}
impl From<RisIntl1> for bool {
    #[inline(always)]
    fn from(variant: RisIntl1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_INTL1` reader - MCAN Interrupt Line 1."]
pub type RisIntl1R = crate::BitReader<RisIntl1>;
impl RisIntl1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisIntl1 {
        match self.bits {
            false => RisIntl1::RisIntl1Clr,
            true => RisIntl1::RisIntl1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_intl1_clr(&self) -> bool {
        *self == RisIntl1::RisIntl1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_intl1_set(&self) -> bool {
        *self == RisIntl1::RisIntl1Set
    }
}
#[doc = "Message RAM SEC interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisSec {
    #[doc = "0: CLR"]
    RisSecClr = 0,
    #[doc = "1: SET"]
    RisSecSet = 1,
}
impl From<RisSec> for bool {
    #[inline(always)]
    fn from(variant: RisSec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_SEC` reader - Message RAM SEC interrupt."]
pub type RisSecR = crate::BitReader<RisSec>;
impl RisSecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisSec {
        match self.bits {
            false => RisSec::RisSecClr,
            true => RisSec::RisSecSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_sec_clr(&self) -> bool {
        *self == RisSec::RisSecClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_sec_set(&self) -> bool {
        *self == RisSec::RisSecSet
    }
}
#[doc = "Message RAM DED interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisDed {
    #[doc = "0: CLR"]
    RisDedClr = 0,
    #[doc = "1: SET"]
    RisDedSet = 1,
}
impl From<RisDed> for bool {
    #[inline(always)]
    fn from(variant: RisDed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_DED` reader - Message RAM DED interrupt."]
pub type RisDedR = crate::BitReader<RisDed>;
impl RisDedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisDed {
        match self.bits {
            false => RisDed::RisDedClr,
            true => RisDed::RisDedSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_ded_clr(&self) -> bool {
        *self == RisDed::RisDedClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_ded_set(&self) -> bool {
        *self == RisDed::RisDedSet
    }
}
#[doc = "External Timestamp Counter Overflow interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisExtTsCntrOvfl {
    #[doc = "0: CLR"]
    RisExtTsCntrOvflClr = 0,
    #[doc = "1: SET"]
    RisExtTsCntrOvflSet = 1,
}
impl From<RisExtTsCntrOvfl> for bool {
    #[inline(always)]
    fn from(variant: RisExtTsCntrOvfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_EXT_TS_CNTR_OVFL` reader - External Timestamp Counter Overflow interrupt."]
pub type RisExtTsCntrOvflR = crate::BitReader<RisExtTsCntrOvfl>;
impl RisExtTsCntrOvflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisExtTsCntrOvfl {
        match self.bits {
            false => RisExtTsCntrOvfl::RisExtTsCntrOvflClr,
            true => RisExtTsCntrOvfl::RisExtTsCntrOvflSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_ext_ts_cntr_ovfl_clr(&self) -> bool {
        *self == RisExtTsCntrOvfl::RisExtTsCntrOvflClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_ext_ts_cntr_ovfl_set(&self) -> bool {
        *self == RisExtTsCntrOvfl::RisExtTsCntrOvflSet
    }
}
#[doc = "Clock Stop Wake Up interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisWakeup {
    #[doc = "0: CLR"]
    RisWakeupClr = 0,
    #[doc = "1: SET"]
    RisWakeupSet = 1,
}
impl From<RisWakeup> for bool {
    #[inline(always)]
    fn from(variant: RisWakeup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_WAKEUP` reader - Clock Stop Wake Up interrupt."]
pub type RisWakeupR = crate::BitReader<RisWakeup>;
impl RisWakeupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisWakeup {
        match self.bits {
            false => RisWakeup::RisWakeupClr,
            true => RisWakeup::RisWakeupSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_wakeup_clr(&self) -> bool {
        *self == RisWakeup::RisWakeupClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_wakeup_set(&self) -> bool {
        *self == RisWakeup::RisWakeupSet
    }
}
impl R {
    #[doc = "Bit 0 - MCAN Interrupt Line 0."]
    #[inline(always)]
    pub fn ris_intl0(&self) -> RisIntl0R {
        RisIntl0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCAN Interrupt Line 1."]
    #[inline(always)]
    pub fn ris_intl1(&self) -> RisIntl1R {
        RisIntl1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Message RAM SEC interrupt."]
    #[inline(always)]
    pub fn ris_sec(&self) -> RisSecR {
        RisSecR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Message RAM DED interrupt."]
    #[inline(always)]
    pub fn ris_ded(&self) -> RisDedR {
        RisDedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Timestamp Counter Overflow interrupt."]
    #[inline(always)]
    pub fn ris_ext_ts_cntr_ovfl(&self) -> RisExtTsCntrOvflR {
        RisExtTsCntrOvflR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Stop Wake Up interrupt."]
    #[inline(always)]
    pub fn ris_wakeup(&self) -> RisWakeupR {
        RisWakeupR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0;
}
