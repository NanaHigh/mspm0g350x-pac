#[doc = "Register `DESC` reader"]
pub type R = crate::R<DescSpec>;
#[doc = "Field `DESC_MINREV` reader - Minor rev of the IP"]
pub type DescMinrevR = crate::FieldReader;
#[doc = "Field `DESC_MAJREV` reader - Major rev of the IP"]
pub type DescMajrevR = crate::FieldReader;
#[doc = "Field `DESC_FEATUREVER` reader - Feature Set for the module *instance*"]
pub type DescFeatureverR = crate::FieldReader;
#[doc = "Field `DESC_MODULEID` reader - Module identification contains a unique peripheral identification number. The assignments are maintained in a central database for all of the platform modules to ensure uniqueness."]
pub type DescModuleidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Minor rev of the IP"]
    #[inline(always)]
    pub fn desc_minrev(&self) -> DescMinrevR {
        DescMinrevR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Major rev of the IP"]
    #[inline(always)]
    pub fn desc_majrev(&self) -> DescMajrevR {
        DescMajrevR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Feature Set for the module *instance*"]
    #[inline(always)]
    pub fn desc_featurever(&self) -> DescFeatureverR {
        DescFeatureverR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Module identification contains a unique peripheral identification number. The assignments are maintained in a central database for all of the platform modules to ensure uniqueness."]
    #[inline(always)]
    pub fn desc_moduleid(&self) -> DescModuleidR {
        DescModuleidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Module Description\n\nYou can [`read`](crate::Reg::read) this register and get [`desc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescSpec;
impl crate::RegisterSpec for DescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`desc::R`](R) reader structure"]
impl crate::Readable for DescSpec {}
#[doc = "`reset()` method sets DESC to value 0"]
impl crate::Resettable for DescSpec {
    const RESET_VALUE: u32 = 0;
}
