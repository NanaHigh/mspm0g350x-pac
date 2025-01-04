#[doc = "Register `MCANSS_IE` reader"]
pub type R = crate::R<McanssIeSpec>;
#[doc = "Register `MCANSS_IE` writer"]
pub type W = crate::W<McanssIeSpec>;
#[doc = "Field `MCANSS_IE_EXT_TS_CNTR_OVFL` reader - External Timestamp Counter Overflow Interrupt Enable. A write of '0' has no effect. A write of '1' sets the MCANSS_IES.EXT_TS_CNTR_OVFL bit."]
pub type McanssIeExtTsCntrOvflR = crate::BitReader;
#[doc = "Field `MCANSS_IE_EXT_TS_CNTR_OVFL` writer - External Timestamp Counter Overflow Interrupt Enable. A write of '0' has no effect. A write of '1' sets the MCANSS_IES.EXT_TS_CNTR_OVFL bit."]
pub type McanssIeExtTsCntrOvflW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - External Timestamp Counter Overflow Interrupt Enable. A write of '0' has no effect. A write of '1' sets the MCANSS_IES.EXT_TS_CNTR_OVFL bit."]
    #[inline(always)]
    pub fn mcanss_ie_ext_ts_cntr_ovfl(&self) -> McanssIeExtTsCntrOvflR {
        McanssIeExtTsCntrOvflR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Timestamp Counter Overflow Interrupt Enable. A write of '0' has no effect. A write of '1' sets the MCANSS_IES.EXT_TS_CNTR_OVFL bit."]
    #[inline(always)]
    pub fn mcanss_ie_ext_ts_cntr_ovfl(&mut self) -> McanssIeExtTsCntrOvflW<McanssIeSpec> {
        McanssIeExtTsCntrOvflW::new(self, 0)
    }
}
#[doc = "MCAN Subsystem Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanss_ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanssIeSpec;
impl crate::RegisterSpec for McanssIeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanss_ie::R`](R) reader structure"]
impl crate::Readable for McanssIeSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanss_ie::W`](W) writer structure"]
impl crate::Writable for McanssIeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANSS_IE to value 0"]
impl crate::Resettable for McanssIeSpec {
    const RESET_VALUE: u32 = 0;
}
