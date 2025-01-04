#[doc = "Register `SUB1CFG` reader"]
pub type R = crate::R<Sub1cfgSpec>;
#[doc = "Register `SUB1CFG` writer"]
pub type W = crate::W<Sub1cfgSpec>;
#[doc = "This bit is used to enable subscriber 1 event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sub1cfgEnable {
    #[doc = "0: CLR"]
    Sub1cfgEnableClr = 0,
    #[doc = "1: SET"]
    Sub1cfgEnableSet = 1,
}
impl From<Sub1cfgEnable> for bool {
    #[inline(always)]
    fn from(variant: Sub1cfgEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUB1CFG_ENABLE` reader - This bit is used to enable subscriber 1 event."]
pub type Sub1cfgEnableR = crate::BitReader<Sub1cfgEnable>;
impl Sub1cfgEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sub1cfgEnable {
        match self.bits {
            false => Sub1cfgEnable::Sub1cfgEnableClr,
            true => Sub1cfgEnable::Sub1cfgEnableSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_sub1cfg_enable_clr(&self) -> bool {
        *self == Sub1cfgEnable::Sub1cfgEnableClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_sub1cfg_enable_set(&self) -> bool {
        *self == Sub1cfgEnable::Sub1cfgEnableSet
    }
}
#[doc = "Field `SUB1CFG_ENABLE` writer - This bit is used to enable subscriber 1 event."]
pub type Sub1cfgEnableW<'a, REG> = crate::BitWriter<'a, REG, Sub1cfgEnable>;
impl<'a, REG> Sub1cfgEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn sub1cfg_enable_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Sub1cfgEnable::Sub1cfgEnableClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn sub1cfg_enable_set(self) -> &'a mut crate::W<REG> {
        self.variant(Sub1cfgEnable::Sub1cfgEnableSet)
    }
}
#[doc = "These bits configure the output policy for subscriber 1 event.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sub1cfgOutpolicy {
    #[doc = "0: SET"]
    Sub1cfgOutpolicySet = 0,
    #[doc = "1: CLR"]
    Sub1cfgOutpolicyClr = 1,
    #[doc = "2: TOGGLE"]
    Sub1cfgOutpolicyToggle = 2,
}
impl From<Sub1cfgOutpolicy> for u8 {
    #[inline(always)]
    fn from(variant: Sub1cfgOutpolicy) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sub1cfgOutpolicy {
    type Ux = u8;
}
impl crate::IsEnum for Sub1cfgOutpolicy {}
#[doc = "Field `SUB1CFG_OUTPOLICY` reader - These bits configure the output policy for subscriber 1 event."]
pub type Sub1cfgOutpolicyR = crate::FieldReader<Sub1cfgOutpolicy>;
impl Sub1cfgOutpolicyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sub1cfgOutpolicy> {
        match self.bits {
            0 => Some(Sub1cfgOutpolicy::Sub1cfgOutpolicySet),
            1 => Some(Sub1cfgOutpolicy::Sub1cfgOutpolicyClr),
            2 => Some(Sub1cfgOutpolicy::Sub1cfgOutpolicyToggle),
            _ => None,
        }
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_sub1cfg_outpolicy_set(&self) -> bool {
        *self == Sub1cfgOutpolicy::Sub1cfgOutpolicySet
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_sub1cfg_outpolicy_clr(&self) -> bool {
        *self == Sub1cfgOutpolicy::Sub1cfgOutpolicyClr
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn is_sub1cfg_outpolicy_toggle(&self) -> bool {
        *self == Sub1cfgOutpolicy::Sub1cfgOutpolicyToggle
    }
}
#[doc = "Field `SUB1CFG_OUTPOLICY` writer - These bits configure the output policy for subscriber 1 event."]
pub type Sub1cfgOutpolicyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Sub1cfgOutpolicy>;
impl<'a, REG> Sub1cfgOutpolicyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SET"]
    #[inline(always)]
    pub fn sub1cfg_outpolicy_set(self) -> &'a mut crate::W<REG> {
        self.variant(Sub1cfgOutpolicy::Sub1cfgOutpolicySet)
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn sub1cfg_outpolicy_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Sub1cfgOutpolicy::Sub1cfgOutpolicyClr)
    }
    #[doc = "TOGGLE"]
    #[inline(always)]
    pub fn sub1cfg_outpolicy_toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Sub1cfgOutpolicy::Sub1cfgOutpolicyToggle)
    }
}
#[doc = "indicates the specific bit in the upper 16 bits that is targeted by the subscriber action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sub1cfgIndex {
    #[doc = "0: MIN"]
    Sub1cfgIndexMin = 0,
    #[doc = "15: MAX"]
    Sub1cfgIndexMax = 15,
}
impl From<Sub1cfgIndex> for u8 {
    #[inline(always)]
    fn from(variant: Sub1cfgIndex) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sub1cfgIndex {
    type Ux = u8;
}
impl crate::IsEnum for Sub1cfgIndex {}
#[doc = "Field `SUB1CFG_INDEX` reader - indicates the specific bit in the upper 16 bits that is targeted by the subscriber action"]
pub type Sub1cfgIndexR = crate::FieldReader<Sub1cfgIndex>;
impl Sub1cfgIndexR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sub1cfgIndex> {
        match self.bits {
            0 => Some(Sub1cfgIndex::Sub1cfgIndexMin),
            15 => Some(Sub1cfgIndex::Sub1cfgIndexMax),
            _ => None,
        }
    }
    #[doc = "MIN"]
    #[inline(always)]
    pub fn is_sub1cfg_index_min(&self) -> bool {
        *self == Sub1cfgIndex::Sub1cfgIndexMin
    }
    #[doc = "MAX"]
    #[inline(always)]
    pub fn is_sub1cfg_index_max(&self) -> bool {
        *self == Sub1cfgIndex::Sub1cfgIndexMax
    }
}
#[doc = "Field `SUB1CFG_INDEX` writer - indicates the specific bit in the upper 16 bits that is targeted by the subscriber action"]
pub type Sub1cfgIndexW<'a, REG> = crate::FieldWriter<'a, REG, 4, Sub1cfgIndex>;
impl<'a, REG> Sub1cfgIndexW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "MIN"]
    #[inline(always)]
    pub fn sub1cfg_index_min(self) -> &'a mut crate::W<REG> {
        self.variant(Sub1cfgIndex::Sub1cfgIndexMin)
    }
    #[doc = "MAX"]
    #[inline(always)]
    pub fn sub1cfg_index_max(self) -> &'a mut crate::W<REG> {
        self.variant(Sub1cfgIndex::Sub1cfgIndexMax)
    }
}
impl R {
    #[doc = "Bit 0 - This bit is used to enable subscriber 1 event."]
    #[inline(always)]
    pub fn sub1cfg_enable(&self) -> Sub1cfgEnableR {
        Sub1cfgEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - These bits configure the output policy for subscriber 1 event."]
    #[inline(always)]
    pub fn sub1cfg_outpolicy(&self) -> Sub1cfgOutpolicyR {
        Sub1cfgOutpolicyR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:19 - indicates the specific bit in the upper 16 bits that is targeted by the subscriber action"]
    #[inline(always)]
    pub fn sub1cfg_index(&self) -> Sub1cfgIndexR {
        Sub1cfgIndexR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to enable subscriber 1 event."]
    #[inline(always)]
    pub fn sub1cfg_enable(&mut self) -> Sub1cfgEnableW<Sub1cfgSpec> {
        Sub1cfgEnableW::new(self, 0)
    }
    #[doc = "Bits 8:9 - These bits configure the output policy for subscriber 1 event."]
    #[inline(always)]
    pub fn sub1cfg_outpolicy(&mut self) -> Sub1cfgOutpolicyW<Sub1cfgSpec> {
        Sub1cfgOutpolicyW::new(self, 8)
    }
    #[doc = "Bits 16:19 - indicates the specific bit in the upper 16 bits that is targeted by the subscriber action"]
    #[inline(always)]
    pub fn sub1cfg_index(&mut self) -> Sub1cfgIndexW<Sub1cfgSpec> {
        Sub1cfgIndexW::new(self, 16)
    }
}
#[doc = "Subscriber 1 configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sub1cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sub1cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sub1cfgSpec;
impl crate::RegisterSpec for Sub1cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sub1cfg::R`](R) reader structure"]
impl crate::Readable for Sub1cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`sub1cfg::W`](W) writer structure"]
impl crate::Writable for Sub1cfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SUB1CFG to value 0"]
impl crate::Resettable for Sub1cfgSpec {
    const RESET_VALUE: u32 = 0;
}
