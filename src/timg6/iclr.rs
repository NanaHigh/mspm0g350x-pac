#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "Zero event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrZ {
    #[doc = "0: NO_EFFECT"]
    IclrZNoEffect = 0,
    #[doc = "1: CLR"]
    IclrZClr = 1,
}
impl From<IclrZ> for bool {
    #[inline(always)]
    fn from(variant: IclrZ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_Z` writer - Zero event CLEAR"]
pub type IclrZW<'a, REG> = crate::BitWriter<'a, REG, IclrZ>;
impl<'a, REG> IclrZW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_z_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrZ::IclrZNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_z_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrZ::IclrZClr)
    }
}
#[doc = "Load event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrL {
    #[doc = "0: NO_EFFECT"]
    IclrLNoEffect = 0,
    #[doc = "1: CLR"]
    IclrLClr = 1,
}
impl From<IclrL> for bool {
    #[inline(always)]
    fn from(variant: IclrL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_L` writer - Load event CLEAR"]
pub type IclrLW<'a, REG> = crate::BitWriter<'a, REG, IclrL>;
impl<'a, REG> IclrLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_l_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrL::IclrLNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_l_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrL::IclrLClr)
    }
}
#[doc = "Capture or compare down event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrCcd0 {
    #[doc = "0: NO_EFFECT"]
    IclrCcd0NoEffect = 0,
    #[doc = "1: CLR"]
    IclrCcd0Clr = 1,
}
impl From<IclrCcd0> for bool {
    #[inline(always)]
    fn from(variant: IclrCcd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_CCD0` writer - Capture or compare down event CLEAR"]
pub type IclrCcd0W<'a, REG> = crate::BitWriter<'a, REG, IclrCcd0>;
impl<'a, REG> IclrCcd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_ccd0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcd0::IclrCcd0NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_ccd0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcd0::IclrCcd0Clr)
    }
}
#[doc = "Capture or compare down event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrCcd1 {
    #[doc = "0: NO_EFFECT"]
    IclrCcd1NoEffect = 0,
    #[doc = "1: CLR"]
    IclrCcd1Clr = 1,
}
impl From<IclrCcd1> for bool {
    #[inline(always)]
    fn from(variant: IclrCcd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_CCD1` writer - Capture or compare down event CLEAR"]
pub type IclrCcd1W<'a, REG> = crate::BitWriter<'a, REG, IclrCcd1>;
impl<'a, REG> IclrCcd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_ccd1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcd1::IclrCcd1NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_ccd1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcd1::IclrCcd1Clr)
    }
}
#[doc = "Capture or compare up event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrCcu0 {
    #[doc = "0: NO_EFFECT"]
    IclrCcu0NoEffect = 0,
    #[doc = "1: CLR"]
    IclrCcu0Clr = 1,
}
impl From<IclrCcu0> for bool {
    #[inline(always)]
    fn from(variant: IclrCcu0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_CCU0` writer - Capture or compare up event CLEAR"]
pub type IclrCcu0W<'a, REG> = crate::BitWriter<'a, REG, IclrCcu0>;
impl<'a, REG> IclrCcu0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_ccu0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcu0::IclrCcu0NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_ccu0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcu0::IclrCcu0Clr)
    }
}
#[doc = "Capture or compare up event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrCcu1 {
    #[doc = "0: NO_EFFECT"]
    IclrCcu1NoEffect = 0,
    #[doc = "1: CLR"]
    IclrCcu1Clr = 1,
}
impl From<IclrCcu1> for bool {
    #[inline(always)]
    fn from(variant: IclrCcu1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_CCU1` writer - Capture or compare up event CLEAR"]
pub type IclrCcu1W<'a, REG> = crate::BitWriter<'a, REG, IclrCcu1>;
impl<'a, REG> IclrCcu1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_ccu1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcu1::IclrCcu1NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_ccu1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcu1::IclrCcu1Clr)
    }
}
#[doc = "Trigger Overflow event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrTov {
    #[doc = "0: NO_EFFECT"]
    IclrTovNoEffect = 0,
    #[doc = "1: CLR"]
    IclrTovClr = 1,
}
impl From<IclrTov> for bool {
    #[inline(always)]
    fn from(variant: IclrTov) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_TOV` writer - Trigger Overflow event CLEAR"]
pub type IclrTovW<'a, REG> = crate::BitWriter<'a, REG, IclrTov>;
impl<'a, REG> IclrTovW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_tov_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrTov::IclrTovNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_tov_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrTov::IclrTovClr)
    }
}
impl W {
    #[doc = "Bit 0 - Zero event CLEAR"]
    #[inline(always)]
    pub fn iclr_z(&mut self) -> IclrZW<IclrSpec> {
        IclrZW::new(self, 0)
    }
    #[doc = "Bit 1 - Load event CLEAR"]
    #[inline(always)]
    pub fn iclr_l(&mut self) -> IclrLW<IclrSpec> {
        IclrLW::new(self, 1)
    }
    #[doc = "Bit 4 - Capture or compare down event CLEAR"]
    #[inline(always)]
    pub fn iclr_ccd0(&mut self) -> IclrCcd0W<IclrSpec> {
        IclrCcd0W::new(self, 4)
    }
    #[doc = "Bit 5 - Capture or compare down event CLEAR"]
    #[inline(always)]
    pub fn iclr_ccd1(&mut self) -> IclrCcd1W<IclrSpec> {
        IclrCcd1W::new(self, 5)
    }
    #[doc = "Bit 8 - Capture or compare up event CLEAR"]
    #[inline(always)]
    pub fn iclr_ccu0(&mut self) -> IclrCcu0W<IclrSpec> {
        IclrCcu0W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture or compare up event CLEAR"]
    #[inline(always)]
    pub fn iclr_ccu1(&mut self) -> IclrCcu1W<IclrSpec> {
        IclrCcu1W::new(self, 9)
    }
    #[doc = "Bit 25 - Trigger Overflow event CLEAR"]
    #[inline(always)]
    pub fn iclr_tov(&mut self) -> IclrTovW<IclrSpec> {
        IclrTovW::new(self, 25)
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
