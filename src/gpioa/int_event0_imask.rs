#[doc = "Register `INT_EVENT0_IMASK` reader"]
pub type R = crate::R<IntEvent0ImaskSpec>;
#[doc = "Register `INT_EVENT0_IMASK` writer"]
pub type W = crate::W<IntEvent0ImaskSpec>;
#[doc = "DIO0 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio0 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio0Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio0Set = 1,
}
impl From<IntEvent0ImaskDio0> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO0` reader - DIO0 event mask"]
pub type IntEvent0ImaskDio0R = crate::BitReader<IntEvent0ImaskDio0>;
impl IntEvent0ImaskDio0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio0 {
        match self.bits {
            false => IntEvent0ImaskDio0::IntEvent0ImaskDio0Clr,
            true => IntEvent0ImaskDio0::IntEvent0ImaskDio0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio0_clr(&self) -> bool {
        *self == IntEvent0ImaskDio0::IntEvent0ImaskDio0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio0_set(&self) -> bool {
        *self == IntEvent0ImaskDio0::IntEvent0ImaskDio0Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO0` writer - DIO0 event mask"]
pub type IntEvent0ImaskDio0W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio0>;
impl<'a, REG> IntEvent0ImaskDio0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio0_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio0::IntEvent0ImaskDio0Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio0_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio0::IntEvent0ImaskDio0Set)
    }
}
#[doc = "DIO1 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio1 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio1Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio1Set = 1,
}
impl From<IntEvent0ImaskDio1> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO1` reader - DIO1 event mask"]
pub type IntEvent0ImaskDio1R = crate::BitReader<IntEvent0ImaskDio1>;
impl IntEvent0ImaskDio1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio1 {
        match self.bits {
            false => IntEvent0ImaskDio1::IntEvent0ImaskDio1Clr,
            true => IntEvent0ImaskDio1::IntEvent0ImaskDio1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio1_clr(&self) -> bool {
        *self == IntEvent0ImaskDio1::IntEvent0ImaskDio1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio1_set(&self) -> bool {
        *self == IntEvent0ImaskDio1::IntEvent0ImaskDio1Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO1` writer - DIO1 event mask"]
pub type IntEvent0ImaskDio1W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio1>;
impl<'a, REG> IntEvent0ImaskDio1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio1_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio1::IntEvent0ImaskDio1Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio1_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio1::IntEvent0ImaskDio1Set)
    }
}
#[doc = "DIO2 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio2 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio2Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio2Set = 1,
}
impl From<IntEvent0ImaskDio2> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO2` reader - DIO2 event mask"]
pub type IntEvent0ImaskDio2R = crate::BitReader<IntEvent0ImaskDio2>;
impl IntEvent0ImaskDio2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio2 {
        match self.bits {
            false => IntEvent0ImaskDio2::IntEvent0ImaskDio2Clr,
            true => IntEvent0ImaskDio2::IntEvent0ImaskDio2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio2_clr(&self) -> bool {
        *self == IntEvent0ImaskDio2::IntEvent0ImaskDio2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio2_set(&self) -> bool {
        *self == IntEvent0ImaskDio2::IntEvent0ImaskDio2Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO2` writer - DIO2 event mask"]
pub type IntEvent0ImaskDio2W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio2>;
impl<'a, REG> IntEvent0ImaskDio2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio2_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio2::IntEvent0ImaskDio2Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio2_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio2::IntEvent0ImaskDio2Set)
    }
}
#[doc = "DIO3 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio3 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio3Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio3Set = 1,
}
impl From<IntEvent0ImaskDio3> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO3` reader - DIO3 event mask"]
pub type IntEvent0ImaskDio3R = crate::BitReader<IntEvent0ImaskDio3>;
impl IntEvent0ImaskDio3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio3 {
        match self.bits {
            false => IntEvent0ImaskDio3::IntEvent0ImaskDio3Clr,
            true => IntEvent0ImaskDio3::IntEvent0ImaskDio3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio3_clr(&self) -> bool {
        *self == IntEvent0ImaskDio3::IntEvent0ImaskDio3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio3_set(&self) -> bool {
        *self == IntEvent0ImaskDio3::IntEvent0ImaskDio3Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO3` writer - DIO3 event mask"]
pub type IntEvent0ImaskDio3W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio3>;
impl<'a, REG> IntEvent0ImaskDio3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio3_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio3::IntEvent0ImaskDio3Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio3_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio3::IntEvent0ImaskDio3Set)
    }
}
#[doc = "DIO4 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio4 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio4Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio4Set = 1,
}
impl From<IntEvent0ImaskDio4> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO4` reader - DIO4 event mask"]
pub type IntEvent0ImaskDio4R = crate::BitReader<IntEvent0ImaskDio4>;
impl IntEvent0ImaskDio4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio4 {
        match self.bits {
            false => IntEvent0ImaskDio4::IntEvent0ImaskDio4Clr,
            true => IntEvent0ImaskDio4::IntEvent0ImaskDio4Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio4_clr(&self) -> bool {
        *self == IntEvent0ImaskDio4::IntEvent0ImaskDio4Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio4_set(&self) -> bool {
        *self == IntEvent0ImaskDio4::IntEvent0ImaskDio4Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO4` writer - DIO4 event mask"]
