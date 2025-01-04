#[doc = "Register `INT_EVENT1_IMASK` reader"]
pub type R = crate::R<IntEvent1ImaskSpec>;
#[doc = "Register `INT_EVENT1_IMASK` writer"]
pub type W = crate::W<IntEvent1ImaskSpec>;
#[doc = "DIO0 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskDio0 {
    #[doc = "0: CLR"]
    IntEvent1ImaskDio0Clr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskDio0Set = 1,
}
impl From<IntEvent1ImaskDio0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskDio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO0` reader - DIO0 event mask"]
pub type IntEvent1ImaskDio0R = crate::BitReader<IntEvent1ImaskDio0>;
impl IntEvent1ImaskDio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskDio0 {
        match self.bits {
            false => IntEvent1ImaskDio0::IntEvent1ImaskDio0Clr,
            true => IntEvent1ImaskDio0::IntEvent1ImaskDio0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio0_clr(&self) -> bool {
        *self == IntEvent1ImaskDio0::IntEvent1ImaskDio0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio0_set(&self) -> bool {
        *self == IntEvent1ImaskDio0::IntEvent1ImaskDio0Set
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO0` writer - DIO0 event mask"]
pub type IntEvent1ImaskDio0W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskDio0>;
impl<'a, REG> IntEvent1ImaskDio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_dio0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio0::IntEvent1ImaskDio0Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_dio0_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio0::IntEvent1ImaskDio0Set)
    }
}
#[doc = "DIO1 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskDio1 {
    #[doc = "0: CLR"]
    IntEvent1ImaskDio1Clr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskDio1Set = 1,
}
impl From<IntEvent1ImaskDio1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskDio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO1` reader - DIO1 event mask"]
pub type IntEvent1ImaskDio1R = crate::BitReader<IntEvent1ImaskDio1>;
impl IntEvent1ImaskDio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskDio1 {
        match self.bits {
            false => IntEvent1ImaskDio1::IntEvent1ImaskDio1Clr,
            true => IntEvent1ImaskDio1::IntEvent1ImaskDio1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio1_clr(&self) -> bool {
        *self == IntEvent1ImaskDio1::IntEvent1ImaskDio1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio1_set(&self) -> bool {
        *self == IntEvent1ImaskDio1::IntEvent1ImaskDio1Set
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO1` writer - DIO1 event mask"]
pub type IntEvent1ImaskDio1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskDio1>;
impl<'a, REG> IntEvent1ImaskDio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_dio1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio1::IntEvent1ImaskDio1Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_dio1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio1::IntEvent1ImaskDio1Set)
    }
}
#[doc = "DIO2 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskDio2 {
    #[doc = "0: CLR"]
    IntEvent1ImaskDio2Clr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskDio2Set = 1,
}
impl From<IntEvent1ImaskDio2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskDio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO2` reader - DIO2 event mask"]
pub type IntEvent1ImaskDio2R = crate::BitReader<IntEvent1ImaskDio2>;
impl IntEvent1ImaskDio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskDio2 {
        match self.bits {
            false => IntEvent1ImaskDio2::IntEvent1ImaskDio2Clr,
            true => IntEvent1ImaskDio2::IntEvent1ImaskDio2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio2_clr(&self) -> bool {
        *self == IntEvent1ImaskDio2::IntEvent1ImaskDio2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio2_set(&self) -> bool {
        *self == IntEvent1ImaskDio2::IntEvent1ImaskDio2Set
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO2` writer - DIO2 event mask"]
pub type IntEvent1ImaskDio2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskDio2>;
impl<'a, REG> IntEvent1ImaskDio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_dio2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio2::IntEvent1ImaskDio2Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_dio2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio2::IntEvent1ImaskDio2Set)
    }
}
#[doc = "DIO3 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskDio3 {
    #[doc = "0: CLR"]
    IntEvent1ImaskDio3Clr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskDio3Set = 1,
}
impl From<IntEvent1ImaskDio3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskDio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO3` reader - DIO3 event mask"]
pub type IntEvent1ImaskDio3R = crate::BitReader<IntEvent1ImaskDio3>;
impl IntEvent1ImaskDio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskDio3 {
        match self.bits {
            false => IntEvent1ImaskDio3::IntEvent1ImaskDio3Clr,
            true => IntEvent1ImaskDio3::IntEvent1ImaskDio3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio3_clr(&self) -> bool {
        *self == IntEvent1ImaskDio3::IntEvent1ImaskDio3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio3_set(&self) -> bool {
        *self == IntEvent1ImaskDio3::IntEvent1ImaskDio3Set
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO3` writer - DIO3 event mask"]
pub type IntEvent1ImaskDio3W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskDio3>;
impl<'a, REG> IntEvent1ImaskDio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_dio3_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio3::IntEvent1ImaskDio3Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_dio3_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio3::IntEvent1ImaskDio3Set)
    }
}
#[doc = "DIO4 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskDio4 {
    #[doc = "0: CLR"]
    IntEvent1ImaskDio4Clr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskDio4Set = 1,
}
impl From<IntEvent1ImaskDio4> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskDio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO4` reader - DIO4 event mask"]
pub type IntEvent1ImaskDio4R = crate::BitReader<IntEvent1ImaskDio4>;
impl IntEvent1ImaskDio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskDio4 {
        match self.bits {
            false => IntEvent1ImaskDio4::IntEvent1ImaskDio4Clr,
            true => IntEvent1ImaskDio4::IntEvent1ImaskDio4Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio4_clr(&self) -> bool {
        *self == IntEvent1ImaskDio4::IntEvent1ImaskDio4Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio4_set(&self) -> bool {
        *self == IntEvent1ImaskDio4::IntEvent1ImaskDio4Set
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO4` writer - DIO4 event mask"]
pub type IntEvent1ImaskDio4W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskDio4>;
impl<'a, REG> IntEvent1ImaskDio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_dio4_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio4::IntEvent1ImaskDio4Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_dio4_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio4::IntEvent1ImaskDio4Set)
    }
}
#[doc = "DIO5 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskDio5 {
    #[doc = "0: CLR"]
    IntEvent1ImaskDio5Clr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskDio5Set = 1,
}
impl From<IntEvent1ImaskDio5> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskDio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO5` reader - DIO5 event mask"]
pub type IntEvent1ImaskDio5R = crate::BitReader<IntEvent1ImaskDio5>;
impl IntEvent1ImaskDio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskDio5 {
        match self.bits {
            false => IntEvent1ImaskDio5::IntEvent1ImaskDio5Clr,
            true => IntEvent1ImaskDio5::IntEvent1ImaskDio5Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio5_clr(&self) -> bool {
        *self == IntEvent1ImaskDio5::IntEvent1ImaskDio5Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio5_set(&self) -> bool {
        *self == IntEvent1ImaskDio5::IntEvent1ImaskDio5Set
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO5` writer - DIO5 event mask"]
pub type IntEvent1ImaskDio5W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskDio5>;
impl<'a, REG> IntEvent1ImaskDio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_dio5_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio5::IntEvent1ImaskDio5Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_dio5_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio5::IntEvent1ImaskDio5Set)
    }
}
#[doc = "DIO6 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskDio6 {
    #[doc = "0: CLR"]
    IntEvent1ImaskDio6Clr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskDio6Set = 1,
}
impl From<IntEvent1ImaskDio6> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskDio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO6` reader - DIO6 event mask"]
pub type IntEvent1ImaskDio6R = crate::BitReader<IntEvent1ImaskDio6>;
impl IntEvent1ImaskDio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskDio6 {
        match self.bits {
            false => IntEvent1ImaskDio6::IntEvent1ImaskDio6Clr,
            true => IntEvent1ImaskDio6::IntEvent1ImaskDio6Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio6_clr(&self) -> bool {
        *self == IntEvent1ImaskDio6::IntEvent1ImaskDio6Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio6_set(&self) -> bool {
        *self == IntEvent1ImaskDio6::IntEvent1ImaskDio6Set
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO6` writer - DIO6 event mask"]
pub type IntEvent1ImaskDio6W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskDio6>;
impl<'a, REG> IntEvent1ImaskDio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_dio6_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio6::IntEvent1ImaskDio6Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_dio6_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio6::IntEvent1ImaskDio6Set)
    }
}
#[doc = "DIO7 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskDio7 {
    #[doc = "0: CLR"]
    IntEvent1ImaskDio7Clr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskDio7Set = 1,
}
impl From<IntEvent1ImaskDio7> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskDio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO7` reader - DIO7 event mask"]
pub type IntEvent1ImaskDio7R = crate::BitReader<IntEvent1ImaskDio7>;
impl IntEvent1ImaskDio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskDio7 {
        match self.bits {
            false => IntEvent1ImaskDio7::IntEvent1ImaskDio7Clr,
            true => IntEvent1ImaskDio7::IntEvent1ImaskDio7Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio7_clr(&self) -> bool {
        *self == IntEvent1ImaskDio7::IntEvent1ImaskDio7Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio7_set(&self) -> bool {
        *self == IntEvent1ImaskDio7::IntEvent1ImaskDio7Set
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO7` writer - DIO7 event mask"]
pub type IntEvent1ImaskDio7W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskDio7>;
impl<'a, REG> IntEvent1ImaskDio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_dio7_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio7::IntEvent1ImaskDio7Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_dio7_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio7::IntEvent1ImaskDio7Set)
    }
}
#[doc = "DIO8 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskDio8 {
    #[doc = "0: CLR"]
    IntEvent1ImaskDio8Clr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskDio8Set = 1,
}
impl From<IntEvent1ImaskDio8> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskDio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO8` reader - DIO8 event mask"]
pub type IntEvent1ImaskDio8R = crate::BitReader<IntEvent1ImaskDio8>;
impl IntEvent1ImaskDio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskDio8 {
        match self.bits {
            false => IntEvent1ImaskDio8::IntEvent1ImaskDio8Clr,
            true => IntEvent1ImaskDio8::IntEvent1ImaskDio8Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio8_clr(&self) -> bool {
        *self == IntEvent1ImaskDio8::IntEvent1ImaskDio8Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio8_set(&self) -> bool {
        *self == IntEvent1ImaskDio8::IntEvent1ImaskDio8Set
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO8` writer - DIO8 event mask"]
pub type IntEvent1ImaskDio8W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskDio8>;
impl<'a, REG> IntEvent1ImaskDio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_dio8_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio8::IntEvent1ImaskDio8Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_dio8_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio8::IntEvent1ImaskDio8Set)
    }
}
#[doc = "DIO9 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskDio9 {
    #[doc = "0: CLR"]
    IntEvent1ImaskDio9Clr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskDio9Set = 1,
}
impl From<IntEvent1ImaskDio9> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskDio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO9` reader - DIO9 event mask"]
pub type IntEvent1ImaskDio9R = crate::BitReader<IntEvent1ImaskDio9>;
impl IntEvent1ImaskDio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskDio9 {
        match self.bits {
            false => IntEvent1ImaskDio9::IntEvent1ImaskDio9Clr,
            true => IntEvent1ImaskDio9::IntEvent1ImaskDio9Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio9_clr(&self) -> bool {
        *self == IntEvent1ImaskDio9::IntEvent1ImaskDio9Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio9_set(&self) -> bool {
        *self == IntEvent1ImaskDio9::IntEvent1ImaskDio9Set
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO9` writer - DIO9 event mask"]
pub type IntEvent1ImaskDio9W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskDio9>;
impl<'a, REG> IntEvent1ImaskDio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_dio9_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio9::IntEvent1ImaskDio9Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_dio9_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio9::IntEvent1ImaskDio9Set)
    }
}
#[doc = "DIO10 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskDio10 {
    #[doc = "0: CLR"]
    IntEvent1ImaskDio10Clr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskDio10Set = 1,
}
impl From<IntEvent1ImaskDio10> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskDio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO10` reader - DIO10 event mask"]
pub type IntEvent1ImaskDio10R = crate::BitReader<IntEvent1ImaskDio10>;
impl IntEvent1ImaskDio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskDio10 {
        match self.bits {
            false => IntEvent1ImaskDio10::IntEvent1ImaskDio10Clr,
            true => IntEvent1ImaskDio10::IntEvent1ImaskDio10Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio10_clr(&self) -> bool {
        *self == IntEvent1ImaskDio10::IntEvent1ImaskDio10Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio10_set(&self) -> bool {
        *self == IntEvent1ImaskDio10::IntEvent1ImaskDio10Set
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO10` writer - DIO10 event mask"]
pub type IntEvent1ImaskDio10W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskDio10>;
impl<'a, REG> IntEvent1ImaskDio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_dio10_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio10::IntEvent1ImaskDio10Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_dio10_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio10::IntEvent1ImaskDio10Set)
    }
}
#[doc = "DIO11 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskDio11 {
    #[doc = "0: CLR"]
    IntEvent1ImaskDio11Clr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskDio11Set = 1,
}
impl From<IntEvent1ImaskDio11> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskDio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO11` reader - DIO11 event mask"]
pub type IntEvent1ImaskDio11R = crate::BitReader<IntEvent1ImaskDio11>;
impl IntEvent1ImaskDio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskDio11 {
        match self.bits {
            false => IntEvent1ImaskDio11::IntEvent1ImaskDio11Clr,
            true => IntEvent1ImaskDio11::IntEvent1ImaskDio11Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio11_clr(&self) -> bool {
        *self == IntEvent1ImaskDio11::IntEvent1ImaskDio11Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio11_set(&self) -> bool {
        *self == IntEvent1ImaskDio11::IntEvent1ImaskDio11Set
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO11` writer - DIO11 event mask"]
pub type IntEvent1ImaskDio11W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskDio11>;
impl<'a, REG> IntEvent1ImaskDio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_dio11_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio11::IntEvent1ImaskDio11Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_dio11_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio11::IntEvent1ImaskDio11Set)
    }
}
#[doc = "DIO12 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskDio12 {
    #[doc = "0: CLR"]
    IntEvent1ImaskDio12Clr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskDio12Set = 1,
}
impl From<IntEvent1ImaskDio12> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskDio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO12` reader - DIO12 event mask"]
pub type IntEvent1ImaskDio12R = crate::BitReader<IntEvent1ImaskDio12>;
impl IntEvent1ImaskDio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskDio12 {
        match self.bits {
            false => IntEvent1ImaskDio12::IntEvent1ImaskDio12Clr,
            true => IntEvent1ImaskDio12::IntEvent1ImaskDio12Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio12_clr(&self) -> bool {
        *self == IntEvent1ImaskDio12::IntEvent1ImaskDio12Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio12_set(&self) -> bool {
        *self == IntEvent1ImaskDio12::IntEvent1ImaskDio12Set
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO12` writer - DIO12 event mask"]
pub type IntEvent1ImaskDio12W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskDio12>;
impl<'a, REG> IntEvent1ImaskDio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_dio12_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio12::IntEvent1ImaskDio12Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_dio12_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio12::IntEvent1ImaskDio12Set)
    }
}
#[doc = "DIO13 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskDio13 {
    #[doc = "0: CLR"]
    IntEvent1ImaskDio13Clr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskDio13Set = 1,
}
impl From<IntEvent1ImaskDio13> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskDio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO13` reader - DIO13 event mask"]
pub type IntEvent1ImaskDio13R = crate::BitReader<IntEvent1ImaskDio13>;
impl IntEvent1ImaskDio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskDio13 {
        match self.bits {
            false => IntEvent1ImaskDio13::IntEvent1ImaskDio13Clr,
            true => IntEvent1ImaskDio13::IntEvent1ImaskDio13Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio13_clr(&self) -> bool {
        *self == IntEvent1ImaskDio13::IntEvent1ImaskDio13Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio13_set(&self) -> bool {
        *self == IntEvent1ImaskDio13::IntEvent1ImaskDio13Set
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO13` writer - DIO13 event mask"]
pub type IntEvent1ImaskDio13W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskDio13>;
impl<'a, REG> IntEvent1ImaskDio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_dio13_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio13::IntEvent1ImaskDio13Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_dio13_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio13::IntEvent1ImaskDio13Set)
    }
}
#[doc = "DIO14 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskDio14 {
    #[doc = "0: CLR"]
    IntEvent1ImaskDio14Clr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskDio14Set = 1,
}
impl From<IntEvent1ImaskDio14> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskDio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO14` reader - DIO14 event mask"]
pub type IntEvent1ImaskDio14R = crate::BitReader<IntEvent1ImaskDio14>;
impl IntEvent1ImaskDio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskDio14 {
        match self.bits {
            false => IntEvent1ImaskDio14::IntEvent1ImaskDio14Clr,
            true => IntEvent1ImaskDio14::IntEvent1ImaskDio14Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio14_clr(&self) -> bool {
        *self == IntEvent1ImaskDio14::IntEvent1ImaskDio14Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio14_set(&self) -> bool {
        *self == IntEvent1ImaskDio14::IntEvent1ImaskDio14Set
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO14` writer - DIO14 event mask"]
pub type IntEvent1ImaskDio14W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskDio14>;
impl<'a, REG> IntEvent1ImaskDio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_dio14_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio14::IntEvent1ImaskDio14Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_dio14_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio14::IntEvent1ImaskDio14Set)
    }
}
#[doc = "DIO15 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent1ImaskDio15 {
    #[doc = "0: CLR"]
    IntEvent1ImaskDio15Clr = 0,
    #[doc = "1: SET"]
    IntEvent1ImaskDio15Set = 1,
}
impl From<IntEvent1ImaskDio15> for bool {
    #[inline(always)]
    fn from(variant: IntEvent1ImaskDio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO15` reader - DIO15 event mask"]
pub type IntEvent1ImaskDio15R = crate::BitReader<IntEvent1ImaskDio15>;
impl IntEvent1ImaskDio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent1ImaskDio15 {
        match self.bits {
            false => IntEvent1ImaskDio15::IntEvent1ImaskDio15Clr,
            true => IntEvent1ImaskDio15::IntEvent1ImaskDio15Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio15_clr(&self) -> bool {
        *self == IntEvent1ImaskDio15::IntEvent1ImaskDio15Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event1_imask_dio15_set(&self) -> bool {
        *self == IntEvent1ImaskDio15::IntEvent1ImaskDio15Set
    }
}
#[doc = "Field `INT_EVENT1_IMASK_DIO15` writer - DIO15 event mask"]
pub type IntEvent1ImaskDio15W<'a, REG> = crate::BitWriter<'a, REG, IntEvent1ImaskDio15>;
impl<'a, REG> IntEvent1ImaskDio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event1_imask_dio15_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio15::IntEvent1ImaskDio15Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event1_imask_dio15_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent1ImaskDio15::IntEvent1ImaskDio15Set)
    }
}
impl R {
    #[doc = "Bit 0 - DIO0 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio0(&self) -> IntEvent1ImaskDio0R {
        IntEvent1ImaskDio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIO1 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio1(&self) -> IntEvent1ImaskDio1R {
        IntEvent1ImaskDio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DIO2 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio2(&self) -> IntEvent1ImaskDio2R {
        IntEvent1ImaskDio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DIO3 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio3(&self) -> IntEvent1ImaskDio3R {
        IntEvent1ImaskDio3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DIO4 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio4(&self) -> IntEvent1ImaskDio4R {
        IntEvent1ImaskDio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DIO5 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio5(&self) -> IntEvent1ImaskDio5R {
        IntEvent1ImaskDio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DIO6 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio6(&self) -> IntEvent1ImaskDio6R {
        IntEvent1ImaskDio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DIO7 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio7(&self) -> IntEvent1ImaskDio7R {
        IntEvent1ImaskDio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DIO8 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio8(&self) -> IntEvent1ImaskDio8R {
        IntEvent1ImaskDio8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DIO9 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio9(&self) -> IntEvent1ImaskDio9R {
        IntEvent1ImaskDio9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DIO10 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio10(&self) -> IntEvent1ImaskDio10R {
        IntEvent1ImaskDio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DIO11 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio11(&self) -> IntEvent1ImaskDio11R {
        IntEvent1ImaskDio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DIO12 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio12(&self) -> IntEvent1ImaskDio12R {
        IntEvent1ImaskDio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DIO13 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio13(&self) -> IntEvent1ImaskDio13R {
        IntEvent1ImaskDio13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DIO14 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio14(&self) -> IntEvent1ImaskDio14R {
        IntEvent1ImaskDio14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DIO15 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio15(&self) -> IntEvent1ImaskDio15R {
        IntEvent1ImaskDio15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIO0 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio0(&mut self) -> IntEvent1ImaskDio0W<IntEvent1ImaskSpec> {
        IntEvent1ImaskDio0W::new(self, 0)
    }
    #[doc = "Bit 1 - DIO1 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio1(&mut self) -> IntEvent1ImaskDio1W<IntEvent1ImaskSpec> {
        IntEvent1ImaskDio1W::new(self, 1)
    }
    #[doc = "Bit 2 - DIO2 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio2(&mut self) -> IntEvent1ImaskDio2W<IntEvent1ImaskSpec> {
        IntEvent1ImaskDio2W::new(self, 2)
    }
    #[doc = "Bit 3 - DIO3 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio3(&mut self) -> IntEvent1ImaskDio3W<IntEvent1ImaskSpec> {
        IntEvent1ImaskDio3W::new(self, 3)
    }
    #[doc = "Bit 4 - DIO4 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio4(&mut self) -> IntEvent1ImaskDio4W<IntEvent1ImaskSpec> {
        IntEvent1ImaskDio4W::new(self, 4)
    }
    #[doc = "Bit 5 - DIO5 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio5(&mut self) -> IntEvent1ImaskDio5W<IntEvent1ImaskSpec> {
        IntEvent1ImaskDio5W::new(self, 5)
    }
    #[doc = "Bit 6 - DIO6 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio6(&mut self) -> IntEvent1ImaskDio6W<IntEvent1ImaskSpec> {
        IntEvent1ImaskDio6W::new(self, 6)
    }
    #[doc = "Bit 7 - DIO7 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio7(&mut self) -> IntEvent1ImaskDio7W<IntEvent1ImaskSpec> {
        IntEvent1ImaskDio7W::new(self, 7)
    }
    #[doc = "Bit 8 - DIO8 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio8(&mut self) -> IntEvent1ImaskDio8W<IntEvent1ImaskSpec> {
        IntEvent1ImaskDio8W::new(self, 8)
    }
    #[doc = "Bit 9 - DIO9 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio9(&mut self) -> IntEvent1ImaskDio9W<IntEvent1ImaskSpec> {
        IntEvent1ImaskDio9W::new(self, 9)
    }
    #[doc = "Bit 10 - DIO10 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio10(&mut self) -> IntEvent1ImaskDio10W<IntEvent1ImaskSpec> {
        IntEvent1ImaskDio10W::new(self, 10)
    }
    #[doc = "Bit 11 - DIO11 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio11(&mut self) -> IntEvent1ImaskDio11W<IntEvent1ImaskSpec> {
        IntEvent1ImaskDio11W::new(self, 11)
    }
    #[doc = "Bit 12 - DIO12 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio12(&mut self) -> IntEvent1ImaskDio12W<IntEvent1ImaskSpec> {
        IntEvent1ImaskDio12W::new(self, 12)
    }
    #[doc = "Bit 13 - DIO13 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio13(&mut self) -> IntEvent1ImaskDio13W<IntEvent1ImaskSpec> {
        IntEvent1ImaskDio13W::new(self, 13)
    }
    #[doc = "Bit 14 - DIO14 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio14(&mut self) -> IntEvent1ImaskDio14W<IntEvent1ImaskSpec> {
        IntEvent1ImaskDio14W::new(self, 14)
    }
    #[doc = "Bit 15 - DIO15 event mask"]
    #[inline(always)]
    pub fn int_event1_imask_dio15(&mut self) -> IntEvent1ImaskDio15W<IntEvent1ImaskSpec> {
        IntEvent1ImaskDio15W::new(self, 15)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event1_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1ImaskSpec;
impl crate::RegisterSpec for IntEvent1ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event1_imask::R`](R) reader structure"]
impl crate::Readable for IntEvent1ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_event1_imask::W`](W) writer structure"]
impl crate::Writable for IntEvent1ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT1_IMASK to value 0"]
impl crate::Resettable for IntEvent1ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
