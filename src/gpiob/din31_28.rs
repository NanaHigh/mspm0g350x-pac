#[doc = "Register `DIN31_28` reader"]
pub type R = crate::R<Din31_28Spec>;
#[doc = "This bit reads the data input value of DIO28.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_28Dio28 {
    #[doc = "0: ZERO"]
    Din31_28Dio28Zero = 0,
    #[doc = "1: ONE"]
    Din31_28Dio28One = 1,
}
impl From<Din31_28Dio28> for bool {
    #[inline(always)]
    fn from(variant: Din31_28Dio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_28_DIO28` reader - This bit reads the data input value of DIO28."]
pub type Din31_28Dio28R = crate::BitReader<Din31_28Dio28>;
impl Din31_28Dio28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_28Dio28 {
        match self.bits {
            false => Din31_28Dio28::Din31_28Dio28Zero,
            true => Din31_28Dio28::Din31_28Dio28One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_28_dio28_zero(&self) -> bool {
        *self == Din31_28Dio28::Din31_28Dio28Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_28_dio28_one(&self) -> bool {
        *self == Din31_28Dio28::Din31_28Dio28One
    }
}
#[doc = "This bit reads the data input value of DIO29.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_28Dio29 {
    #[doc = "0: ZERO"]
    Din31_28Dio29Zero = 0,
    #[doc = "1: ONE"]
    Din31_28Dio29One = 1,
}
impl From<Din31_28Dio29> for bool {
    #[inline(always)]
    fn from(variant: Din31_28Dio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_28_DIO29` reader - This bit reads the data input value of DIO29."]
pub type Din31_28Dio29R = crate::BitReader<Din31_28Dio29>;
impl Din31_28Dio29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_28Dio29 {
        match self.bits {
            false => Din31_28Dio29::Din31_28Dio29Zero,
            true => Din31_28Dio29::Din31_28Dio29One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_28_dio29_zero(&self) -> bool {
        *self == Din31_28Dio29::Din31_28Dio29Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_28_dio29_one(&self) -> bool {
        *self == Din31_28Dio29::Din31_28Dio29One
    }
}
#[doc = "This bit reads the data input value of DIO30.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_28Dio30 {
    #[doc = "0: ZERO"]
    Din31_28Dio30Zero = 0,
    #[doc = "1: ONE"]
    Din31_28Dio30One = 1,
}
impl From<Din31_28Dio30> for bool {
    #[inline(always)]
    fn from(variant: Din31_28Dio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_28_DIO30` reader - This bit reads the data input value of DIO30."]
pub type Din31_28Dio30R = crate::BitReader<Din31_28Dio30>;
impl Din31_28Dio30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_28Dio30 {
        match self.bits {
            false => Din31_28Dio30::Din31_28Dio30Zero,
            true => Din31_28Dio30::Din31_28Dio30One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_28_dio30_zero(&self) -> bool {
        *self == Din31_28Dio30::Din31_28Dio30Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_28_dio30_one(&self) -> bool {
        *self == Din31_28Dio30::Din31_28Dio30One
    }
}
#[doc = "This bit reads the data input value of DIO31.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din31_28Dio31 {
    #[doc = "0: ZERO"]
    Din31_28Dio31Zero = 0,
    #[doc = "1: ONE"]
    Din31_28Dio31One = 1,
}
impl From<Din31_28Dio31> for bool {
    #[inline(always)]
    fn from(variant: Din31_28Dio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN31_28_DIO31` reader - This bit reads the data input value of DIO31."]
pub type Din31_28Dio31R = crate::BitReader<Din31_28Dio31>;
impl Din31_28Dio31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din31_28Dio31 {
        match self.bits {
            false => Din31_28Dio31::Din31_28Dio31Zero,
            true => Din31_28Dio31::Din31_28Dio31One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din31_28_dio31_zero(&self) -> bool {
        *self == Din31_28Dio31::Din31_28Dio31Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din31_28_dio31_one(&self) -> bool {
        *self == Din31_28Dio31::Din31_28Dio31One
    }
}
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of DIO28."]
    #[inline(always)]
    pub fn din31_28_dio28(&self) -> Din31_28Dio28R {
        Din31_28Dio28R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of DIO29."]
    #[inline(always)]
    pub fn din31_28_dio29(&self) -> Din31_28Dio29R {
        Din31_28Dio29R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of DIO30."]
    #[inline(always)]
    pub fn din31_28_dio30(&self) -> Din31_28Dio30R {
        Din31_28Dio30R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of DIO31."]
    #[inline(always)]
    pub fn din31_28_dio31(&self) -> Din31_28Dio31R {
        Din31_28Dio31R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Data input 31 to 28\n\nYou can [`read`](crate::Reg::read) this register and get [`din31_28::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Din31_28Spec;
impl crate::RegisterSpec for Din31_28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din31_28::R`](R) reader structure"]
impl crate::Readable for Din31_28Spec {}
#[doc = "`reset()` method sets DIN31_28 to value 0"]
impl crate::Resettable for Din31_28Spec {
    const RESET_VALUE: u32 = 0;
}
