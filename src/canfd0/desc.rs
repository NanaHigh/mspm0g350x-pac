#[doc = "Register `DESC` reader"]
pub type R = crate::R<DescSpec>;
#[doc = "Field `DESC_MINREV` reader - Minor rev of the IP"]
pub type DescMinrevR = crate::FieldReader;
#[doc = "Field `DESC_MAJREV` reader - Major rev of the IP"]
pub type DescMajrevR = crate::FieldReader;
#[doc = "Feature Set for the module *instance*\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DescFeaturever {
    #[doc = "0: VERSION_0"]
    DescFeatureverVersion0 = 0,
    #[doc = "1: VERSION_1"]
    DescFeatureverVersion1 = 1,
}
impl From<DescFeaturever> for u8 {
    #[inline(always)]
    fn from(variant: DescFeaturever) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DescFeaturever {
    type Ux = u8;
}
impl crate::IsEnum for DescFeaturever {}
#[doc = "Field `DESC_FEATUREVER` reader - Feature Set for the module *instance*"]
pub type DescFeatureverR = crate::FieldReader<DescFeaturever>;
impl DescFeatureverR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DescFeaturever> {
        match self.bits {
            0 => Some(DescFeaturever::DescFeatureverVersion0),
            1 => Some(DescFeaturever::DescFeatureverVersion1),
            _ => None,
        }
    }
    #[doc = "VERSION_0"]
    #[inline(always)]
    pub fn is_desc_featurever_version_0(&self) -> bool {
        *self == DescFeaturever::DescFeatureverVersion0
    }
    #[doc = "VERSION_1"]
    #[inline(always)]
    pub fn is_desc_featurever_version_1(&self) -> bool {
        *self == DescFeaturever::DescFeatureverVersion1
    }
}
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
