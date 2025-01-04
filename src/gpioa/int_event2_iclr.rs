#[doc = "Register `INT_EVENT2_ICLR` writer"]
pub type W = crate::W<IntEvent2IclrSpec>;
#[doc = "DIO16 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrDio16 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrDio16NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrDio16Clr = 1,
}
impl From<IntEvent2IclrDio16> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrDio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_DIO16` writer - DIO16 event"]
pub type IntEvent2IclrDio16W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrDio16>;
impl<'a, REG> IntEvent2IclrDio16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_dio16_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio16::IntEvent2IclrDio16NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_dio16_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio16::IntEvent2IclrDio16Clr)
    }
}
#[doc = "DIO17 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrDio17 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrDio17NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrDio17Clr = 1,
}
impl From<IntEvent2IclrDio17> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrDio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_DIO17` writer - DIO17 event"]
pub type IntEvent2IclrDio17W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrDio17>;
impl<'a, REG> IntEvent2IclrDio17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_dio17_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio17::IntEvent2IclrDio17NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_dio17_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio17::IntEvent2IclrDio17Clr)
    }
}
#[doc = "DIO18 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrDio18 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrDio18NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrDio18Clr = 1,
}
impl From<IntEvent2IclrDio18> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrDio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_DIO18` writer - DIO18 event"]
pub type IntEvent2IclrDio18W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrDio18>;
impl<'a, REG> IntEvent2IclrDio18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_dio18_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio18::IntEvent2IclrDio18NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_dio18_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio18::IntEvent2IclrDio18Clr)
    }
}
#[doc = "DIO19 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrDio19 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrDio19NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrDio19Clr = 1,
}
impl From<IntEvent2IclrDio19> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrDio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_DIO19` writer - DIO19 event"]
pub type IntEvent2IclrDio19W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrDio19>;
impl<'a, REG> IntEvent2IclrDio19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_dio19_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio19::IntEvent2IclrDio19NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_dio19_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio19::IntEvent2IclrDio19Clr)
    }
}
#[doc = "DIO20 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrDio20 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrDio20NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrDio20Clr = 1,
}
impl From<IntEvent2IclrDio20> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrDio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_DIO20` writer - DIO20 event"]
pub type IntEvent2IclrDio20W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrDio20>;
impl<'a, REG> IntEvent2IclrDio20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_dio20_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio20::IntEvent2IclrDio20NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_dio20_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio20::IntEvent2IclrDio20Clr)
    }
}
#[doc = "DIO21 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrDio21 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrDio21NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrDio21Clr = 1,
}
impl From<IntEvent2IclrDio21> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrDio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_DIO21` writer - DIO21 event"]
pub type IntEvent2IclrDio21W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrDio21>;
impl<'a, REG> IntEvent2IclrDio21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_dio21_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio21::IntEvent2IclrDio21NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_dio21_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio21::IntEvent2IclrDio21Clr)
    }
}
#[doc = "DIO22 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrDio22 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrDio22NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrDio22Clr = 1,
}
impl From<IntEvent2IclrDio22> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrDio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_DIO22` writer - DIO22 event"]
pub type IntEvent2IclrDio22W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrDio22>;
impl<'a, REG> IntEvent2IclrDio22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_dio22_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio22::IntEvent2IclrDio22NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_dio22_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio22::IntEvent2IclrDio22Clr)
    }
}
#[doc = "DIO23 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrDio23 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrDio23NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrDio23Clr = 1,
}
impl From<IntEvent2IclrDio23> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrDio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_DIO23` writer - DIO23 event"]
pub type IntEvent2IclrDio23W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrDio23>;
impl<'a, REG> IntEvent2IclrDio23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_dio23_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio23::IntEvent2IclrDio23NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_dio23_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio23::IntEvent2IclrDio23Clr)
    }
}
#[doc = "DIO24 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrDio24 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrDio24NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrDio24Clr = 1,
}
impl From<IntEvent2IclrDio24> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrDio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_DIO24` writer - DIO24 event"]
pub type IntEvent2IclrDio24W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrDio24>;
impl<'a, REG> IntEvent2IclrDio24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_dio24_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio24::IntEvent2IclrDio24NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_dio24_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio24::IntEvent2IclrDio24Clr)
    }
}
#[doc = "DIO25 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrDio25 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrDio25NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrDio25Clr = 1,
}
impl From<IntEvent2IclrDio25> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrDio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_DIO25` writer - DIO25 event"]
pub type IntEvent2IclrDio25W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrDio25>;
impl<'a, REG> IntEvent2IclrDio25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_dio25_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio25::IntEvent2IclrDio25NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_dio25_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio25::IntEvent2IclrDio25Clr)
    }
}
#[doc = "DIO26 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrDio26 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrDio26NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrDio26Clr = 1,
}
impl From<IntEvent2IclrDio26> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrDio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_DIO26` writer - DIO26 event"]
pub type IntEvent2IclrDio26W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrDio26>;
impl<'a, REG> IntEvent2IclrDio26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_dio26_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio26::IntEvent2IclrDio26NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_dio26_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio26::IntEvent2IclrDio26Clr)
    }
}
#[doc = "DIO27 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrDio27 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrDio27NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrDio27Clr = 1,
}
impl From<IntEvent2IclrDio27> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrDio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_DIO27` writer - DIO27 event"]
pub type IntEvent2IclrDio27W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrDio27>;
impl<'a, REG> IntEvent2IclrDio27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_dio27_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio27::IntEvent2IclrDio27NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_dio27_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio27::IntEvent2IclrDio27Clr)
    }
}
#[doc = "DIO28 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrDio28 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrDio28NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrDio28Clr = 1,
}
impl From<IntEvent2IclrDio28> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrDio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_DIO28` writer - DIO28 event"]
pub type IntEvent2IclrDio28W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrDio28>;
impl<'a, REG> IntEvent2IclrDio28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_dio28_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio28::IntEvent2IclrDio28NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_dio28_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio28::IntEvent2IclrDio28Clr)
    }
}
#[doc = "DIO29 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrDio29 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrDio29NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrDio29Clr = 1,
}
impl From<IntEvent2IclrDio29> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrDio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_DIO29` writer - DIO29 event"]
pub type IntEvent2IclrDio29W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrDio29>;
impl<'a, REG> IntEvent2IclrDio29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_dio29_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio29::IntEvent2IclrDio29NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_dio29_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio29::IntEvent2IclrDio29Clr)
    }
}
#[doc = "DIO30 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrDio30 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrDio30NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrDio30Clr = 1,
}
impl From<IntEvent2IclrDio30> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrDio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_DIO30` writer - DIO30 event"]
pub type IntEvent2IclrDio30W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrDio30>;
impl<'a, REG> IntEvent2IclrDio30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_dio30_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio30::IntEvent2IclrDio30NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_dio30_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio30::IntEvent2IclrDio30Clr)
    }
}
#[doc = "DIO31 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IclrDio31 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IclrDio31NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent2IclrDio31Clr = 1,
}
impl From<IntEvent2IclrDio31> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IclrDio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ICLR_DIO31` writer - DIO31 event"]
pub type IntEvent2IclrDio31W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IclrDio31>;
impl<'a, REG> IntEvent2IclrDio31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iclr_dio31_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio31::IntEvent2IclrDio31NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_iclr_dio31_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IclrDio31::IntEvent2IclrDio31Clr)
    }
}
impl W {
    #[doc = "Bit 16 - DIO16 event"]
    #[inline(always)]
    pub fn int_event2_iclr_dio16(&mut self) -> IntEvent2IclrDio16W<IntEvent2IclrSpec> {
        IntEvent2IclrDio16W::new(self, 16)
    }
    #[doc = "Bit 17 - DIO17 event"]
    #[inline(always)]
    pub fn int_event2_iclr_dio17(&mut self) -> IntEvent2IclrDio17W<IntEvent2IclrSpec> {
        IntEvent2IclrDio17W::new(self, 17)
    }
    #[doc = "Bit 18 - DIO18 event"]
    #[inline(always)]
    pub fn int_event2_iclr_dio18(&mut self) -> IntEvent2IclrDio18W<IntEvent2IclrSpec> {
        IntEvent2IclrDio18W::new(self, 18)
    }
    #[doc = "Bit 19 - DIO19 event"]
    #[inline(always)]
    pub fn int_event2_iclr_dio19(&mut self) -> IntEvent2IclrDio19W<IntEvent2IclrSpec> {
        IntEvent2IclrDio19W::new(self, 19)
    }
    #[doc = "Bit 20 - DIO20 event"]
    #[inline(always)]
    pub fn int_event2_iclr_dio20(&mut self) -> IntEvent2IclrDio20W<IntEvent2IclrSpec> {
        IntEvent2IclrDio20W::new(self, 20)
    }
    #[doc = "Bit 21 - DIO21 event"]
    #[inline(always)]
    pub fn int_event2_iclr_dio21(&mut self) -> IntEvent2IclrDio21W<IntEvent2IclrSpec> {
        IntEvent2IclrDio21W::new(self, 21)
    }
    #[doc = "Bit 22 - DIO22 event"]
    #[inline(always)]
    pub fn int_event2_iclr_dio22(&mut self) -> IntEvent2IclrDio22W<IntEvent2IclrSpec> {
        IntEvent2IclrDio22W::new(self, 22)
    }
    #[doc = "Bit 23 - DIO23 event"]
    #[inline(always)]
    pub fn int_event2_iclr_dio23(&mut self) -> IntEvent2IclrDio23W<IntEvent2IclrSpec> {
        IntEvent2IclrDio23W::new(self, 23)
    }
    #[doc = "Bit 24 - DIO24 event"]
    #[inline(always)]
    pub fn int_event2_iclr_dio24(&mut self) -> IntEvent2IclrDio24W<IntEvent2IclrSpec> {
        IntEvent2IclrDio24W::new(self, 24)
    }
    #[doc = "Bit 25 - DIO25 event"]
    #[inline(always)]
    pub fn int_event2_iclr_dio25(&mut self) -> IntEvent2IclrDio25W<IntEvent2IclrSpec> {
        IntEvent2IclrDio25W::new(self, 25)
    }
    #[doc = "Bit 26 - DIO26 event"]
    #[inline(always)]
    pub fn int_event2_iclr_dio26(&mut self) -> IntEvent2IclrDio26W<IntEvent2IclrSpec> {
        IntEvent2IclrDio26W::new(self, 26)
    }
    #[doc = "Bit 27 - DIO27 event"]
    #[inline(always)]
    pub fn int_event2_iclr_dio27(&mut self) -> IntEvent2IclrDio27W<IntEvent2IclrSpec> {
        IntEvent2IclrDio27W::new(self, 27)
    }
    #[doc = "Bit 28 - DIO28 event"]
    #[inline(always)]
    pub fn int_event2_iclr_dio28(&mut self) -> IntEvent2IclrDio28W<IntEvent2IclrSpec> {
        IntEvent2IclrDio28W::new(self, 28)
    }
    #[doc = "Bit 29 - DIO29 event"]
    #[inline(always)]
    pub fn int_event2_iclr_dio29(&mut self) -> IntEvent2IclrDio29W<IntEvent2IclrSpec> {
        IntEvent2IclrDio29W::new(self, 29)
    }
    #[doc = "Bit 30 - DIO30 event"]
    #[inline(always)]
    pub fn int_event2_iclr_dio30(&mut self) -> IntEvent2IclrDio30W<IntEvent2IclrSpec> {
        IntEvent2IclrDio30W::new(self, 30)
    }
    #[doc = "Bit 31 - DIO31 event"]
    #[inline(always)]
    pub fn int_event2_iclr_dio31(&mut self) -> IntEvent2IclrDio31W<IntEvent2IclrSpec> {
        IntEvent2IclrDio31W::new(self, 31)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event2_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2IclrSpec;
impl crate::RegisterSpec for IntEvent2IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event2_iclr::W`](W) writer structure"]
impl crate::Writable for IntEvent2IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT2_ICLR to value 0"]
impl crate::Resettable for IntEvent2IclrSpec {
    const RESET_VALUE: u32 = 0;
}
