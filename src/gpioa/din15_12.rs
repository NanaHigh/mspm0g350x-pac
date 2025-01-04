#[doc = "Register `DIN15_12` reader"]
pub type R = crate::R<Din15_12Spec>;
#[doc = "This bit reads the data input value of DIO12.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din15_12Dio12 {
    #[doc = "0: ZERO"]
    Din15_12Dio12Zero = 0,
    #[doc = "1: ONE"]
    Din15_12Dio12One = 1,
}
impl From<Din15_12Dio12> for bool {
    #[inline(always)]
    fn from(variant: Din15_12Dio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN15_12_DIO12` reader - This bit reads the data input value of DIO12."]
pub type Din15_12Dio12R = crate::BitReader<Din15_12Dio12>;
impl Din15_12Dio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din15_12Dio12 {
        match self.bits {
            false => Din15_12Dio12::Din15_12Dio12Zero,
            true => Din15_12Dio12::Din15_12Dio12One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din15_12_dio12_zero(&self) -> bool {
        *self == Din15_12Dio12::Din15_12Dio12Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din15_12_dio12_one(&self) -> bool {
        *self == Din15_12Dio12::Din15_12Dio12One
    }
}
#[doc = "This bit reads the data input value of DIO13.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din15_12Dio13 {
    #[doc = "0: ZERO"]
    Din15_12Dio13Zero = 0,
    #[doc = "1: ONE"]
    Din15_12Dio13One = 1,
}
impl From<Din15_12Dio13> for bool {
    #[inline(always)]
    fn from(variant: Din15_12Dio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN15_12_DIO13` reader - This bit reads the data input value of DIO13."]
pub type Din15_12Dio13R = crate::BitReader<Din15_12Dio13>;
impl Din15_12Dio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din15_12Dio13 {
        match self.bits {
            false => Din15_12Dio13::Din15_12Dio13Zero,
            true => Din15_12Dio13::Din15_12Dio13One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din15_12_dio13_zero(&self) -> bool {
        *self == Din15_12Dio13::Din15_12Dio13Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din15_12_dio13_one(&self) -> bool {
        *self == Din15_12Dio13::Din15_12Dio13One
    }
}
#[doc = "This bit reads the data input value of DIO14.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din15_12Dio14 {
    #[doc = "0: ZERO"]
    Din15_12Dio14Zero = 0,
    #[doc = "1: ONE"]
    Din15_12Dio14One = 1,
}
impl From<Din15_12Dio14> for bool {
    #[inline(always)]
    fn from(variant: Din15_12Dio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN15_12_DIO14` reader - This bit reads the data input value of DIO14."]
pub type Din15_12Dio14R = crate::BitReader<Din15_12Dio14>;
impl Din15_12Dio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din15_12Dio14 {
        match self.bits {
            false => Din15_12Dio14::Din15_12Dio14Zero,
            true => Din15_12Dio14::Din15_12Dio14One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din15_12_dio14_zero(&self) -> bool {
        *self == Din15_12Dio14::Din15_12Dio14Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din15_12_dio14_one(&self) -> bool {
        *self == Din15_12Dio14::Din15_12Dio14One
    }
}
#[doc = "This bit reads the data input value of DIO15.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din15_12Dio15 {
    #[doc = "0: ZERO"]
    Din15_12Dio15Zero = 0,
    #[doc = "1: ONE"]
    Din15_12Dio15One = 1,
}
impl From<Din15_12Dio15> for bool {
    #[inline(always)]
    fn from(variant: Din15_12Dio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN15_12_DIO15` reader - This bit reads the data input value of DIO15."]
pub type Din15_12Dio15R = crate::BitReader<Din15_12Dio15>;
impl Din15_12Dio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din15_12Dio15 {
        match self.bits {
            false => Din15_12Dio15::Din15_12Dio15Zero,
            true => Din15_12Dio15::Din15_12Dio15One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din15_12_dio15_zero(&self) -> bool {
        *self == Din15_12Dio15::Din15_12Dio15Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din15_12_dio15_one(&self) -> bool {
        *self == Din15_12Dio15::Din15_12Dio15One
    }
}
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of DIO12."]
    #[inline(always)]
    pub fn din15_12_dio12(&self) -> Din15_12Dio12R {
        Din15_12Dio12R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of DIO13."]
    #[inline(always)]
    pub fn din15_12_dio13(&self) -> Din15_12Dio13R {
        Din15_12Dio13R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of DIO14."]
    #[inline(always)]
    pub fn din15_12_dio14(&self) -> Din15_12Dio14R {
        Din15_12Dio14R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of DIO15."]
    #[inline(always)]
    pub fn din15_12_dio15(&self) -> Din15_12Dio15R {
        Din15_12Dio15R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Data input 15 to 12\n\nYou can [`read`](crate::Reg::read) this register and get [`din15_12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Din15_12Spec;
impl crate::RegisterSpec for Din15_12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din15_12::R`](R) reader structure"]
impl crate::Readable for Din15_12Spec {}
#[doc = "`reset()` method sets DIN15_12 to value 0"]
impl crate::Resettable for Din15_12Spec {
    const RESET_VALUE: u32 = 0;
}
