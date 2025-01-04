#[doc = "Register `NMIICLR` writer"]
pub type W = crate::W<NmiiclrSpec>;
#[doc = "Clr the BORLVL NMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NmiiclrBorlvl {
    #[doc = "0: NO_EFFECT"]
    NmiiclrBorlvlNoEffect = 0,
    #[doc = "1: CLR"]
    NmiiclrBorlvlClr = 1,
}
impl From<NmiiclrBorlvl> for bool {
    #[inline(always)]
    fn from(variant: NmiiclrBorlvl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIICLR_BORLVL` writer - Clr the BORLVL NMI"]
pub type NmiiclrBorlvlW<'a, REG> = crate::BitWriter<'a, REG, NmiiclrBorlvl>;
impl<'a, REG> NmiiclrBorlvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn nmiiclr_borlvl_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(NmiiclrBorlvl::NmiiclrBorlvlNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn nmiiclr_borlvl_clr(self) -> &'a mut crate::W<REG> {
        self.variant(NmiiclrBorlvl::NmiiclrBorlvlClr)
    }
}
#[doc = "Watch Dog 0 Fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NmiiclrWwdt0 {
    #[doc = "0: NO_EFFECT"]
    NmiiclrWwdt0NoEffect = 0,
    #[doc = "1: CLR"]
    NmiiclrWwdt0Clr = 1,
}
impl From<NmiiclrWwdt0> for bool {
    #[inline(always)]
    fn from(variant: NmiiclrWwdt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIICLR_WWDT0` writer - Watch Dog 0 Fault"]
pub type NmiiclrWwdt0W<'a, REG> = crate::BitWriter<'a, REG, NmiiclrWwdt0>;
impl<'a, REG> NmiiclrWwdt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn nmiiclr_wwdt0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(NmiiclrWwdt0::NmiiclrWwdt0NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn nmiiclr_wwdt0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(NmiiclrWwdt0::NmiiclrWwdt0Clr)
    }
}
#[doc = "Watch Dog 0 Fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NmiiclrWwdt1 {
    #[doc = "0: NO_EFFECT"]
    NmiiclrWwdt1NoEffect = 0,
    #[doc = "1: CLR"]
    NmiiclrWwdt1Clr = 1,
}
impl From<NmiiclrWwdt1> for bool {
    #[inline(always)]
    fn from(variant: NmiiclrWwdt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIICLR_WWDT1` writer - Watch Dog 0 Fault"]
pub type NmiiclrWwdt1W<'a, REG> = crate::BitWriter<'a, REG, NmiiclrWwdt1>;
impl<'a, REG> NmiiclrWwdt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn nmiiclr_wwdt1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(NmiiclrWwdt1::NmiiclrWwdt1NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn nmiiclr_wwdt1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(NmiiclrWwdt1::NmiiclrWwdt1Clr)
    }
}
#[doc = "LFXT-EXLF Monitor Fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NmiiclrLfclkfail {
    #[doc = "0: NO_EFFECT"]
    NmiiclrLfclkfailNoEffect = 0,
    #[doc = "1: CLR"]
    NmiiclrLfclkfailClr = 1,
}
impl From<NmiiclrLfclkfail> for bool {
    #[inline(always)]
    fn from(variant: NmiiclrLfclkfail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIICLR_LFCLKFAIL` writer - LFXT-EXLF Monitor Fail"]
pub type NmiiclrLfclkfailW<'a, REG> = crate::BitWriter<'a, REG, NmiiclrLfclkfail>;
impl<'a, REG> NmiiclrLfclkfailW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn nmiiclr_lfclkfail_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(NmiiclrLfclkfail::NmiiclrLfclkfailNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn nmiiclr_lfclkfail_clr(self) -> &'a mut crate::W<REG> {
        self.variant(NmiiclrLfclkfail::NmiiclrLfclkfailClr)
    }
}
#[doc = "Flash Double Error Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NmiiclrFlashded {
    #[doc = "0: NO_EFFECT"]
    NmiiclrFlashdedNoEffect = 0,
    #[doc = "1: CLR"]
    NmiiclrFlashdedClr = 1,
}
impl From<NmiiclrFlashded> for bool {
    #[inline(always)]
    fn from(variant: NmiiclrFlashded) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIICLR_FLASHDED` writer - Flash Double Error Detect"]
pub type NmiiclrFlashdedW<'a, REG> = crate::BitWriter<'a, REG, NmiiclrFlashded>;
impl<'a, REG> NmiiclrFlashdedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn nmiiclr_flashded_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(NmiiclrFlashded::NmiiclrFlashdedNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn nmiiclr_flashded_clr(self) -> &'a mut crate::W<REG> {
        self.variant(NmiiclrFlashded::NmiiclrFlashdedClr)
    }
}
#[doc = "SRAM Double Error Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NmiiclrSramded {
    #[doc = "0: NO_EFFECT"]
    NmiiclrSramdedNoEffect = 0,
    #[doc = "1: CLR"]
    NmiiclrSramdedClr = 1,
}
impl From<NmiiclrSramded> for bool {
    #[inline(always)]
    fn from(variant: NmiiclrSramded) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIICLR_SRAMDED` writer - SRAM Double Error Detect"]
pub type NmiiclrSramdedW<'a, REG> = crate::BitWriter<'a, REG, NmiiclrSramded>;
impl<'a, REG> NmiiclrSramdedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn nmiiclr_sramded_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(NmiiclrSramded::NmiiclrSramdedNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn nmiiclr_sramded_clr(self) -> &'a mut crate::W<REG> {
        self.variant(NmiiclrSramded::NmiiclrSramdedClr)
    }
}
impl W {
    #[doc = "Bit 0 - Clr the BORLVL NMI"]
    #[inline(always)]
    pub fn nmiiclr_borlvl(&mut self) -> NmiiclrBorlvlW<NmiiclrSpec> {
        NmiiclrBorlvlW::new(self, 0)
    }
    #[doc = "Bit 1 - Watch Dog 0 Fault"]
    #[inline(always)]
    pub fn nmiiclr_wwdt0(&mut self) -> NmiiclrWwdt0W<NmiiclrSpec> {
        NmiiclrWwdt0W::new(self, 1)
    }
    #[doc = "Bit 2 - Watch Dog 0 Fault"]
    #[inline(always)]
    pub fn nmiiclr_wwdt1(&mut self) -> NmiiclrWwdt1W<NmiiclrSpec> {
        NmiiclrWwdt1W::new(self, 2)
    }
    #[doc = "Bit 3 - LFXT-EXLF Monitor Fail"]
    #[inline(always)]
    pub fn nmiiclr_lfclkfail(&mut self) -> NmiiclrLfclkfailW<NmiiclrSpec> {
        NmiiclrLfclkfailW::new(self, 3)
    }
    #[doc = "Bit 4 - Flash Double Error Detect"]
    #[inline(always)]
    pub fn nmiiclr_flashded(&mut self) -> NmiiclrFlashdedW<NmiiclrSpec> {
        NmiiclrFlashdedW::new(self, 4)
    }
    #[doc = "Bit 5 - SRAM Double Error Detect"]
    #[inline(always)]
    pub fn nmiiclr_sramded(&mut self) -> NmiiclrSramdedW<NmiiclrSpec> {
        NmiiclrSramdedW::new(self, 5)
    }
}
#[doc = "NMI interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmiiclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmiiclrSpec;
impl crate::RegisterSpec for NmiiclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`nmiiclr::W`](W) writer structure"]
impl crate::Writable for NmiiclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NMIICLR to value 0"]
impl crate::Resettable for NmiiclrSpec {
    const RESET_VALUE: u32 = 0;
}
