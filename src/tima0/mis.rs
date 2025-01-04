#[doc = "Register `MIS` reader"]
pub type R = crate::R<MisSpec>;
#[doc = "Zero event generated an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisZ {
    #[doc = "0: CLR"]
    MisZClr = 0,
    #[doc = "1: SET"]
    MisZSet = 1,
}
impl From<MisZ> for bool {
    #[inline(always)]
    fn from(variant: MisZ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_Z` reader - Zero event generated an interrupt."]
pub type MisZR = crate::BitReader<MisZ>;
impl MisZR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisZ {
        match self.bits {
            false => MisZ::MisZClr,
            true => MisZ::MisZSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_z_clr(&self) -> bool {
        *self == MisZ::MisZClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_z_set(&self) -> bool {
        *self == MisZ::MisZSet
    }
}
#[doc = "Load event generated an interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisL {
    #[doc = "0: CLR"]
    MisLClr = 0,
    #[doc = "1: SET"]
    MisLSet = 1,
}
impl From<MisL> for bool {
    #[inline(always)]
    fn from(variant: MisL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_L` reader - Load event generated an interrupt."]
pub type MisLR = crate::BitReader<MisL>;
impl MisLR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisL {
        match self.bits {
            false => MisL::MisLClr,
            true => MisL::MisLSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_l_clr(&self) -> bool {
        *self == MisL::MisLClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_l_set(&self) -> bool {
        *self == MisL::MisLSet
    }
}
#[doc = "Capture or compare down event generated an interrupt CCP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisCcd0 {
    #[doc = "0: CLR"]
    MisCcd0Clr = 0,
    #[doc = "1: SET"]
    MisCcd0Set = 1,
}
impl From<MisCcd0> for bool {
    #[inline(always)]
    fn from(variant: MisCcd0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_CCD0` reader - Capture or compare down event generated an interrupt CCP0"]
pub type MisCcd0R = crate::BitReader<MisCcd0>;
impl MisCcd0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisCcd0 {
        match self.bits {
            false => MisCcd0::MisCcd0Clr,
            true => MisCcd0::MisCcd0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_ccd0_clr(&self) -> bool {
        *self == MisCcd0::MisCcd0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_ccd0_set(&self) -> bool {
        *self == MisCcd0::MisCcd0Set
    }
}
#[doc = "Capture or compare down event generated an interrupt CCP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisCcd1 {
    #[doc = "0: CLR"]
    MisCcd1Clr = 0,
    #[doc = "1: SET"]
    MisCcd1Set = 1,
}
impl From<MisCcd1> for bool {
    #[inline(always)]
    fn from(variant: MisCcd1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_CCD1` reader - Capture or compare down event generated an interrupt CCP1"]
pub type MisCcd1R = crate::BitReader<MisCcd1>;
impl MisCcd1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisCcd1 {
        match self.bits {
            false => MisCcd1::MisCcd1Clr,
            true => MisCcd1::MisCcd1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_ccd1_clr(&self) -> bool {
        *self == MisCcd1::MisCcd1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_ccd1_set(&self) -> bool {
        *self == MisCcd1::MisCcd1Set
    }
}
#[doc = "Capture or compare down event generated an interrupt CCP2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisCcd2 {
    #[doc = "0: CLR"]
    MisCcd2Clr = 0,
    #[doc = "1: SET"]
    MisCcd2Set = 1,
}
impl From<MisCcd2> for bool {
    #[inline(always)]
    fn from(variant: MisCcd2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_CCD2` reader - Capture or compare down event generated an interrupt CCP2"]
pub type MisCcd2R = crate::BitReader<MisCcd2>;
impl MisCcd2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisCcd2 {
        match self.bits {
            false => MisCcd2::MisCcd2Clr,
            true => MisCcd2::MisCcd2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_ccd2_clr(&self) -> bool {
        *self == MisCcd2::MisCcd2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_ccd2_set(&self) -> bool {
        *self == MisCcd2::MisCcd2Set
    }
}
#[doc = "Capture or compare down event generated an interrupt CCP3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisCcd3 {
    #[doc = "0: CLR"]
    MisCcd3Clr = 0,
    #[doc = "1: SET"]
    MisCcd3Set = 1,
}
impl From<MisCcd3> for bool {
    #[inline(always)]
    fn from(variant: MisCcd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_CCD3` reader - Capture or compare down event generated an interrupt CCP3"]
pub type MisCcd3R = crate::BitReader<MisCcd3>;
impl MisCcd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisCcd3 {
        match self.bits {
            false => MisCcd3::MisCcd3Clr,
            true => MisCcd3::MisCcd3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_ccd3_clr(&self) -> bool {
        *self == MisCcd3::MisCcd3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_ccd3_set(&self) -> bool {
        *self == MisCcd3::MisCcd3Set
    }
}
#[doc = "Capture or compare up event generated an interrupt CCP0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisCcu0 {
    #[doc = "0: CLR"]
    MisCcu0Clr = 0,
    #[doc = "1: SET"]
    MisCcu0Set = 1,
}
impl From<MisCcu0> for bool {
    #[inline(always)]
    fn from(variant: MisCcu0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_CCU0` reader - Capture or compare up event generated an interrupt CCP0"]
pub type MisCcu0R = crate::BitReader<MisCcu0>;
impl MisCcu0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisCcu0 {
        match self.bits {
            false => MisCcu0::MisCcu0Clr,
            true => MisCcu0::MisCcu0Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_ccu0_clr(&self) -> bool {
        *self == MisCcu0::MisCcu0Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_ccu0_set(&self) -> bool {
        *self == MisCcu0::MisCcu0Set
    }
}
#[doc = "Capture or compare up event generated an interrupt CCP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisCcu1 {
    #[doc = "0: CLR"]
    MisCcu1Clr = 0,
    #[doc = "1: SET"]
    MisCcu1Set = 1,
}
impl From<MisCcu1> for bool {
    #[inline(always)]
    fn from(variant: MisCcu1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_CCU1` reader - Capture or compare up event generated an interrupt CCP1"]
pub type MisCcu1R = crate::BitReader<MisCcu1>;
impl MisCcu1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisCcu1 {
        match self.bits {
            false => MisCcu1::MisCcu1Clr,
            true => MisCcu1::MisCcu1Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_ccu1_clr(&self) -> bool {
        *self == MisCcu1::MisCcu1Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_ccu1_set(&self) -> bool {
        *self == MisCcu1::MisCcu1Set
    }
}
#[doc = "Capture or compare up event generated an interrupt CCP2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisCcu2 {
    #[doc = "0: CLR"]
    MisCcu2Clr = 0,
    #[doc = "1: SET"]
    MisCcu2Set = 1,
}
impl From<MisCcu2> for bool {
    #[inline(always)]
    fn from(variant: MisCcu2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_CCU2` reader - Capture or compare up event generated an interrupt CCP2"]
pub type MisCcu2R = crate::BitReader<MisCcu2>;
impl MisCcu2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisCcu2 {
        match self.bits {
            false => MisCcu2::MisCcu2Clr,
            true => MisCcu2::MisCcu2Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_ccu2_clr(&self) -> bool {
        *self == MisCcu2::MisCcu2Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_ccu2_set(&self) -> bool {
        *self == MisCcu2::MisCcu2Set
    }
}
#[doc = "Capture or compare up event generated an interrupt CCP3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisCcu3 {
    #[doc = "0: CLR"]
    MisCcu3Clr = 0,
    #[doc = "1: SET"]
    MisCcu3Set = 1,
}
impl From<MisCcu3> for bool {
    #[inline(always)]
    fn from(variant: MisCcu3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_CCU3` reader - Capture or compare up event generated an interrupt CCP3"]
pub type MisCcu3R = crate::BitReader<MisCcu3>;
impl MisCcu3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisCcu3 {
        match self.bits {
            false => MisCcu3::MisCcu3Clr,
            true => MisCcu3::MisCcu3Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_ccu3_clr(&self) -> bool {
        *self == MisCcu3::MisCcu3Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_ccu3_set(&self) -> bool {
        *self == MisCcu3::MisCcu3Set
    }
}
#[doc = "Compare down event generated an interrupt CCP4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisCcd4 {
    #[doc = "0: CLR"]
    MisCcd4Clr = 0,
    #[doc = "1: SET"]
    MisCcd4Set = 1,
}
impl From<MisCcd4> for bool {
    #[inline(always)]
    fn from(variant: MisCcd4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_CCD4` reader - Compare down event generated an interrupt CCP4"]
pub type MisCcd4R = crate::BitReader<MisCcd4>;
impl MisCcd4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisCcd4 {
        match self.bits {
            false => MisCcd4::MisCcd4Clr,
            true => MisCcd4::MisCcd4Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_ccd4_clr(&self) -> bool {
        *self == MisCcd4::MisCcd4Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_ccd4_set(&self) -> bool {
        *self == MisCcd4::MisCcd4Set
    }
}
#[doc = "Compare down event generated an interrupt CCP5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisCcd5 {
    #[doc = "0: CLR"]
    MisCcd5Clr = 0,
    #[doc = "1: SET"]
    MisCcd5Set = 1,
}
impl From<MisCcd5> for bool {
    #[inline(always)]
    fn from(variant: MisCcd5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_CCD5` reader - Compare down event generated an interrupt CCP5"]
pub type MisCcd5R = crate::BitReader<MisCcd5>;
impl MisCcd5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisCcd5 {
        match self.bits {
            false => MisCcd5::MisCcd5Clr,
            true => MisCcd5::MisCcd5Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_ccd5_clr(&self) -> bool {
        *self == MisCcd5::MisCcd5Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_ccd5_set(&self) -> bool {
        *self == MisCcd5::MisCcd5Set
    }
}
#[doc = "Compare up event generated an interrupt CCP4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisCcu4 {
    #[doc = "0: CLR"]
    MisCcu4Clr = 0,
    #[doc = "1: SET"]
    MisCcu4Set = 1,
}
impl From<MisCcu4> for bool {
    #[inline(always)]
    fn from(variant: MisCcu4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_CCU4` reader - Compare up event generated an interrupt CCP4"]
pub type MisCcu4R = crate::BitReader<MisCcu4>;
impl MisCcu4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisCcu4 {
        match self.bits {
            false => MisCcu4::MisCcu4Clr,
            true => MisCcu4::MisCcu4Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_ccu4_clr(&self) -> bool {
        *self == MisCcu4::MisCcu4Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_ccu4_set(&self) -> bool {
        *self == MisCcu4::MisCcu4Set
    }
}
#[doc = "Compare up event generated an interrupt CCP5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisCcu5 {
    #[doc = "0: CLR"]
    MisCcu5Clr = 0,
    #[doc = "1: SET"]
    MisCcu5Set = 1,
}
impl From<MisCcu5> for bool {
    #[inline(always)]
    fn from(variant: MisCcu5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_CCU5` reader - Compare up event generated an interrupt CCP5"]
pub type MisCcu5R = crate::BitReader<MisCcu5>;
impl MisCcu5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisCcu5 {
        match self.bits {
            false => MisCcu5::MisCcu5Clr,
            true => MisCcu5::MisCcu5Set,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_ccu5_clr(&self) -> bool {
        *self == MisCcu5::MisCcu5Clr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_ccu5_set(&self) -> bool {
        *self == MisCcu5::MisCcu5Set
    }
}
#[doc = "Fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisF {
    #[doc = "0: CLR"]
    MisFClr = 0,
    #[doc = "1: SET"]
    MisFSet = 1,
}
impl From<MisF> for bool {
    #[inline(always)]
    fn from(variant: MisF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_F` reader - Fault"]
pub type MisFR = crate::BitReader<MisF>;
impl MisFR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisF {
        match self.bits {
            false => MisF::MisFClr,
            true => MisF::MisFSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_f_clr(&self) -> bool {
        *self == MisF::MisFClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_f_set(&self) -> bool {
        *self == MisF::MisFSet
    }
}
#[doc = "Trigger overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisTov {
    #[doc = "0: CLR"]
    MisTovClr = 0,
    #[doc = "1: SET"]
    MisTovSet = 1,
}
impl From<MisTov> for bool {
    #[inline(always)]
    fn from(variant: MisTov) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_TOV` reader - Trigger overflow"]
pub type MisTovR = crate::BitReader<MisTov>;
impl MisTovR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisTov {
        match self.bits {
            false => MisTov::MisTovClr,
            true => MisTov::MisTovSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_tov_clr(&self) -> bool {
        *self == MisTov::MisTovClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_tov_set(&self) -> bool {
        *self == MisTov::MisTovSet
    }
}
#[doc = "Repeat Counter Zero\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisRepc {
    #[doc = "0: CLR"]
    MisRepcClr = 0,
    #[doc = "1: SET"]
    MisRepcSet = 1,
}
impl From<MisRepc> for bool {
    #[inline(always)]
    fn from(variant: MisRepc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MIS_REPC` reader - Repeat Counter Zero"]
pub type MisRepcR = crate::BitReader<MisRepc>;
impl MisRepcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MisRepc {
        match self.bits {
            false => MisRepc::MisRepcClr,
            true => MisRepc::MisRepcSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_mis_repc_clr(&self) -> bool {
        *self == MisRepc::MisRepcClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_mis_repc_set(&self) -> bool {
        *self == MisRepc::MisRepcSet
    }
}
impl R {
    #[doc = "Bit 0 - Zero event generated an interrupt."]
    #[inline(always)]
    pub fn mis_z(&self) -> MisZR {
        MisZR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Load event generated an interrupt."]
    #[inline(always)]
    pub fn mis_l(&self) -> MisLR {
        MisLR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture or compare down event generated an interrupt CCP0"]
    #[inline(always)]
    pub fn mis_ccd0(&self) -> MisCcd0R {
        MisCcd0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capture or compare down event generated an interrupt CCP1"]
    #[inline(always)]
    pub fn mis_ccd1(&self) -> MisCcd1R {
        MisCcd1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture or compare down event generated an interrupt CCP2"]
    #[inline(always)]
    pub fn mis_ccd2(&self) -> MisCcd2R {
        MisCcd2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture or compare down event generated an interrupt CCP3"]
    #[inline(always)]
    pub fn mis_ccd3(&self) -> MisCcd3R {
        MisCcd3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture or compare up event generated an interrupt CCP0"]
    #[inline(always)]
    pub fn mis_ccu0(&self) -> MisCcu0R {
        MisCcu0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture or compare up event generated an interrupt CCP1"]
    #[inline(always)]
    pub fn mis_ccu1(&self) -> MisCcu1R {
        MisCcu1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture or compare up event generated an interrupt CCP2"]
    #[inline(always)]
    pub fn mis_ccu2(&self) -> MisCcu2R {
        MisCcu2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture or compare up event generated an interrupt CCP3"]
    #[inline(always)]
    pub fn mis_ccu3(&self) -> MisCcu3R {
        MisCcu3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Compare down event generated an interrupt CCP4"]
    #[inline(always)]
    pub fn mis_ccd4(&self) -> MisCcd4R {
        MisCcd4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Compare down event generated an interrupt CCP5"]
    #[inline(always)]
    pub fn mis_ccd5(&self) -> MisCcd5R {
        MisCcd5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Compare up event generated an interrupt CCP4"]
    #[inline(always)]
    pub fn mis_ccu4(&self) -> MisCcu4R {
        MisCcu4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Compare up event generated an interrupt CCP5"]
    #[inline(always)]
    pub fn mis_ccu5(&self) -> MisCcu5R {
        MisCcu5R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - Fault"]
    #[inline(always)]
    pub fn mis_f(&self) -> MisFR {
        MisFR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Trigger overflow"]
    #[inline(always)]
    pub fn mis_tov(&self) -> MisTovR {
        MisTovR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Repeat Counter Zero"]
    #[inline(always)]
    pub fn mis_repc(&self) -> MisRepcR {
        MisRepcR::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisSpec;
impl crate::RegisterSpec for MisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mis::R`](R) reader structure"]
impl crate::Readable for MisSpec {}
#[doc = "`reset()` method sets MIS to value 0"]
impl crate::Resettable for MisSpec {
    const RESET_VALUE: u32 = 0;
}
