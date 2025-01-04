#[doc = "Register `INT_GROUP1_ICLR` writer"]
pub type W = crate::W<IntGroup1IclrSpec>;
#[doc = "Clears INT in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntGroup1IclrInt {
    #[doc = "0: NO_EFFECT"]
    IntGroup1IclrIntNoEffect = 0,
    #[doc = "1: CLR"]
    IntGroup1IclrIntClr = 1,
}
impl From<IntGroup1IclrInt> for bool {
    #[inline(always)]
    fn from(variant: IntGroup1IclrInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_GROUP1_ICLR_INT` writer - Clears INT in RIS register"]
pub type IntGroup1IclrIntW<'a, REG> = crate::BitWriter<'a, REG, IntGroup1IclrInt>;
impl<'a, REG> IntGroup1IclrIntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_group1_iclr_int_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntGroup1IclrInt::IntGroup1IclrIntNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_group1_iclr_int_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntGroup1IclrInt::IntGroup1IclrIntClr)
    }
}
impl W {
    #[doc = "Bit 0 - Clears INT in RIS register"]
    #[inline(always)]
    pub fn int_group1_iclr_int(&mut self) -> IntGroup1IclrIntW<IntGroup1IclrSpec> {
        IntGroup1IclrIntW::new(self, 0)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_group1_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntGroup1IclrSpec;
impl crate::RegisterSpec for IntGroup1IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_group1_iclr::W`](W) writer structure"]
impl crate::Writable for IntGroup1IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_GROUP1_ICLR to value 0"]
impl crate::Resettable for IntGroup1IclrSpec {
    const RESET_VALUE: u32 = 0;
}
