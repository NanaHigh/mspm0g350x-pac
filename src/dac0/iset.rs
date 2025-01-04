#[doc = "Register `ISET` writer"]
pub type W = crate::W<IsetSpec>;
#[doc = "Sets MODRDYIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetModrdyifg {
    #[doc = "0: NO_EFFECT"]
    IsetModrdyifgNoEffect = 0,
    #[doc = "1: SET"]
    IsetModrdyifgSet = 1,
}
impl From<IsetModrdyifg> for bool {
    #[inline(always)]
    fn from(variant: IsetModrdyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_MODRDYIFG` writer - Sets MODRDYIFG in RIS register"]
pub type IsetModrdyifgW<'a, REG> = crate::BitWriter<'a, REG, IsetModrdyifg>;
impl<'a, REG> IsetModrdyifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_modrdyifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetModrdyifg::IsetModrdyifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_modrdyifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetModrdyifg::IsetModrdyifgSet)
    }
}
#[doc = "Sets FIFOFULLIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetFifofullifg {
    #[doc = "0: NO_EFFECT"]
    IsetFifofullifgNoEffect = 0,
    #[doc = "1: SET"]
    IsetFifofullifgSet = 1,
}
impl From<IsetFifofullifg> for bool {
    #[inline(always)]
    fn from(variant: IsetFifofullifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_FIFOFULLIFG` writer - Sets FIFOFULLIFG in RIS register"]
pub type IsetFifofullifgW<'a, REG> = crate::BitWriter<'a, REG, IsetFifofullifg>;
impl<'a, REG> IsetFifofullifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_fifofullifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetFifofullifg::IsetFifofullifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_fifofullifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetFifofullifg::IsetFifofullifgSet)
    }
}
#[doc = "Sets FIFO1B4IFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetFifo1b4ifg {
    #[doc = "0: NO_EFFECT"]
    IsetFifo1b4ifgNoEffect = 0,
    #[doc = "1: SET"]
    IsetFifo1b4ifgSet = 1,
}
impl From<IsetFifo1b4ifg> for bool {
    #[inline(always)]
    fn from(variant: IsetFifo1b4ifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_FIFO1B4IFG` writer - Sets FIFO1B4IFG in RIS register"]
pub type IsetFifo1b4ifgW<'a, REG> = crate::BitWriter<'a, REG, IsetFifo1b4ifg>;
impl<'a, REG> IsetFifo1b4ifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_fifo1b4ifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetFifo1b4ifg::IsetFifo1b4ifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_fifo1b4ifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetFifo1b4ifg::IsetFifo1b4ifgSet)
    }
}
#[doc = "Sets FIFO1B2IFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetFifo1b2ifg {
    #[doc = "0: NO_EFFECT"]
    IsetFifo1b2ifgNoEffect = 0,
    #[doc = "1: SET"]
    IsetFifo1b2ifgSet = 1,
}
impl From<IsetFifo1b2ifg> for bool {
    #[inline(always)]
    fn from(variant: IsetFifo1b2ifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_FIFO1B2IFG` writer - Sets FIFO1B2IFG in RIS register"]
pub type IsetFifo1b2ifgW<'a, REG> = crate::BitWriter<'a, REG, IsetFifo1b2ifg>;
impl<'a, REG> IsetFifo1b2ifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_fifo1b2ifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetFifo1b2ifg::IsetFifo1b2ifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_fifo1b2ifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetFifo1b2ifg::IsetFifo1b2ifgSet)
    }
}
#[doc = "Sets FIFO3B4IFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetFifo3b4ifg {
    #[doc = "0: NO_EFFECT"]
    IsetFifo3b4ifgNoEffect = 0,
    #[doc = "1: SET"]
    IsetFifo3b4ifgSet = 1,
}
impl From<IsetFifo3b4ifg> for bool {
    #[inline(always)]
    fn from(variant: IsetFifo3b4ifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_FIFO3B4IFG` writer - Sets FIFO3B4IFG in RIS register"]
pub type IsetFifo3b4ifgW<'a, REG> = crate::BitWriter<'a, REG, IsetFifo3b4ifg>;
impl<'a, REG> IsetFifo3b4ifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_fifo3b4ifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetFifo3b4ifg::IsetFifo3b4ifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_fifo3b4ifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetFifo3b4ifg::IsetFifo3b4ifgSet)
    }
}
#[doc = "Sets FIFOEMPTYIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetFifoemptyifg {
    #[doc = "0: NO_EFFECT"]
    IsetFifoemptyifgNoEffect = 0,
    #[doc = "1: SET"]
    IsetFifoemptyifgSet = 1,
}
impl From<IsetFifoemptyifg> for bool {
    #[inline(always)]
    fn from(variant: IsetFifoemptyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_FIFOEMPTYIFG` writer - Sets FIFOEMPTYIFG in RIS register"]
pub type IsetFifoemptyifgW<'a, REG> = crate::BitWriter<'a, REG, IsetFifoemptyifg>;
impl<'a, REG> IsetFifoemptyifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_fifoemptyifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetFifoemptyifg::IsetFifoemptyifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_fifoemptyifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetFifoemptyifg::IsetFifoemptyifgSet)
    }
}
#[doc = "Sets FIFOURUNIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetFifourunifg {
    #[doc = "0: NO_EFFECT"]
    IsetFifourunifgNoEffect = 0,
    #[doc = "1: SET"]
    IsetFifourunifgSet = 1,
}
impl From<IsetFifourunifg> for bool {
    #[inline(always)]
    fn from(variant: IsetFifourunifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_FIFOURUNIFG` writer - Sets FIFOURUNIFG in RIS register"]
pub type IsetFifourunifgW<'a, REG> = crate::BitWriter<'a, REG, IsetFifourunifg>;
impl<'a, REG> IsetFifourunifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_fifourunifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetFifourunifg::IsetFifourunifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_fifourunifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetFifourunifg::IsetFifourunifgSet)
    }
}
#[doc = "Sets DMADONEIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetDmadoneifg {
    #[doc = "0: NO_EFFECT"]
    IsetDmadoneifgNoEffect = 0,
    #[doc = "1: SET"]
    IsetDmadoneifgSet = 1,
}
impl From<IsetDmadoneifg> for bool {
    #[inline(always)]
    fn from(variant: IsetDmadoneifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_DMADONEIFG` writer - Sets DMADONEIFG in RIS register"]
pub type IsetDmadoneifgW<'a, REG> = crate::BitWriter<'a, REG, IsetDmadoneifg>;
impl<'a, REG> IsetDmadoneifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_dmadoneifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDmadoneifg::IsetDmadoneifgNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_dmadoneifg_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDmadoneifg::IsetDmadoneifgSet)
    }
}
impl W {
    #[doc = "Bit 1 - Sets MODRDYIFG in RIS register"]
    #[inline(always)]
    pub fn iset_modrdyifg(&mut self) -> IsetModrdyifgW<IsetSpec> {
        IsetModrdyifgW::new(self, 1)
    }
    #[doc = "Bit 8 - Sets FIFOFULLIFG in RIS register"]
    #[inline(always)]
    pub fn iset_fifofullifg(&mut self) -> IsetFifofullifgW<IsetSpec> {
        IsetFifofullifgW::new(self, 8)
    }
    #[doc = "Bit 9 - Sets FIFO1B4IFG in RIS register"]
    #[inline(always)]
    pub fn iset_fifo1b4ifg(&mut self) -> IsetFifo1b4ifgW<IsetSpec> {
        IsetFifo1b4ifgW::new(self, 9)
    }
    #[doc = "Bit 10 - Sets FIFO1B2IFG in RIS register"]
    #[inline(always)]
    pub fn iset_fifo1b2ifg(&mut self) -> IsetFifo1b2ifgW<IsetSpec> {
        IsetFifo1b2ifgW::new(self, 10)
    }
    #[doc = "Bit 11 - Sets FIFO3B4IFG in RIS register"]
    #[inline(always)]
    pub fn iset_fifo3b4ifg(&mut self) -> IsetFifo3b4ifgW<IsetSpec> {
        IsetFifo3b4ifgW::new(self, 11)
    }
    #[doc = "Bit 12 - Sets FIFOEMPTYIFG in RIS register"]
    #[inline(always)]
    pub fn iset_fifoemptyifg(&mut self) -> IsetFifoemptyifgW<IsetSpec> {
        IsetFifoemptyifgW::new(self, 12)
    }
    #[doc = "Bit 13 - Sets FIFOURUNIFG in RIS register"]
    #[inline(always)]
    pub fn iset_fifourunifg(&mut self) -> IsetFifourunifgW<IsetSpec> {
        IsetFifourunifgW::new(self, 13)
    }
    #[doc = "Bit 14 - Sets DMADONEIFG in RIS register"]
    #[inline(always)]
    pub fn iset_dmadoneifg(&mut self) -> IsetDmadoneifgW<IsetSpec> {
        IsetDmadoneifgW::new(self, 14)
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
