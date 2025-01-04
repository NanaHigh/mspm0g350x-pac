#[doc = "Register `INT_EVENT2_IMASK` reader"]
pub type R = crate::R<IntEvent2ImaskSpec>;
#[doc = "Register `INT_EVENT2_IMASK` writer"]
pub type W = crate::W<IntEvent2ImaskSpec>;
#[doc = "DIO16 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskDio16 {
    #[doc = "0: CLR"]
    IntEvent2ImaskDio16Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskDio16Set = 1,
}
impl From<IntEvent2ImaskDio16> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskDio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO16` reader - DIO16 event mask"]
pub type IntEvent2ImaskDio16R = crate::BitReader<IntEvent2ImaskDio16>;
impl IntEvent2ImaskDio16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskDio16 {
        match self.bits {
            false => IntEvent2ImaskDio16::IntEvent2ImaskDio16Clr,
            true => IntEvent2ImaskDio16::IntEvent2ImaskDio16Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio16_clr(&self) -> bool {
        *self == IntEvent2ImaskDio16::IntEvent2ImaskDio16Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio16_set(&self) -> bool {
        *self == IntEvent2ImaskDio16::IntEvent2ImaskDio16Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO16` writer - DIO16 event mask"]
pub type IntEvent2ImaskDio16W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskDio16>;
impl<'a, REG> IntEvent2ImaskDio16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_dio16_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio16::IntEvent2ImaskDio16Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_dio16_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio16::IntEvent2ImaskDio16Set)
    }
}
#[doc = "DIO17 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskDio17 {
    #[doc = "0: CLR"]
    IntEvent2ImaskDio17Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskDio17Set = 1,
}
impl From<IntEvent2ImaskDio17> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskDio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO17` reader - DIO17 event mask"]
pub type IntEvent2ImaskDio17R = crate::BitReader<IntEvent2ImaskDio17>;
impl IntEvent2ImaskDio17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskDio17 {
        match self.bits {
            false => IntEvent2ImaskDio17::IntEvent2ImaskDio17Clr,
            true => IntEvent2ImaskDio17::IntEvent2ImaskDio17Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio17_clr(&self) -> bool {
        *self == IntEvent2ImaskDio17::IntEvent2ImaskDio17Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio17_set(&self) -> bool {
        *self == IntEvent2ImaskDio17::IntEvent2ImaskDio17Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO17` writer - DIO17 event mask"]
pub type IntEvent2ImaskDio17W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskDio17>;
impl<'a, REG> IntEvent2ImaskDio17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_dio17_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio17::IntEvent2ImaskDio17Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_dio17_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio17::IntEvent2ImaskDio17Set)
    }
}
#[doc = "DIO18 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskDio18 {
    #[doc = "0: CLR"]
    IntEvent2ImaskDio18Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskDio18Set = 1,
}
impl From<IntEvent2ImaskDio18> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskDio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO18` reader - DIO18 event mask"]
pub type IntEvent2ImaskDio18R = crate::BitReader<IntEvent2ImaskDio18>;
impl IntEvent2ImaskDio18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskDio18 {
        match self.bits {
            false => IntEvent2ImaskDio18::IntEvent2ImaskDio18Clr,
            true => IntEvent2ImaskDio18::IntEvent2ImaskDio18Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio18_clr(&self) -> bool {
        *self == IntEvent2ImaskDio18::IntEvent2ImaskDio18Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio18_set(&self) -> bool {
        *self == IntEvent2ImaskDio18::IntEvent2ImaskDio18Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO18` writer - DIO18 event mask"]
pub type IntEvent2ImaskDio18W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskDio18>;
impl<'a, REG> IntEvent2ImaskDio18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_dio18_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio18::IntEvent2ImaskDio18Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_dio18_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio18::IntEvent2ImaskDio18Set)
    }
}
#[doc = "DIO19 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskDio19 {
    #[doc = "0: CLR"]
    IntEvent2ImaskDio19Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskDio19Set = 1,
}
impl From<IntEvent2ImaskDio19> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskDio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO19` reader - DIO19 event mask"]
pub type IntEvent2ImaskDio19R = crate::BitReader<IntEvent2ImaskDio19>;
impl IntEvent2ImaskDio19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskDio19 {
        match self.bits {
            false => IntEvent2ImaskDio19::IntEvent2ImaskDio19Clr,
            true => IntEvent2ImaskDio19::IntEvent2ImaskDio19Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio19_clr(&self) -> bool {
        *self == IntEvent2ImaskDio19::IntEvent2ImaskDio19Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio19_set(&self) -> bool {
        *self == IntEvent2ImaskDio19::IntEvent2ImaskDio19Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO19` writer - DIO19 event mask"]
