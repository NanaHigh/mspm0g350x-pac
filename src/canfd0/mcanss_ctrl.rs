#[doc = "Register `MCANSS_CTRL` reader"]
pub type R = crate::R<McanssCtrlSpec>;
#[doc = "Register `MCANSS_CTRL` writer"]
pub type W = crate::W<McanssCtrlSpec>;
#[doc = "Field `MCANSS_CTRL_DBGSUSP_FREE` reader - Debug Suspend Free Bit. Enables debug suspend. 0 Disable debug suspend 1 Enable debug suspend"]
pub type McanssCtrlDbgsuspFreeR = crate::BitReader;
#[doc = "Field `MCANSS_CTRL_DBGSUSP_FREE` writer - Debug Suspend Free Bit. Enables debug suspend. 0 Disable debug suspend 1 Enable debug suspend"]
pub type McanssCtrlDbgsuspFreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCANSS_CTRL_WAKEUPREQEN` reader - Wakeup Request Enable. Enables the MCANSS to wakeup on CAN RXD activity. 0 Disable wakeup request 1 Enables wakeup request"]
pub type McanssCtrlWakeupreqenR = crate::BitReader;
#[doc = "Field `MCANSS_CTRL_WAKEUPREQEN` writer - Wakeup Request Enable. Enables the MCANSS to wakeup on CAN RXD activity. 0 Disable wakeup request 1 Enables wakeup request"]
pub type McanssCtrlWakeupreqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCANSS_CTRL_AUTOWAKEUP` reader - Automatic Wakeup Enable. Enables the MCANSS to automatically clear the MCAN CCCR.INIT bit, fully waking the MCAN up, on an enabled wakeup request. 0 Disable the automatic write to CCCR.INIT 1 Enable the automatic write to CCCR.INIT"]
pub type McanssCtrlAutowakeupR = crate::BitReader;
#[doc = "Field `MCANSS_CTRL_AUTOWAKEUP` writer - Automatic Wakeup Enable. Enables the MCANSS to automatically clear the MCAN CCCR.INIT bit, fully waking the MCAN up, on an enabled wakeup request. 0 Disable the automatic write to CCCR.INIT 1 Enable the automatic write to CCCR.INIT"]
pub type McanssCtrlAutowakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCANSS_CTRL_EXT_TS_CNTR_EN` reader - External Timestamp Counter Enable. 0 External timestamp counter disabled 1 External timestamp counter enabled"]
pub type McanssCtrlExtTsCntrEnR = crate::BitReader;
#[doc = "Field `MCANSS_CTRL_EXT_TS_CNTR_EN` writer - External Timestamp Counter Enable. 0 External timestamp counter disabled 1 External timestamp counter enabled"]
pub type McanssCtrlExtTsCntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - Debug Suspend Free Bit. Enables debug suspend. 0 Disable debug suspend 1 Enable debug suspend"]
    #[inline(always)]
    pub fn mcanss_ctrl_dbgsusp_free(&self) -> McanssCtrlDbgsuspFreeR {
        McanssCtrlDbgsuspFreeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup Request Enable. Enables the MCANSS to wakeup on CAN RXD activity. 0 Disable wakeup request 1 Enables wakeup request"]
    #[inline(always)]
    pub fn mcanss_ctrl_wakeupreqen(&self) -> McanssCtrlWakeupreqenR {
        McanssCtrlWakeupreqenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic Wakeup Enable. Enables the MCANSS to automatically clear the MCAN CCCR.INIT bit, fully waking the MCAN up, on an enabled wakeup request. 0 Disable the automatic write to CCCR.INIT 1 Enable the automatic write to CCCR.INIT"]
    #[inline(always)]
    pub fn mcanss_ctrl_autowakeup(&self) -> McanssCtrlAutowakeupR {
        McanssCtrlAutowakeupR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - External Timestamp Counter Enable. 0 External timestamp counter disabled 1 External timestamp counter enabled"]
    #[inline(always)]
    pub fn mcanss_ctrl_ext_ts_cntr_en(&self) -> McanssCtrlExtTsCntrEnR {
        McanssCtrlExtTsCntrEnR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Debug Suspend Free Bit. Enables debug suspend. 0 Disable debug suspend 1 Enable debug suspend"]
    #[inline(always)]
    pub fn mcanss_ctrl_dbgsusp_free(&mut self) -> McanssCtrlDbgsuspFreeW<McanssCtrlSpec> {
        McanssCtrlDbgsuspFreeW::new(self, 3)
    }
    #[doc = "Bit 4 - Wakeup Request Enable. Enables the MCANSS to wakeup on CAN RXD activity. 0 Disable wakeup request 1 Enables wakeup request"]
    #[inline(always)]
    pub fn mcanss_ctrl_wakeupreqen(&mut self) -> McanssCtrlWakeupreqenW<McanssCtrlSpec> {
        McanssCtrlWakeupreqenW::new(self, 4)
    }
    #[doc = "Bit 5 - Automatic Wakeup Enable. Enables the MCANSS to automatically clear the MCAN CCCR.INIT bit, fully waking the MCAN up, on an enabled wakeup request. 0 Disable the automatic write to CCCR.INIT 1 Enable the automatic write to CCCR.INIT"]
    #[inline(always)]
    pub fn mcanss_ctrl_autowakeup(&mut self) -> McanssCtrlAutowakeupW<McanssCtrlSpec> {
        McanssCtrlAutowakeupW::new(self, 5)
    }
    #[doc = "Bit 6 - External Timestamp Counter Enable. 0 External timestamp counter disabled 1 External timestamp counter enabled"]
    #[inline(always)]
    pub fn mcanss_ctrl_ext_ts_cntr_en(&mut self) -> McanssCtrlExtTsCntrEnW<McanssCtrlSpec> {
        McanssCtrlExtTsCntrEnW::new(self, 6)
    }
}
#[doc = "MCAN Subsystem Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanss_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanssCtrlSpec;
impl crate::RegisterSpec for McanssCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanss_ctrl::R`](R) reader structure"]
impl crate::Readable for McanssCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanss_ctrl::W`](W) writer structure"]
impl crate::Writable for McanssCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANSS_CTRL to value 0x08"]
impl crate::Resettable for McanssCtrlSpec {
    const RESET_VALUE: u32 = 0x08;
}
