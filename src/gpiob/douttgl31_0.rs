#[doc = "Register `DOUTTGL31_0` writer"]
pub type W = crate::W<Douttgl31_0Spec>;
#[doc = "This bit is used to toggle DIO0 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio0 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio0NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio0Toggle = 1,
}
impl From<Douttgl31_0Dio0> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO0` writer - This bit is used to toggle DIO0 output."]
pub type Douttgl31_0Dio0W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio0>;
impl<'a, REG> Douttgl31_0Dio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio0::Douttgl31_0Dio0NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio0_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio0::Douttgl31_0Dio0Toggle)
    }
}
#[doc = "This bit is used to toggle DIO1 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio1 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio1NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio1Toggle = 1,
}
impl From<Douttgl31_0Dio1> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO1` writer - This bit is used to toggle DIO1 output."]
pub type Douttgl31_0Dio1W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio1>;
impl<'a, REG> Douttgl31_0Dio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio1::Douttgl31_0Dio1NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio1_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio1::Douttgl31_0Dio1Toggle)
    }
}
#[doc = "This bit is used to toggle DIO2 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio2 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio2NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio2Toggle = 1,
}
impl From<Douttgl31_0Dio2> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO2` writer - This bit is used to toggle DIO2 output."]
pub type Douttgl31_0Dio2W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio2>;
impl<'a, REG> Douttgl31_0Dio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio2::Douttgl31_0Dio2NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio2_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio2::Douttgl31_0Dio2Toggle)
    }
}
#[doc = "This bit is used to toggle DIO3 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio3 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio3NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio3Toggle = 1,
}
impl From<Douttgl31_0Dio3> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO3` writer - This bit is used to toggle DIO3 output."]
pub type Douttgl31_0Dio3W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio3>;
impl<'a, REG> Douttgl31_0Dio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio3_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio3::Douttgl31_0Dio3NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio3_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio3::Douttgl31_0Dio3Toggle)
    }
}
#[doc = "This bit is used to toggle DIO4 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio4 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio4NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio4Toggle = 1,
}
impl From<Douttgl31_0Dio4> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO4` writer - This bit is used to toggle DIO4 output."]
pub type Douttgl31_0Dio4W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio4>;
impl<'a, REG> Douttgl31_0Dio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio4_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio4::Douttgl31_0Dio4NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio4_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio4::Douttgl31_0Dio4Toggle)
    }
}
#[doc = "This bit is used to toggle DIO5 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio5 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio5NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio5Toggle = 1,
}
impl From<Douttgl31_0Dio5> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO5` writer - This bit is used to toggle DIO5 output."]
pub type Douttgl31_0Dio5W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio5>;
impl<'a, REG> Douttgl31_0Dio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio5_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio5::Douttgl31_0Dio5NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio5_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio5::Douttgl31_0Dio5Toggle)
    }
}
#[doc = "This bit is used to toggle DIO6 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio6 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio6NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio6Toggle = 1,
}
impl From<Douttgl31_0Dio6> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO6` writer - This bit is used to toggle DIO6 output."]
pub type Douttgl31_0Dio6W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio6>;
impl<'a, REG> Douttgl31_0Dio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio6_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio6::Douttgl31_0Dio6NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio6_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio6::Douttgl31_0Dio6Toggle)
    }
}
#[doc = "This bit is used to toggle DIO7 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio7 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio7NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio7Toggle = 1,
}
impl From<Douttgl31_0Dio7> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO7` writer - This bit is used to toggle DIO7 output."]
pub type Douttgl31_0Dio7W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio7>;
impl<'a, REG> Douttgl31_0Dio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio7_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio7::Douttgl31_0Dio7NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio7_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio7::Douttgl31_0Dio7Toggle)
    }
}
#[doc = "This bit is used to toggle DIO8 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio8 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio8NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio8Toggle = 1,
}
impl From<Douttgl31_0Dio8> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO8` writer - This bit is used to toggle DIO8 output."]
pub type Douttgl31_0Dio8W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio8>;
impl<'a, REG> Douttgl31_0Dio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio8_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio8::Douttgl31_0Dio8NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio8_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio8::Douttgl31_0Dio8Toggle)
    }
}
#[doc = "This bit is used to toggle DIO9 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio9 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio9NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio9Toggle = 1,
}
impl From<Douttgl31_0Dio9> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO9` writer - This bit is used to toggle DIO9 output."]
pub type Douttgl31_0Dio9W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio9>;
impl<'a, REG> Douttgl31_0Dio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio9_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio9::Douttgl31_0Dio9NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio9_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio9::Douttgl31_0Dio9Toggle)
    }
}
#[doc = "This bit is used to toggle DIO10 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio10 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio10NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio10Toggle = 1,
}
impl From<Douttgl31_0Dio10> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO10` writer - This bit is used to toggle DIO10 output."]
pub type Douttgl31_0Dio10W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio10>;
impl<'a, REG> Douttgl31_0Dio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio10_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio10::Douttgl31_0Dio10NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio10_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio10::Douttgl31_0Dio10Toggle)
    }
}
#[doc = "This bit is used to toggle DIO11 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio11 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio11NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio11Toggle = 1,
}
impl From<Douttgl31_0Dio11> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO11` writer - This bit is used to toggle DIO11 output."]
pub type Douttgl31_0Dio11W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio11>;
impl<'a, REG> Douttgl31_0Dio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio11_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio11::Douttgl31_0Dio11NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio11_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio11::Douttgl31_0Dio11Toggle)
    }
}
#[doc = "This bit is used to toggle DIO12 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio12 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio12NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio12Toggle = 1,
}
impl From<Douttgl31_0Dio12> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO12` writer - This bit is used to toggle DIO12 output."]
pub type Douttgl31_0Dio12W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio12>;
impl<'a, REG> Douttgl31_0Dio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio12_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio12::Douttgl31_0Dio12NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio12_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio12::Douttgl31_0Dio12Toggle)
    }
}
#[doc = "This bit is used to toggle DIO13 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio13 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio13NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio13Toggle = 1,
}
impl From<Douttgl31_0Dio13> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO13` writer - This bit is used to toggle DIO13 output."]
pub type Douttgl31_0Dio13W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio13>;
impl<'a, REG> Douttgl31_0Dio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio13_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio13::Douttgl31_0Dio13NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio13_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio13::Douttgl31_0Dio13Toggle)
    }
}
#[doc = "This bit is used to toggle DIO14 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio14 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio14NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio14Toggle = 1,
}
impl From<Douttgl31_0Dio14> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO14` writer - This bit is used to toggle DIO14 output."]
pub type Douttgl31_0Dio14W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio14>;
impl<'a, REG> Douttgl31_0Dio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio14_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio14::Douttgl31_0Dio14NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio14_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio14::Douttgl31_0Dio14Toggle)
    }
}
#[doc = "This bit is used to toggle DIO15 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio15 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio15NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio15Toggle = 1,
}
impl From<Douttgl31_0Dio15> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO15` writer - This bit is used to toggle DIO15 output."]
pub type Douttgl31_0Dio15W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio15>;
impl<'a, REG> Douttgl31_0Dio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio15_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio15::Douttgl31_0Dio15NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio15_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio15::Douttgl31_0Dio15Toggle)
    }
}
#[doc = "This bit is used to toggle DIO16 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio16 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio16NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio16Toggle = 1,
}
impl From<Douttgl31_0Dio16> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO16` writer - This bit is used to toggle DIO16 output."]
pub type Douttgl31_0Dio16W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio16>;
impl<'a, REG> Douttgl31_0Dio16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio16_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio16::Douttgl31_0Dio16NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio16_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio16::Douttgl31_0Dio16Toggle)
    }
}
#[doc = "This bit is used to toggle DIO17 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio17 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio17NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio17Toggle = 1,
}
impl From<Douttgl31_0Dio17> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO17` writer - This bit is used to toggle DIO17 output."]
pub type Douttgl31_0Dio17W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio17>;
impl<'a, REG> Douttgl31_0Dio17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio17_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio17::Douttgl31_0Dio17NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio17_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio17::Douttgl31_0Dio17Toggle)
    }
}
#[doc = "This bit is used to toggle DIO18 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio18 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio18NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio18Toggle = 1,
}
impl From<Douttgl31_0Dio18> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO18` writer - This bit is used to toggle DIO18 output."]
pub type Douttgl31_0Dio18W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio18>;
impl<'a, REG> Douttgl31_0Dio18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio18_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio18::Douttgl31_0Dio18NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio18_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio18::Douttgl31_0Dio18Toggle)
    }
}
#[doc = "This bit is used to toggle DIO19 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio19 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio19NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio19Toggle = 1,
}
impl From<Douttgl31_0Dio19> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO19` writer - This bit is used to toggle DIO19 output."]
pub type Douttgl31_0Dio19W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio19>;
impl<'a, REG> Douttgl31_0Dio19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio19_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio19::Douttgl31_0Dio19NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio19_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio19::Douttgl31_0Dio19Toggle)
    }
}
#[doc = "This bit is used to toggle DIO20 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio20 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio20NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio20Toggle = 1,
}
impl From<Douttgl31_0Dio20> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO20` writer - This bit is used to toggle DIO20 output."]
pub type Douttgl31_0Dio20W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio20>;
impl<'a, REG> Douttgl31_0Dio20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio20_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio20::Douttgl31_0Dio20NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio20_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio20::Douttgl31_0Dio20Toggle)
    }
}
#[doc = "This bit is used to toggle DIO21 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio21 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio21NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio21Toggle = 1,
}
impl From<Douttgl31_0Dio21> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO21` writer - This bit is used to toggle DIO21 output."]
pub type Douttgl31_0Dio21W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio21>;
impl<'a, REG> Douttgl31_0Dio21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio21_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio21::Douttgl31_0Dio21NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio21_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio21::Douttgl31_0Dio21Toggle)
    }
}
#[doc = "This bit is used to toggle DIO22 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio22 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio22NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio22Toggle = 1,
}
impl From<Douttgl31_0Dio22> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO22` writer - This bit is used to toggle DIO22 output."]
pub type Douttgl31_0Dio22W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio22>;
impl<'a, REG> Douttgl31_0Dio22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio22_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio22::Douttgl31_0Dio22NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio22_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio22::Douttgl31_0Dio22Toggle)
    }
}
#[doc = "This bit is used to toggle DIO23 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio23 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio23NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio23Toggle = 1,
}
impl From<Douttgl31_0Dio23> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO23` writer - This bit is used to toggle DIO23 output."]
pub type Douttgl31_0Dio23W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio23>;
impl<'a, REG> Douttgl31_0Dio23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio23_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio23::Douttgl31_0Dio23NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio23_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio23::Douttgl31_0Dio23Toggle)
    }
}
#[doc = "This bit is used to toggle DIO24 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio24 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio24NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio24Toggle = 1,
}
impl From<Douttgl31_0Dio24> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO24` writer - This bit is used to toggle DIO24 output."]
pub type Douttgl31_0Dio24W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio24>;
impl<'a, REG> Douttgl31_0Dio24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio24_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio24::Douttgl31_0Dio24NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio24_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio24::Douttgl31_0Dio24Toggle)
    }
}
#[doc = "This bit is used to toggle DIO25 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio25 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio25NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio25Toggle = 1,
}
impl From<Douttgl31_0Dio25> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO25` writer - This bit is used to toggle DIO25 output."]
pub type Douttgl31_0Dio25W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio25>;
impl<'a, REG> Douttgl31_0Dio25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio25_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio25::Douttgl31_0Dio25NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio25_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio25::Douttgl31_0Dio25Toggle)
    }
}
#[doc = "This bit is used to toggle DIO26 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio26 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio26NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio26Toggle = 1,
}
impl From<Douttgl31_0Dio26> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO26` writer - This bit is used to toggle DIO26 output."]
pub type Douttgl31_0Dio26W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio26>;
impl<'a, REG> Douttgl31_0Dio26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio26_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio26::Douttgl31_0Dio26NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio26_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio26::Douttgl31_0Dio26Toggle)
    }
}
#[doc = "This bit is used to toggle DIO27 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio27 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio27NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio27Toggle = 1,
}
impl From<Douttgl31_0Dio27> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO27` writer - This bit is used to toggle DIO27 output."]
pub type Douttgl31_0Dio27W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio27>;
impl<'a, REG> Douttgl31_0Dio27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio27_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio27::Douttgl31_0Dio27NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio27_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio27::Douttgl31_0Dio27Toggle)
    }
}
#[doc = "This bit is used to toggle DIO28 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio28 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio28NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio28Toggle = 1,
}
impl From<Douttgl31_0Dio28> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO28` writer - This bit is used to toggle DIO28 output."]
pub type Douttgl31_0Dio28W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio28>;
impl<'a, REG> Douttgl31_0Dio28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio28_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio28::Douttgl31_0Dio28NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio28_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio28::Douttgl31_0Dio28Toggle)
    }
}
#[doc = "This bit is used to toggle DIO29 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio29 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio29NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio29Toggle = 1,
}
impl From<Douttgl31_0Dio29> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO29` writer - This bit is used to toggle DIO29 output."]
pub type Douttgl31_0Dio29W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio29>;
impl<'a, REG> Douttgl31_0Dio29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio29_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio29::Douttgl31_0Dio29NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio29_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio29::Douttgl31_0Dio29Toggle)
    }
}
#[doc = "This bit is used to toggle DIO30 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio30 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio30NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio30Toggle = 1,
}
impl From<Douttgl31_0Dio30> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO30` writer - This bit is used to toggle DIO30 output."]
pub type Douttgl31_0Dio30W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio30>;
impl<'a, REG> Douttgl31_0Dio30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio30_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio30::Douttgl31_0Dio30NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio30_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio30::Douttgl31_0Dio30Toggle)
    }
}
#[doc = "This bit is used to toggle DIO31 output.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Douttgl31_0Dio31 {
    #[doc = "0: NO_EFFECT"]
    Douttgl31_0Dio31NoEffect = 0,
    #[doc = "1: TOGGLE"]
    Douttgl31_0Dio31Toggle = 1,
}
impl From<Douttgl31_0Dio31> for bool {
    #[inline(always)]
    fn from(variant: Douttgl31_0Dio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTTGL31_0_DIO31` writer - This bit is used to toggle DIO31 output."]
