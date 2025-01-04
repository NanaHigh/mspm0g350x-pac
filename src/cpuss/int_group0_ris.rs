#[doc = "Register `INT_GROUP0_RIS` reader"]
pub type R = crate::R<IntGroup0RisSpec>;
#[doc = "Raw interrupt status for INT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntGroup0RisInt {
    #[doc = "0: CLR"]
    IntGroup0RisIntClr = 0,
    #[doc = "1: SET"]
    IntGroup0RisIntSet = 1,
}
impl From<IntGroup0RisInt> for u8 {
    #[inline(always)]
    fn from(variant: IntGroup0RisInt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntGroup0RisInt {
    type Ux = u8;
}
impl crate::IsEnum for IntGroup0RisInt {}
#[doc = "Field `INT_GROUP0_RIS_INT` reader - Raw interrupt status for INT"]
pub type IntGroup0RisIntR = crate::FieldReader<IntGroup0RisInt>;
impl IntGroup0RisIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntGroup0RisInt> {
        match self.bits {
            0 => Some(IntGroup0RisInt::IntGroup0RisIntClr),
            1 => Some(IntGroup0RisInt::IntGroup0RisIntSet),
            _ => None,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_group0_ris_int_clr(&self) -> bool {
        *self == IntGroup0RisInt::IntGroup0RisIntClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_group0_ris_int_set(&self) -> bool {
        *self == IntGroup0RisInt::IntGroup0RisIntSet
    }
}
impl R {
    #[doc = "Bits 0:7 - Raw interrupt status for INT"]
    #[inline(always)]
    pub fn int_group0_ris_int(&self) -> IntGroup0RisIntR {
        IntGroup0RisIntR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_group0_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntGroup0RisSpec;
impl crate::RegisterSpec for IntGroup0RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_group0_ris::R`](R) reader structure"]
impl crate::Readable for IntGroup0RisSpec {}
#[doc = "`reset()` method sets INT_GROUP0_RIS to value 0"]
impl crate::Resettable for IntGroup0RisSpec {
    const RESET_VALUE: u32 = 0;
}
