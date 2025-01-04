#[doc = "Register `INT_GROUP0_ICLR` writer"]
pub type W = crate::W<IntGroup0IclrSpec>;
#[doc = "Clears INT in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntGroup0IclrInt {
    #[doc = "0: NO_EFFECT"]
    IntGroup0IclrIntNoEffect = 0,
    #[doc = "1: CLR"]
    IntGroup0IclrIntClr = 1,
}
impl From<IntGroup0IclrInt> for bool {
    #[inline(always)]
    fn from(variant: IntGroup0IclrInt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_GROUP0_ICLR_INT` writer - Clears INT in RIS register"]
pub type IntGroup0IclrIntW<'a, REG> = crate::BitWriter<'a, REG, IntGroup0IclrInt>;
impl<'a, REG> IntGroup0IclrIntW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn int_group0_iclr_int_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IntGroup0IclrInt::IntGroup0IclrIntNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_group0_iclr_int_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntGroup0IclrInt::IntGroup0IclrIntClr)
    }
}
impl W {
    #[doc = "Bit 0 - Clears INT in RIS register"]
    #[inline(always)]
    pub fn int_group0_iclr_int(&mut self) -> IntGroup0IclrIntW<IntGroup0IclrSpec> {
        IntGroup0IclrIntW::new(self, 0)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_group0_iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntGroup0IclrSpec;
impl crate::RegisterSpec for IntGroup0IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_group0_iclr::W`](W) writer structure"]
impl crate::Writable for IntGroup0IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_GROUP0_ICLR to value 0"]
impl crate::Resettable for IntGroup0IclrSpec {
    const RESET_VALUE: u32 = 0;
}