pub type IntEvent2ImaskDio19W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskDio19>;
impl<'a, REG> IntEvent2ImaskDio19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_dio19_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio19::IntEvent2ImaskDio19Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_dio19_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio19::IntEvent2ImaskDio19Set)
    }
}
#[doc = "DIO20 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskDio20 {
    #[doc = "0: CLR"]
    IntEvent2ImaskDio20Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskDio20Set = 1,
}
impl From<IntEvent2ImaskDio20> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskDio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO20` reader - DIO20 event mask"]
pub type IntEvent2ImaskDio20R = crate::BitReader<IntEvent2ImaskDio20>;
impl IntEvent2ImaskDio20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskDio20 {
        match self.bits {
            false => IntEvent2ImaskDio20::IntEvent2ImaskDio20Clr,
            true => IntEvent2ImaskDio20::IntEvent2ImaskDio20Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio20_clr(&self) -> bool {
        *self == IntEvent2ImaskDio20::IntEvent2ImaskDio20Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio20_set(&self) -> bool {
        *self == IntEvent2ImaskDio20::IntEvent2ImaskDio20Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO20` writer - DIO20 event mask"]
pub type IntEvent2ImaskDio20W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskDio20>;
impl<'a, REG> IntEvent2ImaskDio20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_dio20_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio20::IntEvent2ImaskDio20Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_dio20_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio20::IntEvent2ImaskDio20Set)
    }
}
#[doc = "DIO21 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskDio21 {
    #[doc = "0: CLR"]
    IntEvent2ImaskDio21Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskDio21Set = 1,
}
impl From<IntEvent2ImaskDio21> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskDio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO21` reader - DIO21 event mask"]
pub type IntEvent2ImaskDio21R = crate::BitReader<IntEvent2ImaskDio21>;
impl IntEvent2ImaskDio21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskDio21 {
        match self.bits {
            false => IntEvent2ImaskDio21::IntEvent2ImaskDio21Clr,
            true => IntEvent2ImaskDio21::IntEvent2ImaskDio21Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio21_clr(&self) -> bool {
        *self == IntEvent2ImaskDio21::IntEvent2ImaskDio21Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio21_set(&self) -> bool {
        *self == IntEvent2ImaskDio21::IntEvent2ImaskDio21Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO21` writer - DIO21 event mask"]
pub type IntEvent2ImaskDio21W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskDio21>;
impl<'a, REG> IntEvent2ImaskDio21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_dio21_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio21::IntEvent2ImaskDio21Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_dio21_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio21::IntEvent2ImaskDio21Set)
    }
}
#[doc = "DIO22 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskDio22 {
    #[doc = "0: CLR"]
    IntEvent2ImaskDio22Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskDio22Set = 1,
}
impl From<IntEvent2ImaskDio22> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskDio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO22` reader - DIO22 event mask"]
pub type IntEvent2ImaskDio22R = crate::BitReader<IntEvent2ImaskDio22>;
impl IntEvent2ImaskDio22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskDio22 {
        match self.bits {
            false => IntEvent2ImaskDio22::IntEvent2ImaskDio22Clr,
            true => IntEvent2ImaskDio22::IntEvent2ImaskDio22Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio22_clr(&self) -> bool {
        *self == IntEvent2ImaskDio22::IntEvent2ImaskDio22Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio22_set(&self) -> bool {
        *self == IntEvent2ImaskDio22::IntEvent2ImaskDio22Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO22` writer - DIO22 event mask"]
