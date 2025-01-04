#[doc = "Register `MCANSS_EXT_TS_UNSERVICED_INTR_CNTR` reader"]
pub type R = crate::R<McanssExtTsUnservicedIntrCntrSpec>;
#[doc = "Field `MCANSS_EXT_TS_UNSERVICED_INTR_CNTR_EXT_TS_INTR_CNTR` reader - External Timestamp Counter Unserviced Rollover Interrupts. If this value is &gt; 1, an MCANSS_EOI write of '1' to bit 0 will issue another interrupt. The status of this bit field is affected by the MCANSS_IRS.EXT_TS_CNTR_OVFL bit field."]
pub type McanssExtTsUnservicedIntrCntrExtTsIntrCntrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - External Timestamp Counter Unserviced Rollover Interrupts. If this value is &gt; 1, an MCANSS_EOI write of '1' to bit 0 will issue another interrupt. The status of this bit field is affected by the MCANSS_IRS.EXT_TS_CNTR_OVFL bit field."]
    #[inline(always)]
    pub fn mcanss_ext_ts_unserviced_intr_cntr_ext_ts_intr_cntr(
        &self,
    ) -> McanssExtTsUnservicedIntrCntrExtTsIntrCntrR {
        McanssExtTsUnservicedIntrCntrExtTsIntrCntrR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "MCAN Subsystem External Timestamp Unserviced Interrupts Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_ext_ts_unserviced_intr_cntr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanssExtTsUnservicedIntrCntrSpec;
impl crate::RegisterSpec for McanssExtTsUnservicedIntrCntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanss_ext_ts_unserviced_intr_cntr::R`](R) reader structure"]
impl crate::Readable for McanssExtTsUnservicedIntrCntrSpec {}
#[doc = "`reset()` method sets MCANSS_EXT_TS_UNSERVICED_INTR_CNTR to value 0"]
impl crate::Resettable for McanssExtTsUnservicedIntrCntrSpec {
    const RESET_VALUE: u32 = 0;
}
