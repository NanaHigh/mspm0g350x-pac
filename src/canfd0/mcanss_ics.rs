#[doc = "Register `MCANSS_ICS` reader"]
pub type R = crate::R<McanssIcsSpec>;
#[doc = "Register `MCANSS_ICS` writer"]
pub type W = crate::W<McanssIcsSpec>;
#[doc = "Field `MCANSS_ICS_EXT_TS_CNTR_OVFL` reader - External Timestamp Counter Overflow Interrupt Status Clear. Reads always return a 0. 0 Write of '0' has no effect 1 Write of '1' clears the MCANSS_IRS.EXT_TS_CNTR_OVFL bit"]
pub type McanssIcsExtTsCntrOvflR = crate::BitReader;
#[doc = "Field `MCANSS_ICS_EXT_TS_CNTR_OVFL` writer - External Timestamp Counter Overflow Interrupt Status Clear. Reads always return a 0. 0 Write of '0' has no effect 1 Write of '1' clears the MCANSS_IRS.EXT_TS_CNTR_OVFL bit"]
pub type McanssIcsExtTsCntrOvflW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - External Timestamp Counter Overflow Interrupt Status Clear. Reads always return a 0. 0 Write of '0' has no effect 1 Write of '1' clears the MCANSS_IRS.EXT_TS_CNTR_OVFL bit"]
    #[inline(always)]
    pub fn mcanss_ics_ext_ts_cntr_ovfl(&self) -> McanssIcsExtTsCntrOvflR {
        McanssIcsExtTsCntrOvflR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Timestamp Counter Overflow Interrupt Status Clear. Reads always return a 0. 0 Write of '0' has no effect 1 Write of '1' clears the MCANSS_IRS.EXT_TS_CNTR_OVFL bit"]
    #[inline(always)]
    pub fn mcanss_ics_ext_ts_cntr_ovfl(&mut self) -> McanssIcsExtTsCntrOvflW<McanssIcsSpec> {
        McanssIcsExtTsCntrOvflW::new(self, 0)
    }
}
#[doc = "MCAN Subsystem Interrupt Clear Shadow Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_ics::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanss_ics::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanssIcsSpec;
impl crate::RegisterSpec for McanssIcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanss_ics::R`](R) reader structure"]
impl crate::Readable for McanssIcsSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanss_ics::W`](W) writer structure"]
impl crate::Writable for McanssIcsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANSS_ICS to value 0"]
impl crate::Resettable for McanssIcsSpec {
    const RESET_VALUE: u32 = 0;
}
