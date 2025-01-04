#[doc = "Register `MCANSS_PID` reader"]
pub type R = crate::R<McanssPidSpec>;
#[doc = "Field `MCANSS_PID_MINOR` reader - Minor Revision of the MCAN Subsystem"]
pub type McanssPidMinorR = crate::FieldReader;
#[doc = "Field `MCANSS_PID_MAJOR` reader - Major Revision of the MCAN Subsystem"]
pub type McanssPidMajorR = crate::FieldReader;
#[doc = "Field `MCANSS_PID_MODULE_ID` reader - Module Identification Number"]
pub type McanssPidModuleIdR = crate::FieldReader<u16>;
#[doc = "Field `MCANSS_PID_SCHEME` reader - PID Register Scheme"]
pub type McanssPidSchemeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Minor Revision of the MCAN Subsystem"]
    #[inline(always)]
    pub fn mcanss_pid_minor(&self) -> McanssPidMinorR {
        McanssPidMinorR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:10 - Major Revision of the MCAN Subsystem"]
    #[inline(always)]
    pub fn mcanss_pid_major(&self) -> McanssPidMajorR {
        McanssPidMajorR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:27 - Module Identification Number"]
    #[inline(always)]
    pub fn mcanss_pid_module_id(&self) -> McanssPidModuleIdR {
        McanssPidModuleIdR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 30:31 - PID Register Scheme"]
    #[inline(always)]
    pub fn mcanss_pid_scheme(&self) -> McanssPidSchemeR {
        McanssPidSchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
#[doc = "MCAN Subsystem Revision Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_pid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanssPidSpec;
impl crate::RegisterSpec for McanssPidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanss_pid::R`](R) reader structure"]
impl crate::Readable for McanssPidSpec {}
#[doc = "`reset()` method sets MCANSS_PID to value 0x68e0_4901"]
impl crate::Resettable for McanssPidSpec {
    const RESET_VALUE: u32 = 0x68e0_4901;
}
