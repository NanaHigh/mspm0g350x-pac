#[doc = "Register `INT_EVENT0_ISET` writer"]
pub type W = crate::W<IntEvent0IsetSpec>;
#[doc = "DIO0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio0 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio0NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio0Set = 1,
}
impl From<IntEvent0IsetDio0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO0` writer - DIO0 event"]
pub type IntEvent0IsetDio0W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio0>;
impl<'a, REG> IntEvent0IsetDio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio0::IntEvent0IsetDio0NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio0_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio0::IntEvent0IsetDio0Set)
    }
}
#[doc = "DIO1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio1 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio1NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio1Set = 1,
}
impl From<IntEvent0IsetDio1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO1` writer - DIO1 event"]
pub type IntEvent0IsetDio1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio1>;
impl<'a, REG> IntEvent0IsetDio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio1::IntEvent0IsetDio1NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio1::IntEvent0IsetDio1Set)
    }
}
#[doc = "DIO2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio2 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio2NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio2Set = 1,
}
impl From<IntEvent0IsetDio2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO2` writer - DIO2 event"]
pub type IntEvent0IsetDio2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio2>;
impl<'a, REG> IntEvent0IsetDio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio2::IntEvent0IsetDio2NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio2::IntEvent0IsetDio2Set)
    }
}
#[doc = "DIO3 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio3 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio3NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio3Set = 1,
}
impl From<IntEvent0IsetDio3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO3` writer - DIO3 event"]
pub type IntEvent0IsetDio3W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio3>;
impl<'a, REG> IntEvent0IsetDio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio3_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio3::IntEvent0IsetDio3NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio3_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio3::IntEvent0IsetDio3Set)
    }
}
#[doc = "DIO4 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio4 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio4NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio4Set = 1,
}
impl From<IntEvent0IsetDio4> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO4` writer - DIO4 event"]
pub type IntEvent0IsetDio4W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio4>;
impl<'a, REG> IntEvent0IsetDio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio4_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio4::IntEvent0IsetDio4NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio4_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio4::IntEvent0IsetDio4Set)
    }
}
#[doc = "DIO5 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio5 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio5NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio5Set = 1,
}
impl From<IntEvent0IsetDio5> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO5` writer - DIO5 event"]
pub type IntEvent0IsetDio5W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio5>;
impl<'a, REG> IntEvent0IsetDio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio5_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio5::IntEvent0IsetDio5NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio5_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio5::IntEvent0IsetDio5Set)
    }
}
#[doc = "DIO6 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio6 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio6NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio6Set = 1,
}
impl From<IntEvent0IsetDio6> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO6` writer - DIO6 event"]
pub type IntEvent0IsetDio6W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio6>;
impl<'a, REG> IntEvent0IsetDio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio6_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio6::IntEvent0IsetDio6NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio6_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio6::IntEvent0IsetDio6Set)
    }
}
#[doc = "DIO7 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio7 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio7NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio7Set = 1,
}
impl From<IntEvent0IsetDio7> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO7` writer - DIO7 event"]
pub type IntEvent0IsetDio7W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio7>;
impl<'a, REG> IntEvent0IsetDio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio7_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio7::IntEvent0IsetDio7NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio7_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio7::IntEvent0IsetDio7Set)
    }
}
#[doc = "DIO8 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio8 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio8NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio8Set = 1,
}
impl From<IntEvent0IsetDio8> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO8` writer - DIO8 event"]
pub type IntEvent0IsetDio8W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio8>;
impl<'a, REG> IntEvent0IsetDio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio8_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio8::IntEvent0IsetDio8NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio8_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio8::IntEvent0IsetDio8Set)
    }
}
#[doc = "DIO9 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio9 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio9NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio9Set = 1,
}
impl From<IntEvent0IsetDio9> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO9` writer - DIO9 event"]
pub type IntEvent0IsetDio9W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio9>;
impl<'a, REG> IntEvent0IsetDio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio9_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio9::IntEvent0IsetDio9NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio9_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio9::IntEvent0IsetDio9Set)
    }
}
#[doc = "DIO10 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio10 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio10NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio10Set = 1,
}
impl From<IntEvent0IsetDio10> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO10` writer - DIO10 event"]
pub type IntEvent0IsetDio10W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio10>;
impl<'a, REG> IntEvent0IsetDio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio10_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio10::IntEvent0IsetDio10NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio10_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio10::IntEvent0IsetDio10Set)
    }
}
#[doc = "DIO11 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio11 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio11NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio11Set = 1,
}
impl From<IntEvent0IsetDio11> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO11` writer - DIO11 event"]
pub type IntEvent0IsetDio11W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio11>;
impl<'a, REG> IntEvent0IsetDio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio11_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio11::IntEvent0IsetDio11NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio11_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio11::IntEvent0IsetDio11Set)
    }
}
#[doc = "DIO12 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio12 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio12NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio12Set = 1,
}
impl From<IntEvent0IsetDio12> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO12` writer - DIO12 event"]
pub type IntEvent0IsetDio12W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio12>;
impl<'a, REG> IntEvent0IsetDio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio12_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio12::IntEvent0IsetDio12NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio12_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio12::IntEvent0IsetDio12Set)
    }
}
#[doc = "DIO13 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio13 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio13NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio13Set = 1,
}
impl From<IntEvent0IsetDio13> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO13` writer - DIO13 event"]
pub type IntEvent0IsetDio13W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio13>;
impl<'a, REG> IntEvent0IsetDio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio13_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio13::IntEvent0IsetDio13NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio13_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio13::IntEvent0IsetDio13Set)
    }
}
#[doc = "DIO14 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio14 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio14NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio14Set = 1,
}
impl From<IntEvent0IsetDio14> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO14` writer - DIO14 event"]
pub type IntEvent0IsetDio14W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio14>;
impl<'a, REG> IntEvent0IsetDio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio14_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio14::IntEvent0IsetDio14NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio14_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio14::IntEvent0IsetDio14Set)
    }
}
#[doc = "DIO15 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio15 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio15NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio15Set = 1,
}
impl From<IntEvent0IsetDio15> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO15` writer - DIO15 event"]
pub type IntEvent0IsetDio15W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio15>;
impl<'a, REG> IntEvent0IsetDio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio15_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio15::IntEvent0IsetDio15NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio15_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio15::IntEvent0IsetDio15Set)
    }
}
#[doc = "DIO16 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio16 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio16NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio16Set = 1,
}
impl From<IntEvent0IsetDio16> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO16` writer - DIO16 event"]
pub type IntEvent0IsetDio16W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio16>;
impl<'a, REG> IntEvent0IsetDio16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio16_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio16::IntEvent0IsetDio16NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio16_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio16::IntEvent0IsetDio16Set)
    }
}
#[doc = "DIO17 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio17 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio17NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio17Set = 1,
}
impl From<IntEvent0IsetDio17> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO17` writer - DIO17 event"]
pub type IntEvent0IsetDio17W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio17>;
impl<'a, REG> IntEvent0IsetDio17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio17_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio17::IntEvent0IsetDio17NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio17_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio17::IntEvent0IsetDio17Set)
    }
}
#[doc = "DIO18 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio18 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio18NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio18Set = 1,
}
impl From<IntEvent0IsetDio18> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO18` writer - DIO18 event"]
pub type IntEvent0IsetDio18W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio18>;
impl<'a, REG> IntEvent0IsetDio18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio18_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio18::IntEvent0IsetDio18NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio18_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio18::IntEvent0IsetDio18Set)
    }
}
#[doc = "DIO19 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio19 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio19NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio19Set = 1,
}
impl From<IntEvent0IsetDio19> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO19` writer - DIO19 event"]
pub type IntEvent0IsetDio19W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio19>;
impl<'a, REG> IntEvent0IsetDio19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio19_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio19::IntEvent0IsetDio19NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio19_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio19::IntEvent0IsetDio19Set)
    }
}
#[doc = "DIO20 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio20 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio20NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio20Set = 1,
}
impl From<IntEvent0IsetDio20> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO20` writer - DIO20 event"]
pub type IntEvent0IsetDio20W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio20>;
impl<'a, REG> IntEvent0IsetDio20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio20_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio20::IntEvent0IsetDio20NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio20_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio20::IntEvent0IsetDio20Set)
    }
}
#[doc = "DIO21 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio21 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio21NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio21Set = 1,
}
impl From<IntEvent0IsetDio21> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO21` writer - DIO21 event"]
pub type IntEvent0IsetDio21W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio21>;
impl<'a, REG> IntEvent0IsetDio21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio21_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio21::IntEvent0IsetDio21NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio21_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio21::IntEvent0IsetDio21Set)
    }
}
#[doc = "DIO22 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio22 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio22NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio22Set = 1,
}
impl From<IntEvent0IsetDio22> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO22` writer - DIO22 event"]
pub type IntEvent0IsetDio22W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio22>;
impl<'a, REG> IntEvent0IsetDio22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio22_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio22::IntEvent0IsetDio22NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio22_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio22::IntEvent0IsetDio22Set)
    }
}
#[doc = "DIO23 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio23 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio23NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio23Set = 1,
}
impl From<IntEvent0IsetDio23> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO23` writer - DIO23 event"]
pub type IntEvent0IsetDio23W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio23>;
impl<'a, REG> IntEvent0IsetDio23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio23_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio23::IntEvent0IsetDio23NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio23_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio23::IntEvent0IsetDio23Set)
    }
}
#[doc = "DIO24 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio24 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio24NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio24Set = 1,
}
impl From<IntEvent0IsetDio24> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO24` writer - DIO24 event"]
pub type IntEvent0IsetDio24W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio24>;
impl<'a, REG> IntEvent0IsetDio24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio24_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio24::IntEvent0IsetDio24NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio24_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio24::IntEvent0IsetDio24Set)
    }
}
#[doc = "DIO25 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio25 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio25NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio25Set = 1,
}
impl From<IntEvent0IsetDio25> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO25` writer - DIO25 event"]
pub type IntEvent0IsetDio25W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio25>;
impl<'a, REG> IntEvent0IsetDio25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio25_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio25::IntEvent0IsetDio25NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio25_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio25::IntEvent0IsetDio25Set)
    }
}
#[doc = "DIO26 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio26 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio26NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio26Set = 1,
}
impl From<IntEvent0IsetDio26> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO26` writer - DIO26 event"]
pub type IntEvent0IsetDio26W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio26>;
impl<'a, REG> IntEvent0IsetDio26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio26_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio26::IntEvent0IsetDio26NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio26_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio26::IntEvent0IsetDio26Set)
    }
}
#[doc = "DIO27 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio27 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio27NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio27Set = 1,
}
impl From<IntEvent0IsetDio27> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO27` writer - DIO27 event"]
pub type IntEvent0IsetDio27W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio27>;
impl<'a, REG> IntEvent0IsetDio27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio27_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio27::IntEvent0IsetDio27NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio27_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio27::IntEvent0IsetDio27Set)
    }
}
#[doc = "DIO28 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio28 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio28NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio28Set = 1,
}
impl From<IntEvent0IsetDio28> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO28` writer - DIO28 event"]
pub type IntEvent0IsetDio28W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio28>;
impl<'a, REG> IntEvent0IsetDio28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio28_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio28::IntEvent0IsetDio28NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio28_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio28::IntEvent0IsetDio28Set)
    }
}
#[doc = "DIO29 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio29 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio29NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio29Set = 1,
}
impl From<IntEvent0IsetDio29> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO29` writer - DIO29 event"]
pub type IntEvent0IsetDio29W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio29>;
impl<'a, REG> IntEvent0IsetDio29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio29_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio29::IntEvent0IsetDio29NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio29_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio29::IntEvent0IsetDio29Set)
    }
}
#[doc = "DIO30 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio30 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio30NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio30Set = 1,
}
impl From<IntEvent0IsetDio30> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO30` writer - DIO30 event"]
pub type IntEvent0IsetDio30W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio30>;
impl<'a, REG> IntEvent0IsetDio30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio30_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio30::IntEvent0IsetDio30NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio30_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio30::IntEvent0IsetDio30Set)
    }
}
#[doc = "DIO31 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IsetDio31 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IsetDio31NoEffect = 0,
    #[doc = "1: SET"]
    IntEvent0IsetDio31Set = 1,
}
impl From<IntEvent0IsetDio31> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IsetDio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ISET_DIO31` writer - DIO31 event"]
pub type IntEvent0IsetDio31W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IsetDio31>;
impl<'a, REG> IntEvent0IsetDio31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iset_dio31_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio31::IntEvent0IsetDio31NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_iset_dio31_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IsetDio31::IntEvent0IsetDio31Set)
    }
}
impl W {
    #[doc = "Bit 0 - DIO0 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio0(&mut self) -> IntEvent0IsetDio0W<IntEvent0IsetSpec> {
        IntEvent0IsetDio0W::new(self, 0)
    }
    #[doc = "Bit 1 - DIO1 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio1(&mut self) -> IntEvent0IsetDio1W<IntEvent0IsetSpec> {
        IntEvent0IsetDio1W::new(self, 1)
    }
    #[doc = "Bit 2 - DIO2 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio2(&mut self) -> IntEvent0IsetDio2W<IntEvent0IsetSpec> {
        IntEvent0IsetDio2W::new(self, 2)
    }
    #[doc = "Bit 3 - DIO3 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio3(&mut self) -> IntEvent0IsetDio3W<IntEvent0IsetSpec> {
        IntEvent0IsetDio3W::new(self, 3)
    }
    #[doc = "Bit 4 - DIO4 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio4(&mut self) -> IntEvent0IsetDio4W<IntEvent0IsetSpec> {
        IntEvent0IsetDio4W::new(self, 4)
    }
    #[doc = "Bit 5 - DIO5 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio5(&mut self) -> IntEvent0IsetDio5W<IntEvent0IsetSpec> {
        IntEvent0IsetDio5W::new(self, 5)
    }
    #[doc = "Bit 6 - DIO6 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio6(&mut self) -> IntEvent0IsetDio6W<IntEvent0IsetSpec> {
        IntEvent0IsetDio6W::new(self, 6)
    }
    #[doc = "Bit 7 - DIO7 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio7(&mut self) -> IntEvent0IsetDio7W<IntEvent0IsetSpec> {
        IntEvent0IsetDio7W::new(self, 7)
    }
    #[doc = "Bit 8 - DIO8 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio8(&mut self) -> IntEvent0IsetDio8W<IntEvent0IsetSpec> {
        IntEvent0IsetDio8W::new(self, 8)
    }
    #[doc = "Bit 9 - DIO9 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio9(&mut self) -> IntEvent0IsetDio9W<IntEvent0IsetSpec> {
        IntEvent0IsetDio9W::new(self, 9)
    }
    #[doc = "Bit 10 - DIO10 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio10(&mut self) -> IntEvent0IsetDio10W<IntEvent0IsetSpec> {
        IntEvent0IsetDio10W::new(self, 10)
    }
    #[doc = "Bit 11 - DIO11 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio11(&mut self) -> IntEvent0IsetDio11W<IntEvent0IsetSpec> {
        IntEvent0IsetDio11W::new(self, 11)
    }
    #[doc = "Bit 12 - DIO12 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio12(&mut self) -> IntEvent0IsetDio12W<IntEvent0IsetSpec> {
        IntEvent0IsetDio12W::new(self, 12)
    }
    #[doc = "Bit 13 - DIO13 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio13(&mut self) -> IntEvent0IsetDio13W<IntEvent0IsetSpec> {
        IntEvent0IsetDio13W::new(self, 13)
    }
    #[doc = "Bit 14 - DIO14 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio14(&mut self) -> IntEvent0IsetDio14W<IntEvent0IsetSpec> {
        IntEvent0IsetDio14W::new(self, 14)
    }
    #[doc = "Bit 15 - DIO15 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio15(&mut self) -> IntEvent0IsetDio15W<IntEvent0IsetSpec> {
        IntEvent0IsetDio15W::new(self, 15)
    }
    #[doc = "Bit 16 - DIO16 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio16(&mut self) -> IntEvent0IsetDio16W<IntEvent0IsetSpec> {
        IntEvent0IsetDio16W::new(self, 16)
    }
    #[doc = "Bit 17 - DIO17 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio17(&mut self) -> IntEvent0IsetDio17W<IntEvent0IsetSpec> {
        IntEvent0IsetDio17W::new(self, 17)
    }
    #[doc = "Bit 18 - DIO18 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio18(&mut self) -> IntEvent0IsetDio18W<IntEvent0IsetSpec> {
        IntEvent0IsetDio18W::new(self, 18)
    }
    #[doc = "Bit 19 - DIO19 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio19(&mut self) -> IntEvent0IsetDio19W<IntEvent0IsetSpec> {
        IntEvent0IsetDio19W::new(self, 19)
    }
    #[doc = "Bit 20 - DIO20 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio20(&mut self) -> IntEvent0IsetDio20W<IntEvent0IsetSpec> {
        IntEvent0IsetDio20W::new(self, 20)
    }
    #[doc = "Bit 21 - DIO21 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio21(&mut self) -> IntEvent0IsetDio21W<IntEvent0IsetSpec> {
        IntEvent0IsetDio21W::new(self, 21)
    }
    #[doc = "Bit 22 - DIO22 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio22(&mut self) -> IntEvent0IsetDio22W<IntEvent0IsetSpec> {
        IntEvent0IsetDio22W::new(self, 22)
    }
    #[doc = "Bit 23 - DIO23 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio23(&mut self) -> IntEvent0IsetDio23W<IntEvent0IsetSpec> {
        IntEvent0IsetDio23W::new(self, 23)
    }
    #[doc = "Bit 24 - DIO24 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio24(&mut self) -> IntEvent0IsetDio24W<IntEvent0IsetSpec> {
        IntEvent0IsetDio24W::new(self, 24)
    }
    #[doc = "Bit 25 - DIO25 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio25(&mut self) -> IntEvent0IsetDio25W<IntEvent0IsetSpec> {
        IntEvent0IsetDio25W::new(self, 25)
    }
    #[doc = "Bit 26 - DIO26 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio26(&mut self) -> IntEvent0IsetDio26W<IntEvent0IsetSpec> {
        IntEvent0IsetDio26W::new(self, 26)
    }
    #[doc = "Bit 27 - DIO27 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio27(&mut self) -> IntEvent0IsetDio27W<IntEvent0IsetSpec> {
        IntEvent0IsetDio27W::new(self, 27)
    }
    #[doc = "Bit 28 - DIO28 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio28(&mut self) -> IntEvent0IsetDio28W<IntEvent0IsetSpec> {
        IntEvent0IsetDio28W::new(self, 28)
    }
    #[doc = "Bit 29 - DIO29 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio29(&mut self) -> IntEvent0IsetDio29W<IntEvent0IsetSpec> {
        IntEvent0IsetDio29W::new(self, 29)
    }
    #[doc = "Bit 30 - DIO30 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio30(&mut self) -> IntEvent0IsetDio30W<IntEvent0IsetSpec> {
        IntEvent0IsetDio30W::new(self, 30)
    }
    #[doc = "Bit 31 - DIO31 event"]
    #[inline(always)]
    pub fn int_event0_iset_dio31(&mut self) -> IntEvent0IsetDio31W<IntEvent0IsetSpec> {
        IntEvent0IsetDio31W::new(self, 31)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event0_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0IsetSpec;
impl crate::RegisterSpec for IntEvent0IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event0_iset::W`](W) writer structure"]
impl crate::Writable for IntEvent0IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT0_ISET to value 0"]
impl crate::Resettable for IntEvent0IsetSpec {
    const RESET_VALUE: u32 = 0;
}
