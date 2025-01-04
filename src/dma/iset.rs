#[doc = "Register `ISET` writer"]
pub type W = crate::W<IsetSpec>;
#[doc = "DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetDmach0 {
    #[doc = "0: NO_EFFECT"]
    IsetDmach0NoEffect = 0,
    #[doc = "1: SET"]
    IsetDmach0Set = 1,
}
impl From<IsetDmach0> for bool {
    #[inline(always)]
    fn from(variant: IsetDmach0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_DMACH0` writer - DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type IsetDmach0W<'a, REG> = crate::BitWriter<'a, REG, IsetDmach0>;
impl<'a, REG> IsetDmach0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_dmach0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDmach0::IsetDmach0NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_dmach0_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDmach0::IsetDmach0Set)
    }
}
#[doc = "DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetDmach1 {
    #[doc = "0: NO_EFFECT"]
    IsetDmach1NoEffect = 0,
    #[doc = "1: SET"]
    IsetDmach1Set = 1,
}
impl From<IsetDmach1> for bool {
    #[inline(always)]
    fn from(variant: IsetDmach1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_DMACH1` writer - DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type IsetDmach1W<'a, REG> = crate::BitWriter<'a, REG, IsetDmach1>;
impl<'a, REG> IsetDmach1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_dmach1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDmach1::IsetDmach1NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_dmach1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDmach1::IsetDmach1Set)
    }
}
#[doc = "DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetDmach2 {
    #[doc = "0: NO_EFFECT"]
    IsetDmach2NoEffect = 0,
    #[doc = "1: SET"]
    IsetDmach2Set = 1,
}
impl From<IsetDmach2> for bool {
    #[inline(always)]
    fn from(variant: IsetDmach2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_DMACH2` writer - DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type IsetDmach2W<'a, REG> = crate::BitWriter<'a, REG, IsetDmach2>;
impl<'a, REG> IsetDmach2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_dmach2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDmach2::IsetDmach2NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_dmach2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDmach2::IsetDmach2Set)
    }
}
#[doc = "DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetDmach3 {
    #[doc = "0: NO_EFFECT"]
    IsetDmach3NoEffect = 0,
    #[doc = "1: SET"]
    IsetDmach3Set = 1,
}
impl From<IsetDmach3> for bool {
    #[inline(always)]
    fn from(variant: IsetDmach3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_DMACH3` writer - DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type IsetDmach3W<'a, REG> = crate::BitWriter<'a, REG, IsetDmach3>;
impl<'a, REG> IsetDmach3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_dmach3_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDmach3::IsetDmach3NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_dmach3_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDmach3::IsetDmach3Set)
    }
}
#[doc = "DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetDmach4 {
    #[doc = "0: NO_EFFECT"]
    IsetDmach4NoEffect = 0,
    #[doc = "1: SET"]
    IsetDmach4Set = 1,
}
impl From<IsetDmach4> for bool {
    #[inline(always)]
    fn from(variant: IsetDmach4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_DMACH4` writer - DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type IsetDmach4W<'a, REG> = crate::BitWriter<'a, REG, IsetDmach4>;
impl<'a, REG> IsetDmach4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_dmach4_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDmach4::IsetDmach4NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_dmach4_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDmach4::IsetDmach4Set)
    }
}
#[doc = "DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetDmach5 {
    #[doc = "0: NO_EFFECT"]
    IsetDmach5NoEffect = 0,
    #[doc = "1: SET"]
    IsetDmach5Set = 1,
}
impl From<IsetDmach5> for bool {
    #[inline(always)]
    fn from(variant: IsetDmach5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_DMACH5` writer - DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type IsetDmach5W<'a, REG> = crate::BitWriter<'a, REG, IsetDmach5>;
impl<'a, REG> IsetDmach5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_dmach5_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDmach5::IsetDmach5NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_dmach5_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDmach5::IsetDmach5Set)
    }
}
#[doc = "DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetDmach6 {
    #[doc = "0: NO_EFFECT"]
    IsetDmach6NoEffect = 0,
    #[doc = "1: SET"]
    IsetDmach6Set = 1,
}
impl From<IsetDmach6> for bool {
    #[inline(always)]
    fn from(variant: IsetDmach6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_DMACH6` writer - DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0)."]
pub type IsetDmach6W<'a, REG> = crate::BitWriter<'a, REG, IsetDmach6>;
impl<'a, REG> IsetDmach6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_dmach6_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDmach6::IsetDmach6NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_dmach6_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDmach6::IsetDmach6Set)
    }
}
#[doc = "Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetPreirqch0 {
    #[doc = "0: CLR"]
    IsetPreirqch0Clr = 0,
    #[doc = "1: SET"]
    IsetPreirqch0Set = 1,
}
impl From<IsetPreirqch0> for bool {
    #[inline(always)]
    fn from(variant: IsetPreirqch0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_PREIRQCH0` writer - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
pub type IsetPreirqch0W<'a, REG> = crate::BitWriter<'a, REG, IsetPreirqch0>;
impl<'a, REG> IsetPreirqch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iset_preirqch0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IsetPreirqch0::IsetPreirqch0Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_preirqch0_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetPreirqch0::IsetPreirqch0Set)
    }
}
#[doc = "Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetPreirqch1 {
    #[doc = "0: CLR"]
    IsetPreirqch1Clr = 0,
    #[doc = "1: SET"]
    IsetPreirqch1Set = 1,
}
impl From<IsetPreirqch1> for bool {
    #[inline(always)]
    fn from(variant: IsetPreirqch1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_PREIRQCH1` writer - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
pub type IsetPreirqch1W<'a, REG> = crate::BitWriter<'a, REG, IsetPreirqch1>;
impl<'a, REG> IsetPreirqch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iset_preirqch1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IsetPreirqch1::IsetPreirqch1Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_preirqch1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetPreirqch1::IsetPreirqch1Set)
    }
}
#[doc = "Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetPreirqch2 {
    #[doc = "0: CLR"]
    IsetPreirqch2Clr = 0,
    #[doc = "1: SET"]
    IsetPreirqch2Set = 1,
}
impl From<IsetPreirqch2> for bool {
    #[inline(always)]
    fn from(variant: IsetPreirqch2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_PREIRQCH2` writer - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
pub type IsetPreirqch2W<'a, REG> = crate::BitWriter<'a, REG, IsetPreirqch2>;
impl<'a, REG> IsetPreirqch2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iset_preirqch2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IsetPreirqch2::IsetPreirqch2Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_preirqch2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetPreirqch2::IsetPreirqch2Set)
    }
}
#[doc = "DMA address error, SRC address not reachable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetAddrerr {
    #[doc = "0: CLR"]
    IsetAddrerrClr = 0,
    #[doc = "1: SET"]
    IsetAddrerrSet = 1,
}
impl From<IsetAddrerr> for bool {
    #[inline(always)]
    fn from(variant: IsetAddrerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_ADDRERR` writer - DMA address error, SRC address not reachable."]
pub type IsetAddrerrW<'a, REG> = crate::BitWriter<'a, REG, IsetAddrerr>;
impl<'a, REG> IsetAddrerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iset_addrerr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IsetAddrerr::IsetAddrerrClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_addrerr_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetAddrerr::IsetAddrerrSet)
    }
}
#[doc = "DMA data error, SRC data might be corrupted (PAR or ECC error).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetDataerr {
    #[doc = "0: CLR"]
    IsetDataerrClr = 0,
    #[doc = "1: SET"]
    IsetDataerrSet = 1,
}
impl From<IsetDataerr> for bool {
    #[inline(always)]
    fn from(variant: IsetDataerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_DATAERR` writer - DMA data error, SRC data might be corrupted (PAR or ECC error)."]
pub type IsetDataerrW<'a, REG> = crate::BitWriter<'a, REG, IsetDataerr>;
impl<'a, REG> IsetDataerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iset_dataerr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDataerr::IsetDataerrClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_dataerr_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDataerr::IsetDataerrSet)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel 0 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn iset_dmach0(&mut self) -> IsetDmach0W<IsetSpec> {
        IsetDmach0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn iset_dmach1(&mut self) -> IsetDmach1W<IsetSpec> {
        IsetDmach1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA Channel 2 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn iset_dmach2(&mut self) -> IsetDmach2W<IsetSpec> {
        IsetDmach2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Channel 3 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn iset_dmach3(&mut self) -> IsetDmach3W<IsetSpec> {
        IsetDmach3W::new(self, 3)
    }
    #[doc = "Bit 4 - DMA Channel 4 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn iset_dmach4(&mut self) -> IsetDmach4W<IsetSpec> {
        IsetDmach4W::new(self, 4)
    }
    #[doc = "Bit 5 - DMA Channel 5 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn iset_dmach5(&mut self) -> IsetDmach5W<IsetSpec> {
        IsetDmach5W::new(self, 5)
    }
    #[doc = "Bit 6 - DMA Channel 6 interrupt signals that size counter reached zero (DMASZ=0)."]
    #[inline(always)]
    pub fn iset_dmach6(&mut self) -> IsetDmach6W<IsetSpec> {
        IsetDmach6W::new(self, 6)
    }
    #[doc = "Bit 16 - Pre-IRQ for Channel 0. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn iset_preirqch0(&mut self) -> IsetPreirqch0W<IsetSpec> {
        IsetPreirqch0W::new(self, 16)
    }
    #[doc = "Bit 17 - Pre-IRQ for Channel 1. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn iset_preirqch1(&mut self) -> IsetPreirqch1W<IsetSpec> {
        IsetPreirqch1W::new(self, 17)
    }
    #[doc = "Bit 18 - Pre-IRQ for Channel 2. Size counter reached Pre-IRQ threshold."]
    #[inline(always)]
    pub fn iset_preirqch2(&mut self) -> IsetPreirqch2W<IsetSpec> {
        IsetPreirqch2W::new(self, 18)
    }
    #[doc = "Bit 24 - DMA address error, SRC address not reachable."]
    #[inline(always)]
    pub fn iset_addrerr(&mut self) -> IsetAddrerrW<IsetSpec> {
        IsetAddrerrW::new(self, 24)
    }
    #[doc = "Bit 25 - DMA data error, SRC data might be corrupted (PAR or ECC error)."]
    #[inline(always)]
    pub fn iset_dataerr(&mut self) -> IsetDataerrW<IsetSpec> {
        IsetDataerrW::new(self, 25)
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
