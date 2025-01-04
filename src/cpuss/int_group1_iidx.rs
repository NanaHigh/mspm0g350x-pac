#[doc = "Register `INT_GROUP1_IIDX` reader"]
pub type R = crate::R<IntGroup1IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntGroup1IidxStat {
    #[doc = "0: NO_INTR"]
    IntGroup1IidxStatNoIntr = 0,
    #[doc = "1: INT0"]
    IntGroup1IidxStatInt0 = 1,
    #[doc = "2: INT1"]
    IntGroup1IidxStatInt1 = 2,
    #[doc = "3: INT2"]
    IntGroup1IidxStatInt2 = 3,
    #[doc = "4: INT3"]
    IntGroup1IidxStatInt3 = 4,
    #[doc = "5: INT4"]
    IntGroup1IidxStatInt4 = 5,
    #[doc = "6: INT5"]
    IntGroup1IidxStatInt5 = 6,
    #[doc = "7: INT6"]
    IntGroup1IidxStatInt6 = 7,
    #[doc = "8: INT7"]
    IntGroup1IidxStatInt7 = 8,
}
impl From<IntGroup1IidxStat> for u8 {
    #[inline(always)]
    fn from(variant: IntGroup1IidxStat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntGroup1IidxStat {
    type Ux = u8;
}
impl crate::IsEnum for IntGroup1IidxStat {}
#[doc = "Field `INT_GROUP1_IIDX_STAT` reader - Interrupt index status"]
pub type IntGroup1IidxStatR = crate::FieldReader<IntGroup1IidxStat>;
impl IntGroup1IidxStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntGroup1IidxStat> {
        match self.bits {
            0 => Some(IntGroup1IidxStat::IntGroup1IidxStatNoIntr),
            1 => Some(IntGroup1IidxStat::IntGroup1IidxStatInt0),
            2 => Some(IntGroup1IidxStat::IntGroup1IidxStatInt1),
            3 => Some(IntGroup1IidxStat::IntGroup1IidxStatInt2),
            4 => Some(IntGroup1IidxStat::IntGroup1IidxStatInt3),
            5 => Some(IntGroup1IidxStat::IntGroup1IidxStatInt4),
            6 => Some(IntGroup1IidxStat::IntGroup1IidxStatInt5),
            7 => Some(IntGroup1IidxStat::IntGroup1IidxStatInt6),
            8 => Some(IntGroup1IidxStat::IntGroup1IidxStatInt7),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_int_group1_iidx_stat_no_intr(&self) -> bool {
        *self == IntGroup1IidxStat::IntGroup1IidxStatNoIntr
    }
    #[doc = "INT0"]
    #[inline(always)]
    pub fn is_int_group1_iidx_stat_int0(&self) -> bool {
        *self == IntGroup1IidxStat::IntGroup1IidxStatInt0
    }
    #[doc = "INT1"]
    #[inline(always)]
    pub fn is_int_group1_iidx_stat_int1(&self) -> bool {
        *self == IntGroup1IidxStat::IntGroup1IidxStatInt1
    }
    #[doc = "INT2"]
    #[inline(always)]
    pub fn is_int_group1_iidx_stat_int2(&self) -> bool {
        *self == IntGroup1IidxStat::IntGroup1IidxStatInt2
    }
    #[doc = "INT3"]
    #[inline(always)]
    pub fn is_int_group1_iidx_stat_int3(&self) -> bool {
        *self == IntGroup1IidxStat::IntGroup1IidxStatInt3
    }
    #[doc = "INT4"]
    #[inline(always)]
    pub fn is_int_group1_iidx_stat_int4(&self) -> bool {
        *self == IntGroup1IidxStat::IntGroup1IidxStatInt4
    }
    #[doc = "INT5"]
    #[inline(always)]
    pub fn is_int_group1_iidx_stat_int5(&self) -> bool {
        *self == IntGroup1IidxStat::IntGroup1IidxStatInt5
    }
    #[doc = "INT6"]
    #[inline(always)]
    pub fn is_int_group1_iidx_stat_int6(&self) -> bool {
        *self == IntGroup1IidxStat::IntGroup1IidxStatInt6
    }
    #[doc = "INT7"]
    #[inline(always)]
    pub fn is_int_group1_iidx_stat_int7(&self) -> bool {
        *self == IntGroup1IidxStat::IntGroup1IidxStatInt7
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn int_group1_iidx_stat(&self) -> IntGroup1IidxStatR {
        IntGroup1IidxStatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`int_group1_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntGroup1IidxSpec;
impl crate::RegisterSpec for IntGroup1IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_group1_iidx::R`](R) reader structure"]
impl crate::Readable for IntGroup1IidxSpec {}
#[doc = "`reset()` method sets INT_GROUP1_IIDX to value 0"]
impl crate::Resettable for IntGroup1IidxSpec {
    const RESET_VALUE: u32 = 0;
}
