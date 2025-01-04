#[doc = "Register `ISET` writer"]
pub type W = crate::W<IsetSpec>;
#[doc = "Set MCAN Interrupt Line 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetIntl0 {
    #[doc = "0: NO_EFFECT"]
    IsetIntl0NoEffect = 0,
    #[doc = "1: SET"]
    IsetIntl0Set = 1,
}
impl From<IsetIntl0> for bool {
    #[inline(always)]
    fn from(variant: IsetIntl0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_INTL0` writer - Set MCAN Interrupt Line 0."]
pub type IsetIntl0W<'a, REG> = crate::BitWriter<'a, REG, IsetIntl0>;
impl<'a, REG> IsetIntl0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_intl0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetIntl0::IsetIntl0NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_intl0_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetIntl0::IsetIntl0Set)
    }
}
#[doc = "Set MCAN Interrupt Line 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetIntl1 {
    #[doc = "0: NO_EFFECT"]
    IsetIntl1NoEffect = 0,
    #[doc = "1: SET"]
    IsetIntl1Set = 1,
}
impl From<IsetIntl1> for bool {
    #[inline(always)]
    fn from(variant: IsetIntl1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_INTL1` writer - Set MCAN Interrupt Line 1."]
pub type IsetIntl1W<'a, REG> = crate::BitWriter<'a, REG, IsetIntl1>;
impl<'a, REG> IsetIntl1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_intl1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetIntl1::IsetIntl1NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_intl1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetIntl1::IsetIntl1Set)
    }
}
#[doc = "Set Message RAM SEC interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetSec {
    #[doc = "0: NO_EFFECT"]
    IsetSecNoEffect = 0,
    #[doc = "1: SET"]
    IsetSecSet = 1,
}
impl From<IsetSec> for bool {
    #[inline(always)]
    fn from(variant: IsetSec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_SEC` writer - Set Message RAM SEC interrupt."]
pub type IsetSecW<'a, REG> = crate::BitWriter<'a, REG, IsetSec>;
impl<'a, REG> IsetSecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_sec_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetSec::IsetSecNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_sec_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetSec::IsetSecSet)
    }
}
#[doc = "Set Message RAM DED interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetDed {
    #[doc = "0: NO_EFFECT"]
    IsetDedNoEffect = 0,
    #[doc = "1: SET"]
    IsetDedSet = 1,
}
impl From<IsetDed> for bool {
    #[inline(always)]
    fn from(variant: IsetDed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_DED` writer - Set Message RAM DED interrupt."]
pub type IsetDedW<'a, REG> = crate::BitWriter<'a, REG, IsetDed>;
impl<'a, REG> IsetDedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_ded_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDed::IsetDedNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_ded_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDed::IsetDedSet)
    }
}
#[doc = "Set External Timestamp Counter Overflow interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetExtTsCntrOvfl {
    #[doc = "0: NO_EFFECT"]
    IsetExtTsCntrOvflNoEffect = 0,
    #[doc = "1: SET"]
    IsetExtTsCntrOvflSet = 1,
}
impl From<IsetExtTsCntrOvfl> for bool {
    #[inline(always)]
    fn from(variant: IsetExtTsCntrOvfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_EXT_TS_CNTR_OVFL` writer - Set External Timestamp Counter Overflow interrupt."]
pub type IsetExtTsCntrOvflW<'a, REG> = crate::BitWriter<'a, REG, IsetExtTsCntrOvfl>;
impl<'a, REG> IsetExtTsCntrOvflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_ext_ts_cntr_ovfl_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetExtTsCntrOvfl::IsetExtTsCntrOvflNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_ext_ts_cntr_ovfl_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetExtTsCntrOvfl::IsetExtTsCntrOvflSet)
    }
}
#[doc = "Set Clock Stop Wake Up interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetWakeup {
    #[doc = "0: NO_EFFECT"]
    IsetWakeupNoEffect = 0,
    #[doc = "1: SET"]
    IsetWakeupSet = 1,
}
impl From<IsetWakeup> for bool {
    #[inline(always)]
    fn from(variant: IsetWakeup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_WAKEUP` writer - Set Clock Stop Wake Up interrupt."]
pub type IsetWakeupW<'a, REG> = crate::BitWriter<'a, REG, IsetWakeup>;
impl<'a, REG> IsetWakeupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_wakeup_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetWakeup::IsetWakeupNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_wakeup_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetWakeup::IsetWakeupSet)
    }
}
impl W {
    #[doc = "Bit 0 - Set MCAN Interrupt Line 0."]
    #[inline(always)]
    pub fn iset_intl0(&mut self) -> IsetIntl0W<IsetSpec> {
        IsetIntl0W::new(self, 0)
    }
    #[doc = "Bit 1 - Set MCAN Interrupt Line 1."]
    #[inline(always)]
    pub fn iset_intl1(&mut self) -> IsetIntl1W<IsetSpec> {
        IsetIntl1W::new(self, 1)
    }
    #[doc = "Bit 2 - Set Message RAM SEC interrupt."]
    #[inline(always)]
    pub fn iset_sec(&mut self) -> IsetSecW<IsetSpec> {
        IsetSecW::new(self, 2)
    }
    #[doc = "Bit 3 - Set Message RAM DED interrupt."]
    #[inline(always)]
    pub fn iset_ded(&mut self) -> IsetDedW<IsetSpec> {
        IsetDedW::new(self, 3)
    }
    #[doc = "Bit 4 - Set External Timestamp Counter Overflow interrupt."]
    #[inline(always)]
    pub fn iset_ext_ts_cntr_ovfl(&mut self) -> IsetExtTsCntrOvflW<IsetSpec> {
        IsetExtTsCntrOvflW::new(self, 4)
    }
    #[doc = "Bit 5 - Set Clock Stop Wake Up interrupt."]
    #[inline(always)]
    pub fn iset_wakeup(&mut self) -> IsetWakeupW<IsetSpec> {
        IsetWakeupW::new(self, 5)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsetSpec;
impl crate::RegisterSpec for IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iset::W`](W) writer structure"]
impl crate::Writable for IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISET to value 0"]
impl crate::Resettable for IsetSpec {
    const RESET_VALUE: u32 = 0;
}
