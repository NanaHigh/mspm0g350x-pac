#[doc = "Register `TEST_RESULTS` reader"]
pub type R = crate::R<TestResultsSpec>;
#[doc = "Field `TEST_RESULTS_DIG_TEST` reader - Bit 0 indicates if the first decimation rate test and health test(verifies conditioning, decimation, and captured buffer) fails and Bit 1 indicates if the second decimation test and health test fails Bit 0 - decim_test0 (decim = 0x0) Bit 1 - decim_test1 (decim = 0x1) ..."]
pub type TestResultsDigTestR = crate::FieldReader;
#[doc = "Field `TEST_RESULTS_ANA_TEST` reader - Runs through 4096 samples from an enabled entropy source and verifies that none of the health tests failed, indicating sufficient entropy was produced by the analog components"]
pub type TestResultsAnaTestR = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - Bit 0 indicates if the first decimation rate test and health test(verifies conditioning, decimation, and captured buffer) fails and Bit 1 indicates if the second decimation test and health test fails Bit 0 - decim_test0 (decim = 0x0) Bit 1 - decim_test1 (decim = 0x1) ..."]
    #[inline(always)]
    pub fn test_results_dig_test(&self) -> TestResultsDigTestR {
        TestResultsDigTestR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Runs through 4096 samples from an enabled entropy source and verifies that none of the health tests failed, indicating sufficient entropy was produced by the analog components"]
    #[inline(always)]
    pub fn test_results_ana_test(&self) -> TestResultsAnaTestR {
        TestResultsAnaTestR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Test results from TEST_ANA and TEST_DIG\n\nYou can [`read`](crate::Reg::read) this register and get [`test_results::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestResultsSpec;
impl crate::RegisterSpec for TestResultsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test_results::R`](R) reader structure"]
impl crate::Readable for TestResultsSpec {}
#[doc = "`reset()` method sets TEST_RESULTS to value 0"]
impl crate::Resettable for TestResultsSpec {
    const RESET_VALUE: u32 = 0;
}