pub type IntEvent0ImaskDio4W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio4>;
impl<'a, REG> IntEvent0ImaskDio4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio4_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio4::IntEvent0ImaskDio4Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio4_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio4::IntEvent0ImaskDio4Set)
    }
}
#[doc = "DIO5 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio5 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio5Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio5Set = 1,
}
impl From<IntEvent0ImaskDio5> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO5` reader - DIO5 event mask"]
pub type IntEvent0ImaskDio5R = crate::BitReader<IntEvent0ImaskDio5>;
impl IntEvent0ImaskDio5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio5 {
        match self.bits {
            false => IntEvent0ImaskDio5::IntEvent0ImaskDio5Clr,
            true => IntEvent0ImaskDio5::IntEvent0ImaskDio5Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio5_clr(&self) -> bool {
        *self == IntEvent0ImaskDio5::IntEvent0ImaskDio5Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio5_set(&self) -> bool {
        *self == IntEvent0ImaskDio5::IntEvent0ImaskDio5Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO5` writer - DIO5 event mask"]
pub type IntEvent0ImaskDio5W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio5>;
impl<'a, REG> IntEvent0ImaskDio5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio5_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio5::IntEvent0ImaskDio5Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio5_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio5::IntEvent0ImaskDio5Set)
    }
}
#[doc = "DIO6 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio6 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio6Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio6Set = 1,
}
impl From<IntEvent0ImaskDio6> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO6` reader - DIO6 event mask"]
pub type IntEvent0ImaskDio6R = crate::BitReader<IntEvent0ImaskDio6>;
impl IntEvent0ImaskDio6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio6 {
        match self.bits {
            false => IntEvent0ImaskDio6::IntEvent0ImaskDio6Clr,
            true => IntEvent0ImaskDio6::IntEvent0ImaskDio6Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio6_clr(&self) -> bool {
        *self == IntEvent0ImaskDio6::IntEvent0ImaskDio6Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio6_set(&self) -> bool {
        *self == IntEvent0ImaskDio6::IntEvent0ImaskDio6Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO6` writer - DIO6 event mask"]
pub type IntEvent0ImaskDio6W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio6>;
impl<'a, REG> IntEvent0ImaskDio6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio6_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio6::IntEvent0ImaskDio6Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio6_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio6::IntEvent0ImaskDio6Set)
    }
}
#[doc = "DIO7 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio7 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio7Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio7Set = 1,
}
impl From<IntEvent0ImaskDio7> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO7` reader - DIO7 event mask"]
pub type IntEvent0ImaskDio7R = crate::BitReader<IntEvent0ImaskDio7>;
impl IntEvent0ImaskDio7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio7 {
        match self.bits {
            false => IntEvent0ImaskDio7::IntEvent0ImaskDio7Clr,
            true => IntEvent0ImaskDio7::IntEvent0ImaskDio7Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio7_clr(&self) -> bool {
        *self == IntEvent0ImaskDio7::IntEvent0ImaskDio7Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio7_set(&self) -> bool {
        *self == IntEvent0ImaskDio7::IntEvent0ImaskDio7Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO7` writer - DIO7 event mask"]
pub type IntEvent0ImaskDio7W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio7>;
impl<'a, REG> IntEvent0ImaskDio7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio7_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio7::IntEvent0ImaskDio7Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio7_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio7::IntEvent0ImaskDio7Set)
    }
}
#[doc = "DIO8 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio8 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio8Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio8Set = 1,
}
impl From<IntEvent0ImaskDio8> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO8` reader - DIO8 event mask"]
pub type IntEvent0ImaskDio8R = crate::BitReader<IntEvent0ImaskDio8>;
impl IntEvent0ImaskDio8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio8 {
        match self.bits {
            false => IntEvent0ImaskDio8::IntEvent0ImaskDio8Clr,
            true => IntEvent0ImaskDio8::IntEvent0ImaskDio8Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio8_clr(&self) -> bool {
        *self == IntEvent0ImaskDio8::IntEvent0ImaskDio8Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio8_set(&self) -> bool {
        *self == IntEvent0ImaskDio8::IntEvent0ImaskDio8Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO8` writer - DIO8 event mask"]
pub type IntEvent0ImaskDio8W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio8>;
impl<'a, REG> IntEvent0ImaskDio8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio8_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio8::IntEvent0ImaskDio8Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio8_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio8::IntEvent0ImaskDio8Set)
    }
}
#[doc = "DIO9 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio9 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio9Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio9Set = 1,
}
impl From<IntEvent0ImaskDio9> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO9` reader - DIO9 event mask"]
pub type IntEvent0ImaskDio9R = crate::BitReader<IntEvent0ImaskDio9>;
impl IntEvent0ImaskDio9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio9 {
        match self.bits {
            false => IntEvent0ImaskDio9::IntEvent0ImaskDio9Clr,
            true => IntEvent0ImaskDio9::IntEvent0ImaskDio9Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio9_clr(&self) -> bool {
        *self == IntEvent0ImaskDio9::IntEvent0ImaskDio9Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio9_set(&self) -> bool {
        *self == IntEvent0ImaskDio9::IntEvent0ImaskDio9Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO9` writer - DIO9 event mask"]
