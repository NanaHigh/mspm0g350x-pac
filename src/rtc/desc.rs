#[doc = "Register `DESC` reader"]
pub type R = crate::R<DescSpec>;
#[doc = "Field `DESC_MINREV` reader - Minor revision. This number holds the module revision and is incremented by the module developers. n = Minor module revision (see device-specific data sheet)"]
pub type DescMinrevR = crate::FieldReader;
#[doc = "Field `DESC_MAJREV` reader - Major revision. This number holds the module revision and is incremented by the module developers. n = Major version (see device-specific data sheet)"]
pub type DescMajrevR = crate::FieldReader;
#[doc = "Instantiated version. Describes which instance of the module accessed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DescInstnum {
    #[doc = "0: INST0"]
    DescInstnumInst0 = 0,
}
impl From<DescInstnum> for u8 {
    #[inline(always)]
    fn from(variant: DescInstnum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DescInstnum {
    type Ux = u8;
}
impl crate::IsEnum for DescInstnum {}
#[doc = "Field `DESC_INSTNUM` reader - Instantiated version. Describes which instance of the module accessed."]
pub type DescInstnumR = crate::FieldReader<DescInstnum>;
impl DescInstnumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DescInstnum> {
        match self.bits {
            0 => Some(DescInstnum::DescInstnumInst0),
            _ => None,
        }
    }
    #[doc = "INST0"]
    #[inline(always)]
    pub fn is_desc_instnum_inst0(&self) -> bool {
        *self == DescInstnum::DescInstnumInst0
    }
}
#[doc = "Field `DESC_FEATUREVER` reader - Feature set of this module. Differentiates the complexity of the actually instantiated module if there are differences."]
pub type DescFeatureverR = crate::FieldReader;
#[doc = "Field `DESC_MODULEID` reader - Module identifier. This ID is unique for each module. 0x0911 = Module ID of the RTC Module"]
pub type DescModuleidR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Minor revision. This number holds the module revision and is incremented by the module developers. n = Minor module revision (see device-specific data sheet)"]
    #[inline(always)]
    pub fn desc_minrev(&self) -> DescMinrevR {
        DescMinrevR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Major revision. This number holds the module revision and is incremented by the module developers. n = Major version (see device-specific data sheet)"]
    #[inline(always)]
    pub fn desc_majrev(&self) -> DescMajrevR {
        DescMajrevR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Instantiated version. Describes which instance of the module accessed."]
    #[inline(always)]
    pub fn desc_instnum(&self) -> DescInstnumR {
        DescInstnumR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Feature set of this module. Differentiates the complexity of the actually instantiated module if there are differences."]
    #[inline(always)]
    pub fn desc_featurever(&self) -> DescFeatureverR {
        DescFeatureverR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - Module identifier. This ID is unique for each module. 0x0911 = Module ID of the RTC Module"]
    #[inline(always)]
    pub fn desc_moduleid(&self) -> DescModuleidR {
        DescModuleidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "RTC Descriptor Register\n\nYou can [`read`](crate::Reg::read) this register and get [`desc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DescSpec;
impl crate::RegisterSpec for DescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`desc::R`](R) reader structure"]
impl crate::Readable for DescSpec {}
#[doc = "`reset()` method sets DESC to value 0x0911_8010"]
impl crate::Resettable for DescSpec {
    const RESET_VALUE: u32 = 0x0911_8010;
}
