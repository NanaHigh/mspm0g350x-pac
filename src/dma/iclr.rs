#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrDmach0 {
    #[doc = "0: NO_EFFECT"]
    IclrDmach0NoEffect = 0,
    #[doc = "1: CLR"]
    IclrDmach0Clr = 1,
}
impl From<IclrDmach0> for bool {
    #[inline(always)]
    fn from(variant: IclrDmach0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_DMACH0` writer - DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type IclrDmach0W<'a, REG> = crate::BitWriter<'a, REG, IclrDmach0>;
impl<'a, REG> IclrDmach0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_dmach0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrDmach0::IclrDmach0NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_dmach0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrDmach0::IclrDmach0Clr)
    }
}
#[doc = "DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrDmach1 {
    #[doc = "0: NO_EFFECT"]
    IclrDmach1NoEffect = 0,
    #[doc = "1: CLR"]
    IclrDmach1Clr = 1,
}
impl From<IclrDmach1> for bool {
    #[inline(always)]
    fn from(variant: IclrDmach1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_DMACH1` writer - DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type IclrDmach1W<'a, REG> = crate::BitWriter<'a, REG, IclrDmach1>;
impl<'a, REG> IclrDmach1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_dmach1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrDmach1::IclrDmach1NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_dmach1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrDmach1::IclrDmach1Clr)
    }
}
#[doc = "DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrDmach2 {
    #[doc = "0: NO_EFFECT"]
    IclrDmach2NoEffect = 0,
    #[doc = "1: CLR"]
    IclrDmach2Clr = 1,
}
impl From<IclrDmach2> for bool {
    #[inline(always)]
    fn from(variant: IclrDmach2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_DMACH2` writer - DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type IclrDmach2W<'a, REG> = crate::BitWriter<'a, REG, IclrDmach2>;
impl<'a, REG> IclrDmach2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_dmach2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrDmach2::IclrDmach2NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_dmach2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrDmach2::IclrDmach2Clr)
    }
}
#[doc = "DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrDmach3 {
    #[doc = "0: NO_EFFECT"]
    IclrDmach3NoEffect = 0,
    #[doc = "1: CLR"]
    IclrDmach3Clr = 1,
}
impl From<IclrDmach3> for bool {
    #[inline(always)]
    fn from(variant: IclrDmach3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_DMACH3` writer - DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type IclrDmach3W<'a, REG> = crate::BitWriter<'a, REG, IclrDmach3>;
impl<'a, REG> IclrDmach3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_dmach3_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrDmach3::IclrDmach3NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_dmach3_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrDmach3::IclrDmach3Clr)
    }
}
#[doc = "DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrDmach4 {
    #[doc = "0: NO_EFFECT"]
    IclrDmach4NoEffect = 0,
    #[doc = "1: CLR"]
    IclrDmach4Clr = 1,
}
impl From<IclrDmach4> for bool {
    #[inline(always)]
    fn from(variant: IclrDmach4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_DMACH4` writer - DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type IclrDmach4W<'a, REG> = crate::BitWriter<'a, REG, IclrDmach4>;
impl<'a, REG> IclrDmach4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_dmach4_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrDmach4::IclrDmach4NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_dmach4_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrDmach4::IclrDmach4Clr)
    }
}
#[doc = "DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrDmach5 {
    #[doc = "0: NO_EFFECT"]
    IclrDmach5NoEffect = 0,
    #[doc = "1: CLR"]
    IclrDmach5Clr = 1,
}
impl From<IclrDmach5> for bool {
    #[inline(always)]
    fn from(variant: IclrDmach5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_DMACH5` writer - DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type IclrDmach5W<'a, REG> = crate::BitWriter<'a, REG, IclrDmach5>;
impl<'a, REG> IclrDmach5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_dmach5_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrDmach5::IclrDmach5NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_dmach5_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrDmach5::IclrDmach5Clr)
    }
}
#[doc = "DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrDmach6 {
    #[doc = "0: NO_EFFECT"]
    IclrDmach6NoEffect = 0,
    #[doc = "1: CLR"]
    IclrDmach6Clr = 1,
}
impl From<IclrDmach6> for bool {
    #[inline(always)]
    fn from(variant: IclrDmach6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_DMACH6` writer - DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type IclrDmach6W<'a, REG> = crate::BitWriter<'a, REG, IclrDmach6>;
impl<'a, REG> IclrDmach6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_dmach6_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrDmach6::IclrDmach6NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_dmach6_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrDmach6::IclrDmach6Clr)
    }
}
#[doc = "Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrPreirqch0 {
    #[doc = "0: CLR"]
    IclrPreirqch0Clr = 0,
    #[doc = "1: SET"]
    IclrPreirqch0Set = 1,
}
impl From<IclrPreirqch0> for bool {
    #[inline(always)]
    fn from(variant: IclrPreirqch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_PREIRQCH0` writer - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
pub type IclrPreirqch0W<'a, REG> = crate::BitWriter<'a, REG, IclrPreirqch0>;
impl<'a, REG> IclrPreirqch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_preirqch0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrPreirqch0::IclrPreirqch0Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iclr_preirqch0_set(self) -> &'a mut crate::W<REG> {
        self.variant(IclrPreirqch0::IclrPreirqch0Set)
    }
}
#[doc = "Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrPreirqch1 {
    #[doc = "0: CLR"]
    IclrPreirqch1Clr = 0,
    #[doc = "1: SET"]
    IclrPreirqch1Set = 1,
}
impl From<IclrPreirqch1> for bool {
    #[inline(always)]
    fn from(variant: IclrPreirqch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_PREIRQCH1` writer - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
pub type IclrPreirqch1W<'a, REG> = crate::BitWriter<'a, REG, IclrPreirqch1>;
impl<'a, REG> IclrPreirqch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_preirqch1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrPreirqch1::IclrPreirqch1Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iclr_preirqch1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IclrPreirqch1::IclrPreirqch1Set)
    }
}
#[doc = "Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrPreirqch2 {
    #[doc = "0: CLR"]
    IclrPreirqch2Clr = 0,
    #[doc = "1: SET"]
    IclrPreirqch2Set = 1,
}
impl From<IclrPreirqch2> for bool {
    #[inline(always)]
    fn from(variant: IclrPreirqch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_PREIRQCH2` writer - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
pub type IclrPreirqch2W<'a, REG> = crate::BitWriter<'a, REG, IclrPreirqch2>;
impl<'a, REG> IclrPreirqch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_preirqch2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrPreirqch2::IclrPreirqch2Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iclr_preirqch2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IclrPreirqch2::IclrPreirqch2Set)
    }
}
#[doc = "DMA address error, SRC address not reachable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrAddrerr {
    #[doc = "0: CLR"]
    IclrAddrerrClr = 0,
    #[doc = "1: SET"]
    IclrAddrerrSet = 1,
}
impl From<IclrAddrerr> for bool {
    #[inline(always)]
    fn from(variant: IclrAddrerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_ADDRERR` writer - DMA address error, SRC address not reachable."]
pub type IclrAddrerrW<'a, REG> = crate::BitWriter<'a, REG, IclrAddrerr>;
impl<'a, REG> IclrAddrerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_addrerr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrAddrerr::IclrAddrerrClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iclr_addrerr_set(self) -> &'a mut crate::W<REG> {
        self.variant(IclrAddrerr::IclrAddrerrSet)
    }
}
#[doc = "DMA data error, SRC data might be corrupted (PAR or ECC error).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrDataerr {
    #[doc = "0: CLR"]
    IclrDataerrClr = 0,
    #[doc = "1: SET"]
    IclrDataerrSet = 1,
}
impl From<IclrDataerr> for bool {
    #[inline(always)]
    fn from(variant: IclrDataerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_DATAERR` writer - DMA data error, SRC data might be corrupted (PAR or ECC error)."]
pub type IclrDataerrW<'a, REG> = crate::BitWriter<'a, REG, IclrDataerr>;
impl<'a, REG> IclrDataerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_dataerr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrDataerr::IclrDataerrClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iclr_dataerr_set(self) -> &'a mut crate::W<REG> {
        self.variant(IclrDataerr::IclrDataerrSet)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn iclr_dmach0(&mut self) -> IclrDmach0W<IclrSpec> {
        IclrDmach0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn iclr_dmach1(&mut self) -> IclrDmach1W<IclrSpec> {
        IclrDmach1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn iclr_dmach2(&mut self) -> IclrDmach2W<IclrSpec> {
        IclrDmach2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn iclr_dmach3(&mut self) -> IclrDmach3W<IclrSpec> {
        IclrDmach3W::new(self, 3)
    }
    #[doc = "Bit 4 - DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn iclr_dmach4(&mut self) -> IclrDmach4W<IclrSpec> {
        IclrDmach4W::new(self, 4)
    }
    #[doc = "Bit 5 - DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn iclr_dmach5(&mut self) -> IclrDmach5W<IclrSpec> {
        IclrDmach5W::new(self, 5)
    }
    #[doc = "Bit 6 - DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn iclr_dmach6(&mut self) -> IclrDmach6W<IclrSpec> {
        IclrDmach6W::new(self, 6)
    }
    #[doc = "Bit 16 - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn iclr_preirqch0(&mut self) -> IclrPreirqch0W<IclrSpec> {
        IclrPreirqch0W::new(self, 16)
    }
    #[doc = "Bit 17 - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn iclr_preirqch1(&mut self) -> IclrPreirqch1W<IclrSpec> {
        IclrPreirqch1W::new(self, 17)
    }
    #[doc = "Bit 18 - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn iclr_preirqch2(&mut self) -> IclrPreirqch2W<IclrSpec> {
        IclrPreirqch2W::new(self, 18)
    }
    #[doc = "Bit 24 - DMA address error, SRC address not reachable."]
    #[inline(always)]
    pub fn iclr_addrerr(&mut self) -> IclrAddrerrW<IclrSpec> {
        IclrAddrerrW::new(self, 24)
    }
    #[doc = "Bit 25 - DMA data error, SRC data might be corrupted (PAR or ECC error)."]
    #[inline(always)]
    pub fn iclr_dataerr(&mut self) -> IclrDataerrW<IclrSpec> {
        IclrDataerrW::new(self, 25)
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
