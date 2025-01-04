#[doc = "Register `INT_EVENT2_ISET` writer"]
pub type W = crate::W<IntEvent2IsetSpec>;
#[doc = "DIO16 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetDio16 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetDio16NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetDio16Set = 1,
}
impl From<IntEvent2IsetDio16> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetDio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_DIO16` writer - DIO16 event"]
pub type IntEvent2IsetDio16W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetDio16>;
impl<'a, REG> IntEvent2IsetDio16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_dio16_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio16::IntEvent2IsetDio16NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_dio16_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio16::IntEvent2IsetDio16Set)
    }
}
#[doc = "DIO17 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetDio17 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetDio17NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetDio17Set = 1,
}
impl From<IntEvent2IsetDio17> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetDio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_DIO17` writer - DIO17 event"]
pub type IntEvent2IsetDio17W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetDio17>;
impl<'a, REG> IntEvent2IsetDio17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_dio17_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio17::IntEvent2IsetDio17NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_dio17_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio17::IntEvent2IsetDio17Set)
    }
}
#[doc = "DIO18 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetDio18 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetDio18NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetDio18Set = 1,
}
impl From<IntEvent2IsetDio18> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetDio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_DIO18` writer - DIO18 event"]
pub type IntEvent2IsetDio18W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetDio18>;
impl<'a, REG> IntEvent2IsetDio18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_dio18_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio18::IntEvent2IsetDio18NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_dio18_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio18::IntEvent2IsetDio18Set)
    }
}
#[doc = "DIO19 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetDio19 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetDio19NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetDio19Set = 1,
}
impl From<IntEvent2IsetDio19> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetDio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_DIO19` writer - DIO19 event"]
pub type IntEvent2IsetDio19W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetDio19>;
impl<'a, REG> IntEvent2IsetDio19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_dio19_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio19::IntEvent2IsetDio19NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_dio19_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio19::IntEvent2IsetDio19Set)
    }
}
#[doc = "DIO20 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetDio20 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetDio20NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetDio20Set = 1,
}
impl From<IntEvent2IsetDio20> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetDio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_DIO20` writer - DIO20 event"]
pub type IntEvent2IsetDio20W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetDio20>;
impl<'a, REG> IntEvent2IsetDio20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_dio20_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio20::IntEvent2IsetDio20NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_dio20_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio20::IntEvent2IsetDio20Set)
    }
}
#[doc = "DIO21 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetDio21 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetDio21NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetDio21Set = 1,
}
impl From<IntEvent2IsetDio21> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetDio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_DIO21` writer - DIO21 event"]
pub type IntEvent2IsetDio21W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetDio21>;
impl<'a, REG> IntEvent2IsetDio21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_dio21_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio21::IntEvent2IsetDio21NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_dio21_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio21::IntEvent2IsetDio21Set)
    }
}
#[doc = "DIO22 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetDio22 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetDio22NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetDio22Set = 1,
}
impl From<IntEvent2IsetDio22> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetDio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_DIO22` writer - DIO22 event"]
pub type IntEvent2IsetDio22W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetDio22>;
impl<'a, REG> IntEvent2IsetDio22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_dio22_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio22::IntEvent2IsetDio22NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_dio22_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio22::IntEvent2IsetDio22Set)
    }
}
#[doc = "DIO23 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetDio23 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetDio23NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetDio23Set = 1,
}
impl From<IntEvent2IsetDio23> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetDio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_DIO23` writer - DIO23 event"]
pub type IntEvent2IsetDio23W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetDio23>;
impl<'a, REG> IntEvent2IsetDio23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_dio23_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio23::IntEvent2IsetDio23NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_dio23_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio23::IntEvent2IsetDio23Set)
    }
}
#[doc = "DIO24 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetDio24 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetDio24NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetDio24Set = 1,
}
impl From<IntEvent2IsetDio24> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetDio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_DIO24` writer - DIO24 event"]
pub type IntEvent2IsetDio24W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetDio24>;
impl<'a, REG> IntEvent2IsetDio24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_dio24_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio24::IntEvent2IsetDio24NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_dio24_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio24::IntEvent2IsetDio24Set)
    }
}
#[doc = "DIO25 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetDio25 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetDio25NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetDio25Set = 1,
}
impl From<IntEvent2IsetDio25> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetDio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_DIO25` writer - DIO25 event"]
pub type IntEvent2IsetDio25W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetDio25>;
impl<'a, REG> IntEvent2IsetDio25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_dio25_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio25::IntEvent2IsetDio25NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_dio25_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio25::IntEvent2IsetDio25Set)
    }
}
#[doc = "DIO26 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetDio26 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetDio26NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetDio26Set = 1,
}
impl From<IntEvent2IsetDio26> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetDio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_DIO26` writer - DIO26 event"]
pub type IntEvent2IsetDio26W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetDio26>;
impl<'a, REG> IntEvent2IsetDio26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_dio26_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio26::IntEvent2IsetDio26NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_dio26_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio26::IntEvent2IsetDio26Set)
    }
}
#[doc = "DIO27 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetDio27 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetDio27NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetDio27Set = 1,
}
impl From<IntEvent2IsetDio27> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetDio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_DIO27` writer - DIO27 event"]
pub type IntEvent2IsetDio27W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetDio27>;
impl<'a, REG> IntEvent2IsetDio27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_dio27_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio27::IntEvent2IsetDio27NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_dio27_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio27::IntEvent2IsetDio27Set)
    }
}
#[doc = "DIO28 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetDio28 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetDio28NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetDio28Set = 1,
}
impl From<IntEvent2IsetDio28> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetDio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_DIO28` writer - DIO28 event"]
pub type IntEvent2IsetDio28W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetDio28>;
impl<'a, REG> IntEvent2IsetDio28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_dio28_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio28::IntEvent2IsetDio28NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_dio28_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio28::IntEvent2IsetDio28Set)
    }
}
#[doc = "DIO29 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetDio29 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetDio29NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetDio29Set = 1,
}
impl From<IntEvent2IsetDio29> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetDio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_DIO29` writer - DIO29 event"]
pub type IntEvent2IsetDio29W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetDio29>;
impl<'a, REG> IntEvent2IsetDio29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_dio29_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio29::IntEvent2IsetDio29NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_dio29_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio29::IntEvent2IsetDio29Set)
    }
}
#[doc = "DIO30 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetDio30 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetDio30NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetDio30Set = 1,
}
impl From<IntEvent2IsetDio30> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetDio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_DIO30` writer - DIO30 event"]
pub type IntEvent2IsetDio30W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetDio30>;
impl<'a, REG> IntEvent2IsetDio30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_dio30_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio30::IntEvent2IsetDio30NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_dio30_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio30::IntEvent2IsetDio30Set)
    }
}
#[doc = "DIO31 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2IsetDio31 {
    #[doc = "0: NO_EFFECT"]
    IntEvent2IsetDio31NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent2IsetDio31Set = 1,
}
impl From<IntEvent2IsetDio31> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2IsetDio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_ISET_DIO31` writer - DIO31 event"]
pub type IntEvent2IsetDio31W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2IsetDio31>;
impl<'a, REG> IntEvent2IsetDio31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event2_iset_dio31_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio31::IntEvent2IsetDio31NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_iset_dio31_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2IsetDio31::IntEvent2IsetDio31Set)
    }
}
impl W {
    #[doc = "Bit 16 - DIO16 event"]
    #[inline(always)]
    pub fn int_event2_iset_dio16(&mut self) -> IntEvent2IsetDio16W<IntEvent2IsetSpec> {
        IntEvent2IsetDio16W::new(self, 16)
    }
    #[doc = "Bit 17 - DIO17 event"]
    #[inline(always)]
    pub fn int_event2_iset_dio17(&mut self) -> IntEvent2IsetDio17W<IntEvent2IsetSpec> {
        IntEvent2IsetDio17W::new(self, 17)
    }
    #[doc = "Bit 18 - DIO18 event"]
    #[inline(always)]
    pub fn int_event2_iset_dio18(&mut self) -> IntEvent2IsetDio18W<IntEvent2IsetSpec> {
        IntEvent2IsetDio18W::new(self, 18)
    }
    #[doc = "Bit 19 - DIO19 event"]
    #[inline(always)]
    pub fn int_event2_iset_dio19(&mut self) -> IntEvent2IsetDio19W<IntEvent2IsetSpec> {
        IntEvent2IsetDio19W::new(self, 19)
    }
    #[doc = "Bit 20 - DIO20 event"]
    #[inline(always)]
    pub fn int_event2_iset_dio20(&mut self) -> IntEvent2IsetDio20W<IntEvent2IsetSpec> {
        IntEvent2IsetDio20W::new(self, 20)
    }
    #[doc = "Bit 21 - DIO21 event"]
    #[inline(always)]
    pub fn int_event2_iset_dio21(&mut self) -> IntEvent2IsetDio21W<IntEvent2IsetSpec> {
        IntEvent2IsetDio21W::new(self, 21)
    }
    #[doc = "Bit 22 - DIO22 event"]
    #[inline(always)]
    pub fn int_event2_iset_dio22(&mut self) -> IntEvent2IsetDio22W<IntEvent2IsetSpec> {
        IntEvent2IsetDio22W::new(self, 22)
    }
    #[doc = "Bit 23 - DIO23 event"]
    #[inline(always)]
    pub fn int_event2_iset_dio23(&mut self) -> IntEvent2IsetDio23W<IntEvent2IsetSpec> {
        IntEvent2IsetDio23W::new(self, 23)
    }
    #[doc = "Bit 24 - DIO24 event"]
    #[inline(always)]
    pub fn int_event2_iset_dio24(&mut self) -> IntEvent2IsetDio24W<IntEvent2IsetSpec> {
        IntEvent2IsetDio24W::new(self, 24)
    }
    #[doc = "Bit 25 - DIO25 event"]
    #[inline(always)]
    pub fn int_event2_iset_dio25(&mut self) -> IntEvent2IsetDio25W<IntEvent2IsetSpec> {
        IntEvent2IsetDio25W::new(self, 25)
    }
    #[doc = "Bit 26 - DIO26 event"]
    #[inline(always)]
    pub fn int_event2_iset_dio26(&mut self) -> IntEvent2IsetDio26W<IntEvent2IsetSpec> {
        IntEvent2IsetDio26W::new(self, 26)
    }
    #[doc = "Bit 27 - DIO27 event"]
    #[inline(always)]
    pub fn int_event2_iset_dio27(&mut self) -> IntEvent2IsetDio27W<IntEvent2IsetSpec> {
        IntEvent2IsetDio27W::new(self, 27)
    }
    #[doc = "Bit 28 - DIO28 event"]
    #[inline(always)]
    pub fn int_event2_iset_dio28(&mut self) -> IntEvent2IsetDio28W<IntEvent2IsetSpec> {
        IntEvent2IsetDio28W::new(self, 28)
    }
    #[doc = "Bit 29 - DIO29 event"]
    #[inline(always)]
    pub fn int_event2_iset_dio29(&mut self) -> IntEvent2IsetDio29W<IntEvent2IsetSpec> {
        IntEvent2IsetDio29W::new(self, 29)
    }
    #[doc = "Bit 30 - DIO30 event"]
    #[inline(always)]
    pub fn int_event2_iset_dio30(&mut self) -> IntEvent2IsetDio30W<IntEvent2IsetSpec> {
        IntEvent2IsetDio30W::new(self, 30)
    }
    #[doc = "Bit 31 - DIO31 event"]
    #[inline(always)]
    pub fn int_event2_iset_dio31(&mut self) -> IntEvent2IsetDio31W<IntEvent2IsetSpec> {
        IntEvent2IsetDio31W::new(self, 31)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event2_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2IsetSpec;
impl crate::RegisterSpec for IntEvent2IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event2_iset::W`](W) writer structure"]
impl crate::Writable for IntEvent2IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT2_ISET to value 0"]
impl crate::Resettable for IntEvent2IsetSpec {
    const RESET_VALUE: u32 = 0;
}
