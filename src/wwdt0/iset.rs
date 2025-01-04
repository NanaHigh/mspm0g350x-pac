#[doc = "Register `ISET` writer"]
pub type W = crate::W<IsetSpec>;
#[doc = "Interval Timer Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetInttim {
    #[doc = "0: NO_EFFECT"]
    IsetInttimNoEffect = 0,
    #[doc = "1: SET"]
    IsetInttimSet = 1,
}
impl From<IsetInttim> for bool {
    #[inline(always)]
    fn from(variant: IsetInttim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_INTTIM` writer - Interval Timer Interrupt."]
pub type IsetInttimW<'a, REG> = crate::BitWriter<'a, REG, IsetInttim>;
impl<'a, REG> IsetInttimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_inttim_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetInttim::IsetInttimNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_inttim_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetInttim::IsetInttimSet)
    }
}
impl W {
    #[doc = "Bit 0 - Interval Timer Interrupt."]
    #[inline(always)]
    pub fn iset_inttim(&mut self) -> IsetInttimW<IsetSpec> {
        IsetInttimW::new(self, 0)
    }
}
#[doc = "Interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsetSpec;
impl crate::RegisterSpec for IsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iset::W`](W) writer structure"]
impl crate::Writable for IsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISET to value 0"]
impl crate::Resettable for IsetSpec {
    const RESET_VALUE: u32 = 0;
}
