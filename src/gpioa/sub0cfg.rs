#[doc = "Register `SUB0CFG` reader"]
pub type R = crate::R<Sub0cfgSpec>;
#[doc = "Register `SUB0CFG` writer"]
pub type W = crate::W<Sub0cfgSpec>;
#[doc = "This bit is used to enable subscriber 0 event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sub0cfgEnable {
    #[doc = "0: CLR"]
    Sub0cfgEnableClr = 0,
    #[doc = "1: SET"]
    Sub0cfgEnableSet = 1,
}
impl From<Sub0cfgEnable> for bool {
    #[inline(always)]
    fn from(variant: Sub0cfgEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUB0CFG_ENABLE` reader - This bit is used to enable subscriber 0 event."]
pub type Sub0cfgEnableR = crate::BitReader<Sub0cfgEnable>;
impl Sub0cfgEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sub0cfgEnable {
        match self.bits {
            false => Sub0cfgEnable::Sub0cfgEnableClr,
            true => Sub0cfgEnable::Sub0cfgEnableSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_sub0cfg_enable_clr(&self) -> bool {
        *self == Sub0cfgEnable::Sub0cfgEnableClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_sub0cfg_enable_set(&self) -> bool {
        *self == Sub0cfgEnable::Sub0cfgEnableSet
    }
}
#[doc = "Field `SUB0CFG_ENABLE` writer - This bit is used to enable subscriber 0 event."]
pub type Sub0cfgEnableW<'a, REG> = crate::BitWriter<'a, REG, Sub0cfgEnable>;
impl<'a, REG> Sub0cfgEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn sub0cfg_enable_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Sub0cfgEnable::Sub0cfgEnableClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn sub0cfg_enable_set(self) -> &'a mut crate::W<REG> {
        self.variant(Sub0cfgEnable::Sub0cfgEnableSet)
    }
}
#[doc = "These bits configure the output policy for subscriber 0 event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sub0cfgOutpolicy {
    #[doc = "0: SET"]
    Sub0cfgOutpolicySet = 0,
    #[doc = "1: CLR"]
    Sub0cfgOutpolicyClr = 1,
    #[doc = "2: TOGGLE"]
    Sub0cfgOutpolicyToggle = 2,
}
impl From<Sub0cfgOutpolicy> for u8 {
    #[inline(always)]
    fn from(variant: Sub0cfgOutpolicy) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sub0cfgOutpolicy {
    type Ux = u8;
}
impl crate::IsEnum for Sub0cfgOutpolicy {}
#[doc = "Field `SUB0CFG_OUTPOLICY` reader - These bits configure the output policy for subscriber 0 event."]
pub type Sub0cfgOutpolicyR = crate::FieldReader<Sub0cfgOutpolicy>;
impl Sub0cfgOutpolicyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sub0cfgOutpolicy> {
        match self.bits {
            0 => Some(Sub0cfgOutpolicy::Sub0cfgOutpolicySet),
            1 => Some(Sub0cfgOutpolicy::Sub0cfgOutpolicyClr),
            2 => Some(Sub0cfgOutpolicy::Sub0cfgOutpolicyToggle),
            _ => None,
        }
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_sub0cfg_outpolicy_set(&self) -> bool {
        *self == Sub0cfgOutpolicy::Sub0cfgOutpolicySet
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_sub0cfg_outpolicy_clr(&self) -> bool {
        *self == Sub0cfgOutpolicy::Sub0cfgOutpolicyClr
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn is_sub0cfg_outpolicy_toggle(&self) -> bool {
        *self == Sub0cfgOutpolicy::Sub0cfgOutpolicyToggle
    }
}
#[doc = "Field `SUB0CFG_OUTPOLICY` writer - These bits configure the output policy for subscriber 0 event."]
pub type Sub0cfgOutpolicyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sub0cfgOutpolicy>;
impl<'a, REG> Sub0cfgOutpolicyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SET"]
    #[inline(always)]
    pub fn sub0cfg_outpolicy_set(self) -> &'a mut crate::W<REG> {
        self.variant(Sub0cfgOutpolicy::Sub0cfgOutpolicySet)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn sub0cfg_outpolicy_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Sub0cfgOutpolicy::Sub0cfgOutpolicyClr)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn sub0cfg_outpolicy_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Sub0cfgOutpolicy::Sub0cfgOutpolicyToggle)
    }
}
#[doc = "Indicates the specific bit among lower 16 bits that is targeted by the subscriber action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sub0cfgIndex {
    #[doc = "0: MIN"]
    Sub0cfgIndexMin = 0,
    #[doc = "15: MAX"]
    Sub0cfgIndexMax = 15,
}
impl From<Sub0cfgIndex> for u8 {
    #[inline(always)]
    fn from(variant: Sub0cfgIndex) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sub0cfgIndex {
    type Ux = u8;
}
impl crate::IsEnum for Sub0cfgIndex {}
#[doc = "Field `SUB0CFG_INDEX` reader - Indicates the specific bit among lower 16 bits that is targeted by the subscriber action"]
pub type Sub0cfgIndexR = crate::FieldReader<Sub0cfgIndex>;
impl Sub0cfgIndexR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sub0cfgIndex> {
        match self.bits {
            0 => Some(Sub0cfgIndex::Sub0cfgIndexMin),
            15 => Some(Sub0cfgIndex::Sub0cfgIndexMax),
            _ => None,
        }
    }
    #[doc = "MIN"]
    #[inline(always)]
    pub fn is_sub0cfg_index_min(&self) -> bool {
        *self == Sub0cfgIndex::Sub0cfgIndexMin
    }
    #[doc = "MAX"]
    #[inline(always)]
    pub fn is_sub0cfg_index_max(&self) -> bool {
        *self == Sub0cfgIndex::Sub0cfgIndexMax
    }
}
#[doc = "Field `SUB0CFG_INDEX` writer - Indicates the specific bit among lower 16 bits that is targeted by the subscriber action"]
pub type Sub0cfgIndexW<'a, REG> = crate::FieldWriter<'a, REG, 4, Sub0cfgIndex>;
impl<'a, REG> Sub0cfgIndexW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MIN"]
    #[inline(always)]
    pub fn sub0cfg_index_min(self) -> &'a mut crate::W<REG> {
        self.variant(Sub0cfgIndex::Sub0cfgIndexMin)
    }
    #[doc = "MAX"]
    #[inline(always)]
    pub fn sub0cfg_index_max(self) -> &'a mut crate::W<REG> {
        self.variant(Sub0cfgIndex::Sub0cfgIndexMax)
    }
}
impl R {
    #[doc = "Bit 0 - This bit is used to enable subscriber 0 event."]
    #[inline(always)]
    pub fn sub0cfg_enable(&self) -> Sub0cfgEnableR {
        Sub0cfgEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - These bits configure the output policy for subscriber 0 event."]
    #[inline(always)]
    pub fn sub0cfg_outpolicy(&self) -> Sub0cfgOutpolicyR {
        Sub0cfgOutpolicyR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Indicates the specific bit among lower 16 bits that is targeted by the subscriber action"]
    #[inline(always)]
    pub fn sub0cfg_index(&self) -> Sub0cfgIndexR {
        Sub0cfgIndexR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to enable subscriber 0 event."]
    #[inline(always)]
    pub fn sub0cfg_enable(&mut self) -> Sub0cfgEnableW<Sub0cfgSpec> {
        Sub0cfgEnableW::new(self, 0)
    }
    #[doc = "Bits 8:9 - These bits configure the output policy for subscriber 0 event."]
    #[inline(always)]
    pub fn sub0cfg_outpolicy(&mut self) -> Sub0cfgOutpolicyW<Sub0cfgSpec> {
        Sub0cfgOutpolicyW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Indicates the specific bit among lower 16 bits that is targeted by the subscriber action"]
    #[inline(always)]
    pub fn sub0cfg_index(&mut self) -> Sub0cfgIndexW<Sub0cfgSpec> {
        Sub0cfgIndexW::new(self, 16)
    }
}
#[doc = "Subscriber 0 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sub0cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sub0cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sub0cfgSpec;
impl crate::RegisterSpec for Sub0cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sub0cfg::R`](R) reader structure"]
impl crate::Readable for Sub0cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sub0cfg::W`](W) writer structure"]
impl crate::Writable for Sub0cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUB0CFG to value 0"]
impl crate::Resettable for Sub0cfgSpec {
    const RESET_VALUE: u32 = 0;
}
