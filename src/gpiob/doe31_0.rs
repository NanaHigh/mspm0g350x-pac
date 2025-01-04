#[doc = "Register `DOE31_0` reader"]
pub type R = crate::R<Doe31_0Spec>;
#[doc = "Register `DOE31_0` writer"]
pub type W = crate::W<Doe31_0Spec>;
#[doc = "Enables data output for DIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio0 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio0Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio0Enable = 1,
}
impl From<Doe31_0Dio0> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO0` reader - Enables data output for DIO0."]
pub type Doe31_0Dio0R = crate::BitReader<Doe31_0Dio0>;
impl Doe31_0Dio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio0 {
        match self.bits {
            false => Doe31_0Dio0::Doe31_0Dio0Disable,
            true => Doe31_0Dio0::Doe31_0Dio0Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio0_disable(&self) -> bool {
        *self == Doe31_0Dio0::Doe31_0Dio0Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio0_enable(&self) -> bool {
        *self == Doe31_0Dio0::Doe31_0Dio0Enable
    }
}
#[doc = "Field `DOE31_0_DIO0` writer - Enables data output for DIO0."]
pub type Doe31_0Dio0W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio0>;
impl<'a, REG> Doe31_0Dio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio0_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio0::Doe31_0Dio0Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio0_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio0::Doe31_0Dio0Enable)
    }
}
#[doc = "Enables data output for DIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio1 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio1Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio1Enable = 1,
}
impl From<Doe31_0Dio1> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO1` reader - Enables data output for DIO1."]
pub type Doe31_0Dio1R = crate::BitReader<Doe31_0Dio1>;
impl Doe31_0Dio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio1 {
        match self.bits {
            false => Doe31_0Dio1::Doe31_0Dio1Disable,
            true => Doe31_0Dio1::Doe31_0Dio1Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio1_disable(&self) -> bool {
        *self == Doe31_0Dio1::Doe31_0Dio1Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio1_enable(&self) -> bool {
        *self == Doe31_0Dio1::Doe31_0Dio1Enable
    }
}
#[doc = "Field `DOE31_0_DIO1` writer - Enables data output for DIO1."]
pub type Doe31_0Dio1W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio1>;
impl<'a, REG> Doe31_0Dio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio1_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio1::Doe31_0Dio1Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio1_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio1::Doe31_0Dio1Enable)
    }
}
#[doc = "Enables data output for DIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio2 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio2Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio2Enable = 1,
}
impl From<Doe31_0Dio2> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO2` reader - Enables data output for DIO2."]
pub type Doe31_0Dio2R = crate::BitReader<Doe31_0Dio2>;
impl Doe31_0Dio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio2 {
        match self.bits {
            false => Doe31_0Dio2::Doe31_0Dio2Disable,
            true => Doe31_0Dio2::Doe31_0Dio2Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio2_disable(&self) -> bool {
        *self == Doe31_0Dio2::Doe31_0Dio2Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio2_enable(&self) -> bool {
        *self == Doe31_0Dio2::Doe31_0Dio2Enable
    }
}
#[doc = "Field `DOE31_0_DIO2` writer - Enables data output for DIO2."]
pub type Doe31_0Dio2W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio2>;
impl<'a, REG> Doe31_0Dio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio2_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio2::Doe31_0Dio2Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio2_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio2::Doe31_0Dio2Enable)
    }
}
#[doc = "Enables data output for DIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio3 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio3Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio3Enable = 1,
}
impl From<Doe31_0Dio3> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO3` reader - Enables data output for DIO3."]
pub type Doe31_0Dio3R = crate::BitReader<Doe31_0Dio3>;
impl Doe31_0Dio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio3 {
        match self.bits {
            false => Doe31_0Dio3::Doe31_0Dio3Disable,
            true => Doe31_0Dio3::Doe31_0Dio3Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio3_disable(&self) -> bool {
        *self == Doe31_0Dio3::Doe31_0Dio3Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio3_enable(&self) -> bool {
        *self == Doe31_0Dio3::Doe31_0Dio3Enable
    }
}
#[doc = "Field `DOE31_0_DIO3` writer - Enables data output for DIO3."]
pub type Doe31_0Dio3W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio3>;
impl<'a, REG> Doe31_0Dio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio3_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio3::Doe31_0Dio3Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio3_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio3::Doe31_0Dio3Enable)
    }
}
#[doc = "Enables data output for DIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio4 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio4Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio4Enable = 1,
}
impl From<Doe31_0Dio4> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO4` reader - Enables data output for DIO4."]
pub type Doe31_0Dio4R = crate::BitReader<Doe31_0Dio4>;
impl Doe31_0Dio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio4 {
        match self.bits {
            false => Doe31_0Dio4::Doe31_0Dio4Disable,
            true => Doe31_0Dio4::Doe31_0Dio4Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio4_disable(&self) -> bool {
        *self == Doe31_0Dio4::Doe31_0Dio4Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio4_enable(&self) -> bool {
        *self == Doe31_0Dio4::Doe31_0Dio4Enable
    }
}
#[doc = "Field `DOE31_0_DIO4` writer - Enables data output for DIO4."]
pub type Doe31_0Dio4W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio4>;
impl<'a, REG> Doe31_0Dio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio4_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio4::Doe31_0Dio4Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio4_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio4::Doe31_0Dio4Enable)
    }
}
#[doc = "Enables data output for DIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio5 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio5Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio5Enable = 1,
}
impl From<Doe31_0Dio5> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO5` reader - Enables data output for DIO5."]
pub type Doe31_0Dio5R = crate::BitReader<Doe31_0Dio5>;
impl Doe31_0Dio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio5 {
        match self.bits {
            false => Doe31_0Dio5::Doe31_0Dio5Disable,
            true => Doe31_0Dio5::Doe31_0Dio5Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio5_disable(&self) -> bool {
        *self == Doe31_0Dio5::Doe31_0Dio5Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio5_enable(&self) -> bool {
        *self == Doe31_0Dio5::Doe31_0Dio5Enable
    }
}
#[doc = "Field `DOE31_0_DIO5` writer - Enables data output for DIO5."]
pub type Doe31_0Dio5W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio5>;
impl<'a, REG> Doe31_0Dio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio5_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio5::Doe31_0Dio5Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio5_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio5::Doe31_0Dio5Enable)
    }
}
#[doc = "Enables data output for DIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio6 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio6Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio6Enable = 1,
}
impl From<Doe31_0Dio6> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO6` reader - Enables data output for DIO6."]
pub type Doe31_0Dio6R = crate::BitReader<Doe31_0Dio6>;
impl Doe31_0Dio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio6 {
        match self.bits {
            false => Doe31_0Dio6::Doe31_0Dio6Disable,
            true => Doe31_0Dio6::Doe31_0Dio6Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio6_disable(&self) -> bool {
        *self == Doe31_0Dio6::Doe31_0Dio6Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio6_enable(&self) -> bool {
        *self == Doe31_0Dio6::Doe31_0Dio6Enable
    }
}
#[doc = "Field `DOE31_0_DIO6` writer - Enables data output for DIO6."]
pub type Doe31_0Dio6W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio6>;
impl<'a, REG> Doe31_0Dio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio6_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio6::Doe31_0Dio6Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio6_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio6::Doe31_0Dio6Enable)
    }
}
#[doc = "Enables data output for DIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio7 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio7Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio7Enable = 1,
}
impl From<Doe31_0Dio7> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO7` reader - Enables data output for DIO7."]
pub type Doe31_0Dio7R = crate::BitReader<Doe31_0Dio7>;
impl Doe31_0Dio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio7 {
        match self.bits {
            false => Doe31_0Dio7::Doe31_0Dio7Disable,
            true => Doe31_0Dio7::Doe31_0Dio7Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio7_disable(&self) -> bool {
        *self == Doe31_0Dio7::Doe31_0Dio7Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio7_enable(&self) -> bool {
        *self == Doe31_0Dio7::Doe31_0Dio7Enable
    }
}
#[doc = "Field `DOE31_0_DIO7` writer - Enables data output for DIO7."]
pub type Doe31_0Dio7W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio7>;
impl<'a, REG> Doe31_0Dio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio7_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio7::Doe31_0Dio7Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio7_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio7::Doe31_0Dio7Enable)
    }
}
#[doc = "Enables data output for DIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio8 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio8Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio8Enable = 1,
}
impl From<Doe31_0Dio8> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO8` reader - Enables data output for DIO8."]
pub type Doe31_0Dio8R = crate::BitReader<Doe31_0Dio8>;
impl Doe31_0Dio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio8 {
        match self.bits {
            false => Doe31_0Dio8::Doe31_0Dio8Disable,
            true => Doe31_0Dio8::Doe31_0Dio8Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio8_disable(&self) -> bool {
        *self == Doe31_0Dio8::Doe31_0Dio8Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio8_enable(&self) -> bool {
        *self == Doe31_0Dio8::Doe31_0Dio8Enable
    }
}
#[doc = "Field `DOE31_0_DIO8` writer - Enables data output for DIO8."]
pub type Doe31_0Dio8W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio8>;
impl<'a, REG> Doe31_0Dio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio8_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio8::Doe31_0Dio8Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio8_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio8::Doe31_0Dio8Enable)
    }
}
#[doc = "Enables data output for DIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio9 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio9Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio9Enable = 1,
}
impl From<Doe31_0Dio9> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO9` reader - Enables data output for DIO9."]
pub type Doe31_0Dio9R = crate::BitReader<Doe31_0Dio9>;
impl Doe31_0Dio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio9 {
        match self.bits {
            false => Doe31_0Dio9::Doe31_0Dio9Disable,
            true => Doe31_0Dio9::Doe31_0Dio9Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio9_disable(&self) -> bool {
        *self == Doe31_0Dio9::Doe31_0Dio9Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio9_enable(&self) -> bool {
        *self == Doe31_0Dio9::Doe31_0Dio9Enable
    }
}
#[doc = "Field `DOE31_0_DIO9` writer - Enables data output for DIO9."]
pub type Doe31_0Dio9W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio9>;
impl<'a, REG> Doe31_0Dio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio9_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio9::Doe31_0Dio9Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio9_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio9::Doe31_0Dio9Enable)
    }
}
#[doc = "Enables data output for DIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio10 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio10Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio10Enable = 1,
}
impl From<Doe31_0Dio10> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO10` reader - Enables data output for DIO10."]
pub type Doe31_0Dio10R = crate::BitReader<Doe31_0Dio10>;
impl Doe31_0Dio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio10 {
        match self.bits {
            false => Doe31_0Dio10::Doe31_0Dio10Disable,
            true => Doe31_0Dio10::Doe31_0Dio10Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio10_disable(&self) -> bool {
        *self == Doe31_0Dio10::Doe31_0Dio10Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio10_enable(&self) -> bool {
        *self == Doe31_0Dio10::Doe31_0Dio10Enable
    }
}
#[doc = "Field `DOE31_0_DIO10` writer - Enables data output for DIO10."]
pub type Doe31_0Dio10W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio10>;
impl<'a, REG> Doe31_0Dio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio10_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio10::Doe31_0Dio10Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio10_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio10::Doe31_0Dio10Enable)
    }
}
#[doc = "Enables data output for DIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio11 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio11Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio11Enable = 1,
}
impl From<Doe31_0Dio11> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO11` reader - Enables data output for DIO11."]
pub type Doe31_0Dio11R = crate::BitReader<Doe31_0Dio11>;
impl Doe31_0Dio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio11 {
        match self.bits {
            false => Doe31_0Dio11::Doe31_0Dio11Disable,
            true => Doe31_0Dio11::Doe31_0Dio11Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio11_disable(&self) -> bool {
        *self == Doe31_0Dio11::Doe31_0Dio11Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio11_enable(&self) -> bool {
        *self == Doe31_0Dio11::Doe31_0Dio11Enable
    }
}
#[doc = "Field `DOE31_0_DIO11` writer - Enables data output for DIO11."]
pub type Doe31_0Dio11W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio11>;
impl<'a, REG> Doe31_0Dio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio11_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio11::Doe31_0Dio11Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio11_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio11::Doe31_0Dio11Enable)
    }
}
#[doc = "Enables data output for DIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio12 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio12Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio12Enable = 1,
}
impl From<Doe31_0Dio12> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO12` reader - Enables data output for DIO12."]
pub type Doe31_0Dio12R = crate::BitReader<Doe31_0Dio12>;
impl Doe31_0Dio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio12 {
        match self.bits {
            false => Doe31_0Dio12::Doe31_0Dio12Disable,
            true => Doe31_0Dio12::Doe31_0Dio12Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio12_disable(&self) -> bool {
        *self == Doe31_0Dio12::Doe31_0Dio12Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio12_enable(&self) -> bool {
        *self == Doe31_0Dio12::Doe31_0Dio12Enable
    }
}
#[doc = "Field `DOE31_0_DIO12` writer - Enables data output for DIO12."]
pub type Doe31_0Dio12W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio12>;
impl<'a, REG> Doe31_0Dio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio12_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio12::Doe31_0Dio12Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio12_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio12::Doe31_0Dio12Enable)
    }
}
#[doc = "Enables data output for DIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio13 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio13Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio13Enable = 1,
}
impl From<Doe31_0Dio13> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO13` reader - Enables data output for DIO13."]
pub type Doe31_0Dio13R = crate::BitReader<Doe31_0Dio13>;
impl Doe31_0Dio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio13 {
        match self.bits {
            false => Doe31_0Dio13::Doe31_0Dio13Disable,
            true => Doe31_0Dio13::Doe31_0Dio13Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio13_disable(&self) -> bool {
        *self == Doe31_0Dio13::Doe31_0Dio13Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio13_enable(&self) -> bool {
        *self == Doe31_0Dio13::Doe31_0Dio13Enable
    }
}
#[doc = "Field `DOE31_0_DIO13` writer - Enables data output for DIO13."]
pub type Doe31_0Dio13W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio13>;
impl<'a, REG> Doe31_0Dio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio13_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio13::Doe31_0Dio13Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio13_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio13::Doe31_0Dio13Enable)
    }
}
#[doc = "Enables data output for DIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio14 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio14Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio14Enable = 1,
}
impl From<Doe31_0Dio14> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO14` reader - Enables data output for DIO14."]
pub type Doe31_0Dio14R = crate::BitReader<Doe31_0Dio14>;
impl Doe31_0Dio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio14 {
        match self.bits {
            false => Doe31_0Dio14::Doe31_0Dio14Disable,
            true => Doe31_0Dio14::Doe31_0Dio14Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio14_disable(&self) -> bool {
        *self == Doe31_0Dio14::Doe31_0Dio14Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio14_enable(&self) -> bool {
        *self == Doe31_0Dio14::Doe31_0Dio14Enable
    }
}
#[doc = "Field `DOE31_0_DIO14` writer - Enables data output for DIO14."]
pub type Doe31_0Dio14W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio14>;
impl<'a, REG> Doe31_0Dio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio14_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio14::Doe31_0Dio14Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio14_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio14::Doe31_0Dio14Enable)
    }
}
#[doc = "Enables data output for DIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio15 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio15Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio15Enable = 1,
}
impl From<Doe31_0Dio15> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO15` reader - Enables data output for DIO15."]
pub type Doe31_0Dio15R = crate::BitReader<Doe31_0Dio15>;
impl Doe31_0Dio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio15 {
        match self.bits {
            false => Doe31_0Dio15::Doe31_0Dio15Disable,
            true => Doe31_0Dio15::Doe31_0Dio15Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio15_disable(&self) -> bool {
        *self == Doe31_0Dio15::Doe31_0Dio15Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio15_enable(&self) -> bool {
        *self == Doe31_0Dio15::Doe31_0Dio15Enable
    }
}
#[doc = "Field `DOE31_0_DIO15` writer - Enables data output for DIO15."]
pub type Doe31_0Dio15W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio15>;
impl<'a, REG> Doe31_0Dio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio15_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio15::Doe31_0Dio15Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio15_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio15::Doe31_0Dio15Enable)
    }
}
#[doc = "Enables data output for DIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio16 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio16Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio16Enable = 1,
}
impl From<Doe31_0Dio16> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO16` reader - Enables data output for DIO16."]
pub type Doe31_0Dio16R = crate::BitReader<Doe31_0Dio16>;
impl Doe31_0Dio16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio16 {
        match self.bits {
            false => Doe31_0Dio16::Doe31_0Dio16Disable,
            true => Doe31_0Dio16::Doe31_0Dio16Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio16_disable(&self) -> bool {
        *self == Doe31_0Dio16::Doe31_0Dio16Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio16_enable(&self) -> bool {
        *self == Doe31_0Dio16::Doe31_0Dio16Enable
    }
}
#[doc = "Field `DOE31_0_DIO16` writer - Enables data output for DIO16."]
pub type Doe31_0Dio16W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio16>;
impl<'a, REG> Doe31_0Dio16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio16_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio16::Doe31_0Dio16Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio16_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio16::Doe31_0Dio16Enable)
    }
}
#[doc = "Enables data output for DIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio17 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio17Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio17Enable = 1,
}
impl From<Doe31_0Dio17> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO17` reader - Enables data output for DIO17."]
pub type Doe31_0Dio17R = crate::BitReader<Doe31_0Dio17>;
impl Doe31_0Dio17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio17 {
        match self.bits {
            false => Doe31_0Dio17::Doe31_0Dio17Disable,
            true => Doe31_0Dio17::Doe31_0Dio17Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio17_disable(&self) -> bool {
        *self == Doe31_0Dio17::Doe31_0Dio17Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio17_enable(&self) -> bool {
        *self == Doe31_0Dio17::Doe31_0Dio17Enable
    }
}
#[doc = "Field `DOE31_0_DIO17` writer - Enables data output for DIO17."]
pub type Doe31_0Dio17W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio17>;
impl<'a, REG> Doe31_0Dio17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio17_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio17::Doe31_0Dio17Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio17_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio17::Doe31_0Dio17Enable)
    }
}
#[doc = "Enables data output for DIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio18 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio18Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio18Enable = 1,
}
impl From<Doe31_0Dio18> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO18` reader - Enables data output for DIO18."]
pub type Doe31_0Dio18R = crate::BitReader<Doe31_0Dio18>;
impl Doe31_0Dio18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio18 {
        match self.bits {
            false => Doe31_0Dio18::Doe31_0Dio18Disable,
            true => Doe31_0Dio18::Doe31_0Dio18Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio18_disable(&self) -> bool {
        *self == Doe31_0Dio18::Doe31_0Dio18Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio18_enable(&self) -> bool {
        *self == Doe31_0Dio18::Doe31_0Dio18Enable
    }
}
#[doc = "Field `DOE31_0_DIO18` writer - Enables data output for DIO18."]
pub type Doe31_0Dio18W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio18>;
impl<'a, REG> Doe31_0Dio18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio18_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio18::Doe31_0Dio18Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio18_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio18::Doe31_0Dio18Enable)
    }
}
#[doc = "Enables data output for DIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio19 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio19Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio19Enable = 1,
}
impl From<Doe31_0Dio19> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO19` reader - Enables data output for DIO19."]
pub type Doe31_0Dio19R = crate::BitReader<Doe31_0Dio19>;
impl Doe31_0Dio19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio19 {
        match self.bits {
            false => Doe31_0Dio19::Doe31_0Dio19Disable,
            true => Doe31_0Dio19::Doe31_0Dio19Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio19_disable(&self) -> bool {
        *self == Doe31_0Dio19::Doe31_0Dio19Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio19_enable(&self) -> bool {
        *self == Doe31_0Dio19::Doe31_0Dio19Enable
    }
}
#[doc = "Field `DOE31_0_DIO19` writer - Enables data output for DIO19."]
pub type Doe31_0Dio19W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio19>;
impl<'a, REG> Doe31_0Dio19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio19_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio19::Doe31_0Dio19Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio19_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio19::Doe31_0Dio19Enable)
    }
}
#[doc = "Enables data output for DIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio20 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio20Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio20Enable = 1,
}
impl From<Doe31_0Dio20> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO20` reader - Enables data output for DIO20."]
pub type Doe31_0Dio20R = crate::BitReader<Doe31_0Dio20>;
impl Doe31_0Dio20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio20 {
        match self.bits {
            false => Doe31_0Dio20::Doe31_0Dio20Disable,
            true => Doe31_0Dio20::Doe31_0Dio20Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio20_disable(&self) -> bool {
        *self == Doe31_0Dio20::Doe31_0Dio20Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio20_enable(&self) -> bool {
        *self == Doe31_0Dio20::Doe31_0Dio20Enable
    }
}
#[doc = "Field `DOE31_0_DIO20` writer - Enables data output for DIO20."]
pub type Doe31_0Dio20W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio20>;
impl<'a, REG> Doe31_0Dio20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio20_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio20::Doe31_0Dio20Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio20_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio20::Doe31_0Dio20Enable)
    }
}
#[doc = "Enables data output for DIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio21 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio21Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio21Enable = 1,
}
impl From<Doe31_0Dio21> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO21` reader - Enables data output for DIO21."]
pub type Doe31_0Dio21R = crate::BitReader<Doe31_0Dio21>;
impl Doe31_0Dio21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio21 {
        match self.bits {
            false => Doe31_0Dio21::Doe31_0Dio21Disable,
            true => Doe31_0Dio21::Doe31_0Dio21Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio21_disable(&self) -> bool {
        *self == Doe31_0Dio21::Doe31_0Dio21Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio21_enable(&self) -> bool {
        *self == Doe31_0Dio21::Doe31_0Dio21Enable
    }
}
#[doc = "Field `DOE31_0_DIO21` writer - Enables data output for DIO21."]
pub type Doe31_0Dio21W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio21>;
impl<'a, REG> Doe31_0Dio21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio21_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio21::Doe31_0Dio21Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio21_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio21::Doe31_0Dio21Enable)
    }
}
#[doc = "Enables data output for DIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio22 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio22Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio22Enable = 1,
}
impl From<Doe31_0Dio22> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO22` reader - Enables data output for DIO22."]
pub type Doe31_0Dio22R = crate::BitReader<Doe31_0Dio22>;
impl Doe31_0Dio22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio22 {
        match self.bits {
            false => Doe31_0Dio22::Doe31_0Dio22Disable,
            true => Doe31_0Dio22::Doe31_0Dio22Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio22_disable(&self) -> bool {
        *self == Doe31_0Dio22::Doe31_0Dio22Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio22_enable(&self) -> bool {
        *self == Doe31_0Dio22::Doe31_0Dio22Enable
    }
}
#[doc = "Field `DOE31_0_DIO22` writer - Enables data output for DIO22."]
pub type Doe31_0Dio22W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio22>;
impl<'a, REG> Doe31_0Dio22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio22_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio22::Doe31_0Dio22Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio22_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio22::Doe31_0Dio22Enable)
    }
}
#[doc = "Enables data output for DIO23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio23 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio23Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio23Enable = 1,
}
impl From<Doe31_0Dio23> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO23` reader - Enables data output for DIO23."]
pub type Doe31_0Dio23R = crate::BitReader<Doe31_0Dio23>;
impl Doe31_0Dio23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio23 {
        match self.bits {
            false => Doe31_0Dio23::Doe31_0Dio23Disable,
            true => Doe31_0Dio23::Doe31_0Dio23Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio23_disable(&self) -> bool {
        *self == Doe31_0Dio23::Doe31_0Dio23Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio23_enable(&self) -> bool {
        *self == Doe31_0Dio23::Doe31_0Dio23Enable
    }
}
#[doc = "Field `DOE31_0_DIO23` writer - Enables data output for DIO23."]
pub type Doe31_0Dio23W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio23>;
impl<'a, REG> Doe31_0Dio23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio23_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio23::Doe31_0Dio23Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio23_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio23::Doe31_0Dio23Enable)
    }
}
#[doc = "Enables data output for DIO24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio24 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio24Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio24Enable = 1,
}
impl From<Doe31_0Dio24> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO24` reader - Enables data output for DIO24."]
pub type Doe31_0Dio24R = crate::BitReader<Doe31_0Dio24>;
impl Doe31_0Dio24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio24 {
        match self.bits {
            false => Doe31_0Dio24::Doe31_0Dio24Disable,
            true => Doe31_0Dio24::Doe31_0Dio24Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio24_disable(&self) -> bool {
        *self == Doe31_0Dio24::Doe31_0Dio24Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio24_enable(&self) -> bool {
        *self == Doe31_0Dio24::Doe31_0Dio24Enable
    }
}
#[doc = "Field `DOE31_0_DIO24` writer - Enables data output for DIO24."]
pub type Doe31_0Dio24W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio24>;
impl<'a, REG> Doe31_0Dio24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio24_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio24::Doe31_0Dio24Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio24_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio24::Doe31_0Dio24Enable)
    }
}
#[doc = "Enables data output for DIO25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio25 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio25Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio25Enable = 1,
}
impl From<Doe31_0Dio25> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO25` reader - Enables data output for DIO25."]
pub type Doe31_0Dio25R = crate::BitReader<Doe31_0Dio25>;
impl Doe31_0Dio25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio25 {
        match self.bits {
            false => Doe31_0Dio25::Doe31_0Dio25Disable,
            true => Doe31_0Dio25::Doe31_0Dio25Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio25_disable(&self) -> bool {
        *self == Doe31_0Dio25::Doe31_0Dio25Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio25_enable(&self) -> bool {
        *self == Doe31_0Dio25::Doe31_0Dio25Enable
    }
}
#[doc = "Field `DOE31_0_DIO25` writer - Enables data output for DIO25."]
pub type Doe31_0Dio25W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio25>;
impl<'a, REG> Doe31_0Dio25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio25_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio25::Doe31_0Dio25Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio25_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio25::Doe31_0Dio25Enable)
    }
}
#[doc = "Enables data output for DIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio26 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio26Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio26Enable = 1,
}
impl From<Doe31_0Dio26> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO26` reader - Enables data output for DIO26."]
pub type Doe31_0Dio26R = crate::BitReader<Doe31_0Dio26>;
impl Doe31_0Dio26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio26 {
        match self.bits {
            false => Doe31_0Dio26::Doe31_0Dio26Disable,
            true => Doe31_0Dio26::Doe31_0Dio26Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio26_disable(&self) -> bool {
        *self == Doe31_0Dio26::Doe31_0Dio26Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio26_enable(&self) -> bool {
        *self == Doe31_0Dio26::Doe31_0Dio26Enable
    }
}
#[doc = "Field `DOE31_0_DIO26` writer - Enables data output for DIO26."]
pub type Doe31_0Dio26W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio26>;
impl<'a, REG> Doe31_0Dio26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio26_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio26::Doe31_0Dio26Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio26_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio26::Doe31_0Dio26Enable)
    }
}
#[doc = "Enables data output for DIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio27 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio27Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio27Enable = 1,
}
impl From<Doe31_0Dio27> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO27` reader - Enables data output for DIO27."]
pub type Doe31_0Dio27R = crate::BitReader<Doe31_0Dio27>;
impl Doe31_0Dio27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio27 {
        match self.bits {
            false => Doe31_0Dio27::Doe31_0Dio27Disable,
            true => Doe31_0Dio27::Doe31_0Dio27Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio27_disable(&self) -> bool {
        *self == Doe31_0Dio27::Doe31_0Dio27Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio27_enable(&self) -> bool {
        *self == Doe31_0Dio27::Doe31_0Dio27Enable
    }
}
#[doc = "Field `DOE31_0_DIO27` writer - Enables data output for DIO27."]
pub type Doe31_0Dio27W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio27>;
impl<'a, REG> Doe31_0Dio27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio27_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio27::Doe31_0Dio27Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio27_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio27::Doe31_0Dio27Enable)
    }
}
#[doc = "Enables data output for DIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio28 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio28Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio28Enable = 1,
}
impl From<Doe31_0Dio28> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO28` reader - Enables data output for DIO28."]
pub type Doe31_0Dio28R = crate::BitReader<Doe31_0Dio28>;
impl Doe31_0Dio28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio28 {
        match self.bits {
            false => Doe31_0Dio28::Doe31_0Dio28Disable,
            true => Doe31_0Dio28::Doe31_0Dio28Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio28_disable(&self) -> bool {
        *self == Doe31_0Dio28::Doe31_0Dio28Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio28_enable(&self) -> bool {
        *self == Doe31_0Dio28::Doe31_0Dio28Enable
    }
}
#[doc = "Field `DOE31_0_DIO28` writer - Enables data output for DIO28."]
pub type Doe31_0Dio28W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio28>;
impl<'a, REG> Doe31_0Dio28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio28_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio28::Doe31_0Dio28Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio28_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio28::Doe31_0Dio28Enable)
    }
}
#[doc = "Enables data output for DIO29.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio29 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio29Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio29Enable = 1,
}
impl From<Doe31_0Dio29> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO29` reader - Enables data output for DIO29."]
pub type Doe31_0Dio29R = crate::BitReader<Doe31_0Dio29>;
impl Doe31_0Dio29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio29 {
        match self.bits {
            false => Doe31_0Dio29::Doe31_0Dio29Disable,
            true => Doe31_0Dio29::Doe31_0Dio29Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio29_disable(&self) -> bool {
        *self == Doe31_0Dio29::Doe31_0Dio29Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio29_enable(&self) -> bool {
        *self == Doe31_0Dio29::Doe31_0Dio29Enable
    }
}
#[doc = "Field `DOE31_0_DIO29` writer - Enables data output for DIO29."]
pub type Doe31_0Dio29W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio29>;
impl<'a, REG> Doe31_0Dio29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio29_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio29::Doe31_0Dio29Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio29_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio29::Doe31_0Dio29Enable)
    }
}
#[doc = "Enables data output for DIO30.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio30 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio30Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio30Enable = 1,
}
impl From<Doe31_0Dio30> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO30` reader - Enables data output for DIO30."]
pub type Doe31_0Dio30R = crate::BitReader<Doe31_0Dio30>;
impl Doe31_0Dio30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio30 {
        match self.bits {
            false => Doe31_0Dio30::Doe31_0Dio30Disable,
            true => Doe31_0Dio30::Doe31_0Dio30Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio30_disable(&self) -> bool {
        *self == Doe31_0Dio30::Doe31_0Dio30Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio30_enable(&self) -> bool {
        *self == Doe31_0Dio30::Doe31_0Dio30Enable
    }
}
#[doc = "Field `DOE31_0_DIO30` writer - Enables data output for DIO30."]
pub type Doe31_0Dio30W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio30>;
impl<'a, REG> Doe31_0Dio30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio30_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio30::Doe31_0Dio30Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio30_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio30::Doe31_0Dio30Enable)
    }
}
#[doc = "Enables data output for DIO31.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Doe31_0Dio31 {
    #[doc = "0: DISABLE"]
    Doe31_0Dio31Disable = 0,
    #[doc = "1: ENABLE"]
    Doe31_0Dio31Enable = 1,
}
impl From<Doe31_0Dio31> for bool {
    #[inline(always)]
    fn from(variant: Doe31_0Dio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOE31_0_DIO31` reader - Enables data output for DIO31."]
