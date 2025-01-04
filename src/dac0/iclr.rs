#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "Clears MODRDYIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrModrdyifg {
    #[doc = "0: NO_EFFECT"]
    IclrModrdyifgNoEffect = 0,
    #[doc = "1: CLR"]
    IclrModrdyifgClr = 1,
}
impl From<IclrModrdyifg> for bool {
    #[inline(always)]
    fn from(variant: IclrModrdyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_MODRDYIFG` writer - Clears MODRDYIFG in RIS register"]
pub type IclrModrdyifgW<'a, REG> = crate::BitWriter<'a, REG, IclrModrdyifg>;
impl<'a, REG> IclrModrdyifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_modrdyifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrModrdyifg::IclrModrdyifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_modrdyifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrModrdyifg::IclrModrdyifgClr)
    }
}
#[doc = "Clears FIFOFULLIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrFifofullifg {
    #[doc = "0: NO_EFFECT"]
    IclrFifofullifgNoEffect = 0,
    #[doc = "1: CLR"]
    IclrFifofullifgClr = 1,
}
impl From<IclrFifofullifg> for bool {
    #[inline(always)]
    fn from(variant: IclrFifofullifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_FIFOFULLIFG` writer - Clears FIFOFULLIFG in RIS register"]
pub type IclrFifofullifgW<'a, REG> = crate::BitWriter<'a, REG, IclrFifofullifg>;
impl<'a, REG> IclrFifofullifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_fifofullifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrFifofullifg::IclrFifofullifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_fifofullifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrFifofullifg::IclrFifofullifgClr)
    }
}
#[doc = "Clears FIFO1B4IFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrFifo1b4ifg {
    #[doc = "0: NO_EFFECT"]
    IclrFifo1b4ifgNoEffect = 0,
    #[doc = "1: CLR"]
    IclrFifo1b4ifgClr = 1,
}
impl From<IclrFifo1b4ifg> for bool {
    #[inline(always)]
    fn from(variant: IclrFifo1b4ifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_FIFO1B4IFG` writer - Clears FIFO1B4IFG in RIS register"]
pub type IclrFifo1b4ifgW<'a, REG> = crate::BitWriter<'a, REG, IclrFifo1b4ifg>;
impl<'a, REG> IclrFifo1b4ifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_fifo1b4ifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrFifo1b4ifg::IclrFifo1b4ifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_fifo1b4ifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrFifo1b4ifg::IclrFifo1b4ifgClr)
    }
}
#[doc = "Clears FIFO1B2IFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrFifo1b2ifg {
    #[doc = "0: NO_EFFECT"]
    IclrFifo1b2ifgNoEffect = 0,
    #[doc = "1: CLR"]
    IclrFifo1b2ifgClr = 1,
}
impl From<IclrFifo1b2ifg> for bool {
    #[inline(always)]
    fn from(variant: IclrFifo1b2ifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_FIFO1B2IFG` writer - Clears FIFO1B2IFG in RIS register"]
pub type IclrFifo1b2ifgW<'a, REG> = crate::BitWriter<'a, REG, IclrFifo1b2ifg>;
impl<'a, REG> IclrFifo1b2ifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_fifo1b2ifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrFifo1b2ifg::IclrFifo1b2ifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_fifo1b2ifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrFifo1b2ifg::IclrFifo1b2ifgClr)
    }
}
#[doc = "Clears FIFO3B4IFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrFifo3b4ifg {
    #[doc = "0: NO_EFFECT"]
    IclrFifo3b4ifgNoEffect = 0,
    #[doc = "1: CLR"]
    IclrFifo3b4ifgClr = 1,
}
impl From<IclrFifo3b4ifg> for bool {
    #[inline(always)]
    fn from(variant: IclrFifo3b4ifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_FIFO3B4IFG` writer - Clears FIFO3B4IFG in RIS register"]
pub type IclrFifo3b4ifgW<'a, REG> = crate::BitWriter<'a, REG, IclrFifo3b4ifg>;
impl<'a, REG> IclrFifo3b4ifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_fifo3b4ifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrFifo3b4ifg::IclrFifo3b4ifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_fifo3b4ifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrFifo3b4ifg::IclrFifo3b4ifgClr)
    }
}
#[doc = "Clears FIFOEMPTYIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrFifoemptyifg {
    #[doc = "0: NO_EFFECT"]
    IclrFifoemptyifgNoEffect = 0,
    #[doc = "1: CLR"]
    IclrFifoemptyifgClr = 1,
}
impl From<IclrFifoemptyifg> for bool {
    #[inline(always)]
    fn from(variant: IclrFifoemptyifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_FIFOEMPTYIFG` writer - Clears FIFOEMPTYIFG in RIS register"]
pub type IclrFifoemptyifgW<'a, REG> = crate::BitWriter<'a, REG, IclrFifoemptyifg>;
impl<'a, REG> IclrFifoemptyifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_fifoemptyifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrFifoemptyifg::IclrFifoemptyifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_fifoemptyifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrFifoemptyifg::IclrFifoemptyifgClr)
    }
}
#[doc = "Clears FIFOURUNIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrFifourunifg {
    #[doc = "0: NO_EFFECT"]
    IclrFifourunifgNoEffect = 0,
    #[doc = "1: CLR"]
    IclrFifourunifgClr = 1,
}
impl From<IclrFifourunifg> for bool {
    #[inline(always)]
    fn from(variant: IclrFifourunifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_FIFOURUNIFG` writer - Clears FIFOURUNIFG in RIS register"]
pub type IclrFifourunifgW<'a, REG> = crate::BitWriter<'a, REG, IclrFifourunifg>;
impl<'a, REG> IclrFifourunifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_fifourunifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrFifourunifg::IclrFifourunifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_fifourunifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrFifourunifg::IclrFifourunifgClr)
    }
}
#[doc = "Clears DMADONEIFG in RIS register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrDmadoneifg {
    #[doc = "0: NO_EFFECT"]
    IclrDmadoneifgNoEffect = 0,
    #[doc = "1: CLR"]
    IclrDmadoneifgClr = 1,
}
impl From<IclrDmadoneifg> for bool {
    #[inline(always)]
    fn from(variant: IclrDmadoneifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_DMADONEIFG` writer - Clears DMADONEIFG in RIS register"]
pub type IclrDmadoneifgW<'a, REG> = crate::BitWriter<'a, REG, IclrDmadoneifg>;
impl<'a, REG> IclrDmadoneifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_dmadoneifg_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrDmadoneifg::IclrDmadoneifgNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_dmadoneifg_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrDmadoneifg::IclrDmadoneifgClr)
    }
}
impl W {
    #[doc = "Bit 1 - Clears MODRDYIFG in RIS register"]
    #[inline(always)]
    pub fn iclr_modrdyifg(&mut self) -> IclrModrdyifgW<IclrSpec> {
        IclrModrdyifgW::new(self, 1)
    }
    #[doc = "Bit 8 - Clears FIFOFULLIFG in RIS register"]
    #[inline(always)]
    pub fn iclr_fifofullifg(&mut self) -> IclrFifofullifgW<IclrSpec> {
        IclrFifofullifgW::new(self, 8)
    }
    #[doc = "Bit 9 - Clears FIFO1B4IFG in RIS register"]
    #[inline(always)]
    pub fn iclr_fifo1b4ifg(&mut self) -> IclrFifo1b4ifgW<IclrSpec> {
        IclrFifo1b4ifgW::new(self, 9)
    }
    #[doc = "Bit 10 - Clears FIFO1B2IFG in RIS register"]
    #[inline(always)]
    pub fn iclr_fifo1b2ifg(&mut self) -> IclrFifo1b2ifgW<IclrSpec> {
        IclrFifo1b2ifgW::new(self, 10)
    }
    #[doc = "Bit 11 - Clears FIFO3B4IFG in RIS register"]
    #[inline(always)]
    pub fn iclr_fifo3b4ifg(&mut self) -> IclrFifo3b4ifgW<IclrSpec> {
        IclrFifo3b4ifgW::new(self, 11)
    }
    #[doc = "Bit 12 - Clears FIFOEMPTYIFG in RIS register"]
    #[inline(always)]
    pub fn iclr_fifoemptyifg(&mut self) -> IclrFifoemptyifgW<IclrSpec> {
        IclrFifoemptyifgW::new(self, 12)
    }
    #[doc = "Bit 13 - Clears FIFOURUNIFG in RIS register"]
    #[inline(always)]
    pub fn iclr_fifourunifg(&mut self) -> IclrFifourunifgW<IclrSpec> {
        IclrFifourunifgW::new(self, 13)
    }
    #[doc = "Bit 14 - Clears DMADONEIFG in RIS register"]
    #[inline(always)]
    pub fn iclr_dmadoneifg(&mut self) -> IclrDmadoneifgW<IclrSpec> {
        IclrDmadoneifgW::new(self, 14)
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
