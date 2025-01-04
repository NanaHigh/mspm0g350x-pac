#[doc = "Register `ISET` writer"]
pub type W = crate::W<IsetSpec>;
#[doc = "Sets COMPIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetCompifg {
    #[doc = "0: NO_EFFECT"]
    IsetCompifgNoEffect = 0,
    #[doc = "1: SET"]
    IsetCompifgSet = 1,
}
impl From<IsetCompifg> for bool {
    #[inline(always)]
    fn from(variant: IsetCompifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_COMPIFG` writer - Sets COMPIFG in RIS register"]
pub type IsetCompifgW<'a, REG> = crate::BitWriter<'a, REG, IsetCompifg>;
impl<'a, REG> IsetCompifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_compifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetCompifg::IsetCompifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_compifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetCompifg::IsetCompifgSet)
    }
}
#[doc = "Sets COMPINVIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetCompinvifg {
    #[doc = "0: NO_EFFECT"]
    IsetCompinvifgNoEffect = 0,
    #[doc = "1: SET"]
    IsetCompinvifgSet = 1,
}
impl From<IsetCompinvifg> for bool {
    #[inline(always)]
    fn from(variant: IsetCompinvifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_COMPINVIFG` writer - Sets COMPINVIFG in RIS register"]
pub type IsetCompinvifgW<'a, REG> = crate::BitWriter<'a, REG, IsetCompinvifg>;
impl<'a, REG> IsetCompinvifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_compinvifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetCompinvifg::IsetCompinvifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_compinvifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetCompinvifg::IsetCompinvifgSet)
    }
}
#[doc = "Sets OUTRDYIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetOutrdyifg {
    #[doc = "0: NO_EFFECT"]
    IsetOutrdyifgNoEffect = 0,
    #[doc = "1: SET"]
    IsetOutrdyifgSet = 1,
}
impl From<IsetOutrdyifg> for bool {
    #[inline(always)]
    fn from(variant: IsetOutrdyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_OUTRDYIFG` writer - Sets OUTRDYIFG in RIS register"]
pub type IsetOutrdyifgW<'a, REG> = crate::BitWriter<'a, REG, IsetOutrdyifg>;
impl<'a, REG> IsetOutrdyifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_outrdyifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetOutrdyifg::IsetOutrdyifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_outrdyifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetOutrdyifg::IsetOutrdyifgSet)
    }
}
impl W {
    #[doc = "Bit 1 - Sets COMPIFG in RIS register"]
    #[inline(always)]
    pub fn iset_compifg(&mut self) -> IsetCompifgW<IsetSpec> {
        IsetCompifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Sets COMPINVIFG in RIS register"]
    #[inline(always)]
    pub fn iset_compinvifg(&mut self) -> IsetCompinvifgW<IsetSpec> {
        IsetCompinvifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Sets OUTRDYIFG in RIS register"]
    #[inline(always)]
    pub fn iset_outrdyifg(&mut self) -> IsetOutrdyifgW<IsetSpec> {
        IsetOutrdyifgW::new(self, 3)
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
