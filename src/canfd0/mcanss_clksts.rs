#[doc = "Register `MCANSS_CLKSTS` reader"]
pub type R = crate::R<McanssClkstsSpec>;
#[doc = "Clock stop acknowledge status from MCAN IP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McanssClkstsClkstopAcksts {
    #[doc = "0: RESET"]
    McanssClkstsClkstopAckstsReset = 0,
    #[doc = "1: SET"]
    McanssClkstsClkstopAckstsSet = 1,
}
impl From<McanssClkstsClkstopAcksts> for bool {
    #[inline(always)]
    fn from(variant: McanssClkstsClkstopAcksts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCANSS_CLKSTS_CLKSTOP_ACKSTS` reader - Clock stop acknowledge status from MCAN IP"]
pub type McanssClkstsClkstopAckstsR = crate::BitReader<McanssClkstsClkstopAcksts>;
impl McanssClkstsClkstopAckstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McanssClkstsClkstopAcksts {
        match self.bits {
            false => McanssClkstsClkstopAcksts::McanssClkstsClkstopAckstsReset,
            true => McanssClkstsClkstopAcksts::McanssClkstsClkstopAckstsSet,
        }
    }
    #[doc = "RESET"]
    #[inline(always)]
    pub fn is_mcanss_clksts_clkstop_acksts_reset(&self) -> bool {
        *self == McanssClkstsClkstopAcksts::McanssClkstsClkstopAckstsReset
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mcanss_clksts_clkstop_acksts_set(&self) -> bool {
        *self == McanssClkstsClkstopAcksts::McanssClkstsClkstopAckstsSet
    }
}
#[doc = "MCANSS clock stop HW override status bit. This bit indicates when the MCANSS_CLKCTL.STOPREQ bit has been cleared by HW when a clock-stop wake-up event via CAN RX activity is trigged.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McanssClkstsStopreqHwOvr {
    #[doc = "0: RESET"]
    McanssClkstsStopreqHwOvrReset = 0,
    #[doc = "1: SET"]
    McanssClkstsStopreqHwOvrSet = 1,
}
impl From<McanssClkstsStopreqHwOvr> for bool {
    #[inline(always)]
    fn from(variant: McanssClkstsStopreqHwOvr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCANSS_CLKSTS_STOPREQ_HW_OVR` reader - MCANSS clock stop HW override status bit. This bit indicates when the MCANSS_CLKCTL.STOPREQ bit has been cleared by HW when a clock-stop wake-up event via CAN RX activity is trigged."]
pub type McanssClkstsStopreqHwOvrR = crate::BitReader<McanssClkstsStopreqHwOvr>;
impl McanssClkstsStopreqHwOvrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McanssClkstsStopreqHwOvr {
        match self.bits {
            false => McanssClkstsStopreqHwOvr::McanssClkstsStopreqHwOvrReset,
            true => McanssClkstsStopreqHwOvr::McanssClkstsStopreqHwOvrSet,
        }
    }
    #[doc = "RESET"]
    #[inline(always)]
    pub fn is_mcanss_clksts_stopreq_hw_ovr_reset(&self) -> bool {
        *self == McanssClkstsStopreqHwOvr::McanssClkstsStopreqHwOvrReset
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mcanss_clksts_stopreq_hw_ovr_set(&self) -> bool {
        *self == McanssClkstsStopreqHwOvr::McanssClkstsStopreqHwOvrSet
    }
}
#[doc = "This bit indicates the status of MCAN contoller clock request from GPRCM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McanssClkstsCclkdone {
    #[doc = "0: RESET"]
    McanssClkstsCclkdoneReset = 0,
    #[doc = "1: SET"]
    McanssClkstsCclkdoneSet = 1,
}
impl From<McanssClkstsCclkdone> for bool {
    #[inline(always)]
    fn from(variant: McanssClkstsCclkdone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCANSS_CLKSTS_CCLKDONE` reader - This bit indicates the status of MCAN contoller clock request from GPRCM."]
pub type McanssClkstsCclkdoneR = crate::BitReader<McanssClkstsCclkdone>;
impl McanssClkstsCclkdoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> McanssClkstsCclkdone {
        match self.bits {
            false => McanssClkstsCclkdone::McanssClkstsCclkdoneReset,
            true => McanssClkstsCclkdone::McanssClkstsCclkdoneSet,
        }
    }
    #[doc = "RESET"]
    #[inline(always)]
    pub fn is_mcanss_clksts_cclkdone_reset(&self) -> bool {
        *self == McanssClkstsCclkdone::McanssClkstsCclkdoneReset
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mcanss_clksts_cclkdone_set(&self) -> bool {
        *self == McanssClkstsCclkdone::McanssClkstsCclkdoneSet
    }
}
impl R {
    #[doc = "Bit 0 - Clock stop acknowledge status from MCAN IP"]
    #[inline(always)]
    pub fn mcanss_clksts_clkstop_acksts(&self) -> McanssClkstsClkstopAckstsR {
        McanssClkstsClkstopAckstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - MCANSS clock stop HW override status bit. This bit indicates when the MCANSS_CLKCTL.STOPREQ bit has been cleared by HW when a clock-stop wake-up event via CAN RX activity is trigged."]
    #[inline(always)]
    pub fn mcanss_clksts_stopreq_hw_ovr(&self) -> McanssClkstsStopreqHwOvrR {
        McanssClkstsStopreqHwOvrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit indicates the status of MCAN contoller clock request from GPRCM."]
    #[inline(always)]
    pub fn mcanss_clksts_cclkdone(&self) -> McanssClkstsCclkdoneR {
        McanssClkstsCclkdoneR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "MCANSS clock stop status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_clksts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanssClkstsSpec;
impl crate::RegisterSpec for McanssClkstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanss_clksts::R`](R) reader structure"]
impl crate::Readable for McanssClkstsSpec {}
#[doc = "`reset()` method sets MCANSS_CLKSTS to value 0"]
impl crate::Resettable for McanssClkstsSpec {
    const RESET_VALUE: u32 = 0;
}
