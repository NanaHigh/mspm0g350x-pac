#[doc = "Register `MCANSS_IECS` reader"]
pub type R = crate::R<McanssIecsSpec>;
#[doc = "Register `MCANSS_IECS` writer"]
pub type W = crate::W<McanssIecsSpec>;
#[doc = "Field `MCANSS_IECS_EXT_TS_CNTR_OVFL` reader - External Timestamp Counter Overflow Interrupt Enable Clear. Reads always return a 0. 0 Write of '0' has no effect 1 Write of '1' clears the MCANSS_IES.EXT_TS_CNTR_OVFL bit"]
pub type McanssIecsExtTsCntrOvflR = crate::BitReader;
#[doc = "Field `MCANSS_IECS_EXT_TS_CNTR_OVFL` writer - External Timestamp Counter Overflow Interrupt Enable Clear. Reads always return a 0. 0 Write of '0' has no effect 1 Write of '1' clears the MCANSS_IES.EXT_TS_CNTR_OVFL bit"]
pub type McanssIecsExtTsCntrOvflW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - External Timestamp Counter Overflow Interrupt Enable Clear. Reads always return a 0. 0 Write of '0' has no effect 1 Write of '1' clears the MCANSS_IES.EXT_TS_CNTR_OVFL bit"]
    #[inline(always)]
    pub fn mcanss_iecs_ext_ts_cntr_ovfl(&self) -> McanssIecsExtTsCntrOvflR {
        McanssIecsExtTsCntrOvflR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Timestamp Counter Overflow Interrupt Enable Clear. Reads always return a 0. 0 Write of '0' has no effect 1 Write of '1' clears the MCANSS_IES.EXT_TS_CNTR_OVFL bit"]
    #[inline(always)]
    pub fn mcanss_iecs_ext_ts_cntr_ovfl(&mut self) -> McanssIecsExtTsCntrOvflW<McanssIecsSpec> {
        McanssIecsExtTsCntrOvflW::new(self, 0)
    }
}
#[doc = "MCAN Subsystem Interrupt Enable Clear Shadow Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_iecs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanss_iecs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanssIecsSpec;
impl crate::RegisterSpec for McanssIecsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanss_iecs::R`](R) reader structure"]
impl crate::Readable for McanssIecsSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanss_iecs::W`](W) writer structure"]
impl crate::Writable for McanssIecsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANSS_IECS to value 0"]
impl crate::Resettable for McanssIecsSpec {
    const RESET_VALUE: u32 = 0;
}