pub type IntEvent0ImaskDio9W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio9>;
impl<'a, REG> IntEvent0ImaskDio9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio9_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio9::IntEvent0ImaskDio9Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio9_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio9::IntEvent0ImaskDio9Set)
    }
}
#[doc = "DIO10 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio10 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio10Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio10Set = 1,
}
impl From<IntEvent0ImaskDio10> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO10` reader - DIO10 event mask"]
pub type IntEvent0ImaskDio10R = crate::BitReader<IntEvent0ImaskDio10>;
impl IntEvent0ImaskDio10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio10 {
        match self.bits {
            false => IntEvent0ImaskDio10::IntEvent0ImaskDio10Clr,
            true => IntEvent0ImaskDio10::IntEvent0ImaskDio10Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio10_clr(&self) -> bool {
        *self == IntEvent0ImaskDio10::IntEvent0ImaskDio10Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio10_set(&self) -> bool {
        *self == IntEvent0ImaskDio10::IntEvent0ImaskDio10Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO10` writer - DIO10 event mask"]
pub type IntEvent0ImaskDio10W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio10>;
impl<'a, REG> IntEvent0ImaskDio10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio10_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio10::IntEvent0ImaskDio10Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio10_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio10::IntEvent0ImaskDio10Set)
    }
}
#[doc = "DIO11 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio11 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio11Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio11Set = 1,
}
impl From<IntEvent0ImaskDio11> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO11` reader - DIO11 event mask"]
pub type IntEvent0ImaskDio11R = crate::BitReader<IntEvent0ImaskDio11>;
impl IntEvent0ImaskDio11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio11 {
        match self.bits {
            false => IntEvent0ImaskDio11::IntEvent0ImaskDio11Clr,
            true => IntEvent0ImaskDio11::IntEvent0ImaskDio11Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio11_clr(&self) -> bool {
        *self == IntEvent0ImaskDio11::IntEvent0ImaskDio11Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio11_set(&self) -> bool {
        *self == IntEvent0ImaskDio11::IntEvent0ImaskDio11Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO11` writer - DIO11 event mask"]
pub type IntEvent0ImaskDio11W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio11>;
impl<'a, REG> IntEvent0ImaskDio11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio11_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio11::IntEvent0ImaskDio11Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio11_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio11::IntEvent0ImaskDio11Set)
    }
}
#[doc = "DIO12 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio12 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio12Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio12Set = 1,
}
impl From<IntEvent0ImaskDio12> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO12` reader - DIO12 event mask"]
pub type IntEvent0ImaskDio12R = crate::BitReader<IntEvent0ImaskDio12>;
impl IntEvent0ImaskDio12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio12 {
        match self.bits {
            false => IntEvent0ImaskDio12::IntEvent0ImaskDio12Clr,
            true => IntEvent0ImaskDio12::IntEvent0ImaskDio12Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio12_clr(&self) -> bool {
        *self == IntEvent0ImaskDio12::IntEvent0ImaskDio12Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio12_set(&self) -> bool {
        *self == IntEvent0ImaskDio12::IntEvent0ImaskDio12Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO12` writer - DIO12 event mask"]
pub type IntEvent0ImaskDio12W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio12>;
impl<'a, REG> IntEvent0ImaskDio12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio12_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio12::IntEvent0ImaskDio12Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio12_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio12::IntEvent0ImaskDio12Set)
    }
}
#[doc = "DIO13 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio13 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio13Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio13Set = 1,
}
impl From<IntEvent0ImaskDio13> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO13` reader - DIO13 event mask"]
pub type IntEvent0ImaskDio13R = crate::BitReader<IntEvent0ImaskDio13>;
impl IntEvent0ImaskDio13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio13 {
        match self.bits {
            false => IntEvent0ImaskDio13::IntEvent0ImaskDio13Clr,
            true => IntEvent0ImaskDio13::IntEvent0ImaskDio13Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio13_clr(&self) -> bool {
        *self == IntEvent0ImaskDio13::IntEvent0ImaskDio13Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio13_set(&self) -> bool {
        *self == IntEvent0ImaskDio13::IntEvent0ImaskDio13Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO13` writer - DIO13 event mask"]
pub type IntEvent0ImaskDio13W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio13>;
impl<'a, REG> IntEvent0ImaskDio13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio13_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio13::IntEvent0ImaskDio13Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio13_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio13::IntEvent0ImaskDio13Set)
    }
}
#[doc = "DIO14 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio14 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio14Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio14Set = 1,
}
impl From<IntEvent0ImaskDio14> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO14` reader - DIO14 event mask"]
pub type IntEvent0ImaskDio14R = crate::BitReader<IntEvent0ImaskDio14>;
impl IntEvent0ImaskDio14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio14 {
        match self.bits {
            false => IntEvent0ImaskDio14::IntEvent0ImaskDio14Clr,
            true => IntEvent0ImaskDio14::IntEvent0ImaskDio14Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio14_clr(&self) -> bool {
        *self == IntEvent0ImaskDio14::IntEvent0ImaskDio14Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio14_set(&self) -> bool {
        *self == IntEvent0ImaskDio14::IntEvent0ImaskDio14Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO14` writer - DIO14 event mask"]
pub type IntEvent0ImaskDio14W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio14>;
impl<'a, REG> IntEvent0ImaskDio14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio14_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio14::IntEvent0ImaskDio14Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio14_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio14::IntEvent0ImaskDio14Set)
    }
}
#[doc = "DIO15 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio15 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio15Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio15Set = 1,
}
impl From<IntEvent0ImaskDio15> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO15` reader - DIO15 event mask"]
pub type IntEvent0ImaskDio15R = crate::BitReader<IntEvent0ImaskDio15>;
impl IntEvent0ImaskDio15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio15 {
        match self.bits {
            false => IntEvent0ImaskDio15::IntEvent0ImaskDio15Clr,
            true => IntEvent0ImaskDio15::IntEvent0ImaskDio15Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio15_clr(&self) -> bool {
        *self == IntEvent0ImaskDio15::IntEvent0ImaskDio15Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio15_set(&self) -> bool {
        *self == IntEvent0ImaskDio15::IntEvent0ImaskDio15Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO15` writer - DIO15 event mask"]
