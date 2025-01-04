#[doc = "Register `INT_GROUP0_MIS` reader"]
pub type R = crate::R<IntGroup0MisSpec>;
#[doc = "Masked interrupt status for INT0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntGroup0MisInt {
    #[doc = "0: CLR"]
    IntGroup0MisIntClr = 0,
    #[doc = "1: SET"]
    IntGroup0MisIntSet = 1,
}
impl From<IntGroup0MisInt> for bool {
    #[inline(always)]
    fn from(variant: IntGroup0MisInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_GROUP0_MIS_INT` reader - Masked interrupt status for INT0"]
pub type IntGroup0MisIntR = crate::BitReader<IntGroup0MisInt>;
impl IntGroup0MisIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntGroup0MisInt {
        match self.bits {
            false => IntGroup0MisInt::IntGroup0MisIntClr,
            true => IntGroup0MisInt::IntGroup0MisIntSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_group0_mis_int_clr(&self) -> bool {
        *self == IntGroup0MisInt::IntGroup0MisIntClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_group0_mis_int_set(&self) -> bool {
        *self == IntGroup0MisInt::IntGroup0MisIntSet
    }
}
impl R {
    #[doc = "Bit 0 - Masked interrupt status for INT0"]
    #[inline(always)]
    pub fn int_group0_mis_int(&self) -> IntGroup0MisIntR {
        IntGroup0MisIntR::new((self.bits & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_group0_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntGroup0MisSpec;
impl crate::RegisterSpec for IntGroup0MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_group0_mis::R`](R) reader structure"]
impl crate::Readable for IntGroup0MisSpec {}
#[doc = "`reset()` method sets INT_GROUP0_MIS to value 0"]
impl crate::Resettable for IntGroup0MisSpec {
    const RESET_VALUE: u32 = 0;
}
