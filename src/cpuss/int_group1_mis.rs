#[doc = "Register `INT_GROUP1_MIS` reader"]
pub type R = crate::R<IntGroup1MisSpec>;
#[doc = "Masked interrupt status for INT0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntGroup1MisInt {
    #[doc = "0: CLR"]
    IntGroup1MisIntClr = 0,
    #[doc = "1: SET"]
    IntGroup1MisIntSet = 1,
}
impl From<IntGroup1MisInt> for bool {
    #[inline(always)]
    fn from(variant: IntGroup1MisInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_GROUP1_MIS_INT` reader - Masked interrupt status for INT0"]
pub type IntGroup1MisIntR = crate::BitReader<IntGroup1MisInt>;
impl IntGroup1MisIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntGroup1MisInt {
        match self.bits {
            false => IntGroup1MisInt::IntGroup1MisIntClr,
            true => IntGroup1MisInt::IntGroup1MisIntSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_group1_mis_int_clr(&self) -> bool {
        *self == IntGroup1MisInt::IntGroup1MisIntClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_group1_mis_int_set(&self) -> bool {
        *self == IntGroup1MisInt::IntGroup1MisIntSet
    }
}
impl R {
    #[doc = "Bit 0 - Masked interrupt status for INT0"]
    #[inline(always)]
    pub fn int_group1_mis_int(&self) -> IntGroup1MisIntR {
        IntGroup1MisIntR::new((self.bits & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_group1_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntGroup1MisSpec;
impl crate::RegisterSpec for IntGroup1MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_group1_mis::R`](R) reader structure"]
impl crate::Readable for IntGroup1MisSpec {}
#[doc = "`reset()` method sets INT_GROUP1_MIS to value 0"]
impl crate::Resettable for IntGroup1MisSpec {
    const RESET_VALUE: u32 = 0;
}