pub type IntEvent0ImaskDio15W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio15>;
impl<'a, REG> IntEvent0ImaskDio15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio15_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio15::IntEvent0ImaskDio15Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio15_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio15::IntEvent0ImaskDio15Set)
    }
}
#[doc = "DIO16 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio16 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio16Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio16Set = 1,
}
impl From<IntEvent0ImaskDio16> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO16` reader - DIO16 event mask"]
pub type IntEvent0ImaskDio16R = crate::BitReader<IntEvent0ImaskDio16>;
impl IntEvent0ImaskDio16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio16 {
        match self.bits {
            false => IntEvent0ImaskDio16::IntEvent0ImaskDio16Clr,
            true => IntEvent0ImaskDio16::IntEvent0ImaskDio16Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio16_clr(&self) -> bool {
        *self == IntEvent0ImaskDio16::IntEvent0ImaskDio16Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio16_set(&self) -> bool {
        *self == IntEvent0ImaskDio16::IntEvent0ImaskDio16Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO16` writer - DIO16 event mask"]
pub type IntEvent0ImaskDio16W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio16>;
impl<'a, REG> IntEvent0ImaskDio16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio16_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio16::IntEvent0ImaskDio16Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio16_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio16::IntEvent0ImaskDio16Set)
    }
}
#[doc = "DIO17 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio17 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio17Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio17Set = 1,
}
impl From<IntEvent0ImaskDio17> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO17` reader - DIO17 event mask"]
pub type IntEvent0ImaskDio17R = crate::BitReader<IntEvent0ImaskDio17>;
impl IntEvent0ImaskDio17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio17 {
        match self.bits {
            false => IntEvent0ImaskDio17::IntEvent0ImaskDio17Clr,
            true => IntEvent0ImaskDio17::IntEvent0ImaskDio17Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio17_clr(&self) -> bool {
        *self == IntEvent0ImaskDio17::IntEvent0ImaskDio17Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio17_set(&self) -> bool {
        *self == IntEvent0ImaskDio17::IntEvent0ImaskDio17Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO17` writer - DIO17 event mask"]
pub type IntEvent0ImaskDio17W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio17>;
impl<'a, REG> IntEvent0ImaskDio17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio17_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio17::IntEvent0ImaskDio17Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio17_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio17::IntEvent0ImaskDio17Set)
    }
}
#[doc = "DIO18 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio18 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio18Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio18Set = 1,
}
impl From<IntEvent0ImaskDio18> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO18` reader - DIO18 event mask"]
pub type IntEvent0ImaskDio18R = crate::BitReader<IntEvent0ImaskDio18>;
impl IntEvent0ImaskDio18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio18 {
        match self.bits {
            false => IntEvent0ImaskDio18::IntEvent0ImaskDio18Clr,
            true => IntEvent0ImaskDio18::IntEvent0ImaskDio18Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio18_clr(&self) -> bool {
        *self == IntEvent0ImaskDio18::IntEvent0ImaskDio18Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio18_set(&self) -> bool {
        *self == IntEvent0ImaskDio18::IntEvent0ImaskDio18Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO18` writer - DIO18 event mask"]
pub type IntEvent0ImaskDio18W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio18>;
impl<'a, REG> IntEvent0ImaskDio18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio18_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio18::IntEvent0ImaskDio18Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio18_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio18::IntEvent0ImaskDio18Set)
    }
}
#[doc = "DIO19 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio19 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio19Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio19Set = 1,
}
impl From<IntEvent0ImaskDio19> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO19` reader - DIO19 event mask"]
pub type IntEvent0ImaskDio19R = crate::BitReader<IntEvent0ImaskDio19>;
impl IntEvent0ImaskDio19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio19 {
        match self.bits {
            false => IntEvent0ImaskDio19::IntEvent0ImaskDio19Clr,
            true => IntEvent0ImaskDio19::IntEvent0ImaskDio19Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio19_clr(&self) -> bool {
        *self == IntEvent0ImaskDio19::IntEvent0ImaskDio19Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio19_set(&self) -> bool {
        *self == IntEvent0ImaskDio19::IntEvent0ImaskDio19Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO19` writer - DIO19 event mask"]
pub type IntEvent0ImaskDio19W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio19>;
impl<'a, REG> IntEvent0ImaskDio19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio19_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio19::IntEvent0ImaskDio19Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio19_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio19::IntEvent0ImaskDio19Set)
    }
}
#[doc = "DIO20 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio20 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio20Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio20Set = 1,
}
impl From<IntEvent0ImaskDio20> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO20` reader - DIO20 event mask"]
pub type IntEvent0ImaskDio20R = crate::BitReader<IntEvent0ImaskDio20>;
impl IntEvent0ImaskDio20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio20 {
        match self.bits {
            false => IntEvent0ImaskDio20::IntEvent0ImaskDio20Clr,
            true => IntEvent0ImaskDio20::IntEvent0ImaskDio20Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio20_clr(&self) -> bool {
        *self == IntEvent0ImaskDio20::IntEvent0ImaskDio20Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio20_set(&self) -> bool {
        *self == IntEvent0ImaskDio20::IntEvent0ImaskDio20Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO20` writer - DIO20 event mask"]
