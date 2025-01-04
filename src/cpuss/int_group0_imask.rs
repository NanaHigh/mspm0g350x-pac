#[doc = "Register `INT_GROUP0_IMASK` reader"]
pub type R = crate::R<IntGroup0ImaskSpec>;
#[doc = "Masks the corresponding interrupt\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntGroup0ImaskInt {
    #[doc = "0: CLR"]
    IntGroup0ImaskIntClr = 0,
    #[doc = "1: SET"]
    IntGroup0ImaskIntSet = 1,
}
impl From<IntGroup0ImaskInt> for u8 {
    #[inline(always)]
    fn from(variant: IntGroup0ImaskInt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntGroup0ImaskInt {
    type Ux = u8;
}
impl crate::IsEnum for IntGroup0ImaskInt {}
#[doc = "Field `INT_GROUP0_IMASK_INT` reader - Masks the corresponding interrupt"]
pub type IntGroup0ImaskIntR = crate::FieldReader<IntGroup0ImaskInt>;
impl IntGroup0ImaskIntR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntGroup0ImaskInt> {
        match self.bits {
            0 => Some(IntGroup0ImaskInt::IntGroup0ImaskIntClr),
            1 => Some(IntGroup0ImaskInt::IntGroup0ImaskIntSet),
            _ => None,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_group0_imask_int_clr(&self) -> bool {
        *self == IntGroup0ImaskInt::IntGroup0ImaskIntClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_group0_imask_int_set(&self) -> bool {
        *self == IntGroup0ImaskInt::IntGroup0ImaskIntSet
    }
}
impl R {
    #[doc = "Bits 0:7 - Masks the corresponding interrupt"]
    #[inline(always)]
    pub fn int_group0_imask_int(&self) -> IntGroup0ImaskIntR {
        IntGroup0ImaskIntR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_group0_imask::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntGroup0ImaskSpec;
impl crate::RegisterSpec for IntGroup0ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_group0_imask::R`](R) reader structure"]
impl crate::Readable for IntGroup0ImaskSpec {}
#[doc = "`reset()` method sets INT_GROUP0_IMASK to value 0xff"]
impl crate::Resettable for IntGroup0ImaskSpec {
    const RESET_VALUE: u32 = 0xff;
}
