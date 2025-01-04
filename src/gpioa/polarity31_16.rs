#[doc = "Register `POLARITY31_16` reader"]
pub type R = crate::R<Polarity31_16Spec>;
#[doc = "Register `POLARITY31_16` writer"]
pub type W = crate::W<Polarity31_16Spec>;
#[doc = "Enables and configures edge detection polarity for DIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity31_16Dio16 {
    #[doc = "0: DISABLE"]
    Polarity31_16Dio16Disable = 0,
    #[doc = "1: RISE"]
    Polarity31_16Dio16Rise = 1,
    #[doc = "2: FALL"]
    Polarity31_16Dio16Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity31_16Dio16RiseFall = 3,
}
impl From<Polarity31_16Dio16> for u8 {
    #[inline(always)]
    fn from(variant: Polarity31_16Dio16) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity31_16Dio16 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity31_16Dio16 {}
#[doc = "Field `POLARITY31_16_DIO16` reader - Enables and configures edge detection polarity for DIO16."]
pub type Polarity31_16Dio16R = crate::FieldReader<Polarity31_16Dio16>;
impl Polarity31_16Dio16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity31_16Dio16 {
        match self.bits {
            0 => Polarity31_16Dio16::Polarity31_16Dio16Disable,
            1 => Polarity31_16Dio16::Polarity31_16Dio16Rise,
            2 => Polarity31_16Dio16::Polarity31_16Dio16Fall,
            3 => Polarity31_16Dio16::Polarity31_16Dio16RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio16_disable(&self) -> bool {
        *self == Polarity31_16Dio16::Polarity31_16Dio16Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio16_rise(&self) -> bool {
        *self == Polarity31_16Dio16::Polarity31_16Dio16Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio16_fall(&self) -> bool {
        *self == Polarity31_16Dio16::Polarity31_16Dio16Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio16_rise_fall(&self) -> bool {
        *self == Polarity31_16Dio16::Polarity31_16Dio16RiseFall
    }
}
#[doc = "Field `POLARITY31_16_DIO16` writer - Enables and configures edge detection polarity for DIO16."]
pub type Polarity31_16Dio16W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity31_16Dio16, crate::Safe>;
impl<'a, REG> Polarity31_16Dio16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity31_16_dio16_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio16::Polarity31_16Dio16Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity31_16_dio16_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio16::Polarity31_16Dio16Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio16_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio16::Polarity31_16Dio16Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio16_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio16::Polarity31_16Dio16RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity31_16Dio17 {
    #[doc = "0: DISABLE"]
    Polarity31_16Dio17Disable = 0,
    #[doc = "1: RISE"]
    Polarity31_16Dio17Rise = 1,
    #[doc = "2: FALL"]
    Polarity31_16Dio17Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity31_16Dio17RiseFall = 3,
}
impl From<Polarity31_16Dio17> for u8 {
    #[inline(always)]
    fn from(variant: Polarity31_16Dio17) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity31_16Dio17 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity31_16Dio17 {}
#[doc = "Field `POLARITY31_16_DIO17` reader - Enables and configures edge detection polarity for DIO17."]
pub type Polarity31_16Dio17R = crate::FieldReader<Polarity31_16Dio17>;
impl Polarity31_16Dio17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity31_16Dio17 {
        match self.bits {
            0 => Polarity31_16Dio17::Polarity31_16Dio17Disable,
            1 => Polarity31_16Dio17::Polarity31_16Dio17Rise,
            2 => Polarity31_16Dio17::Polarity31_16Dio17Fall,
            3 => Polarity31_16Dio17::Polarity31_16Dio17RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio17_disable(&self) -> bool {
        *self == Polarity31_16Dio17::Polarity31_16Dio17Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio17_rise(&self) -> bool {
        *self == Polarity31_16Dio17::Polarity31_16Dio17Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio17_fall(&self) -> bool {
        *self == Polarity31_16Dio17::Polarity31_16Dio17Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio17_rise_fall(&self) -> bool {
        *self == Polarity31_16Dio17::Polarity31_16Dio17RiseFall
    }
}
#[doc = "Field `POLARITY31_16_DIO17` writer - Enables and configures edge detection polarity for DIO17."]
pub type Polarity31_16Dio17W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity31_16Dio17, crate::Safe>;
impl<'a, REG> Polarity31_16Dio17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity31_16_dio17_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio17::Polarity31_16Dio17Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity31_16_dio17_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio17::Polarity31_16Dio17Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio17_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio17::Polarity31_16Dio17Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio17_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio17::Polarity31_16Dio17RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity31_16Dio18 {
    #[doc = "0: DISABLE"]
    Polarity31_16Dio18Disable = 0,
    #[doc = "1: RISE"]
    Polarity31_16Dio18Rise = 1,
    #[doc = "2: FALL"]
    Polarity31_16Dio18Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity31_16Dio18RiseFall = 3,
}
impl From<Polarity31_16Dio18> for u8 {
    #[inline(always)]
    fn from(variant: Polarity31_16Dio18) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity31_16Dio18 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity31_16Dio18 {}
#[doc = "Field `POLARITY31_16_DIO18` reader - Enables and configures edge detection polarity for DIO18."]
pub type Polarity31_16Dio18R = crate::FieldReader<Polarity31_16Dio18>;
impl Polarity31_16Dio18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity31_16Dio18 {
        match self.bits {
            0 => Polarity31_16Dio18::Polarity31_16Dio18Disable,
            1 => Polarity31_16Dio18::Polarity31_16Dio18Rise,
            2 => Polarity31_16Dio18::Polarity31_16Dio18Fall,
            3 => Polarity31_16Dio18::Polarity31_16Dio18RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio18_disable(&self) -> bool {
        *self == Polarity31_16Dio18::Polarity31_16Dio18Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio18_rise(&self) -> bool {
        *self == Polarity31_16Dio18::Polarity31_16Dio18Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio18_fall(&self) -> bool {
        *self == Polarity31_16Dio18::Polarity31_16Dio18Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio18_rise_fall(&self) -> bool {
        *self == Polarity31_16Dio18::Polarity31_16Dio18RiseFall
    }
}
#[doc = "Field `POLARITY31_16_DIO18` writer - Enables and configures edge detection polarity for DIO18."]
pub type Polarity31_16Dio18W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity31_16Dio18, crate::Safe>;
impl<'a, REG> Polarity31_16Dio18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity31_16_dio18_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio18::Polarity31_16Dio18Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity31_16_dio18_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio18::Polarity31_16Dio18Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio18_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio18::Polarity31_16Dio18Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio18_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio18::Polarity31_16Dio18RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity31_16Dio19 {
    #[doc = "0: DISABLE"]
    Polarity31_16Dio19Disable = 0,
    #[doc = "1: RISE"]
    Polarity31_16Dio19Rise = 1,
    #[doc = "2: FALL"]
    Polarity31_16Dio19Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity31_16Dio19RiseFall = 3,
}
impl From<Polarity31_16Dio19> for u8 {
    #[inline(always)]
    fn from(variant: Polarity31_16Dio19) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity31_16Dio19 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity31_16Dio19 {}
#[doc = "Field `POLARITY31_16_DIO19` reader - Enables and configures edge detection polarity for DIO19."]
pub type Polarity31_16Dio19R = crate::FieldReader<Polarity31_16Dio19>;
impl Polarity31_16Dio19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity31_16Dio19 {
        match self.bits {
            0 => Polarity31_16Dio19::Polarity31_16Dio19Disable,
            1 => Polarity31_16Dio19::Polarity31_16Dio19Rise,
            2 => Polarity31_16Dio19::Polarity31_16Dio19Fall,
            3 => Polarity31_16Dio19::Polarity31_16Dio19RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio19_disable(&self) -> bool {
        *self == Polarity31_16Dio19::Polarity31_16Dio19Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio19_rise(&self) -> bool {
        *self == Polarity31_16Dio19::Polarity31_16Dio19Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio19_fall(&self) -> bool {
        *self == Polarity31_16Dio19::Polarity31_16Dio19Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio19_rise_fall(&self) -> bool {
        *self == Polarity31_16Dio19::Polarity31_16Dio19RiseFall
    }
}
#[doc = "Field `POLARITY31_16_DIO19` writer - Enables and configures edge detection polarity for DIO19."]
pub type Polarity31_16Dio19W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity31_16Dio19, crate::Safe>;
impl<'a, REG> Polarity31_16Dio19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity31_16_dio19_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio19::Polarity31_16Dio19Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity31_16_dio19_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio19::Polarity31_16Dio19Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio19_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio19::Polarity31_16Dio19Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio19_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio19::Polarity31_16Dio19RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity31_16Dio20 {
    #[doc = "0: DISABLE"]
    Polarity31_16Dio20Disable = 0,
    #[doc = "1: RISE"]
    Polarity31_16Dio20Rise = 1,
    #[doc = "2: FALL"]
    Polarity31_16Dio20Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity31_16Dio20RiseFall = 3,
}
impl From<Polarity31_16Dio20> for u8 {
    #[inline(always)]
    fn from(variant: Polarity31_16Dio20) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity31_16Dio20 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity31_16Dio20 {}
#[doc = "Field `POLARITY31_16_DIO20` reader - Enables and configures edge detection polarity for DIO20."]
pub type Polarity31_16Dio20R = crate::FieldReader<Polarity31_16Dio20>;
impl Polarity31_16Dio20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity31_16Dio20 {
        match self.bits {
            0 => Polarity31_16Dio20::Polarity31_16Dio20Disable,
            1 => Polarity31_16Dio20::Polarity31_16Dio20Rise,
            2 => Polarity31_16Dio20::Polarity31_16Dio20Fall,
            3 => Polarity31_16Dio20::Polarity31_16Dio20RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio20_disable(&self) -> bool {
        *self == Polarity31_16Dio20::Polarity31_16Dio20Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio20_rise(&self) -> bool {
        *self == Polarity31_16Dio20::Polarity31_16Dio20Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio20_fall(&self) -> bool {
        *self == Polarity31_16Dio20::Polarity31_16Dio20Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio20_rise_fall(&self) -> bool {
        *self == Polarity31_16Dio20::Polarity31_16Dio20RiseFall
    }
}
#[doc = "Field `POLARITY31_16_DIO20` writer - Enables and configures edge detection polarity for DIO20."]
pub type Polarity31_16Dio20W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity31_16Dio20, crate::Safe>;
impl<'a, REG> Polarity31_16Dio20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity31_16_dio20_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio20::Polarity31_16Dio20Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity31_16_dio20_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio20::Polarity31_16Dio20Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio20_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio20::Polarity31_16Dio20Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio20_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio20::Polarity31_16Dio20RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity31_16Dio21 {
    #[doc = "0: DISABLE"]
    Polarity31_16Dio21Disable = 0,
    #[doc = "1: RISE"]
    Polarity31_16Dio21Rise = 1,
    #[doc = "2: FALL"]
    Polarity31_16Dio21Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity31_16Dio21RiseFall = 3,
}
impl From<Polarity31_16Dio21> for u8 {
    #[inline(always)]
    fn from(variant: Polarity31_16Dio21) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity31_16Dio21 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity31_16Dio21 {}
#[doc = "Field `POLARITY31_16_DIO21` reader - Enables and configures edge detection polarity for DIO21."]
pub type Polarity31_16Dio21R = crate::FieldReader<Polarity31_16Dio21>;
impl Polarity31_16Dio21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity31_16Dio21 {
        match self.bits {
            0 => Polarity31_16Dio21::Polarity31_16Dio21Disable,
            1 => Polarity31_16Dio21::Polarity31_16Dio21Rise,
            2 => Polarity31_16Dio21::Polarity31_16Dio21Fall,
            3 => Polarity31_16Dio21::Polarity31_16Dio21RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio21_disable(&self) -> bool {
        *self == Polarity31_16Dio21::Polarity31_16Dio21Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio21_rise(&self) -> bool {
        *self == Polarity31_16Dio21::Polarity31_16Dio21Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio21_fall(&self) -> bool {
        *self == Polarity31_16Dio21::Polarity31_16Dio21Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio21_rise_fall(&self) -> bool {
        *self == Polarity31_16Dio21::Polarity31_16Dio21RiseFall
    }
}
#[doc = "Field `POLARITY31_16_DIO21` writer - Enables and configures edge detection polarity for DIO21."]
pub type Polarity31_16Dio21W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity31_16Dio21, crate::Safe>;
impl<'a, REG> Polarity31_16Dio21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity31_16_dio21_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio21::Polarity31_16Dio21Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity31_16_dio21_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio21::Polarity31_16Dio21Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio21_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio21::Polarity31_16Dio21Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio21_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio21::Polarity31_16Dio21RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity31_16Dio22 {
    #[doc = "0: DISABLE"]
    Polarity31_16Dio22Disable = 0,
    #[doc = "1: RISE"]
    Polarity31_16Dio22Rise = 1,
    #[doc = "2: FALL"]
    Polarity31_16Dio22Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity31_16Dio22RiseFall = 3,
}
impl From<Polarity31_16Dio22> for u8 {
    #[inline(always)]
    fn from(variant: Polarity31_16Dio22) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity31_16Dio22 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity31_16Dio22 {}
#[doc = "Field `POLARITY31_16_DIO22` reader - Enables and configures edge detection polarity for DIO22."]
pub type Polarity31_16Dio22R = crate::FieldReader<Polarity31_16Dio22>;
impl Polarity31_16Dio22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity31_16Dio22 {
        match self.bits {
            0 => Polarity31_16Dio22::Polarity31_16Dio22Disable,
            1 => Polarity31_16Dio22::Polarity31_16Dio22Rise,
            2 => Polarity31_16Dio22::Polarity31_16Dio22Fall,
            3 => Polarity31_16Dio22::Polarity31_16Dio22RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio22_disable(&self) -> bool {
        *self == Polarity31_16Dio22::Polarity31_16Dio22Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio22_rise(&self) -> bool {
        *self == Polarity31_16Dio22::Polarity31_16Dio22Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio22_fall(&self) -> bool {
        *self == Polarity31_16Dio22::Polarity31_16Dio22Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio22_rise_fall(&self) -> bool {
        *self == Polarity31_16Dio22::Polarity31_16Dio22RiseFall
    }
}
#[doc = "Field `POLARITY31_16_DIO22` writer - Enables and configures edge detection polarity for DIO22."]
pub type Polarity31_16Dio22W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity31_16Dio22, crate::Safe>;
impl<'a, REG> Polarity31_16Dio22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity31_16_dio22_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio22::Polarity31_16Dio22Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity31_16_dio22_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio22::Polarity31_16Dio22Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio22_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio22::Polarity31_16Dio22Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio22_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio22::Polarity31_16Dio22RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity31_16Dio23 {
    #[doc = "0: DISABLE"]
    Polarity31_16Dio23Disable = 0,
    #[doc = "1: RISE"]
    Polarity31_16Dio23Rise = 1,
    #[doc = "2: FALL"]
    Polarity31_16Dio23Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity31_16Dio23RiseFall = 3,
}
impl From<Polarity31_16Dio23> for u8 {
    #[inline(always)]
    fn from(variant: Polarity31_16Dio23) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity31_16Dio23 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity31_16Dio23 {}
#[doc = "Field `POLARITY31_16_DIO23` reader - Enables and configures edge detection polarity for DIO23."]
pub type Polarity31_16Dio23R = crate::FieldReader<Polarity31_16Dio23>;
impl Polarity31_16Dio23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity31_16Dio23 {
        match self.bits {
            0 => Polarity31_16Dio23::Polarity31_16Dio23Disable,
            1 => Polarity31_16Dio23::Polarity31_16Dio23Rise,
            2 => Polarity31_16Dio23::Polarity31_16Dio23Fall,
            3 => Polarity31_16Dio23::Polarity31_16Dio23RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio23_disable(&self) -> bool {
        *self == Polarity31_16Dio23::Polarity31_16Dio23Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio23_rise(&self) -> bool {
        *self == Polarity31_16Dio23::Polarity31_16Dio23Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio23_fall(&self) -> bool {
        *self == Polarity31_16Dio23::Polarity31_16Dio23Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio23_rise_fall(&self) -> bool {
        *self == Polarity31_16Dio23::Polarity31_16Dio23RiseFall
    }
}
#[doc = "Field `POLARITY31_16_DIO23` writer - Enables and configures edge detection polarity for DIO23."]
pub type Polarity31_16Dio23W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity31_16Dio23, crate::Safe>;
impl<'a, REG> Polarity31_16Dio23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity31_16_dio23_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio23::Polarity31_16Dio23Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity31_16_dio23_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio23::Polarity31_16Dio23Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio23_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio23::Polarity31_16Dio23Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio23_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio23::Polarity31_16Dio23RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity31_16Dio24 {
    #[doc = "0: DISABLE"]
    Polarity31_16Dio24Disable = 0,
    #[doc = "1: RISE"]
    Polarity31_16Dio24Rise = 1,
    #[doc = "2: FALL"]
    Polarity31_16Dio24Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity31_16Dio24RiseFall = 3,
}
impl From<Polarity31_16Dio24> for u8 {
    #[inline(always)]
    fn from(variant: Polarity31_16Dio24) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity31_16Dio24 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity31_16Dio24 {}
#[doc = "Field `POLARITY31_16_DIO24` reader - Enables and configures edge detection polarity for DIO24."]
pub type Polarity31_16Dio24R = crate::FieldReader<Polarity31_16Dio24>;
impl Polarity31_16Dio24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity31_16Dio24 {
        match self.bits {
            0 => Polarity31_16Dio24::Polarity31_16Dio24Disable,
            1 => Polarity31_16Dio24::Polarity31_16Dio24Rise,
            2 => Polarity31_16Dio24::Polarity31_16Dio24Fall,
            3 => Polarity31_16Dio24::Polarity31_16Dio24RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio24_disable(&self) -> bool {
        *self == Polarity31_16Dio24::Polarity31_16Dio24Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio24_rise(&self) -> bool {
        *self == Polarity31_16Dio24::Polarity31_16Dio24Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio24_fall(&self) -> bool {
        *self == Polarity31_16Dio24::Polarity31_16Dio24Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio24_rise_fall(&self) -> bool {
        *self == Polarity31_16Dio24::Polarity31_16Dio24RiseFall
    }
}
#[doc = "Field `POLARITY31_16_DIO24` writer - Enables and configures edge detection polarity for DIO24."]
pub type Polarity31_16Dio24W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity31_16Dio24, crate::Safe>;
impl<'a, REG> Polarity31_16Dio24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity31_16_dio24_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio24::Polarity31_16Dio24Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity31_16_dio24_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio24::Polarity31_16Dio24Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio24_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio24::Polarity31_16Dio24Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio24_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio24::Polarity31_16Dio24RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity31_16Dio25 {
    #[doc = "0: DISABLE"]
    Polarity31_16Dio25Disable = 0,
    #[doc = "1: RISE"]
    Polarity31_16Dio25Rise = 1,
    #[doc = "2: FALL"]
    Polarity31_16Dio25Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity31_16Dio25RiseFall = 3,
}
impl From<Polarity31_16Dio25> for u8 {
    #[inline(always)]
    fn from(variant: Polarity31_16Dio25) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity31_16Dio25 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity31_16Dio25 {}
#[doc = "Field `POLARITY31_16_DIO25` reader - Enables and configures edge detection polarity for DIO25."]
pub type Polarity31_16Dio25R = crate::FieldReader<Polarity31_16Dio25>;
impl Polarity31_16Dio25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity31_16Dio25 {
        match self.bits {
            0 => Polarity31_16Dio25::Polarity31_16Dio25Disable,
            1 => Polarity31_16Dio25::Polarity31_16Dio25Rise,
            2 => Polarity31_16Dio25::Polarity31_16Dio25Fall,
            3 => Polarity31_16Dio25::Polarity31_16Dio25RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio25_disable(&self) -> bool {
        *self == Polarity31_16Dio25::Polarity31_16Dio25Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio25_rise(&self) -> bool {
        *self == Polarity31_16Dio25::Polarity31_16Dio25Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio25_fall(&self) -> bool {
        *self == Polarity31_16Dio25::Polarity31_16Dio25Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio25_rise_fall(&self) -> bool {
        *self == Polarity31_16Dio25::Polarity31_16Dio25RiseFall
    }
}
#[doc = "Field `POLARITY31_16_DIO25` writer - Enables and configures edge detection polarity for DIO25."]
pub type Polarity31_16Dio25W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity31_16Dio25, crate::Safe>;
impl<'a, REG> Polarity31_16Dio25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity31_16_dio25_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio25::Polarity31_16Dio25Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity31_16_dio25_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio25::Polarity31_16Dio25Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio25_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio25::Polarity31_16Dio25Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio25_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio25::Polarity31_16Dio25RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity31_16Dio26 {
    #[doc = "0: DISABLE"]
    Polarity31_16Dio26Disable = 0,
    #[doc = "1: RISE"]
    Polarity31_16Dio26Rise = 1,
    #[doc = "2: FALL"]
    Polarity31_16Dio26Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity31_16Dio26RiseFall = 3,
}
impl From<Polarity31_16Dio26> for u8 {
    #[inline(always)]
    fn from(variant: Polarity31_16Dio26) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity31_16Dio26 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity31_16Dio26 {}
#[doc = "Field `POLARITY31_16_DIO26` reader - Enables and configures edge detection polarity for DIO26."]
pub type Polarity31_16Dio26R = crate::FieldReader<Polarity31_16Dio26>;
impl Polarity31_16Dio26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity31_16Dio26 {
        match self.bits {
            0 => Polarity31_16Dio26::Polarity31_16Dio26Disable,
            1 => Polarity31_16Dio26::Polarity31_16Dio26Rise,
            2 => Polarity31_16Dio26::Polarity31_16Dio26Fall,
            3 => Polarity31_16Dio26::Polarity31_16Dio26RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio26_disable(&self) -> bool {
        *self == Polarity31_16Dio26::Polarity31_16Dio26Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio26_rise(&self) -> bool {
        *self == Polarity31_16Dio26::Polarity31_16Dio26Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio26_fall(&self) -> bool {
        *self == Polarity31_16Dio26::Polarity31_16Dio26Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio26_rise_fall(&self) -> bool {
        *self == Polarity31_16Dio26::Polarity31_16Dio26RiseFall
    }
}
#[doc = "Field `POLARITY31_16_DIO26` writer - Enables and configures edge detection polarity for DIO26."]
pub type Polarity31_16Dio26W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity31_16Dio26, crate::Safe>;
impl<'a, REG> Polarity31_16Dio26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity31_16_dio26_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio26::Polarity31_16Dio26Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity31_16_dio26_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio26::Polarity31_16Dio26Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio26_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio26::Polarity31_16Dio26Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio26_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio26::Polarity31_16Dio26RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity31_16Dio27 {
    #[doc = "0: DISABLE"]
    Polarity31_16Dio27Disable = 0,
    #[doc = "1: RISE"]
    Polarity31_16Dio27Rise = 1,
    #[doc = "2: FALL"]
    Polarity31_16Dio27Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity31_16Dio27RiseFall = 3,
}
impl From<Polarity31_16Dio27> for u8 {
    #[inline(always)]
    fn from(variant: Polarity31_16Dio27) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity31_16Dio27 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity31_16Dio27 {}
#[doc = "Field `POLARITY31_16_DIO27` reader - Enables and configures edge detection polarity for DIO27."]
pub type Polarity31_16Dio27R = crate::FieldReader<Polarity31_16Dio27>;
impl Polarity31_16Dio27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity31_16Dio27 {
        match self.bits {
            0 => Polarity31_16Dio27::Polarity31_16Dio27Disable,
            1 => Polarity31_16Dio27::Polarity31_16Dio27Rise,
            2 => Polarity31_16Dio27::Polarity31_16Dio27Fall,
            3 => Polarity31_16Dio27::Polarity31_16Dio27RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio27_disable(&self) -> bool {
        *self == Polarity31_16Dio27::Polarity31_16Dio27Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio27_rise(&self) -> bool {
        *self == Polarity31_16Dio27::Polarity31_16Dio27Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio27_fall(&self) -> bool {
        *self == Polarity31_16Dio27::Polarity31_16Dio27Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio27_rise_fall(&self) -> bool {
        *self == Polarity31_16Dio27::Polarity31_16Dio27RiseFall
    }
}
#[doc = "Field `POLARITY31_16_DIO27` writer - Enables and configures edge detection polarity for DIO27."]
pub type Polarity31_16Dio27W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity31_16Dio27, crate::Safe>;
impl<'a, REG> Polarity31_16Dio27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity31_16_dio27_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio27::Polarity31_16Dio27Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity31_16_dio27_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio27::Polarity31_16Dio27Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio27_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio27::Polarity31_16Dio27Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio27_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio27::Polarity31_16Dio27RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity31_16Dio28 {
    #[doc = "0: DISABLE"]
    Polarity31_16Dio28Disable = 0,
    #[doc = "1: RISE"]
    Polarity31_16Dio28Rise = 1,
    #[doc = "2: FALL"]
    Polarity31_16Dio28Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity31_16Dio28RiseFall = 3,
}
impl From<Polarity31_16Dio28> for u8 {
    #[inline(always)]
    fn from(variant: Polarity31_16Dio28) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity31_16Dio28 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity31_16Dio28 {}
#[doc = "Field `POLARITY31_16_DIO28` reader - Enables and configures edge detection polarity for DIO28."]
pub type Polarity31_16Dio28R = crate::FieldReader<Polarity31_16Dio28>;
impl Polarity31_16Dio28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity31_16Dio28 {
        match self.bits {
            0 => Polarity31_16Dio28::Polarity31_16Dio28Disable,
            1 => Polarity31_16Dio28::Polarity31_16Dio28Rise,
            2 => Polarity31_16Dio28::Polarity31_16Dio28Fall,
            3 => Polarity31_16Dio28::Polarity31_16Dio28RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio28_disable(&self) -> bool {
        *self == Polarity31_16Dio28::Polarity31_16Dio28Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio28_rise(&self) -> bool {
        *self == Polarity31_16Dio28::Polarity31_16Dio28Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio28_fall(&self) -> bool {
        *self == Polarity31_16Dio28::Polarity31_16Dio28Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio28_rise_fall(&self) -> bool {
        *self == Polarity31_16Dio28::Polarity31_16Dio28RiseFall
    }
}
#[doc = "Field `POLARITY31_16_DIO28` writer - Enables and configures edge detection polarity for DIO28."]
pub type Polarity31_16Dio28W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity31_16Dio28, crate::Safe>;
impl<'a, REG> Polarity31_16Dio28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity31_16_dio28_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio28::Polarity31_16Dio28Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity31_16_dio28_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio28::Polarity31_16Dio28Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio28_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio28::Polarity31_16Dio28Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio28_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio28::Polarity31_16Dio28RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO29.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity31_16Dio29 {
    #[doc = "0: DISABLE"]
    Polarity31_16Dio29Disable = 0,
    #[doc = "1: RISE"]
    Polarity31_16Dio29Rise = 1,
    #[doc = "2: FALL"]
    Polarity31_16Dio29Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity31_16Dio29RiseFall = 3,
}
impl From<Polarity31_16Dio29> for u8 {
    #[inline(always)]
    fn from(variant: Polarity31_16Dio29) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity31_16Dio29 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity31_16Dio29 {}
#[doc = "Field `POLARITY31_16_DIO29` reader - Enables and configures edge detection polarity for DIO29."]
pub type Polarity31_16Dio29R = crate::FieldReader<Polarity31_16Dio29>;
impl Polarity31_16Dio29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity31_16Dio29 {
        match self.bits {
            0 => Polarity31_16Dio29::Polarity31_16Dio29Disable,
            1 => Polarity31_16Dio29::Polarity31_16Dio29Rise,
            2 => Polarity31_16Dio29::Polarity31_16Dio29Fall,
            3 => Polarity31_16Dio29::Polarity31_16Dio29RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio29_disable(&self) -> bool {
        *self == Polarity31_16Dio29::Polarity31_16Dio29Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio29_rise(&self) -> bool {
        *self == Polarity31_16Dio29::Polarity31_16Dio29Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio29_fall(&self) -> bool {
        *self == Polarity31_16Dio29::Polarity31_16Dio29Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio29_rise_fall(&self) -> bool {
        *self == Polarity31_16Dio29::Polarity31_16Dio29RiseFall
    }
}
#[doc = "Field `POLARITY31_16_DIO29` writer - Enables and configures edge detection polarity for DIO29."]
pub type Polarity31_16Dio29W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity31_16Dio29, crate::Safe>;
impl<'a, REG> Polarity31_16Dio29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity31_16_dio29_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio29::Polarity31_16Dio29Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity31_16_dio29_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio29::Polarity31_16Dio29Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio29_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio29::Polarity31_16Dio29Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio29_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio29::Polarity31_16Dio29RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO30.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity31_16Dio30 {
    #[doc = "0: DISABLE"]
    Polarity31_16Dio30Disable = 0,
    #[doc = "1: RISE"]
    Polarity31_16Dio30Rise = 1,
    #[doc = "2: FALL"]
    Polarity31_16Dio30Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity31_16Dio30RiseFall = 3,
}
impl From<Polarity31_16Dio30> for u8 {
    #[inline(always)]
    fn from(variant: Polarity31_16Dio30) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity31_16Dio30 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity31_16Dio30 {}
#[doc = "Field `POLARITY31_16_DIO30` reader - Enables and configures edge detection polarity for DIO30."]
pub type Polarity31_16Dio30R = crate::FieldReader<Polarity31_16Dio30>;
impl Polarity31_16Dio30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity31_16Dio30 {
        match self.bits {
            0 => Polarity31_16Dio30::Polarity31_16Dio30Disable,
            1 => Polarity31_16Dio30::Polarity31_16Dio30Rise,
            2 => Polarity31_16Dio30::Polarity31_16Dio30Fall,
            3 => Polarity31_16Dio30::Polarity31_16Dio30RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio30_disable(&self) -> bool {
        *self == Polarity31_16Dio30::Polarity31_16Dio30Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio30_rise(&self) -> bool {
        *self == Polarity31_16Dio30::Polarity31_16Dio30Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio30_fall(&self) -> bool {
        *self == Polarity31_16Dio30::Polarity31_16Dio30Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio30_rise_fall(&self) -> bool {
        *self == Polarity31_16Dio30::Polarity31_16Dio30RiseFall
    }
}
#[doc = "Field `POLARITY31_16_DIO30` writer - Enables and configures edge detection polarity for DIO30."]
pub type Polarity31_16Dio30W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity31_16Dio30, crate::Safe>;
impl<'a, REG> Polarity31_16Dio30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity31_16_dio30_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio30::Polarity31_16Dio30Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity31_16_dio30_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio30::Polarity31_16Dio30Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio30_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio30::Polarity31_16Dio30Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio30_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio30::Polarity31_16Dio30RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO31.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity31_16Dio31 {
    #[doc = "0: DISABLE"]
    Polarity31_16Dio31Disable = 0,
    #[doc = "1: RISE"]
    Polarity31_16Dio31Rise = 1,
    #[doc = "2: FALL"]
    Polarity31_16Dio31Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity31_16Dio31RiseFall = 3,
}
impl From<Polarity31_16Dio31> for u8 {
    #[inline(always)]
    fn from(variant: Polarity31_16Dio31) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity31_16Dio31 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity31_16Dio31 {}
#[doc = "Field `POLARITY31_16_DIO31` reader - Enables and configures edge detection polarity for DIO31."]
pub type Polarity31_16Dio31R = crate::FieldReader<Polarity31_16Dio31>;
impl Polarity31_16Dio31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity31_16Dio31 {
        match self.bits {
            0 => Polarity31_16Dio31::Polarity31_16Dio31Disable,
            1 => Polarity31_16Dio31::Polarity31_16Dio31Rise,
            2 => Polarity31_16Dio31::Polarity31_16Dio31Fall,
            3 => Polarity31_16Dio31::Polarity31_16Dio31RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio31_disable(&self) -> bool {
        *self == Polarity31_16Dio31::Polarity31_16Dio31Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity31_16_dio31_rise(&self) -> bool {
        *self == Polarity31_16Dio31::Polarity31_16Dio31Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio31_fall(&self) -> bool {
        *self == Polarity31_16Dio31::Polarity31_16Dio31Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity31_16_dio31_rise_fall(&self) -> bool {
        *self == Polarity31_16Dio31::Polarity31_16Dio31RiseFall
    }
}
#[doc = "Field `POLARITY31_16_DIO31` writer - Enables and configures edge detection polarity for DIO31."]
pub type Polarity31_16Dio31W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity31_16Dio31, crate::Safe>;
impl<'a, REG> Polarity31_16Dio31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity31_16_dio31_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio31::Polarity31_16Dio31Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity31_16_dio31_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio31::Polarity31_16Dio31Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio31_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio31::Polarity31_16Dio31Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity31_16_dio31_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity31_16Dio31::Polarity31_16Dio31RiseFall)
    }
}
impl R {
    #[doc = "Bits 0:1 - Enables and configures edge detection polarity for DIO16."]
    #[inline(always)]
    pub fn polarity31_16_dio16(&self) -> Polarity31_16Dio16R {
        Polarity31_16Dio16R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Enables and configures edge detection polarity for DIO17."]
    #[inline(always)]
    pub fn polarity31_16_dio17(&self) -> Polarity31_16Dio17R {
        Polarity31_16Dio17R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Enables and configures edge detection polarity for DIO18."]
    #[inline(always)]
    pub fn polarity31_16_dio18(&self) -> Polarity31_16Dio18R {
        Polarity31_16Dio18R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Enables and configures edge detection polarity for DIO19."]
    #[inline(always)]
    pub fn polarity31_16_dio19(&self) -> Polarity31_16Dio19R {
        Polarity31_16Dio19R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Enables and configures edge detection polarity for DIO20."]
    #[inline(always)]
    pub fn polarity31_16_dio20(&self) -> Polarity31_16Dio20R {
        Polarity31_16Dio20R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Enables and configures edge detection polarity for DIO21."]
    #[inline(always)]
    pub fn polarity31_16_dio21(&self) -> Polarity31_16Dio21R {
        Polarity31_16Dio21R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Enables and configures edge detection polarity for DIO22."]
    #[inline(always)]
    pub fn polarity31_16_dio22(&self) -> Polarity31_16Dio22R {
        Polarity31_16Dio22R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Enables and configures edge detection polarity for DIO23."]
    #[inline(always)]
    pub fn polarity31_16_dio23(&self) -> Polarity31_16Dio23R {
        Polarity31_16Dio23R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Enables and configures edge detection polarity for DIO24."]
    #[inline(always)]
    pub fn polarity31_16_dio24(&self) -> Polarity31_16Dio24R {
        Polarity31_16Dio24R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Enables and configures edge detection polarity for DIO25."]
    #[inline(always)]
    pub fn polarity31_16_dio25(&self) -> Polarity31_16Dio25R {
        Polarity31_16Dio25R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Enables and configures edge detection polarity for DIO26."]
    #[inline(always)]
    pub fn polarity31_16_dio26(&self) -> Polarity31_16Dio26R {
        Polarity31_16Dio26R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Enables and configures edge detection polarity for DIO27."]
    #[inline(always)]
    pub fn polarity31_16_dio27(&self) -> Polarity31_16Dio27R {
        Polarity31_16Dio27R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Enables and configures edge detection polarity for DIO28."]
    #[inline(always)]
    pub fn polarity31_16_dio28(&self) -> Polarity31_16Dio28R {
        Polarity31_16Dio28R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Enables and configures edge detection polarity for DIO29."]
    #[inline(always)]
    pub fn polarity31_16_dio29(&self) -> Polarity31_16Dio29R {
        Polarity31_16Dio29R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Enables and configures edge detection polarity for DIO30."]
    #[inline(always)]
    pub fn polarity31_16_dio30(&self) -> Polarity31_16Dio30R {
        Polarity31_16Dio30R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Enables and configures edge detection polarity for DIO31."]
    #[inline(always)]
    pub fn polarity31_16_dio31(&self) -> Polarity31_16Dio31R {
        Polarity31_16Dio31R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Enables and configures edge detection polarity for DIO16."]
    #[inline(always)]
    pub fn polarity31_16_dio16(&mut self) -> Polarity31_16Dio16W<Polarity31_16Spec> {
        Polarity31_16Dio16W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Enables and configures edge detection polarity for DIO17."]
    #[inline(always)]
    pub fn polarity31_16_dio17(&mut self) -> Polarity31_16Dio17W<Polarity31_16Spec> {
        Polarity31_16Dio17W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Enables and configures edge detection polarity for DIO18."]
    #[inline(always)]
    pub fn polarity31_16_dio18(&mut self) -> Polarity31_16Dio18W<Polarity31_16Spec> {
        Polarity31_16Dio18W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Enables and configures edge detection polarity for DIO19."]
    #[inline(always)]
    pub fn polarity31_16_dio19(&mut self) -> Polarity31_16Dio19W<Polarity31_16Spec> {
        Polarity31_16Dio19W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Enables and configures edge detection polarity for DIO20."]
    #[inline(always)]
    pub fn polarity31_16_dio20(&mut self) -> Polarity31_16Dio20W<Polarity31_16Spec> {
        Polarity31_16Dio20W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Enables and configures edge detection polarity for DIO21."]
    #[inline(always)]
    pub fn polarity31_16_dio21(&mut self) -> Polarity31_16Dio21W<Polarity31_16Spec> {
        Polarity31_16Dio21W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Enables and configures edge detection polarity for DIO22."]
    #[inline(always)]
    pub fn polarity31_16_dio22(&mut self) -> Polarity31_16Dio22W<Polarity31_16Spec> {
        Polarity31_16Dio22W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Enables and configures edge detection polarity for DIO23."]
    #[inline(always)]
    pub fn polarity31_16_dio23(&mut self) -> Polarity31_16Dio23W<Polarity31_16Spec> {
        Polarity31_16Dio23W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Enables and configures edge detection polarity for DIO24."]
    #[inline(always)]
    pub fn polarity31_16_dio24(&mut self) -> Polarity31_16Dio24W<Polarity31_16Spec> {
        Polarity31_16Dio24W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Enables and configures edge detection polarity for DIO25."]
    #[inline(always)]
    pub fn polarity31_16_dio25(&mut self) -> Polarity31_16Dio25W<Polarity31_16Spec> {
        Polarity31_16Dio25W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Enables and configures edge detection polarity for DIO26."]
    #[inline(always)]
    pub fn polarity31_16_dio26(&mut self) -> Polarity31_16Dio26W<Polarity31_16Spec> {
        Polarity31_16Dio26W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Enables and configures edge detection polarity for DIO27."]
    #[inline(always)]
    pub fn polarity31_16_dio27(&mut self) -> Polarity31_16Dio27W<Polarity31_16Spec> {
        Polarity31_16Dio27W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Enables and configures edge detection polarity for DIO28."]
    #[inline(always)]
    pub fn polarity31_16_dio28(&mut self) -> Polarity31_16Dio28W<Polarity31_16Spec> {
        Polarity31_16Dio28W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Enables and configures edge detection polarity for DIO29."]
    #[inline(always)]
    pub fn polarity31_16_dio29(&mut self) -> Polarity31_16Dio29W<Polarity31_16Spec> {
        Polarity31_16Dio29W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Enables and configures edge detection polarity for DIO30."]
    #[inline(always)]
    pub fn polarity31_16_dio30(&mut self) -> Polarity31_16Dio30W<Polarity31_16Spec> {
        Polarity31_16Dio30W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Enables and configures edge detection polarity for DIO31."]
    #[inline(always)]
    pub fn polarity31_16_dio31(&mut self) -> Polarity31_16Dio31W<Polarity31_16Spec> {
        Polarity31_16Dio31W::new(self, 30)
    }
}
#[doc = "Polarity 31 to 16\n\nYou can [`read`](crate::Reg::read) this register and get [`polarity31_16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polarity31_16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Polarity31_16Spec;
impl crate::RegisterSpec for Polarity31_16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`polarity31_16::R`](R) reader structure"]
impl crate::Readable for Polarity31_16Spec {}
#[doc = "`write(|w| ..)` method takes [`polarity31_16::W`](W) writer structure"]
impl crate::Writable for Polarity31_16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POLARITY31_16 to value 0"]
impl crate::Resettable for Polarity31_16Spec {
    const RESET_VALUE: u32 = 0;
}
