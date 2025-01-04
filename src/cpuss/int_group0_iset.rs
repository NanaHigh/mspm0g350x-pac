#[doc = "Register `INT_GROUP0_ISET` writer"]
pub type W = crate::W<IntGroup0IsetSpec>;
#[doc = "Sets INT in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntGroup0IsetInt {
    #[doc = "0: NO_EFFECT"]
    IntGroup0IsetIntNoEffect = 0,
    #[doc = "1: SET"]
    IntGroup0IsetIntSet = 1,
}
impl From<IntGroup0IsetInt> for bool {
    #[inline(always)]
    fn from(variant: IntGroup0IsetInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_GROUP0_ISET_INT` writer - Sets INT in RIS register"]
pub type IntGroup0IsetIntW<'a, REG> = crate::BitWriter<'a, REG, IntGroup0IsetInt>;
impl<'a, REG> IntGroup0IsetIntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_group0_iset_int_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntGroup0IsetInt::IntGroup0IsetIntNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_group0_iset_int_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntGroup0IsetInt::IntGroup0IsetIntSet)
    }
}
impl W {
    #[doc = "Bit 0 - Sets INT in RIS register"]
    #[inline(always)]
    pub fn int_group0_iset_int(&mut self) -> IntGroup0IsetIntW<IntGroup0IsetSpec> {
        IntGroup0IsetIntW::new(self, 0)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_group0_iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntGroup0IsetSpec;
impl crate::RegisterSpec for IntGroup0IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_group0_iset::W`](W) writer structure"]
impl crate::Writable for IntGroup0IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_GROUP0_ISET to value 0"]
impl crate::Resettable for IntGroup0IsetSpec {
    const RESET_VALUE: u32 = 0;
}
