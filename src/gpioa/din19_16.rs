#[doc = "Register `DIN19_16` reader"]
pub type R = crate::R<Din19_16Spec>;
#[doc = "This bit reads the data input value of DIO16.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din19_16Dio16 {
    #[doc = "0: ZERO"]
    Din19_16Dio16Zero = 0,
    #[doc = "1: ONE"]
    Din19_16Dio16One = 1,
}
impl From<Din19_16Dio16> for bool {
    #[inline(always)]
    fn from(variant: Din19_16Dio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN19_16_DIO16` reader - This bit reads the data input value of DIO16."]
pub type Din19_16Dio16R = crate::BitReader<Din19_16Dio16>;
impl Din19_16Dio16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din19_16Dio16 {
        match self.bits {
            false => Din19_16Dio16::Din19_16Dio16Zero,
            true => Din19_16Dio16::Din19_16Dio16One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din19_16_dio16_zero(&self) -> bool {
        *self == Din19_16Dio16::Din19_16Dio16Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din19_16_dio16_one(&self) -> bool {
        *self == Din19_16Dio16::Din19_16Dio16One
    }
}
#[doc = "This bit reads the data input value of DIO17.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din19_16Dio17 {
    #[doc = "0: ZERO"]
    Din19_16Dio17Zero = 0,
    #[doc = "1: ONE"]
    Din19_16Dio17One = 1,
}
impl From<Din19_16Dio17> for bool {
    #[inline(always)]
    fn from(variant: Din19_16Dio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN19_16_DIO17` reader - This bit reads the data input value of DIO17."]
pub type Din19_16Dio17R = crate::BitReader<Din19_16Dio17>;
impl Din19_16Dio17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din19_16Dio17 {
        match self.bits {
            false => Din19_16Dio17::Din19_16Dio17Zero,
            true => Din19_16Dio17::Din19_16Dio17One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din19_16_dio17_zero(&self) -> bool {
        *self == Din19_16Dio17::Din19_16Dio17Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din19_16_dio17_one(&self) -> bool {
        *self == Din19_16Dio17::Din19_16Dio17One
    }
}
#[doc = "This bit reads the data input value of DIO18.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din19_16Dio18 {
    #[doc = "0: ZERO"]
    Din19_16Dio18Zero = 0,
    #[doc = "1: ONE"]
    Din19_16Dio18One = 1,
}
impl From<Din19_16Dio18> for bool {
    #[inline(always)]
    fn from(variant: Din19_16Dio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN19_16_DIO18` reader - This bit reads the data input value of DIO18."]
pub type Din19_16Dio18R = crate::BitReader<Din19_16Dio18>;
impl Din19_16Dio18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din19_16Dio18 {
        match self.bits {
            false => Din19_16Dio18::Din19_16Dio18Zero,
            true => Din19_16Dio18::Din19_16Dio18One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din19_16_dio18_zero(&self) -> bool {
        *self == Din19_16Dio18::Din19_16Dio18Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din19_16_dio18_one(&self) -> bool {
        *self == Din19_16Dio18::Din19_16Dio18One
    }
}
#[doc = "This bit reads the data input value of DIO19.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din19_16Dio19 {
    #[doc = "0: ZERO"]
    Din19_16Dio19Zero = 0,
    #[doc = "1: ONE"]
    Din19_16Dio19One = 1,
}
impl From<Din19_16Dio19> for bool {
    #[inline(always)]
    fn from(variant: Din19_16Dio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN19_16_DIO19` reader - This bit reads the data input value of DIO19."]
pub type Din19_16Dio19R = crate::BitReader<Din19_16Dio19>;
impl Din19_16Dio19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din19_16Dio19 {
        match self.bits {
            false => Din19_16Dio19::Din19_16Dio19Zero,
            true => Din19_16Dio19::Din19_16Dio19One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din19_16_dio19_zero(&self) -> bool {
        *self == Din19_16Dio19::Din19_16Dio19Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din19_16_dio19_one(&self) -> bool {
        *self == Din19_16Dio19::Din19_16Dio19One
    }
}
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of DIO16."]
    #[inline(always)]
    pub fn din19_16_dio16(&self) -> Din19_16Dio16R {
        Din19_16Dio16R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of DIO17."]
    #[inline(always)]
    pub fn din19_16_dio17(&self) -> Din19_16Dio17R {
        Din19_16Dio17R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of DIO18."]
    #[inline(always)]
    pub fn din19_16_dio18(&self) -> Din19_16Dio18R {
        Din19_16Dio18R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of DIO19."]
    #[inline(always)]
    pub fn din19_16_dio19(&self) -> Din19_16Dio19R {
        Din19_16Dio19R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Data input 19 to 16\n\nYou can [`read`](crate::Reg::read) this register and get [`din19_16::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Din19_16Spec;
impl crate::RegisterSpec for Din19_16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din19_16::R`](R) reader structure"]
impl crate::Readable for Din19_16Spec {}
#[doc = "`reset()` method sets DIN19_16 to value 0"]
impl crate::Resettable for Din19_16Spec {
    const RESET_VALUE: u32 = 0;
}
