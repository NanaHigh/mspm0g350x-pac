#[doc = "Register `NMIRIS` reader"]
pub type R = crate::R<NmirisSpec>;
#[doc = "Raw status of the BORLVL NMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NmirisBorlvl {
    #[doc = "0: FALSE"]
    NmirisBorlvlFalse = 0,
    #[doc = "1: TRUE"]
    NmirisBorlvlTrue = 1,
}
impl From<NmirisBorlvl> for bool {
    #[inline(always)]
    fn from(variant: NmirisBorlvl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIRIS_BORLVL` reader - Raw status of the BORLVL NMI"]
pub type NmirisBorlvlR = crate::BitReader<NmirisBorlvl>;
impl NmirisBorlvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NmirisBorlvl {
        match self.bits {
            false => NmirisBorlvl::NmirisBorlvlFalse,
            true => NmirisBorlvl::NmirisBorlvlTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_nmiris_borlvl_false(&self) -> bool {
        *self == NmirisBorlvl::NmirisBorlvlFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_nmiris_borlvl_true(&self) -> bool {
        *self == NmirisBorlvl::NmirisBorlvlTrue
    }
}
#[doc = "Watch Dog 0 Fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NmirisWwdt0 {
    #[doc = "0: FALSE"]
    NmirisWwdt0False = 0,
    #[doc = "1: TRUE"]
    NmirisWwdt0True = 1,
}
impl From<NmirisWwdt0> for bool {
    #[inline(always)]
    fn from(variant: NmirisWwdt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIRIS_WWDT0` reader - Watch Dog 0 Fault"]
pub type NmirisWwdt0R = crate::BitReader<NmirisWwdt0>;
impl NmirisWwdt0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NmirisWwdt0 {
        match self.bits {
            false => NmirisWwdt0::NmirisWwdt0False,
            true => NmirisWwdt0::NmirisWwdt0True,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_nmiris_wwdt0_false(&self) -> bool {
        *self == NmirisWwdt0::NmirisWwdt0False
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_nmiris_wwdt0_true(&self) -> bool {
        *self == NmirisWwdt0::NmirisWwdt0True
    }
}
#[doc = "Watch Dog 0 Fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NmirisWwdt1 {
    #[doc = "0: FALSE"]
    NmirisWwdt1False = 0,
    #[doc = "1: TRUE"]
    NmirisWwdt1True = 1,
}
impl From<NmirisWwdt1> for bool {
    #[inline(always)]
    fn from(variant: NmirisWwdt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIRIS_WWDT1` reader - Watch Dog 0 Fault"]
pub type NmirisWwdt1R = crate::BitReader<NmirisWwdt1>;
impl NmirisWwdt1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NmirisWwdt1 {
        match self.bits {
            false => NmirisWwdt1::NmirisWwdt1False,
            true => NmirisWwdt1::NmirisWwdt1True,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_nmiris_wwdt1_false(&self) -> bool {
        *self == NmirisWwdt1::NmirisWwdt1False
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_nmiris_wwdt1_true(&self) -> bool {
        *self == NmirisWwdt1::NmirisWwdt1True
    }
}
#[doc = "LFXT-EXLF Monitor Fail\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NmirisLfclkfail {
    #[doc = "0: FALSE"]
    NmirisLfclkfailFalse = 0,
    #[doc = "1: TRUE"]
    NmirisLfclkfailTrue = 1,
}
impl From<NmirisLfclkfail> for bool {
    #[inline(always)]
    fn from(variant: NmirisLfclkfail) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIRIS_LFCLKFAIL` reader - LFXT-EXLF Monitor Fail"]
pub type NmirisLfclkfailR = crate::BitReader<NmirisLfclkfail>;
impl NmirisLfclkfailR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NmirisLfclkfail {
        match self.bits {
            false => NmirisLfclkfail::NmirisLfclkfailFalse,
            true => NmirisLfclkfail::NmirisLfclkfailTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_nmiris_lfclkfail_false(&self) -> bool {
        *self == NmirisLfclkfail::NmirisLfclkfailFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_nmiris_lfclkfail_true(&self) -> bool {
        *self == NmirisLfclkfail::NmirisLfclkfailTrue
    }
}
#[doc = "Flash Double Error Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NmirisFlashded {
    #[doc = "0: FALSE"]
    NmirisFlashdedFalse = 0,
    #[doc = "1: TRUE"]
    NmirisFlashdedTrue = 1,
}
impl From<NmirisFlashded> for bool {
    #[inline(always)]
    fn from(variant: NmirisFlashded) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIRIS_FLASHDED` reader - Flash Double Error Detect"]
pub type NmirisFlashdedR = crate::BitReader<NmirisFlashded>;
impl NmirisFlashdedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NmirisFlashded {
        match self.bits {
            false => NmirisFlashded::NmirisFlashdedFalse,
            true => NmirisFlashded::NmirisFlashdedTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_nmiris_flashded_false(&self) -> bool {
        *self == NmirisFlashded::NmirisFlashdedFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_nmiris_flashded_true(&self) -> bool {
        *self == NmirisFlashded::NmirisFlashdedTrue
    }
}
#[doc = "SRAM Double Error Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NmirisSramded {
    #[doc = "0: FALSE"]
    NmirisSramdedFalse = 0,
    #[doc = "1: TRUE"]
    NmirisSramdedTrue = 1,
}
impl From<NmirisSramded> for bool {
    #[inline(always)]
    fn from(variant: NmirisSramded) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIRIS_SRAMDED` reader - SRAM Double Error Detect"]
pub type NmirisSramdedR = crate::BitReader<NmirisSramded>;
impl NmirisSramdedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NmirisSramded {
        match self.bits {
            false => NmirisSramded::NmirisSramdedFalse,
            true => NmirisSramded::NmirisSramdedTrue,
        }
    }
    #[doc = "FALSE"]
    #[inline(always)]
    pub fn is_nmiris_sramded_false(&self) -> bool {
        *self == NmirisSramded::NmirisSramdedFalse
    }
    #[doc = "TRUE"]
    #[inline(always)]
    pub fn is_nmiris_sramded_true(&self) -> bool {
        *self == NmirisSramded::NmirisSramdedTrue
    }
}
impl R {
    #[doc = "Bit 0 - Raw status of the BORLVL NMI"]
    #[inline(always)]
    pub fn nmiris_borlvl(&self) -> NmirisBorlvlR {
        NmirisBorlvlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watch Dog 0 Fault"]
    #[inline(always)]
    pub fn nmiris_wwdt0(&self) -> NmirisWwdt0R {
        NmirisWwdt0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watch Dog 0 Fault"]
    #[inline(always)]
    pub fn nmiris_wwdt1(&self) -> NmirisWwdt1R {
        NmirisWwdt1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LFXT-EXLF Monitor Fail"]
    #[inline(always)]
    pub fn nmiris_lfclkfail(&self) -> NmirisLfclkfailR {
        NmirisLfclkfailR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash Double Error Detect"]
    #[inline(always)]
    pub fn nmiris_flashded(&self) -> NmirisFlashdedR {
        NmirisFlashdedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SRAM Double Error Detect"]
    #[inline(always)]
    pub fn nmiris_sramded(&self) -> NmirisSramdedR {
        NmirisSramdedR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "NMI raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`nmiris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmirisSpec;
impl crate::RegisterSpec for NmirisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmiris::R`](R) reader structure"]
impl crate::Readable for NmirisSpec {}
#[doc = "`reset()` method sets NMIRIS to value 0"]
impl crate::Resettable for NmirisSpec {
    const RESET_VALUE: u32 = 0;
}
