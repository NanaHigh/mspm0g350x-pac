#[doc = "Register `ICLR` writer"]
pub type W = crate::W<IclrSpec>;
#[doc = "Clear the LFOSCGOOD interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrLfoscgood {
    #[doc = "0: NO_EFFECT"]
    IclrLfoscgoodNoEffect = 0,
    #[doc = "1: CLR"]
    IclrLfoscgoodClr = 1,
}
impl From<IclrLfoscgood> for bool {
    #[inline(always)]
    fn from(variant: IclrLfoscgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_LFOSCGOOD` writer - Clear the LFOSCGOOD interrupt."]
pub type IclrLfoscgoodW<'a, REG> = crate::BitWriter<'a, REG, IclrLfoscgood>;
impl<'a, REG> IclrLfoscgoodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_lfoscgood_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrLfoscgood::IclrLfoscgoodNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_lfoscgood_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrLfoscgood::IclrLfoscgoodClr)
    }
}
#[doc = "Analog Clocking Consistency Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrAnaclkerr {
    #[doc = "0: NO_EFFECT"]
    IclrAnaclkerrNoEffect = 0,
    #[doc = "1: CLR"]
    IclrAnaclkerrClr = 1,
}
impl From<IclrAnaclkerr> for bool {
    #[inline(always)]
    fn from(variant: IclrAnaclkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_ANACLKERR` writer - Analog Clocking Consistency Error"]
pub type IclrAnaclkerrW<'a, REG> = crate::BitWriter<'a, REG, IclrAnaclkerr>;
impl<'a, REG> IclrAnaclkerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_anaclkerr_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrAnaclkerr::IclrAnaclkerrNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_anaclkerr_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrAnaclkerr::IclrAnaclkerrClr)
    }
}
#[doc = "Flash Single Error Correct\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrFlashsec {
    #[doc = "0: NO_EFFECT"]
    IclrFlashsecNoEffect = 0,
    #[doc = "1: CLR"]
    IclrFlashsecClr = 1,
}
impl From<IclrFlashsec> for bool {
    #[inline(always)]
    fn from(variant: IclrFlashsec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_FLASHSEC` writer - Flash Single Error Correct"]
pub type IclrFlashsecW<'a, REG> = crate::BitWriter<'a, REG, IclrFlashsec>;
impl<'a, REG> IclrFlashsecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_flashsec_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrFlashsec::IclrFlashsecNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_flashsec_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrFlashsec::IclrFlashsecClr)
    }
}
#[doc = "SRAM Single Error Correct\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrSramsec {
    #[doc = "0: NO_EFFECT"]
    IclrSramsecNoEffect = 0,
    #[doc = "1: CLR"]
    IclrSramsecClr = 1,
}
impl From<IclrSramsec> for bool {
    #[inline(always)]
    fn from(variant: IclrSramsec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_SRAMSEC` writer - SRAM Single Error Correct"]
pub type IclrSramsecW<'a, REG> = crate::BitWriter<'a, REG, IclrSramsec>;
impl<'a, REG> IclrSramsecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_sramsec_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrSramsec::IclrSramsecNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_sramsec_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrSramsec::IclrSramsecClr)
    }
}
#[doc = "LFXT GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrLfxtgood {
    #[doc = "0: NO_EFFECT"]
    IclrLfxtgoodNoEffect = 0,
    #[doc = "1: CLR"]
    IclrLfxtgoodClr = 1,
}
impl From<IclrLfxtgood> for bool {
    #[inline(always)]
    fn from(variant: IclrLfxtgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_LFXTGOOD` writer - LFXT GOOD"]
pub type IclrLfxtgoodW<'a, REG> = crate::BitWriter<'a, REG, IclrLfxtgood>;
impl<'a, REG> IclrLfxtgoodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_lfxtgood_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrLfxtgood::IclrLfxtgoodNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_lfxtgood_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrLfxtgood::IclrLfxtgoodClr)
    }
}
#[doc = "HFCLK GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrHfclkgood {
    #[doc = "0: NO_EFFECT"]
    IclrHfclkgoodNoEffect = 0,
    #[doc = "1: CLR"]
    IclrHfclkgoodClr = 1,
}
impl From<IclrHfclkgood> for bool {
    #[inline(always)]
    fn from(variant: IclrHfclkgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_HFCLKGOOD` writer - HFCLK GOOD"]
pub type IclrHfclkgoodW<'a, REG> = crate::BitWriter<'a, REG, IclrHfclkgood>;
impl<'a, REG> IclrHfclkgoodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_hfclkgood_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrHfclkgood::IclrHfclkgoodNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_hfclkgood_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrHfclkgood::IclrHfclkgoodClr)
    }
}
#[doc = "SYSPLL GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrSyspllgood {
    #[doc = "0: NO_EFFECT"]
    IclrSyspllgoodNoEffect = 0,
    #[doc = "1: CLR"]
    IclrSyspllgoodClr = 1,
}
impl From<IclrSyspllgood> for bool {
    #[inline(always)]
    fn from(variant: IclrSyspllgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_SYSPLLGOOD` writer - SYSPLL GOOD"]
pub type IclrSyspllgoodW<'a, REG> = crate::BitWriter<'a, REG, IclrSyspllgood>;
impl<'a, REG> IclrSyspllgoodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_syspllgood_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrSyspllgood::IclrSyspllgoodNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_syspllgood_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrSyspllgood::IclrSyspllgoodClr)
    }
}
#[doc = "HSCLK GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IclrHsclkgood {
    #[doc = "0: NO_EFFECT"]
    IclrHsclkgoodNoEffect = 0,
    #[doc = "1: CLR"]
    IclrHsclkgoodClr = 1,
}
impl From<IclrHsclkgood> for bool {
    #[inline(always)]
    fn from(variant: IclrHsclkgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICLR_HSCLKGOOD` writer - HSCLK GOOD"]
pub type IclrHsclkgoodW<'a, REG> = crate::BitWriter<'a, REG, IclrHsclkgood>;
impl<'a, REG> IclrHsclkgoodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iclr_hsclkgood_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IclrHsclkgood::IclrHsclkgoodNoEffect)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn iclr_hsclkgood_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IclrHsclkgood::IclrHsclkgoodClr)
    }
}
impl W {
    #[doc = "Bit 0 - Clear the LFOSCGOOD interrupt."]
    #[inline(always)]
    pub fn iclr_lfoscgood(&mut self) -> IclrLfoscgoodW<IclrSpec> {
        IclrLfoscgoodW::new(self, 0)
    }
    #[doc = "Bit 1 - Analog Clocking Consistency Error"]
    #[inline(always)]
    pub fn iclr_anaclkerr(&mut self) -> IclrAnaclkerrW<IclrSpec> {
        IclrAnaclkerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Flash Single Error Correct"]
    #[inline(always)]
    pub fn iclr_flashsec(&mut self) -> IclrFlashsecW<IclrSpec> {
        IclrFlashsecW::new(self, 2)
    }
    #[doc = "Bit 3 - SRAM Single Error Correct"]
    #[inline(always)]
    pub fn iclr_sramsec(&mut self) -> IclrSramsecW<IclrSpec> {
        IclrSramsecW::new(self, 3)
    }
    #[doc = "Bit 4 - LFXT GOOD"]
    #[inline(always)]
    pub fn iclr_lfxtgood(&mut self) -> IclrLfxtgoodW<IclrSpec> {
        IclrLfxtgoodW::new(self, 4)
    }
    #[doc = "Bit 5 - HFCLK GOOD"]
    #[inline(always)]
    pub fn iclr_hfclkgood(&mut self) -> IclrHfclkgoodW<IclrSpec> {
        IclrHfclkgoodW::new(self, 5)
    }
    #[doc = "Bit 6 - SYSPLL GOOD"]
    #[inline(always)]
    pub fn iclr_syspllgood(&mut self) -> IclrSyspllgoodW<IclrSpec> {
        IclrSyspllgoodW::new(self, 6)
    }
    #[doc = "Bit 7 - HSCLK GOOD"]
    #[inline(always)]
    pub fn iclr_hsclkgood(&mut self) -> IclrHsclkgoodW<IclrSpec> {
        IclrHsclkgoodW::new(self, 7)
    }
}
#[doc = "SYSCTL interrupt clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
