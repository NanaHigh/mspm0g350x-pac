#[doc = "Register `DIN23_20` reader"]
pub type R = crate::R<Din23_20Spec>;
#[doc = "This bit reads the data input value of DIO20.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din23_20Dio20 {
    #[doc = "0: ZERO"]
    Din23_20Dio20Zero = 0,
    #[doc = "1: ONE"]
    Din23_20Dio20One = 1,
}
impl From<Din23_20Dio20> for bool {
    #[inline(always)]
    fn from(variant: Din23_20Dio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN23_20_DIO20` reader - This bit reads the data input value of DIO20."]
pub type Din23_20Dio20R = crate::BitReader<Din23_20Dio20>;
impl Din23_20Dio20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din23_20Dio20 {
        match self.bits {
            false => Din23_20Dio20::Din23_20Dio20Zero,
            true => Din23_20Dio20::Din23_20Dio20One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din23_20_dio20_zero(&self) -> bool {
        *self == Din23_20Dio20::Din23_20Dio20Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din23_20_dio20_one(&self) -> bool {
        *self == Din23_20Dio20::Din23_20Dio20One
    }
}
#[doc = "This bit reads the data input value of DIO21.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din23_20Dio21 {
    #[doc = "0: ZERO"]
    Din23_20Dio21Zero = 0,
    #[doc = "1: ONE"]
    Din23_20Dio21One = 1,
}
impl From<Din23_20Dio21> for bool {
    #[inline(always)]
    fn from(variant: Din23_20Dio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN23_20_DIO21` reader - This bit reads the data input value of DIO21."]
pub type Din23_20Dio21R = crate::BitReader<Din23_20Dio21>;
impl Din23_20Dio21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din23_20Dio21 {
        match self.bits {
            false => Din23_20Dio21::Din23_20Dio21Zero,
            true => Din23_20Dio21::Din23_20Dio21One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din23_20_dio21_zero(&self) -> bool {
        *self == Din23_20Dio21::Din23_20Dio21Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din23_20_dio21_one(&self) -> bool {
        *self == Din23_20Dio21::Din23_20Dio21One
    }
}
#[doc = "This bit reads the data input value of DIO22.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din23_20Dio22 {
    #[doc = "0: ZERO"]
    Din23_20Dio22Zero = 0,
    #[doc = "1: ONE"]
    Din23_20Dio22One = 1,
}
impl From<Din23_20Dio22> for bool {
    #[inline(always)]
    fn from(variant: Din23_20Dio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN23_20_DIO22` reader - This bit reads the data input value of DIO22."]
pub type Din23_20Dio22R = crate::BitReader<Din23_20Dio22>;
impl Din23_20Dio22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din23_20Dio22 {
        match self.bits {
            false => Din23_20Dio22::Din23_20Dio22Zero,
            true => Din23_20Dio22::Din23_20Dio22One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din23_20_dio22_zero(&self) -> bool {
        *self == Din23_20Dio22::Din23_20Dio22Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din23_20_dio22_one(&self) -> bool {
        *self == Din23_20Dio22::Din23_20Dio22One
    }
}
#[doc = "This bit reads the data input value of DIO23.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din23_20Dio23 {
    #[doc = "0: ZERO"]
    Din23_20Dio23Zero = 0,
    #[doc = "1: ONE"]
    Din23_20Dio23One = 1,
}
impl From<Din23_20Dio23> for bool {
    #[inline(always)]
    fn from(variant: Din23_20Dio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN23_20_DIO23` reader - This bit reads the data input value of DIO23."]
pub type Din23_20Dio23R = crate::BitReader<Din23_20Dio23>;
impl Din23_20Dio23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din23_20Dio23 {
        match self.bits {
            false => Din23_20Dio23::Din23_20Dio23Zero,
            true => Din23_20Dio23::Din23_20Dio23One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din23_20_dio23_zero(&self) -> bool {
        *self == Din23_20Dio23::Din23_20Dio23Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din23_20_dio23_one(&self) -> bool {
        *self == Din23_20Dio23::Din23_20Dio23One
    }
}
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of DIO20."]
    #[inline(always)]
    pub fn din23_20_dio20(&self) -> Din23_20Dio20R {
        Din23_20Dio20R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of DIO21."]
    #[inline(always)]
    pub fn din23_20_dio21(&self) -> Din23_20Dio21R {
        Din23_20Dio21R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of DIO22."]
    #[inline(always)]
    pub fn din23_20_dio22(&self) -> Din23_20Dio22R {
        Din23_20Dio22R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of DIO23."]
    #[inline(always)]
    pub fn din23_20_dio23(&self) -> Din23_20Dio23R {
        Din23_20Dio23R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Data input 23 to 20\n\nYou can [`read`](crate::Reg::read) this register and get [`din23_20::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Din23_20Spec;
impl crate::RegisterSpec for Din23_20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din23_20::R`](R) reader structure"]
impl crate::Readable for Din23_20Spec {}
#[doc = "`reset()` method sets DIN23_20 to value 0"]
impl crate::Resettable for Din23_20Spec {
    const RESET_VALUE: u32 = 0;
}