pub type IntEvent2ImaskDio22W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskDio22>;
impl<'a, REG> IntEvent2ImaskDio22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_dio22_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio22::IntEvent2ImaskDio22Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_dio22_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio22::IntEvent2ImaskDio22Set)
    }
}
#[doc = "DIO23 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskDio23 {
    #[doc = "0: CLR"]
    IntEvent2ImaskDio23Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskDio23Set = 1,
}
impl From<IntEvent2ImaskDio23> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskDio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO23` reader - DIO23 event mask"]
pub type IntEvent2ImaskDio23R = crate::BitReader<IntEvent2ImaskDio23>;
impl IntEvent2ImaskDio23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskDio23 {
        match self.bits {
            false => IntEvent2ImaskDio23::IntEvent2ImaskDio23Clr,
            true => IntEvent2ImaskDio23::IntEvent2ImaskDio23Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio23_clr(&self) -> bool {
        *self == IntEvent2ImaskDio23::IntEvent2ImaskDio23Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio23_set(&self) -> bool {
        *self == IntEvent2ImaskDio23::IntEvent2ImaskDio23Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO23` writer - DIO23 event mask"]
pub type IntEvent2ImaskDio23W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskDio23>;
impl<'a, REG> IntEvent2ImaskDio23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_dio23_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio23::IntEvent2ImaskDio23Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_dio23_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio23::IntEvent2ImaskDio23Set)
    }
}
#[doc = "DIO24 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskDio24 {
    #[doc = "0: CLR"]
    IntEvent2ImaskDio24Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskDio24Set = 1,
}
impl From<IntEvent2ImaskDio24> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskDio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO24` reader - DIO24 event mask"]
pub type IntEvent2ImaskDio24R = crate::BitReader<IntEvent2ImaskDio24>;
impl IntEvent2ImaskDio24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskDio24 {
        match self.bits {
            false => IntEvent2ImaskDio24::IntEvent2ImaskDio24Clr,
            true => IntEvent2ImaskDio24::IntEvent2ImaskDio24Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio24_clr(&self) -> bool {
        *self == IntEvent2ImaskDio24::IntEvent2ImaskDio24Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio24_set(&self) -> bool {
        *self == IntEvent2ImaskDio24::IntEvent2ImaskDio24Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO24` writer - DIO24 event mask"]
pub type IntEvent2ImaskDio24W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskDio24>;
impl<'a, REG> IntEvent2ImaskDio24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_dio24_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio24::IntEvent2ImaskDio24Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_dio24_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio24::IntEvent2ImaskDio24Set)
    }
}
#[doc = "DIO25 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskDio25 {
    #[doc = "0: CLR"]
    IntEvent2ImaskDio25Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskDio25Set = 1,
}
impl From<IntEvent2ImaskDio25> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskDio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO25` reader - DIO25 event mask"]
pub type IntEvent2ImaskDio25R = crate::BitReader<IntEvent2ImaskDio25>;
impl IntEvent2ImaskDio25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskDio25 {
        match self.bits {
            false => IntEvent2ImaskDio25::IntEvent2ImaskDio25Clr,
            true => IntEvent2ImaskDio25::IntEvent2ImaskDio25Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio25_clr(&self) -> bool {
        *self == IntEvent2ImaskDio25::IntEvent2ImaskDio25Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio25_set(&self) -> bool {
        *self == IntEvent2ImaskDio25::IntEvent2ImaskDio25Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO25` writer - DIO25 event mask"]
pub type IntEvent2ImaskDio25W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskDio25>;
impl<'a, REG> IntEvent2ImaskDio25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_dio25_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio25::IntEvent2ImaskDio25Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_dio25_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio25::IntEvent2ImaskDio25Set)
    }
}
#[doc = "DIO26 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskDio26 {
    #[doc = "0: CLR"]
    IntEvent2ImaskDio26Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskDio26Set = 1,
}
impl From<IntEvent2ImaskDio26> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskDio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO26` reader - DIO26 event mask"]
pub type IntEvent2ImaskDio26R = crate::BitReader<IntEvent2ImaskDio26>;
impl IntEvent2ImaskDio26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskDio26 {
        match self.bits {
            false => IntEvent2ImaskDio26::IntEvent2ImaskDio26Clr,
            true => IntEvent2ImaskDio26::IntEvent2ImaskDio26Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio26_clr(&self) -> bool {
        *self == IntEvent2ImaskDio26::IntEvent2ImaskDio26Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio26_set(&self) -> bool {
        *self == IntEvent2ImaskDio26::IntEvent2ImaskDio26Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO26` writer - DIO26 event mask"]
