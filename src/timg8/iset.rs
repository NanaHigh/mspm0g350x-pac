#[doc = "Register `ISET` writer"]
pub type W = crate::W<IsetSpec>;
#[doc = "Zero event SET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetZ {
    #[doc = "0: NO_EFFECT"]
    IsetZNoEffect = 0,
    #[doc = "1: SET"]
    IsetZSet = 1,
}
impl From<IsetZ> for bool {
    #[inline(always)]
    fn from(variant: IsetZ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_Z` writer - Zero event SET"]
pub type IsetZW<'a, REG> = crate::BitWriter<'a, REG, IsetZ>;
impl<'a, REG> IsetZW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_z_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetZ::IsetZNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_z_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetZ::IsetZSet)
    }
}
#[doc = "Load event SET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetL {
    #[doc = "0: NO_EFFECT"]
    IsetLNoEffect = 0,
    #[doc = "1: SET"]
    IsetLSet = 1,
}
impl From<IsetL> for bool {
    #[inline(always)]
    fn from(variant: IsetL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_L` writer - Load event SET"]
pub type IsetLW<'a, REG> = crate::BitWriter<'a, REG, IsetL>;
impl<'a, REG> IsetLW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_l_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetL::IsetLNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_l_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetL::IsetLSet)
    }
}
#[doc = "Capture or compare down event SET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetCcd0 {
    #[doc = "0: NO_EFFECT"]
    IsetCcd0NoEffect = 0,
    #[doc = "1: SET"]
    IsetCcd0Set = 1,
}
impl From<IsetCcd0> for bool {
    #[inline(always)]
    fn from(variant: IsetCcd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_CCD0` writer - Capture or compare down event SET"]
pub type IsetCcd0W<'a, REG> = crate::BitWriter<'a, REG, IsetCcd0>;
impl<'a, REG> IsetCcd0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_ccd0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetCcd0::IsetCcd0NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_ccd0_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetCcd0::IsetCcd0Set)
    }
}
#[doc = "Capture or compare down event SET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetCcd1 {
    #[doc = "0: NO_EFFECT"]
    IsetCcd1NoEffect = 0,
    #[doc = "1: SET"]
    IsetCcd1Set = 1,
}
impl From<IsetCcd1> for bool {
    #[inline(always)]
    fn from(variant: IsetCcd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_CCD1` writer - Capture or compare down event SET"]
pub type IsetCcd1W<'a, REG> = crate::BitWriter<'a, REG, IsetCcd1>;
impl<'a, REG> IsetCcd1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_ccd1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetCcd1::IsetCcd1NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_ccd1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetCcd1::IsetCcd1Set)
    }
}
#[doc = "Capture or compare up event SET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetCcu0 {
    #[doc = "0: NO_EFFECT"]
    IsetCcu0NoEffect = 0,
    #[doc = "1: SET"]
    IsetCcu0Set = 1,
}
impl From<IsetCcu0> for bool {
    #[inline(always)]
    fn from(variant: IsetCcu0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_CCU0` writer - Capture or compare up event SET"]
pub type IsetCcu0W<'a, REG> = crate::BitWriter<'a, REG, IsetCcu0>;
impl<'a, REG> IsetCcu0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_ccu0_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetCcu0::IsetCcu0NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_ccu0_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetCcu0::IsetCcu0Set)
    }
}
#[doc = "Capture or compare up event SET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetCcu1 {
    #[doc = "0: NO_EFFECT"]
    IsetCcu1NoEffect = 0,
    #[doc = "1: SET"]
    IsetCcu1Set = 1,
}
impl From<IsetCcu1> for bool {
    #[inline(always)]
    fn from(variant: IsetCcu1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_CCU1` writer - Capture or compare up event SET"]
pub type IsetCcu1W<'a, REG> = crate::BitWriter<'a, REG, IsetCcu1>;
impl<'a, REG> IsetCcu1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_ccu1_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetCcu1::IsetCcu1NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_ccu1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetCcu1::IsetCcu1Set)
    }
}
#[doc = "Trigger Overflow event SET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetTov {
    #[doc = "0: NO_EFFECT"]
    IsetTovNoEffect = 0,
    #[doc = "1: SET"]
    IsetTovSet = 1,
}
impl From<IsetTov> for bool {
    #[inline(always)]
    fn from(variant: IsetTov) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_TOV` writer - Trigger Overflow event SET"]
pub type IsetTovW<'a, REG> = crate::BitWriter<'a, REG, IsetTov>;
impl<'a, REG> IsetTovW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_tov_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetTov::IsetTovNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_tov_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetTov::IsetTovSet)
    }
}
#[doc = "Direction Change event SET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetDc {
    #[doc = "0: NO_EFFECT"]
    IsetDcNoEffect = 0,
    #[doc = "1: SET"]
    IsetDcSet = 1,
}
impl From<IsetDc> for bool {
    #[inline(always)]
    fn from(variant: IsetDc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_DC` writer - Direction Change event SET"]
pub type IsetDcW<'a, REG> = crate::BitWriter<'a, REG, IsetDc>;
impl<'a, REG> IsetDcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_dc_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDc::IsetDcNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_dc_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetDc::IsetDcSet)
    }
}
#[doc = "QEIERR event SET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetQeierr {
    #[doc = "0: NO_EFFECT"]
    IsetQeierrNoEffect = 0,
    #[doc = "1: SET"]
    IsetQeierrSet = 1,
}
impl From<IsetQeierr> for bool {
    #[inline(always)]
    fn from(variant: IsetQeierr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_QEIERR` writer - QEIERR event SET"]
pub type IsetQeierrW<'a, REG> = crate::BitWriter<'a, REG, IsetQeierr>;
impl<'a, REG> IsetQeierrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_qeierr_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetQeierr::IsetQeierrNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_qeierr_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetQeierr::IsetQeierrSet)
    }
}
impl W {
    #[doc = "Bit 0 - Zero event SET"]
    #[inline(always)]
    pub fn iset_z(&mut self) -> IsetZW<IsetSpec> {
        IsetZW::new(self, 0)
    }
    #[doc = "Bit 1 - Load event SET"]
    #[inline(always)]
    pub fn iset_l(&mut self) -> IsetLW<IsetSpec> {
        IsetLW::new(self, 1)
    }
    #[doc = "Bit 4 - Capture or compare down event SET"]
    #[inline(always)]
    pub fn iset_ccd0(&mut self) -> IsetCcd0W<IsetSpec> {
        IsetCcd0W::new(self, 4)
    }
    #[doc = "Bit 5 - Capture or compare down event SET"]
    #[inline(always)]
    pub fn iset_ccd1(&mut self) -> IsetCcd1W<IsetSpec> {
        IsetCcd1W::new(self, 5)
    }
    #[doc = "Bit 8 - Capture or compare up event SET"]
    #[inline(always)]
    pub fn iset_ccu0(&mut self) -> IsetCcu0W<IsetSpec> {
        IsetCcu0W::new(self, 8)
    }
    #[doc = "Bit 9 - Capture or compare up event SET"]
    #[inline(always)]
    pub fn iset_ccu1(&mut self) -> IsetCcu1W<IsetSpec> {
        IsetCcu1W::new(self, 9)
    }
    #[doc = "Bit 25 - Trigger Overflow event SET"]
    #[inline(always)]
    pub fn iset_tov(&mut self) -> IsetTovW<IsetSpec> {
        IsetTovW::new(self, 25)
    }
    #[doc = "Bit 27 - Direction Change event SET"]
    #[inline(always)]
    pub fn iset_dc(&mut self) -> IsetDcW<IsetSpec> {
        IsetDcW::new(self, 27)
    }
    #[doc = "Bit 28 - QEIERR event SET"]
    #[inline(always)]
    pub fn iset_qeierr(&mut self) -> IsetQeierrW<IsetSpec> {
        IsetQeierrW::new(self, 28)
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
