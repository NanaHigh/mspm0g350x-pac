#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Masked MCAN Interrupt Line 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisIntl0 {
    #[doc = "0: CLR"]
    MisIntl0Clr = 0,
    #[doc = "1: SET"]
    MisIntl0Set = 1,
}
impl From<MisIntl0> for bool {
    #[inline(always)]
    fn from(variant: MisIntl0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_INTL0` reader - Masked MCAN Interrupt Line 0."]
pub type MisIntl0R = crate::BitReader<MisIntl0>;
impl MisIntl0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisIntl0 {
        match self.bits {
            false => MisIntl0::MisIntl0Clr,
            true => MisIntl0::MisIntl0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_intl0_clr(&self) -> bool {
        *self == MisIntl0::MisIntl0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_intl0_set(&self) -> bool {
        *self == MisIntl0::MisIntl0Set
    }
}
#[doc = "Masked MCAN Interrupt Line 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisIntl1 {
    #[doc = "0: CLR"]
    MisIntl1Clr = 0,
    #[doc = "1: SET"]
    MisIntl1Set = 1,
}
impl From<MisIntl1> for bool {
    #[inline(always)]
    fn from(variant: MisIntl1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_INTL1` reader - Masked MCAN Interrupt Line 1."]
pub type MisIntl1R = crate::BitReader<MisIntl1>;
impl MisIntl1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisIntl1 {
        match self.bits {
            false => MisIntl1::MisIntl1Clr,
            true => MisIntl1::MisIntl1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_intl1_clr(&self) -> bool {
        *self == MisIntl1::MisIntl1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_intl1_set(&self) -> bool {
        *self == MisIntl1::MisIntl1Set
    }
}
#[doc = "Masked Message RAM SEC interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisSec {
    #[doc = "0: CLR"]
    MisSecClr = 0,
    #[doc = "1: SET"]
    MisSecSet = 1,
}
impl From<MisSec> for bool {
    #[inline(always)]
    fn from(variant: MisSec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_SEC` reader - Masked Message RAM SEC interrupt."]
pub type MisSecR = crate::BitReader<MisSec>;
impl MisSecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisSec {
        match self.bits {
            false => MisSec::MisSecClr,
            true => MisSec::MisSecSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_sec_clr(&self) -> bool {
        *self == MisSec::MisSecClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_sec_set(&self) -> bool {
        *self == MisSec::MisSecSet
    }
}
#[doc = "Masked Message RAM DED interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisDed {
    #[doc = "0: CLR"]
    MisDedClr = 0,
    #[doc = "1: SET"]
    MisDedSet = 1,
}
impl From<MisDed> for bool {
    #[inline(always)]
    fn from(variant: MisDed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_DED` reader - Masked Message RAM DED interrupt."]
pub type MisDedR = crate::BitReader<MisDed>;
impl MisDedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisDed {
        match self.bits {
            false => MisDed::MisDedClr,
            true => MisDed::MisDedSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_ded_clr(&self) -> bool {
        *self == MisDed::MisDedClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_ded_set(&self) -> bool {
        *self == MisDed::MisDedSet
    }
}
#[doc = "Masked External Timestamp Counter Overflow interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisExtTsCntrOvfl {
    #[doc = "0: CLR"]
    MisExtTsCntrOvflClr = 0,
    #[doc = "1: SET"]
    MisExtTsCntrOvflSet = 1,
}
impl From<MisExtTsCntrOvfl> for bool {
    #[inline(always)]
    fn from(variant: MisExtTsCntrOvfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_EXT_TS_CNTR_OVFL` reader - Masked External Timestamp Counter Overflow interrupt."]
pub type MisExtTsCntrOvflR = crate::BitReader<MisExtTsCntrOvfl>;
impl MisExtTsCntrOvflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisExtTsCntrOvfl {
        match self.bits {
            false => MisExtTsCntrOvfl::MisExtTsCntrOvflClr,
            true => MisExtTsCntrOvfl::MisExtTsCntrOvflSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_ext_ts_cntr_ovfl_clr(&self) -> bool {
        *self == MisExtTsCntrOvfl::MisExtTsCntrOvflClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_ext_ts_cntr_ovfl_set(&self) -> bool {
        *self == MisExtTsCntrOvfl::MisExtTsCntrOvflSet
    }
}
#[doc = "Masked Clock Stop Wake Up interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisWakeup {
    #[doc = "0: CLR"]
    MisWakeupClr = 0,
    #[doc = "1: SET"]
    MisWakeupSet = 1,
}
impl From<MisWakeup> for bool {
    #[inline(always)]
    fn from(variant: MisWakeup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_WAKEUP` reader - Masked Clock Stop Wake Up interrupt."]
pub type MisWakeupR = crate::BitReader<MisWakeup>;
impl MisWakeupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisWakeup {
        match self.bits {
            false => MisWakeup::MisWakeupClr,
            true => MisWakeup::MisWakeupSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_wakeup_clr(&self) -> bool {
        *self == MisWakeup::MisWakeupClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_wakeup_set(&self) -> bool {
        *self == MisWakeup::MisWakeupSet
    }
}
impl R {
    #[doc = "Bit 0 - Masked MCAN Interrupt Line 0."]
    #[inline(always)]
    pub fn mis_intl0(&self) -> MisIntl0R {
        MisIntl0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Masked MCAN Interrupt Line 1."]
    #[inline(always)]
    pub fn mis_intl1(&self) -> MisIntl1R {
        MisIntl1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Masked Message RAM SEC interrupt."]
    #[inline(always)]
    pub fn mis_sec(&self) -> MisSecR {
        MisSecR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Masked Message RAM DED interrupt."]
    #[inline(always)]
    pub fn mis_ded(&self) -> MisDedR {
        MisDedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Masked External Timestamp Counter Overflow interrupt."]
    #[inline(always)]
    pub fn mis_ext_ts_cntr_ovfl(&self) -> MisExtTsCntrOvflR {
        MisExtTsCntrOvflR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Masked Clock Stop Wake Up interrupt."]
    #[inline(always)]
    pub fn mis_wakeup(&self) -> MisWakeupR {
        MisWakeupR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {
    const RESET_VALUE: u32 = 0;
}
