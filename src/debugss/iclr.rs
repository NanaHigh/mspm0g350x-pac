#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "Clears TXIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrTxifg {
    #[doc = "0: NO_EFFECT"]
    IclrTxifgNoEffect = 0,
    #[doc = "1: CLR"]
    IclrTxifgClr = 1,
}
impl From<IclrTxifg> for bool {
    #[inline(always)]
    fn from(variant: IclrTxifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_TXIFG` writer - Clears TXIFG in RIS register"]
pub type IclrTxifgW<'a, REG> = crate::BitWriter<'a, REG, IclrTxifg>;
impl<'a, REG> IclrTxifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_txifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrTxifg::IclrTxifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_txifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrTxifg::IclrTxifgClr)
    }
}
#[doc = "Clears RXIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrRxifg {
    #[doc = "0: NO_EFFECT"]
    IclrRxifgNoEffect = 0,
    #[doc = "1: CLR"]
    IclrRxifgClr = 1,
}
impl From<IclrRxifg> for bool {
    #[inline(always)]
    fn from(variant: IclrRxifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_RXIFG` writer - Clears RXIFG in RIS register"]
pub type IclrRxifgW<'a, REG> = crate::BitWriter<'a, REG, IclrRxifg>;
impl<'a, REG> IclrRxifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_rxifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrRxifg::IclrRxifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_rxifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrRxifg::IclrRxifgClr)
    }
}
#[doc = "Clears PWRUPIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrPwrupifg {
    #[doc = "0: NO_EFFECT"]
    IclrPwrupifgNoEffect = 0,
    #[doc = "1: CLR"]
    IclrPwrupifgClr = 1,
}
impl From<IclrPwrupifg> for bool {
    #[inline(always)]
    fn from(variant: IclrPwrupifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_PWRUPIFG` writer - Clears PWRUPIFG in RIS register"]
pub type IclrPwrupifgW<'a, REG> = crate::BitWriter<'a, REG, IclrPwrupifg>;
impl<'a, REG> IclrPwrupifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_pwrupifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrPwrupifg::IclrPwrupifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_pwrupifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrPwrupifg::IclrPwrupifgClr)
    }
}
#[doc = "Clears PWRDWNIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrPwrdwnifg {
    #[doc = "0: NO_EFFECT"]
    IclrPwrdwnifgNoEffect = 0,
    #[doc = "1: CLR"]
    IclrPwrdwnifgClr = 1,
}
impl From<IclrPwrdwnifg> for bool {
    #[inline(always)]
    fn from(variant: IclrPwrdwnifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_PWRDWNIFG` writer - Clears PWRDWNIFG in RIS register"]
pub type IclrPwrdwnifgW<'a, REG> = crate::BitWriter<'a, REG, IclrPwrdwnifg>;
impl<'a, REG> IclrPwrdwnifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_pwrdwnifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrPwrdwnifg::IclrPwrdwnifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_pwrdwnifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrPwrdwnifg::IclrPwrdwnifgClr)
    }
}
impl W {
    #[doc = "Bit 0 - Clears TXIFG in RIS register"]
    #[inline(always)]
    pub fn iclr_txifg(&mut self) -> IclrTxifgW<IclrSpec> {
        IclrTxifgW::new(self, 0)
    }
    #[doc = "Bit 1 - Clears RXIFG in RIS register"]
    #[inline(always)]
    pub fn iclr_rxifg(&mut self) -> IclrRxifgW<IclrSpec> {
        IclrRxifgW::new(self, 1)
    }
    #[doc = "Bit 2 - Clears PWRUPIFG in RIS register"]
    #[inline(always)]
    pub fn iclr_pwrupifg(&mut self) -> IclrPwrupifgW<IclrSpec> {
        IclrPwrupifgW::new(self, 2)
    }
    #[doc = "Bit 3 - Clears PWRDWNIFG in RIS register"]
    #[inline(always)]
    pub fn iclr_pwrdwnifg(&mut self) -> IclrPwrdwnifgW<IclrSpec> {
        IclrPwrdwnifgW::new(self, 3)
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
