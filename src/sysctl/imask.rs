#[doc = "Register `IMASK` reader"]
pub type R = crate::R<ImaskSpec>;
#[doc = "Register `IMASK` writer"]
pub type W = crate::W<ImaskSpec>;
#[doc = "Enable or disable the LFOSCGOOD interrupt. LFOSCGOOD indicates that the LFOSC has started successfully.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskLfoscgood {
    #[doc = "0: DISABLE"]
    ImaskLfoscgoodDisable = 0,
    #[doc = "1: ENABLE"]
    ImaskLfoscgoodEnable = 1,
}
impl From<ImaskLfoscgood> for bool {
    #[inline(always)]
    fn from(variant: ImaskLfoscgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_LFOSCGOOD` reader - Enable or disable the LFOSCGOOD interrupt. LFOSCGOOD indicates that the LFOSC has started successfully."]
pub type ImaskLfoscgoodR = crate::BitReader<ImaskLfoscgood>;
impl ImaskLfoscgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskLfoscgood {
        match self.bits {
            false => ImaskLfoscgood::ImaskLfoscgoodDisable,
            true => ImaskLfoscgood::ImaskLfoscgoodEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_imask_lfoscgood_disable(&self) -> bool {
        *self == ImaskLfoscgood::ImaskLfoscgoodDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_imask_lfoscgood_enable(&self) -> bool {
        *self == ImaskLfoscgood::ImaskLfoscgoodEnable
    }
}
#[doc = "Field `IMASK_LFOSCGOOD` writer - Enable or disable the LFOSCGOOD interrupt. LFOSCGOOD indicates that the LFOSC has started successfully."]
pub type ImaskLfoscgoodW<'a, REG> = crate::BitWriter<'a, REG, ImaskLfoscgood>;
impl<'a, REG> ImaskLfoscgoodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn imask_lfoscgood_disable(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskLfoscgood::ImaskLfoscgoodDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn imask_lfoscgood_enable(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskLfoscgood::ImaskLfoscgoodEnable)
    }
}
#[doc = "Analog Clocking Consistency Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskAnaclkerr {
    #[doc = "0: DISABLE"]
    ImaskAnaclkerrDisable = 0,
    #[doc = "1: ENABLE"]
    ImaskAnaclkerrEnable = 1,
}
impl From<ImaskAnaclkerr> for bool {
    #[inline(always)]
    fn from(variant: ImaskAnaclkerr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_ANACLKERR` reader - Analog Clocking Consistency Error"]
pub type ImaskAnaclkerrR = crate::BitReader<ImaskAnaclkerr>;
impl ImaskAnaclkerrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskAnaclkerr {
        match self.bits {
            false => ImaskAnaclkerr::ImaskAnaclkerrDisable,
            true => ImaskAnaclkerr::ImaskAnaclkerrEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_imask_anaclkerr_disable(&self) -> bool {
        *self == ImaskAnaclkerr::ImaskAnaclkerrDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_imask_anaclkerr_enable(&self) -> bool {
        *self == ImaskAnaclkerr::ImaskAnaclkerrEnable
    }
}
#[doc = "Field `IMASK_ANACLKERR` writer - Analog Clocking Consistency Error"]
pub type ImaskAnaclkerrW<'a, REG> = crate::BitWriter<'a, REG, ImaskAnaclkerr>;
impl<'a, REG> ImaskAnaclkerrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn imask_anaclkerr_disable(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskAnaclkerr::ImaskAnaclkerrDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn imask_anaclkerr_enable(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskAnaclkerr::ImaskAnaclkerrEnable)
    }
}
#[doc = "Flash Single Error Correct\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskFlashsec {
    #[doc = "0: DISABLE"]
    ImaskFlashsecDisable = 0,
    #[doc = "1: ENABLE"]
    ImaskFlashsecEnable = 1,
}
impl From<ImaskFlashsec> for bool {
    #[inline(always)]
    fn from(variant: ImaskFlashsec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_FLASHSEC` reader - Flash Single Error Correct"]
pub type ImaskFlashsecR = crate::BitReader<ImaskFlashsec>;
impl ImaskFlashsecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskFlashsec {
        match self.bits {
            false => ImaskFlashsec::ImaskFlashsecDisable,
            true => ImaskFlashsec::ImaskFlashsecEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_imask_flashsec_disable(&self) -> bool {
        *self == ImaskFlashsec::ImaskFlashsecDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_imask_flashsec_enable(&self) -> bool {
        *self == ImaskFlashsec::ImaskFlashsecEnable
    }
}
#[doc = "Field `IMASK_FLASHSEC` writer - Flash Single Error Correct"]
pub type ImaskFlashsecW<'a, REG> = crate::BitWriter<'a, REG, ImaskFlashsec>;
impl<'a, REG> ImaskFlashsecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn imask_flashsec_disable(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskFlashsec::ImaskFlashsecDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn imask_flashsec_enable(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskFlashsec::ImaskFlashsecEnable)
    }
}
#[doc = "SRAM Single Error Correct\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskSramsec {
    #[doc = "0: DISABLE"]
    ImaskSramsecDisable = 0,
    #[doc = "1: ENABLE"]
    ImaskSramsecEnable = 1,
}
impl From<ImaskSramsec> for bool {
    #[inline(always)]
    fn from(variant: ImaskSramsec) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_SRAMSEC` reader - SRAM Single Error Correct"]
pub type ImaskSramsecR = crate::BitReader<ImaskSramsec>;
impl ImaskSramsecR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskSramsec {
        match self.bits {
            false => ImaskSramsec::ImaskSramsecDisable,
            true => ImaskSramsec::ImaskSramsecEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_imask_sramsec_disable(&self) -> bool {
        *self == ImaskSramsec::ImaskSramsecDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_imask_sramsec_enable(&self) -> bool {
        *self == ImaskSramsec::ImaskSramsecEnable
    }
}
#[doc = "Field `IMASK_SRAMSEC` writer - SRAM Single Error Correct"]
pub type ImaskSramsecW<'a, REG> = crate::BitWriter<'a, REG, ImaskSramsec>;
impl<'a, REG> ImaskSramsecW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn imask_sramsec_disable(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskSramsec::ImaskSramsecDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn imask_sramsec_enable(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskSramsec::ImaskSramsecEnable)
    }
}
#[doc = "LFXT GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskLfxtgood {
    #[doc = "0: DISABLE"]
    ImaskLfxtgoodDisable = 0,
    #[doc = "1: ENABLE"]
    ImaskLfxtgoodEnable = 1,
}
impl From<ImaskLfxtgood> for bool {
    #[inline(always)]
    fn from(variant: ImaskLfxtgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_LFXTGOOD` reader - LFXT GOOD"]
pub type ImaskLfxtgoodR = crate::BitReader<ImaskLfxtgood>;
impl ImaskLfxtgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskLfxtgood {
        match self.bits {
            false => ImaskLfxtgood::ImaskLfxtgoodDisable,
            true => ImaskLfxtgood::ImaskLfxtgoodEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_imask_lfxtgood_disable(&self) -> bool {
        *self == ImaskLfxtgood::ImaskLfxtgoodDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_imask_lfxtgood_enable(&self) -> bool {
        *self == ImaskLfxtgood::ImaskLfxtgoodEnable
    }
}
#[doc = "Field `IMASK_LFXTGOOD` writer - LFXT GOOD"]
pub type ImaskLfxtgoodW<'a, REG> = crate::BitWriter<'a, REG, ImaskLfxtgood>;
impl<'a, REG> ImaskLfxtgoodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn imask_lfxtgood_disable(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskLfxtgood::ImaskLfxtgoodDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn imask_lfxtgood_enable(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskLfxtgood::ImaskLfxtgoodEnable)
    }
}
#[doc = "HFCLK GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskHfclkgood {
    #[doc = "0: DISABLE"]
    ImaskHfclkgoodDisable = 0,
    #[doc = "1: ENABLE"]
    ImaskHfclkgoodEnable = 1,
}
impl From<ImaskHfclkgood> for bool {
    #[inline(always)]
    fn from(variant: ImaskHfclkgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_HFCLKGOOD` reader - HFCLK GOOD"]
pub type ImaskHfclkgoodR = crate::BitReader<ImaskHfclkgood>;
impl ImaskHfclkgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskHfclkgood {
        match self.bits {
            false => ImaskHfclkgood::ImaskHfclkgoodDisable,
            true => ImaskHfclkgood::ImaskHfclkgoodEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_imask_hfclkgood_disable(&self) -> bool {
        *self == ImaskHfclkgood::ImaskHfclkgoodDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_imask_hfclkgood_enable(&self) -> bool {
        *self == ImaskHfclkgood::ImaskHfclkgoodEnable
    }
}
#[doc = "Field `IMASK_HFCLKGOOD` writer - HFCLK GOOD"]
pub type ImaskHfclkgoodW<'a, REG> = crate::BitWriter<'a, REG, ImaskHfclkgood>;
impl<'a, REG> ImaskHfclkgoodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn imask_hfclkgood_disable(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskHfclkgood::ImaskHfclkgoodDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn imask_hfclkgood_enable(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskHfclkgood::ImaskHfclkgoodEnable)
    }
}
#[doc = "SYSPLL GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskSyspllgood {
    #[doc = "0: DISABLE"]
    ImaskSyspllgoodDisable = 0,
    #[doc = "1: ENABLE"]
    ImaskSyspllgoodEnable = 1,
}
impl From<ImaskSyspllgood> for bool {
    #[inline(always)]
    fn from(variant: ImaskSyspllgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_SYSPLLGOOD` reader - SYSPLL GOOD"]
pub type ImaskSyspllgoodR = crate::BitReader<ImaskSyspllgood>;
impl ImaskSyspllgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskSyspllgood {
        match self.bits {
            false => ImaskSyspllgood::ImaskSyspllgoodDisable,
            true => ImaskSyspllgood::ImaskSyspllgoodEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_imask_syspllgood_disable(&self) -> bool {
        *self == ImaskSyspllgood::ImaskSyspllgoodDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_imask_syspllgood_enable(&self) -> bool {
        *self == ImaskSyspllgood::ImaskSyspllgoodEnable
    }
}
#[doc = "Field `IMASK_SYSPLLGOOD` writer - SYSPLL GOOD"]
pub type ImaskSyspllgoodW<'a, REG> = crate::BitWriter<'a, REG, ImaskSyspllgood>;
impl<'a, REG> ImaskSyspllgoodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn imask_syspllgood_disable(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskSyspllgood::ImaskSyspllgoodDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn imask_syspllgood_enable(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskSyspllgood::ImaskSyspllgoodEnable)
    }
}
#[doc = "HSCLK GOOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImaskHsclkgood {
    #[doc = "0: DISABLE"]
    ImaskHsclkgoodDisable = 0,
    #[doc = "1: ENABLE"]
    ImaskHsclkgoodEnable = 1,
}
impl From<ImaskHsclkgood> for bool {
    #[inline(always)]
    fn from(variant: ImaskHsclkgood) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMASK_HSCLKGOOD` reader - HSCLK GOOD"]
pub type ImaskHsclkgoodR = crate::BitReader<ImaskHsclkgood>;
impl ImaskHsclkgoodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ImaskHsclkgood {
        match self.bits {
            false => ImaskHsclkgood::ImaskHsclkgoodDisable,
            true => ImaskHsclkgood::ImaskHsclkgoodEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_imask_hsclkgood_disable(&self) -> bool {
        *self == ImaskHsclkgood::ImaskHsclkgoodDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_imask_hsclkgood_enable(&self) -> bool {
        *self == ImaskHsclkgood::ImaskHsclkgoodEnable
    }
}
#[doc = "Field `IMASK_HSCLKGOOD` writer - HSCLK GOOD"]
pub type ImaskHsclkgoodW<'a, REG> = crate::BitWriter<'a, REG, ImaskHsclkgood>;
impl<'a, REG> ImaskHsclkgoodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn imask_hsclkgood_disable(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskHsclkgood::ImaskHsclkgoodDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn imask_hsclkgood_enable(self) -> &'a mut crate::W<REG> {
        self.variant(ImaskHsclkgood::ImaskHsclkgoodEnable)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable the LFOSCGOOD interrupt. LFOSCGOOD indicates that the LFOSC has started successfully."]
    #[inline(always)]
    pub fn imask_lfoscgood(&self) -> ImaskLfoscgoodR {
        ImaskLfoscgoodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Clocking Consistency Error"]
    #[inline(always)]
    pub fn imask_anaclkerr(&self) -> ImaskAnaclkerrR {
        ImaskAnaclkerrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flash Single Error Correct"]
    #[inline(always)]
    pub fn imask_flashsec(&self) -> ImaskFlashsecR {
        ImaskFlashsecR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SRAM Single Error Correct"]
    #[inline(always)]
    pub fn imask_sramsec(&self) -> ImaskSramsecR {
        ImaskSramsecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LFXT GOOD"]
    #[inline(always)]
    pub fn imask_lfxtgood(&self) -> ImaskLfxtgoodR {
        ImaskLfxtgoodR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HFCLK GOOD"]
    #[inline(always)]
    pub fn imask_hfclkgood(&self) -> ImaskHfclkgoodR {
        ImaskHfclkgoodR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SYSPLL GOOD"]
    #[inline(always)]
    pub fn imask_syspllgood(&self) -> ImaskSyspllgoodR {
        ImaskSyspllgoodR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HSCLK GOOD"]
    #[inline(always)]
    pub fn imask_hsclkgood(&self) -> ImaskHsclkgoodR {
        ImaskHsclkgoodR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable the LFOSCGOOD interrupt. LFOSCGOOD indicates that the LFOSC has started successfully."]
    #[inline(always)]
    pub fn imask_lfoscgood(&mut self) -> ImaskLfoscgoodW<ImaskSpec> {
        ImaskLfoscgoodW::new(self, 0)
    }
    #[doc = "Bit 1 - Analog Clocking Consistency Error"]
    #[inline(always)]
    pub fn imask_anaclkerr(&mut self) -> ImaskAnaclkerrW<ImaskSpec> {
        ImaskAnaclkerrW::new(self, 1)
    }
    #[doc = "Bit 2 - Flash Single Error Correct"]
    #[inline(always)]
    pub fn imask_flashsec(&mut self) -> ImaskFlashsecW<ImaskSpec> {
        ImaskFlashsecW::new(self, 2)
    }
    #[doc = "Bit 3 - SRAM Single Error Correct"]
    #[inline(always)]
    pub fn imask_sramsec(&mut self) -> ImaskSramsecW<ImaskSpec> {
        ImaskSramsecW::new(self, 3)
    }
    #[doc = "Bit 4 - LFXT GOOD"]
    #[inline(always)]
    pub fn imask_lfxtgood(&mut self) -> ImaskLfxtgoodW<ImaskSpec> {
        ImaskLfxtgoodW::new(self, 4)
    }
    #[doc = "Bit 5 - HFCLK GOOD"]
    #[inline(always)]
    pub fn imask_hfclkgood(&mut self) -> ImaskHfclkgoodW<ImaskSpec> {
        ImaskHfclkgoodW::new(self, 5)
    }
    #[doc = "Bit 6 - SYSPLL GOOD"]
    #[inline(always)]
    pub fn imask_syspllgood(&mut self) -> ImaskSyspllgoodW<ImaskSpec> {
        ImaskSyspllgoodW::new(self, 6)
    }
    #[doc = "Bit 7 - HSCLK GOOD"]
    #[inline(always)]
    pub fn imask_hsclkgood(&mut self) -> ImaskHsclkgoodW<ImaskSpec> {
        ImaskHsclkgoodW::new(self, 7)
    }
}
#[doc = "SYSCTL interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImaskSpec;
impl crate::RegisterSpec for ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imask::R`](R) reader structure"]
impl crate::Readable for ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`imask::W`](W) writer structure"]
impl crate::Writable for ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMASK to value 0"]
impl crate::Resettable for ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
