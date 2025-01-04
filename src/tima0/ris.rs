#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Zero event generated an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisZ {
    #[doc = "0: CLR"]
    RisZClr = 0,
    #[doc = "1: SET"]
    RisZSet = 1,
}
impl From<RisZ> for bool {
    #[inline(always)]
    fn from(variant: RisZ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_Z` reader - Zero event generated an interrupt."]
pub type RisZR = crate::BitReader<RisZ>;
impl RisZR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisZ {
        match self.bits {
            false => RisZ::RisZClr,
            true => RisZ::RisZSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_z_clr(&self) -> bool {
        *self == RisZ::RisZClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_z_set(&self) -> bool {
        *self == RisZ::RisZSet
    }
}
#[doc = "Load event generated an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisL {
    #[doc = "0: CLR"]
    RisLClr = 0,
    #[doc = "1: SET"]
    RisLSet = 1,
}
impl From<RisL> for bool {
    #[inline(always)]
    fn from(variant: RisL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_L` reader - Load event generated an interrupt."]
pub type RisLR = crate::BitReader<RisL>;
impl RisLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisL {
        match self.bits {
            false => RisL::RisLClr,
            true => RisL::RisLSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_l_clr(&self) -> bool {
        *self == RisL::RisLClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_l_set(&self) -> bool {
        *self == RisL::RisLSet
    }
}
#[doc = "Capture or compare down event generated an interrupt CCP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisCcd0 {
    #[doc = "0: CLR"]
    RisCcd0Clr = 0,
    #[doc = "1: SET"]
    RisCcd0Set = 1,
}
impl From<RisCcd0> for bool {
    #[inline(always)]
    fn from(variant: RisCcd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_CCD0` reader - Capture or compare down event generated an interrupt CCP0"]
pub type RisCcd0R = crate::BitReader<RisCcd0>;
impl RisCcd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisCcd0 {
        match self.bits {
            false => RisCcd0::RisCcd0Clr,
            true => RisCcd0::RisCcd0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_ccd0_clr(&self) -> bool {
        *self == RisCcd0::RisCcd0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_ccd0_set(&self) -> bool {
        *self == RisCcd0::RisCcd0Set
    }
}
#[doc = "Capture or compare down event generated an interrupt CCP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisCcd1 {
    #[doc = "0: CLR"]
    RisCcd1Clr = 0,
    #[doc = "1: SET"]
    RisCcd1Set = 1,
}
impl From<RisCcd1> for bool {
    #[inline(always)]
    fn from(variant: RisCcd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_CCD1` reader - Capture or compare down event generated an interrupt CCP1"]
pub type RisCcd1R = crate::BitReader<RisCcd1>;
impl RisCcd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisCcd1 {
        match self.bits {
            false => RisCcd1::RisCcd1Clr,
            true => RisCcd1::RisCcd1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_ccd1_clr(&self) -> bool {
        *self == RisCcd1::RisCcd1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_ccd1_set(&self) -> bool {
        *self == RisCcd1::RisCcd1Set
    }
}
#[doc = "Capture or compare down event generated an interrupt CCP2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisCcd2 {
    #[doc = "0: CLR"]
    RisCcd2Clr = 0,
    #[doc = "1: SET"]
    RisCcd2Set = 1,
}
impl From<RisCcd2> for bool {
    #[inline(always)]
    fn from(variant: RisCcd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_CCD2` reader - Capture or compare down event generated an interrupt CCP2"]
pub type RisCcd2R = crate::BitReader<RisCcd2>;
impl RisCcd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisCcd2 {
        match self.bits {
            false => RisCcd2::RisCcd2Clr,
            true => RisCcd2::RisCcd2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_ccd2_clr(&self) -> bool {
        *self == RisCcd2::RisCcd2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_ccd2_set(&self) -> bool {
        *self == RisCcd2::RisCcd2Set
    }
}
#[doc = "Capture or compare down event generated an interrupt CCP3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisCcd3 {
    #[doc = "0: CLR"]
    RisCcd3Clr = 0,
    #[doc = "1: SET"]
    RisCcd3Set = 1,
}
impl From<RisCcd3> for bool {
    #[inline(always)]
    fn from(variant: RisCcd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_CCD3` reader - Capture or compare down event generated an interrupt CCP3"]
pub type RisCcd3R = crate::BitReader<RisCcd3>;
impl RisCcd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisCcd3 {
        match self.bits {
            false => RisCcd3::RisCcd3Clr,
            true => RisCcd3::RisCcd3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_ccd3_clr(&self) -> bool {
        *self == RisCcd3::RisCcd3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_ccd3_set(&self) -> bool {
        *self == RisCcd3::RisCcd3Set
    }
}
#[doc = "Capture or compare up event generated an interrupt CCP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisCcu0 {
    #[doc = "0: CLR"]
    RisCcu0Clr = 0,
    #[doc = "1: SET"]
    RisCcu0Set = 1,
}
impl From<RisCcu0> for bool {
    #[inline(always)]
    fn from(variant: RisCcu0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_CCU0` reader - Capture or compare up event generated an interrupt CCP0"]
pub type RisCcu0R = crate::BitReader<RisCcu0>;
impl RisCcu0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisCcu0 {
        match self.bits {
            false => RisCcu0::RisCcu0Clr,
            true => RisCcu0::RisCcu0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_ccu0_clr(&self) -> bool {
        *self == RisCcu0::RisCcu0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_ccu0_set(&self) -> bool {
        *self == RisCcu0::RisCcu0Set
    }
}
#[doc = "Capture or compare up event generated an interrupt CCP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisCcu1 {
    #[doc = "0: CLR"]
    RisCcu1Clr = 0,
    #[doc = "1: SET"]
    RisCcu1Set = 1,
}
impl From<RisCcu1> for bool {
    #[inline(always)]
    fn from(variant: RisCcu1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_CCU1` reader - Capture or compare up event generated an interrupt CCP1"]
pub type RisCcu1R = crate::BitReader<RisCcu1>;
impl RisCcu1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisCcu1 {
        match self.bits {
            false => RisCcu1::RisCcu1Clr,
            true => RisCcu1::RisCcu1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_ccu1_clr(&self) -> bool {
        *self == RisCcu1::RisCcu1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_ccu1_set(&self) -> bool {
        *self == RisCcu1::RisCcu1Set
    }
}
#[doc = "Capture or compare up event generated an interrupt CCP2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisCcu2 {
    #[doc = "0: CLR"]
    RisCcu2Clr = 0,
    #[doc = "1: SET"]
    RisCcu2Set = 1,
}
impl From<RisCcu2> for bool {
    #[inline(always)]
    fn from(variant: RisCcu2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_CCU2` reader - Capture or compare up event generated an interrupt CCP2"]
pub type RisCcu2R = crate::BitReader<RisCcu2>;
impl RisCcu2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisCcu2 {
        match self.bits {
            false => RisCcu2::RisCcu2Clr,
            true => RisCcu2::RisCcu2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_ccu2_clr(&self) -> bool {
        *self == RisCcu2::RisCcu2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_ccu2_set(&self) -> bool {
        *self == RisCcu2::RisCcu2Set
    }
}
#[doc = "Capture or compare up event generated an interrupt CCP3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisCcu3 {
    #[doc = "0: CLR"]
    RisCcu3Clr = 0,
    #[doc = "1: SET"]
    RisCcu3Set = 1,
}
impl From<RisCcu3> for bool {
    #[inline(always)]
    fn from(variant: RisCcu3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_CCU3` reader - Capture or compare up event generated an interrupt CCP3"]
pub type RisCcu3R = crate::BitReader<RisCcu3>;
impl RisCcu3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisCcu3 {
        match self.bits {
            false => RisCcu3::RisCcu3Clr,
            true => RisCcu3::RisCcu3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_ccu3_clr(&self) -> bool {
        *self == RisCcu3::RisCcu3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_ccu3_set(&self) -> bool {
        *self == RisCcu3::RisCcu3Set
    }
}
#[doc = "Compare down event generated an interrupt CCD4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisCcd4 {
    #[doc = "0: CLR"]
    RisCcd4Clr = 0,
    #[doc = "1: SET"]
    RisCcd4Set = 1,
}
impl From<RisCcd4> for bool {
    #[inline(always)]
    fn from(variant: RisCcd4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_CCD4` reader - Compare down event generated an interrupt CCD4"]
pub type RisCcd4R = crate::BitReader<RisCcd4>;
impl RisCcd4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisCcd4 {
        match self.bits {
            false => RisCcd4::RisCcd4Clr,
            true => RisCcd4::RisCcd4Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_ccd4_clr(&self) -> bool {
        *self == RisCcd4::RisCcd4Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_ccd4_set(&self) -> bool {
        *self == RisCcd4::RisCcd4Set
    }
}
#[doc = "Compare down event generated an interrupt CCD5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisCcd5 {
    #[doc = "0: CLR"]
    RisCcd5Clr = 0,
    #[doc = "1: SET"]
    RisCcd5Set = 1,
}
impl From<RisCcd5> for bool {
    #[inline(always)]
    fn from(variant: RisCcd5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_CCD5` reader - Compare down event generated an interrupt CCD5"]
pub type RisCcd5R = crate::BitReader<RisCcd5>;
impl RisCcd5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisCcd5 {
        match self.bits {
            false => RisCcd5::RisCcd5Clr,
            true => RisCcd5::RisCcd5Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_ccd5_clr(&self) -> bool {
        *self == RisCcd5::RisCcd5Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_ccd5_set(&self) -> bool {
        *self == RisCcd5::RisCcd5Set
    }
}
#[doc = "Compare up event generated an interrupt CCU4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisCcu4 {
    #[doc = "0: CLR"]
    RisCcu4Clr = 0,
    #[doc = "1: SET"]
    RisCcu4Set = 1,
}
impl From<RisCcu4> for bool {
    #[inline(always)]
    fn from(variant: RisCcu4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_CCU4` reader - Compare up event generated an interrupt CCU4"]
pub type RisCcu4R = crate::BitReader<RisCcu4>;
impl RisCcu4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisCcu4 {
        match self.bits {
            false => RisCcu4::RisCcu4Clr,
            true => RisCcu4::RisCcu4Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_ccu4_clr(&self) -> bool {
        *self == RisCcu4::RisCcu4Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_ccu4_set(&self) -> bool {
        *self == RisCcu4::RisCcu4Set
    }
}
#[doc = "Compare up event generated an interrupt CCP5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisCcu5 {
    #[doc = "0: CLR"]
    RisCcu5Clr = 0,
    #[doc = "1: SET"]
    RisCcu5Set = 1,
}
impl From<RisCcu5> for bool {
    #[inline(always)]
    fn from(variant: RisCcu5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_CCU5` reader - Compare up event generated an interrupt CCP5"]
pub type RisCcu5R = crate::BitReader<RisCcu5>;
impl RisCcu5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisCcu5 {
        match self.bits {
            false => RisCcu5::RisCcu5Clr,
            true => RisCcu5::RisCcu5Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_ccu5_clr(&self) -> bool {
        *self == RisCcu5::RisCcu5Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_ccu5_set(&self) -> bool {
        *self == RisCcu5::RisCcu5Set
    }
}
#[doc = "Fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisF {
    #[doc = "0: CLR"]
    RisFClr = 0,
    #[doc = "1: SET"]
    RisFSet = 1,
}
impl From<RisF> for bool {
    #[inline(always)]
    fn from(variant: RisF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_F` reader - Fault"]
pub type RisFR = crate::BitReader<RisF>;
impl RisFR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisF {
        match self.bits {
            false => RisF::RisFClr,
            true => RisF::RisFSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_f_clr(&self) -> bool {
        *self == RisF::RisFClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_f_set(&self) -> bool {
        *self == RisF::RisFSet
    }
}
#[doc = "Trigger overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisTov {
    #[doc = "0: CLR"]
    RisTovClr = 0,
    #[doc = "1: SET"]
    RisTovSet = 1,
}
impl From<RisTov> for bool {
    #[inline(always)]
    fn from(variant: RisTov) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_TOV` reader - Trigger overflow"]
pub type RisTovR = crate::BitReader<RisTov>;
impl RisTovR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisTov {
        match self.bits {
            false => RisTov::RisTovClr,
            true => RisTov::RisTovSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_tov_clr(&self) -> bool {
        *self == RisTov::RisTovClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_tov_set(&self) -> bool {
        *self == RisTov::RisTovSet
    }
}
#[doc = "Repeat Counter Zero\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RisRepc {
    #[doc = "0: CLR"]
    RisRepcClr = 0,
    #[doc = "1: SET"]
    RisRepcSet = 1,
}
impl From<RisRepc> for bool {
    #[inline(always)]
    fn from(variant: RisRepc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RIS_REPC` reader - Repeat Counter Zero"]
pub type RisRepcR = crate::BitReader<RisRepc>;
impl RisRepcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RisRepc {
        match self.bits {
            false => RisRepc::RisRepcClr,
            true => RisRepc::RisRepcSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ris_repc_clr(&self) -> bool {
        *self == RisRepc::RisRepcClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ris_repc_set(&self) -> bool {
        *self == RisRepc::RisRepcSet
    }
}
impl R {
    #[doc = "Bit 0 - Zero event generated an interrupt."]
    #[inline(always)]
    pub fn ris_z(&self) -> RisZR {
        RisZR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load event generated an interrupt."]
    #[inline(always)]
    pub fn ris_l(&self) -> RisLR {
        RisLR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture or compare down event generated an interrupt CCP0"]
    #[inline(always)]
    pub fn ris_ccd0(&self) -> RisCcd0R {
        RisCcd0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture or compare down event generated an interrupt CCP1"]
    #[inline(always)]
    pub fn ris_ccd1(&self) -> RisCcd1R {
        RisCcd1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture or compare down event generated an interrupt CCP2"]
    #[inline(always)]
    pub fn ris_ccd2(&self) -> RisCcd2R {
        RisCcd2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture or compare down event generated an interrupt CCP3"]
    #[inline(always)]
    pub fn ris_ccd3(&self) -> RisCcd3R {
        RisCcd3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture or compare up event generated an interrupt CCP0"]
    #[inline(always)]
    pub fn ris_ccu0(&self) -> RisCcu0R {
        RisCcu0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture or compare up event generated an interrupt CCP1"]
    #[inline(always)]
    pub fn ris_ccu1(&self) -> RisCcu1R {
        RisCcu1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture or compare up event generated an interrupt CCP2"]
    #[inline(always)]
    pub fn ris_ccu2(&self) -> RisCcu2R {
        RisCcu2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture or compare up event generated an interrupt CCP3"]
    #[inline(always)]
    pub fn ris_ccu3(&self) -> RisCcu3R {
        RisCcu3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Compare down event generated an interrupt CCD4"]
    #[inline(always)]
    pub fn ris_ccd4(&self) -> RisCcd4R {
        RisCcd4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Compare down event generated an interrupt CCD5"]
    #[inline(always)]
    pub fn ris_ccd5(&self) -> RisCcd5R {
        RisCcd5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Compare up event generated an interrupt CCU4"]
    #[inline(always)]
    pub fn ris_ccu4(&self) -> RisCcu4R {
        RisCcu4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Compare up event generated an interrupt CCP5"]
    #[inline(always)]
    pub fn ris_ccu5(&self) -> RisCcu5R {
        RisCcu5R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - Fault"]
    #[inline(always)]
    pub fn ris_f(&self) -> RisFR {
        RisFR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Trigger overflow"]
    #[inline(always)]
    pub fn ris_tov(&self) -> RisTovR {
        RisTovR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Repeat Counter Zero"]
    #[inline(always)]
    pub fn ris_repc(&self) -> RisRepcR {
        RisRepcR::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0;
}
