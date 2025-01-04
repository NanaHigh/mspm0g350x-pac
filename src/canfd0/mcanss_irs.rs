#[doc = "Register `MCANSS_IRS` reader"]
pub type R = crate::R<McanssIrsSpec>;
#[doc = "Register `MCANSS_IRS` writer"]
pub type W = crate::W<McanssIrsSpec>;
#[doc = "Field `MCANSS_IRS_EXT_TS_CNTR_OVFL` reader - External Timestamp Counter Overflow Interrupt Status. This bit is set by HW or by a SW write of '1'. To clear, use the MCANSS_ICS.EXT_TS_CNTR_OVFL bit. 0 External timestamp counter has not overflowed 1 External timestamp counter has overflowed When this bit is set to '1' by HW or SW, the MCANSS_EXT_TS_UNSERVICED_INTR_CNTR.EXT_TS_INTR_CNTR bit field will increment by 1."]
pub type McanssIrsExtTsCntrOvflR = crate::BitReader;
#[doc = "Field `MCANSS_IRS_EXT_TS_CNTR_OVFL` writer - External Timestamp Counter Overflow Interrupt Status. This bit is set by HW or by a SW write of '1'. To clear, use the MCANSS_ICS.EXT_TS_CNTR_OVFL bit. 0 External timestamp counter has not overflowed 1 External timestamp counter has overflowed When this bit is set to '1' by HW or SW, the MCANSS_EXT_TS_UNSERVICED_INTR_CNTR.EXT_TS_INTR_CNTR bit field will increment by 1."]
pub type McanssIrsExtTsCntrOvflW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - External Timestamp Counter Overflow Interrupt Status. This bit is set by HW or by a SW write of '1'. To clear, use the MCANSS_ICS.EXT_TS_CNTR_OVFL bit. 0 External timestamp counter has not overflowed 1 External timestamp counter has overflowed When this bit is set to '1' by HW or SW, the MCANSS_EXT_TS_UNSERVICED_INTR_CNTR.EXT_TS_INTR_CNTR bit field will increment by 1."]
    #[inline(always)]
    pub fn mcanss_irs_ext_ts_cntr_ovfl(&self) -> McanssIrsExtTsCntrOvflR {
        McanssIrsExtTsCntrOvflR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Timestamp Counter Overflow Interrupt Status. This bit is set by HW or by a SW write of '1'. To clear, use the MCANSS_ICS.EXT_TS_CNTR_OVFL bit. 0 External timestamp counter has not overflowed 1 External timestamp counter has overflowed When this bit is set to '1' by HW or SW, the MCANSS_EXT_TS_UNSERVICED_INTR_CNTR.EXT_TS_INTR_CNTR bit field will increment by 1."]
    #[inline(always)]
    pub fn mcanss_irs_ext_ts_cntr_ovfl(&mut self) -> McanssIrsExtTsCntrOvflW<McanssIrsSpec> {
        McanssIrsExtTsCntrOvflW::new(self, 0)
    }
}
#[doc = "MCAN Subsystem Interrupt Raw Satus Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_irs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanss_irs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanssIrsSpec;
impl crate::RegisterSpec for McanssIrsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanss_irs::R`](R) reader structure"]
impl crate::Readable for McanssIrsSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanss_irs::W`](W) writer structure"]
impl crate::Writable for McanssIrsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANSS_IRS to value 0"]
impl crate::Resettable for McanssIrsSpec {
    const RESET_VALUE: u32 = 0;
}