pub type Doe31_0Dio31R = crate::BitReader<Doe31_0Dio31>;
impl Doe31_0Dio31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Doe31_0Dio31 {
        match self.bits {
            false => Doe31_0Dio31::Doe31_0Dio31Disable,
            true => Doe31_0Dio31::Doe31_0Dio31Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio31_disable(&self) -> bool {
        *self == Doe31_0Dio31::Doe31_0Dio31Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_doe31_0_dio31_enable(&self) -> bool {
        *self == Doe31_0Dio31::Doe31_0Dio31Enable
    }
}
#[doc = "Field `DOE31_0_DIO31` writer - Enables data output for DIO31."]
pub type Doe31_0Dio31W<'a, REG> = crate::BitWriter<'a, REG, Doe31_0Dio31>;
impl<'a, REG> Doe31_0Dio31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn doe31_0_dio31_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio31::Doe31_0Dio31Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn doe31_0_dio31_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Doe31_0Dio31::Doe31_0Dio31Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enables data output for DIO0."]
    #[inline(always)]
    pub fn doe31_0_dio0(&self) -> Doe31_0Dio0R {
        Doe31_0Dio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables data output for DIO1."]
    #[inline(always)]
    pub fn doe31_0_dio1(&self) -> Doe31_0Dio1R {
        Doe31_0Dio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables data output for DIO2."]
    #[inline(always)]
    pub fn doe31_0_dio2(&self) -> Doe31_0Dio2R {
        Doe31_0Dio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables data output for DIO3."]
    #[inline(always)]
    pub fn doe31_0_dio3(&self) -> Doe31_0Dio3R {
        Doe31_0Dio3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables data output for DIO4."]
    #[inline(always)]
    pub fn doe31_0_dio4(&self) -> Doe31_0Dio4R {
        Doe31_0Dio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables data output for DIO5."]
    #[inline(always)]
    pub fn doe31_0_dio5(&self) -> Doe31_0Dio5R {
        Doe31_0Dio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables data output for DIO6."]
    #[inline(always)]
    pub fn doe31_0_dio6(&self) -> Doe31_0Dio6R {
        Doe31_0Dio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables data output for DIO7."]
    #[inline(always)]
    pub fn doe31_0_dio7(&self) -> Doe31_0Dio7R {
        Doe31_0Dio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables data output for DIO8."]
    #[inline(always)]
    pub fn doe31_0_dio8(&self) -> Doe31_0Dio8R {
        Doe31_0Dio8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables data output for DIO9."]
    #[inline(always)]
    pub fn doe31_0_dio9(&self) -> Doe31_0Dio9R {
        Doe31_0Dio9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enables data output for DIO10."]
    #[inline(always)]
    pub fn doe31_0_dio10(&self) -> Doe31_0Dio10R {
        Doe31_0Dio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables data output for DIO11."]
    #[inline(always)]
    pub fn doe31_0_dio11(&self) -> Doe31_0Dio11R {
        Doe31_0Dio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enables data output for DIO12."]
    #[inline(always)]
    pub fn doe31_0_dio12(&self) -> Doe31_0Dio12R {
        Doe31_0Dio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables data output for DIO13."]
    #[inline(always)]
    pub fn doe31_0_dio13(&self) -> Doe31_0Dio13R {
        Doe31_0Dio13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enables data output for DIO14."]
    #[inline(always)]
    pub fn doe31_0_dio14(&self) -> Doe31_0Dio14R {
        Doe31_0Dio14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enables data output for DIO15."]
    #[inline(always)]
    pub fn doe31_0_dio15(&self) -> Doe31_0Dio15R {
        Doe31_0Dio15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables data output for DIO16."]
    #[inline(always)]
    pub fn doe31_0_dio16(&self) -> Doe31_0Dio16R {
        Doe31_0Dio16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enables data output for DIO17."]
    #[inline(always)]
    pub fn doe31_0_dio17(&self) -> Doe31_0Dio17R {
        Doe31_0Dio17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables data output for DIO18."]
    #[inline(always)]
    pub fn doe31_0_dio18(&self) -> Doe31_0Dio18R {
        Doe31_0Dio18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enables data output for DIO19."]
    #[inline(always)]
    pub fn doe31_0_dio19(&self) -> Doe31_0Dio19R {
        Doe31_0Dio19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enables data output for DIO20."]
    #[inline(always)]
    pub fn doe31_0_dio20(&self) -> Doe31_0Dio20R {
        Doe31_0Dio20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enables data output for DIO21."]
    #[inline(always)]
    pub fn doe31_0_dio21(&self) -> Doe31_0Dio21R {
        Doe31_0Dio21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enables data output for DIO22."]
    #[inline(always)]
    pub fn doe31_0_dio22(&self) -> Doe31_0Dio22R {
        Doe31_0Dio22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enables data output for DIO23."]
    #[inline(always)]
    pub fn doe31_0_dio23(&self) -> Doe31_0Dio23R {
        Doe31_0Dio23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enables data output for DIO24."]
    #[inline(always)]
    pub fn doe31_0_dio24(&self) -> Doe31_0Dio24R {
        Doe31_0Dio24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enables data output for DIO25."]
    #[inline(always)]
    pub fn doe31_0_dio25(&self) -> Doe31_0Dio25R {
        Doe31_0Dio25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enables data output for DIO26."]
    #[inline(always)]
    pub fn doe31_0_dio26(&self) -> Doe31_0Dio26R {
        Doe31_0Dio26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enables data output for DIO27."]
    #[inline(always)]
    pub fn doe31_0_dio27(&self) -> Doe31_0Dio27R {
        Doe31_0Dio27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enables data output for DIO28."]
    #[inline(always)]
    pub fn doe31_0_dio28(&self) -> Doe31_0Dio28R {
        Doe31_0Dio28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enables data output for DIO29."]
    #[inline(always)]
    pub fn doe31_0_dio29(&self) -> Doe31_0Dio29R {
        Doe31_0Dio29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enables data output for DIO30."]
    #[inline(always)]
    pub fn doe31_0_dio30(&self) -> Doe31_0Dio30R {
        Doe31_0Dio30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enables data output for DIO31."]
    #[inline(always)]
    pub fn doe31_0_dio31(&self) -> Doe31_0Dio31R {
        Doe31_0Dio31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables data output for DIO0."]
    #[inline(always)]
    pub fn doe31_0_dio0(&mut self) -> Doe31_0Dio0W<Doe31_0Spec> {
        Doe31_0Dio0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enables data output for DIO1."]
    #[inline(always)]
    pub fn doe31_0_dio1(&mut self) -> Doe31_0Dio1W<Doe31_0Spec> {
        Doe31_0Dio1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enables data output for DIO2."]
    #[inline(always)]
    pub fn doe31_0_dio2(&mut self) -> Doe31_0Dio2W<Doe31_0Spec> {
        Doe31_0Dio2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enables data output for DIO3."]
    #[inline(always)]
    pub fn doe31_0_dio3(&mut self) -> Doe31_0Dio3W<Doe31_0Spec> {
        Doe31_0Dio3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enables data output for DIO4."]
    #[inline(always)]
    pub fn doe31_0_dio4(&mut self) -> Doe31_0Dio4W<Doe31_0Spec> {
        Doe31_0Dio4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enables data output for DIO5."]
    #[inline(always)]
    pub fn doe31_0_dio5(&mut self) -> Doe31_0Dio5W<Doe31_0Spec> {
        Doe31_0Dio5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enables data output for DIO6."]
    #[inline(always)]
    pub fn doe31_0_dio6(&mut self) -> Doe31_0Dio6W<Doe31_0Spec> {
        Doe31_0Dio6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enables data output for DIO7."]
    #[inline(always)]
    pub fn doe31_0_dio7(&mut self) -> Doe31_0Dio7W<Doe31_0Spec> {
        Doe31_0Dio7W::new(self, 7)
    }
    #[doc = "Bit 8 - Enables data output for DIO8."]
    #[inline(always)]
    pub fn doe31_0_dio8(&mut self) -> Doe31_0Dio8W<Doe31_0Spec> {
        Doe31_0Dio8W::new(self, 8)
    }
    #[doc = "Bit 9 - Enables data output for DIO9."]
    #[inline(always)]
    pub fn doe31_0_dio9(&mut self) -> Doe31_0Dio9W<Doe31_0Spec> {
        Doe31_0Dio9W::new(self, 9)
    }
    #[doc = "Bit 10 - Enables data output for DIO10."]
    #[inline(always)]
    pub fn doe31_0_dio10(&mut self) -> Doe31_0Dio10W<Doe31_0Spec> {
        Doe31_0Dio10W::new(self, 10)
    }
    #[doc = "Bit 11 - Enables data output for DIO11."]
    #[inline(always)]
    pub fn doe31_0_dio11(&mut self) -> Doe31_0Dio11W<Doe31_0Spec> {
        Doe31_0Dio11W::new(self, 11)
    }
    #[doc = "Bit 12 - Enables data output for DIO12."]
    #[inline(always)]
    pub fn doe31_0_dio12(&mut self) -> Doe31_0Dio12W<Doe31_0Spec> {
        Doe31_0Dio12W::new(self, 12)
    }
    #[doc = "Bit 13 - Enables data output for DIO13."]
    #[inline(always)]
    pub fn doe31_0_dio13(&mut self) -> Doe31_0Dio13W<Doe31_0Spec> {
        Doe31_0Dio13W::new(self, 13)
    }
    #[doc = "Bit 14 - Enables data output for DIO14."]
    #[inline(always)]
    pub fn doe31_0_dio14(&mut self) -> Doe31_0Dio14W<Doe31_0Spec> {
        Doe31_0Dio14W::new(self, 14)
    }
    #[doc = "Bit 15 - Enables data output for DIO15."]
    #[inline(always)]
    pub fn doe31_0_dio15(&mut self) -> Doe31_0Dio15W<Doe31_0Spec> {
        Doe31_0Dio15W::new(self, 15)
    }
    #[doc = "Bit 16 - Enables data output for DIO16."]
    #[inline(always)]
    pub fn doe31_0_dio16(&mut self) -> Doe31_0Dio16W<Doe31_0Spec> {
        Doe31_0Dio16W::new(self, 16)
    }
    #[doc = "Bit 17 - Enables data output for DIO17."]
    #[inline(always)]
    pub fn doe31_0_dio17(&mut self) -> Doe31_0Dio17W<Doe31_0Spec> {
        Doe31_0Dio17W::new(self, 17)
    }
    #[doc = "Bit 18 - Enables data output for DIO18."]
    #[inline(always)]
    pub fn doe31_0_dio18(&mut self) -> Doe31_0Dio18W<Doe31_0Spec> {
        Doe31_0Dio18W::new(self, 18)
    }
    #[doc = "Bit 19 - Enables data output for DIO19."]
    #[inline(always)]
    pub fn doe31_0_dio19(&mut self) -> Doe31_0Dio19W<Doe31_0Spec> {
        Doe31_0Dio19W::new(self, 19)
    }
    #[doc = "Bit 20 - Enables data output for DIO20."]
    #[inline(always)]
    pub fn doe31_0_dio20(&mut self) -> Doe31_0Dio20W<Doe31_0Spec> {
        Doe31_0Dio20W::new(self, 20)
    }
    #[doc = "Bit 21 - Enables data output for DIO21."]
    #[inline(always)]
    pub fn doe31_0_dio21(&mut self) -> Doe31_0Dio21W<Doe31_0Spec> {
        Doe31_0Dio21W::new(self, 21)
    }
    #[doc = "Bit 22 - Enables data output for DIO22."]
    #[inline(always)]
    pub fn doe31_0_dio22(&mut self) -> Doe31_0Dio22W<Doe31_0Spec> {
        Doe31_0Dio22W::new(self, 22)
    }
    #[doc = "Bit 23 - Enables data output for DIO23."]
    #[inline(always)]
    pub fn doe31_0_dio23(&mut self) -> Doe31_0Dio23W<Doe31_0Spec> {
        Doe31_0Dio23W::new(self, 23)
    }
    #[doc = "Bit 24 - Enables data output for DIO24."]
    #[inline(always)]
    pub fn doe31_0_dio24(&mut self) -> Doe31_0Dio24W<Doe31_0Spec> {
        Doe31_0Dio24W::new(self, 24)
    }
    #[doc = "Bit 25 - Enables data output for DIO25."]
    #[inline(always)]
    pub fn doe31_0_dio25(&mut self) -> Doe31_0Dio25W<Doe31_0Spec> {
        Doe31_0Dio25W::new(self, 25)
    }
    #[doc = "Bit 26 - Enables data output for DIO26."]
    #[inline(always)]
    pub fn doe31_0_dio26(&mut self) -> Doe31_0Dio26W<Doe31_0Spec> {
        Doe31_0Dio26W::new(self, 26)
    }
    #[doc = "Bit 27 - Enables data output for DIO27."]
    #[inline(always)]
    pub fn doe31_0_dio27(&mut self) -> Doe31_0Dio27W<Doe31_0Spec> {
        Doe31_0Dio27W::new(self, 27)
    }
    #[doc = "Bit 28 - Enables data output for DIO28."]
    #[inline(always)]
    pub fn doe31_0_dio28(&mut self) -> Doe31_0Dio28W<Doe31_0Spec> {
        Doe31_0Dio28W::new(self, 28)
    }
    #[doc = "Bit 29 - Enables data output for DIO29."]
    #[inline(always)]
    pub fn doe31_0_dio29(&mut self) -> Doe31_0Dio29W<Doe31_0Spec> {
        Doe31_0Dio29W::new(self, 29)
    }
    #[doc = "Bit 30 - Enables data output for DIO30."]
    #[inline(always)]
    pub fn doe31_0_dio30(&mut self) -> Doe31_0Dio30W<Doe31_0Spec> {
        Doe31_0Dio30W::new(self, 30)
    }
    #[doc = "Bit 31 - Enables data output for DIO31."]
    #[inline(always)]
    pub fn doe31_0_dio31(&mut self) -> Doe31_0Dio31W<Doe31_0Spec> {
        Doe31_0Dio31W::new(self, 31)
    }
}
#[doc = "Data output enable 31 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`doe31_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doe31_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Doe31_0Spec;
impl crate::RegisterSpec for Doe31_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doe31_0::R`](R) reader structure"]
impl crate::Readable for Doe31_0Spec {}
#[doc = "`write(|w| ..)` method takes [`doe31_0::W`](W) writer structure"]
impl crate::Writable for Doe31_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DOE31_0 to value 0"]
impl crate::Resettable for Doe31_0Spec {
    const RESET_VALUE: u32 = 0;
}
