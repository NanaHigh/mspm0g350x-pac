#[doc = "Register `DOUTSET31_0` writer"]
pub type W = crate::W<Doutset31_0Spec>;
#[doc = "Writing 1 to this bit sets the DIO0 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio0 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio0NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio0Set = 1,
}
impl From<Doutset31_0Dio0> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO0` writer - Writing 1 to this bit sets the DIO0 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio0W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio0>;
impl<'a, REG> Doutset31_0Dio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio0::Doutset31_0Dio0NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio0_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio0::Doutset31_0Dio0Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO1 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio1 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio1NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio1Set = 1,
}
impl From<Doutset31_0Dio1> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO1` writer - Writing 1 to this bit sets the DIO1 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio1W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio1>;
impl<'a, REG> Doutset31_0Dio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio1::Doutset31_0Dio1NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio1_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio1::Doutset31_0Dio1Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO2 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio2 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio2NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio2Set = 1,
}
impl From<Doutset31_0Dio2> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO2` writer - Writing 1 to this bit sets the DIO2 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio2W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio2>;
impl<'a, REG> Doutset31_0Dio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio2::Doutset31_0Dio2NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio2_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio2::Doutset31_0Dio2Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO3 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio3 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio3NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio3Set = 1,
}
impl From<Doutset31_0Dio3> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO3` writer - Writing 1 to this bit sets the DIO3 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio3W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio3>;
impl<'a, REG> Doutset31_0Dio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio3_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio3::Doutset31_0Dio3NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio3_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio3::Doutset31_0Dio3Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO4 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio4 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio4NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio4Set = 1,
}
impl From<Doutset31_0Dio4> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO4` writer - Writing 1 to this bit sets the DIO4 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio4W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio4>;
impl<'a, REG> Doutset31_0Dio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio4_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio4::Doutset31_0Dio4NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio4_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio4::Doutset31_0Dio4Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO5 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio5 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio5NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio5Set = 1,
}
impl From<Doutset31_0Dio5> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO5` writer - Writing 1 to this bit sets the DIO5 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio5W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio5>;
impl<'a, REG> Doutset31_0Dio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio5_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio5::Doutset31_0Dio5NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio5_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio5::Doutset31_0Dio5Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO6 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio6 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio6NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio6Set = 1,
}
impl From<Doutset31_0Dio6> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO6` writer - Writing 1 to this bit sets the DIO6 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio6W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio6>;
impl<'a, REG> Doutset31_0Dio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio6_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio6::Doutset31_0Dio6NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio6_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio6::Doutset31_0Dio6Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO7 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio7 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio7NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio7Set = 1,
}
impl From<Doutset31_0Dio7> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO7` writer - Writing 1 to this bit sets the DIO7 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio7W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio7>;
impl<'a, REG> Doutset31_0Dio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio7_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio7::Doutset31_0Dio7NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio7_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio7::Doutset31_0Dio7Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO8 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio8 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio8NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio8Set = 1,
}
impl From<Doutset31_0Dio8> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO8` writer - Writing 1 to this bit sets the DIO8 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio8W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio8>;
impl<'a, REG> Doutset31_0Dio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio8_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio8::Doutset31_0Dio8NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio8_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio8::Doutset31_0Dio8Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO9 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio9 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio9NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio9Set = 1,
}
impl From<Doutset31_0Dio9> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO9` writer - Writing 1 to this bit sets the DIO9 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio9W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio9>;
impl<'a, REG> Doutset31_0Dio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio9_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio9::Doutset31_0Dio9NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio9_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio9::Doutset31_0Dio9Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO10 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio10 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio10NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio10Set = 1,
}
impl From<Doutset31_0Dio10> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO10` writer - Writing 1 to this bit sets the DIO10 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio10W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio10>;
impl<'a, REG> Doutset31_0Dio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio10_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio10::Doutset31_0Dio10NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio10_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio10::Doutset31_0Dio10Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO11 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio11 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio11NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio11Set = 1,
}
impl From<Doutset31_0Dio11> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO11` writer - Writing 1 to this bit sets the DIO11 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio11W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio11>;
impl<'a, REG> Doutset31_0Dio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio11_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio11::Doutset31_0Dio11NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio11_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio11::Doutset31_0Dio11Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO12 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio12 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio12NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio12Set = 1,
}
impl From<Doutset31_0Dio12> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO12` writer - Writing 1 to this bit sets the DIO12 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio12W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio12>;
impl<'a, REG> Doutset31_0Dio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio12_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio12::Doutset31_0Dio12NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio12_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio12::Doutset31_0Dio12Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO13 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio13 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio13NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio13Set = 1,
}
impl From<Doutset31_0Dio13> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO13` writer - Writing 1 to this bit sets the DIO13 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio13W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio13>;
impl<'a, REG> Doutset31_0Dio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio13_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio13::Doutset31_0Dio13NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio13_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio13::Doutset31_0Dio13Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO14 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio14 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio14NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio14Set = 1,
}
impl From<Doutset31_0Dio14> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO14` writer - Writing 1 to this bit sets the DIO14 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio14W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio14>;
impl<'a, REG> Doutset31_0Dio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio14_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio14::Doutset31_0Dio14NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio14_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio14::Doutset31_0Dio14Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO15 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio15 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio15NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio15Set = 1,
}
impl From<Doutset31_0Dio15> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO15` writer - Writing 1 to this bit sets the DIO15 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio15W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio15>;
impl<'a, REG> Doutset31_0Dio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio15_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio15::Doutset31_0Dio15NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio15_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio15::Doutset31_0Dio15Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO16 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio16 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio16NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio16Set = 1,
}
impl From<Doutset31_0Dio16> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO16` writer - Writing 1 to this bit sets the DIO16 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio16W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio16>;
impl<'a, REG> Doutset31_0Dio16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio16_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio16::Doutset31_0Dio16NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio16_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio16::Doutset31_0Dio16Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO17 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio17 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio17NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio17Set = 1,
}
impl From<Doutset31_0Dio17> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO17` writer - Writing 1 to this bit sets the DIO17 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio17W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio17>;
impl<'a, REG> Doutset31_0Dio17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio17_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio17::Doutset31_0Dio17NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio17_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio17::Doutset31_0Dio17Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO18 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio18 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio18NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio18Set = 1,
}
impl From<Doutset31_0Dio18> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO18` writer - Writing 1 to this bit sets the DIO18 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio18W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio18>;
impl<'a, REG> Doutset31_0Dio18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio18_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio18::Doutset31_0Dio18NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio18_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio18::Doutset31_0Dio18Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO19 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio19 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio19NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio19Set = 1,
}
impl From<Doutset31_0Dio19> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO19` writer - Writing 1 to this bit sets the DIO19 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio19W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio19>;
impl<'a, REG> Doutset31_0Dio19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio19_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio19::Doutset31_0Dio19NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio19_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio19::Doutset31_0Dio19Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO20 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio20 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio20NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio20Set = 1,
}
impl From<Doutset31_0Dio20> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO20` writer - Writing 1 to this bit sets the DIO20 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio20W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio20>;
impl<'a, REG> Doutset31_0Dio20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio20_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio20::Doutset31_0Dio20NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio20_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio20::Doutset31_0Dio20Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO21 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio21 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio21NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio21Set = 1,
}
impl From<Doutset31_0Dio21> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO21` writer - Writing 1 to this bit sets the DIO21 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio21W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio21>;
impl<'a, REG> Doutset31_0Dio21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio21_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio21::Doutset31_0Dio21NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio21_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio21::Doutset31_0Dio21Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO22 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio22 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio22NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio22Set = 1,
}
impl From<Doutset31_0Dio22> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO22` writer - Writing 1 to this bit sets the DIO22 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio22W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio22>;
impl<'a, REG> Doutset31_0Dio22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio22_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio22::Doutset31_0Dio22NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio22_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio22::Doutset31_0Dio22Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO23 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio23 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio23NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio23Set = 1,
}
impl From<Doutset31_0Dio23> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO23` writer - Writing 1 to this bit sets the DIO23 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio23W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio23>;
impl<'a, REG> Doutset31_0Dio23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio23_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio23::Doutset31_0Dio23NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio23_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio23::Doutset31_0Dio23Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO24 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio24 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio24NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio24Set = 1,
}
impl From<Doutset31_0Dio24> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO24` writer - Writing 1 to this bit sets the DIO24 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio24W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio24>;
impl<'a, REG> Doutset31_0Dio24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio24_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio24::Doutset31_0Dio24NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio24_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio24::Doutset31_0Dio24Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO25 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio25 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio25NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio25Set = 1,
}
impl From<Doutset31_0Dio25> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO25` writer - Writing 1 to this bit sets the DIO25 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio25W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio25>;
impl<'a, REG> Doutset31_0Dio25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio25_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio25::Doutset31_0Dio25NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio25_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio25::Doutset31_0Dio25Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO26 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio26 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio26NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio26Set = 1,
}
impl From<Doutset31_0Dio26> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO26` writer - Writing 1 to this bit sets the DIO26 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio26W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio26>;
impl<'a, REG> Doutset31_0Dio26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio26_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio26::Doutset31_0Dio26NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio26_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio26::Doutset31_0Dio26Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO27 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio27 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio27NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio27Set = 1,
}
impl From<Doutset31_0Dio27> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO27` writer - Writing 1 to this bit sets the DIO27 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio27W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio27>;
impl<'a, REG> Doutset31_0Dio27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio27_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio27::Doutset31_0Dio27NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio27_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio27::Doutset31_0Dio27Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO28 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio28 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio28NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio28Set = 1,
}
impl From<Doutset31_0Dio28> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO28` writer - Writing 1 to this bit sets the DIO28 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio28W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio28>;
impl<'a, REG> Doutset31_0Dio28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio28_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio28::Doutset31_0Dio28NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio28_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio28::Doutset31_0Dio28Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO29 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio29 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio29NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio29Set = 1,
}
impl From<Doutset31_0Dio29> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO29` writer - Writing 1 to this bit sets the DIO29 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio29W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio29>;
impl<'a, REG> Doutset31_0Dio29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio29_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio29::Doutset31_0Dio29NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio29_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio29::Doutset31_0Dio29Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO30 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio30 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio30NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio30Set = 1,
}
impl From<Doutset31_0Dio30> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO30` writer - Writing 1 to this bit sets the DIO30 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio30W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio30>;
impl<'a, REG> Doutset31_0Dio30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio30_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio30::Doutset31_0Dio30NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio30_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio30::Doutset31_0Dio30Set)
    }
}
#[doc = "Writing 1 to this bit sets the DIO31 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutset31_0Dio31 {
    #[doc = "0: NO_EFFECT"]
    Doutset31_0Dio31NoEffect = 0,
    #[doc = "1: SET"]
    Doutset31_0Dio31Set = 1,
}
impl From<Doutset31_0Dio31> for bool {
    #[inline(always)]
    fn from(variant: Doutset31_0Dio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTSET31_0_DIO31` writer - Writing 1 to this bit sets the DIO31 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutset31_0Dio31W<'a, REG> = crate::BitWriter<'a, REG, Doutset31_0Dio31>;
