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
#[doc = "Compare down event 4 SET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetCcd4 {
    #[doc = "0: NO_EFFECT"]
    IsetCcd4NoEffect = 0,
    #[doc = "1: SET"]
    IsetCcd4Set = 1,
}
impl From<IsetCcd4> for bool {
    #[inline(always)]
    fn from(variant: IsetCcd4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_CCD4` writer - Compare down event 4 SET"]
pub type IsetCcd4W<'a, REG> = crate::BitWriter<'a, REG, IsetCcd4>;
impl<'a, REG> IsetCcd4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_ccd4_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetCcd4::IsetCcd4NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_ccd4_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetCcd4::IsetCcd4Set)
    }
}
#[doc = "Compare down event 5 SET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetCcd5 {
    #[doc = "0: NO_EFFECT"]
    IsetCcd5NoEffect = 0,
    #[doc = "1: SET"]
    IsetCcd5Set = 1,
}
impl From<IsetCcd5> for bool {
    #[inline(always)]
    fn from(variant: IsetCcd5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_CCD5` writer - Compare down event 5 SET"]
pub type IsetCcd5W<'a, REG> = crate::BitWriter<'a, REG, IsetCcd5>;
impl<'a, REG> IsetCcd5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_ccd5_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetCcd5::IsetCcd5NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_ccd5_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetCcd5::IsetCcd5Set)
    }
}
#[doc = "Compare up event 4 SET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetCcu4 {
    #[doc = "0: NO_EFFECT"]
    IsetCcu4NoEffect = 0,
    #[doc = "1: SET"]
    IsetCcu4Set = 1,
}
impl From<IsetCcu4> for bool {
    #[inline(always)]
    fn from(variant: IsetCcu4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_CCU4` writer - Compare up event 4 SET"]
pub type IsetCcu4W<'a, REG> = crate::BitWriter<'a, REG, IsetCcu4>;
impl<'a, REG> IsetCcu4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_ccu4_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetCcu4::IsetCcu4NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_ccu4_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetCcu4::IsetCcu4Set)
    }
}
#[doc = "Compare up event 5 SET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetCcu5 {
    #[doc = "0: NO_EFFECT"]
    IsetCcu5NoEffect = 0,
    #[doc = "1: SET"]
    IsetCcu5Set = 1,
}
impl From<IsetCcu5> for bool {
    #[inline(always)]
    fn from(variant: IsetCcu5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_CCU5` writer - Compare up event 5 SET"]
pub type IsetCcu5W<'a, REG> = crate::BitWriter<'a, REG, IsetCcu5>;
impl<'a, REG> IsetCcu5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_ccu5_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetCcu5::IsetCcu5NoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_ccu5_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetCcu5::IsetCcu5Set)
    }
}
#[doc = "Fault event SET\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetF {
    #[doc = "0: NO_EFFECT"]
    IsetFNoEffect = 0,
    #[doc = "1: SET"]
    IsetFSet = 1,
}
impl From<IsetF> for bool {
    #[inline(always)]
    fn from(variant: IsetF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_F` writer - Fault event SET"]
pub type IsetFW<'a, REG> = crate::BitWriter<'a, REG, IsetF>;
impl<'a, REG> IsetFW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_f_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetF::IsetFNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_f_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetF::IsetFSet)
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
    #[doc = "Bit 12 - Compare down event 4 SET"]
    #[inline(always)]
    pub fn iset_ccd4(&mut self) -> IsetCcd4W<IsetSpec> {
        IsetCcd4W::new(self, 12)
    }
    #[doc = "Bit 13 - Compare down event 5 SET"]
    #[inline(always)]
    pub fn iset_ccd5(&mut self) -> IsetCcd5W<IsetSpec> {
        IsetCcd5W::new(self, 13)
    }
    #[doc = "Bit 14 - Compare up event 4 SET"]
    #[inline(always)]
    pub fn iset_ccu4(&mut self) -> IsetCcu4W<IsetSpec> {
        IsetCcu4W::new(self, 14)
    }
    #[doc = "Bit 15 - Compare up event 5 SET"]
    #[inline(always)]
    pub fn iset_ccu5(&mut self) -> IsetCcu5W<IsetSpec> {
        IsetCcu5W::new(self, 15)
    }
    #[doc = "Bit 24 - Fault event SET"]
    #[inline(always)]
    pub fn iset_f(&mut self) -> IsetFW<IsetSpec> {
        IsetFW::new(self, 24)
    }
    #[doc = "Bit 25 - Trigger Overflow event SET"]
    #[inline(always)]
    pub fn iset_tov(&mut self) -> IsetTovW<IsetSpec> {
        IsetTovW::new(self, 25)
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
