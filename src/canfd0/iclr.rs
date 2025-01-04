#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "Clear MCAN Interrupt Line 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrIntl0 {
    #[doc = "0: NO_EFFECT"]
    IclrIntl0NoEffect = 0,
    #[doc = "1: CLR"]
    IclrIntl0Clr = 1,
}
impl From<IclrIntl0> for bool {
    #[inline(always)]
    fn from(variant: IclrIntl0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_INTL0` writer - Clear MCAN Interrupt Line 0."]
pub type IclrIntl0W<'a, REG> = crate::BitWriter<'a, REG, IclrIntl0>;
impl<'a, REG> IclrIntl0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_intl0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrIntl0::IclrIntl0NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_intl0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrIntl0::IclrIntl0Clr)
    }
}
#[doc = "Clear MCAN Interrupt Line 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrIntl1 {
    #[doc = "0: NO_EFFECT"]
    IclrIntl1NoEffect = 0,
    #[doc = "1: CLR"]
    IclrIntl1Clr = 1,
}
impl From<IclrIntl1> for bool {
    #[inline(always)]
    fn from(variant: IclrIntl1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_INTL1` writer - Clear MCAN Interrupt Line 1."]
pub type IclrIntl1W<'a, REG> = crate::BitWriter<'a, REG, IclrIntl1>;
impl<'a, REG> IclrIntl1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_intl1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrIntl1::IclrIntl1NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_intl1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrIntl1::IclrIntl1Clr)
    }
}
#[doc = "Clear Message RAM SEC interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrSec {
    #[doc = "0: NO_EFFECT"]
    IclrSecNoEffect = 0,
    #[doc = "1: CLR"]
    IclrSecClr = 1,
}
impl From<IclrSec> for bool {
    #[inline(always)]
    fn from(variant: IclrSec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_SEC` writer - Clear Message RAM SEC interrupt."]
pub type IclrSecW<'a, REG> = crate::BitWriter<'a, REG, IclrSec>;
impl<'a, REG> IclrSecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_sec_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrSec::IclrSecNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_sec_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrSec::IclrSecClr)
    }
}
#[doc = "Clear Message RAM DED interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrDed {
    #[doc = "0: NO_EFFECT"]
    IclrDedNoEffect = 0,
    #[doc = "1: CLR"]
    IclrDedClr = 1,
}
impl From<IclrDed> for bool {
    #[inline(always)]
    fn from(variant: IclrDed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_DED` writer - Clear Message RAM DED interrupt."]
pub type IclrDedW<'a, REG> = crate::BitWriter<'a, REG, IclrDed>;
impl<'a, REG> IclrDedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_ded_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrDed::IclrDedNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_ded_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrDed::IclrDedClr)
    }
}
#[doc = "Clear External Timestamp Counter Overflow interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrExtTsCntrOvfl {
    #[doc = "0: NO_EFFECT"]
    IclrExtTsCntrOvflNoEffect = 0,
    #[doc = "1: CLR"]
    IclrExtTsCntrOvflClr = 1,
}
impl From<IclrExtTsCntrOvfl> for bool {
    #[inline(always)]
    fn from(variant: IclrExtTsCntrOvfl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_EXT_TS_CNTR_OVFL` writer - Clear External Timestamp Counter Overflow interrupt."]
pub type IclrExtTsCntrOvflW<'a, REG> = crate::BitWriter<'a, REG, IclrExtTsCntrOvfl>;
impl<'a, REG> IclrExtTsCntrOvflW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_ext_ts_cntr_ovfl_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrExtTsCntrOvfl::IclrExtTsCntrOvflNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_ext_ts_cntr_ovfl_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrExtTsCntrOvfl::IclrExtTsCntrOvflClr)
    }
}
#[doc = "Clear Clock Stop Wake Up interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrWakeup {
    #[doc = "0: NO_EFFECT"]
    IclrWakeupNoEffect = 0,
    #[doc = "1: CLR"]
    IclrWakeupClr = 1,
}
impl From<IclrWakeup> for bool {
    #[inline(always)]
    fn from(variant: IclrWakeup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_WAKEUP` writer - Clear Clock Stop Wake Up interrupt."]
pub type IclrWakeupW<'a, REG> = crate::BitWriter<'a, REG, IclrWakeup>;
impl<'a, REG> IclrWakeupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_wakeup_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrWakeup::IclrWakeupNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_wakeup_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrWakeup::IclrWakeupClr)
    }
}
impl W {
    #[doc = "Bit 0 - Clear MCAN Interrupt Line 0."]
    #[inline(always)]
    pub fn iclr_intl0(&mut self) -> IclrIntl0W<IclrSpec> {
        IclrIntl0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear MCAN Interrupt Line 1."]
    #[inline(always)]
    pub fn iclr_intl1(&mut self) -> IclrIntl1W<IclrSpec> {
        IclrIntl1W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Message RAM SEC interrupt."]
    #[inline(always)]
    pub fn iclr_sec(&mut self) -> IclrSecW<IclrSpec> {
        IclrSecW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Message RAM DED interrupt."]
    #[inline(always)]
    pub fn iclr_ded(&mut self) -> IclrDedW<IclrSpec> {
        IclrDedW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear External Timestamp Counter Overflow interrupt."]
    #[inline(always)]
    pub fn iclr_ext_ts_cntr_ovfl(&mut self) -> IclrExtTsCntrOvflW<IclrSpec> {
        IclrExtTsCntrOvflW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear Clock Stop Wake Up interrupt."]
    #[inline(always)]
    pub fn iclr_wakeup(&mut self) -> IclrWakeupW<IclrSpec> {
        IclrWakeupW::new(self, 5)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IclrSpec;
impl crate::RegisterSpec for IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iclr::W`](W) writer structure"]
impl crate::Writable for IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for IclrSpec {
    const RESET_VALUE: u32 = 0;
}
