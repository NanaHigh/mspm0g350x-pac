#[doc = "Register `DIN3_0` reader"]
pub type R = crate::R<Din3_0Spec>;
#[doc = "This bit reads the data input value of DIO0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din3_0Dio0 {
    #[doc = "0: ZERO"]
    Din3_0Dio0Zero = 0,
    #[doc = "1: ONE"]
    Din3_0Dio0One = 1,
}
impl From<Din3_0Dio0> for bool {
    #[inline(always)]
    fn from(variant: Din3_0Dio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN3_0_DIO0` reader - This bit reads the data input value of DIO0."]
pub type Din3_0Dio0R = crate::BitReader<Din3_0Dio0>;
impl Din3_0Dio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din3_0Dio0 {
        match self.bits {
            false => Din3_0Dio0::Din3_0Dio0Zero,
            true => Din3_0Dio0::Din3_0Dio0One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din3_0_dio0_zero(&self) -> bool {
        *self == Din3_0Dio0::Din3_0Dio0Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din3_0_dio0_one(&self) -> bool {
        *self == Din3_0Dio0::Din3_0Dio0One
    }
}
#[doc = "This bit reads the data input value of DIO1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din3_0Dio1 {
    #[doc = "0: ZERO"]
    Din3_0Dio1Zero = 0,
    #[doc = "1: ONE"]
    Din3_0Dio1One = 1,
}
impl From<Din3_0Dio1> for bool {
    #[inline(always)]
    fn from(variant: Din3_0Dio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN3_0_DIO1` reader - This bit reads the data input value of DIO1."]
pub type Din3_0Dio1R = crate::BitReader<Din3_0Dio1>;
impl Din3_0Dio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din3_0Dio1 {
        match self.bits {
            false => Din3_0Dio1::Din3_0Dio1Zero,
            true => Din3_0Dio1::Din3_0Dio1One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din3_0_dio1_zero(&self) -> bool {
        *self == Din3_0Dio1::Din3_0Dio1Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din3_0_dio1_one(&self) -> bool {
        *self == Din3_0Dio1::Din3_0Dio1One
    }
}
#[doc = "This bit reads the data input value of DIO2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din3_0Dio2 {
    #[doc = "0: ZERO"]
    Din3_0Dio2Zero = 0,
    #[doc = "1: ONE"]
    Din3_0Dio2One = 1,
}
impl From<Din3_0Dio2> for bool {
    #[inline(always)]
    fn from(variant: Din3_0Dio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN3_0_DIO2` reader - This bit reads the data input value of DIO2."]
pub type Din3_0Dio2R = crate::BitReader<Din3_0Dio2>;
impl Din3_0Dio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din3_0Dio2 {
        match self.bits {
            false => Din3_0Dio2::Din3_0Dio2Zero,
            true => Din3_0Dio2::Din3_0Dio2One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din3_0_dio2_zero(&self) -> bool {
        *self == Din3_0Dio2::Din3_0Dio2Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din3_0_dio2_one(&self) -> bool {
        *self == Din3_0Dio2::Din3_0Dio2One
    }
}
#[doc = "This bit reads the data input value of DIO3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din3_0Dio3 {
    #[doc = "0: ZERO"]
    Din3_0Dio3Zero = 0,
    #[doc = "1: ONE"]
    Din3_0Dio3One = 1,
}
impl From<Din3_0Dio3> for bool {
    #[inline(always)]
    fn from(variant: Din3_0Dio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN3_0_DIO3` reader - This bit reads the data input value of DIO3."]
pub type Din3_0Dio3R = crate::BitReader<Din3_0Dio3>;
impl Din3_0Dio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din3_0Dio3 {
        match self.bits {
            false => Din3_0Dio3::Din3_0Dio3Zero,
            true => Din3_0Dio3::Din3_0Dio3One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din3_0_dio3_zero(&self) -> bool {
        *self == Din3_0Dio3::Din3_0Dio3Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din3_0_dio3_one(&self) -> bool {
        *self == Din3_0Dio3::Din3_0Dio3One
    }
}
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of DIO0."]
    #[inline(always)]
    pub fn din3_0_dio0(&self) -> Din3_0Dio0R {
        Din3_0Dio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of DIO1."]
    #[inline(always)]
    pub fn din3_0_dio1(&self) -> Din3_0Dio1R {
        Din3_0Dio1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of DIO2."]
    #[inline(always)]
    pub fn din3_0_dio2(&self) -> Din3_0Dio2R {
        Din3_0Dio2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of DIO3."]
    #[inline(always)]
    pub fn din3_0_dio3(&self) -> Din3_0Dio3R {
        Din3_0Dio3R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Data input 3 to 0\n\nYou can [`read`](crate::Reg::read) this register and get [`din3_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Din3_0Spec;
impl crate::RegisterSpec for Din3_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din3_0::R`](R) reader structure"]
impl crate::Readable for Din3_0Spec {}
#[doc = "`reset()` method sets DIN3_0 to value 0"]
impl crate::Resettable for Din3_0Spec {
    const RESET_VALUE: u32 = 0;
}