pub type IntEvent0ImaskDio20W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio20>;
impl<'a, REG> IntEvent0ImaskDio20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio20_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio20::IntEvent0ImaskDio20Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio20_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio20::IntEvent0ImaskDio20Set)
    }
}
#[doc = "DIO21 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio21 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio21Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio21Set = 1,
}
impl From<IntEvent0ImaskDio21> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO21` reader - DIO21 event mask"]
pub type IntEvent0ImaskDio21R = crate::BitReader<IntEvent0ImaskDio21>;
impl IntEvent0ImaskDio21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio21 {
        match self.bits {
            false => IntEvent0ImaskDio21::IntEvent0ImaskDio21Clr,
            true => IntEvent0ImaskDio21::IntEvent0ImaskDio21Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio21_clr(&self) -> bool {
        *self == IntEvent0ImaskDio21::IntEvent0ImaskDio21Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio21_set(&self) -> bool {
        *self == IntEvent0ImaskDio21::IntEvent0ImaskDio21Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO21` writer - DIO21 event mask"]
pub type IntEvent0ImaskDio21W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio21>;
impl<'a, REG> IntEvent0ImaskDio21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio21_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio21::IntEvent0ImaskDio21Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio21_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio21::IntEvent0ImaskDio21Set)
    }
}
#[doc = "DIO22 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio22 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio22Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio22Set = 1,
}
impl From<IntEvent0ImaskDio22> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO22` reader - DIO22 event mask"]
pub type IntEvent0ImaskDio22R = crate::BitReader<IntEvent0ImaskDio22>;
impl IntEvent0ImaskDio22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio22 {
        match self.bits {
            false => IntEvent0ImaskDio22::IntEvent0ImaskDio22Clr,
            true => IntEvent0ImaskDio22::IntEvent0ImaskDio22Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio22_clr(&self) -> bool {
        *self == IntEvent0ImaskDio22::IntEvent0ImaskDio22Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio22_set(&self) -> bool {
        *self == IntEvent0ImaskDio22::IntEvent0ImaskDio22Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO22` writer - DIO22 event mask"]
pub type IntEvent0ImaskDio22W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio22>;
impl<'a, REG> IntEvent0ImaskDio22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio22_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio22::IntEvent0ImaskDio22Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio22_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio22::IntEvent0ImaskDio22Set)
    }
}
#[doc = "DIO23 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio23 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio23Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio23Set = 1,
}
impl From<IntEvent0ImaskDio23> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO23` reader - DIO23 event mask"]
pub type IntEvent0ImaskDio23R = crate::BitReader<IntEvent0ImaskDio23>;
impl IntEvent0ImaskDio23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio23 {
        match self.bits {
            false => IntEvent0ImaskDio23::IntEvent0ImaskDio23Clr,
            true => IntEvent0ImaskDio23::IntEvent0ImaskDio23Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio23_clr(&self) -> bool {
        *self == IntEvent0ImaskDio23::IntEvent0ImaskDio23Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio23_set(&self) -> bool {
        *self == IntEvent0ImaskDio23::IntEvent0ImaskDio23Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO23` writer - DIO23 event mask"]
pub type IntEvent0ImaskDio23W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio23>;
impl<'a, REG> IntEvent0ImaskDio23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio23_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio23::IntEvent0ImaskDio23Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio23_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio23::IntEvent0ImaskDio23Set)
    }
}
#[doc = "DIO24 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio24 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio24Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio24Set = 1,
}
impl From<IntEvent0ImaskDio24> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO24` reader - DIO24 event mask"]
pub type IntEvent0ImaskDio24R = crate::BitReader<IntEvent0ImaskDio24>;
impl IntEvent0ImaskDio24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio24 {
        match self.bits {
            false => IntEvent0ImaskDio24::IntEvent0ImaskDio24Clr,
            true => IntEvent0ImaskDio24::IntEvent0ImaskDio24Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio24_clr(&self) -> bool {
        *self == IntEvent0ImaskDio24::IntEvent0ImaskDio24Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio24_set(&self) -> bool {
        *self == IntEvent0ImaskDio24::IntEvent0ImaskDio24Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO24` writer - DIO24 event mask"]
pub type IntEvent0ImaskDio24W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio24>;
impl<'a, REG> IntEvent0ImaskDio24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio24_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio24::IntEvent0ImaskDio24Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio24_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio24::IntEvent0ImaskDio24Set)
    }
}
#[doc = "DIO25 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio25 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio25Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio25Set = 1,
}
impl From<IntEvent0ImaskDio25> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO25` reader - DIO25 event mask"]
pub type IntEvent0ImaskDio25R = crate::BitReader<IntEvent0ImaskDio25>;
impl IntEvent0ImaskDio25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio25 {
        match self.bits {
            false => IntEvent0ImaskDio25::IntEvent0ImaskDio25Clr,
            true => IntEvent0ImaskDio25::IntEvent0ImaskDio25Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio25_clr(&self) -> bool {
        *self == IntEvent0ImaskDio25::IntEvent0ImaskDio25Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio25_set(&self) -> bool {
        *self == IntEvent0ImaskDio25::IntEvent0ImaskDio25Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO25` writer - DIO25 event mask"]
