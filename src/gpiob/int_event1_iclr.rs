#[doc = "Register `INT_EVENT1_ICLR` writer"]
pub type W = crate::W<IntEvent1IclrSpec>;
#[doc = "DIO0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrDio0 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrDio0NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrDio0Clr = 1,
}
impl From<IntEvent1IclrDio0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrDio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_DIO0` writer - DIO0 event"]
pub type IntEvent1IclrDio0W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrDio0>;
impl<'a, REG> IntEvent1IclrDio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_dio0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio0::IntEvent1IclrDio0NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_dio0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio0::IntEvent1IclrDio0Clr)
    }
}
#[doc = "DIO1 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrDio1 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrDio1NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrDio1Clr = 1,
}
impl From<IntEvent1IclrDio1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrDio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_DIO1` writer - DIO1 event"]
pub type IntEvent1IclrDio1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrDio1>;
impl<'a, REG> IntEvent1IclrDio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_dio1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio1::IntEvent1IclrDio1NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_dio1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio1::IntEvent1IclrDio1Clr)
    }
}
#[doc = "DIO2 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrDio2 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrDio2NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrDio2Clr = 1,
}
impl From<IntEvent1IclrDio2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrDio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_DIO2` writer - DIO2 event"]
pub type IntEvent1IclrDio2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrDio2>;
impl<'a, REG> IntEvent1IclrDio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_dio2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio2::IntEvent1IclrDio2NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_dio2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio2::IntEvent1IclrDio2Clr)
    }
}
#[doc = "DIO3 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrDio3 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrDio3NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrDio3Clr = 1,
}
impl From<IntEvent1IclrDio3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrDio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_DIO3` writer - DIO3 event"]
pub type IntEvent1IclrDio3W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrDio3>;
impl<'a, REG> IntEvent1IclrDio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_dio3_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio3::IntEvent1IclrDio3NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_dio3_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio3::IntEvent1IclrDio3Clr)
    }
}
#[doc = "DIO4 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrDio4 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrDio4NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrDio4Clr = 1,
}
impl From<IntEvent1IclrDio4> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrDio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_DIO4` writer - DIO4 event"]
pub type IntEvent1IclrDio4W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrDio4>;
impl<'a, REG> IntEvent1IclrDio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_dio4_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio4::IntEvent1IclrDio4NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_dio4_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio4::IntEvent1IclrDio4Clr)
    }
}
#[doc = "DIO5 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrDio5 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrDio5NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrDio5Clr = 1,
}
impl From<IntEvent1IclrDio5> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrDio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_DIO5` writer - DIO5 event"]
pub type IntEvent1IclrDio5W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrDio5>;
impl<'a, REG> IntEvent1IclrDio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_dio5_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio5::IntEvent1IclrDio5NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_dio5_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio5::IntEvent1IclrDio5Clr)
    }
}
#[doc = "DIO6 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrDio6 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrDio6NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrDio6Clr = 1,
}
impl From<IntEvent1IclrDio6> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrDio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_DIO6` writer - DIO6 event"]
pub type IntEvent1IclrDio6W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrDio6>;
impl<'a, REG> IntEvent1IclrDio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_dio6_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio6::IntEvent1IclrDio6NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_dio6_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio6::IntEvent1IclrDio6Clr)
    }
}
#[doc = "DIO7 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrDio7 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrDio7NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrDio7Clr = 1,
}
impl From<IntEvent1IclrDio7> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrDio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_DIO7` writer - DIO7 event"]
pub type IntEvent1IclrDio7W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrDio7>;
impl<'a, REG> IntEvent1IclrDio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_dio7_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio7::IntEvent1IclrDio7NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_dio7_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio7::IntEvent1IclrDio7Clr)
    }
}
#[doc = "DIO8 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrDio8 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrDio8NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrDio8Clr = 1,
}
impl From<IntEvent1IclrDio8> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrDio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_DIO8` writer - DIO8 event"]
pub type IntEvent1IclrDio8W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrDio8>;
impl<'a, REG> IntEvent1IclrDio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_dio8_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio8::IntEvent1IclrDio8NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_dio8_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio8::IntEvent1IclrDio8Clr)
    }
}
#[doc = "DIO9 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrDio9 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrDio9NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrDio9Clr = 1,
}
impl From<IntEvent1IclrDio9> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrDio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_DIO9` writer - DIO9 event"]
pub type IntEvent1IclrDio9W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrDio9>;
impl<'a, REG> IntEvent1IclrDio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_dio9_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio9::IntEvent1IclrDio9NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_dio9_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio9::IntEvent1IclrDio9Clr)
    }
}
#[doc = "DIO10 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrDio10 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrDio10NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrDio10Clr = 1,
}
impl From<IntEvent1IclrDio10> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrDio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_DIO10` writer - DIO10 event"]
pub type IntEvent1IclrDio10W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrDio10>;
impl<'a, REG> IntEvent1IclrDio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_dio10_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio10::IntEvent1IclrDio10NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_dio10_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio10::IntEvent1IclrDio10Clr)
    }
}
#[doc = "DIO11 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrDio11 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrDio11NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrDio11Clr = 1,
}
impl From<IntEvent1IclrDio11> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrDio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_DIO11` writer - DIO11 event"]
pub type IntEvent1IclrDio11W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrDio11>;
impl<'a, REG> IntEvent1IclrDio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_dio11_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio11::IntEvent1IclrDio11NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_dio11_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio11::IntEvent1IclrDio11Clr)
    }
}
#[doc = "DIO12 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrDio12 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrDio12NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrDio12Clr = 1,
}
impl From<IntEvent1IclrDio12> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrDio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_DIO12` writer - DIO12 event"]
pub type IntEvent1IclrDio12W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrDio12>;
impl<'a, REG> IntEvent1IclrDio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_dio12_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio12::IntEvent1IclrDio12NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_dio12_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio12::IntEvent1IclrDio12Clr)
    }
}
#[doc = "DIO13 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrDio13 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrDio13NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrDio13Clr = 1,
}
impl From<IntEvent1IclrDio13> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrDio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_DIO13` writer - DIO13 event"]
pub type IntEvent1IclrDio13W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrDio13>;
impl<'a, REG> IntEvent1IclrDio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_dio13_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio13::IntEvent1IclrDio13NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_dio13_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio13::IntEvent1IclrDio13Clr)
    }
}
#[doc = "DIO14 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrDio14 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrDio14NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrDio14Clr = 1,
}
impl From<IntEvent1IclrDio14> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrDio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_DIO14` writer - DIO14 event"]
pub type IntEvent1IclrDio14W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrDio14>;
impl<'a, REG> IntEvent1IclrDio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_dio14_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio14::IntEvent1IclrDio14NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_dio14_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio14::IntEvent1IclrDio14Clr)
    }
}
#[doc = "DIO15 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1IclrDio15 {
    #[doc = "0: NO_EFFECT"]
    IntEvent1IclrDio15NoEffect = 0,
    #[doc = "1: CLR"]
    IntEvent1IclrDio15Clr = 1,
}
impl From<IntEvent1IclrDio15> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1IclrDio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_ICLR_DIO15` writer - DIO15 event"]
pub type IntEvent1IclrDio15W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1IclrDio15>;
impl<'a, REG> IntEvent1IclrDio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_event1_iclr_dio15_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio15::IntEvent1IclrDio15NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_iclr_dio15_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1IclrDio15::IntEvent1IclrDio15Clr)
    }
}
impl W {
    #[doc = "Bit 0 - DIO0 event"]
    #[inline(always)]
    pub fn int_event1_iclr_dio0(&mut self) -> IntEvent1IclrDio0W<IntEvent1IclrSpec> {
        IntEvent1IclrDio0W::new(self, 0)
    }
    #[doc = "Bit 1 - DIO1 event"]
    #[inline(always)]
    pub fn int_event1_iclr_dio1(&mut self) -> IntEvent1IclrDio1W<IntEvent1IclrSpec> {
        IntEvent1IclrDio1W::new(self, 1)
    }
    #[doc = "Bit 2 - DIO2 event"]
    #[inline(always)]
    pub fn int_event1_iclr_dio2(&mut self) -> IntEvent1IclrDio2W<IntEvent1IclrSpec> {
        IntEvent1IclrDio2W::new(self, 2)
    }
    #[doc = "Bit 3 - DIO3 event"]
    #[inline(always)]
    pub fn int_event1_iclr_dio3(&mut self) -> IntEvent1IclrDio3W<IntEvent1IclrSpec> {
        IntEvent1IclrDio3W::new(self, 3)
    }
    #[doc = "Bit 4 - DIO4 event"]
    #[inline(always)]
    pub fn int_event1_iclr_dio4(&mut self) -> IntEvent1IclrDio4W<IntEvent1IclrSpec> {
        IntEvent1IclrDio4W::new(self, 4)
    }
    #[doc = "Bit 5 - DIO5 event"]
    #[inline(always)]
    pub fn int_event1_iclr_dio5(&mut self) -> IntEvent1IclrDio5W<IntEvent1IclrSpec> {
        IntEvent1IclrDio5W::new(self, 5)
    }
    #[doc = "Bit 6 - DIO6 event"]
    #[inline(always)]
    pub fn int_event1_iclr_dio6(&mut self) -> IntEvent1IclrDio6W<IntEvent1IclrSpec> {
        IntEvent1IclrDio6W::new(self, 6)
    }
    #[doc = "Bit 7 - DIO7 event"]
    #[inline(always)]
    pub fn int_event1_iclr_dio7(&mut self) -> IntEvent1IclrDio7W<IntEvent1IclrSpec> {
        IntEvent1IclrDio7W::new(self, 7)
    }
    #[doc = "Bit 8 - DIO8 event"]
    #[inline(always)]
    pub fn int_event1_iclr_dio8(&mut self) -> IntEvent1IclrDio8W<IntEvent1IclrSpec> {
        IntEvent1IclrDio8W::new(self, 8)
    }
    #[doc = "Bit 9 - DIO9 event"]
    #[inline(always)]
    pub fn int_event1_iclr_dio9(&mut self) -> IntEvent1IclrDio9W<IntEvent1IclrSpec> {
        IntEvent1IclrDio9W::new(self, 9)
    }
    #[doc = "Bit 10 - DIO10 event"]
    #[inline(always)]
    pub fn int_event1_iclr_dio10(&mut self) -> IntEvent1IclrDio10W<IntEvent1IclrSpec> {
        IntEvent1IclrDio10W::new(self, 10)
    }
    #[doc = "Bit 11 - DIO11 event"]
    #[inline(always)]
    pub fn int_event1_iclr_dio11(&mut self) -> IntEvent1IclrDio11W<IntEvent1IclrSpec> {
        IntEvent1IclrDio11W::new(self, 11)
    }
    #[doc = "Bit 12 - DIO12 event"]
    #[inline(always)]
    pub fn int_event1_iclr_dio12(&mut self) -> IntEvent1IclrDio12W<IntEvent1IclrSpec> {
        IntEvent1IclrDio12W::new(self, 12)
    }
    #[doc = "Bit 13 - DIO13 event"]
    #[inline(always)]
    pub fn int_event1_iclr_dio13(&mut self) -> IntEvent1IclrDio13W<IntEvent1IclrSpec> {
        IntEvent1IclrDio13W::new(self, 13)
    }
    #[doc = "Bit 14 - DIO14 event"]
    #[inline(always)]
    pub fn int_event1_iclr_dio14(&mut self) -> IntEvent1IclrDio14W<IntEvent1IclrSpec> {
        IntEvent1IclrDio14W::new(self, 14)
    }
    #[doc = "Bit 15 - DIO15 event"]
    #[inline(always)]
    pub fn int_event1_iclr_dio15(&mut self) -> IntEvent1IclrDio15W<IntEvent1IclrSpec> {
        IntEvent1IclrDio15W::new(self, 15)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event1_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1IclrSpec;
impl crate::RegisterSpec for IntEvent1IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_event1_iclr::W`](W) writer structure"]
impl crate::Writable for IntEvent1IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT1_ICLR to value 0"]
impl crate::Resettable for IntEvent1IclrSpec {
    const RESET_VALUE: u32 = 0;
}
