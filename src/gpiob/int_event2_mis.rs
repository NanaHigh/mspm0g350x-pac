#[doc = "Register `INT_EVENT2_MIS` reader"]
pub type R = crate::R<IntEvent2MisSpec>;
#[doc = "DIO16 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisDio16 {
    #[doc = "0: CLR"]
    IntEvent2MisDio16Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisDio16Set = 1,
}
impl From<IntEvent2MisDio16> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisDio16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_DIO16` reader - DIO16 event"]
pub type IntEvent2MisDio16R = crate::BitReader<IntEvent2MisDio16>;
impl IntEvent2MisDio16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisDio16 {
        match self.bits {
            false => IntEvent2MisDio16::IntEvent2MisDio16Clr,
            true => IntEvent2MisDio16::IntEvent2MisDio16Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio16_clr(&self) -> bool {
        *self == IntEvent2MisDio16::IntEvent2MisDio16Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio16_set(&self) -> bool {
        *self == IntEvent2MisDio16::IntEvent2MisDio16Set
    }
}
#[doc = "DIO17 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisDio17 {
    #[doc = "0: CLR"]
    IntEvent2MisDio17Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisDio17Set = 1,
}
impl From<IntEvent2MisDio17> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisDio17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_DIO17` reader - DIO17 event"]
pub type IntEvent2MisDio17R = crate::BitReader<IntEvent2MisDio17>;
impl IntEvent2MisDio17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisDio17 {
        match self.bits {
            false => IntEvent2MisDio17::IntEvent2MisDio17Clr,
            true => IntEvent2MisDio17::IntEvent2MisDio17Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio17_clr(&self) -> bool {
        *self == IntEvent2MisDio17::IntEvent2MisDio17Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio17_set(&self) -> bool {
        *self == IntEvent2MisDio17::IntEvent2MisDio17Set
    }
}
#[doc = "DIO18 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisDio18 {
    #[doc = "0: CLR"]
    IntEvent2MisDio18Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisDio18Set = 1,
}
impl From<IntEvent2MisDio18> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisDio18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_DIO18` reader - DIO18 event"]
pub type IntEvent2MisDio18R = crate::BitReader<IntEvent2MisDio18>;
impl IntEvent2MisDio18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisDio18 {
        match self.bits {
            false => IntEvent2MisDio18::IntEvent2MisDio18Clr,
            true => IntEvent2MisDio18::IntEvent2MisDio18Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio18_clr(&self) -> bool {
        *self == IntEvent2MisDio18::IntEvent2MisDio18Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio18_set(&self) -> bool {
        *self == IntEvent2MisDio18::IntEvent2MisDio18Set
    }
}
#[doc = "DIO19 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisDio19 {
    #[doc = "0: CLR"]
    IntEvent2MisDio19Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisDio19Set = 1,
}
impl From<IntEvent2MisDio19> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisDio19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_DIO19` reader - DIO19 event"]
pub type IntEvent2MisDio19R = crate::BitReader<IntEvent2MisDio19>;
impl IntEvent2MisDio19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisDio19 {
        match self.bits {
            false => IntEvent2MisDio19::IntEvent2MisDio19Clr,
            true => IntEvent2MisDio19::IntEvent2MisDio19Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio19_clr(&self) -> bool {
        *self == IntEvent2MisDio19::IntEvent2MisDio19Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio19_set(&self) -> bool {
        *self == IntEvent2MisDio19::IntEvent2MisDio19Set
    }
}
#[doc = "DIO20 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisDio20 {
    #[doc = "0: CLR"]
    IntEvent2MisDio20Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisDio20Set = 1,
}
impl From<IntEvent2MisDio20> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisDio20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_DIO20` reader - DIO20 event"]
pub type IntEvent2MisDio20R = crate::BitReader<IntEvent2MisDio20>;
impl IntEvent2MisDio20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisDio20 {
        match self.bits {
            false => IntEvent2MisDio20::IntEvent2MisDio20Clr,
            true => IntEvent2MisDio20::IntEvent2MisDio20Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio20_clr(&self) -> bool {
        *self == IntEvent2MisDio20::IntEvent2MisDio20Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio20_set(&self) -> bool {
        *self == IntEvent2MisDio20::IntEvent2MisDio20Set
    }
}
#[doc = "DIO21 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisDio21 {
    #[doc = "0: CLR"]
    IntEvent2MisDio21Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisDio21Set = 1,
}
impl From<IntEvent2MisDio21> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisDio21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_DIO21` reader - DIO21 event"]
pub type IntEvent2MisDio21R = crate::BitReader<IntEvent2MisDio21>;
impl IntEvent2MisDio21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisDio21 {
        match self.bits {
            false => IntEvent2MisDio21::IntEvent2MisDio21Clr,
            true => IntEvent2MisDio21::IntEvent2MisDio21Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio21_clr(&self) -> bool {
        *self == IntEvent2MisDio21::IntEvent2MisDio21Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio21_set(&self) -> bool {
        *self == IntEvent2MisDio21::IntEvent2MisDio21Set
    }
}
#[doc = "DIO22 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisDio22 {
    #[doc = "0: CLR"]
    IntEvent2MisDio22Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisDio22Set = 1,
}
impl From<IntEvent2MisDio22> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisDio22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_DIO22` reader - DIO22 event"]
pub type IntEvent2MisDio22R = crate::BitReader<IntEvent2MisDio22>;
impl IntEvent2MisDio22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisDio22 {
        match self.bits {
            false => IntEvent2MisDio22::IntEvent2MisDio22Clr,
            true => IntEvent2MisDio22::IntEvent2MisDio22Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio22_clr(&self) -> bool {
        *self == IntEvent2MisDio22::IntEvent2MisDio22Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio22_set(&self) -> bool {
        *self == IntEvent2MisDio22::IntEvent2MisDio22Set
    }
}
#[doc = "DIO23 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisDio23 {
    #[doc = "0: CLR"]
    IntEvent2MisDio23Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisDio23Set = 1,
}
impl From<IntEvent2MisDio23> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisDio23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_DIO23` reader - DIO23 event"]
pub type IntEvent2MisDio23R = crate::BitReader<IntEvent2MisDio23>;
impl IntEvent2MisDio23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisDio23 {
        match self.bits {
            false => IntEvent2MisDio23::IntEvent2MisDio23Clr,
            true => IntEvent2MisDio23::IntEvent2MisDio23Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio23_clr(&self) -> bool {
        *self == IntEvent2MisDio23::IntEvent2MisDio23Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio23_set(&self) -> bool {
        *self == IntEvent2MisDio23::IntEvent2MisDio23Set
    }
}
#[doc = "DIO24 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisDio24 {
    #[doc = "0: CLR"]
    IntEvent2MisDio24Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisDio24Set = 1,
}
impl From<IntEvent2MisDio24> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisDio24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_DIO24` reader - DIO24 event"]
pub type IntEvent2MisDio24R = crate::BitReader<IntEvent2MisDio24>;
impl IntEvent2MisDio24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisDio24 {
        match self.bits {
            false => IntEvent2MisDio24::IntEvent2MisDio24Clr,
            true => IntEvent2MisDio24::IntEvent2MisDio24Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio24_clr(&self) -> bool {
        *self == IntEvent2MisDio24::IntEvent2MisDio24Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio24_set(&self) -> bool {
        *self == IntEvent2MisDio24::IntEvent2MisDio24Set
    }
}
#[doc = "DIO25 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisDio25 {
    #[doc = "0: CLR"]
    IntEvent2MisDio25Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisDio25Set = 1,
}
impl From<IntEvent2MisDio25> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisDio25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_DIO25` reader - DIO25 event"]
pub type IntEvent2MisDio25R = crate::BitReader<IntEvent2MisDio25>;
impl IntEvent2MisDio25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisDio25 {
        match self.bits {
            false => IntEvent2MisDio25::IntEvent2MisDio25Clr,
            true => IntEvent2MisDio25::IntEvent2MisDio25Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio25_clr(&self) -> bool {
        *self == IntEvent2MisDio25::IntEvent2MisDio25Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio25_set(&self) -> bool {
        *self == IntEvent2MisDio25::IntEvent2MisDio25Set
    }
}
#[doc = "DIO26 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisDio26 {
    #[doc = "0: CLR"]
    IntEvent2MisDio26Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisDio26Set = 1,
}
impl From<IntEvent2MisDio26> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisDio26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_DIO26` reader - DIO26 event"]
pub type IntEvent2MisDio26R = crate::BitReader<IntEvent2MisDio26>;
impl IntEvent2MisDio26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisDio26 {
        match self.bits {
            false => IntEvent2MisDio26::IntEvent2MisDio26Clr,
            true => IntEvent2MisDio26::IntEvent2MisDio26Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio26_clr(&self) -> bool {
        *self == IntEvent2MisDio26::IntEvent2MisDio26Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio26_set(&self) -> bool {
        *self == IntEvent2MisDio26::IntEvent2MisDio26Set
    }
}
#[doc = "DIO27 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisDio27 {
    #[doc = "0: CLR"]
    IntEvent2MisDio27Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisDio27Set = 1,
}
impl From<IntEvent2MisDio27> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisDio27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_DIO27` reader - DIO27 event"]
pub type IntEvent2MisDio27R = crate::BitReader<IntEvent2MisDio27>;
impl IntEvent2MisDio27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisDio27 {
        match self.bits {
            false => IntEvent2MisDio27::IntEvent2MisDio27Clr,
            true => IntEvent2MisDio27::IntEvent2MisDio27Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio27_clr(&self) -> bool {
        *self == IntEvent2MisDio27::IntEvent2MisDio27Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio27_set(&self) -> bool {
        *self == IntEvent2MisDio27::IntEvent2MisDio27Set
    }
}
#[doc = "DIO28 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisDio28 {
    #[doc = "0: CLR"]
    IntEvent2MisDio28Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisDio28Set = 1,
}
impl From<IntEvent2MisDio28> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisDio28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_DIO28` reader - DIO28 event"]
pub type IntEvent2MisDio28R = crate::BitReader<IntEvent2MisDio28>;
impl IntEvent2MisDio28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisDio28 {
        match self.bits {
            false => IntEvent2MisDio28::IntEvent2MisDio28Clr,
            true => IntEvent2MisDio28::IntEvent2MisDio28Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio28_clr(&self) -> bool {
        *self == IntEvent2MisDio28::IntEvent2MisDio28Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio28_set(&self) -> bool {
        *self == IntEvent2MisDio28::IntEvent2MisDio28Set
    }
}
#[doc = "DIO29 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisDio29 {
    #[doc = "0: CLR"]
    IntEvent2MisDio29Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisDio29Set = 1,
}
impl From<IntEvent2MisDio29> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisDio29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_DIO29` reader - DIO29 event"]
pub type IntEvent2MisDio29R = crate::BitReader<IntEvent2MisDio29>;
impl IntEvent2MisDio29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisDio29 {
        match self.bits {
            false => IntEvent2MisDio29::IntEvent2MisDio29Clr,
            true => IntEvent2MisDio29::IntEvent2MisDio29Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio29_clr(&self) -> bool {
        *self == IntEvent2MisDio29::IntEvent2MisDio29Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio29_set(&self) -> bool {
        *self == IntEvent2MisDio29::IntEvent2MisDio29Set
    }
}
#[doc = "DIO30 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisDio30 {
    #[doc = "0: CLR"]
    IntEvent2MisDio30Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisDio30Set = 1,
}
impl From<IntEvent2MisDio30> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisDio30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_DIO30` reader - DIO30 event"]
pub type IntEvent2MisDio30R = crate::BitReader<IntEvent2MisDio30>;
impl IntEvent2MisDio30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisDio30 {
        match self.bits {
            false => IntEvent2MisDio30::IntEvent2MisDio30Clr,
            true => IntEvent2MisDio30::IntEvent2MisDio30Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio30_clr(&self) -> bool {
        *self == IntEvent2MisDio30::IntEvent2MisDio30Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio30_set(&self) -> bool {
        *self == IntEvent2MisDio30::IntEvent2MisDio30Set
    }
}
#[doc = "DIO31 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntEvent2MisDio31 {
    #[doc = "0: CLR"]
    IntEvent2MisDio31Clr = 0,
    #[doc = "1: SET"]
    IntEvent2MisDio31Set = 1,
}
impl From<IntEvent2MisDio31> for bool {
    #[inline(always)]
    fn from(variant: IntEvent2MisDio31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_EVENT2_MIS_DIO31` reader - DIO31 event"]
pub type IntEvent2MisDio31R = crate::BitReader<IntEvent2MisDio31>;
impl IntEvent2MisDio31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntEvent2MisDio31 {
        match self.bits {
            false => IntEvent2MisDio31::IntEvent2MisDio31Clr,
            true => IntEvent2MisDio31::IntEvent2MisDio31Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio31_clr(&self) -> bool {
        *self == IntEvent2MisDio31::IntEvent2MisDio31Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_int_event2_mis_dio31_set(&self) -> bool {
        *self == IntEvent2MisDio31::IntEvent2MisDio31Set
    }
}
impl R {
    #[doc = "Bit 16 - DIO16 event"]
    #[inline(always)]
    pub fn int_event2_mis_dio16(&self) -> IntEvent2MisDio16R {
        IntEvent2MisDio16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DIO17 event"]
    #[inline(always)]
    pub fn int_event2_mis_dio17(&self) -> IntEvent2MisDio17R {
        IntEvent2MisDio17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DIO18 event"]
    #[inline(always)]
    pub fn int_event2_mis_dio18(&self) -> IntEvent2MisDio18R {
        IntEvent2MisDio18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DIO19 event"]
    #[inline(always)]
    pub fn int_event2_mis_dio19(&self) -> IntEvent2MisDio19R {
        IntEvent2MisDio19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DIO20 event"]
    #[inline(always)]
    pub fn int_event2_mis_dio20(&self) -> IntEvent2MisDio20R {
        IntEvent2MisDio20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DIO21 event"]
    #[inline(always)]
    pub fn int_event2_mis_dio21(&self) -> IntEvent2MisDio21R {
        IntEvent2MisDio21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DIO22 event"]
    #[inline(always)]
    pub fn int_event2_mis_dio22(&self) -> IntEvent2MisDio22R {
        IntEvent2MisDio22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DIO23 event"]
    #[inline(always)]
    pub fn int_event2_mis_dio23(&self) -> IntEvent2MisDio23R {
        IntEvent2MisDio23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DIO24 event"]
    #[inline(always)]
    pub fn int_event2_mis_dio24(&self) -> IntEvent2MisDio24R {
        IntEvent2MisDio24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DIO25 event"]
    #[inline(always)]
    pub fn int_event2_mis_dio25(&self) -> IntEvent2MisDio25R {
        IntEvent2MisDio25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DIO26 event"]
    #[inline(always)]
    pub fn int_event2_mis_dio26(&self) -> IntEvent2MisDio26R {
        IntEvent2MisDio26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DIO27 event"]
    #[inline(always)]
    pub fn int_event2_mis_dio27(&self) -> IntEvent2MisDio27R {
        IntEvent2MisDio27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DIO28 event"]
    #[inline(always)]
    pub fn int_event2_mis_dio28(&self) -> IntEvent2MisDio28R {
        IntEvent2MisDio28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DIO29 event"]
    #[inline(always)]
    pub fn int_event2_mis_dio29(&self) -> IntEvent2MisDio29R {
        IntEvent2MisDio29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DIO30 event"]
    #[inline(always)]
    pub fn int_event2_mis_dio30(&self) -> IntEvent2MisDio30R {
        IntEvent2MisDio30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DIO31 event"]
    #[inline(always)]
    pub fn int_event2_mis_dio31(&self) -> IntEvent2MisDio31R {
        IntEvent2MisDio31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2MisSpec;
impl crate::RegisterSpec for IntEvent2MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event2_mis::R`](R) reader structure"]
impl crate::Readable for IntEvent2MisSpec {}
#[doc = "`reset()` method sets INT_EVENT2_MIS to value 0"]
impl crate::Resettable for IntEvent2MisSpec {
    const RESET_VALUE: u32 = 0;
}
