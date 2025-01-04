#[doc = "Register `INT_EVENT0_ICLR` writer"]
pub type W = crate::W<IntEvent0IclrSpec>;
#[doc = "DIO0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio0 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio0NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio0Clr = 1,
}
impl From<IntEvent0IclrDio0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO0` writer - DIO0 event"]
pub type IntEvent0IclrDio0W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio0>;
impl<'a, REG> IntEvent0IclrDio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio0::IntEvent0IclrDio0NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio0::IntEvent0IclrDio0Clr)
    }
}
#[doc = "DIO1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio1 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio1NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio1Clr = 1,
}
impl From<IntEvent0IclrDio1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO1` writer - DIO1 event"]
pub type IntEvent0IclrDio1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio1>;
impl<'a, REG> IntEvent0IclrDio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio1::IntEvent0IclrDio1NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio1::IntEvent0IclrDio1Clr)
    }
}
#[doc = "DIO2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio2 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio2NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio2Clr = 1,
}
impl From<IntEvent0IclrDio2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO2` writer - DIO2 event"]
pub type IntEvent0IclrDio2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio2>;
impl<'a, REG> IntEvent0IclrDio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio2::IntEvent0IclrDio2NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio2::IntEvent0IclrDio2Clr)
    }
}
#[doc = "DIO3 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio3 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio3NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio3Clr = 1,
}
impl From<IntEvent0IclrDio3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO3` writer - DIO3 event"]
pub type IntEvent0IclrDio3W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio3>;
impl<'a, REG> IntEvent0IclrDio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio3_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio3::IntEvent0IclrDio3NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio3_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio3::IntEvent0IclrDio3Clr)
    }
}
#[doc = "DIO4 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio4 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio4NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio4Clr = 1,
}
impl From<IntEvent0IclrDio4> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO4` writer - DIO4 event"]
pub type IntEvent0IclrDio4W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio4>;
impl<'a, REG> IntEvent0IclrDio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio4_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio4::IntEvent0IclrDio4NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio4_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio4::IntEvent0IclrDio4Clr)
    }
}
#[doc = "DIO5 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio5 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio5NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio5Clr = 1,
}
impl From<IntEvent0IclrDio5> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO5` writer - DIO5 event"]
pub type IntEvent0IclrDio5W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio5>;
impl<'a, REG> IntEvent0IclrDio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio5_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio5::IntEvent0IclrDio5NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio5_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio5::IntEvent0IclrDio5Clr)
    }
}
#[doc = "DIO6 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio6 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio6NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio6Clr = 1,
}
impl From<IntEvent0IclrDio6> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO6` writer - DIO6 event"]
pub type IntEvent0IclrDio6W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio6>;
impl<'a, REG> IntEvent0IclrDio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio6_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio6::IntEvent0IclrDio6NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio6_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio6::IntEvent0IclrDio6Clr)
    }
}
#[doc = "DIO7 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio7 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio7NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio7Clr = 1,
}
impl From<IntEvent0IclrDio7> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO7` writer - DIO7 event"]
pub type IntEvent0IclrDio7W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio7>;
impl<'a, REG> IntEvent0IclrDio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio7_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio7::IntEvent0IclrDio7NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio7_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio7::IntEvent0IclrDio7Clr)
    }
}
#[doc = "DIO8 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio8 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio8NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio8Clr = 1,
}
impl From<IntEvent0IclrDio8> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO8` writer - DIO8 event"]
pub type IntEvent0IclrDio8W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio8>;
impl<'a, REG> IntEvent0IclrDio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio8_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio8::IntEvent0IclrDio8NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio8_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio8::IntEvent0IclrDio8Clr)
    }
}
#[doc = "DIO9 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio9 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio9NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio9Clr = 1,
}
impl From<IntEvent0IclrDio9> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO9` writer - DIO9 event"]
pub type IntEvent0IclrDio9W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio9>;
impl<'a, REG> IntEvent0IclrDio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio9_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio9::IntEvent0IclrDio9NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio9_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio9::IntEvent0IclrDio9Clr)
    }
}
#[doc = "DIO10 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio10 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio10NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio10Clr = 1,
}
impl From<IntEvent0IclrDio10> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO10` writer - DIO10 event"]
pub type IntEvent0IclrDio10W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio10>;
impl<'a, REG> IntEvent0IclrDio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio10_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio10::IntEvent0IclrDio10NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio10_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio10::IntEvent0IclrDio10Clr)
    }
}
#[doc = "DIO11 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio11 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio11NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio11Clr = 1,
}
impl From<IntEvent0IclrDio11> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO11` writer - DIO11 event"]
pub type IntEvent0IclrDio11W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio11>;
impl<'a, REG> IntEvent0IclrDio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio11_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio11::IntEvent0IclrDio11NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio11_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio11::IntEvent0IclrDio11Clr)
    }
}
#[doc = "DIO12 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio12 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio12NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio12Clr = 1,
}
impl From<IntEvent0IclrDio12> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO12` writer - DIO12 event"]
pub type IntEvent0IclrDio12W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio12>;
impl<'a, REG> IntEvent0IclrDio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio12_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio12::IntEvent0IclrDio12NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio12_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio12::IntEvent0IclrDio12Clr)
    }
}
#[doc = "DIO13 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio13 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio13NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio13Clr = 1,
}
impl From<IntEvent0IclrDio13> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO13` writer - DIO13 event"]
pub type IntEvent0IclrDio13W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio13>;
impl<'a, REG> IntEvent0IclrDio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio13_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio13::IntEvent0IclrDio13NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio13_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio13::IntEvent0IclrDio13Clr)
    }
}
#[doc = "DIO14 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio14 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio14NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio14Clr = 1,
}
impl From<IntEvent0IclrDio14> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO14` writer - DIO14 event"]
pub type IntEvent0IclrDio14W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio14>;
impl<'a, REG> IntEvent0IclrDio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio14_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio14::IntEvent0IclrDio14NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio14_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio14::IntEvent0IclrDio14Clr)
    }
}
#[doc = "DIO15 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio15 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio15NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio15Clr = 1,
}
impl From<IntEvent0IclrDio15> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO15` writer - DIO15 event"]
pub type IntEvent0IclrDio15W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio15>;
impl<'a, REG> IntEvent0IclrDio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio15_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio15::IntEvent0IclrDio15NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio15_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio15::IntEvent0IclrDio15Clr)
    }
}
#[doc = "DIO16 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio16 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio16NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio16Clr = 1,
}
impl From<IntEvent0IclrDio16> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO16` writer - DIO16 event"]
pub type IntEvent0IclrDio16W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio16>;
impl<'a, REG> IntEvent0IclrDio16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio16_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio16::IntEvent0IclrDio16NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio16_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio16::IntEvent0IclrDio16Clr)
    }
}
#[doc = "DIO17 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio17 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio17NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio17Clr = 1,
}
impl From<IntEvent0IclrDio17> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO17` writer - DIO17 event"]
pub type IntEvent0IclrDio17W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio17>;
impl<'a, REG> IntEvent0IclrDio17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio17_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio17::IntEvent0IclrDio17NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio17_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio17::IntEvent0IclrDio17Clr)
    }
}
#[doc = "DIO18 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio18 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio18NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio18Clr = 1,
}
impl From<IntEvent0IclrDio18> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO18` writer - DIO18 event"]
pub type IntEvent0IclrDio18W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio18>;
impl<'a, REG> IntEvent0IclrDio18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio18_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio18::IntEvent0IclrDio18NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio18_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio18::IntEvent0IclrDio18Clr)
    }
}
#[doc = "DIO19 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio19 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio19NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio19Clr = 1,
}
impl From<IntEvent0IclrDio19> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO19` writer - DIO19 event"]
pub type IntEvent0IclrDio19W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio19>;
impl<'a, REG> IntEvent0IclrDio19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio19_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio19::IntEvent0IclrDio19NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio19_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio19::IntEvent0IclrDio19Clr)
    }
}
#[doc = "DIO20 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio20 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio20NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio20Clr = 1,
}
impl From<IntEvent0IclrDio20> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO20` writer - DIO20 event"]
pub type IntEvent0IclrDio20W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio20>;
impl<'a, REG> IntEvent0IclrDio20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio20_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio20::IntEvent0IclrDio20NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio20_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio20::IntEvent0IclrDio20Clr)
    }
}
#[doc = "DIO21 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio21 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio21NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio21Clr = 1,
}
impl From<IntEvent0IclrDio21> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO21` writer - DIO21 event"]
pub type IntEvent0IclrDio21W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio21>;
impl<'a, REG> IntEvent0IclrDio21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio21_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio21::IntEvent0IclrDio21NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio21_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio21::IntEvent0IclrDio21Clr)
    }
}
#[doc = "DIO22 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio22 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio22NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio22Clr = 1,
}
impl From<IntEvent0IclrDio22> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO22` writer - DIO22 event"]
pub type IntEvent0IclrDio22W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio22>;
impl<'a, REG> IntEvent0IclrDio22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio22_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio22::IntEvent0IclrDio22NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio22_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio22::IntEvent0IclrDio22Clr)
    }
}
#[doc = "DIO23 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio23 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio23NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio23Clr = 1,
}
impl From<IntEvent0IclrDio23> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO23` writer - DIO23 event"]
pub type IntEvent0IclrDio23W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio23>;
impl<'a, REG> IntEvent0IclrDio23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio23_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio23::IntEvent0IclrDio23NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio23_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio23::IntEvent0IclrDio23Clr)
    }
}
#[doc = "DIO24 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio24 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio24NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio24Clr = 1,
}
impl From<IntEvent0IclrDio24> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO24` writer - DIO24 event"]
pub type IntEvent0IclrDio24W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio24>;
impl<'a, REG> IntEvent0IclrDio24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio24_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio24::IntEvent0IclrDio24NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio24_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio24::IntEvent0IclrDio24Clr)
    }
}
#[doc = "DIO25 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio25 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio25NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio25Clr = 1,
}
impl From<IntEvent0IclrDio25> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO25` writer - DIO25 event"]
pub type IntEvent0IclrDio25W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio25>;
impl<'a, REG> IntEvent0IclrDio25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio25_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio25::IntEvent0IclrDio25NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio25_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio25::IntEvent0IclrDio25Clr)
    }
}
#[doc = "DIO26 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio26 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio26NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio26Clr = 1,
}
impl From<IntEvent0IclrDio26> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO26` writer - DIO26 event"]
pub type IntEvent0IclrDio26W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio26>;
impl<'a, REG> IntEvent0IclrDio26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio26_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio26::IntEvent0IclrDio26NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio26_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio26::IntEvent0IclrDio26Clr)
    }
}
#[doc = "DIO27 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio27 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio27NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio27Clr = 1,
}
impl From<IntEvent0IclrDio27> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO27` writer - DIO27 event"]
pub type IntEvent0IclrDio27W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio27>;
impl<'a, REG> IntEvent0IclrDio27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio27_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio27::IntEvent0IclrDio27NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio27_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio27::IntEvent0IclrDio27Clr)
    }
}
#[doc = "DIO28 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio28 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio28NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio28Clr = 1,
}
impl From<IntEvent0IclrDio28> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO28` writer - DIO28 event"]
pub type IntEvent0IclrDio28W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio28>;
impl<'a, REG> IntEvent0IclrDio28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio28_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio28::IntEvent0IclrDio28NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio28_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio28::IntEvent0IclrDio28Clr)
    }
}
#[doc = "DIO29 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio29 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio29NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio29Clr = 1,
}
impl From<IntEvent0IclrDio29> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO29` writer - DIO29 event"]
pub type IntEvent0IclrDio29W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio29>;
impl<'a, REG> IntEvent0IclrDio29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio29_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio29::IntEvent0IclrDio29NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio29_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio29::IntEvent0IclrDio29Clr)
    }
}
#[doc = "DIO30 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio30 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio30NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio30Clr = 1,
}
impl From<IntEvent0IclrDio30> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO30` writer - DIO30 event"]
pub type IntEvent0IclrDio30W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio30>;
impl<'a, REG> IntEvent0IclrDio30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio30_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio30::IntEvent0IclrDio30NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio30_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio30::IntEvent0IclrDio30Clr)
    }
}
#[doc = "DIO31 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0IclrDio31 {
    #[doc = "0: NO_EFFECT"]
    IntEvent0IclrDio31NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent0IclrDio31Clr = 1,
}
impl From<IntEvent0IclrDio31> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0IclrDio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_ICLR_DIO31` writer - DIO31 event"]
pub type IntEvent0IclrDio31W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0IclrDio31>;
impl<'a, REG> IntEvent0IclrDio31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event0_iclr_dio31_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio31::IntEvent0IclrDio31NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_iclr_dio31_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0IclrDio31::IntEvent0IclrDio31Clr)
    }
}
impl W {
    #[doc = "Bit 0 - DIO0 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio0(&mut self) -> IntEvent0IclrDio0W<IntEvent0IclrSpec> {
        IntEvent0IclrDio0W::new(self, 0)
    }
    #[doc = "Bit 1 - DIO1 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio1(&mut self) -> IntEvent0IclrDio1W<IntEvent0IclrSpec> {
        IntEvent0IclrDio1W::new(self, 1)
    }
    #[doc = "Bit 2 - DIO2 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio2(&mut self) -> IntEvent0IclrDio2W<IntEvent0IclrSpec> {
        IntEvent0IclrDio2W::new(self, 2)
    }
    #[doc = "Bit 3 - DIO3 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio3(&mut self) -> IntEvent0IclrDio3W<IntEvent0IclrSpec> {
        IntEvent0IclrDio3W::new(self, 3)
    }
    #[doc = "Bit 4 - DIO4 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio4(&mut self) -> IntEvent0IclrDio4W<IntEvent0IclrSpec> {
        IntEvent0IclrDio4W::new(self, 4)
    }
    #[doc = "Bit 5 - DIO5 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio5(&mut self) -> IntEvent0IclrDio5W<IntEvent0IclrSpec> {
        IntEvent0IclrDio5W::new(self, 5)
    }
    #[doc = "Bit 6 - DIO6 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio6(&mut self) -> IntEvent0IclrDio6W<IntEvent0IclrSpec> {
        IntEvent0IclrDio6W::new(self, 6)
    }
    #[doc = "Bit 7 - DIO7 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio7(&mut self) -> IntEvent0IclrDio7W<IntEvent0IclrSpec> {
        IntEvent0IclrDio7W::new(self, 7)
    }
    #[doc = "Bit 8 - DIO8 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio8(&mut self) -> IntEvent0IclrDio8W<IntEvent0IclrSpec> {
        IntEvent0IclrDio8W::new(self, 8)
    }
    #[doc = "Bit 9 - DIO9 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio9(&mut self) -> IntEvent0IclrDio9W<IntEvent0IclrSpec> {
        IntEvent0IclrDio9W::new(self, 9)
    }
    #[doc = "Bit 10 - DIO10 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio10(&mut self) -> IntEvent0IclrDio10W<IntEvent0IclrSpec> {
        IntEvent0IclrDio10W::new(self, 10)
    }
    #[doc = "Bit 11 - DIO11 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio11(&mut self) -> IntEvent0IclrDio11W<IntEvent0IclrSpec> {
        IntEvent0IclrDio11W::new(self, 11)
    }
    #[doc = "Bit 12 - DIO12 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio12(&mut self) -> IntEvent0IclrDio12W<IntEvent0IclrSpec> {
        IntEvent0IclrDio12W::new(self, 12)
    }
    #[doc = "Bit 13 - DIO13 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio13(&mut self) -> IntEvent0IclrDio13W<IntEvent0IclrSpec> {
        IntEvent0IclrDio13W::new(self, 13)
    }
    #[doc = "Bit 14 - DIO14 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio14(&mut self) -> IntEvent0IclrDio14W<IntEvent0IclrSpec> {
        IntEvent0IclrDio14W::new(self, 14)
    }
    #[doc = "Bit 15 - DIO15 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio15(&mut self) -> IntEvent0IclrDio15W<IntEvent0IclrSpec> {
        IntEvent0IclrDio15W::new(self, 15)
    }
    #[doc = "Bit 16 - DIO16 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio16(&mut self) -> IntEvent0IclrDio16W<IntEvent0IclrSpec> {
        IntEvent0IclrDio16W::new(self, 16)
    }
    #[doc = "Bit 17 - DIO17 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio17(&mut self) -> IntEvent0IclrDio17W<IntEvent0IclrSpec> {
        IntEvent0IclrDio17W::new(self, 17)
    }
    #[doc = "Bit 18 - DIO18 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio18(&mut self) -> IntEvent0IclrDio18W<IntEvent0IclrSpec> {
        IntEvent0IclrDio18W::new(self, 18)
    }
    #[doc = "Bit 19 - DIO19 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio19(&mut self) -> IntEvent0IclrDio19W<IntEvent0IclrSpec> {
        IntEvent0IclrDio19W::new(self, 19)
    }
    #[doc = "Bit 20 - DIO20 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio20(&mut self) -> IntEvent0IclrDio20W<IntEvent0IclrSpec> {
        IntEvent0IclrDio20W::new(self, 20)
    }
    #[doc = "Bit 21 - DIO21 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio21(&mut self) -> IntEvent0IclrDio21W<IntEvent0IclrSpec> {
        IntEvent0IclrDio21W::new(self, 21)
    }
    #[doc = "Bit 22 - DIO22 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio22(&mut self) -> IntEvent0IclrDio22W<IntEvent0IclrSpec> {
        IntEvent0IclrDio22W::new(self, 22)
    }
    #[doc = "Bit 23 - DIO23 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio23(&mut self) -> IntEvent0IclrDio23W<IntEvent0IclrSpec> {
        IntEvent0IclrDio23W::new(self, 23)
    }
    #[doc = "Bit 24 - DIO24 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio24(&mut self) -> IntEvent0IclrDio24W<IntEvent0IclrSpec> {
        IntEvent0IclrDio24W::new(self, 24)
    }
    #[doc = "Bit 25 - DIO25 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio25(&mut self) -> IntEvent0IclrDio25W<IntEvent0IclrSpec> {
        IntEvent0IclrDio25W::new(self, 25)
    }
    #[doc = "Bit 26 - DIO26 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio26(&mut self) -> IntEvent0IclrDio26W<IntEvent0IclrSpec> {
        IntEvent0IclrDio26W::new(self, 26)
    }
    #[doc = "Bit 27 - DIO27 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio27(&mut self) -> IntEvent0IclrDio27W<IntEvent0IclrSpec> {
        IntEvent0IclrDio27W::new(self, 27)
    }
    #[doc = "Bit 28 - DIO28 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio28(&mut self) -> IntEvent0IclrDio28W<IntEvent0IclrSpec> {
        IntEvent0IclrDio28W::new(self, 28)
    }
    #[doc = "Bit 29 - DIO29 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio29(&mut self) -> IntEvent0IclrDio29W<IntEvent0IclrSpec> {
        IntEvent0IclrDio29W::new(self, 29)
    }
    #[doc = "Bit 30 - DIO30 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio30(&mut self) -> IntEvent0IclrDio30W<IntEvent0IclrSpec> {
        IntEvent0IclrDio30W::new(self, 30)
    }
    #[doc = "Bit 31 - DIO31 event"]
    #[inline(always)]
    pub fn int_event0_iclr_dio31(&mut self) -> IntEvent0IclrDio31W<IntEvent0IclrSpec> {
        IntEvent0IclrDio31W::new(self, 31)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event0_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0IclrSpec;
impl crate::RegisterSpec for IntEvent0IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event0_iclr::W`](W) writer structure"]
impl crate::Writable for IntEvent0IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT0_ICLR to value 0"]
impl crate::Resettable for IntEvent0IclrSpec {
    const RESET_VALUE: u32 = 0;
}
