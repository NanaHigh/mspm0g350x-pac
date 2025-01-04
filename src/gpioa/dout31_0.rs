#[doc = "Register `DOUT31_0` reader"]
pub type R = crate::R<Dout31_0Spec>;
#[doc = "Register `DOUT31_0` writer"]
pub type W = crate::W<Dout31_0Spec>;
#[doc = "This bit sets the value of the pin configured as DIO0 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio0 {
    #[doc = "0: ZERO"]
    Dout31_0Dio0Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio0One = 1,
}
impl From<Dout31_0Dio0> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO0` reader - This bit sets the value of the pin configured as DIO0 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio0R = crate::BitReader<Dout31_0Dio0>;
impl Dout31_0Dio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio0 {
        match self.bits {
            false => Dout31_0Dio0::Dout31_0Dio0Zero,
            true => Dout31_0Dio0::Dout31_0Dio0One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio0_zero(&self) -> bool {
        *self == Dout31_0Dio0::Dout31_0Dio0Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio0_one(&self) -> bool {
        *self == Dout31_0Dio0::Dout31_0Dio0One
    }
}
#[doc = "Field `DOUT31_0_DIO0` writer - This bit sets the value of the pin configured as DIO0 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio0W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio0>;
impl<'a, REG> Dout31_0Dio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio0_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio0::Dout31_0Dio0Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio0_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio0::Dout31_0Dio0One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO1 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio1 {
    #[doc = "0: ZERO"]
    Dout31_0Dio1Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio1One = 1,
}
impl From<Dout31_0Dio1> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO1` reader - This bit sets the value of the pin configured as DIO1 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio1R = crate::BitReader<Dout31_0Dio1>;
impl Dout31_0Dio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio1 {
        match self.bits {
            false => Dout31_0Dio1::Dout31_0Dio1Zero,
            true => Dout31_0Dio1::Dout31_0Dio1One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio1_zero(&self) -> bool {
        *self == Dout31_0Dio1::Dout31_0Dio1Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio1_one(&self) -> bool {
        *self == Dout31_0Dio1::Dout31_0Dio1One
    }
}
#[doc = "Field `DOUT31_0_DIO1` writer - This bit sets the value of the pin configured as DIO1 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio1W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio1>;
impl<'a, REG> Dout31_0Dio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio1_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio1::Dout31_0Dio1Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio1_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio1::Dout31_0Dio1One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO2 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio2 {
    #[doc = "0: ZERO"]
    Dout31_0Dio2Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio2One = 1,
}
impl From<Dout31_0Dio2> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO2` reader - This bit sets the value of the pin configured as DIO2 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio2R = crate::BitReader<Dout31_0Dio2>;
impl Dout31_0Dio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio2 {
        match self.bits {
            false => Dout31_0Dio2::Dout31_0Dio2Zero,
            true => Dout31_0Dio2::Dout31_0Dio2One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio2_zero(&self) -> bool {
        *self == Dout31_0Dio2::Dout31_0Dio2Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio2_one(&self) -> bool {
        *self == Dout31_0Dio2::Dout31_0Dio2One
    }
}
#[doc = "Field `DOUT31_0_DIO2` writer - This bit sets the value of the pin configured as DIO2 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio2W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio2>;
impl<'a, REG> Dout31_0Dio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio2_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio2::Dout31_0Dio2Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio2_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio2::Dout31_0Dio2One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO3 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio3 {
    #[doc = "0: ZERO"]
    Dout31_0Dio3Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio3One = 1,
}
impl From<Dout31_0Dio3> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO3` reader - This bit sets the value of the pin configured as DIO3 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio3R = crate::BitReader<Dout31_0Dio3>;
impl Dout31_0Dio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio3 {
        match self.bits {
            false => Dout31_0Dio3::Dout31_0Dio3Zero,
            true => Dout31_0Dio3::Dout31_0Dio3One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio3_zero(&self) -> bool {
        *self == Dout31_0Dio3::Dout31_0Dio3Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio3_one(&self) -> bool {
        *self == Dout31_0Dio3::Dout31_0Dio3One
    }
}
#[doc = "Field `DOUT31_0_DIO3` writer - This bit sets the value of the pin configured as DIO3 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio3W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio3>;
impl<'a, REG> Dout31_0Dio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio3_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio3::Dout31_0Dio3Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio3_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio3::Dout31_0Dio3One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO4 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio4 {
    #[doc = "0: ZERO"]
    Dout31_0Dio4Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio4One = 1,
}
impl From<Dout31_0Dio4> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO4` reader - This bit sets the value of the pin configured as DIO4 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio4R = crate::BitReader<Dout31_0Dio4>;
impl Dout31_0Dio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio4 {
        match self.bits {
            false => Dout31_0Dio4::Dout31_0Dio4Zero,
            true => Dout31_0Dio4::Dout31_0Dio4One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio4_zero(&self) -> bool {
        *self == Dout31_0Dio4::Dout31_0Dio4Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio4_one(&self) -> bool {
        *self == Dout31_0Dio4::Dout31_0Dio4One
    }
}
#[doc = "Field `DOUT31_0_DIO4` writer - This bit sets the value of the pin configured as DIO4 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio4W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio4>;
impl<'a, REG> Dout31_0Dio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio4_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio4::Dout31_0Dio4Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio4_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio4::Dout31_0Dio4One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO5 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio5 {
    #[doc = "0: ZERO"]
    Dout31_0Dio5Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio5One = 1,
}
impl From<Dout31_0Dio5> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO5` reader - This bit sets the value of the pin configured as DIO5 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio5R = crate::BitReader<Dout31_0Dio5>;
impl Dout31_0Dio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio5 {
        match self.bits {
            false => Dout31_0Dio5::Dout31_0Dio5Zero,
            true => Dout31_0Dio5::Dout31_0Dio5One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio5_zero(&self) -> bool {
        *self == Dout31_0Dio5::Dout31_0Dio5Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio5_one(&self) -> bool {
        *self == Dout31_0Dio5::Dout31_0Dio5One
    }
}
#[doc = "Field `DOUT31_0_DIO5` writer - This bit sets the value of the pin configured as DIO5 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio5W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio5>;
impl<'a, REG> Dout31_0Dio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio5_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio5::Dout31_0Dio5Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio5_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio5::Dout31_0Dio5One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO6 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio6 {
    #[doc = "0: ZERO"]
    Dout31_0Dio6Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio6One = 1,
}
impl From<Dout31_0Dio6> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO6` reader - This bit sets the value of the pin configured as DIO6 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio6R = crate::BitReader<Dout31_0Dio6>;
impl Dout31_0Dio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio6 {
        match self.bits {
            false => Dout31_0Dio6::Dout31_0Dio6Zero,
            true => Dout31_0Dio6::Dout31_0Dio6One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio6_zero(&self) -> bool {
        *self == Dout31_0Dio6::Dout31_0Dio6Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio6_one(&self) -> bool {
        *self == Dout31_0Dio6::Dout31_0Dio6One
    }
}
#[doc = "Field `DOUT31_0_DIO6` writer - This bit sets the value of the pin configured as DIO6 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio6W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio6>;
impl<'a, REG> Dout31_0Dio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio6_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio6::Dout31_0Dio6Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio6_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio6::Dout31_0Dio6One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO7 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio7 {
    #[doc = "0: ZERO"]
    Dout31_0Dio7Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio7One = 1,
}
impl From<Dout31_0Dio7> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO7` reader - This bit sets the value of the pin configured as DIO7 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio7R = crate::BitReader<Dout31_0Dio7>;
impl Dout31_0Dio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio7 {
        match self.bits {
            false => Dout31_0Dio7::Dout31_0Dio7Zero,
            true => Dout31_0Dio7::Dout31_0Dio7One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio7_zero(&self) -> bool {
        *self == Dout31_0Dio7::Dout31_0Dio7Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio7_one(&self) -> bool {
        *self == Dout31_0Dio7::Dout31_0Dio7One
    }
}
#[doc = "Field `DOUT31_0_DIO7` writer - This bit sets the value of the pin configured as DIO7 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio7W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio7>;
impl<'a, REG> Dout31_0Dio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio7_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio7::Dout31_0Dio7Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio7_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio7::Dout31_0Dio7One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO8 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio8 {
    #[doc = "0: ZERO"]
    Dout31_0Dio8Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio8One = 1,
}
impl From<Dout31_0Dio8> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO8` reader - This bit sets the value of the pin configured as DIO8 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio8R = crate::BitReader<Dout31_0Dio8>;
impl Dout31_0Dio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio8 {
        match self.bits {
            false => Dout31_0Dio8::Dout31_0Dio8Zero,
            true => Dout31_0Dio8::Dout31_0Dio8One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio8_zero(&self) -> bool {
        *self == Dout31_0Dio8::Dout31_0Dio8Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio8_one(&self) -> bool {
        *self == Dout31_0Dio8::Dout31_0Dio8One
    }
}
#[doc = "Field `DOUT31_0_DIO8` writer - This bit sets the value of the pin configured as DIO8 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio8W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio8>;
impl<'a, REG> Dout31_0Dio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio8_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio8::Dout31_0Dio8Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio8_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio8::Dout31_0Dio8One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO9 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio9 {
    #[doc = "0: ZERO"]
    Dout31_0Dio9Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio9One = 1,
}
impl From<Dout31_0Dio9> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO9` reader - This bit sets the value of the pin configured as DIO9 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio9R = crate::BitReader<Dout31_0Dio9>;
impl Dout31_0Dio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio9 {
        match self.bits {
            false => Dout31_0Dio9::Dout31_0Dio9Zero,
            true => Dout31_0Dio9::Dout31_0Dio9One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio9_zero(&self) -> bool {
        *self == Dout31_0Dio9::Dout31_0Dio9Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio9_one(&self) -> bool {
        *self == Dout31_0Dio9::Dout31_0Dio9One
    }
}
#[doc = "Field `DOUT31_0_DIO9` writer - This bit sets the value of the pin configured as DIO9 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio9W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio9>;
impl<'a, REG> Dout31_0Dio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio9_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio9::Dout31_0Dio9Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio9_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio9::Dout31_0Dio9One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO10 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio10 {
    #[doc = "0: ZERO"]
    Dout31_0Dio10Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio10One = 1,
}
impl From<Dout31_0Dio10> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO10` reader - This bit sets the value of the pin configured as DIO10 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio10R = crate::BitReader<Dout31_0Dio10>;
impl Dout31_0Dio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio10 {
        match self.bits {
            false => Dout31_0Dio10::Dout31_0Dio10Zero,
            true => Dout31_0Dio10::Dout31_0Dio10One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio10_zero(&self) -> bool {
        *self == Dout31_0Dio10::Dout31_0Dio10Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio10_one(&self) -> bool {
        *self == Dout31_0Dio10::Dout31_0Dio10One
    }
}
#[doc = "Field `DOUT31_0_DIO10` writer - This bit sets the value of the pin configured as DIO10 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio10W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio10>;
impl<'a, REG> Dout31_0Dio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio10_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio10::Dout31_0Dio10Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio10_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio10::Dout31_0Dio10One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO11 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio11 {
    #[doc = "0: ZERO"]
    Dout31_0Dio11Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio11One = 1,
}
impl From<Dout31_0Dio11> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO11` reader - This bit sets the value of the pin configured as DIO11 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio11R = crate::BitReader<Dout31_0Dio11>;
impl Dout31_0Dio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio11 {
        match self.bits {
            false => Dout31_0Dio11::Dout31_0Dio11Zero,
            true => Dout31_0Dio11::Dout31_0Dio11One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio11_zero(&self) -> bool {
        *self == Dout31_0Dio11::Dout31_0Dio11Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio11_one(&self) -> bool {
        *self == Dout31_0Dio11::Dout31_0Dio11One
    }
}
#[doc = "Field `DOUT31_0_DIO11` writer - This bit sets the value of the pin configured as DIO11 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio11W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio11>;
impl<'a, REG> Dout31_0Dio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio11_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio11::Dout31_0Dio11Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio11_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio11::Dout31_0Dio11One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO12 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio12 {
    #[doc = "0: ZERO"]
    Dout31_0Dio12Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio12One = 1,
}
impl From<Dout31_0Dio12> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO12` reader - This bit sets the value of the pin configured as DIO12 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio12R = crate::BitReader<Dout31_0Dio12>;
impl Dout31_0Dio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio12 {
        match self.bits {
            false => Dout31_0Dio12::Dout31_0Dio12Zero,
            true => Dout31_0Dio12::Dout31_0Dio12One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio12_zero(&self) -> bool {
        *self == Dout31_0Dio12::Dout31_0Dio12Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio12_one(&self) -> bool {
        *self == Dout31_0Dio12::Dout31_0Dio12One
    }
}
#[doc = "Field `DOUT31_0_DIO12` writer - This bit sets the value of the pin configured as DIO12 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio12W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio12>;
impl<'a, REG> Dout31_0Dio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio12_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio12::Dout31_0Dio12Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio12_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio12::Dout31_0Dio12One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO13 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio13 {
    #[doc = "0: ZERO"]
    Dout31_0Dio13Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio13One = 1,
}
impl From<Dout31_0Dio13> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO13` reader - This bit sets the value of the pin configured as DIO13 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio13R = crate::BitReader<Dout31_0Dio13>;
impl Dout31_0Dio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio13 {
        match self.bits {
            false => Dout31_0Dio13::Dout31_0Dio13Zero,
            true => Dout31_0Dio13::Dout31_0Dio13One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio13_zero(&self) -> bool {
        *self == Dout31_0Dio13::Dout31_0Dio13Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio13_one(&self) -> bool {
        *self == Dout31_0Dio13::Dout31_0Dio13One
    }
}
#[doc = "Field `DOUT31_0_DIO13` writer - This bit sets the value of the pin configured as DIO13 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio13W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio13>;
impl<'a, REG> Dout31_0Dio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio13_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio13::Dout31_0Dio13Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio13_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio13::Dout31_0Dio13One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO14 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio14 {
    #[doc = "0: ZERO"]
    Dout31_0Dio14Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio14One = 1,
}
impl From<Dout31_0Dio14> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO14` reader - This bit sets the value of the pin configured as DIO14 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio14R = crate::BitReader<Dout31_0Dio14>;
impl Dout31_0Dio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio14 {
        match self.bits {
            false => Dout31_0Dio14::Dout31_0Dio14Zero,
            true => Dout31_0Dio14::Dout31_0Dio14One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio14_zero(&self) -> bool {
        *self == Dout31_0Dio14::Dout31_0Dio14Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio14_one(&self) -> bool {
        *self == Dout31_0Dio14::Dout31_0Dio14One
    }
}
#[doc = "Field `DOUT31_0_DIO14` writer - This bit sets the value of the pin configured as DIO14 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio14W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio14>;
impl<'a, REG> Dout31_0Dio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio14_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio14::Dout31_0Dio14Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio14_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio14::Dout31_0Dio14One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO15 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio15 {
    #[doc = "0: ZERO"]
    Dout31_0Dio15Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio15One = 1,
}
impl From<Dout31_0Dio15> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO15` reader - This bit sets the value of the pin configured as DIO15 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio15R = crate::BitReader<Dout31_0Dio15>;
impl Dout31_0Dio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio15 {
        match self.bits {
            false => Dout31_0Dio15::Dout31_0Dio15Zero,
            true => Dout31_0Dio15::Dout31_0Dio15One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio15_zero(&self) -> bool {
        *self == Dout31_0Dio15::Dout31_0Dio15Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio15_one(&self) -> bool {
        *self == Dout31_0Dio15::Dout31_0Dio15One
    }
}
#[doc = "Field `DOUT31_0_DIO15` writer - This bit sets the value of the pin configured as DIO15 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio15W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio15>;
impl<'a, REG> Dout31_0Dio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio15_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio15::Dout31_0Dio15Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio15_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio15::Dout31_0Dio15One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO16 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio16 {
    #[doc = "0: ZERO"]
    Dout31_0Dio16Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio16One = 1,
}
impl From<Dout31_0Dio16> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO16` reader - This bit sets the value of the pin configured as DIO16 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio16R = crate::BitReader<Dout31_0Dio16>;
impl Dout31_0Dio16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio16 {
        match self.bits {
            false => Dout31_0Dio16::Dout31_0Dio16Zero,
            true => Dout31_0Dio16::Dout31_0Dio16One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio16_zero(&self) -> bool {
        *self == Dout31_0Dio16::Dout31_0Dio16Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio16_one(&self) -> bool {
        *self == Dout31_0Dio16::Dout31_0Dio16One
    }
}
#[doc = "Field `DOUT31_0_DIO16` writer - This bit sets the value of the pin configured as DIO16 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio16W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio16>;
impl<'a, REG> Dout31_0Dio16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio16_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio16::Dout31_0Dio16Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio16_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio16::Dout31_0Dio16One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO17 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio17 {
    #[doc = "0: ZERO"]
    Dout31_0Dio17Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio17One = 1,
}
impl From<Dout31_0Dio17> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO17` reader - This bit sets the value of the pin configured as DIO17 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio17R = crate::BitReader<Dout31_0Dio17>;
impl Dout31_0Dio17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio17 {
        match self.bits {
            false => Dout31_0Dio17::Dout31_0Dio17Zero,
            true => Dout31_0Dio17::Dout31_0Dio17One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio17_zero(&self) -> bool {
        *self == Dout31_0Dio17::Dout31_0Dio17Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio17_one(&self) -> bool {
        *self == Dout31_0Dio17::Dout31_0Dio17One
    }
}
#[doc = "Field `DOUT31_0_DIO17` writer - This bit sets the value of the pin configured as DIO17 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio17W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio17>;
impl<'a, REG> Dout31_0Dio17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio17_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio17::Dout31_0Dio17Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio17_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio17::Dout31_0Dio17One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO18 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio18 {
    #[doc = "0: ZERO"]
    Dout31_0Dio18Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio18One = 1,
}
impl From<Dout31_0Dio18> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO18` reader - This bit sets the value of the pin configured as DIO18 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio18R = crate::BitReader<Dout31_0Dio18>;
impl Dout31_0Dio18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio18 {
        match self.bits {
            false => Dout31_0Dio18::Dout31_0Dio18Zero,
            true => Dout31_0Dio18::Dout31_0Dio18One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio18_zero(&self) -> bool {
        *self == Dout31_0Dio18::Dout31_0Dio18Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio18_one(&self) -> bool {
        *self == Dout31_0Dio18::Dout31_0Dio18One
    }
}
#[doc = "Field `DOUT31_0_DIO18` writer - This bit sets the value of the pin configured as DIO18 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio18W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio18>;
impl<'a, REG> Dout31_0Dio18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio18_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio18::Dout31_0Dio18Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio18_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio18::Dout31_0Dio18One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO19 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio19 {
    #[doc = "0: ZERO"]
    Dout31_0Dio19Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio19One = 1,
}
impl From<Dout31_0Dio19> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO19` reader - This bit sets the value of the pin configured as DIO19 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio19R = crate::BitReader<Dout31_0Dio19>;
impl Dout31_0Dio19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio19 {
        match self.bits {
            false => Dout31_0Dio19::Dout31_0Dio19Zero,
            true => Dout31_0Dio19::Dout31_0Dio19One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio19_zero(&self) -> bool {
        *self == Dout31_0Dio19::Dout31_0Dio19Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio19_one(&self) -> bool {
        *self == Dout31_0Dio19::Dout31_0Dio19One
    }
}
#[doc = "Field `DOUT31_0_DIO19` writer - This bit sets the value of the pin configured as DIO19 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio19W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio19>;
impl<'a, REG> Dout31_0Dio19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio19_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio19::Dout31_0Dio19Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio19_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio19::Dout31_0Dio19One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO20 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio20 {
    #[doc = "0: ZERO"]
    Dout31_0Dio20Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio20One = 1,
}
impl From<Dout31_0Dio20> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO20` reader - This bit sets the value of the pin configured as DIO20 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio20R = crate::BitReader<Dout31_0Dio20>;
impl Dout31_0Dio20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio20 {
        match self.bits {
            false => Dout31_0Dio20::Dout31_0Dio20Zero,
            true => Dout31_0Dio20::Dout31_0Dio20One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio20_zero(&self) -> bool {
        *self == Dout31_0Dio20::Dout31_0Dio20Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio20_one(&self) -> bool {
        *self == Dout31_0Dio20::Dout31_0Dio20One
    }
}
#[doc = "Field `DOUT31_0_DIO20` writer - This bit sets the value of the pin configured as DIO20 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio20W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio20>;
impl<'a, REG> Dout31_0Dio20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio20_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio20::Dout31_0Dio20Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio20_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio20::Dout31_0Dio20One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO21 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio21 {
    #[doc = "0: ZERO"]
    Dout31_0Dio21Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio21One = 1,
}
impl From<Dout31_0Dio21> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO21` reader - This bit sets the value of the pin configured as DIO21 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio21R = crate::BitReader<Dout31_0Dio21>;
impl Dout31_0Dio21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio21 {
        match self.bits {
            false => Dout31_0Dio21::Dout31_0Dio21Zero,
            true => Dout31_0Dio21::Dout31_0Dio21One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio21_zero(&self) -> bool {
        *self == Dout31_0Dio21::Dout31_0Dio21Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio21_one(&self) -> bool {
        *self == Dout31_0Dio21::Dout31_0Dio21One
    }
}
#[doc = "Field `DOUT31_0_DIO21` writer - This bit sets the value of the pin configured as DIO21 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio21W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio21>;
impl<'a, REG> Dout31_0Dio21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio21_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio21::Dout31_0Dio21Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio21_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio21::Dout31_0Dio21One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO22 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio22 {
    #[doc = "0: ZERO"]
    Dout31_0Dio22Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio22One = 1,
}
impl From<Dout31_0Dio22> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO22` reader - This bit sets the value of the pin configured as DIO22 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio22R = crate::BitReader<Dout31_0Dio22>;
impl Dout31_0Dio22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio22 {
        match self.bits {
            false => Dout31_0Dio22::Dout31_0Dio22Zero,
            true => Dout31_0Dio22::Dout31_0Dio22One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio22_zero(&self) -> bool {
        *self == Dout31_0Dio22::Dout31_0Dio22Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio22_one(&self) -> bool {
        *self == Dout31_0Dio22::Dout31_0Dio22One
    }
}
#[doc = "Field `DOUT31_0_DIO22` writer - This bit sets the value of the pin configured as DIO22 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio22W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio22>;
impl<'a, REG> Dout31_0Dio22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio22_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio22::Dout31_0Dio22Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio22_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio22::Dout31_0Dio22One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO23 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio23 {
    #[doc = "0: ZERO"]
    Dout31_0Dio23Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio23One = 1,
}
impl From<Dout31_0Dio23> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO23` reader - This bit sets the value of the pin configured as DIO23 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio23R = crate::BitReader<Dout31_0Dio23>;
impl Dout31_0Dio23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio23 {
        match self.bits {
            false => Dout31_0Dio23::Dout31_0Dio23Zero,
            true => Dout31_0Dio23::Dout31_0Dio23One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio23_zero(&self) -> bool {
        *self == Dout31_0Dio23::Dout31_0Dio23Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio23_one(&self) -> bool {
        *self == Dout31_0Dio23::Dout31_0Dio23One
    }
}
#[doc = "Field `DOUT31_0_DIO23` writer - This bit sets the value of the pin configured as DIO23 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio23W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio23>;
impl<'a, REG> Dout31_0Dio23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio23_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio23::Dout31_0Dio23Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio23_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio23::Dout31_0Dio23One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO24 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio24 {
    #[doc = "0: ZERO"]
    Dout31_0Dio24Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio24One = 1,
}
impl From<Dout31_0Dio24> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO24` reader - This bit sets the value of the pin configured as DIO24 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio24R = crate::BitReader<Dout31_0Dio24>;
impl Dout31_0Dio24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio24 {
        match self.bits {
            false => Dout31_0Dio24::Dout31_0Dio24Zero,
            true => Dout31_0Dio24::Dout31_0Dio24One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio24_zero(&self) -> bool {
        *self == Dout31_0Dio24::Dout31_0Dio24Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio24_one(&self) -> bool {
        *self == Dout31_0Dio24::Dout31_0Dio24One
    }
}
#[doc = "Field `DOUT31_0_DIO24` writer - This bit sets the value of the pin configured as DIO24 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio24W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio24>;
impl<'a, REG> Dout31_0Dio24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio24_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio24::Dout31_0Dio24Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio24_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio24::Dout31_0Dio24One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO25 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio25 {
    #[doc = "0: ZERO"]
    Dout31_0Dio25Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio25One = 1,
}
impl From<Dout31_0Dio25> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO25` reader - This bit sets the value of the pin configured as DIO25 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio25R = crate::BitReader<Dout31_0Dio25>;
impl Dout31_0Dio25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio25 {
        match self.bits {
            false => Dout31_0Dio25::Dout31_0Dio25Zero,
            true => Dout31_0Dio25::Dout31_0Dio25One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio25_zero(&self) -> bool {
        *self == Dout31_0Dio25::Dout31_0Dio25Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio25_one(&self) -> bool {
        *self == Dout31_0Dio25::Dout31_0Dio25One
    }
}
#[doc = "Field `DOUT31_0_DIO25` writer - This bit sets the value of the pin configured as DIO25 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio25W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio25>;
impl<'a, REG> Dout31_0Dio25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio25_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio25::Dout31_0Dio25Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio25_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio25::Dout31_0Dio25One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO26 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio26 {
    #[doc = "0: ZERO"]
    Dout31_0Dio26Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio26One = 1,
}
impl From<Dout31_0Dio26> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO26` reader - This bit sets the value of the pin configured as DIO26 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio26R = crate::BitReader<Dout31_0Dio26>;
impl Dout31_0Dio26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio26 {
        match self.bits {
            false => Dout31_0Dio26::Dout31_0Dio26Zero,
            true => Dout31_0Dio26::Dout31_0Dio26One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio26_zero(&self) -> bool {
        *self == Dout31_0Dio26::Dout31_0Dio26Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio26_one(&self) -> bool {
        *self == Dout31_0Dio26::Dout31_0Dio26One
    }
}
#[doc = "Field `DOUT31_0_DIO26` writer - This bit sets the value of the pin configured as DIO26 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio26W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio26>;
impl<'a, REG> Dout31_0Dio26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio26_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio26::Dout31_0Dio26Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio26_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio26::Dout31_0Dio26One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO27 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio27 {
    #[doc = "0: ZERO"]
    Dout31_0Dio27Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio27One = 1,
}
impl From<Dout31_0Dio27> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO27` reader - This bit sets the value of the pin configured as DIO27 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio27R = crate::BitReader<Dout31_0Dio27>;
impl Dout31_0Dio27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio27 {
        match self.bits {
            false => Dout31_0Dio27::Dout31_0Dio27Zero,
            true => Dout31_0Dio27::Dout31_0Dio27One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio27_zero(&self) -> bool {
        *self == Dout31_0Dio27::Dout31_0Dio27Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio27_one(&self) -> bool {
        *self == Dout31_0Dio27::Dout31_0Dio27One
    }
}
#[doc = "Field `DOUT31_0_DIO27` writer - This bit sets the value of the pin configured as DIO27 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio27W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio27>;
impl<'a, REG> Dout31_0Dio27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio27_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio27::Dout31_0Dio27Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio27_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio27::Dout31_0Dio27One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO28 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio28 {
    #[doc = "0: ZERO"]
    Dout31_0Dio28Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio28One = 1,
}
impl From<Dout31_0Dio28> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO28` reader - This bit sets the value of the pin configured as DIO28 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio28R = crate::BitReader<Dout31_0Dio28>;
impl Dout31_0Dio28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio28 {
        match self.bits {
            false => Dout31_0Dio28::Dout31_0Dio28Zero,
            true => Dout31_0Dio28::Dout31_0Dio28One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio28_zero(&self) -> bool {
        *self == Dout31_0Dio28::Dout31_0Dio28Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio28_one(&self) -> bool {
        *self == Dout31_0Dio28::Dout31_0Dio28One
    }
}
#[doc = "Field `DOUT31_0_DIO28` writer - This bit sets the value of the pin configured as DIO28 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio28W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio28>;
impl<'a, REG> Dout31_0Dio28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio28_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio28::Dout31_0Dio28Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio28_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio28::Dout31_0Dio28One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO29 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio29 {
    #[doc = "0: ZERO"]
    Dout31_0Dio29Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio29One = 1,
}
impl From<Dout31_0Dio29> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO29` reader - This bit sets the value of the pin configured as DIO29 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio29R = crate::BitReader<Dout31_0Dio29>;
impl Dout31_0Dio29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio29 {
        match self.bits {
            false => Dout31_0Dio29::Dout31_0Dio29Zero,
            true => Dout31_0Dio29::Dout31_0Dio29One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio29_zero(&self) -> bool {
        *self == Dout31_0Dio29::Dout31_0Dio29Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio29_one(&self) -> bool {
        *self == Dout31_0Dio29::Dout31_0Dio29One
    }
}
#[doc = "Field `DOUT31_0_DIO29` writer - This bit sets the value of the pin configured as DIO29 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio29W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio29>;
impl<'a, REG> Dout31_0Dio29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio29_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio29::Dout31_0Dio29Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio29_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio29::Dout31_0Dio29One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO30 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio30 {
    #[doc = "0: ZERO"]
    Dout31_0Dio30Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio30One = 1,
}
impl From<Dout31_0Dio30> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO30` reader - This bit sets the value of the pin configured as DIO30 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio30R = crate::BitReader<Dout31_0Dio30>;
impl Dout31_0Dio30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio30 {
        match self.bits {
            false => Dout31_0Dio30::Dout31_0Dio30Zero,
            true => Dout31_0Dio30::Dout31_0Dio30One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio30_zero(&self) -> bool {
        *self == Dout31_0Dio30::Dout31_0Dio30Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio30_one(&self) -> bool {
        *self == Dout31_0Dio30::Dout31_0Dio30One
    }
}
#[doc = "Field `DOUT31_0_DIO30` writer - This bit sets the value of the pin configured as DIO30 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio30W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio30>;
impl<'a, REG> Dout31_0Dio30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio30_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio30::Dout31_0Dio30Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio30_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio30::Dout31_0Dio30One)
    }
}
#[doc = "This bit sets the value of the pin configured as DIO31 when the output is enabled through DOE31_0 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dout31_0Dio31 {
    #[doc = "0: ZERO"]
    Dout31_0Dio31Zero = 0,
    #[doc = "1: ONE"]
    Dout31_0Dio31One = 1,
}
impl From<Dout31_0Dio31> for bool {
    #[inline(always)]
    fn from(variant: Dout31_0Dio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOUT31_0_DIO31` reader - This bit sets the value of the pin configured as DIO31 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio31R = crate::BitReader<Dout31_0Dio31>;
impl Dout31_0Dio31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dout31_0Dio31 {
        match self.bits {
            false => Dout31_0Dio31::Dout31_0Dio31Zero,
            true => Dout31_0Dio31::Dout31_0Dio31One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_dout31_0_dio31_zero(&self) -> bool {
        *self == Dout31_0Dio31::Dout31_0Dio31Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_dout31_0_dio31_one(&self) -> bool {
        *self == Dout31_0Dio31::Dout31_0Dio31One
    }
}
#[doc = "Field `DOUT31_0_DIO31` writer - This bit sets the value of the pin configured as DIO31 when the output is enabled through DOE31_0 register."]
pub type Dout31_0Dio31W<'a, REG> = crate::BitWriter<'a, REG, Dout31_0Dio31>;
impl<'a, REG> Dout31_0Dio31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn dout31_0_dio31_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio31::Dout31_0Dio31Zero)
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn dout31_0_dio31_one(self) -> &'a mut crate::W<REG> {
        self.variant(Dout31_0Dio31::Dout31_0Dio31One)
    }
}
impl R {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO0 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio0(&self) -> Dout31_0Dio0R {
        Dout31_0Dio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit sets the value of the pin configured as DIO1 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio1(&self) -> Dout31_0Dio1R {
        Dout31_0Dio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit sets the value of the pin configured as DIO2 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio2(&self) -> Dout31_0Dio2R {
        Dout31_0Dio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit sets the value of the pin configured as DIO3 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio3(&self) -> Dout31_0Dio3R {
        Dout31_0Dio3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit sets the value of the pin configured as DIO4 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio4(&self) -> Dout31_0Dio4R {
        Dout31_0Dio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit sets the value of the pin configured as DIO5 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio5(&self) -> Dout31_0Dio5R {
        Dout31_0Dio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit sets the value of the pin configured as DIO6 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio6(&self) -> Dout31_0Dio6R {
        Dout31_0Dio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit sets the value of the pin configured as DIO7 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio7(&self) -> Dout31_0Dio7R {
        Dout31_0Dio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO8 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio8(&self) -> Dout31_0Dio8R {
        Dout31_0Dio8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit sets the value of the pin configured as DIO9 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio9(&self) -> Dout31_0Dio9R {
        Dout31_0Dio9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit sets the value of the pin configured as DIO10 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio10(&self) -> Dout31_0Dio10R {
        Dout31_0Dio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit sets the value of the pin configured as DIO11 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio11(&self) -> Dout31_0Dio11R {
        Dout31_0Dio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit sets the value of the pin configured as DIO12 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio12(&self) -> Dout31_0Dio12R {
        Dout31_0Dio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit sets the value of the pin configured as DIO13 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio13(&self) -> Dout31_0Dio13R {
        Dout31_0Dio13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This bit sets the value of the pin configured as DIO14 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio14(&self) -> Dout31_0Dio14R {
        Dout31_0Dio14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This bit sets the value of the pin configured as DIO15 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio15(&self) -> Dout31_0Dio15R {
        Dout31_0Dio15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO16 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio16(&self) -> Dout31_0Dio16R {
        Dout31_0Dio16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - This bit sets the value of the pin configured as DIO17 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio17(&self) -> Dout31_0Dio17R {
        Dout31_0Dio17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - This bit sets the value of the pin configured as DIO18 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio18(&self) -> Dout31_0Dio18R {
        Dout31_0Dio18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - This bit sets the value of the pin configured as DIO19 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio19(&self) -> Dout31_0Dio19R {
        Dout31_0Dio19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - This bit sets the value of the pin configured as DIO20 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio20(&self) -> Dout31_0Dio20R {
        Dout31_0Dio20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - This bit sets the value of the pin configured as DIO21 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio21(&self) -> Dout31_0Dio21R {
        Dout31_0Dio21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - This bit sets the value of the pin configured as DIO22 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio22(&self) -> Dout31_0Dio22R {
        Dout31_0Dio22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - This bit sets the value of the pin configured as DIO23 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio23(&self) -> Dout31_0Dio23R {
        Dout31_0Dio23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO24 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio24(&self) -> Dout31_0Dio24R {
        Dout31_0Dio24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This bit sets the value of the pin configured as DIO25 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio25(&self) -> Dout31_0Dio25R {
        Dout31_0Dio25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - This bit sets the value of the pin configured as DIO26 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio26(&self) -> Dout31_0Dio26R {
        Dout31_0Dio26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - This bit sets the value of the pin configured as DIO27 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio27(&self) -> Dout31_0Dio27R {
        Dout31_0Dio27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - This bit sets the value of the pin configured as DIO28 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio28(&self) -> Dout31_0Dio28R {
        Dout31_0Dio28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - This bit sets the value of the pin configured as DIO29 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio29(&self) -> Dout31_0Dio29R {
        Dout31_0Dio29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - This bit sets the value of the pin configured as DIO30 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio30(&self) -> Dout31_0Dio30R {
        Dout31_0Dio30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit sets the value of the pin configured as DIO31 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio31(&self) -> Dout31_0Dio31R {
        Dout31_0Dio31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit sets the value of the pin configured as DIO0 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio0(&mut self) -> Dout31_0Dio0W<Dout31_0Spec> {
        Dout31_0Dio0W::new(self, 0)
    }
    #[doc = "Bit 1 - This bit sets the value of the pin configured as DIO1 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio1(&mut self) -> Dout31_0Dio1W<Dout31_0Spec> {
        Dout31_0Dio1W::new(self, 1)
    }
    #[doc = "Bit 2 - This bit sets the value of the pin configured as DIO2 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio2(&mut self) -> Dout31_0Dio2W<Dout31_0Spec> {
        Dout31_0Dio2W::new(self, 2)
    }
    #[doc = "Bit 3 - This bit sets the value of the pin configured as DIO3 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio3(&mut self) -> Dout31_0Dio3W<Dout31_0Spec> {
        Dout31_0Dio3W::new(self, 3)
    }
    #[doc = "Bit 4 - This bit sets the value of the pin configured as DIO4 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio4(&mut self) -> Dout31_0Dio4W<Dout31_0Spec> {
        Dout31_0Dio4W::new(self, 4)
    }
    #[doc = "Bit 5 - This bit sets the value of the pin configured as DIO5 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio5(&mut self) -> Dout31_0Dio5W<Dout31_0Spec> {
        Dout31_0Dio5W::new(self, 5)
    }
    #[doc = "Bit 6 - This bit sets the value of the pin configured as DIO6 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio6(&mut self) -> Dout31_0Dio6W<Dout31_0Spec> {
        Dout31_0Dio6W::new(self, 6)
    }
    #[doc = "Bit 7 - This bit sets the value of the pin configured as DIO7 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio7(&mut self) -> Dout31_0Dio7W<Dout31_0Spec> {
        Dout31_0Dio7W::new(self, 7)
    }
    #[doc = "Bit 8 - This bit sets the value of the pin configured as DIO8 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio8(&mut self) -> Dout31_0Dio8W<Dout31_0Spec> {
        Dout31_0Dio8W::new(self, 8)
    }
    #[doc = "Bit 9 - This bit sets the value of the pin configured as DIO9 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio9(&mut self) -> Dout31_0Dio9W<Dout31_0Spec> {
        Dout31_0Dio9W::new(self, 9)
    }
    #[doc = "Bit 10 - This bit sets the value of the pin configured as DIO10 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio10(&mut self) -> Dout31_0Dio10W<Dout31_0Spec> {
        Dout31_0Dio10W::new(self, 10)
    }
    #[doc = "Bit 11 - This bit sets the value of the pin configured as DIO11 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio11(&mut self) -> Dout31_0Dio11W<Dout31_0Spec> {
        Dout31_0Dio11W::new(self, 11)
    }
    #[doc = "Bit 12 - This bit sets the value of the pin configured as DIO12 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio12(&mut self) -> Dout31_0Dio12W<Dout31_0Spec> {
        Dout31_0Dio12W::new(self, 12)
    }
    #[doc = "Bit 13 - This bit sets the value of the pin configured as DIO13 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio13(&mut self) -> Dout31_0Dio13W<Dout31_0Spec> {
        Dout31_0Dio13W::new(self, 13)
    }
    #[doc = "Bit 14 - This bit sets the value of the pin configured as DIO14 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio14(&mut self) -> Dout31_0Dio14W<Dout31_0Spec> {
        Dout31_0Dio14W::new(self, 14)
    }
    #[doc = "Bit 15 - This bit sets the value of the pin configured as DIO15 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio15(&mut self) -> Dout31_0Dio15W<Dout31_0Spec> {
        Dout31_0Dio15W::new(self, 15)
    }
    #[doc = "Bit 16 - This bit sets the value of the pin configured as DIO16 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio16(&mut self) -> Dout31_0Dio16W<Dout31_0Spec> {
        Dout31_0Dio16W::new(self, 16)
    }
    #[doc = "Bit 17 - This bit sets the value of the pin configured as DIO17 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio17(&mut self) -> Dout31_0Dio17W<Dout31_0Spec> {
        Dout31_0Dio17W::new(self, 17)
    }
    #[doc = "Bit 18 - This bit sets the value of the pin configured as DIO18 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio18(&mut self) -> Dout31_0Dio18W<Dout31_0Spec> {
        Dout31_0Dio18W::new(self, 18)
    }
    #[doc = "Bit 19 - This bit sets the value of the pin configured as DIO19 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio19(&mut self) -> Dout31_0Dio19W<Dout31_0Spec> {
        Dout31_0Dio19W::new(self, 19)
    }
    #[doc = "Bit 20 - This bit sets the value of the pin configured as DIO20 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio20(&mut self) -> Dout31_0Dio20W<Dout31_0Spec> {
        Dout31_0Dio20W::new(self, 20)
    }
    #[doc = "Bit 21 - This bit sets the value of the pin configured as DIO21 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio21(&mut self) -> Dout31_0Dio21W<Dout31_0Spec> {
        Dout31_0Dio21W::new(self, 21)
    }
    #[doc = "Bit 22 - This bit sets the value of the pin configured as DIO22 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio22(&mut self) -> Dout31_0Dio22W<Dout31_0Spec> {
        Dout31_0Dio22W::new(self, 22)
    }
    #[doc = "Bit 23 - This bit sets the value of the pin configured as DIO23 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio23(&mut self) -> Dout31_0Dio23W<Dout31_0Spec> {
        Dout31_0Dio23W::new(self, 23)
    }
    #[doc = "Bit 24 - This bit sets the value of the pin configured as DIO24 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio24(&mut self) -> Dout31_0Dio24W<Dout31_0Spec> {
        Dout31_0Dio24W::new(self, 24)
    }
    #[doc = "Bit 25 - This bit sets the value of the pin configured as DIO25 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio25(&mut self) -> Dout31_0Dio25W<Dout31_0Spec> {
        Dout31_0Dio25W::new(self, 25)
    }
    #[doc = "Bit 26 - This bit sets the value of the pin configured as DIO26 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio26(&mut self) -> Dout31_0Dio26W<Dout31_0Spec> {
        Dout31_0Dio26W::new(self, 26)
    }
    #[doc = "Bit 27 - This bit sets the value of the pin configured as DIO27 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio27(&mut self) -> Dout31_0Dio27W<Dout31_0Spec> {
        Dout31_0Dio27W::new(self, 27)
    }
    #[doc = "Bit 28 - This bit sets the value of the pin configured as DIO28 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio28(&mut self) -> Dout31_0Dio28W<Dout31_0Spec> {
        Dout31_0Dio28W::new(self, 28)
    }
    #[doc = "Bit 29 - This bit sets the value of the pin configured as DIO29 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio29(&mut self) -> Dout31_0Dio29W<Dout31_0Spec> {
        Dout31_0Dio29W::new(self, 29)
    }
    #[doc = "Bit 30 - This bit sets the value of the pin configured as DIO30 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio30(&mut self) -> Dout31_0Dio30W<Dout31_0Spec> {
        Dout31_0Dio30W::new(self, 30)
    }
    #[doc = "Bit 31 - This bit sets the value of the pin configured as DIO31 when the output is enabled through DOE31_0 register."]
    #[inline(always)]
    pub fn dout31_0_dio31(&mut self) -> Dout31_0Dio31W<Dout31_0Spec> {
        Dout31_0Dio31W::new(self, 31)
    }
}
#[doc = "Data output 31 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dout31_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dout31_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dout31_0Spec;
impl crate::RegisterSpec for Dout31_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout31_0::R`](R) reader structure"]
impl crate::Readable for Dout31_0Spec {}
#[doc = "`write(|w| ..)` method takes [`dout31_0::W`](W) writer structure"]
impl crate::Writable for Dout31_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOUT31_0 to value 0"]
impl crate::Resettable for Dout31_0Spec {
    const RESET_VALUE: u32 = 0;
}
