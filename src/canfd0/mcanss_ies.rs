#[doc = "Register `MCANSS_IES` reader"]
pub type R = crate::R<McanssIesSpec>;
#[doc = "Field `MCANSS_IES_EXT_TS_CNTR_OVFL` reader - External Timestamp Counter Overflow Interrupt Enable Status. To set, use the CANSS_IE.EXT_TS_CNTR_OVFL bit. To clear, use the MCANSS_IECS.EXT_TS_CNTR_OVFL bit. 0 External timestamp counter overflow interrupt is not enabled 1 External timestamp counter overflow interrupt is enabled"]
pub type McanssIesExtTsCntrOvflR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - External Timestamp Counter Overflow Interrupt Enable Status. To set, use the CANSS_IE.EXT_TS_CNTR_OVFL bit. To clear, use the MCANSS_IECS.EXT_TS_CNTR_OVFL bit. 0 External timestamp counter overflow interrupt is not enabled 1 External timestamp counter overflow interrupt is enabled"]
    #[inline(always)]
    pub fn mcanss_ies_ext_ts_cntr_ovfl(&self) -> McanssIesExtTsCntrOvflR {
        McanssIesExtTsCntrOvflR::new((self.bits & 1) != 0)
    }
}
#[doc = "MCAN Subsystem Interrupt Enable Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_ies::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanssIesSpec;
impl crate::RegisterSpec for McanssIesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanss_ies::R`](R) reader structure"]
impl crate::Readable for McanssIesSpec {}
#[doc = "`reset()` method sets MCANSS_IES to value 0"]
impl crate::Resettable for McanssIesSpec {
    const RESET_VALUE: u32 = 0;
}
