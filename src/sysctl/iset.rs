#[doc = "Register `ISET` writer"]
pub type W = crate::W<IsetSpec>;
#[doc = "Set the LFOSCGOOD interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetLfoscgood {
    #[doc = "0: NO_EFFECT"]
    IsetLfoscgoodNoEffect = 0,
    #[doc = "1: SET"]
    IsetLfoscgoodSet = 1,
}
impl From<IsetLfoscgood> for bool {
    #[inline(always)]
    fn from(variant: IsetLfoscgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_LFOSCGOOD` writer - Set the LFOSCGOOD interrupt."]
pub type IsetLfoscgoodW<'a, REG> = crate::BitWriter<'a, REG, IsetLfoscgood>;
impl<'a, REG> IsetLfoscgoodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_lfoscgood_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetLfoscgood::IsetLfoscgoodNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_lfoscgood_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetLfoscgood::IsetLfoscgoodSet)
    }
}
#[doc = "Analog Clocking Consistency Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetAnaclkerr {
    #[doc = "0: NO_EFFECT"]
    IsetAnaclkerrNoEffect = 0,
    #[doc = "1: SET"]
    IsetAnaclkerrSet = 1,
}
impl From<IsetAnaclkerr> for bool {
    #[inline(always)]
    fn from(variant: IsetAnaclkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_ANACLKERR` writer - Analog Clocking Consistency Error"]
pub type IsetAnaclkerrW<'a, REG> = crate::BitWriter<'a, REG, IsetAnaclkerr>;
impl<'a, REG> IsetAnaclkerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_anaclkerr_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetAnaclkerr::IsetAnaclkerrNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_anaclkerr_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetAnaclkerr::IsetAnaclkerrSet)
    }
}
#[doc = "Flash Single Error Correct\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetFlashsec {
    #[doc = "0: NO_EFFECT"]
    IsetFlashsecNoEffect = 0,
    #[doc = "1: SET"]
    IsetFlashsecSet = 1,
}
impl From<IsetFlashsec> for bool {
    #[inline(always)]
    fn from(variant: IsetFlashsec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_FLASHSEC` writer - Flash Single Error Correct"]
pub type IsetFlashsecW<'a, REG> = crate::BitWriter<'a, REG, IsetFlashsec>;
impl<'a, REG> IsetFlashsecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_flashsec_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetFlashsec::IsetFlashsecNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_flashsec_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetFlashsec::IsetFlashsecSet)
    }
}
#[doc = "SRAM Single Error Correct\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetSramsec {
    #[doc = "0: NO_EFFECT"]
    IsetSramsecNoEffect = 0,
    #[doc = "1: SET"]
    IsetSramsecSet = 1,
}
impl From<IsetSramsec> for bool {
    #[inline(always)]
    fn from(variant: IsetSramsec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_SRAMSEC` writer - SRAM Single Error Correct"]
pub type IsetSramsecW<'a, REG> = crate::BitWriter<'a, REG, IsetSramsec>;
impl<'a, REG> IsetSramsecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_sramsec_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetSramsec::IsetSramsecNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_sramsec_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetSramsec::IsetSramsecSet)
    }
}
#[doc = "LFXT GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetLfxtgood {
    #[doc = "0: NO_EFFECT"]
    IsetLfxtgoodNoEffect = 0,
    #[doc = "1: SET"]
    IsetLfxtgoodSet = 1,
}
impl From<IsetLfxtgood> for bool {
    #[inline(always)]
    fn from(variant: IsetLfxtgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_LFXTGOOD` writer - LFXT GOOD"]
pub type IsetLfxtgoodW<'a, REG> = crate::BitWriter<'a, REG, IsetLfxtgood>;
impl<'a, REG> IsetLfxtgoodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_lfxtgood_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetLfxtgood::IsetLfxtgoodNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_lfxtgood_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetLfxtgood::IsetLfxtgoodSet)
    }
}
#[doc = "HFCLK GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetHfclkgood {
    #[doc = "0: NO_EFFECT"]
    IsetHfclkgoodNoEffect = 0,
    #[doc = "1: SET"]
    IsetHfclkgoodSet = 1,
}
impl From<IsetHfclkgood> for bool {
    #[inline(always)]
    fn from(variant: IsetHfclkgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_HFCLKGOOD` writer - HFCLK GOOD"]
pub type IsetHfclkgoodW<'a, REG> = crate::BitWriter<'a, REG, IsetHfclkgood>;
impl<'a, REG> IsetHfclkgoodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_hfclkgood_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetHfclkgood::IsetHfclkgoodNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_hfclkgood_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetHfclkgood::IsetHfclkgoodSet)
    }
}
#[doc = "SYSPLL GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetSyspllgood {
    #[doc = "0: NO_EFFECT"]
    IsetSyspllgoodNoEffect = 0,
    #[doc = "1: SET"]
    IsetSyspllgoodSet = 1,
}
impl From<IsetSyspllgood> for bool {
    #[inline(always)]
    fn from(variant: IsetSyspllgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_SYSPLLGOOD` writer - SYSPLL GOOD"]
pub type IsetSyspllgoodW<'a, REG> = crate::BitWriter<'a, REG, IsetSyspllgood>;
impl<'a, REG> IsetSyspllgoodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_syspllgood_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetSyspllgood::IsetSyspllgoodNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_syspllgood_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetSyspllgood::IsetSyspllgoodSet)
    }
}
#[doc = "HSCLK GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsetHsclkgood {
    #[doc = "0: NO_EFFECT"]
    IsetHsclkgoodNoEffect = 0,
    #[doc = "1: SET"]
    IsetHsclkgoodSet = 1,
}
impl From<IsetHsclkgood> for bool {
    #[inline(always)]
    fn from(variant: IsetHsclkgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ISET_HSCLKGOOD` writer - HSCLK GOOD"]
pub type IsetHsclkgoodW<'a, REG> = crate::BitWriter<'a, REG, IsetHsclkgood>;
impl<'a, REG> IsetHsclkgoodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "NO_EFFECT"]
    #[inline(always)]
    pub fn iset_hsclkgood_no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(IsetHsclkgood::IsetHsclkgoodNoEffect)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn iset_hsclkgood_set(self) -> &'a mut crate::W<REG> {
        self.variant(IsetHsclkgood::IsetHsclkgoodSet)
    }
}
impl W {
    #[doc = "Bit 0 - Set the LFOSCGOOD interrupt."]
    #[inline(always)]
    pub fn iset_lfoscgood(&mut self) -> IsetLfoscgoodW<IsetSpec> {
        IsetLfoscgoodW::new(self, 0)
    }
    #[doc = "Bit 1 - Analog Clocking Consistency Error"]
    #[inline(always)]
    pub fn iset_anaclkerr(&mut self) -> IsetAnaclkerrW<IsetSpec> {
        IsetAnaclkerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Flash Single Error Correct"]
    #[inline(always)]
    pub fn iset_flashsec(&mut self) -> IsetFlashsecW<IsetSpec> {
        IsetFlashsecW::new(self, 2)
    }
    #[doc = "Bit 3 - SRAM Single Error Correct"]
    #[inline(always)]
    pub fn iset_sramsec(&mut self) -> IsetSramsecW<IsetSpec> {
        IsetSramsecW::new(self, 3)
    }
    #[doc = "Bit 4 - LFXT GOOD"]
    #[inline(always)]
    pub fn iset_lfxtgood(&mut self) -> IsetLfxtgoodW<IsetSpec> {
        IsetLfxtgoodW::new(self, 4)
    }
    #[doc = "Bit 5 - HFCLK GOOD"]
    #[inline(always)]
    pub fn iset_hfclkgood(&mut self) -> IsetHfclkgoodW<IsetSpec> {
        IsetHfclkgoodW::new(self, 5)
    }
    #[doc = "Bit 6 - SYSPLL GOOD"]
    #[inline(always)]
    pub fn iset_syspllgood(&mut self) -> IsetSyspllgoodW<IsetSpec> {
        IsetSyspllgoodW::new(self, 6)
    }
    #[doc = "Bit 7 - HSCLK GOOD"]
    #[inline(always)]
    pub fn iset_hsclkgood(&mut self) -> IsetHsclkgoodW<IsetSpec> {
        IsetHsclkgoodW::new(self, 7)
    }
}
#[doc = "SYSCTL interrupt set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
