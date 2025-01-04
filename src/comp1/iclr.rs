#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "Clears COMPIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrCompifg {
    #[doc = "0: NO_EFFECT"]
    IclrCompifgNoEffect = 0,
    #[doc = "1: CLR"]
    IclrCompifgClr = 1,
}
impl From<IclrCompifg> for bool {
    #[inline(always)]
    fn from(variant: IclrCompifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_COMPIFG` writer - Clears COMPIFG in RIS register"]
pub type IclrCompifgW<'a, REG> = crate::BitWriter<'a, REG, IclrCompifg>;
impl<'a, REG> IclrCompifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_compifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCompifg::IclrCompifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_compifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCompifg::IclrCompifgClr)
    }
}
#[doc = "Clears COMPINVIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrCompinvifg {
    #[doc = "0: NO_EFFECT"]
    IclrCompinvifgNoEffect = 0,
    #[doc = "1: CLR"]
    IclrCompinvifgClr = 1,
}
impl From<IclrCompinvifg> for bool {
    #[inline(always)]
    fn from(variant: IclrCompinvifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_COMPINVIFG` writer - Clears COMPINVIFG in RIS register"]
pub type IclrCompinvifgW<'a, REG> = crate::BitWriter<'a, REG, IclrCompinvifg>;
impl<'a, REG> IclrCompinvifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_compinvifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCompinvifg::IclrCompinvifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_compinvifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCompinvifg::IclrCompinvifgClr)
    }
}
#[doc = "Clears OUTRDYIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrOutrdyifg {
    #[doc = "0: NO_EFFECT"]
    IclrOutrdyifgNoEffect = 0,
    #[doc = "1: CLR"]
    IclrOutrdyifgClr = 1,
}
impl From<IclrOutrdyifg> for bool {
    #[inline(always)]
    fn from(variant: IclrOutrdyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_OUTRDYIFG` writer - Clears OUTRDYIFG in RIS register"]
pub type IclrOutrdyifgW<'a, REG> = crate::BitWriter<'a, REG, IclrOutrdyifg>;
impl<'a, REG> IclrOutrdyifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_outrdyifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrOutrdyifg::IclrOutrdyifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_outrdyifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrOutrdyifg::IclrOutrdyifgClr)
    }
}
impl W {
    #[doc = "Bit 1 - Clears COMPIFG in RIS register"]
    #[inline(always)]
    pub fn iclr_compifg(&mut self) -> IclrCompifgW<IclrSpec> {
        IclrCompifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Clears COMPINVIFG in RIS register"]
    #[inline(always)]
    pub fn iclr_compinvifg(&mut self) -> IclrCompinvifgW<IclrSpec> {
        IclrCompinvifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Clears OUTRDYIFG in RIS register"]
    #[inline(always)]
    pub fn iclr_outrdyifg(&mut self) -> IclrOutrdyifgW<IclrSpec> {
        IclrOutrdyifgW::new(self, 3)
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
