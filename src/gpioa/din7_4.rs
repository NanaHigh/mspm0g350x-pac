#[doc = "Register `DIN7_4` reader"]
pub type R = crate::R<Din7_4Spec>;
#[doc = "This bit reads the data input value of DIO4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din7_4Dio4 {
    #[doc = "0: ZERO"]
    Din7_4Dio4Zero = 0,
    #[doc = "1: ONE"]
    Din7_4Dio4One = 1,
}
impl From<Din7_4Dio4> for bool {
    #[inline(always)]
    fn from(variant: Din7_4Dio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN7_4_DIO4` reader - This bit reads the data input value of DIO4."]
pub type Din7_4Dio4R = crate::BitReader<Din7_4Dio4>;
impl Din7_4Dio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din7_4Dio4 {
        match self.bits {
            false => Din7_4Dio4::Din7_4Dio4Zero,
            true => Din7_4Dio4::Din7_4Dio4One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din7_4_dio4_zero(&self) -> bool {
        *self == Din7_4Dio4::Din7_4Dio4Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din7_4_dio4_one(&self) -> bool {
        *self == Din7_4Dio4::Din7_4Dio4One
    }
}
#[doc = "This bit reads the data input value of DIO5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din7_4Dio5 {
    #[doc = "0: ZERO"]
    Din7_4Dio5Zero = 0,
    #[doc = "1: ONE"]
    Din7_4Dio5One = 1,
}
impl From<Din7_4Dio5> for bool {
    #[inline(always)]
    fn from(variant: Din7_4Dio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN7_4_DIO5` reader - This bit reads the data input value of DIO5."]
pub type Din7_4Dio5R = crate::BitReader<Din7_4Dio5>;
impl Din7_4Dio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din7_4Dio5 {
        match self.bits {
            false => Din7_4Dio5::Din7_4Dio5Zero,
            true => Din7_4Dio5::Din7_4Dio5One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din7_4_dio5_zero(&self) -> bool {
        *self == Din7_4Dio5::Din7_4Dio5Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din7_4_dio5_one(&self) -> bool {
        *self == Din7_4Dio5::Din7_4Dio5One
    }
}
#[doc = "This bit reads the data input value of DIO6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din7_4Dio6 {
    #[doc = "0: ZERO"]
    Din7_4Dio6Zero = 0,
    #[doc = "1: ONE"]
    Din7_4Dio6One = 1,
}
impl From<Din7_4Dio6> for bool {
    #[inline(always)]
    fn from(variant: Din7_4Dio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN7_4_DIO6` reader - This bit reads the data input value of DIO6."]
pub type Din7_4Dio6R = crate::BitReader<Din7_4Dio6>;
impl Din7_4Dio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din7_4Dio6 {
        match self.bits {
            false => Din7_4Dio6::Din7_4Dio6Zero,
            true => Din7_4Dio6::Din7_4Dio6One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din7_4_dio6_zero(&self) -> bool {
        *self == Din7_4Dio6::Din7_4Dio6Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din7_4_dio6_one(&self) -> bool {
        *self == Din7_4Dio6::Din7_4Dio6One
    }
}
#[doc = "This bit reads the data input value of DIO7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Din7_4Dio7 {
    #[doc = "0: ZERO"]
    Din7_4Dio7Zero = 0,
    #[doc = "1: ONE"]
    Din7_4Dio7One = 1,
}
impl From<Din7_4Dio7> for bool {
    #[inline(always)]
    fn from(variant: Din7_4Dio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIN7_4_DIO7` reader - This bit reads the data input value of DIO7."]
pub type Din7_4Dio7R = crate::BitReader<Din7_4Dio7>;
impl Din7_4Dio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Din7_4Dio7 {
        match self.bits {
            false => Din7_4Dio7::Din7_4Dio7Zero,
            true => Din7_4Dio7::Din7_4Dio7One,
        }
    }
    #[doc = "ZERO"]
    #[inline(always)]
    pub fn is_din7_4_dio7_zero(&self) -> bool {
        *self == Din7_4Dio7::Din7_4Dio7Zero
    }
    #[doc = "ONE"]
    #[inline(always)]
    pub fn is_din7_4_dio7_one(&self) -> bool {
        *self == Din7_4Dio7::Din7_4Dio7One
    }
}
impl R {
    #[doc = "Bit 0 - This bit reads the data input value of DIO4."]
    #[inline(always)]
    pub fn din7_4_dio4(&self) -> Din7_4Dio4R {
        Din7_4Dio4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - This bit reads the data input value of DIO5."]
    #[inline(always)]
    pub fn din7_4_dio5(&self) -> Din7_4Dio5R {
        Din7_4Dio5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit reads the data input value of DIO6."]
    #[inline(always)]
    pub fn din7_4_dio6(&self) -> Din7_4Dio6R {
        Din7_4Dio6R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit reads the data input value of DIO7."]
    #[inline(always)]
    pub fn din7_4_dio7(&self) -> Din7_4Dio7R {
        Din7_4Dio7R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Data input 7 to 4\n\nYou can [`read`](crate::Reg::read) this register and get [`din7_4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Din7_4Spec;
impl crate::RegisterSpec for Din7_4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`din7_4::R`](R) reader structure"]
impl crate::Readable for Din7_4Spec {}
#[doc = "`reset()` method sets DIN7_4 to value 0"]
impl crate::Resettable for Din7_4Spec {
    const RESET_VALUE: u32 = 0;
}
