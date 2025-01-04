#[doc = "Register `DOUTCLR31_0` writer"]
pub type W = crate::W<Doutclr31_0Spec>;
#[doc = "Writing 1 to this bit clears the DIO0 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio0 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio0NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio0Clr = 1,
}
impl From<Doutclr31_0Dio0> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO0` writer - Writing 1 to this bit clears the DIO0 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio0W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio0>;
impl<'a, REG> Doutclr31_0Dio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio0::Doutclr31_0Dio0NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio0::Doutclr31_0Dio0Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO1 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio1 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio1NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio1Clr = 1,
}
impl From<Doutclr31_0Dio1> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO1` writer - Writing 1 to this bit clears the DIO1 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio1W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio1>;
impl<'a, REG> Doutclr31_0Dio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio1::Doutclr31_0Dio1NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio1::Doutclr31_0Dio1Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO2 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio2 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio2NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio2Clr = 1,
}
impl From<Doutclr31_0Dio2> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO2` writer - Writing 1 to this bit clears the DIO2 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio2W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio2>;
impl<'a, REG> Doutclr31_0Dio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio2::Doutclr31_0Dio2NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio2::Doutclr31_0Dio2Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO3 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio3 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio3NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio3Clr = 1,
}
impl From<Doutclr31_0Dio3> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO3` writer - Writing 1 to this bit clears the DIO3 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio3W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio3>;
impl<'a, REG> Doutclr31_0Dio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio3_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio3::Doutclr31_0Dio3NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio3_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio3::Doutclr31_0Dio3Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO4 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio4 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio4NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio4Clr = 1,
}
impl From<Doutclr31_0Dio4> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO4` writer - Writing 1 to this bit clears the DIO4 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio4W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio4>;
impl<'a, REG> Doutclr31_0Dio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio4_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio4::Doutclr31_0Dio4NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio4_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio4::Doutclr31_0Dio4Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO5 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio5 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio5NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio5Clr = 1,
}
impl From<Doutclr31_0Dio5> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO5` writer - Writing 1 to this bit clears the DIO5 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio5W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio5>;
impl<'a, REG> Doutclr31_0Dio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio5_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio5::Doutclr31_0Dio5NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio5_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio5::Doutclr31_0Dio5Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO6 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio6 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio6NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio6Clr = 1,
}
impl From<Doutclr31_0Dio6> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO6` writer - Writing 1 to this bit clears the DIO6 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio6W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio6>;
impl<'a, REG> Doutclr31_0Dio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio6_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio6::Doutclr31_0Dio6NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio6_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio6::Doutclr31_0Dio6Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO7 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio7 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio7NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio7Clr = 1,
}
impl From<Doutclr31_0Dio7> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO7` writer - Writing 1 to this bit clears the DIO7 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio7W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio7>;
impl<'a, REG> Doutclr31_0Dio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio7_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio7::Doutclr31_0Dio7NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio7_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio7::Doutclr31_0Dio7Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO8 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio8 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio8NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio8Clr = 1,
}
impl From<Doutclr31_0Dio8> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO8` writer - Writing 1 to this bit clears the DIO8 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio8W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio8>;
impl<'a, REG> Doutclr31_0Dio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio8_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio8::Doutclr31_0Dio8NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio8_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio8::Doutclr31_0Dio8Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO9 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio9 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio9NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio9Clr = 1,
}
impl From<Doutclr31_0Dio9> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO9` writer - Writing 1 to this bit clears the DIO9 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio9W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio9>;
impl<'a, REG> Doutclr31_0Dio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio9_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio9::Doutclr31_0Dio9NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio9_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio9::Doutclr31_0Dio9Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO10 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio10 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio10NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio10Clr = 1,
}
impl From<Doutclr31_0Dio10> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO10` writer - Writing 1 to this bit clears the DIO10 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio10W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio10>;
impl<'a, REG> Doutclr31_0Dio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio10_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio10::Doutclr31_0Dio10NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio10_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio10::Doutclr31_0Dio10Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO11 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio11 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio11NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio11Clr = 1,
}
impl From<Doutclr31_0Dio11> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO11` writer - Writing 1 to this bit clears the DIO11 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio11W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio11>;
impl<'a, REG> Doutclr31_0Dio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio11_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio11::Doutclr31_0Dio11NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio11_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio11::Doutclr31_0Dio11Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO12 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio12 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio12NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio12Clr = 1,
}
impl From<Doutclr31_0Dio12> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO12` writer - Writing 1 to this bit clears the DIO12 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio12W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio12>;
impl<'a, REG> Doutclr31_0Dio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio12_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio12::Doutclr31_0Dio12NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio12_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio12::Doutclr31_0Dio12Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO13 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio13 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio13NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio13Clr = 1,
}
impl From<Doutclr31_0Dio13> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO13` writer - Writing 1 to this bit clears the DIO13 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio13W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio13>;
impl<'a, REG> Doutclr31_0Dio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio13_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio13::Doutclr31_0Dio13NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio13_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio13::Doutclr31_0Dio13Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO14 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio14 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio14NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio14Clr = 1,
}
impl From<Doutclr31_0Dio14> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO14` writer - Writing 1 to this bit clears the DIO14 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio14W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio14>;
impl<'a, REG> Doutclr31_0Dio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio14_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio14::Doutclr31_0Dio14NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio14_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio14::Doutclr31_0Dio14Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO15 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio15 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio15NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio15Clr = 1,
}
impl From<Doutclr31_0Dio15> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO15` writer - Writing 1 to this bit clears the DIO15 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio15W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio15>;
impl<'a, REG> Doutclr31_0Dio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio15_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio15::Doutclr31_0Dio15NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio15_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio15::Doutclr31_0Dio15Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO16 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio16 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio16NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio16Clr = 1,
}
impl From<Doutclr31_0Dio16> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO16` writer - Writing 1 to this bit clears the DIO16 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio16W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio16>;
impl<'a, REG> Doutclr31_0Dio16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio16_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio16::Doutclr31_0Dio16NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio16_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio16::Doutclr31_0Dio16Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO17 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio17 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio17NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio17Clr = 1,
}
impl From<Doutclr31_0Dio17> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO17` writer - Writing 1 to this bit clears the DIO17 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio17W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio17>;
impl<'a, REG> Doutclr31_0Dio17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio17_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio17::Doutclr31_0Dio17NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio17_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio17::Doutclr31_0Dio17Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO18 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio18 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio18NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio18Clr = 1,
}
impl From<Doutclr31_0Dio18> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO18` writer - Writing 1 to this bit clears the DIO18 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio18W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio18>;
impl<'a, REG> Doutclr31_0Dio18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio18_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio18::Doutclr31_0Dio18NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio18_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio18::Doutclr31_0Dio18Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO19 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio19 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio19NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio19Clr = 1,
}
impl From<Doutclr31_0Dio19> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO19` writer - Writing 1 to this bit clears the DIO19 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio19W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio19>;
impl<'a, REG> Doutclr31_0Dio19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio19_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio19::Doutclr31_0Dio19NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio19_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio19::Doutclr31_0Dio19Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO20 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio20 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio20NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio20Clr = 1,
}
impl From<Doutclr31_0Dio20> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO20` writer - Writing 1 to this bit clears the DIO20 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio20W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio20>;
impl<'a, REG> Doutclr31_0Dio20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio20_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio20::Doutclr31_0Dio20NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio20_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio20::Doutclr31_0Dio20Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO21 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio21 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio21NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio21Clr = 1,
}
impl From<Doutclr31_0Dio21> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO21` writer - Writing 1 to this bit clears the DIO21 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio21W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio21>;
impl<'a, REG> Doutclr31_0Dio21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio21_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio21::Doutclr31_0Dio21NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio21_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio21::Doutclr31_0Dio21Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO22 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio22 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio22NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio22Clr = 1,
}
impl From<Doutclr31_0Dio22> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO22` writer - Writing 1 to this bit clears the DIO22 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio22W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio22>;
impl<'a, REG> Doutclr31_0Dio22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio22_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio22::Doutclr31_0Dio22NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio22_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio22::Doutclr31_0Dio22Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO23 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio23 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio23NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio23Clr = 1,
}
impl From<Doutclr31_0Dio23> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO23` writer - Writing 1 to this bit clears the DIO23 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio23W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio23>;
impl<'a, REG> Doutclr31_0Dio23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio23_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio23::Doutclr31_0Dio23NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio23_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio23::Doutclr31_0Dio23Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO24 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio24 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio24NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio24Clr = 1,
}
impl From<Doutclr31_0Dio24> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO24` writer - Writing 1 to this bit clears the DIO24 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio24W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio24>;
impl<'a, REG> Doutclr31_0Dio24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio24_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio24::Doutclr31_0Dio24NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio24_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio24::Doutclr31_0Dio24Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO25 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio25 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio25NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio25Clr = 1,
}
impl From<Doutclr31_0Dio25> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO25` writer - Writing 1 to this bit clears the DIO25 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio25W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio25>;
impl<'a, REG> Doutclr31_0Dio25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio25_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio25::Doutclr31_0Dio25NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio25_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio25::Doutclr31_0Dio25Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO26 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio26 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio26NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio26Clr = 1,
}
impl From<Doutclr31_0Dio26> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO26` writer - Writing 1 to this bit clears the DIO26 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio26W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio26>;
impl<'a, REG> Doutclr31_0Dio26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio26_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio26::Doutclr31_0Dio26NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio26_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio26::Doutclr31_0Dio26Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO27 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio27 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio27NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio27Clr = 1,
}
impl From<Doutclr31_0Dio27> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO27` writer - Writing 1 to this bit clears the DIO27 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio27W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio27>;
impl<'a, REG> Doutclr31_0Dio27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio27_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio27::Doutclr31_0Dio27NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio27_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio27::Doutclr31_0Dio27Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO28 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio28 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio28NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio28Clr = 1,
}
impl From<Doutclr31_0Dio28> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO28` writer - Writing 1 to this bit clears the DIO28 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio28W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio28>;
impl<'a, REG> Doutclr31_0Dio28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio28_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio28::Doutclr31_0Dio28NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio28_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio28::Doutclr31_0Dio28Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO29 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio29 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio29NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio29Clr = 1,
}
impl From<Doutclr31_0Dio29> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO29` writer - Writing 1 to this bit clears the DIO29 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio29W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio29>;
impl<'a, REG> Doutclr31_0Dio29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio29_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio29::Doutclr31_0Dio29NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio29_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio29::Doutclr31_0Dio29Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO30 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio30 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio30NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio30Clr = 1,
}
impl From<Doutclr31_0Dio30> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO30` writer - Writing 1 to this bit clears the DIO30 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio30W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio30>;
impl<'a, REG> Doutclr31_0Dio30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio30_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio30::Doutclr31_0Dio30NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio30_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio30::Doutclr31_0Dio30Clr)
    }
}
#[doc = "Writing 1 to this bit clears the DIO31 bit in the DOUT31_0 register. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doutclr31_0Dio31 {
    #[doc = "0: NO_EFFECT"]
    Doutclr31_0Dio31NoEffect = 0,
    #[doc = "1: CLR"]
    Doutclr31_0Dio31Clr = 1,
}
impl From<Doutclr31_0Dio31> for bool {
    #[inline(always)]
    fn from(variant: Doutclr31_0Dio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUTCLR31_0_DIO31` writer - Writing 1 to this bit clears the DIO31 bit in the DOUT31_0 register. Writing 0 has no effect."]
