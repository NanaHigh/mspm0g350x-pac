#[doc = "Register `POLARITY15_0` reader"]
pub type R = crate::R<Polarity15_0Spec>;
#[doc = "Register `POLARITY15_0` writer"]
pub type W = crate::W<Polarity15_0Spec>;
#[doc = "Enables and configures edge detection polarity for DIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity15_0Dio0 {
    #[doc = "0: DISABLE"]
    Polarity15_0Dio0Disable = 0,
    #[doc = "1: RISE"]
    Polarity15_0Dio0Rise = 1,
    #[doc = "2: FALL"]
    Polarity15_0Dio0Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity15_0Dio0RiseFall = 3,
}
impl From<Polarity15_0Dio0> for u8 {
    #[inline(always)]
    fn from(variant: Polarity15_0Dio0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity15_0Dio0 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity15_0Dio0 {}
#[doc = "Field `POLARITY15_0_DIO0` reader - Enables and configures edge detection polarity for DIO0."]
pub type Polarity15_0Dio0R = crate::FieldReader<Polarity15_0Dio0>;
impl Polarity15_0Dio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity15_0Dio0 {
        match self.bits {
            0 => Polarity15_0Dio0::Polarity15_0Dio0Disable,
            1 => Polarity15_0Dio0::Polarity15_0Dio0Rise,
            2 => Polarity15_0Dio0::Polarity15_0Dio0Fall,
            3 => Polarity15_0Dio0::Polarity15_0Dio0RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio0_disable(&self) -> bool {
        *self == Polarity15_0Dio0::Polarity15_0Dio0Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio0_rise(&self) -> bool {
        *self == Polarity15_0Dio0::Polarity15_0Dio0Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio0_fall(&self) -> bool {
        *self == Polarity15_0Dio0::Polarity15_0Dio0Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio0_rise_fall(&self) -> bool {
        *self == Polarity15_0Dio0::Polarity15_0Dio0RiseFall
    }
}
#[doc = "Field `POLARITY15_0_DIO0` writer - Enables and configures edge detection polarity for DIO0."]
pub type Polarity15_0Dio0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Polarity15_0Dio0, crate::Safe>;
impl<'a, REG> Polarity15_0Dio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity15_0_dio0_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio0::Polarity15_0Dio0Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity15_0_dio0_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio0::Polarity15_0Dio0Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio0_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio0::Polarity15_0Dio0Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio0_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio0::Polarity15_0Dio0RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity15_0Dio1 {
    #[doc = "0: DISABLE"]
    Polarity15_0Dio1Disable = 0,
    #[doc = "1: RISE"]
    Polarity15_0Dio1Rise = 1,
    #[doc = "2: FALL"]
    Polarity15_0Dio1Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity15_0Dio1RiseFall = 3,
}
impl From<Polarity15_0Dio1> for u8 {
    #[inline(always)]
    fn from(variant: Polarity15_0Dio1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity15_0Dio1 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity15_0Dio1 {}
#[doc = "Field `POLARITY15_0_DIO1` reader - Enables and configures edge detection polarity for DIO1."]
pub type Polarity15_0Dio1R = crate::FieldReader<Polarity15_0Dio1>;
impl Polarity15_0Dio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity15_0Dio1 {
        match self.bits {
            0 => Polarity15_0Dio1::Polarity15_0Dio1Disable,
            1 => Polarity15_0Dio1::Polarity15_0Dio1Rise,
            2 => Polarity15_0Dio1::Polarity15_0Dio1Fall,
            3 => Polarity15_0Dio1::Polarity15_0Dio1RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio1_disable(&self) -> bool {
        *self == Polarity15_0Dio1::Polarity15_0Dio1Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio1_rise(&self) -> bool {
        *self == Polarity15_0Dio1::Polarity15_0Dio1Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio1_fall(&self) -> bool {
        *self == Polarity15_0Dio1::Polarity15_0Dio1Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio1_rise_fall(&self) -> bool {
        *self == Polarity15_0Dio1::Polarity15_0Dio1RiseFall
    }
}
#[doc = "Field `POLARITY15_0_DIO1` writer - Enables and configures edge detection polarity for DIO1."]
pub type Polarity15_0Dio1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Polarity15_0Dio1, crate::Safe>;
impl<'a, REG> Polarity15_0Dio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity15_0_dio1_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio1::Polarity15_0Dio1Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity15_0_dio1_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio1::Polarity15_0Dio1Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio1_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio1::Polarity15_0Dio1Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio1_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio1::Polarity15_0Dio1RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity15_0Dio2 {
    #[doc = "0: DISABLE"]
    Polarity15_0Dio2Disable = 0,
    #[doc = "1: RISE"]
    Polarity15_0Dio2Rise = 1,
    #[doc = "2: FALL"]
    Polarity15_0Dio2Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity15_0Dio2RiseFall = 3,
}
impl From<Polarity15_0Dio2> for u8 {
    #[inline(always)]
    fn from(variant: Polarity15_0Dio2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity15_0Dio2 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity15_0Dio2 {}
#[doc = "Field `POLARITY15_0_DIO2` reader - Enables and configures edge detection polarity for DIO2."]
pub type Polarity15_0Dio2R = crate::FieldReader<Polarity15_0Dio2>;
impl Polarity15_0Dio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity15_0Dio2 {
        match self.bits {
            0 => Polarity15_0Dio2::Polarity15_0Dio2Disable,
            1 => Polarity15_0Dio2::Polarity15_0Dio2Rise,
            2 => Polarity15_0Dio2::Polarity15_0Dio2Fall,
            3 => Polarity15_0Dio2::Polarity15_0Dio2RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio2_disable(&self) -> bool {
        *self == Polarity15_0Dio2::Polarity15_0Dio2Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio2_rise(&self) -> bool {
        *self == Polarity15_0Dio2::Polarity15_0Dio2Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio2_fall(&self) -> bool {
        *self == Polarity15_0Dio2::Polarity15_0Dio2Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio2_rise_fall(&self) -> bool {
        *self == Polarity15_0Dio2::Polarity15_0Dio2RiseFall
    }
}
#[doc = "Field `POLARITY15_0_DIO2` writer - Enables and configures edge detection polarity for DIO2."]
pub type Polarity15_0Dio2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Polarity15_0Dio2, crate::Safe>;
impl<'a, REG> Polarity15_0Dio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity15_0_dio2_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio2::Polarity15_0Dio2Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity15_0_dio2_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio2::Polarity15_0Dio2Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio2_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio2::Polarity15_0Dio2Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio2_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio2::Polarity15_0Dio2RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity15_0Dio3 {
    #[doc = "0: DISABLE"]
    Polarity15_0Dio3Disable = 0,
    #[doc = "1: RISE"]
    Polarity15_0Dio3Rise = 1,
    #[doc = "2: FALL"]
    Polarity15_0Dio3Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity15_0Dio3RiseFall = 3,
}
impl From<Polarity15_0Dio3> for u8 {
    #[inline(always)]
    fn from(variant: Polarity15_0Dio3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity15_0Dio3 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity15_0Dio3 {}
#[doc = "Field `POLARITY15_0_DIO3` reader - Enables and configures edge detection polarity for DIO3."]
pub type Polarity15_0Dio3R = crate::FieldReader<Polarity15_0Dio3>;
impl Polarity15_0Dio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity15_0Dio3 {
        match self.bits {
            0 => Polarity15_0Dio3::Polarity15_0Dio3Disable,
            1 => Polarity15_0Dio3::Polarity15_0Dio3Rise,
            2 => Polarity15_0Dio3::Polarity15_0Dio3Fall,
            3 => Polarity15_0Dio3::Polarity15_0Dio3RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio3_disable(&self) -> bool {
        *self == Polarity15_0Dio3::Polarity15_0Dio3Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio3_rise(&self) -> bool {
        *self == Polarity15_0Dio3::Polarity15_0Dio3Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio3_fall(&self) -> bool {
        *self == Polarity15_0Dio3::Polarity15_0Dio3Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio3_rise_fall(&self) -> bool {
        *self == Polarity15_0Dio3::Polarity15_0Dio3RiseFall
    }
}
#[doc = "Field `POLARITY15_0_DIO3` writer - Enables and configures edge detection polarity for DIO3."]
pub type Polarity15_0Dio3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Polarity15_0Dio3, crate::Safe>;
impl<'a, REG> Polarity15_0Dio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity15_0_dio3_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio3::Polarity15_0Dio3Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity15_0_dio3_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio3::Polarity15_0Dio3Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio3_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio3::Polarity15_0Dio3Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio3_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio3::Polarity15_0Dio3RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity15_0Dio4 {
    #[doc = "0: DISABLE"]
    Polarity15_0Dio4Disable = 0,
    #[doc = "1: RISE"]
    Polarity15_0Dio4Rise = 1,
    #[doc = "2: FALL"]
    Polarity15_0Dio4Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity15_0Dio4RiseFall = 3,
}
impl From<Polarity15_0Dio4> for u8 {
    #[inline(always)]
    fn from(variant: Polarity15_0Dio4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity15_0Dio4 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity15_0Dio4 {}
#[doc = "Field `POLARITY15_0_DIO4` reader - Enables and configures edge detection polarity for DIO4."]
pub type Polarity15_0Dio4R = crate::FieldReader<Polarity15_0Dio4>;
impl Polarity15_0Dio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity15_0Dio4 {
        match self.bits {
            0 => Polarity15_0Dio4::Polarity15_0Dio4Disable,
            1 => Polarity15_0Dio4::Polarity15_0Dio4Rise,
            2 => Polarity15_0Dio4::Polarity15_0Dio4Fall,
            3 => Polarity15_0Dio4::Polarity15_0Dio4RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio4_disable(&self) -> bool {
        *self == Polarity15_0Dio4::Polarity15_0Dio4Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio4_rise(&self) -> bool {
        *self == Polarity15_0Dio4::Polarity15_0Dio4Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio4_fall(&self) -> bool {
        *self == Polarity15_0Dio4::Polarity15_0Dio4Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio4_rise_fall(&self) -> bool {
        *self == Polarity15_0Dio4::Polarity15_0Dio4RiseFall
    }
}
#[doc = "Field `POLARITY15_0_DIO4` writer - Enables and configures edge detection polarity for DIO4."]
pub type Polarity15_0Dio4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Polarity15_0Dio4, crate::Safe>;
impl<'a, REG> Polarity15_0Dio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity15_0_dio4_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio4::Polarity15_0Dio4Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity15_0_dio4_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio4::Polarity15_0Dio4Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio4_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio4::Polarity15_0Dio4Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio4_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio4::Polarity15_0Dio4RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity15_0Dio5 {
    #[doc = "0: DISABLE"]
    Polarity15_0Dio5Disable = 0,
    #[doc = "1: RISE"]
    Polarity15_0Dio5Rise = 1,
    #[doc = "2: FALL"]
    Polarity15_0Dio5Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity15_0Dio5RiseFall = 3,
}
impl From<Polarity15_0Dio5> for u8 {
    #[inline(always)]
    fn from(variant: Polarity15_0Dio5) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity15_0Dio5 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity15_0Dio5 {}
#[doc = "Field `POLARITY15_0_DIO5` reader - Enables and configures edge detection polarity for DIO5."]
pub type Polarity15_0Dio5R = crate::FieldReader<Polarity15_0Dio5>;
impl Polarity15_0Dio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity15_0Dio5 {
        match self.bits {
            0 => Polarity15_0Dio5::Polarity15_0Dio5Disable,
            1 => Polarity15_0Dio5::Polarity15_0Dio5Rise,
            2 => Polarity15_0Dio5::Polarity15_0Dio5Fall,
            3 => Polarity15_0Dio5::Polarity15_0Dio5RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio5_disable(&self) -> bool {
        *self == Polarity15_0Dio5::Polarity15_0Dio5Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio5_rise(&self) -> bool {
        *self == Polarity15_0Dio5::Polarity15_0Dio5Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio5_fall(&self) -> bool {
        *self == Polarity15_0Dio5::Polarity15_0Dio5Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio5_rise_fall(&self) -> bool {
        *self == Polarity15_0Dio5::Polarity15_0Dio5RiseFall
    }
}
#[doc = "Field `POLARITY15_0_DIO5` writer - Enables and configures edge detection polarity for DIO5."]
pub type Polarity15_0Dio5W<'a, REG> = crate::FieldWriter<'a, REG, 2, Polarity15_0Dio5, crate::Safe>;
impl<'a, REG> Polarity15_0Dio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity15_0_dio5_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio5::Polarity15_0Dio5Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity15_0_dio5_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio5::Polarity15_0Dio5Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio5_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio5::Polarity15_0Dio5Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio5_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio5::Polarity15_0Dio5RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity15_0Dio6 {
    #[doc = "0: DISABLE"]
    Polarity15_0Dio6Disable = 0,
    #[doc = "1: RISE"]
    Polarity15_0Dio6Rise = 1,
    #[doc = "2: FALL"]
    Polarity15_0Dio6Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity15_0Dio6RiseFall = 3,
}
impl From<Polarity15_0Dio6> for u8 {
    #[inline(always)]
    fn from(variant: Polarity15_0Dio6) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity15_0Dio6 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity15_0Dio6 {}
#[doc = "Field `POLARITY15_0_DIO6` reader - Enables and configures edge detection polarity for DIO6."]
pub type Polarity15_0Dio6R = crate::FieldReader<Polarity15_0Dio6>;
impl Polarity15_0Dio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity15_0Dio6 {
        match self.bits {
            0 => Polarity15_0Dio6::Polarity15_0Dio6Disable,
            1 => Polarity15_0Dio6::Polarity15_0Dio6Rise,
            2 => Polarity15_0Dio6::Polarity15_0Dio6Fall,
            3 => Polarity15_0Dio6::Polarity15_0Dio6RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio6_disable(&self) -> bool {
        *self == Polarity15_0Dio6::Polarity15_0Dio6Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio6_rise(&self) -> bool {
        *self == Polarity15_0Dio6::Polarity15_0Dio6Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio6_fall(&self) -> bool {
        *self == Polarity15_0Dio6::Polarity15_0Dio6Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio6_rise_fall(&self) -> bool {
        *self == Polarity15_0Dio6::Polarity15_0Dio6RiseFall
    }
}
#[doc = "Field `POLARITY15_0_DIO6` writer - Enables and configures edge detection polarity for DIO6."]
pub type Polarity15_0Dio6W<'a, REG> = crate::FieldWriter<'a, REG, 2, Polarity15_0Dio6, crate::Safe>;
impl<'a, REG> Polarity15_0Dio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity15_0_dio6_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio6::Polarity15_0Dio6Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity15_0_dio6_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio6::Polarity15_0Dio6Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio6_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio6::Polarity15_0Dio6Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio6_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio6::Polarity15_0Dio6RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity15_0Dio7 {
    #[doc = "0: DISABLE"]
    Polarity15_0Dio7Disable = 0,
    #[doc = "1: RISE"]
    Polarity15_0Dio7Rise = 1,
    #[doc = "2: FALL"]
    Polarity15_0Dio7Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity15_0Dio7RiseFall = 3,
}
impl From<Polarity15_0Dio7> for u8 {
    #[inline(always)]
    fn from(variant: Polarity15_0Dio7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity15_0Dio7 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity15_0Dio7 {}
#[doc = "Field `POLARITY15_0_DIO7` reader - Enables and configures edge detection polarity for DIO7."]
pub type Polarity15_0Dio7R = crate::FieldReader<Polarity15_0Dio7>;
impl Polarity15_0Dio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity15_0Dio7 {
        match self.bits {
            0 => Polarity15_0Dio7::Polarity15_0Dio7Disable,
            1 => Polarity15_0Dio7::Polarity15_0Dio7Rise,
            2 => Polarity15_0Dio7::Polarity15_0Dio7Fall,
            3 => Polarity15_0Dio7::Polarity15_0Dio7RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio7_disable(&self) -> bool {
        *self == Polarity15_0Dio7::Polarity15_0Dio7Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio7_rise(&self) -> bool {
        *self == Polarity15_0Dio7::Polarity15_0Dio7Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio7_fall(&self) -> bool {
        *self == Polarity15_0Dio7::Polarity15_0Dio7Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio7_rise_fall(&self) -> bool {
        *self == Polarity15_0Dio7::Polarity15_0Dio7RiseFall
    }
}
#[doc = "Field `POLARITY15_0_DIO7` writer - Enables and configures edge detection polarity for DIO7."]
pub type Polarity15_0Dio7W<'a, REG> = crate::FieldWriter<'a, REG, 2, Polarity15_0Dio7, crate::Safe>;
impl<'a, REG> Polarity15_0Dio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity15_0_dio7_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio7::Polarity15_0Dio7Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity15_0_dio7_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio7::Polarity15_0Dio7Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio7_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio7::Polarity15_0Dio7Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio7_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio7::Polarity15_0Dio7RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity15_0Dio8 {
    #[doc = "0: DISABLE"]
    Polarity15_0Dio8Disable = 0,
    #[doc = "1: RISE"]
    Polarity15_0Dio8Rise = 1,
    #[doc = "2: FALL"]
    Polarity15_0Dio8Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity15_0Dio8RiseFall = 3,
}
impl From<Polarity15_0Dio8> for u8 {
    #[inline(always)]
    fn from(variant: Polarity15_0Dio8) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity15_0Dio8 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity15_0Dio8 {}
#[doc = "Field `POLARITY15_0_DIO8` reader - Enables and configures edge detection polarity for DIO8."]
pub type Polarity15_0Dio8R = crate::FieldReader<Polarity15_0Dio8>;
impl Polarity15_0Dio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity15_0Dio8 {
        match self.bits {
            0 => Polarity15_0Dio8::Polarity15_0Dio8Disable,
            1 => Polarity15_0Dio8::Polarity15_0Dio8Rise,
            2 => Polarity15_0Dio8::Polarity15_0Dio8Fall,
            3 => Polarity15_0Dio8::Polarity15_0Dio8RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio8_disable(&self) -> bool {
        *self == Polarity15_0Dio8::Polarity15_0Dio8Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio8_rise(&self) -> bool {
        *self == Polarity15_0Dio8::Polarity15_0Dio8Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio8_fall(&self) -> bool {
        *self == Polarity15_0Dio8::Polarity15_0Dio8Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio8_rise_fall(&self) -> bool {
        *self == Polarity15_0Dio8::Polarity15_0Dio8RiseFall
    }
}
#[doc = "Field `POLARITY15_0_DIO8` writer - Enables and configures edge detection polarity for DIO8."]
pub type Polarity15_0Dio8W<'a, REG> = crate::FieldWriter<'a, REG, 2, Polarity15_0Dio8, crate::Safe>;
impl<'a, REG> Polarity15_0Dio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity15_0_dio8_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio8::Polarity15_0Dio8Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity15_0_dio8_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio8::Polarity15_0Dio8Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio8_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio8::Polarity15_0Dio8Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio8_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio8::Polarity15_0Dio8RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity15_0Dio9 {
    #[doc = "0: DISABLE"]
    Polarity15_0Dio9Disable = 0,
    #[doc = "1: RISE"]
    Polarity15_0Dio9Rise = 1,
    #[doc = "2: FALL"]
    Polarity15_0Dio9Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity15_0Dio9RiseFall = 3,
}
impl From<Polarity15_0Dio9> for u8 {
    #[inline(always)]
    fn from(variant: Polarity15_0Dio9) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity15_0Dio9 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity15_0Dio9 {}
#[doc = "Field `POLARITY15_0_DIO9` reader - Enables and configures edge detection polarity for DIO9."]
pub type Polarity15_0Dio9R = crate::FieldReader<Polarity15_0Dio9>;
impl Polarity15_0Dio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity15_0Dio9 {
        match self.bits {
            0 => Polarity15_0Dio9::Polarity15_0Dio9Disable,
            1 => Polarity15_0Dio9::Polarity15_0Dio9Rise,
            2 => Polarity15_0Dio9::Polarity15_0Dio9Fall,
            3 => Polarity15_0Dio9::Polarity15_0Dio9RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio9_disable(&self) -> bool {
        *self == Polarity15_0Dio9::Polarity15_0Dio9Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio9_rise(&self) -> bool {
        *self == Polarity15_0Dio9::Polarity15_0Dio9Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio9_fall(&self) -> bool {
        *self == Polarity15_0Dio9::Polarity15_0Dio9Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio9_rise_fall(&self) -> bool {
        *self == Polarity15_0Dio9::Polarity15_0Dio9RiseFall
    }
}
#[doc = "Field `POLARITY15_0_DIO9` writer - Enables and configures edge detection polarity for DIO9."]
pub type Polarity15_0Dio9W<'a, REG> = crate::FieldWriter<'a, REG, 2, Polarity15_0Dio9, crate::Safe>;
impl<'a, REG> Polarity15_0Dio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity15_0_dio9_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio9::Polarity15_0Dio9Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity15_0_dio9_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio9::Polarity15_0Dio9Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio9_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio9::Polarity15_0Dio9Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio9_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio9::Polarity15_0Dio9RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity15_0Dio10 {
    #[doc = "0: DISABLE"]
    Polarity15_0Dio10Disable = 0,
    #[doc = "1: RISE"]
    Polarity15_0Dio10Rise = 1,
    #[doc = "2: FALL"]
    Polarity15_0Dio10Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity15_0Dio10RiseFall = 3,
}
impl From<Polarity15_0Dio10> for u8 {
    #[inline(always)]
    fn from(variant: Polarity15_0Dio10) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity15_0Dio10 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity15_0Dio10 {}
#[doc = "Field `POLARITY15_0_DIO10` reader - Enables and configures edge detection polarity for DIO10."]
pub type Polarity15_0Dio10R = crate::FieldReader<Polarity15_0Dio10>;
impl Polarity15_0Dio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity15_0Dio10 {
        match self.bits {
            0 => Polarity15_0Dio10::Polarity15_0Dio10Disable,
            1 => Polarity15_0Dio10::Polarity15_0Dio10Rise,
            2 => Polarity15_0Dio10::Polarity15_0Dio10Fall,
            3 => Polarity15_0Dio10::Polarity15_0Dio10RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio10_disable(&self) -> bool {
        *self == Polarity15_0Dio10::Polarity15_0Dio10Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio10_rise(&self) -> bool {
        *self == Polarity15_0Dio10::Polarity15_0Dio10Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio10_fall(&self) -> bool {
        *self == Polarity15_0Dio10::Polarity15_0Dio10Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio10_rise_fall(&self) -> bool {
        *self == Polarity15_0Dio10::Polarity15_0Dio10RiseFall
    }
}
#[doc = "Field `POLARITY15_0_DIO10` writer - Enables and configures edge detection polarity for DIO10."]
pub type Polarity15_0Dio10W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity15_0Dio10, crate::Safe>;
impl<'a, REG> Polarity15_0Dio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity15_0_dio10_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio10::Polarity15_0Dio10Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity15_0_dio10_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio10::Polarity15_0Dio10Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio10_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio10::Polarity15_0Dio10Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio10_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio10::Polarity15_0Dio10RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity15_0Dio11 {
    #[doc = "0: DISABLE"]
    Polarity15_0Dio11Disable = 0,
    #[doc = "1: RISE"]
    Polarity15_0Dio11Rise = 1,
    #[doc = "2: FALL"]
    Polarity15_0Dio11Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity15_0Dio11RiseFall = 3,
}
impl From<Polarity15_0Dio11> for u8 {
    #[inline(always)]
    fn from(variant: Polarity15_0Dio11) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity15_0Dio11 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity15_0Dio11 {}
#[doc = "Field `POLARITY15_0_DIO11` reader - Enables and configures edge detection polarity for DIO11."]
pub type Polarity15_0Dio11R = crate::FieldReader<Polarity15_0Dio11>;
impl Polarity15_0Dio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity15_0Dio11 {
        match self.bits {
            0 => Polarity15_0Dio11::Polarity15_0Dio11Disable,
            1 => Polarity15_0Dio11::Polarity15_0Dio11Rise,
            2 => Polarity15_0Dio11::Polarity15_0Dio11Fall,
            3 => Polarity15_0Dio11::Polarity15_0Dio11RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio11_disable(&self) -> bool {
        *self == Polarity15_0Dio11::Polarity15_0Dio11Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio11_rise(&self) -> bool {
        *self == Polarity15_0Dio11::Polarity15_0Dio11Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio11_fall(&self) -> bool {
        *self == Polarity15_0Dio11::Polarity15_0Dio11Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio11_rise_fall(&self) -> bool {
        *self == Polarity15_0Dio11::Polarity15_0Dio11RiseFall
    }
}
#[doc = "Field `POLARITY15_0_DIO11` writer - Enables and configures edge detection polarity for DIO11."]
pub type Polarity15_0Dio11W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity15_0Dio11, crate::Safe>;
impl<'a, REG> Polarity15_0Dio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity15_0_dio11_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio11::Polarity15_0Dio11Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity15_0_dio11_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio11::Polarity15_0Dio11Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio11_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio11::Polarity15_0Dio11Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio11_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio11::Polarity15_0Dio11RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity15_0Dio12 {
    #[doc = "0: DISABLE"]
    Polarity15_0Dio12Disable = 0,
    #[doc = "1: RISE"]
    Polarity15_0Dio12Rise = 1,
    #[doc = "2: FALL"]
    Polarity15_0Dio12Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity15_0Dio12RiseFall = 3,
}
impl From<Polarity15_0Dio12> for u8 {
    #[inline(always)]
    fn from(variant: Polarity15_0Dio12) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity15_0Dio12 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity15_0Dio12 {}
#[doc = "Field `POLARITY15_0_DIO12` reader - Enables and configures edge detection polarity for DIO12."]
pub type Polarity15_0Dio12R = crate::FieldReader<Polarity15_0Dio12>;
impl Polarity15_0Dio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity15_0Dio12 {
        match self.bits {
            0 => Polarity15_0Dio12::Polarity15_0Dio12Disable,
            1 => Polarity15_0Dio12::Polarity15_0Dio12Rise,
            2 => Polarity15_0Dio12::Polarity15_0Dio12Fall,
            3 => Polarity15_0Dio12::Polarity15_0Dio12RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio12_disable(&self) -> bool {
        *self == Polarity15_0Dio12::Polarity15_0Dio12Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio12_rise(&self) -> bool {
        *self == Polarity15_0Dio12::Polarity15_0Dio12Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio12_fall(&self) -> bool {
        *self == Polarity15_0Dio12::Polarity15_0Dio12Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio12_rise_fall(&self) -> bool {
        *self == Polarity15_0Dio12::Polarity15_0Dio12RiseFall
    }
}
#[doc = "Field `POLARITY15_0_DIO12` writer - Enables and configures edge detection polarity for DIO12."]
pub type Polarity15_0Dio12W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity15_0Dio12, crate::Safe>;
impl<'a, REG> Polarity15_0Dio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity15_0_dio12_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio12::Polarity15_0Dio12Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity15_0_dio12_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio12::Polarity15_0Dio12Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio12_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio12::Polarity15_0Dio12Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio12_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio12::Polarity15_0Dio12RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity15_0Dio13 {
    #[doc = "0: DISABLE"]
    Polarity15_0Dio13Disable = 0,
    #[doc = "1: RISE"]
    Polarity15_0Dio13Rise = 1,
    #[doc = "2: FALL"]
    Polarity15_0Dio13Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity15_0Dio13RiseFall = 3,
}
impl From<Polarity15_0Dio13> for u8 {
    #[inline(always)]
    fn from(variant: Polarity15_0Dio13) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity15_0Dio13 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity15_0Dio13 {}
#[doc = "Field `POLARITY15_0_DIO13` reader - Enables and configures edge detection polarity for DIO13."]
pub type Polarity15_0Dio13R = crate::FieldReader<Polarity15_0Dio13>;
impl Polarity15_0Dio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity15_0Dio13 {
        match self.bits {
            0 => Polarity15_0Dio13::Polarity15_0Dio13Disable,
            1 => Polarity15_0Dio13::Polarity15_0Dio13Rise,
            2 => Polarity15_0Dio13::Polarity15_0Dio13Fall,
            3 => Polarity15_0Dio13::Polarity15_0Dio13RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio13_disable(&self) -> bool {
        *self == Polarity15_0Dio13::Polarity15_0Dio13Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio13_rise(&self) -> bool {
        *self == Polarity15_0Dio13::Polarity15_0Dio13Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio13_fall(&self) -> bool {
        *self == Polarity15_0Dio13::Polarity15_0Dio13Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio13_rise_fall(&self) -> bool {
        *self == Polarity15_0Dio13::Polarity15_0Dio13RiseFall
    }
}
#[doc = "Field `POLARITY15_0_DIO13` writer - Enables and configures edge detection polarity for DIO13."]
pub type Polarity15_0Dio13W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity15_0Dio13, crate::Safe>;
impl<'a, REG> Polarity15_0Dio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity15_0_dio13_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio13::Polarity15_0Dio13Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity15_0_dio13_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio13::Polarity15_0Dio13Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio13_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio13::Polarity15_0Dio13Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio13_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio13::Polarity15_0Dio13RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity15_0Dio14 {
    #[doc = "0: DISABLE"]
    Polarity15_0Dio14Disable = 0,
    #[doc = "1: RISE"]
    Polarity15_0Dio14Rise = 1,
    #[doc = "2: FALL"]
    Polarity15_0Dio14Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity15_0Dio14RiseFall = 3,
}
impl From<Polarity15_0Dio14> for u8 {
    #[inline(always)]
    fn from(variant: Polarity15_0Dio14) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity15_0Dio14 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity15_0Dio14 {}
#[doc = "Field `POLARITY15_0_DIO14` reader - Enables and configures edge detection polarity for DIO14."]
pub type Polarity15_0Dio14R = crate::FieldReader<Polarity15_0Dio14>;
impl Polarity15_0Dio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity15_0Dio14 {
        match self.bits {
            0 => Polarity15_0Dio14::Polarity15_0Dio14Disable,
            1 => Polarity15_0Dio14::Polarity15_0Dio14Rise,
            2 => Polarity15_0Dio14::Polarity15_0Dio14Fall,
            3 => Polarity15_0Dio14::Polarity15_0Dio14RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio14_disable(&self) -> bool {
        *self == Polarity15_0Dio14::Polarity15_0Dio14Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio14_rise(&self) -> bool {
        *self == Polarity15_0Dio14::Polarity15_0Dio14Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio14_fall(&self) -> bool {
        *self == Polarity15_0Dio14::Polarity15_0Dio14Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio14_rise_fall(&self) -> bool {
        *self == Polarity15_0Dio14::Polarity15_0Dio14RiseFall
    }
}
#[doc = "Field `POLARITY15_0_DIO14` writer - Enables and configures edge detection polarity for DIO14."]
pub type Polarity15_0Dio14W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity15_0Dio14, crate::Safe>;
impl<'a, REG> Polarity15_0Dio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity15_0_dio14_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio14::Polarity15_0Dio14Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity15_0_dio14_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio14::Polarity15_0Dio14Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio14_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio14::Polarity15_0Dio14Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio14_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio14::Polarity15_0Dio14RiseFall)
    }
}
#[doc = "Enables and configures edge detection polarity for DIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Polarity15_0Dio15 {
    #[doc = "0: DISABLE"]
    Polarity15_0Dio15Disable = 0,
    #[doc = "1: RISE"]
    Polarity15_0Dio15Rise = 1,
    #[doc = "2: FALL"]
    Polarity15_0Dio15Fall = 2,
    #[doc = "3: RISE_FALL"]
    Polarity15_0Dio15RiseFall = 3,
}
impl From<Polarity15_0Dio15> for u8 {
    #[inline(always)]
    fn from(variant: Polarity15_0Dio15) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Polarity15_0Dio15 {
    type Ux = u8;
}
impl crate::IsEnum for Polarity15_0Dio15 {}
#[doc = "Field `POLARITY15_0_DIO15` reader - Enables and configures edge detection polarity for DIO15."]
pub type Polarity15_0Dio15R = crate::FieldReader<Polarity15_0Dio15>;
impl Polarity15_0Dio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Polarity15_0Dio15 {
        match self.bits {
            0 => Polarity15_0Dio15::Polarity15_0Dio15Disable,
            1 => Polarity15_0Dio15::Polarity15_0Dio15Rise,
            2 => Polarity15_0Dio15::Polarity15_0Dio15Fall,
            3 => Polarity15_0Dio15::Polarity15_0Dio15RiseFall,
            _ => unreachable!(),
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio15_disable(&self) -> bool {
        *self == Polarity15_0Dio15::Polarity15_0Dio15Disable
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn is_polarity15_0_dio15_rise(&self) -> bool {
        *self == Polarity15_0Dio15::Polarity15_0Dio15Rise
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio15_fall(&self) -> bool {
        *self == Polarity15_0Dio15::Polarity15_0Dio15Fall
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn is_polarity15_0_dio15_rise_fall(&self) -> bool {
        *self == Polarity15_0Dio15::Polarity15_0Dio15RiseFall
    }
}
#[doc = "Field `POLARITY15_0_DIO15` writer - Enables and configures edge detection polarity for DIO15."]
pub type Polarity15_0Dio15W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, Polarity15_0Dio15, crate::Safe>;
impl<'a, REG> Polarity15_0Dio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn polarity15_0_dio15_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio15::Polarity15_0Dio15Disable)
    }
    #[doc = "RISE"]
    #[inline(always)]
    pub fn polarity15_0_dio15_rise(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio15::Polarity15_0Dio15Rise)
    }
    #[doc = "FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio15_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio15::Polarity15_0Dio15Fall)
    }
    #[doc = "RISE_FALL"]
    #[inline(always)]
    pub fn polarity15_0_dio15_rise_fall(self) -> &'a mut crate::W<REG> {
        self.variant(Polarity15_0Dio15::Polarity15_0Dio15RiseFall)
    }
}
impl R {
    #[doc = "Bits 0:1 - Enables and configures edge detection polarity for DIO0."]
    #[inline(always)]
    pub fn polarity15_0_dio0(&self) -> Polarity15_0Dio0R {
        Polarity15_0Dio0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Enables and configures edge detection polarity for DIO1."]
    #[inline(always)]
    pub fn polarity15_0_dio1(&self) -> Polarity15_0Dio1R {
        Polarity15_0Dio1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Enables and configures edge detection polarity for DIO2."]
    #[inline(always)]
    pub fn polarity15_0_dio2(&self) -> Polarity15_0Dio2R {
        Polarity15_0Dio2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Enables and configures edge detection polarity for DIO3."]
    #[inline(always)]
    pub fn polarity15_0_dio3(&self) -> Polarity15_0Dio3R {
        Polarity15_0Dio3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Enables and configures edge detection polarity for DIO4."]
    #[inline(always)]
    pub fn polarity15_0_dio4(&self) -> Polarity15_0Dio4R {
        Polarity15_0Dio4R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Enables and configures edge detection polarity for DIO5."]
    #[inline(always)]
    pub fn polarity15_0_dio5(&self) -> Polarity15_0Dio5R {
        Polarity15_0Dio5R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Enables and configures edge detection polarity for DIO6."]
    #[inline(always)]
    pub fn polarity15_0_dio6(&self) -> Polarity15_0Dio6R {
        Polarity15_0Dio6R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Enables and configures edge detection polarity for DIO7."]
    #[inline(always)]
    pub fn polarity15_0_dio7(&self) -> Polarity15_0Dio7R {
        Polarity15_0Dio7R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Enables and configures edge detection polarity for DIO8."]
    #[inline(always)]
    pub fn polarity15_0_dio8(&self) -> Polarity15_0Dio8R {
        Polarity15_0Dio8R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Enables and configures edge detection polarity for DIO9."]
    #[inline(always)]
    pub fn polarity15_0_dio9(&self) -> Polarity15_0Dio9R {
        Polarity15_0Dio9R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Enables and configures edge detection polarity for DIO10."]
    #[inline(always)]
    pub fn polarity15_0_dio10(&self) -> Polarity15_0Dio10R {
        Polarity15_0Dio10R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Enables and configures edge detection polarity for DIO11."]
    #[inline(always)]
    pub fn polarity15_0_dio11(&self) -> Polarity15_0Dio11R {
        Polarity15_0Dio11R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Enables and configures edge detection polarity for DIO12."]
    #[inline(always)]
    pub fn polarity15_0_dio12(&self) -> Polarity15_0Dio12R {
        Polarity15_0Dio12R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Enables and configures edge detection polarity for DIO13."]
    #[inline(always)]
    pub fn polarity15_0_dio13(&self) -> Polarity15_0Dio13R {
        Polarity15_0Dio13R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Enables and configures edge detection polarity for DIO14."]
    #[inline(always)]
    pub fn polarity15_0_dio14(&self) -> Polarity15_0Dio14R {
        Polarity15_0Dio14R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Enables and configures edge detection polarity for DIO15."]
    #[inline(always)]
    pub fn polarity15_0_dio15(&self) -> Polarity15_0Dio15R {
        Polarity15_0Dio15R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Enables and configures edge detection polarity for DIO0."]
    #[inline(always)]
    pub fn polarity15_0_dio0(&mut self) -> Polarity15_0Dio0W<Polarity15_0Spec> {
        Polarity15_0Dio0W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Enables and configures edge detection polarity for DIO1."]
    #[inline(always)]
    pub fn polarity15_0_dio1(&mut self) -> Polarity15_0Dio1W<Polarity15_0Spec> {
        Polarity15_0Dio1W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Enables and configures edge detection polarity for DIO2."]
    #[inline(always)]
    pub fn polarity15_0_dio2(&mut self) -> Polarity15_0Dio2W<Polarity15_0Spec> {
        Polarity15_0Dio2W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Enables and configures edge detection polarity for DIO3."]
    #[inline(always)]
    pub fn polarity15_0_dio3(&mut self) -> Polarity15_0Dio3W<Polarity15_0Spec> {
        Polarity15_0Dio3W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Enables and configures edge detection polarity for DIO4."]
    #[inline(always)]
    pub fn polarity15_0_dio4(&mut self) -> Polarity15_0Dio4W<Polarity15_0Spec> {
        Polarity15_0Dio4W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Enables and configures edge detection polarity for DIO5."]
    #[inline(always)]
    pub fn polarity15_0_dio5(&mut self) -> Polarity15_0Dio5W<Polarity15_0Spec> {
        Polarity15_0Dio5W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Enables and configures edge detection polarity for DIO6."]
    #[inline(always)]
    pub fn polarity15_0_dio6(&mut self) -> Polarity15_0Dio6W<Polarity15_0Spec> {
        Polarity15_0Dio6W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Enables and configures edge detection polarity for DIO7."]
    #[inline(always)]
    pub fn polarity15_0_dio7(&mut self) -> Polarity15_0Dio7W<Polarity15_0Spec> {
        Polarity15_0Dio7W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Enables and configures edge detection polarity for DIO8."]
    #[inline(always)]
    pub fn polarity15_0_dio8(&mut self) -> Polarity15_0Dio8W<Polarity15_0Spec> {
        Polarity15_0Dio8W::new(self, 16)
    }
    #[doc = "Bits 18:19 - Enables and configures edge detection polarity for DIO9."]
    #[inline(always)]
    pub fn polarity15_0_dio9(&mut self) -> Polarity15_0Dio9W<Polarity15_0Spec> {
        Polarity15_0Dio9W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Enables and configures edge detection polarity for DIO10."]
    #[inline(always)]
    pub fn polarity15_0_dio10(&mut self) -> Polarity15_0Dio10W<Polarity15_0Spec> {
        Polarity15_0Dio10W::new(self, 20)
    }
    #[doc = "Bits 22:23 - Enables and configures edge detection polarity for DIO11."]
    #[inline(always)]
    pub fn polarity15_0_dio11(&mut self) -> Polarity15_0Dio11W<Polarity15_0Spec> {
        Polarity15_0Dio11W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Enables and configures edge detection polarity for DIO12."]
    #[inline(always)]
    pub fn polarity15_0_dio12(&mut self) -> Polarity15_0Dio12W<Polarity15_0Spec> {
        Polarity15_0Dio12W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Enables and configures edge detection polarity for DIO13."]
    #[inline(always)]
    pub fn polarity15_0_dio13(&mut self) -> Polarity15_0Dio13W<Polarity15_0Spec> {
        Polarity15_0Dio13W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Enables and configures edge detection polarity for DIO14."]
    #[inline(always)]
    pub fn polarity15_0_dio14(&mut self) -> Polarity15_0Dio14W<Polarity15_0Spec> {
        Polarity15_0Dio14W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Enables and configures edge detection polarity for DIO15."]
    #[inline(always)]
    pub fn polarity15_0_dio15(&mut self) -> Polarity15_0Dio15W<Polarity15_0Spec> {
        Polarity15_0Dio15W::new(self, 30)
    }
}
#[doc = "Polarity 15 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`polarity15_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`polarity15_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Polarity15_0Spec;
impl crate::RegisterSpec for Polarity15_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`polarity15_0::R`](R) reader structure"]
impl crate::Readable for Polarity15_0Spec {}
#[doc = "`write(|w| ..)` method takes [`polarity15_0::W`](W) writer structure"]
impl crate::Writable for Polarity15_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POLARITY15_0 to value 0"]
impl crate::Resettable for Polarity15_0Spec {
    const RESET_VALUE: u32 = 0;
}