impl<'a, REG> Doutset31_0Dio31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutset31_0_dio31_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio31::Doutset31_0Dio31NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn doutset31_0_dio31_set(self) -> &'a mut crate::W<REG> {
        self.variant(Doutset31_0Dio31::Doutset31_0Dio31Set)
    }
}
impl W {
    #[doc = "Bit 0 - Writing 1 to this bit sets the DIO0 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio0(&mut self) -> Doutset31_0Dio0W<Doutset31_0Spec> {
        Doutset31_0Dio0W::new(self, 0)
    }
    #[doc = "Bit 1 - Writing 1 to this bit sets the DIO1 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio1(&mut self) -> Doutset31_0Dio1W<Doutset31_0Spec> {
        Doutset31_0Dio1W::new(self, 1)
    }
    #[doc = "Bit 2 - Writing 1 to this bit sets the DIO2 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio2(&mut self) -> Doutset31_0Dio2W<Doutset31_0Spec> {
        Doutset31_0Dio2W::new(self, 2)
    }
    #[doc = "Bit 3 - Writing 1 to this bit sets the DIO3 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio3(&mut self) -> Doutset31_0Dio3W<Doutset31_0Spec> {
        Doutset31_0Dio3W::new(self, 3)
    }
    #[doc = "Bit 4 - Writing 1 to this bit sets the DIO4 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio4(&mut self) -> Doutset31_0Dio4W<Doutset31_0Spec> {
        Doutset31_0Dio4W::new(self, 4)
    }
    #[doc = "Bit 5 - Writing 1 to this bit sets the DIO5 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio5(&mut self) -> Doutset31_0Dio5W<Doutset31_0Spec> {
        Doutset31_0Dio5W::new(self, 5)
    }
    #[doc = "Bit 6 - Writing 1 to this bit sets the DIO6 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio6(&mut self) -> Doutset31_0Dio6W<Doutset31_0Spec> {
        Doutset31_0Dio6W::new(self, 6)
    }
    #[doc = "Bit 7 - Writing 1 to this bit sets the DIO7 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio7(&mut self) -> Doutset31_0Dio7W<Doutset31_0Spec> {
        Doutset31_0Dio7W::new(self, 7)
    }
    #[doc = "Bit 8 - Writing 1 to this bit sets the DIO8 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio8(&mut self) -> Doutset31_0Dio8W<Doutset31_0Spec> {
        Doutset31_0Dio8W::new(self, 8)
    }
    #[doc = "Bit 9 - Writing 1 to this bit sets the DIO9 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio9(&mut self) -> Doutset31_0Dio9W<Doutset31_0Spec> {
        Doutset31_0Dio9W::new(self, 9)
    }
    #[doc = "Bit 10 - Writing 1 to this bit sets the DIO10 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio10(&mut self) -> Doutset31_0Dio10W<Doutset31_0Spec> {
        Doutset31_0Dio10W::new(self, 10)
    }
    #[doc = "Bit 11 - Writing 1 to this bit sets the DIO11 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio11(&mut self) -> Doutset31_0Dio11W<Doutset31_0Spec> {
        Doutset31_0Dio11W::new(self, 11)
    }
    #[doc = "Bit 12 - Writing 1 to this bit sets the DIO12 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio12(&mut self) -> Doutset31_0Dio12W<Doutset31_0Spec> {
        Doutset31_0Dio12W::new(self, 12)
    }
    #[doc = "Bit 13 - Writing 1 to this bit sets the DIO13 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio13(&mut self) -> Doutset31_0Dio13W<Doutset31_0Spec> {
        Doutset31_0Dio13W::new(self, 13)
    }
    #[doc = "Bit 14 - Writing 1 to this bit sets the DIO14 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio14(&mut self) -> Doutset31_0Dio14W<Doutset31_0Spec> {
        Doutset31_0Dio14W::new(self, 14)
    }
    #[doc = "Bit 15 - Writing 1 to this bit sets the DIO15 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio15(&mut self) -> Doutset31_0Dio15W<Doutset31_0Spec> {
        Doutset31_0Dio15W::new(self, 15)
    }
    #[doc = "Bit 16 - Writing 1 to this bit sets the DIO16 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio16(&mut self) -> Doutset31_0Dio16W<Doutset31_0Spec> {
        Doutset31_0Dio16W::new(self, 16)
    }
    #[doc = "Bit 17 - Writing 1 to this bit sets the DIO17 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio17(&mut self) -> Doutset31_0Dio17W<Doutset31_0Spec> {
        Doutset31_0Dio17W::new(self, 17)
    }
    #[doc = "Bit 18 - Writing 1 to this bit sets the DIO18 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio18(&mut self) -> Doutset31_0Dio18W<Doutset31_0Spec> {
        Doutset31_0Dio18W::new(self, 18)
    }
    #[doc = "Bit 19 - Writing 1 to this bit sets the DIO19 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio19(&mut self) -> Doutset31_0Dio19W<Doutset31_0Spec> {
        Doutset31_0Dio19W::new(self, 19)
    }
    #[doc = "Bit 20 - Writing 1 to this bit sets the DIO20 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio20(&mut self) -> Doutset31_0Dio20W<Doutset31_0Spec> {
        Doutset31_0Dio20W::new(self, 20)
    }
    #[doc = "Bit 21 - Writing 1 to this bit sets the DIO21 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio21(&mut self) -> Doutset31_0Dio21W<Doutset31_0Spec> {
        Doutset31_0Dio21W::new(self, 21)
    }
    #[doc = "Bit 22 - Writing 1 to this bit sets the DIO22 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio22(&mut self) -> Doutset31_0Dio22W<Doutset31_0Spec> {
        Doutset31_0Dio22W::new(self, 22)
    }
    #[doc = "Bit 23 - Writing 1 to this bit sets the DIO23 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio23(&mut self) -> Doutset31_0Dio23W<Doutset31_0Spec> {
        Doutset31_0Dio23W::new(self, 23)
    }
    #[doc = "Bit 24 - Writing 1 to this bit sets the DIO24 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio24(&mut self) -> Doutset31_0Dio24W<Doutset31_0Spec> {
        Doutset31_0Dio24W::new(self, 24)
    }
    #[doc = "Bit 25 - Writing 1 to this bit sets the DIO25 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio25(&mut self) -> Doutset31_0Dio25W<Doutset31_0Spec> {
        Doutset31_0Dio25W::new(self, 25)
    }
    #[doc = "Bit 26 - Writing 1 to this bit sets the DIO26 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio26(&mut self) -> Doutset31_0Dio26W<Doutset31_0Spec> {
        Doutset31_0Dio26W::new(self, 26)
    }
    #[doc = "Bit 27 - Writing 1 to this bit sets the DIO27 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio27(&mut self) -> Doutset31_0Dio27W<Doutset31_0Spec> {
        Doutset31_0Dio27W::new(self, 27)
    }
    #[doc = "Bit 28 - Writing 1 to this bit sets the DIO28 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio28(&mut self) -> Doutset31_0Dio28W<Doutset31_0Spec> {
        Doutset31_0Dio28W::new(self, 28)
    }
    #[doc = "Bit 29 - Writing 1 to this bit sets the DIO29 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio29(&mut self) -> Doutset31_0Dio29W<Doutset31_0Spec> {
        Doutset31_0Dio29W::new(self, 29)
    }
    #[doc = "Bit 30 - Writing 1 to this bit sets the DIO30 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio30(&mut self) -> Doutset31_0Dio30W<Doutset31_0Spec> {
        Doutset31_0Dio30W::new(self, 30)
    }
    #[doc = "Bit 31 - Writing 1 to this bit sets the DIO31 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutset31_0_dio31(&mut self) -> Doutset31_0Dio31W<Doutset31_0Spec> {
        Doutset31_0Dio31W::new(self, 31)
    }
}
#[doc = "Data output set 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutset31_0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doutset31_0Spec;
impl crate::RegisterSpec for Doutset31_0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`doutset31_0::W`](W) writer structure"]
impl crate::Writable for Doutset31_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTSET31_0 to value 0"]
impl crate::Resettable for Doutset31_0Spec {
    const RESET_VALUE: u32 = 0;
}