pub type IntEvent2ImaskDio26W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskDio26>;
impl<'a, REG> IntEvent2ImaskDio26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_dio26_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio26::IntEvent2ImaskDio26Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_dio26_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio26::IntEvent2ImaskDio26Set)
    }
}
#[doc = "DIO27 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskDio27 {
    #[doc = "0: CLR"]
    IntEvent2ImaskDio27Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskDio27Set = 1,
}
impl From<IntEvent2ImaskDio27> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskDio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO27` reader - DIO27 event mask"]
pub type IntEvent2ImaskDio27R = crate::BitReader<IntEvent2ImaskDio27>;
impl IntEvent2ImaskDio27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskDio27 {
        match self.bits {
            false => IntEvent2ImaskDio27::IntEvent2ImaskDio27Clr,
            true => IntEvent2ImaskDio27::IntEvent2ImaskDio27Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio27_clr(&self) -> bool {
        *self == IntEvent2ImaskDio27::IntEvent2ImaskDio27Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio27_set(&self) -> bool {
        *self == IntEvent2ImaskDio27::IntEvent2ImaskDio27Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO27` writer - DIO27 event mask"]
pub type IntEvent2ImaskDio27W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskDio27>;
impl<'a, REG> IntEvent2ImaskDio27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_dio27_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio27::IntEvent2ImaskDio27Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_dio27_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio27::IntEvent2ImaskDio27Set)
    }
}
#[doc = "DIO28 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskDio28 {
    #[doc = "0: CLR"]
    IntEvent2ImaskDio28Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskDio28Set = 1,
}
impl From<IntEvent2ImaskDio28> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskDio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO28` reader - DIO28 event mask"]
pub type IntEvent2ImaskDio28R = crate::BitReader<IntEvent2ImaskDio28>;
impl IntEvent2ImaskDio28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskDio28 {
        match self.bits {
            false => IntEvent2ImaskDio28::IntEvent2ImaskDio28Clr,
            true => IntEvent2ImaskDio28::IntEvent2ImaskDio28Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio28_clr(&self) -> bool {
        *self == IntEvent2ImaskDio28::IntEvent2ImaskDio28Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio28_set(&self) -> bool {
        *self == IntEvent2ImaskDio28::IntEvent2ImaskDio28Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO28` writer - DIO28 event mask"]
pub type IntEvent2ImaskDio28W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskDio28>;
impl<'a, REG> IntEvent2ImaskDio28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_dio28_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio28::IntEvent2ImaskDio28Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_dio28_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio28::IntEvent2ImaskDio28Set)
    }
}
#[doc = "DIO29 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskDio29 {
    #[doc = "0: CLR"]
    IntEvent2ImaskDio29Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskDio29Set = 1,
}
impl From<IntEvent2ImaskDio29> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskDio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO29` reader - DIO29 event mask"]
pub type IntEvent2ImaskDio29R = crate::BitReader<IntEvent2ImaskDio29>;
impl IntEvent2ImaskDio29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskDio29 {
        match self.bits {
            false => IntEvent2ImaskDio29::IntEvent2ImaskDio29Clr,
            true => IntEvent2ImaskDio29::IntEvent2ImaskDio29Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio29_clr(&self) -> bool {
        *self == IntEvent2ImaskDio29::IntEvent2ImaskDio29Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio29_set(&self) -> bool {
        *self == IntEvent2ImaskDio29::IntEvent2ImaskDio29Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO29` writer - DIO29 event mask"]
