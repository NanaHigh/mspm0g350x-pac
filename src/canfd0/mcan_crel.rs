#[doc = "Register `MCAN_CREL` reader"]
pub type R = crate::R<McanCrelSpec>;
#[doc = "Field `MCAN_CREL_DAY` reader - Time Stamp Day. Two digits, BCD-coded."]
pub type McanCrelDayR = crate::FieldReader;
#[doc = "Field `MCAN_CREL_MON` reader - Time Stamp Month. Two digits, BCD-coded."]
pub type McanCrelMonR = crate::FieldReader;
#[doc = "Field `MCAN_CREL_YEAR` reader - Time Stamp Year. One digit, BCD-coded."]
pub type McanCrelYearR = crate::FieldReader;
#[doc = "Field `MCAN_CREL_SUBSTEP` reader - Sub-Step of Core Release. One digit, BCD-coded."]
pub type McanCrelSubstepR = crate::FieldReader;
#[doc = "Field `MCAN_CREL_STEP` reader - Step of Core Release. One digit, BCD-coded."]
pub type McanCrelStepR = crate::FieldReader;
#[doc = "Field `MCAN_CREL_REL` reader - Core Release. One digit, BCD-coded."]
pub type McanCrelRelR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Time Stamp Day. Two digits, BCD-coded."]
    #[inline(always)]
    pub fn mcan_crel_day(&self) -> McanCrelDayR {
        McanCrelDayR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Time Stamp Month. Two digits, BCD-coded."]
    #[inline(always)]
    pub fn mcan_crel_mon(&self) -> McanCrelMonR {
        McanCrelMonR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Time Stamp Year. One digit, BCD-coded."]
    #[inline(always)]
    pub fn mcan_crel_year(&self) -> McanCrelYearR {
        McanCrelYearR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Sub-Step of Core Release. One digit, BCD-coded."]
    #[inline(always)]
    pub fn mcan_crel_substep(&self) -> McanCrelSubstepR {
        McanCrelSubstepR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Step of Core Release. One digit, BCD-coded."]
    #[inline(always)]
    pub fn mcan_crel_step(&self) -> McanCrelStepR {
        McanCrelStepR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Core Release. One digit, BCD-coded."]
    #[inline(always)]
    pub fn mcan_crel_rel(&self) -> McanCrelRelR {
        McanCrelRelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "MCAN Core Release Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_crel::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanCrelSpec;
impl crate::RegisterSpec for McanCrelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_crel::R`](R) reader structure"]
impl crate::Readable for McanCrelSpec {}
#[doc = "`reset()` method sets MCAN_CREL to value 0x3238_0608"]
impl crate::Resettable for McanCrelSpec {
    const RESET_VALUE: u32 = 0x3238_0608;
}
