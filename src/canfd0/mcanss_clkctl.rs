#[doc = "Register `MCANSS_CLKCTL` reader"]
pub type R = crate::R<McanssClkctlSpec>;
#[doc = "Register `MCANSS_CLKCTL` writer"]
pub type W = crate::W<McanssClkctlSpec>;
#[doc = "This bit is used to enable/disable MCAN clock (both host clock and functional clock) gating request. Note: This bit can be reset by HW by Clock-Stop Wake-up via CAN RX Activity. See spec for more details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McanssClkctlStopreq {
    #[doc = "0: DISABLE"]
    McanssClkctlStopreqDisable = 0,
    #[doc = "1: ENABLE"]
    McanssClkctlStopreqEnable = 1,
}
impl From<McanssClkctlStopreq> for bool {
    #[inline(always)]
    fn from(variant: McanssClkctlStopreq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCANSS_CLKCTL_STOPREQ` reader - This bit is used to enable/disable MCAN clock (both host clock and functional clock) gating request. Note: This bit can be reset by HW by Clock-Stop Wake-up via CAN RX Activity. See spec for more details."]
pub type McanssClkctlStopreqR = crate::BitReader<McanssClkctlStopreq>;
impl McanssClkctlStopreqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McanssClkctlStopreq {
        match self.bits {
            false => McanssClkctlStopreq::McanssClkctlStopreqDisable,
            true => McanssClkctlStopreq::McanssClkctlStopreqEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_mcanss_clkctl_stopreq_disable(&self) -> bool {
        *self == McanssClkctlStopreq::McanssClkctlStopreqDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_mcanss_clkctl_stopreq_enable(&self) -> bool {
        *self == McanssClkctlStopreq::McanssClkctlStopreqEnable
    }
}
#[doc = "Field `MCANSS_CLKCTL_STOPREQ` writer - This bit is used to enable/disable MCAN clock (both host clock and functional clock) gating request. Note: This bit can be reset by HW by Clock-Stop Wake-up via CAN RX Activity. See spec for more details."]
pub type McanssClkctlStopreqW<'a, REG> = crate::BitWriter<'a, REG, McanssClkctlStopreq>;
impl<'a, REG> McanssClkctlStopreqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn mcanss_clkctl_stopreq_disable(self) -> &'a mut crate::W<REG> {
        self.variant(McanssClkctlStopreq::McanssClkctlStopreqDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn mcanss_clkctl_stopreq_enable(self) -> &'a mut crate::W<REG> {
        self.variant(McanssClkctlStopreq::McanssClkctlStopreqEnable)
    }
}
#[doc = "This bit contols enabling or disabling the MCAN IP clock stop wakeup interrupt (when MCANSS_CTRL.WAKEUPREQEN wakeup request is enabled to wakeup MCAN IP upon CAN RXD activity)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McanssClkctlWakeupIntEn {
    #[doc = "0: DISABLE"]
    McanssClkctlWakeupIntEnDisable = 0,
    #[doc = "1: ENABLE"]
    McanssClkctlWakeupIntEnEnable = 1,
}
impl From<McanssClkctlWakeupIntEn> for bool {
    #[inline(always)]
    fn from(variant: McanssClkctlWakeupIntEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCANSS_CLKCTL_WAKEUP_INT_EN` reader - This bit contols enabling or disabling the MCAN IP clock stop wakeup interrupt (when MCANSS_CTRL.WAKEUPREQEN wakeup request is enabled to wakeup MCAN IP upon CAN RXD activity)"]
pub type McanssClkctlWakeupIntEnR = crate::BitReader<McanssClkctlWakeupIntEn>;
impl McanssClkctlWakeupIntEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McanssClkctlWakeupIntEn {
        match self.bits {
            false => McanssClkctlWakeupIntEn::McanssClkctlWakeupIntEnDisable,
            true => McanssClkctlWakeupIntEn::McanssClkctlWakeupIntEnEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_mcanss_clkctl_wakeup_int_en_disable(&self) -> bool {
        *self == McanssClkctlWakeupIntEn::McanssClkctlWakeupIntEnDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_mcanss_clkctl_wakeup_int_en_enable(&self) -> bool {
        *self == McanssClkctlWakeupIntEn::McanssClkctlWakeupIntEnEnable
    }
}
#[doc = "Field `MCANSS_CLKCTL_WAKEUP_INT_EN` writer - This bit contols enabling or disabling the MCAN IP clock stop wakeup interrupt (when MCANSS_CTRL.WAKEUPREQEN wakeup request is enabled to wakeup MCAN IP upon CAN RXD activity)"]
pub type McanssClkctlWakeupIntEnW<'a, REG> = crate::BitWriter<'a, REG, McanssClkctlWakeupIntEn>;
impl<'a, REG> McanssClkctlWakeupIntEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn mcanss_clkctl_wakeup_int_en_disable(self) -> &'a mut crate::W<REG> {
        self.variant(McanssClkctlWakeupIntEn::McanssClkctlWakeupIntEnDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn mcanss_clkctl_wakeup_int_en_enable(self) -> &'a mut crate::W<REG> {
        self.variant(McanssClkctlWakeupIntEn::McanssClkctlWakeupIntEnEnable)
    }
}
#[doc = "Setting this bit enables the glitch filter on MCAN RXD input, which wakes up the MCAN controller to exit clock gating.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McanssClkctlWkupGltfltEn {
    #[doc = "0: DISABLE"]
    McanssClkctlWkupGltfltEnDisable = 0,
    #[doc = "1: ENABLE"]
    McanssClkctlWkupGltfltEnEnable = 1,
}
impl From<McanssClkctlWkupGltfltEn> for bool {
    #[inline(always)]
    fn from(variant: McanssClkctlWkupGltfltEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCANSS_CLKCTL_WKUP_GLTFLT_EN` reader - Setting this bit enables the glitch filter on MCAN RXD input, which wakes up the MCAN controller to exit clock gating."]
pub type McanssClkctlWkupGltfltEnR = crate::BitReader<McanssClkctlWkupGltfltEn>;
impl McanssClkctlWkupGltfltEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McanssClkctlWkupGltfltEn {
        match self.bits {
            false => McanssClkctlWkupGltfltEn::McanssClkctlWkupGltfltEnDisable,
            true => McanssClkctlWkupGltfltEn::McanssClkctlWkupGltfltEnEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_mcanss_clkctl_wkup_gltflt_en_disable(&self) -> bool {
        *self == McanssClkctlWkupGltfltEn::McanssClkctlWkupGltfltEnDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_mcanss_clkctl_wkup_gltflt_en_enable(&self) -> bool {
        *self == McanssClkctlWkupGltfltEn::McanssClkctlWkupGltfltEnEnable
    }
}
#[doc = "Field `MCANSS_CLKCTL_WKUP_GLTFLT_EN` writer - Setting this bit enables the glitch filter on MCAN RXD input, which wakes up the MCAN controller to exit clock gating."]
pub type McanssClkctlWkupGltfltEnW<'a, REG> = crate::BitWriter<'a, REG, McanssClkctlWkupGltfltEn>;
impl<'a, REG> McanssClkctlWkupGltfltEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn mcanss_clkctl_wkup_gltflt_en_disable(self) -> &'a mut crate::W<REG> {
        self.variant(McanssClkctlWkupGltfltEn::McanssClkctlWkupGltfltEnDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn mcanss_clkctl_wkup_gltflt_en_enable(self) -> &'a mut crate::W<REG> {
        self.variant(McanssClkctlWkupGltfltEn::McanssClkctlWkupGltfltEnEnable)
    }
}
impl R {
    #[doc = "Bit 0 - This bit is used to enable/disable MCAN clock (both host clock and functional clock) gating request. Note: This bit can be reset by HW by Clock-Stop Wake-up via CAN RX Activity. See spec for more details."]
    #[inline(always)]
    pub fn mcanss_clkctl_stopreq(&self) -> McanssClkctlStopreqR {
        McanssClkctlStopreqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - This bit contols enabling or disabling the MCAN IP clock stop wakeup interrupt (when MCANSS_CTRL.WAKEUPREQEN wakeup request is enabled to wakeup MCAN IP upon CAN RXD activity)"]
    #[inline(always)]
    pub fn mcanss_clkctl_wakeup_int_en(&self) -> McanssClkctlWakeupIntEnR {
        McanssClkctlWakeupIntEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Setting this bit enables the glitch filter on MCAN RXD input, which wakes up the MCAN controller to exit clock gating."]
    #[inline(always)]
    pub fn mcanss_clkctl_wkup_gltflt_en(&self) -> McanssClkctlWkupGltfltEnR {
        McanssClkctlWkupGltfltEnR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to enable/disable MCAN clock (both host clock and functional clock) gating request. Note: This bit can be reset by HW by Clock-Stop Wake-up via CAN RX Activity. See spec for more details."]
    #[inline(always)]
    pub fn mcanss_clkctl_stopreq(&mut self) -> McanssClkctlStopreqW<McanssClkctlSpec> {
        McanssClkctlStopreqW::new(self, 0)
    }
    #[doc = "Bit 4 - This bit contols enabling or disabling the MCAN IP clock stop wakeup interrupt (when MCANSS_CTRL.WAKEUPREQEN wakeup request is enabled to wakeup MCAN IP upon CAN RXD activity)"]
    #[inline(always)]
    pub fn mcanss_clkctl_wakeup_int_en(&mut self) -> McanssClkctlWakeupIntEnW<McanssClkctlSpec> {
        McanssClkctlWakeupIntEnW::new(self, 4)
    }
    #[doc = "Bit 8 - Setting this bit enables the glitch filter on MCAN RXD input, which wakes up the MCAN controller to exit clock gating."]
    #[inline(always)]
    pub fn mcanss_clkctl_wkup_gltflt_en(&mut self) -> McanssClkctlWkupGltfltEnW<McanssClkctlSpec> {
        McanssClkctlWkupGltfltEnW::new(self, 8)
    }
}
#[doc = "MCAN-SS clock stop control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_clkctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanss_clkctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanssClkctlSpec;
impl crate::RegisterSpec for McanssClkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanss_clkctl::R`](R) reader structure"]
impl crate::Readable for McanssClkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanss_clkctl::W`](W) writer structure"]
impl crate::Writable for McanssClkctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANSS_CLKCTL to value 0"]
impl crate::Resettable for McanssClkctlSpec {
    const RESET_VALUE: u32 = 0;
}