pub type IntEvent2ImaskDio29W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskDio29>;
impl<'a, REG> IntEvent2ImaskDio29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_dio29_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio29::IntEvent2ImaskDio29Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_dio29_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio29::IntEvent2ImaskDio29Set)
    }
}
#[doc = "DIO30 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskDio30 {
    #[doc = "0: CLR"]
    IntEvent2ImaskDio30Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskDio30Set = 1,
}
impl From<IntEvent2ImaskDio30> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskDio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO30` reader - DIO30 event mask"]
pub type IntEvent2ImaskDio30R = crate::BitReader<IntEvent2ImaskDio30>;
impl IntEvent2ImaskDio30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskDio30 {
        match self.bits {
            false => IntEvent2ImaskDio30::IntEvent2ImaskDio30Clr,
            true => IntEvent2ImaskDio30::IntEvent2ImaskDio30Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio30_clr(&self) -> bool {
        *self == IntEvent2ImaskDio30::IntEvent2ImaskDio30Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio30_set(&self) -> bool {
        *self == IntEvent2ImaskDio30::IntEvent2ImaskDio30Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO30` writer - DIO30 event mask"]
pub type IntEvent2ImaskDio30W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskDio30>;
impl<'a, REG> IntEvent2ImaskDio30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_dio30_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio30::IntEvent2ImaskDio30Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_dio30_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio30::IntEvent2ImaskDio30Set)
    }
}
#[doc = "DIO31 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2ImaskDio31 {
    #[doc = "0: CLR"]
    IntEvent2ImaskDio31Clr = 0,
    #[doc = "1: SET"]
    IntEvent2ImaskDio31Set = 1,
}
impl From<IntEvent2ImaskDio31> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2ImaskDio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO31` reader - DIO31 event mask"]
pub type IntEvent2ImaskDio31R = crate::BitReader<IntEvent2ImaskDio31>;
impl IntEvent2ImaskDio31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2ImaskDio31 {
        match self.bits {
            false => IntEvent2ImaskDio31::IntEvent2ImaskDio31Clr,
            true => IntEvent2ImaskDio31::IntEvent2ImaskDio31Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio31_clr(&self) -> bool {
        *self == IntEvent2ImaskDio31::IntEvent2ImaskDio31Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_imask_dio31_set(&self) -> bool {
        *self == IntEvent2ImaskDio31::IntEvent2ImaskDio31Set
    }
}
#[doc = "Field `INT_EVENT2_IMASK_DIO31` writer - DIO31 event mask"]
pub type IntEvent2ImaskDio31W<'a, REG> = crate::BitWriter<'a, REG, IntEvent2ImaskDio31>;
impl<'a, REG> IntEvent2ImaskDio31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event2_imask_dio31_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio31::IntEvent2ImaskDio31Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event2_imask_dio31_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent2ImaskDio31::IntEvent2ImaskDio31Set)
    }
}
impl R {
    #[doc = "Bit 16 - DIO16 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio16(&self) -> IntEvent2ImaskDio16R {
        IntEvent2ImaskDio16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DIO17 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio17(&self) -> IntEvent2ImaskDio17R {
        IntEvent2ImaskDio17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DIO18 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio18(&self) -> IntEvent2ImaskDio18R {
        IntEvent2ImaskDio18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DIO19 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio19(&self) -> IntEvent2ImaskDio19R {
        IntEvent2ImaskDio19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DIO20 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio20(&self) -> IntEvent2ImaskDio20R {
        IntEvent2ImaskDio20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DIO21 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio21(&self) -> IntEvent2ImaskDio21R {
        IntEvent2ImaskDio21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DIO22 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio22(&self) -> IntEvent2ImaskDio22R {
        IntEvent2ImaskDio22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DIO23 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio23(&self) -> IntEvent2ImaskDio23R {
        IntEvent2ImaskDio23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DIO24 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio24(&self) -> IntEvent2ImaskDio24R {
        IntEvent2ImaskDio24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DIO25 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio25(&self) -> IntEvent2ImaskDio25R {
        IntEvent2ImaskDio25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DIO26 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio26(&self) -> IntEvent2ImaskDio26R {
        IntEvent2ImaskDio26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DIO27 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio27(&self) -> IntEvent2ImaskDio27R {
        IntEvent2ImaskDio27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DIO28 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio28(&self) -> IntEvent2ImaskDio28R {
        IntEvent2ImaskDio28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DIO29 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio29(&self) -> IntEvent2ImaskDio29R {
        IntEvent2ImaskDio29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DIO30 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio30(&self) -> IntEvent2ImaskDio30R {
        IntEvent2ImaskDio30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DIO31 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio31(&self) -> IntEvent2ImaskDio31R {
        IntEvent2ImaskDio31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - DIO16 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio16(&mut self) -> IntEvent2ImaskDio16W<IntEvent2ImaskSpec> {
        IntEvent2ImaskDio16W::new(self, 16)
    }
    #[doc = "Bit 17 - DIO17 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio17(&mut self) -> IntEvent2ImaskDio17W<IntEvent2ImaskSpec> {
        IntEvent2ImaskDio17W::new(self, 17)
    }
    #[doc = "Bit 18 - DIO18 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio18(&mut self) -> IntEvent2ImaskDio18W<IntEvent2ImaskSpec> {
        IntEvent2ImaskDio18W::new(self, 18)
    }
    #[doc = "Bit 19 - DIO19 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio19(&mut self) -> IntEvent2ImaskDio19W<IntEvent2ImaskSpec> {
        IntEvent2ImaskDio19W::new(self, 19)
    }
    #[doc = "Bit 20 - DIO20 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio20(&mut self) -> IntEvent2ImaskDio20W<IntEvent2ImaskSpec> {
        IntEvent2ImaskDio20W::new(self, 20)
    }
    #[doc = "Bit 21 - DIO21 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio21(&mut self) -> IntEvent2ImaskDio21W<IntEvent2ImaskSpec> {
        IntEvent2ImaskDio21W::new(self, 21)
    }
    #[doc = "Bit 22 - DIO22 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio22(&mut self) -> IntEvent2ImaskDio22W<IntEvent2ImaskSpec> {
        IntEvent2ImaskDio22W::new(self, 22)
    }
    #[doc = "Bit 23 - DIO23 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio23(&mut self) -> IntEvent2ImaskDio23W<IntEvent2ImaskSpec> {
        IntEvent2ImaskDio23W::new(self, 23)
    }
    #[doc = "Bit 24 - DIO24 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio24(&mut self) -> IntEvent2ImaskDio24W<IntEvent2ImaskSpec> {
        IntEvent2ImaskDio24W::new(self, 24)
    }
    #[doc = "Bit 25 - DIO25 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio25(&mut self) -> IntEvent2ImaskDio25W<IntEvent2ImaskSpec> {
        IntEvent2ImaskDio25W::new(self, 25)
    }
    #[doc = "Bit 26 - DIO26 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio26(&mut self) -> IntEvent2ImaskDio26W<IntEvent2ImaskSpec> {
        IntEvent2ImaskDio26W::new(self, 26)
    }
    #[doc = "Bit 27 - DIO27 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio27(&mut self) -> IntEvent2ImaskDio27W<IntEvent2ImaskSpec> {
        IntEvent2ImaskDio27W::new(self, 27)
    }
    #[doc = "Bit 28 - DIO28 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio28(&mut self) -> IntEvent2ImaskDio28W<IntEvent2ImaskSpec> {
        IntEvent2ImaskDio28W::new(self, 28)
    }
    #[doc = "Bit 29 - DIO29 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio29(&mut self) -> IntEvent2ImaskDio29W<IntEvent2ImaskSpec> {
        IntEvent2ImaskDio29W::new(self, 29)
    }
    #[doc = "Bit 30 - DIO30 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio30(&mut self) -> IntEvent2ImaskDio30W<IntEvent2ImaskSpec> {
        IntEvent2ImaskDio30W::new(self, 30)
    }
    #[doc = "Bit 31 - DIO31 event mask"]
    #[inline(always)]
    pub fn int_event2_imask_dio31(&mut self) -> IntEvent2ImaskDio31W<IntEvent2ImaskSpec> {
        IntEvent2ImaskDio31W::new(self, 31)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event2_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2ImaskSpec;
impl crate::RegisterSpec for IntEvent2ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event2_imask::R`](R) reader structure"]
impl crate::Readable for IntEvent2ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_event2_imask::W`](W) writer structure"]
impl crate::Writable for IntEvent2ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT2_IMASK to value 0"]
impl crate::Resettable for IntEvent2ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