pub type Doutclr31_0Dio31W<'a, REG> = crate::BitWriter<'a, REG, Doutclr31_0Dio31>;
impl<'a, REG> Doutclr31_0Dio31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn doutclr31_0_dio31_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio31::Doutclr31_0Dio31NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn doutclr31_0_dio31_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Doutclr31_0Dio31::Doutclr31_0Dio31Clr)
    }
}
impl W {
    #[doc = "Bit 0 - Writing 1 to this bit clears the DIO0 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio0(&mut self) -> Doutclr31_0Dio0W<Doutclr31_0Spec> {
        Doutclr31_0Dio0W::new(self, 0)
    }
    #[doc = "Bit 1 - Writing 1 to this bit clears the DIO1 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio1(&mut self) -> Doutclr31_0Dio1W<Doutclr31_0Spec> {
        Doutclr31_0Dio1W::new(self, 1)
    }
    #[doc = "Bit 2 - Writing 1 to this bit clears the DIO2 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio2(&mut self) -> Doutclr31_0Dio2W<Doutclr31_0Spec> {
        Doutclr31_0Dio2W::new(self, 2)
    }
    #[doc = "Bit 3 - Writing 1 to this bit clears the DIO3 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio3(&mut self) -> Doutclr31_0Dio3W<Doutclr31_0Spec> {
        Doutclr31_0Dio3W::new(self, 3)
    }
    #[doc = "Bit 4 - Writing 1 to this bit clears the DIO4 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio4(&mut self) -> Doutclr31_0Dio4W<Doutclr31_0Spec> {
        Doutclr31_0Dio4W::new(self, 4)
    }
    #[doc = "Bit 5 - Writing 1 to this bit clears the DIO5 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio5(&mut self) -> Doutclr31_0Dio5W<Doutclr31_0Spec> {
        Doutclr31_0Dio5W::new(self, 5)
    }
    #[doc = "Bit 6 - Writing 1 to this bit clears the DIO6 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio6(&mut self) -> Doutclr31_0Dio6W<Doutclr31_0Spec> {
        Doutclr31_0Dio6W::new(self, 6)
    }
    #[doc = "Bit 7 - Writing 1 to this bit clears the DIO7 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio7(&mut self) -> Doutclr31_0Dio7W<Doutclr31_0Spec> {
        Doutclr31_0Dio7W::new(self, 7)
    }
    #[doc = "Bit 8 - Writing 1 to this bit clears the DIO8 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio8(&mut self) -> Doutclr31_0Dio8W<Doutclr31_0Spec> {
        Doutclr31_0Dio8W::new(self, 8)
    }
    #[doc = "Bit 9 - Writing 1 to this bit clears the DIO9 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio9(&mut self) -> Doutclr31_0Dio9W<Doutclr31_0Spec> {
        Doutclr31_0Dio9W::new(self, 9)
    }
    #[doc = "Bit 10 - Writing 1 to this bit clears the DIO10 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio10(&mut self) -> Doutclr31_0Dio10W<Doutclr31_0Spec> {
        Doutclr31_0Dio10W::new(self, 10)
    }
    #[doc = "Bit 11 - Writing 1 to this bit clears the DIO11 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio11(&mut self) -> Doutclr31_0Dio11W<Doutclr31_0Spec> {
        Doutclr31_0Dio11W::new(self, 11)
    }
    #[doc = "Bit 12 - Writing 1 to this bit clears the DIO12 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio12(&mut self) -> Doutclr31_0Dio12W<Doutclr31_0Spec> {
        Doutclr31_0Dio12W::new(self, 12)
    }
    #[doc = "Bit 13 - Writing 1 to this bit clears the DIO13 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio13(&mut self) -> Doutclr31_0Dio13W<Doutclr31_0Spec> {
        Doutclr31_0Dio13W::new(self, 13)
    }
    #[doc = "Bit 14 - Writing 1 to this bit clears the DIO14 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio14(&mut self) -> Doutclr31_0Dio14W<Doutclr31_0Spec> {
        Doutclr31_0Dio14W::new(self, 14)
    }
    #[doc = "Bit 15 - Writing 1 to this bit clears the DIO15 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio15(&mut self) -> Doutclr31_0Dio15W<Doutclr31_0Spec> {
        Doutclr31_0Dio15W::new(self, 15)
    }
    #[doc = "Bit 16 - Writing 1 to this bit clears the DIO16 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio16(&mut self) -> Doutclr31_0Dio16W<Doutclr31_0Spec> {
        Doutclr31_0Dio16W::new(self, 16)
    }
    #[doc = "Bit 17 - Writing 1 to this bit clears the DIO17 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio17(&mut self) -> Doutclr31_0Dio17W<Doutclr31_0Spec> {
        Doutclr31_0Dio17W::new(self, 17)
    }
    #[doc = "Bit 18 - Writing 1 to this bit clears the DIO18 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio18(&mut self) -> Doutclr31_0Dio18W<Doutclr31_0Spec> {
        Doutclr31_0Dio18W::new(self, 18)
    }
    #[doc = "Bit 19 - Writing 1 to this bit clears the DIO19 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio19(&mut self) -> Doutclr31_0Dio19W<Doutclr31_0Spec> {
        Doutclr31_0Dio19W::new(self, 19)
    }
    #[doc = "Bit 20 - Writing 1 to this bit clears the DIO20 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio20(&mut self) -> Doutclr31_0Dio20W<Doutclr31_0Spec> {
        Doutclr31_0Dio20W::new(self, 20)
    }
    #[doc = "Bit 21 - Writing 1 to this bit clears the DIO21 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio21(&mut self) -> Doutclr31_0Dio21W<Doutclr31_0Spec> {
        Doutclr31_0Dio21W::new(self, 21)
    }
    #[doc = "Bit 22 - Writing 1 to this bit clears the DIO22 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio22(&mut self) -> Doutclr31_0Dio22W<Doutclr31_0Spec> {
        Doutclr31_0Dio22W::new(self, 22)
    }
    #[doc = "Bit 23 - Writing 1 to this bit clears the DIO23 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio23(&mut self) -> Doutclr31_0Dio23W<Doutclr31_0Spec> {
        Doutclr31_0Dio23W::new(self, 23)
    }
    #[doc = "Bit 24 - Writing 1 to this bit clears the DIO24 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio24(&mut self) -> Doutclr31_0Dio24W<Doutclr31_0Spec> {
        Doutclr31_0Dio24W::new(self, 24)
    }
    #[doc = "Bit 25 - Writing 1 to this bit clears the DIO25 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio25(&mut self) -> Doutclr31_0Dio25W<Doutclr31_0Spec> {
        Doutclr31_0Dio25W::new(self, 25)
    }
    #[doc = "Bit 26 - Writing 1 to this bit clears the DIO26 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio26(&mut self) -> Doutclr31_0Dio26W<Doutclr31_0Spec> {
        Doutclr31_0Dio26W::new(self, 26)
    }
    #[doc = "Bit 27 - Writing 1 to this bit clears the DIO27 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio27(&mut self) -> Doutclr31_0Dio27W<Doutclr31_0Spec> {
        Doutclr31_0Dio27W::new(self, 27)
    }
    #[doc = "Bit 28 - Writing 1 to this bit clears the DIO28 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio28(&mut self) -> Doutclr31_0Dio28W<Doutclr31_0Spec> {
        Doutclr31_0Dio28W::new(self, 28)
    }
    #[doc = "Bit 29 - Writing 1 to this bit clears the DIO29 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio29(&mut self) -> Doutclr31_0Dio29W<Doutclr31_0Spec> {
        Doutclr31_0Dio29W::new(self, 29)
    }
    #[doc = "Bit 30 - Writing 1 to this bit clears the DIO30 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio30(&mut self) -> Doutclr31_0Dio30W<Doutclr31_0Spec> {
        Doutclr31_0Dio30W::new(self, 30)
    }
    #[doc = "Bit 31 - Writing 1 to this bit clears the DIO31 bit in the DOUT31_0 register. Writing 0 has no effect."]
    #[inline(always)]
    pub fn doutclr31_0_dio31(&mut self) -> Doutclr31_0Dio31W<Doutclr31_0Spec> {
        Doutclr31_0Dio31W::new(self, 31)
    }
}
#[doc = "Data output clear 31 to 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doutclr31_0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doutclr31_0Spec;
impl crate::RegisterSpec for Doutclr31_0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`doutclr31_0::W`](W) writer structure"]
impl crate::Writable for Doutclr31_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUTCLR31_0 to value 0"]
impl crate::Resettable for Doutclr31_0Spec {
    const RESET_VALUE: u32 = 0;
}
