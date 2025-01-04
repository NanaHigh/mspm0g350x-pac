#[doc = "Register `ISET` writer"]
pub type W = crate::W<IsetSpec>;
#[doc = "Sets TXIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetTxifg {
    #[doc = "0: NO_EFFECT"]
    IsetTxifgNoEffect = 0,
    #[doc = "1: SET"]
    IsetTxifgSet = 1,
}
impl From<IsetTxifg> for bool {
    #[inline(always)]
    fn from(variant: IsetTxifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_TXIFG` writer - Sets TXIFG in RIS register"]
pub type IsetTxifgW<'a, REG> = crate::BitWriter<'a, REG, IsetTxifg>;
impl<'a, REG> IsetTxifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_txifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetTxifg::IsetTxifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_txifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetTxifg::IsetTxifgSet)
    }
}
#[doc = "Sets RXIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetRxifg {
    #[doc = "0: NO_EFFECT"]
    IsetRxifgNoEffect = 0,
    #[doc = "1: SET"]
    IsetRxifgSet = 1,
}
impl From<IsetRxifg> for bool {
    #[inline(always)]
    fn from(variant: IsetRxifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_RXIFG` writer - Sets RXIFG in RIS register"]
pub type IsetRxifgW<'a, REG> = crate::BitWriter<'a, REG, IsetRxifg>;
impl<'a, REG> IsetRxifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_rxifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetRxifg::IsetRxifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_rxifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetRxifg::IsetRxifgSet)
    }
}
#[doc = "Sets PWRUPIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetPwrupifg {
    #[doc = "0: NO_EFFECT"]
    IsetPwrupifgNoEffect = 0,
    #[doc = "1: SET"]
    IsetPwrupifgSet = 1,
}
impl From<IsetPwrupifg> for bool {
    #[inline(always)]
    fn from(variant: IsetPwrupifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_PWRUPIFG` writer - Sets PWRUPIFG in RIS register"]
pub type IsetPwrupifgW<'a, REG> = crate::BitWriter<'a, REG, IsetPwrupifg>;
impl<'a, REG> IsetPwrupifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_pwrupifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetPwrupifg::IsetPwrupifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_pwrupifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetPwrupifg::IsetPwrupifgSet)
    }
}
#[doc = "Sets PWRDWNIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetPwrdwnifg {
    #[doc = "0: NO_EFFECT"]
    IsetPwrdwnifgNoEffect = 0,
    #[doc = "1: SET"]
    IsetPwrdwnifgSet = 1,
}
impl From<IsetPwrdwnifg> for bool {
    #[inline(always)]
    fn from(variant: IsetPwrdwnifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_PWRDWNIFG` writer - Sets PWRDWNIFG in RIS register"]
pub type IsetPwrdwnifgW<'a, REG> = crate::BitWriter<'a, REG, IsetPwrdwnifg>;
impl<'a, REG> IsetPwrdwnifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_pwrdwnifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetPwrdwnifg::IsetPwrdwnifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_pwrdwnifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetPwrdwnifg::IsetPwrdwnifgSet)
    }
}
impl W {
    #[doc = "Bit 0 - Sets TXIFG in RIS register"]
    #[inline(always)]
    pub fn iset_txifg(&mut self) -> IsetTxifgW<IsetSpec> {
        IsetTxifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Sets RXIFG in RIS register"]
    #[inline(always)]
    pub fn iset_rxifg(&mut self) -> IsetRxifgW<IsetSpec> {
        IsetRxifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Sets PWRUPIFG in RIS register"]
    #[inline(always)]
    pub fn iset_pwrupifg(&mut self) -> IsetPwrupifgW<IsetSpec> {
        IsetPwrupifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Sets PWRDWNIFG in RIS register"]
    #[inline(always)]
    pub fn iset_pwrdwnifg(&mut self) -> IsetPwrdwnifgW<IsetSpec> {
        IsetPwrdwnifgW::new(self, 3)
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
