#[doc = "Register `MCAN_RWD` reader"]
pub type R = crate::R<McanRwdSpec>;
#[doc = "Register `MCAN_RWD` writer"]
pub type W = crate::W<McanRwdSpec>;
#[doc = "Field `MCAN_RWD_WDC` reader - Watchdog Configuration. Start value of the Message RAM Watchdog Counter. With the reset value of &amp;quot;00&amp;quot; the counter is disabled. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanRwdWdcR = crate::FieldReader;
#[doc = "Field `MCAN_RWD_WDC` writer - Watchdog Configuration. Start value of the Message RAM Watchdog Counter. With the reset value of &amp;quot;00&amp;quot; the counter is disabled. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanRwdWdcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MCAN_RWD_WDV` reader - Watchdog Value. Acutal Message RAM Watchdog Counter Value. The RAM Watchdog monitors the READY output of the Message RAM. A Message RAM access via the MCAN's Generic Master Interface starts the Message RAM Watchdog Counter with the value configured by the WDC field. The counter is reloaded with WDC when the Message RAM signals successful completion by activating its READY output. In case there is no response from the Message RAM until the counter has counted down to zero, the counter stops and interrupt flag MCAN_IR.WDI is set. The RAM Watchdog Counter is clocked by the host (system) clock."]
pub type McanRwdWdvR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Watchdog Configuration. Start value of the Message RAM Watchdog Counter. With the reset value of &amp;quot;00&amp;quot; the counter is disabled. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_rwd_wdc(&self) -> McanRwdWdcR {
        McanRwdWdcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Watchdog Value. Acutal Message RAM Watchdog Counter Value. The RAM Watchdog monitors the READY output of the Message RAM. A Message RAM access via the MCAN's Generic Master Interface starts the Message RAM Watchdog Counter with the value configured by the WDC field. The counter is reloaded with WDC when the Message RAM signals successful completion by activating its READY output. In case there is no response from the Message RAM until the counter has counted down to zero, the counter stops and interrupt flag MCAN_IR.WDI is set. The RAM Watchdog Counter is clocked by the host (system) clock."]
    #[inline(always)]
    pub fn mcan_rwd_wdv(&self) -> McanRwdWdvR {
        McanRwdWdvR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Watchdog Configuration. Start value of the Message RAM Watchdog Counter. With the reset value of &amp;quot;00&amp;quot; the counter is disabled. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_rwd_wdc(&mut self) -> McanRwdWdcW<McanRwdSpec> {
        McanRwdWdcW::new(self, 0)
    }
}
#[doc = "MCAN RAM Watchdog\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_rwd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_rwd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanRwdSpec;
impl crate::RegisterSpec for McanRwdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_rwd::R`](R) reader structure"]
impl crate::Readable for McanRwdSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_rwd::W`](W) writer structure"]
impl crate::Writable for McanRwdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_RWD to value 0"]
impl crate::Resettable for McanRwdSpec {
    const RESET_VALUE: u32 = 0;
}
