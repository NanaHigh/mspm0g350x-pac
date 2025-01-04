#[doc = "Register `MCANERR_REV` reader"]
pub type R = crate::R<McanerrRevSpec>;
#[doc = "Field `MCANERR_REV_REVMIN` reader - Minor Revision of the Error Aggregator"]
pub type McanerrRevRevminR = crate::FieldReader;
#[doc = "Field `MCANERR_REV_REVMAJ` reader - Major Revision of the Error Aggregator"]
pub type McanerrRevRevmajR = crate::FieldReader;
#[doc = "Field `MCANERR_REV_MODULE_ID` reader - Module Identification Number"]
pub type McanerrRevModuleIdR = crate::FieldReader<u16>;
#[doc = "Field `MCANERR_REV_SCHEME` reader - PID Register Scheme"]
pub type McanerrRevSchemeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Minor Revision of the Error Aggregator"]
    #[inline(always)]
    pub fn mcanerr_rev_revmin(&self) -> McanerrRevRevminR {
        McanerrRevRevminR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:10 - Major Revision of the Error Aggregator"]
    #[inline(always)]
    pub fn mcanerr_rev_revmaj(&self) -> McanerrRevRevmajR {
        McanerrRevRevmajR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:27 - Module Identification Number"]
    #[inline(always)]
    pub fn mcanerr_rev_module_id(&self) -> McanerrRevModuleIdR {
        McanerrRevModuleIdR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 30:31 - PID Register Scheme"]
    #[inline(always)]
    pub fn mcanerr_rev_scheme(&self) -> McanerrRevSchemeR {
        McanerrRevSchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "MCAN Error Aggregator Revision Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanerr_rev::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanerrRevSpec;
impl crate::RegisterSpec for McanerrRevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanerr_rev::R`](R) reader structure"]
impl crate::Readable for McanerrRevSpec {}
#[doc = "`reset()` method sets MCANERR_REV to value 0x66a0_ea00"]
impl crate::Resettable for McanerrRevSpec {
    const RESET_VALUE: u32 = 0x66a0_ea00;
}
