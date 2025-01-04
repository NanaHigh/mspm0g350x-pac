#[doc = "Register `INT_GROUP1_IMASK` reader"]
pub type R = crate::R<IntGroup1ImaskSpec>;
#[doc = "Masks the corresponding interrupt\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntGroup1ImaskInt {
    #[doc = "0: CLR"]
    IntGroup1ImaskIntClr = 0,
    #[doc = "1: SET"]
    IntGroup1ImaskIntSet = 1,
}
impl From<IntGroup1ImaskInt> for u8 {
    #[inline(always)]
    fn from(variant: IntGroup1ImaskInt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntGroup1ImaskInt {
    type Ux = u8;
}
impl crate::IsEnum for IntGroup1ImaskInt {}
#[doc = "Field `INT_GROUP1_IMASK_INT` reader - Masks the corresponding interrupt"]
pub type IntGroup1ImaskIntR = crate::FieldReader<IntGroup1ImaskInt>;
impl IntGroup1ImaskIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntGroup1ImaskInt> {
        match self.bits {
            0 => Some(IntGroup1ImaskInt::IntGroup1ImaskIntClr),
            1 => Some(IntGroup1ImaskInt::IntGroup1ImaskIntSet),
            _ => None,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_group1_imask_int_clr(&self) -> bool {
        *self == IntGroup1ImaskInt::IntGroup1ImaskIntClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_group1_imask_int_set(&self) -> bool {
        *self == IntGroup1ImaskInt::IntGroup1ImaskIntSet
    }
}
impl R {
    #[doc = "Bits 0:7 - Masks the corresponding interrupt"]
    #[inline(always)]
    pub fn int_group1_imask_int(&self) -> IntGroup1ImaskIntR {
        IntGroup1ImaskIntR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_group1_imask::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntGroup1ImaskSpec;
impl crate::RegisterSpec for IntGroup1ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_group1_imask::R`](R) reader structure"]
impl crate::Readable for IntGroup1ImaskSpec {}
#[doc = "`reset()` method sets INT_GROUP1_IMASK to value 0xff"]
impl crate::Resettable for IntGroup1ImaskSpec {
    const RESET_VALUE: u32 = 0xff;
}
