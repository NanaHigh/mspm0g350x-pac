#[doc = "Register `MCANERR_WRAP_REV` reader"]
pub type R = crate::R<McanerrWrapRevSpec>;
#[doc = "Field `MCANERR_WRAP_REV_REVMIN` reader - Minor Revision of the Error Aggregator"]
pub type McanerrWrapRevRevminR = crate::FieldReader;
#[doc = "Field `MCANERR_WRAP_REV_REVMAJ` reader - Major Revision of the Error Aggregator"]
pub type McanerrWrapRevRevmajR = crate::FieldReader;
#[doc = "Field `MCANERR_WRAP_REV_MODULE_ID` reader - Module Identification Number"]
pub type McanerrWrapRevModuleIdR = crate::FieldReader<u16>;
#[doc = "Field `MCANERR_WRAP_REV_SCHEME` reader - PID Register Scheme"]
pub type McanerrWrapRevSchemeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Minor Revision of the Error Aggregator"]
    #[inline(always)]
    pub fn mcanerr_wrap_rev_revmin(&self) -> McanerrWrapRevRevminR {
        McanerrWrapRevRevminR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:10 - Major Revision of the Error Aggregator"]
    #[inline(always)]
    pub fn mcanerr_wrap_rev_revmaj(&self) -> McanerrWrapRevRevmajR {
        McanerrWrapRevRevmajR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:27 - Module Identification Number"]
    #[inline(always)]
    pub fn mcanerr_wrap_rev_module_id(&self) -> McanerrWrapRevModuleIdR {
        McanerrWrapRevModuleIdR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 30:31 - PID Register Scheme"]
    #[inline(always)]
    pub fn mcanerr_wrap_rev_scheme(&self) -> McanerrWrapRevSchemeR {
        McanerrWrapRevSchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "MCAN ECC Wrapper Revision Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_wrap_rev::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanerrWrapRevSpec;
impl crate::RegisterSpec for McanerrWrapRevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanerr_wrap_rev::R`](R) reader structure"]
impl crate::Readable for McanerrWrapRevSpec {}
#[doc = "`reset()` method sets MCANERR_WRAP_REV to value 0x66a4_6a02"]
impl crate::Resettable for McanerrWrapRevSpec {
    const RESET_VALUE: u32 = 0x66a4_6a02;
}
