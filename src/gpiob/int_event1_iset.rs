#[doc = "Register `INT_EVENT1_ISET` writer"]
pub type W = crate::W<IntEvent1IsetSpec>;
#[doc = "DIO0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetDio0 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetDio0NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetDio0Set = 1,
}
impl From<IntEvent1IsetDio0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetDio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_DIO0` writer - DIO0 event"]
pub type IntEvent1IsetDio0W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetDio0>;
impl<'a, REG> IntEvent1IsetDio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_dio0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio0::IntEvent1IsetDio0NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_dio0_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio0::IntEvent1IsetDio0Set)
    }
}
#[doc = "DIO1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetDio1 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetDio1NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetDio1Set = 1,
}
impl From<IntEvent1IsetDio1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetDio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_DIO1` writer - DIO1 event"]
pub type IntEvent1IsetDio1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetDio1>;
impl<'a, REG> IntEvent1IsetDio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_dio1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio1::IntEvent1IsetDio1NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_dio1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio1::IntEvent1IsetDio1Set)
    }
}
#[doc = "DIO2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetDio2 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetDio2NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetDio2Set = 1,
}
impl From<IntEvent1IsetDio2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetDio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_DIO2` writer - DIO2 event"]
pub type IntEvent1IsetDio2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetDio2>;
impl<'a, REG> IntEvent1IsetDio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_dio2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio2::IntEvent1IsetDio2NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_dio2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio2::IntEvent1IsetDio2Set)
    }
}
#[doc = "DIO3 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetDio3 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetDio3NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetDio3Set = 1,
}
impl From<IntEvent1IsetDio3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetDio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_DIO3` writer - DIO3 event"]
pub type IntEvent1IsetDio3W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetDio3>;
impl<'a, REG> IntEvent1IsetDio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_dio3_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio3::IntEvent1IsetDio3NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_dio3_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio3::IntEvent1IsetDio3Set)
    }
}
#[doc = "DIO4 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetDio4 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetDio4NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetDio4Set = 1,
}
impl From<IntEvent1IsetDio4> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetDio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_DIO4` writer - DIO4 event"]
pub type IntEvent1IsetDio4W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetDio4>;
impl<'a, REG> IntEvent1IsetDio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_dio4_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio4::IntEvent1IsetDio4NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_dio4_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio4::IntEvent1IsetDio4Set)
    }
}
#[doc = "DIO5 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetDio5 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetDio5NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetDio5Set = 1,
}
impl From<IntEvent1IsetDio5> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetDio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_DIO5` writer - DIO5 event"]
pub type IntEvent1IsetDio5W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetDio5>;
impl<'a, REG> IntEvent1IsetDio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_dio5_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio5::IntEvent1IsetDio5NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_dio5_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio5::IntEvent1IsetDio5Set)
    }
}
#[doc = "DIO6 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetDio6 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetDio6NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetDio6Set = 1,
}
impl From<IntEvent1IsetDio6> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetDio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_DIO6` writer - DIO6 event"]
pub type IntEvent1IsetDio6W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetDio6>;
impl<'a, REG> IntEvent1IsetDio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_dio6_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio6::IntEvent1IsetDio6NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_dio6_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio6::IntEvent1IsetDio6Set)
    }
}
#[doc = "DIO7 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetDio7 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetDio7NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetDio7Set = 1,
}
impl From<IntEvent1IsetDio7> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetDio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_DIO7` writer - DIO7 event"]
pub type IntEvent1IsetDio7W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetDio7>;
impl<'a, REG> IntEvent1IsetDio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_dio7_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio7::IntEvent1IsetDio7NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_dio7_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio7::IntEvent1IsetDio7Set)
    }
}
#[doc = "DIO8 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetDio8 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetDio8NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetDio8Set = 1,
}
impl From<IntEvent1IsetDio8> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetDio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_DIO8` writer - DIO8 event"]
pub type IntEvent1IsetDio8W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetDio8>;
impl<'a, REG> IntEvent1IsetDio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_dio8_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio8::IntEvent1IsetDio8NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_dio8_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio8::IntEvent1IsetDio8Set)
    }
}
#[doc = "DIO9 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetDio9 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetDio9NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetDio9Set = 1,
}
impl From<IntEvent1IsetDio9> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetDio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_DIO9` writer - DIO9 event"]
pub type IntEvent1IsetDio9W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetDio9>;
impl<'a, REG> IntEvent1IsetDio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_dio9_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio9::IntEvent1IsetDio9NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_dio9_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio9::IntEvent1IsetDio9Set)
    }
}
#[doc = "DIO10 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetDio10 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetDio10NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetDio10Set = 1,
}
impl From<IntEvent1IsetDio10> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetDio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_DIO10` writer - DIO10 event"]
pub type IntEvent1IsetDio10W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetDio10>;
impl<'a, REG> IntEvent1IsetDio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_dio10_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio10::IntEvent1IsetDio10NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_dio10_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio10::IntEvent1IsetDio10Set)
    }
}
#[doc = "DIO11 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetDio11 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetDio11NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetDio11Set = 1,
}
impl From<IntEvent1IsetDio11> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetDio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_DIO11` writer - DIO11 event"]
pub type IntEvent1IsetDio11W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetDio11>;
impl<'a, REG> IntEvent1IsetDio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_dio11_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio11::IntEvent1IsetDio11NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_dio11_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio11::IntEvent1IsetDio11Set)
    }
}
#[doc = "DIO12 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetDio12 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetDio12NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetDio12Set = 1,
}
impl From<IntEvent1IsetDio12> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetDio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_DIO12` writer - DIO12 event"]
pub type IntEvent1IsetDio12W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetDio12>;
impl<'a, REG> IntEvent1IsetDio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_dio12_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio12::IntEvent1IsetDio12NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_dio12_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio12::IntEvent1IsetDio12Set)
    }
}
#[doc = "DIO13 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetDio13 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetDio13NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetDio13Set = 1,
}
impl From<IntEvent1IsetDio13> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetDio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_DIO13` writer - DIO13 event"]
pub type IntEvent1IsetDio13W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetDio13>;
impl<'a, REG> IntEvent1IsetDio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_dio13_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio13::IntEvent1IsetDio13NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_dio13_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio13::IntEvent1IsetDio13Set)
    }
}
#[doc = "DIO14 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetDio14 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetDio14NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetDio14Set = 1,
}
impl From<IntEvent1IsetDio14> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetDio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_DIO14` writer - DIO14 event"]
pub type IntEvent1IsetDio14W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetDio14>;
impl<'a, REG> IntEvent1IsetDio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_dio14_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio14::IntEvent1IsetDio14NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_dio14_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio14::IntEvent1IsetDio14Set)
    }
}
#[doc = "DIO15 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IsetDio15 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IsetDio15NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent1IsetDio15Set = 1,
}
impl From<IntEvent1IsetDio15> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IsetDio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ISET_DIO15` writer - DIO15 event"]
pub type IntEvent1IsetDio15W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IsetDio15>;
impl<'a, REG> IntEvent1IsetDio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iset_dio15_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio15::IntEvent1IsetDio15NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_iset_dio15_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IsetDio15::IntEvent1IsetDio15Set)
    }
}
impl W {
    #[doc = "Bit 0 - DIO0 event"]
    #[inline(always)]
    pub fn int_event1_iset_dio0(&mut self) -> IntEvent1IsetDio0W<IntEvent1IsetSpec> {
        IntEvent1IsetDio0W::new(self, 0)
    }
    #[doc = "Bit 1 - DIO1 event"]
    #[inline(always)]
    pub fn int_event1_iset_dio1(&mut self) -> IntEvent1IsetDio1W<IntEvent1IsetSpec> {
        IntEvent1IsetDio1W::new(self, 1)
    }
    #[doc = "Bit 2 - DIO2 event"]
    #[inline(always)]
    pub fn int_event1_iset_dio2(&mut self) -> IntEvent1IsetDio2W<IntEvent1IsetSpec> {
        IntEvent1IsetDio2W::new(self, 2)
    }
    #[doc = "Bit 3 - DIO3 event"]
    #[inline(always)]
    pub fn int_event1_iset_dio3(&mut self) -> IntEvent1IsetDio3W<IntEvent1IsetSpec> {
        IntEvent1IsetDio3W::new(self, 3)
    }
    #[doc = "Bit 4 - DIO4 event"]
    #[inline(always)]
    pub fn int_event1_iset_dio4(&mut self) -> IntEvent1IsetDio4W<IntEvent1IsetSpec> {
        IntEvent1IsetDio4W::new(self, 4)
    }
    #[doc = "Bit 5 - DIO5 event"]
    #[inline(always)]
    pub fn int_event1_iset_dio5(&mut self) -> IntEvent1IsetDio5W<IntEvent1IsetSpec> {
        IntEvent1IsetDio5W::new(self, 5)
    }
    #[doc = "Bit 6 - DIO6 event"]
    #[inline(always)]
    pub fn int_event1_iset_dio6(&mut self) -> IntEvent1IsetDio6W<IntEvent1IsetSpec> {
        IntEvent1IsetDio6W::new(self, 6)
    }
    #[doc = "Bit 7 - DIO7 event"]
    #[inline(always)]
    pub fn int_event1_iset_dio7(&mut self) -> IntEvent1IsetDio7W<IntEvent1IsetSpec> {
        IntEvent1IsetDio7W::new(self, 7)
    }
    #[doc = "Bit 8 - DIO8 event"]
    #[inline(always)]
    pub fn int_event1_iset_dio8(&mut self) -> IntEvent1IsetDio8W<IntEvent1IsetSpec> {
        IntEvent1IsetDio8W::new(self, 8)
    }
    #[doc = "Bit 9 - DIO9 event"]
    #[inline(always)]
    pub fn int_event1_iset_dio9(&mut self) -> IntEvent1IsetDio9W<IntEvent1IsetSpec> {
        IntEvent1IsetDio9W::new(self, 9)
    }
    #[doc = "Bit 10 - DIO10 event"]
    #[inline(always)]
    pub fn int_event1_iset_dio10(&mut self) -> IntEvent1IsetDio10W<IntEvent1IsetSpec> {
        IntEvent1IsetDio10W::new(self, 10)
    }
    #[doc = "Bit 11 - DIO11 event"]
    #[inline(always)]
    pub fn int_event1_iset_dio11(&mut self) -> IntEvent1IsetDio11W<IntEvent1IsetSpec> {
        IntEvent1IsetDio11W::new(self, 11)
    }
    #[doc = "Bit 12 - DIO12 event"]
    #[inline(always)]
    pub fn int_event1_iset_dio12(&mut self) -> IntEvent1IsetDio12W<IntEvent1IsetSpec> {
        IntEvent1IsetDio12W::new(self, 12)
    }
    #[doc = "Bit 13 - DIO13 event"]
    #[inline(always)]
    pub fn int_event1_iset_dio13(&mut self) -> IntEvent1IsetDio13W<IntEvent1IsetSpec> {
        IntEvent1IsetDio13W::new(self, 13)
    }
    #[doc = "Bit 14 - DIO14 event"]
    #[inline(always)]
    pub fn int_event1_iset_dio14(&mut self) -> IntEvent1IsetDio14W<IntEvent1IsetSpec> {
        IntEvent1IsetDio14W::new(self, 14)
    }
    #[doc = "Bit 15 - DIO15 event"]
    #[inline(always)]
    pub fn int_event1_iset_dio15(&mut self) -> IntEvent1IsetDio15W<IntEvent1IsetSpec> {
        IntEvent1IsetDio15W::new(self, 15)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event1_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1IsetSpec;
impl crate::RegisterSpec for IntEvent1IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event1_iset::W`](W) writer structure"]
impl crate::Writable for IntEvent1IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT1_ISET to value 0"]
impl crate::Resettable for IntEvent1IsetSpec {
    const RESET_VALUE: u32 = 0;
}
