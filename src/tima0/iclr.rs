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
#[doc = "Capture or compare down event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrCcd2 {
    #[doc = "0: NO_EFFECT"]
    IclrCcd2NoEffect = 0,
    #[doc = "1: CLR"]
    IclrCcd2Clr = 1,
}
impl From<IclrCcd2> for bool {
    #[inline(always)]
    fn from(variant: IclrCcd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_CCD2` writer - Capture or compare down event CLEAR"]
pub type IclrCcd2W<'a, REG> = crate::BitWriter<'a, REG, IclrCcd2>;
impl<'a, REG> IclrCcd2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_ccd2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcd2::IclrCcd2NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_ccd2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcd2::IclrCcd2Clr)
    }
}
#[doc = "Capture or compare down event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrCcd3 {
    #[doc = "0: NO_EFFECT"]
    IclrCcd3NoEffect = 0,
    #[doc = "1: CLR"]
    IclrCcd3Clr = 1,
}
impl From<IclrCcd3> for bool {
    #[inline(always)]
    fn from(variant: IclrCcd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_CCD3` writer - Capture or compare down event CLEAR"]
pub type IclrCcd3W<'a, REG> = crate::BitWriter<'a, REG, IclrCcd3>;
impl<'a, REG> IclrCcd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_ccd3_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcd3::IclrCcd3NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_ccd3_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcd3::IclrCcd3Clr)
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
#[doc = "Capture or compare up event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrCcu2 {
    #[doc = "0: NO_EFFECT"]
    IclrCcu2NoEffect = 0,
    #[doc = "1: CLR"]
    IclrCcu2Clr = 1,
}
impl From<IclrCcu2> for bool {
    #[inline(always)]
    fn from(variant: IclrCcu2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_CCU2` writer - Capture or compare up event CLEAR"]
pub type IclrCcu2W<'a, REG> = crate::BitWriter<'a, REG, IclrCcu2>;
impl<'a, REG> IclrCcu2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_ccu2_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcu2::IclrCcu2NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_ccu2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcu2::IclrCcu2Clr)
    }
}
#[doc = "Capture or compare up event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrCcu3 {
    #[doc = "0: NO_EFFECT"]
    IclrCcu3NoEffect = 0,
    #[doc = "1: CLR"]
    IclrCcu3Clr = 1,
}
impl From<IclrCcu3> for bool {
    #[inline(always)]
    fn from(variant: IclrCcu3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_CCU3` writer - Capture or compare up event CLEAR"]
pub type IclrCcu3W<'a, REG> = crate::BitWriter<'a, REG, IclrCcu3>;
impl<'a, REG> IclrCcu3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_ccu3_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcu3::IclrCcu3NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_ccu3_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcu3::IclrCcu3Clr)
    }
}
#[doc = "Compare down event 4 CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrCcd4 {
    #[doc = "0: NO_EFFECT"]
    IclrCcd4NoEffect = 0,
    #[doc = "1: CLR"]
    IclrCcd4Clr = 1,
}
impl From<IclrCcd4> for bool {
    #[inline(always)]
    fn from(variant: IclrCcd4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_CCD4` writer - Compare down event 4 CLEAR"]
pub type IclrCcd4W<'a, REG> = crate::BitWriter<'a, REG, IclrCcd4>;
impl<'a, REG> IclrCcd4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_ccd4_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcd4::IclrCcd4NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_ccd4_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcd4::IclrCcd4Clr)
    }
}
#[doc = "Compare down event 5 CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrCcd5 {
    #[doc = "0: NO_EFFECT"]
    IclrCcd5NoEffect = 0,
    #[doc = "1: CLR"]
    IclrCcd5Clr = 1,
}
impl From<IclrCcd5> for bool {
    #[inline(always)]
    fn from(variant: IclrCcd5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_CCD5` writer - Compare down event 5 CLEAR"]
pub type IclrCcd5W<'a, REG> = crate::BitWriter<'a, REG, IclrCcd5>;
impl<'a, REG> IclrCcd5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_ccd5_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcd5::IclrCcd5NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_ccd5_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcd5::IclrCcd5Clr)
    }
}
#[doc = "Compare up event 4 CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrCcu4 {
    #[doc = "0: NO_EFFECT"]
    IclrCcu4NoEffect = 0,
    #[doc = "1: CLR"]
    IclrCcu4Clr = 1,
}
impl From<IclrCcu4> for bool {
    #[inline(always)]
    fn from(variant: IclrCcu4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_CCU4` writer - Compare up event 4 CLEAR"]
pub type IclrCcu4W<'a, REG> = crate::BitWriter<'a, REG, IclrCcu4>;
impl<'a, REG> IclrCcu4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_ccu4_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcu4::IclrCcu4NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_ccu4_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcu4::IclrCcu4Clr)
    }
}
#[doc = "Compare up event 5 CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrCcu5 {
    #[doc = "0: NO_EFFECT"]
    IclrCcu5NoEffect = 0,
    #[doc = "1: CLR"]
    IclrCcu5Clr = 1,
}
impl From<IclrCcu5> for bool {
    #[inline(always)]
    fn from(variant: IclrCcu5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_CCU5` writer - Compare up event 5 CLEAR"]
pub type IclrCcu5W<'a, REG> = crate::BitWriter<'a, REG, IclrCcu5>;
impl<'a, REG> IclrCcu5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_ccu5_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcu5::IclrCcu5NoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_ccu5_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrCcu5::IclrCcu5Clr)
    }
}
#[doc = "Fault event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrF {
    #[doc = "0: NO_EFFECT"]
    IclrFNoEffect = 0,
    #[doc = "1: CLR"]
    IclrFClr = 1,
}
impl From<IclrF> for bool {
    #[inline(always)]
    fn from(variant: IclrF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_F` writer - Fault event CLEAR"]
pub type IclrFW<'a, REG> = crate::BitWriter<'a, REG, IclrF>;
impl<'a, REG> IclrFW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_f_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrF::IclrFNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_f_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrF::IclrFClr)
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
#[doc = "Repeat Counter Zero event CLEAR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrRepc {
    #[doc = "0: NO_EFFECT"]
    IclrRepcNoEffect = 0,
    #[doc = "1: CLR"]
    IclrRepcClr = 1,
}
impl From<IclrRepc> for bool {
    #[inline(always)]
    fn from(variant: IclrRepc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_REPC` writer - Repeat Counter Zero event CLEAR"]
pub type IclrRepcW<'a, REG> = crate::BitWriter<'a, REG, IclrRepc>;
impl<'a, REG> IclrRepcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_repc_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrRepc::IclrRepcNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_repc_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrRepc::IclrRepcClr)
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
    #[doc = "Bit 6 - Capture or compare down event CLEAR"]
    #[inline(always)]
    pub fn iclr_ccd2(&mut self) -> IclrCcd2W<IclrSpec> {
        IclrCcd2W::new(self, 6)
    }
    #[doc = "Bit 7 - Capture or compare down event CLEAR"]
    #[inline(always)]
    pub fn iclr_ccd3(&mut self) -> IclrCcd3W<IclrSpec> {
        IclrCcd3W::new(self, 7)
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
    #[doc = "Bit 10 - Capture or compare up event CLEAR"]
    #[inline(always)]
    pub fn iclr_ccu2(&mut self) -> IclrCcu2W<IclrSpec> {
        IclrCcu2W::new(self, 10)
    }
    #[doc = "Bit 11 - Capture or compare up event CLEAR"]
    #[inline(always)]
    pub fn iclr_ccu3(&mut self) -> IclrCcu3W<IclrSpec> {
        IclrCcu3W::new(self, 11)
    }
    #[doc = "Bit 12 - Compare down event 4 CLEAR"]
    #[inline(always)]
    pub fn iclr_ccd4(&mut self) -> IclrCcd4W<IclrSpec> {
        IclrCcd4W::new(self, 12)
    }
    #[doc = "Bit 13 - Compare down event 5 CLEAR"]
    #[inline(always)]
    pub fn iclr_ccd5(&mut self) -> IclrCcd5W<IclrSpec> {
        IclrCcd5W::new(self, 13)
    }
    #[doc = "Bit 14 - Compare up event 4 CLEAR"]
    #[inline(always)]
    pub fn iclr_ccu4(&mut self) -> IclrCcu4W<IclrSpec> {
        IclrCcu4W::new(self, 14)
    }
    #[doc = "Bit 15 - Compare up event 5 CLEAR"]
    #[inline(always)]
    pub fn iclr_ccu5(&mut self) -> IclrCcu5W<IclrSpec> {
        IclrCcu5W::new(self, 15)
    }
    #[doc = "Bit 24 - Fault event CLEAR"]
    #[inline(always)]
    pub fn iclr_f(&mut self) -> IclrFW<IclrSpec> {
        IclrFW::new(self, 24)
    }
    #[doc = "Bit 25 - Trigger Overflow event CLEAR"]
    #[inline(always)]
    pub fn iclr_tov(&mut self) -> IclrTovW<IclrSpec> {
        IclrTovW::new(self, 25)
    }
    #[doc = "Bit 26 - Repeat Counter Zero event CLEAR"]
    #[inline(always)]
    pub fn iclr_repc(&mut self) -> IclrRepcW<IclrSpec> {
        IclrRepcW::new(self, 26)
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
