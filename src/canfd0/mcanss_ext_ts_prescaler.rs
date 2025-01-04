#[doc = "Register `MCANSS_EXT_TS_PRESCALER` reader"]
pub type R = crate::R<McanssExtTsPrescalerSpec>;
#[doc = "Register `MCANSS_EXT_TS_PRESCALER` writer"]
pub type W = crate::W<McanssExtTsPrescalerSpec>;
#[doc = "Field `MCANSS_EXT_TS_PRESCALER_PRESCALER` reader - External Timestamp Prescaler Reload Value. The external timestamp count rate is the host (system) clock rate divided by this value, except in the case of 0. A zero value in this bit field will act identically to a value of 0x000001."]
pub type McanssExtTsPrescalerPrescalerR = crate::FieldReader<u32>;
#[doc = "Field `MCANSS_EXT_TS_PRESCALER_PRESCALER` writer - External Timestamp Prescaler Reload Value. The external timestamp count rate is the host (system) clock rate divided by this value, except in the case of 0. A zero value in this bit field will act identically to a value of 0x000001."]
pub type McanssExtTsPrescalerPrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - External Timestamp Prescaler Reload Value. The external timestamp count rate is the host (system) clock rate divided by this value, except in the case of 0. A zero value in this bit field will act identically to a value of 0x000001."]
    #[inline(always)]
    pub fn mcanss_ext_ts_prescaler_prescaler(&self) -> McanssExtTsPrescalerPrescalerR {
        McanssExtTsPrescalerPrescalerR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - External Timestamp Prescaler Reload Value. The external timestamp count rate is the host (system) clock rate divided by this value, except in the case of 0. A zero value in this bit field will act identically to a value of 0x000001."]
    #[inline(always)]
    pub fn mcanss_ext_ts_prescaler_prescaler(
        &mut self,
    ) -> McanssExtTsPrescalerPrescalerW<McanssExtTsPrescalerSpec> {
        McanssExtTsPrescalerPrescalerW::new(self, 0)
    }
}
#[doc = "MCAN Subsystem External Timestamp Prescaler 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_ext_ts_prescaler::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcanss_ext_ts_prescaler::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanssExtTsPrescalerSpec;
impl crate::RegisterSpec for McanssExtTsPrescalerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanss_ext_ts_prescaler::R`](R) reader structure"]
impl crate::Readable for McanssExtTsPrescalerSpec {}
#[doc = "`write(|w| ..)` method takes [`mcanss_ext_ts_prescaler::W`](W) writer structure"]
impl crate::Writable for McanssExtTsPrescalerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCANSS_EXT_TS_PRESCALER to value 0"]
impl crate::Resettable for McanssExtTsPrescalerSpec {
    const RESET_VALUE: u32 = 0;
}
