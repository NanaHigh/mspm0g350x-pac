#[doc = "Register `DIN11_8` reader"]
pub type R = crate::R<Din11_8Spec>;
#[doc = "This bit reads the data input value of DIO8.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din11_8Dio8 {
    #[doc = "0: ZERO"]
    Din11_8Dio8Zero = 0,
    #[doc = "1: ONE"]
    Din11_8Dio8One = 1,
}
impl From<Din11_8Dio8> for bool {
    #[inline(always)]
    fn from(variant: Din11_8Dio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN11_8_DIO8` reader - This bit reads the data input value of DIO8."]
pub type Din11_8Dio8R = crate::BitReader<Din11_8Dio8>;
impl Din11_8Dio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din11_8Dio8 {
        match self.bits {
            false => Din11_8Dio8::Din11_8Dio8Zero,
            true => Din11_8Dio8::Din11_8Dio8One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din11_8_dio8_zero(&self) -> bool {
        *self == Din11_8Dio8::Din11_8Dio8Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din11_8_dio8_one(&self) -> bool {
        *self == Din11_8Dio8::Din11_8Dio8One
    }
}
#[doc = "This bit reads the data input value of DIO9.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din11_8Dio9 {
    #[doc = "0: ZERO"]
    Din11_8Dio9Zero = 0,
    #[doc = "1: ONE"]
    Din11_8Dio9One = 1,
}
impl From<Din11_8Dio9> for bool {
    #[inline(always)]
    fn from(variant: Din11_8Dio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN11_8_DIO9` reader - This bit reads the data input value of DIO9."]
pub type Din11_8Dio9R = crate::BitReader<Din11_8Dio9>;
impl Din11_8Dio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din11_8Dio9 {
        match self.bits {
            false => Din11_8Dio9::Din11_8Dio9Zero,
            true => Din11_8Dio9::Din11_8Dio9One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din11_8_dio9_zero(&self) -> bool {
        *self == Din11_8Dio9::Din11_8Dio9Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din11_8_dio9_one(&self) -> bool {
        *self == Din11_8Dio9::Din11_8Dio9One
    }
}
#[doc = "This bit reads the data input value of DIO10.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din11_8Dio10 {
    #[doc = "0: ZERO"]
    Din11_8Dio10Zero = 0,
    #[doc = "1: ONE"]
    Din11_8Dio10One = 1,
}
impl From<Din11_8Dio10> for bool {
    #[inline(always)]
    fn from(variant: Din11_8Dio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN11_8_DIO10` reader - This bit reads the data input value of DIO10."]
pub type Din11_8Dio10R = crate::BitReader<Din11_8Dio10>;
impl Din11_8Dio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din11_8Dio10 {
        match self.bits {
            false => Din11_8Dio10::Din11_8Dio10Zero,
            true => Din11_8Dio10::Din11_8Dio10One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din11_8_dio10_zero(&self) -> bool {
        *self == Din11_8Dio10::Din11_8Dio10Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din11_8_dio10_one(&self) -> bool {
        *self == Din11_8Dio10::Din11_8Dio10One
    }
}
#[doc = "This bit reads the data input value of DIO11.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din11_8Dio11 {
    #[doc = "0: ZERO"]
    Din11_8Dio11Zero = 0,
    #[doc = "1: ONE"]
    Din11_8Dio11One = 1,
}
impl From<Din11_8Dio11> for bool {
    #[inline(always)]
    fn from(variant: Din11_8Dio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN11_8_DIO11` reader - This bit reads the data input value of DIO11."]
pub type Din11_8Dio11R = crate::BitReader<Din11_8Dio11>;
impl Din11_8Dio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din11_8Dio11 {
        match self.bits {
            false => Din11_8Dio11::Din11_8Dio11Zero,
            true => Din11_8Dio11::Din11_8Dio11One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din11_8_dio11_zero(&self) -> bool {
        *self == Din11_8Dio11::Din11_8Dio11Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din11_8_dio11_one(&self) -> bool {
        *self == Din11_8Dio11::Din11_8Dio11One
    }
}
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of DIO8."]
    #[inline(always)]
    pub fn din11_8_dio8(&self) -> Din11_8Dio8R {
        Din11_8Dio8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of DIO9."]
    #[inline(always)]
    pub fn din11_8_dio9(&self) -> Din11_8Dio9R {
        Din11_8Dio9R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of DIO10."]
    #[inline(always)]
    pub fn din11_8_dio10(&self) -> Din11_8Dio10R {
        Din11_8Dio10R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of DIO11."]
    #[inline(always)]
    pub fn din11_8_dio11(&self) -> Din11_8Dio11R {
        Din11_8Dio11R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Data input 11 to 8\n\nYou can [`read`](crate::Reg::read) this register and get [`din11_8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Din11_8Spec;
impl crate::RegisterSpec for Din11_8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din11_8::R`](R) reader structure"]
impl crate::Readable for Din11_8Spec {}
#[doc = "`reset()` method sets DIN11_8 to value 0"]
impl crate::Resettable for Din11_8Spec {
    const RESET_VALUE: u32 = 0;
}