pub type IntEvent0ImaskDio25W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio25>;
impl<'a, REG> IntEvent0ImaskDio25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio25_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio25::IntEvent0ImaskDio25Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio25_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio25::IntEvent0ImaskDio25Set)
    }
}
#[doc = "DIO26 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio26 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio26Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio26Set = 1,
}
impl From<IntEvent0ImaskDio26> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO26` reader - DIO26 event mask"]
pub type IntEvent0ImaskDio26R = crate::BitReader<IntEvent0ImaskDio26>;
impl IntEvent0ImaskDio26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio26 {
        match self.bits {
            false => IntEvent0ImaskDio26::IntEvent0ImaskDio26Clr,
            true => IntEvent0ImaskDio26::IntEvent0ImaskDio26Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio26_clr(&self) -> bool {
        *self == IntEvent0ImaskDio26::IntEvent0ImaskDio26Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio26_set(&self) -> bool {
        *self == IntEvent0ImaskDio26::IntEvent0ImaskDio26Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO26` writer - DIO26 event mask"]
pub type IntEvent0ImaskDio26W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio26>;
impl<'a, REG> IntEvent0ImaskDio26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio26_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio26::IntEvent0ImaskDio26Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio26_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio26::IntEvent0ImaskDio26Set)
    }
}
#[doc = "DIO27 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio27 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio27Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio27Set = 1,
}
impl From<IntEvent0ImaskDio27> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO27` reader - DIO27 event mask"]
pub type IntEvent0ImaskDio27R = crate::BitReader<IntEvent0ImaskDio27>;
impl IntEvent0ImaskDio27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio27 {
        match self.bits {
            false => IntEvent0ImaskDio27::IntEvent0ImaskDio27Clr,
            true => IntEvent0ImaskDio27::IntEvent0ImaskDio27Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio27_clr(&self) -> bool {
        *self == IntEvent0ImaskDio27::IntEvent0ImaskDio27Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio27_set(&self) -> bool {
        *self == IntEvent0ImaskDio27::IntEvent0ImaskDio27Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO27` writer - DIO27 event mask"]
pub type IntEvent0ImaskDio27W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio27>;
impl<'a, REG> IntEvent0ImaskDio27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio27_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio27::IntEvent0ImaskDio27Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio27_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio27::IntEvent0ImaskDio27Set)
    }
}
#[doc = "DIO28 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio28 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio28Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio28Set = 1,
}
impl From<IntEvent0ImaskDio28> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO28` reader - DIO28 event mask"]
pub type IntEvent0ImaskDio28R = crate::BitReader<IntEvent0ImaskDio28>;
impl IntEvent0ImaskDio28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio28 {
        match self.bits {
            false => IntEvent0ImaskDio28::IntEvent0ImaskDio28Clr,
            true => IntEvent0ImaskDio28::IntEvent0ImaskDio28Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio28_clr(&self) -> bool {
        *self == IntEvent0ImaskDio28::IntEvent0ImaskDio28Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio28_set(&self) -> bool {
        *self == IntEvent0ImaskDio28::IntEvent0ImaskDio28Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO28` writer - DIO28 event mask"]
pub type IntEvent0ImaskDio28W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio28>;
impl<'a, REG> IntEvent0ImaskDio28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio28_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio28::IntEvent0ImaskDio28Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio28_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio28::IntEvent0ImaskDio28Set)
    }
}
#[doc = "DIO29 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio29 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio29Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio29Set = 1,
}
impl From<IntEvent0ImaskDio29> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO29` reader - DIO29 event mask"]
pub type IntEvent0ImaskDio29R = crate::BitReader<IntEvent0ImaskDio29>;
impl IntEvent0ImaskDio29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio29 {
        match self.bits {
            false => IntEvent0ImaskDio29::IntEvent0ImaskDio29Clr,
            true => IntEvent0ImaskDio29::IntEvent0ImaskDio29Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio29_clr(&self) -> bool {
        *self == IntEvent0ImaskDio29::IntEvent0ImaskDio29Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio29_set(&self) -> bool {
        *self == IntEvent0ImaskDio29::IntEvent0ImaskDio29Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO29` writer - DIO29 event mask"]
pub type IntEvent0ImaskDio29W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio29>;
impl<'a, REG> IntEvent0ImaskDio29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio29_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio29::IntEvent0ImaskDio29Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio29_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio29::IntEvent0ImaskDio29Set)
    }
}
#[doc = "DIO30 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio30 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio30Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio30Set = 1,
}
impl From<IntEvent0ImaskDio30> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO30` reader - DIO30 event mask"]
pub type IntEvent0ImaskDio30R = crate::BitReader<IntEvent0ImaskDio30>;
impl IntEvent0ImaskDio30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio30 {
        match self.bits {
            false => IntEvent0ImaskDio30::IntEvent0ImaskDio30Clr,
            true => IntEvent0ImaskDio30::IntEvent0ImaskDio30Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio30_clr(&self) -> bool {
        *self == IntEvent0ImaskDio30::IntEvent0ImaskDio30Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio30_set(&self) -> bool {
        *self == IntEvent0ImaskDio30::IntEvent0ImaskDio30Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO30` writer - DIO30 event mask"]
