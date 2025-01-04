#[doc = "Register `DIN27_24` reader"]
pub type R = crate::R<Din27_24Spec>;
#[doc = "This bit reads the data input value of DIO24.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din27_24Dio24 {
    #[doc = "0: ZERO"]
    Din27_24Dio24Zero = 0,
    #[doc = "1: ONE"]
    Din27_24Dio24One = 1,
}
impl From<Din27_24Dio24> for bool {
    #[inline(always)]
    fn from(variant: Din27_24Dio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN27_24_DIO24` reader - This bit reads the data input value of DIO24."]
pub type Din27_24Dio24R = crate::BitReader<Din27_24Dio24>;
impl Din27_24Dio24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din27_24Dio24 {
        match self.bits {
            false => Din27_24Dio24::Din27_24Dio24Zero,
            true => Din27_24Dio24::Din27_24Dio24One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din27_24_dio24_zero(&self) -> bool {
        *self == Din27_24Dio24::Din27_24Dio24Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din27_24_dio24_one(&self) -> bool {
        *self == Din27_24Dio24::Din27_24Dio24One
    }
}
#[doc = "This bit reads the data input value of DIO25.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din27_24Dio25 {
    #[doc = "0: ZERO"]
    Din27_24Dio25Zero = 0,
    #[doc = "1: ONE"]
    Din27_24Dio25One = 1,
}
impl From<Din27_24Dio25> for bool {
    #[inline(always)]
    fn from(variant: Din27_24Dio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN27_24_DIO25` reader - This bit reads the data input value of DIO25."]
pub type Din27_24Dio25R = crate::BitReader<Din27_24Dio25>;
impl Din27_24Dio25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din27_24Dio25 {
        match self.bits {
            false => Din27_24Dio25::Din27_24Dio25Zero,
            true => Din27_24Dio25::Din27_24Dio25One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din27_24_dio25_zero(&self) -> bool {
        *self == Din27_24Dio25::Din27_24Dio25Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din27_24_dio25_one(&self) -> bool {
        *self == Din27_24Dio25::Din27_24Dio25One
    }
}
#[doc = "This bit reads the data input value of DIO26.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din27_24Dio26 {
    #[doc = "0: ZERO"]
    Din27_24Dio26Zero = 0,
    #[doc = "1: ONE"]
    Din27_24Dio26One = 1,
}
impl From<Din27_24Dio26> for bool {
    #[inline(always)]
    fn from(variant: Din27_24Dio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN27_24_DIO26` reader - This bit reads the data input value of DIO26."]
pub type Din27_24Dio26R = crate::BitReader<Din27_24Dio26>;
impl Din27_24Dio26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din27_24Dio26 {
        match self.bits {
            false => Din27_24Dio26::Din27_24Dio26Zero,
            true => Din27_24Dio26::Din27_24Dio26One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din27_24_dio26_zero(&self) -> bool {
        *self == Din27_24Dio26::Din27_24Dio26Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din27_24_dio26_one(&self) -> bool {
        *self == Din27_24Dio26::Din27_24Dio26One
    }
}
#[doc = "This bit reads the data input value of DIO27.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din27_24Dio27 {
    #[doc = "0: ZERO"]
    Din27_24Dio27Zero = 0,
    #[doc = "1: ONE"]
    Din27_24Dio27One = 1,
}
impl From<Din27_24Dio27> for bool {
    #[inline(always)]
    fn from(variant: Din27_24Dio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN27_24_DIO27` reader - This bit reads the data input value of DIO27."]
pub type Din27_24Dio27R = crate::BitReader<Din27_24Dio27>;
impl Din27_24Dio27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din27_24Dio27 {
        match self.bits {
            false => Din27_24Dio27::Din27_24Dio27Zero,
            true => Din27_24Dio27::Din27_24Dio27One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din27_24_dio27_zero(&self) -> bool {
        *self == Din27_24Dio27::Din27_24Dio27Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din27_24_dio27_one(&self) -> bool {
        *self == Din27_24Dio27::Din27_24Dio27One
    }
}
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of DIO24."]
    #[inline(always)]
    pub fn din27_24_dio24(&self) -> Din27_24Dio24R {
        Din27_24Dio24R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of DIO25."]
    #[inline(always)]
    pub fn din27_24_dio25(&self) -> Din27_24Dio25R {
        Din27_24Dio25R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of DIO26."]
    #[inline(always)]
    pub fn din27_24_dio26(&self) -> Din27_24Dio26R {
        Din27_24Dio26R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of DIO27."]
    #[inline(always)]
    pub fn din27_24_dio27(&self) -> Din27_24Dio27R {
        Din27_24Dio27R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Data input 27 to 24\n\nYou can [`read`](crate::Reg::read) this register and get [`din27_24::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Din27_24Spec;
impl crate::RegisterSpec for Din27_24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din27_24::R`](R) reader structure"]
impl crate::Readable for Din27_24Spec {}
#[doc = "`reset()` method sets DIN27_24 to value 0"]
impl crate::Resettable for Din27_24Spec {
    const RESET_VALUE: u32 = 0;
}
