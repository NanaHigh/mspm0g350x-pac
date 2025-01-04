#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "Interval Timer Interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrInttim {
    #[doc = "0: NO_EFFECT"]
    IclrInttimNoEffect = 0,
    #[doc = "1: CLR"]
    IclrInttimClr = 1,
}
impl From<IclrInttim> for bool {
    #[inline(always)]
    fn from(variant: IclrInttim) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_INTTIM` writer - Interval Timer Interrupt."]
pub type IclrInttimW<'a, REG> = crate::BitWriter<'a, REG, IclrInttim>;
impl<'a, REG> IclrInttimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_inttim_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrInttim::IclrInttimNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_inttim_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrInttim::IclrInttimClr)
    }
}
impl W {
    #[doc = "Bit 0 - Interval Timer Interrupt."]
    #[inline(always)]
    pub fn iclr_inttim(&mut self) -> IclrInttimW<IclrSpec> {
        IclrInttimW::new(self, 0)
    }
}
#[doc = "Interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IclrSpec;
impl crate::RegisterSpec for IclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iclr::W`](W) writer structure"]
impl crate::Writable for IclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICLR to value 0"]
impl crate::Resettable for IclrSpec {
    const RESET_VALUE: u32 = 0;
}
