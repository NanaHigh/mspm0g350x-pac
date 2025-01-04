#[doc = "Register `INT_GROUP1_RIS` reader"]
pub type R = crate::R<IntGroup1RisSpec>;
#[doc = "Raw interrupt status for INT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntGroup1RisInt {
    #[doc = "0: CLR"]
    IntGroup1RisIntClr = 0,
    #[doc = "1: SET"]
    IntGroup1RisIntSet = 1,
}
impl From<IntGroup1RisInt> for bool {
    #[inline(always)]
    fn from(variant: IntGroup1RisInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_GROUP1_RIS_INT` reader - Raw interrupt status for INT"]
pub type IntGroup1RisIntR = crate::BitReader<IntGroup1RisInt>;
impl IntGroup1RisIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntGroup1RisInt {
        match self.bits {
            false => IntGroup1RisInt::IntGroup1RisIntClr,
            true => IntGroup1RisInt::IntGroup1RisIntSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_group1_ris_int_clr(&self) -> bool {
        *self == IntGroup1RisInt::IntGroup1RisIntClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_group1_ris_int_set(&self) -> bool {
        *self == IntGroup1RisInt::IntGroup1RisIntSet
    }
}
impl R {
    #[doc = "Bit 0 - Raw interrupt status for INT"]
    #[inline(always)]
    pub fn int_group1_ris_int(&self) -> IntGroup1RisIntR {
        IntGroup1RisIntR::new((self.bits & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_group1_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntGroup1RisSpec;
impl crate::RegisterSpec for IntGroup1RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_group1_ris::R`](R) reader structure"]
impl crate::Readable for IntGroup1RisSpec {}
#[doc = "`reset()` method sets INT_GROUP1_RIS to value 0"]
impl crate::Resettable for IntGroup1RisSpec {
    const RESET_VALUE: u32 = 0;
}