pub type IntEvent0ImaskDio30W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio30>;
impl<'a, REG> IntEvent0ImaskDio30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio30_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio30::IntEvent0ImaskDio30Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio30_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio30::IntEvent0ImaskDio30Set)
    }
}
#[doc = "DIO31 event mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent0ImaskDio31 {
    #[doc = "0: CLR"]
    IntEvent0ImaskDio31Clr = 0,
    #[doc = "1: SET"]
    IntEvent0ImaskDio31Set = 1,
}
impl From<IntEvent0ImaskDio31> for bool {
    #[inline(always)]
    fn from(variant: IntEvent0ImaskDio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO31` reader - DIO31 event mask"]
pub type IntEvent0ImaskDio31R = crate::BitReader<IntEvent0ImaskDio31>;
impl IntEvent0ImaskDio31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent0ImaskDio31 {
        match self.bits {
            false => IntEvent0ImaskDio31::IntEvent0ImaskDio31Clr,
            true => IntEvent0ImaskDio31::IntEvent0ImaskDio31Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio31_clr(&self) -> bool {
        *self == IntEvent0ImaskDio31::IntEvent0ImaskDio31Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event0_imask_dio31_set(&self) -> bool {
        *self == IntEvent0ImaskDio31::IntEvent0ImaskDio31Set
    }
}
#[doc = "Field `INT_EVENT0_IMASK_DIO31` writer - DIO31 event mask"]
pub type IntEvent0ImaskDio31W<'a, REG> = crate::BitWriter<'a, REG, IntEvent0ImaskDio31>;
impl<'a, REG> IntEvent0ImaskDio31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn int_event0_imask_dio31_clr(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio31::IntEvent0ImaskDio31Clr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn int_event0_imask_dio31_set(self) -> &'a mut crate::W<REG> {
        self.variant(IntEvent0ImaskDio31::IntEvent0ImaskDio31Set)
    }
}
impl R {
    #[doc = "Bit 0 - DIO0 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio0(&self) -> IntEvent0ImaskDio0R {
        IntEvent0ImaskDio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DIO1 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio1(&self) -> IntEvent0ImaskDio1R {
        IntEvent0ImaskDio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DIO2 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio2(&self) -> IntEvent0ImaskDio2R {
        IntEvent0ImaskDio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DIO3 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio3(&self) -> IntEvent0ImaskDio3R {
        IntEvent0ImaskDio3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DIO4 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio4(&self) -> IntEvent0ImaskDio4R {
        IntEvent0ImaskDio4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DIO5 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio5(&self) -> IntEvent0ImaskDio5R {
        IntEvent0ImaskDio5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DIO6 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio6(&self) -> IntEvent0ImaskDio6R {
        IntEvent0ImaskDio6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DIO7 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio7(&self) -> IntEvent0ImaskDio7R {
        IntEvent0ImaskDio7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DIO8 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio8(&self) -> IntEvent0ImaskDio8R {
        IntEvent0ImaskDio8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DIO9 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio9(&self) -> IntEvent0ImaskDio9R {
        IntEvent0ImaskDio9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DIO10 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio10(&self) -> IntEvent0ImaskDio10R {
        IntEvent0ImaskDio10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DIO11 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio11(&self) -> IntEvent0ImaskDio11R {
        IntEvent0ImaskDio11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DIO12 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio12(&self) -> IntEvent0ImaskDio12R {
        IntEvent0ImaskDio12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DIO13 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio13(&self) -> IntEvent0ImaskDio13R {
        IntEvent0ImaskDio13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DIO14 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio14(&self) -> IntEvent0ImaskDio14R {
        IntEvent0ImaskDio14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DIO15 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio15(&self) -> IntEvent0ImaskDio15R {
        IntEvent0ImaskDio15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DIO16 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio16(&self) -> IntEvent0ImaskDio16R {
        IntEvent0ImaskDio16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DIO17 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio17(&self) -> IntEvent0ImaskDio17R {
        IntEvent0ImaskDio17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DIO18 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio18(&self) -> IntEvent0ImaskDio18R {
        IntEvent0ImaskDio18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DIO19 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio19(&self) -> IntEvent0ImaskDio19R {
        IntEvent0ImaskDio19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DIO20 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio20(&self) -> IntEvent0ImaskDio20R {
        IntEvent0ImaskDio20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DIO21 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio21(&self) -> IntEvent0ImaskDio21R {
        IntEvent0ImaskDio21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DIO22 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio22(&self) -> IntEvent0ImaskDio22R {
        IntEvent0ImaskDio22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DIO23 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio23(&self) -> IntEvent0ImaskDio23R {
        IntEvent0ImaskDio23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DIO24 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio24(&self) -> IntEvent0ImaskDio24R {
        IntEvent0ImaskDio24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DIO25 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio25(&self) -> IntEvent0ImaskDio25R {
        IntEvent0ImaskDio25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DIO26 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio26(&self) -> IntEvent0ImaskDio26R {
        IntEvent0ImaskDio26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DIO27 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio27(&self) -> IntEvent0ImaskDio27R {
        IntEvent0ImaskDio27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DIO28 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio28(&self) -> IntEvent0ImaskDio28R {
        IntEvent0ImaskDio28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DIO29 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio29(&self) -> IntEvent0ImaskDio29R {
        IntEvent0ImaskDio29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DIO30 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio30(&self) -> IntEvent0ImaskDio30R {
        IntEvent0ImaskDio30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DIO31 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio31(&self) -> IntEvent0ImaskDio31R {
        IntEvent0ImaskDio31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIO0 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio0(&mut self) -> IntEvent0ImaskDio0W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio0W::new(self, 0)
    }
    #[doc = "Bit 1 - DIO1 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio1(&mut self) -> IntEvent0ImaskDio1W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio1W::new(self, 1)
    }
    #[doc = "Bit 2 - DIO2 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio2(&mut self) -> IntEvent0ImaskDio2W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio2W::new(self, 2)
    }
    #[doc = "Bit 3 - DIO3 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio3(&mut self) -> IntEvent0ImaskDio3W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio3W::new(self, 3)
    }
    #[doc = "Bit 4 - DIO4 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio4(&mut self) -> IntEvent0ImaskDio4W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio4W::new(self, 4)
    }
    #[doc = "Bit 5 - DIO5 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio5(&mut self) -> IntEvent0ImaskDio5W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio5W::new(self, 5)
    }
    #[doc = "Bit 6 - DIO6 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio6(&mut self) -> IntEvent0ImaskDio6W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio6W::new(self, 6)
    }
    #[doc = "Bit 7 - DIO7 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio7(&mut self) -> IntEvent0ImaskDio7W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio7W::new(self, 7)
    }
    #[doc = "Bit 8 - DIO8 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio8(&mut self) -> IntEvent0ImaskDio8W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio8W::new(self, 8)
    }
    #[doc = "Bit 9 - DIO9 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio9(&mut self) -> IntEvent0ImaskDio9W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio9W::new(self, 9)
    }
    #[doc = "Bit 10 - DIO10 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio10(&mut self) -> IntEvent0ImaskDio10W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio10W::new(self, 10)
    }
    #[doc = "Bit 11 - DIO11 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio11(&mut self) -> IntEvent0ImaskDio11W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio11W::new(self, 11)
    }
    #[doc = "Bit 12 - DIO12 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio12(&mut self) -> IntEvent0ImaskDio12W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio12W::new(self, 12)
    }
    #[doc = "Bit 13 - DIO13 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio13(&mut self) -> IntEvent0ImaskDio13W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio13W::new(self, 13)
    }
    #[doc = "Bit 14 - DIO14 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio14(&mut self) -> IntEvent0ImaskDio14W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio14W::new(self, 14)
    }
    #[doc = "Bit 15 - DIO15 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio15(&mut self) -> IntEvent0ImaskDio15W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio15W::new(self, 15)
    }
    #[doc = "Bit 16 - DIO16 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio16(&mut self) -> IntEvent0ImaskDio16W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio16W::new(self, 16)
    }
    #[doc = "Bit 17 - DIO17 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio17(&mut self) -> IntEvent0ImaskDio17W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio17W::new(self, 17)
    }
    #[doc = "Bit 18 - DIO18 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio18(&mut self) -> IntEvent0ImaskDio18W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio18W::new(self, 18)
    }
    #[doc = "Bit 19 - DIO19 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio19(&mut self) -> IntEvent0ImaskDio19W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio19W::new(self, 19)
    }
    #[doc = "Bit 20 - DIO20 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio20(&mut self) -> IntEvent0ImaskDio20W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio20W::new(self, 20)
    }
    #[doc = "Bit 21 - DIO21 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio21(&mut self) -> IntEvent0ImaskDio21W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio21W::new(self, 21)
    }
    #[doc = "Bit 22 - DIO22 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio22(&mut self) -> IntEvent0ImaskDio22W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio22W::new(self, 22)
    }
    #[doc = "Bit 23 - DIO23 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio23(&mut self) -> IntEvent0ImaskDio23W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio23W::new(self, 23)
    }
    #[doc = "Bit 24 - DIO24 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio24(&mut self) -> IntEvent0ImaskDio24W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio24W::new(self, 24)
    }
    #[doc = "Bit 25 - DIO25 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio25(&mut self) -> IntEvent0ImaskDio25W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio25W::new(self, 25)
    }
    #[doc = "Bit 26 - DIO26 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio26(&mut self) -> IntEvent0ImaskDio26W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio26W::new(self, 26)
    }
    #[doc = "Bit 27 - DIO27 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio27(&mut self) -> IntEvent0ImaskDio27W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio27W::new(self, 27)
    }
    #[doc = "Bit 28 - DIO28 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio28(&mut self) -> IntEvent0ImaskDio28W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio28W::new(self, 28)
    }
    #[doc = "Bit 29 - DIO29 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio29(&mut self) -> IntEvent0ImaskDio29W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio29W::new(self, 29)
    }
    #[doc = "Bit 30 - DIO30 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio30(&mut self) -> IntEvent0ImaskDio30W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio30W::new(self, 30)
    }
    #[doc = "Bit 31 - DIO31 event mask"]
    #[inline(always)]
    pub fn int_event0_imask_dio31(&mut self) -> IntEvent0ImaskDio31W<IntEvent0ImaskSpec> {
        IntEvent0ImaskDio31W::new(self, 31)
    }
}
#[doc = "Interrupt mask\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_imask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_event0_imask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0ImaskSpec;
impl crate::RegisterSpec for IntEvent0ImaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event0_imask::R`](R) reader structure"]
impl crate::Readable for IntEvent0ImaskSpec {}
#[doc = "`write(|w| ..)` method takes [`int_event0_imask::W`](W) writer structure"]
impl crate::Writable for IntEvent0ImaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_EVENT0_IMASK to value 0"]
impl crate::Resettable for IntEvent0ImaskSpec {
    const RESET_VALUE: u32 = 0;
}
