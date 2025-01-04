#[doc = "Register `INTCTL` reader"]
pub type R = crate::R<IntctlSpec>;
#[doc = "Register `INTCTL` writer"]
pub type W = crate::W<IntctlSpec>;
#[doc = "Writing a 1 to this field re-evaluates the interrupt sources.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntctlInteval {
    #[doc = "0: DISABLE"]
    IntctlIntevalDisable = 0,
    #[doc = "1: EVAL"]
    IntctlIntevalEval = 1,
}
impl From<IntctlInteval> for bool {
    #[inline(always)]
    fn from(variant: IntctlInteval) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTCTL_INTEVAL` writer - Writing a 1 to this field re-evaluates the interrupt sources."]
pub type IntctlIntevalW<'a, REG> = crate::BitWriter<'a, REG, IntctlInteval>;
impl<'a, REG> IntctlIntevalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn intctl_inteval_disable(self) -> &'a mut crate::W<REG> {
        self.variant(IntctlInteval::IntctlIntevalDisable)
    }
    #[doc = "EVAL"]
    #[inline(always)]
    pub fn intctl_inteval_eval(self) -> &'a mut crate::W<REG> {
        self.variant(IntctlInteval::IntctlIntevalEval)
    }
}
impl W {
    #[doc = "Bit 0 - Writing a 1 to this field re-evaluates the interrupt sources."]
    #[inline(always)]
    pub fn intctl_inteval(&mut self) -> IntctlIntevalW<IntctlSpec> {
        IntctlIntevalW::new(self, 0)
    }
}
#[doc = "Interrupt control register\n\nYou can [`read`](crate::Reg::read) this register and get [`intctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntctlSpec;
impl crate::RegisterSpec for IntctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intctl::R`](R) reader structure"]
impl crate::Readable for IntctlSpec {}
#[doc = "`write(|w| ..)` method takes [`intctl::W`](W) writer structure"]
impl crate::Writable for IntctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTCTL to value 0"]
impl crate::Resettable for IntctlSpec {
    const RESET_VALUE: u32 = 0;
}
