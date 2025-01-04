#[doc = "Register `INT_EVENT2_RIS` reader"]
pub type R = crate::R<IntEvent2RisSpec>;
#[doc = "DIO16 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisDio16 {
    #[doc = "0: CLR"]
    IntEvent2RisDio16Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisDio16Set = 1,
}
impl From<IntEvent2RisDio16> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisDio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_DIO16` reader - DIO16 event"]
pub type IntEvent2RisDio16R = crate::BitReader<IntEvent2RisDio16>;
impl IntEvent2RisDio16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisDio16 {
        match self.bits {
            false => IntEvent2RisDio16::IntEvent2RisDio16Clr,
            true => IntEvent2RisDio16::IntEvent2RisDio16Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio16_clr(&self) -> bool {
        *self == IntEvent2RisDio16::IntEvent2RisDio16Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio16_set(&self) -> bool {
        *self == IntEvent2RisDio16::IntEvent2RisDio16Set
    }
}
#[doc = "DIO17 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisDio17 {
    #[doc = "0: CLR"]
    IntEvent2RisDio17Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisDio17Set = 1,
}
impl From<IntEvent2RisDio17> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisDio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_DIO17` reader - DIO17 event"]
pub type IntEvent2RisDio17R = crate::BitReader<IntEvent2RisDio17>;
impl IntEvent2RisDio17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisDio17 {
        match self.bits {
            false => IntEvent2RisDio17::IntEvent2RisDio17Clr,
            true => IntEvent2RisDio17::IntEvent2RisDio17Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio17_clr(&self) -> bool {
        *self == IntEvent2RisDio17::IntEvent2RisDio17Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio17_set(&self) -> bool {
        *self == IntEvent2RisDio17::IntEvent2RisDio17Set
    }
}
#[doc = "DIO18 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisDio18 {
    #[doc = "0: CLR"]
    IntEvent2RisDio18Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisDio18Set = 1,
}
impl From<IntEvent2RisDio18> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisDio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_DIO18` reader - DIO18 event"]
pub type IntEvent2RisDio18R = crate::BitReader<IntEvent2RisDio18>;
impl IntEvent2RisDio18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisDio18 {
        match self.bits {
            false => IntEvent2RisDio18::IntEvent2RisDio18Clr,
            true => IntEvent2RisDio18::IntEvent2RisDio18Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio18_clr(&self) -> bool {
        *self == IntEvent2RisDio18::IntEvent2RisDio18Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio18_set(&self) -> bool {
        *self == IntEvent2RisDio18::IntEvent2RisDio18Set
    }
}
#[doc = "DIO19 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisDio19 {
    #[doc = "0: CLR"]
    IntEvent2RisDio19Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisDio19Set = 1,
}
impl From<IntEvent2RisDio19> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisDio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_DIO19` reader - DIO19 event"]
pub type IntEvent2RisDio19R = crate::BitReader<IntEvent2RisDio19>;
impl IntEvent2RisDio19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisDio19 {
        match self.bits {
            false => IntEvent2RisDio19::IntEvent2RisDio19Clr,
            true => IntEvent2RisDio19::IntEvent2RisDio19Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio19_clr(&self) -> bool {
        *self == IntEvent2RisDio19::IntEvent2RisDio19Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio19_set(&self) -> bool {
        *self == IntEvent2RisDio19::IntEvent2RisDio19Set
    }
}
#[doc = "DIO20 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisDio20 {
    #[doc = "0: CLR"]
    IntEvent2RisDio20Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisDio20Set = 1,
}
impl From<IntEvent2RisDio20> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisDio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_DIO20` reader - DIO20 event"]
pub type IntEvent2RisDio20R = crate::BitReader<IntEvent2RisDio20>;
impl IntEvent2RisDio20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisDio20 {
        match self.bits {
            false => IntEvent2RisDio20::IntEvent2RisDio20Clr,
            true => IntEvent2RisDio20::IntEvent2RisDio20Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio20_clr(&self) -> bool {
        *self == IntEvent2RisDio20::IntEvent2RisDio20Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio20_set(&self) -> bool {
        *self == IntEvent2RisDio20::IntEvent2RisDio20Set
    }
}
#[doc = "DIO21 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisDio21 {
    #[doc = "0: CLR"]
    IntEvent2RisDio21Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisDio21Set = 1,
}
impl From<IntEvent2RisDio21> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisDio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_DIO21` reader - DIO21 event"]
pub type IntEvent2RisDio21R = crate::BitReader<IntEvent2RisDio21>;
impl IntEvent2RisDio21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisDio21 {
        match self.bits {
            false => IntEvent2RisDio21::IntEvent2RisDio21Clr,
            true => IntEvent2RisDio21::IntEvent2RisDio21Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio21_clr(&self) -> bool {
        *self == IntEvent2RisDio21::IntEvent2RisDio21Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio21_set(&self) -> bool {
        *self == IntEvent2RisDio21::IntEvent2RisDio21Set
    }
}
#[doc = "DIO22 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisDio22 {
    #[doc = "0: CLR"]
    IntEvent2RisDio22Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisDio22Set = 1,
}
impl From<IntEvent2RisDio22> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisDio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_DIO22` reader - DIO22 event"]
pub type IntEvent2RisDio22R = crate::BitReader<IntEvent2RisDio22>;
impl IntEvent2RisDio22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisDio22 {
        match self.bits {
            false => IntEvent2RisDio22::IntEvent2RisDio22Clr,
            true => IntEvent2RisDio22::IntEvent2RisDio22Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio22_clr(&self) -> bool {
        *self == IntEvent2RisDio22::IntEvent2RisDio22Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio22_set(&self) -> bool {
        *self == IntEvent2RisDio22::IntEvent2RisDio22Set
    }
}
#[doc = "DIO23 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisDio23 {
    #[doc = "0: CLR"]
    IntEvent2RisDio23Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisDio23Set = 1,
}
impl From<IntEvent2RisDio23> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisDio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_DIO23` reader - DIO23 event"]
pub type IntEvent2RisDio23R = crate::BitReader<IntEvent2RisDio23>;
impl IntEvent2RisDio23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisDio23 {
        match self.bits {
            false => IntEvent2RisDio23::IntEvent2RisDio23Clr,
            true => IntEvent2RisDio23::IntEvent2RisDio23Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio23_clr(&self) -> bool {
        *self == IntEvent2RisDio23::IntEvent2RisDio23Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio23_set(&self) -> bool {
        *self == IntEvent2RisDio23::IntEvent2RisDio23Set
    }
}
#[doc = "DIO24 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisDio24 {
    #[doc = "0: CLR"]
    IntEvent2RisDio24Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisDio24Set = 1,
}
impl From<IntEvent2RisDio24> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisDio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_DIO24` reader - DIO24 event"]
pub type IntEvent2RisDio24R = crate::BitReader<IntEvent2RisDio24>;
impl IntEvent2RisDio24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisDio24 {
        match self.bits {
            false => IntEvent2RisDio24::IntEvent2RisDio24Clr,
            true => IntEvent2RisDio24::IntEvent2RisDio24Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio24_clr(&self) -> bool {
        *self == IntEvent2RisDio24::IntEvent2RisDio24Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio24_set(&self) -> bool {
        *self == IntEvent2RisDio24::IntEvent2RisDio24Set
    }
}
#[doc = "DIO25 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisDio25 {
    #[doc = "0: CLR"]
    IntEvent2RisDio25Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisDio25Set = 1,
}
impl From<IntEvent2RisDio25> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisDio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_DIO25` reader - DIO25 event"]
pub type IntEvent2RisDio25R = crate::BitReader<IntEvent2RisDio25>;
impl IntEvent2RisDio25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisDio25 {
        match self.bits {
            false => IntEvent2RisDio25::IntEvent2RisDio25Clr,
            true => IntEvent2RisDio25::IntEvent2RisDio25Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio25_clr(&self) -> bool {
        *self == IntEvent2RisDio25::IntEvent2RisDio25Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio25_set(&self) -> bool {
        *self == IntEvent2RisDio25::IntEvent2RisDio25Set
    }
}
#[doc = "DIO26 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisDio26 {
    #[doc = "0: CLR"]
    IntEvent2RisDio26Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisDio26Set = 1,
}
impl From<IntEvent2RisDio26> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisDio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_DIO26` reader - DIO26 event"]
pub type IntEvent2RisDio26R = crate::BitReader<IntEvent2RisDio26>;
impl IntEvent2RisDio26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisDio26 {
        match self.bits {
            false => IntEvent2RisDio26::IntEvent2RisDio26Clr,
            true => IntEvent2RisDio26::IntEvent2RisDio26Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio26_clr(&self) -> bool {
        *self == IntEvent2RisDio26::IntEvent2RisDio26Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio26_set(&self) -> bool {
        *self == IntEvent2RisDio26::IntEvent2RisDio26Set
    }
}
#[doc = "DIO27 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisDio27 {
    #[doc = "0: CLR"]
    IntEvent2RisDio27Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisDio27Set = 1,
}
impl From<IntEvent2RisDio27> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisDio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_DIO27` reader - DIO27 event"]
pub type IntEvent2RisDio27R = crate::BitReader<IntEvent2RisDio27>;
impl IntEvent2RisDio27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisDio27 {
        match self.bits {
            false => IntEvent2RisDio27::IntEvent2RisDio27Clr,
            true => IntEvent2RisDio27::IntEvent2RisDio27Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio27_clr(&self) -> bool {
        *self == IntEvent2RisDio27::IntEvent2RisDio27Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio27_set(&self) -> bool {
        *self == IntEvent2RisDio27::IntEvent2RisDio27Set
    }
}
#[doc = "DIO28 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisDio28 {
    #[doc = "0: CLR"]
    IntEvent2RisDio28Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisDio28Set = 1,
}
impl From<IntEvent2RisDio28> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisDio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_DIO28` reader - DIO28 event"]
pub type IntEvent2RisDio28R = crate::BitReader<IntEvent2RisDio28>;
impl IntEvent2RisDio28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisDio28 {
        match self.bits {
            false => IntEvent2RisDio28::IntEvent2RisDio28Clr,
            true => IntEvent2RisDio28::IntEvent2RisDio28Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio28_clr(&self) -> bool {
        *self == IntEvent2RisDio28::IntEvent2RisDio28Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio28_set(&self) -> bool {
        *self == IntEvent2RisDio28::IntEvent2RisDio28Set
    }
}
#[doc = "DIO29 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisDio29 {
    #[doc = "0: CLR"]
    IntEvent2RisDio29Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisDio29Set = 1,
}
impl From<IntEvent2RisDio29> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisDio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_DIO29` reader - DIO29 event"]
pub type IntEvent2RisDio29R = crate::BitReader<IntEvent2RisDio29>;
impl IntEvent2RisDio29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisDio29 {
        match self.bits {
            false => IntEvent2RisDio29::IntEvent2RisDio29Clr,
            true => IntEvent2RisDio29::IntEvent2RisDio29Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio29_clr(&self) -> bool {
        *self == IntEvent2RisDio29::IntEvent2RisDio29Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio29_set(&self) -> bool {
        *self == IntEvent2RisDio29::IntEvent2RisDio29Set
    }
}
#[doc = "DIO30 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisDio30 {
    #[doc = "0: CLR"]
    IntEvent2RisDio30Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisDio30Set = 1,
}
impl From<IntEvent2RisDio30> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisDio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_DIO30` reader - DIO30 event"]
pub type IntEvent2RisDio30R = crate::BitReader<IntEvent2RisDio30>;
impl IntEvent2RisDio30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisDio30 {
        match self.bits {
            false => IntEvent2RisDio30::IntEvent2RisDio30Clr,
            true => IntEvent2RisDio30::IntEvent2RisDio30Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio30_clr(&self) -> bool {
        *self == IntEvent2RisDio30::IntEvent2RisDio30Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio30_set(&self) -> bool {
        *self == IntEvent2RisDio30::IntEvent2RisDio30Set
    }
}
#[doc = "DIO31 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2RisDio31 {
    #[doc = "0: CLR"]
    IntEvent2RisDio31Clr = 0,
    #[doc = "1: SET"]
    IntEvent2RisDio31Set = 1,
}
impl From<IntEvent2RisDio31> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2RisDio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_RIS_DIO31` reader - DIO31 event"]
pub type IntEvent2RisDio31R = crate::BitReader<IntEvent2RisDio31>;
impl IntEvent2RisDio31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2RisDio31 {
        match self.bits {
            false => IntEvent2RisDio31::IntEvent2RisDio31Clr,
            true => IntEvent2RisDio31::IntEvent2RisDio31Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio31_clr(&self) -> bool {
        *self == IntEvent2RisDio31::IntEvent2RisDio31Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_ris_dio31_set(&self) -> bool {
        *self == IntEvent2RisDio31::IntEvent2RisDio31Set
    }
}
impl R {
    #[doc = "Bit 16 - DIO16 event"]
    #[inline(always)]
    pub fn int_event2_ris_dio16(&self) -> IntEvent2RisDio16R {
        IntEvent2RisDio16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DIO17 event"]
    #[inline(always)]
    pub fn int_event2_ris_dio17(&self) -> IntEvent2RisDio17R {
        IntEvent2RisDio17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DIO18 event"]
    #[inline(always)]
    pub fn int_event2_ris_dio18(&self) -> IntEvent2RisDio18R {
        IntEvent2RisDio18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DIO19 event"]
    #[inline(always)]
    pub fn int_event2_ris_dio19(&self) -> IntEvent2RisDio19R {
        IntEvent2RisDio19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DIO20 event"]
    #[inline(always)]
    pub fn int_event2_ris_dio20(&self) -> IntEvent2RisDio20R {
        IntEvent2RisDio20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DIO21 event"]
    #[inline(always)]
    pub fn int_event2_ris_dio21(&self) -> IntEvent2RisDio21R {
        IntEvent2RisDio21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DIO22 event"]
    #[inline(always)]
    pub fn int_event2_ris_dio22(&self) -> IntEvent2RisDio22R {
        IntEvent2RisDio22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DIO23 event"]
    #[inline(always)]
    pub fn int_event2_ris_dio23(&self) -> IntEvent2RisDio23R {
        IntEvent2RisDio23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DIO24 event"]
    #[inline(always)]
    pub fn int_event2_ris_dio24(&self) -> IntEvent2RisDio24R {
        IntEvent2RisDio24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DIO25 event"]
    #[inline(always)]
    pub fn int_event2_ris_dio25(&self) -> IntEvent2RisDio25R {
        IntEvent2RisDio25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DIO26 event"]
    #[inline(always)]
    pub fn int_event2_ris_dio26(&self) -> IntEvent2RisDio26R {
        IntEvent2RisDio26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DIO27 event"]
    #[inline(always)]
    pub fn int_event2_ris_dio27(&self) -> IntEvent2RisDio27R {
        IntEvent2RisDio27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DIO28 event"]
    #[inline(always)]
    pub fn int_event2_ris_dio28(&self) -> IntEvent2RisDio28R {
        IntEvent2RisDio28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DIO29 event"]
    #[inline(always)]
    pub fn int_event2_ris_dio29(&self) -> IntEvent2RisDio29R {
        IntEvent2RisDio29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DIO30 event"]
    #[inline(always)]
    pub fn int_event2_ris_dio30(&self) -> IntEvent2RisDio30R {
        IntEvent2RisDio30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DIO31 event"]
    #[inline(always)]
    pub fn int_event2_ris_dio31(&self) -> IntEvent2RisDio31R {
        IntEvent2RisDio31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2RisSpec;
impl crate::RegisterSpec for IntEvent2RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event2_ris::R`](R) reader structure"]
impl crate::Readable for IntEvent2RisSpec {}
#[doc = "`reset()` method sets INT_EVENT2_RIS to value 0"]
impl crate::Resettable for IntEvent2RisSpec {
    const RESET_VALUE: u32 = 0;
}