pub type Douttgl31_0Dio31W<'a, REG> = crate::BitWriter<'a, REG, Douttgl31_0Dio31>;
impl<'a, REG> Douttgl31_0Dio31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn douttgl31_0_dio31_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio31::Douttgl31_0Dio31NoEffect)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn douttgl31_0_dio31_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Douttgl31_0Dio31::Douttgl31_0Dio31Toggle)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to toggle DIO0 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio0(&mut self) -> Douttgl31_0Dio0W<Douttgl31_0Spec> {
        Douttgl31_0Dio0W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit is used to toggle DIO1 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio1(&mut self) -> Douttgl31_0Dio1W<Douttgl31_0Spec> {
        Douttgl31_0Dio1W::new(self, 1)
    }
    #[doc = "Bit 2 - This bit is used to toggle DIO2 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio2(&mut self) -> Douttgl31_0Dio2W<Douttgl31_0Spec> {
        Douttgl31_0Dio2W::new(self, 2)
    }
    #[doc = "Bit 3 - This bit is used to toggle DIO3 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio3(&mut self) -> Douttgl31_0Dio3W<Douttgl31_0Spec> {
        Douttgl31_0Dio3W::new(self, 3)
    }
    #[doc = "Bit 4 - This bit is used to toggle DIO4 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio4(&mut self) -> Douttgl31_0Dio4W<Douttgl31_0Spec> {
        Douttgl31_0Dio4W::new(self, 4)
    }
    #[doc = "Bit 5 - This bit is used to toggle DIO5 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio5(&mut self) -> Douttgl31_0Dio5W<Douttgl31_0Spec> {
        Douttgl31_0Dio5W::new(self, 5)
    }
    #[doc = "Bit 6 - This bit is used to toggle DIO6 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio6(&mut self) -> Douttgl31_0Dio6W<Douttgl31_0Spec> {
        Douttgl31_0Dio6W::new(self, 6)
    }
    #[doc = "Bit 7 - This bit is used to toggle DIO7 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio7(&mut self) -> Douttgl31_0Dio7W<Douttgl31_0Spec> {
        Douttgl31_0Dio7W::new(self, 7)
    }
    #[doc = "Bit 8 - This bit is used to toggle DIO8 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio8(&mut self) -> Douttgl31_0Dio8W<Douttgl31_0Spec> {
        Douttgl31_0Dio8W::new(self, 8)
    }
    #[doc = "Bit 9 - This bit is used to toggle DIO9 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio9(&mut self) -> Douttgl31_0Dio9W<Douttgl31_0Spec> {
        Douttgl31_0Dio9W::new(self, 9)
    }
    #[doc = "Bit 10 - This bit is used to toggle DIO10 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio10(&mut self) -> Douttgl31_0Dio10W<Douttgl31_0Spec> {
        Douttgl31_0Dio10W::new(self, 10)
    }
    #[doc = "Bit 11 - This bit is used to toggle DIO11 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio11(&mut self) -> Douttgl31_0Dio11W<Douttgl31_0Spec> {
        Douttgl31_0Dio11W::new(self, 11)
    }
    #[doc = "Bit 12 - This bit is used to toggle DIO12 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio12(&mut self) -> Douttgl31_0Dio12W<Douttgl31_0Spec> {
        Douttgl31_0Dio12W::new(self, 12)
    }
    #[doc = "Bit 13 - This bit is used to toggle DIO13 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio13(&mut self) -> Douttgl31_0Dio13W<Douttgl31_0Spec> {
        Douttgl31_0Dio13W::new(self, 13)
    }
    #[doc = "Bit 14 - This bit is used to toggle DIO14 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio14(&mut self) -> Douttgl31_0Dio14W<Douttgl31_0Spec> {
        Douttgl31_0Dio14W::new(self, 14)
    }
    #[doc = "Bit 15 - This bit is used to toggle DIO15 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio15(&mut self) -> Douttgl31_0Dio15W<Douttgl31_0Spec> {
        Douttgl31_0Dio15W::new(self, 15)
    }
    #[doc = "Bit 16 - This bit is used to toggle DIO16 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio16(&mut self) -> Douttgl31_0Dio16W<Douttgl31_0Spec> {
        Douttgl31_0Dio16W::new(self, 16)
    }
    #[doc = "Bit 17 - This bit is used to toggle DIO17 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio17(&mut self) -> Douttgl31_0Dio17W<Douttgl31_0Spec> {
        Douttgl31_0Dio17W::new(self, 17)
    }
    #[doc = "Bit 18 - This bit is used to toggle DIO18 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio18(&mut self) -> Douttgl31_0Dio18W<Douttgl31_0Spec> {
        Douttgl31_0Dio18W::new(self, 18)
    }
    #[doc = "Bit 19 - This bit is used to toggle DIO19 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio19(&mut self) -> Douttgl31_0Dio19W<Douttgl31_0Spec> {
        Douttgl31_0Dio19W::new(self, 19)
    }
    #[doc = "Bit 20 - This bit is used to toggle DIO20 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio20(&mut self) -> Douttgl31_0Dio20W<Douttgl31_0Spec> {
        Douttgl31_0Dio20W::new(self, 20)
    }
    #[doc = "Bit 21 - This bit is used to toggle DIO21 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio21(&mut self) -> Douttgl31_0Dio21W<Douttgl31_0Spec> {
        Douttgl31_0Dio21W::new(self, 21)
    }
    #[doc = "Bit 22 - This bit is used to toggle DIO22 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio22(&mut self) -> Douttgl31_0Dio22W<Douttgl31_0Spec> {
        Douttgl31_0Dio22W::new(self, 22)
    }
    #[doc = "Bit 23 - This bit is used to toggle DIO23 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio23(&mut self) -> Douttgl31_0Dio23W<Douttgl31_0Spec> {
        Douttgl31_0Dio23W::new(self, 23)
    }
    #[doc = "Bit 24 - This bit is used to toggle DIO24 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio24(&mut self) -> Douttgl31_0Dio24W<Douttgl31_0Spec> {
        Douttgl31_0Dio24W::new(self, 24)
    }
    #[doc = "Bit 25 - This bit is used to toggle DIO25 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio25(&mut self) -> Douttgl31_0Dio25W<Douttgl31_0Spec> {
        Douttgl31_0Dio25W::new(self, 25)
    }
    #[doc = "Bit 26 - This bit is used to toggle DIO26 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio26(&mut self) -> Douttgl31_0Dio26W<Douttgl31_0Spec> {
        Douttgl31_0Dio26W::new(self, 26)
    }
    #[doc = "Bit 27 - This bit is used to toggle DIO27 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio27(&mut self) -> Douttgl31_0Dio27W<Douttgl31_0Spec> {
        Douttgl31_0Dio27W::new(self, 27)
    }
    #[doc = "Bit 28 - This bit is used to toggle DIO28 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio28(&mut self) -> Douttgl31_0Dio28W<Douttgl31_0Spec> {
        Douttgl31_0Dio28W::new(self, 28)
    }
    #[doc = "Bit 29 - This bit is used to toggle DIO29 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio29(&mut self) -> Douttgl31_0Dio29W<Douttgl31_0Spec> {
        Douttgl31_0Dio29W::new(self, 29)
    }
    #[doc = "Bit 30 - This bit is used to toggle DIO30 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio30(&mut self) -> Douttgl31_0Dio30W<Douttgl31_0Spec> {
        Douttgl31_0Dio30W::new(self, 30)
    }
    #[doc = "Bit 31 - This bit is used to toggle DIO31 output."]
    #[inline(always)]
    pub fn douttgl31_0_dio31(&mut self) -> Douttgl31_0Dio31W<Douttgl31_0Spec> {
        Douttgl31_0Dio31W::new(self, 31)
    }
}
#[doc = "Data output toggle 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`douttgl31_0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Douttgl31_0Spec;
impl crate::RegisterSpec for Douttgl31_0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`douttgl31_0::W`](W) writer structure"]
impl crate::Writable for Douttgl31_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTTGL31_0 to value 0"]
impl crate::Resettable for Douttgl31_0Spec {
    const RESET_VALUE: u32 = 0;
}
