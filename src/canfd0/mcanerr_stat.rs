#[doc = "Register `MCANERR_STAT` reader"]
pub type R = crate::R<McanerrStatSpec>;
#[doc = "Field `MCANERR_STAT_NUM_RAMS` reader - Number of RAMs. Number of ECC RAMs serviced by the aggregator."]
pub type McanerrStatNumRamsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Number of RAMs. Number of ECC RAMs serviced by the aggregator."]
    #[inline(always)]
    pub fn mcanerr_stat_num_rams(&self) -> McanerrStatNumRamsR {
        McanerrStatNumRamsR::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "MCAN Error Misc Status\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanerrStatSpec;
impl crate::RegisterSpec for McanerrStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanerr_stat::R`](R) reader structure"]
impl crate::Readable for McanerrStatSpec {}
#[doc = "`reset()` method sets MCANERR_STAT to value 0x02"]
impl crate::Resettable for McanerrStatSpec {
    const RESET_VALUE: u32 = 0x02;
}
