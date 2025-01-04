#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "MCAN Interrupt Line 0 mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskIntl0 {
    #[doc = "0: CLR"]
    ImaskIntl0Clr = 0,
    #[doc = "1: SET"]
    ImaskIntl0Set = 1,
}
impl From<ImaskIntl0> for bool {
    #[inline(always)]
    fn from(variant: ImaskIntl0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_INTL0` reader - MCAN Interrupt Line 0 mask."]
pub type ImaskIntl0R = crate::BitReader<ImaskIntl0>;
impl ImaskIntl0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskIntl0 {
        match self.bits {
            false => ImaskIntl0::ImaskIntl0Clr,
            true => ImaskIntl0::ImaskIntl0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_intl0_clr(&self) -> bool {
        *self == ImaskIntl0::ImaskIntl0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_intl0_set(&self) -> bool {
        *self == ImaskIntl0::ImaskIntl0Set
    }
}
#[doc = "Field `IMASK_INTL0` writer - MCAN Interrupt Line 0 mask."]
pub type ImaskIntl0W<'a, REG> = crate::BitWriter<'a, REG, ImaskIntl0>;
impl<'a, REG> ImaskIntl0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_intl0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskIntl0::ImaskIntl0Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_intl0_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskIntl0::ImaskIntl0Set)
    }
}
#[doc = "MCAN Interrupt Line 1 mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskIntl1 {
    #[doc = "0: CLR"]
    ImaskIntl1Clr = 0,
    #[doc = "1: SET"]
    ImaskIntl1Set = 1,
}
impl From<ImaskIntl1> for bool {
    #[inline(always)]
    fn from(variant: ImaskIntl1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_INTL1` reader - MCAN Interrupt Line 1 mask."]
pub type ImaskIntl1R = crate::BitReader<ImaskIntl1>;
impl ImaskIntl1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskIntl1 {
        match self.bits {
            false => ImaskIntl1::ImaskIntl1Clr,
            true => ImaskIntl1::ImaskIntl1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_intl1_clr(&self) -> bool {
        *self == ImaskIntl1::ImaskIntl1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_intl1_set(&self) -> bool {
        *self == ImaskIntl1::ImaskIntl1Set
    }
}
#[doc = "Field `IMASK_INTL1` writer - MCAN Interrupt Line 1 mask."]
pub type ImaskIntl1W<'a, REG> = crate::BitWriter<'a, REG, ImaskIntl1>;
impl<'a, REG> ImaskIntl1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_intl1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskIntl1::ImaskIntl1Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_intl1_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskIntl1::ImaskIntl1Set)
    }
}
#[doc = "Message RAM SEC interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskSec {
    #[doc = "0: CLR"]
    ImaskSecClr = 0,
    #[doc = "1: SET"]
    ImaskSecSet = 1,
}
impl From<ImaskSec> for bool {
    #[inline(always)]
    fn from(variant: ImaskSec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_SEC` reader - Message RAM SEC interrupt mask."]
pub type ImaskSecR = crate::BitReader<ImaskSec>;
impl ImaskSecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskSec {
        match self.bits {
            false => ImaskSec::ImaskSecClr,
            true => ImaskSec::ImaskSecSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_sec_clr(&self) -> bool {
        *self == ImaskSec::ImaskSecClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_sec_set(&self) -> bool {
        *self == ImaskSec::ImaskSecSet
    }
}
#[doc = "Field `IMASK_SEC` writer - Message RAM SEC interrupt mask."]
pub type ImaskSecW<'a, REG> = crate::BitWriter<'a, REG, ImaskSec>;
impl<'a, REG> ImaskSecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_sec_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskSec::ImaskSecClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_sec_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskSec::ImaskSecSet)
    }
}
#[doc = "Massage RAM DED interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskDed {
    #[doc = "0: CLR"]
    ImaskDedClr = 0,
    #[doc = "1: SET"]
    ImaskDedSet = 1,
}
impl From<ImaskDed> for bool {
    #[inline(always)]
    fn from(variant: ImaskDed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_DED` reader - Massage RAM DED interrupt mask."]
pub type ImaskDedR = crate::BitReader<ImaskDed>;
impl ImaskDedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskDed {
        match self.bits {
            false => ImaskDed::ImaskDedClr,
            true => ImaskDed::ImaskDedSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_ded_clr(&self) -> bool {
        *self == ImaskDed::ImaskDedClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_ded_set(&self) -> bool {
        *self == ImaskDed::ImaskDedSet
    }
}
#[doc = "Field `IMASK_DED` writer - Massage RAM DED interrupt mask."]
pub type ImaskDedW<'a, REG> = crate::BitWriter<'a, REG, ImaskDed>;
impl<'a, REG> ImaskDedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_ded_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskDed::ImaskDedClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_ded_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskDed::ImaskDedSet)
    }
}
#[doc = "External Timestamp Counter Overflow interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskExtTsCntrOvfl {
    #[doc = "0: CLR"]
    ImaskExtTsCntrOvflClr = 0,
    #[doc = "1: SET"]
    ImaskExtTsCntrOvflSet = 1,
}
impl From<ImaskExtTsCntrOvfl> for bool {
    #[inline(always)]
    fn from(variant: ImaskExtTsCntrOvfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_EXT_TS_CNTR_OVFL` reader - External Timestamp Counter Overflow interrupt mask."]
pub type ImaskExtTsCntrOvflR = crate::BitReader<ImaskExtTsCntrOvfl>;
impl ImaskExtTsCntrOvflR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskExtTsCntrOvfl {
        match self.bits {
            false => ImaskExtTsCntrOvfl::ImaskExtTsCntrOvflClr,
            true => ImaskExtTsCntrOvfl::ImaskExtTsCntrOvflSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_ext_ts_cntr_ovfl_clr(&self) -> bool {
        *self == ImaskExtTsCntrOvfl::ImaskExtTsCntrOvflClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_ext_ts_cntr_ovfl_set(&self) -> bool {
        *self == ImaskExtTsCntrOvfl::ImaskExtTsCntrOvflSet
    }
}
#[doc = "Field `IMASK_EXT_TS_CNTR_OVFL` writer - External Timestamp Counter Overflow interrupt mask."]
pub type ImaskExtTsCntrOvflW<'a, REG> = crate::BitWriter<'a, REG, ImaskExtTsCntrOvfl>;
impl<'a, REG> ImaskExtTsCntrOvflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_ext_ts_cntr_ovfl_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskExtTsCntrOvfl::ImaskExtTsCntrOvflClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_ext_ts_cntr_ovfl_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskExtTsCntrOvfl::ImaskExtTsCntrOvflSet)
    }
}
#[doc = "Clock Stop Wake Up interrupt mask.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskWakeup {
    #[doc = "0: CLR"]
    ImaskWakeupClr = 0,
    #[doc = "1: SET"]
    ImaskWakeupSet = 1,
}
impl From<ImaskWakeup> for bool {
    #[inline(always)]
    fn from(variant: ImaskWakeup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_WAKEUP` reader - Clock Stop Wake Up interrupt mask."]
pub type ImaskWakeupR = crate::BitReader<ImaskWakeup>;
impl ImaskWakeupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskWakeup {
        match self.bits {
            false => ImaskWakeup::ImaskWakeupClr,
            true => ImaskWakeup::ImaskWakeupSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_imask_wakeup_clr(&self) -> bool {
        *self == ImaskWakeup::ImaskWakeupClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_imask_wakeup_set(&self) -> bool {
        *self == ImaskWakeup::ImaskWakeupSet
    }
}
#[doc = "Field `IMASK_WAKEUP` writer - Clock Stop Wake Up interrupt mask."]
pub type ImaskWakeupW<'a, REG> = crate::BitWriter<'a, REG, ImaskWakeup>;
impl<'a, REG> ImaskWakeupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn imask_wakeup_clr(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskWakeup::ImaskWakeupClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn imask_wakeup_set(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskWakeup::ImaskWakeupSet)
    }
}
impl R {
    #[doc = "Bit 0 - MCAN Interrupt Line 0 mask."]
    #[inline(always)]
    pub fn imask_intl0(&self) -> ImaskIntl0R {
        ImaskIntl0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MCAN Interrupt Line 1 mask."]
    #[inline(always)]
    pub fn imask_intl1(&self) -> ImaskIntl1R {
        ImaskIntl1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Message RAM SEC interrupt mask."]
    #[inline(always)]
    pub fn imask_sec(&self) -> ImaskSecR {
        ImaskSecR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Massage RAM DED interrupt mask."]
    #[inline(always)]
    pub fn imask_ded(&self) -> ImaskDedR {
        ImaskDedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External Timestamp Counter Overflow interrupt mask."]
    #[inline(always)]
    pub fn imask_ext_ts_cntr_ovfl(&self) -> ImaskExtTsCntrOvflR {
        ImaskExtTsCntrOvflR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clock Stop Wake Up interrupt mask."]
    #[inline(always)]
    pub fn imask_wakeup(&self) -> ImaskWakeupR {
        ImaskWakeupR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCAN Interrupt Line 0 mask."]
    #[inline(always)]
    pub fn imask_intl0(&mut self) -> ImaskIntl0W<ImaskSpec> {
        ImaskIntl0W::new(self, 0)
    }
    #[doc = "Bit 1 - MCAN Interrupt Line 1 mask."]
    #[inline(always)]
    pub fn imask_intl1(&mut self) -> ImaskIntl1W<ImaskSpec> {
        ImaskIntl1W::new(self, 1)
    }
    #[doc = "Bit 2 - Message RAM SEC interrupt mask."]
    #[inline(always)]
    pub fn imask_sec(&mut self) -> ImaskSecW<ImaskSpec> {
        ImaskSecW::new(self, 2)
    }
    #[doc = "Bit 3 - Massage RAM DED interrupt mask."]
    #[inline(always)]
    pub fn imask_ded(&mut self) -> ImaskDedW<ImaskSpec> {
        ImaskDedW::new(self, 3)
    }
    #[doc = "Bit 4 - External Timestamp Counter Overflow interrupt mask."]
    #[inline(always)]
    pub fn imask_ext_ts_cntr_ovfl(&mut self) -> ImaskExtTsCntrOvflW<ImaskSpec> {
        ImaskExtTsCntrOvflW::new(self, 4)
    }
    #[doc = "Bit 5 - Clock Stop Wake Up interrupt mask."]
    #[inline(always)]
    pub fn imask_wakeup(&mut self) -> ImaskWakeupW<ImaskSpec> {
        ImaskWakeupW::new(self, 5)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImaskSpec;
impl crate::RegisterSpec for ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask::R`](R) reader structure"]
impl crate::Readable for ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`imask::W`](W) writer structure"]
impl crate::Writable for ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMASK to value 0"]
impl crate::Resettable for ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
