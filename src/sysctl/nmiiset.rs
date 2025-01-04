#[doc = "Register `NMIISET` writer"]
pub type W = crate::W<NmiisetSpec>;
#[doc = "Set the BORLVL NMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NmiisetBorlvl {
    #[doc = "0: NO_EFFECT"]
    NmiisetBorlvlNoEffect = 0,
    #[doc = "1: SET"]
    NmiisetBorlvlSet = 1,
}
impl From<NmiisetBorlvl> for bool {
    #[inline(always)]
    fn from(variant: NmiisetBorlvl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIISET_BORLVL` writer - Set the BORLVL NMI"]
pub type NmiisetBorlvlW<'a, REG> = crate::BitWriter<'a, REG, NmiisetBorlvl>;
impl<'a, REG> NmiisetBorlvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn nmiiset_borlvl_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(NmiisetBorlvl::NmiisetBorlvlNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn nmiiset_borlvl_set(self) -> &'a mut crate::W<REG> {
        self.variant(NmiisetBorlvl::NmiisetBorlvlSet)
    }
}
#[doc = "Watch Dog 0 Fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NmiisetWwdt0 {
    #[doc = "0: NO_EFFECT"]
    NmiisetWwdt0NoEffect = 0,
    #[doc = "1: SET"]
    NmiisetWwdt0Set = 1,
}
impl From<NmiisetWwdt0> for bool {
    #[inline(always)]
    fn from(variant: NmiisetWwdt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIISET_WWDT0` writer - Watch Dog 0 Fault"]
pub type NmiisetWwdt0W<'a, REG> = crate::BitWriter<'a, REG, NmiisetWwdt0>;
impl<'a, REG> NmiisetWwdt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn nmiiset_wwdt0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(NmiisetWwdt0::NmiisetWwdt0NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn nmiiset_wwdt0_set(self) -> &'a mut crate::W<REG> {
        self.variant(NmiisetWwdt0::NmiisetWwdt0Set)
    }
}
#[doc = "Watch Dog 0 Fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NmiisetWwdt1 {
    #[doc = "0: NO_EFFECT"]
    NmiisetWwdt1NoEffect = 0,
    #[doc = "1: SET"]
    NmiisetWwdt1Set = 1,
}
impl From<NmiisetWwdt1> for bool {
    #[inline(always)]
    fn from(variant: NmiisetWwdt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIISET_WWDT1` writer - Watch Dog 0 Fault"]
pub type NmiisetWwdt1W<'a, REG> = crate::BitWriter<'a, REG, NmiisetWwdt1>;
impl<'a, REG> NmiisetWwdt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn nmiiset_wwdt1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(NmiisetWwdt1::NmiisetWwdt1NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn nmiiset_wwdt1_set(self) -> &'a mut crate::W<REG> {
        self.variant(NmiisetWwdt1::NmiisetWwdt1Set)
    }
}
#[doc = "LFXT-EXLF Monitor Fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NmiisetLfclkfail {
    #[doc = "0: NO_EFFECT"]
    NmiisetLfclkfailNoEffect = 0,
    #[doc = "1: SET"]
    NmiisetLfclkfailSet = 1,
}
impl From<NmiisetLfclkfail> for bool {
    #[inline(always)]
    fn from(variant: NmiisetLfclkfail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIISET_LFCLKFAIL` writer - LFXT-EXLF Monitor Fail"]
pub type NmiisetLfclkfailW<'a, REG> = crate::BitWriter<'a, REG, NmiisetLfclkfail>;
impl<'a, REG> NmiisetLfclkfailW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn nmiiset_lfclkfail_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(NmiisetLfclkfail::NmiisetLfclkfailNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn nmiiset_lfclkfail_set(self) -> &'a mut crate::W<REG> {
        self.variant(NmiisetLfclkfail::NmiisetLfclkfailSet)
    }
}
#[doc = "Flash Double Error Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NmiisetFlashded {
    #[doc = "0: NO_EFFECT"]
    NmiisetFlashdedNoEffect = 0,
    #[doc = "1: SET"]
    NmiisetFlashdedSet = 1,
}
impl From<NmiisetFlashded> for bool {
    #[inline(always)]
    fn from(variant: NmiisetFlashded) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIISET_FLASHDED` writer - Flash Double Error Detect"]
pub type NmiisetFlashdedW<'a, REG> = crate::BitWriter<'a, REG, NmiisetFlashded>;
impl<'a, REG> NmiisetFlashdedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn nmiiset_flashded_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(NmiisetFlashded::NmiisetFlashdedNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn nmiiset_flashded_set(self) -> &'a mut crate::W<REG> {
        self.variant(NmiisetFlashded::NmiisetFlashdedSet)
    }
}
#[doc = "SRAM Double Error Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NmiisetSramded {
    #[doc = "0: NO_EFFECT"]
    NmiisetSramdedNoEffect = 0,
    #[doc = "1: SET"]
    NmiisetSramdedSet = 1,
}
impl From<NmiisetSramded> for bool {
    #[inline(always)]
    fn from(variant: NmiisetSramded) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIISET_SRAMDED` writer - SRAM Double Error Detect"]
pub type NmiisetSramdedW<'a, REG> = crate::BitWriter<'a, REG, NmiisetSramded>;
impl<'a, REG> NmiisetSramdedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn nmiiset_sramded_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(NmiisetSramded::NmiisetSramdedNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn nmiiset_sramded_set(self) -> &'a mut crate::W<REG> {
        self.variant(NmiisetSramded::NmiisetSramdedSet)
    }
}
impl W {
    #[doc = "Bit 0 - Set the BORLVL NMI"]
    #[inline(always)]
    pub fn nmiiset_borlvl(&mut self) -> NmiisetBorlvlW<NmiisetSpec> {
        NmiisetBorlvlW::new(self, 0)
    }
    #[doc = "Bit 1 - Watch Dog 0 Fault"]
    #[inline(always)]
    pub fn nmiiset_wwdt0(&mut self) -> NmiisetWwdt0W<NmiisetSpec> {
        NmiisetWwdt0W::new(self, 1)
    }
    #[doc = "Bit 2 - Watch Dog 0 Fault"]
    #[inline(always)]
    pub fn nmiiset_wwdt1(&mut self) -> NmiisetWwdt1W<NmiisetSpec> {
        NmiisetWwdt1W::new(self, 2)
    }
    #[doc = "Bit 3 - LFXT-EXLF Monitor Fail"]
    #[inline(always)]
    pub fn nmiiset_lfclkfail(&mut self) -> NmiisetLfclkfailW<NmiisetSpec> {
        NmiisetLfclkfailW::new(self, 3)
    }
    #[doc = "Bit 4 - Flash Double Error Detect"]
    #[inline(always)]
    pub fn nmiiset_flashded(&mut self) -> NmiisetFlashdedW<NmiisetSpec> {
        NmiisetFlashdedW::new(self, 4)
    }
    #[doc = "Bit 5 - SRAM Double Error Detect"]
    #[inline(always)]
    pub fn nmiiset_sramded(&mut self) -> NmiisetSramdedW<NmiisetSpec> {
        NmiisetSramdedW::new(self, 5)
    }
}
#[doc = "NMI interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmiiset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmiisetSpec;
impl crate::RegisterSpec for NmiisetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`nmiiset::W`](W) writer structure"]
impl crate::Writable for NmiisetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NMIISET to value 0"]
impl crate::Resettable for NmiisetSpec {
    const RESET_VALUE: u32 = 0;
}
