#[doc = "Register `INT_GROUP0_IIDX` reader"]
pub type R = crate::R<IntGroup0IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntGroup0IidxStat {
    #[doc = "0: NO_INTR"]
    IntGroup0IidxStatNoIntr = 0,
    #[doc = "1: INT0"]
    IntGroup0IidxStatInt0 = 1,
    #[doc = "2: INT1"]
    IntGroup0IidxStatInt1 = 2,
    #[doc = "3: INT2"]
    IntGroup0IidxStatInt2 = 3,
    #[doc = "4: INT3"]
    IntGroup0IidxStatInt3 = 4,
    #[doc = "5: INT4"]
    IntGroup0IidxStatInt4 = 5,
    #[doc = "6: INT5"]
    IntGroup0IidxStatInt5 = 6,
    #[doc = "7: INT6"]
    IntGroup0IidxStatInt6 = 7,
    #[doc = "8: INT7"]
    IntGroup0IidxStatInt7 = 8,
}
impl From<IntGroup0IidxStat> for u8 {
    #[inline(always)]
    fn from(variant: IntGroup0IidxStat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntGroup0IidxStat {
    type Ux = u8;
}
impl crate::IsEnum for IntGroup0IidxStat {}
#[doc = "Field `INT_GROUP0_IIDX_STAT` reader - Interrupt index status"]
pub type IntGroup0IidxStatR = crate::FieldReader<IntGroup0IidxStat>;
impl IntGroup0IidxStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntGroup0IidxStat> {
        match self.bits {
            0 => Some(IntGroup0IidxStat::IntGroup0IidxStatNoIntr),
            1 => Some(IntGroup0IidxStat::IntGroup0IidxStatInt0),
            2 => Some(IntGroup0IidxStat::IntGroup0IidxStatInt1),
            3 => Some(IntGroup0IidxStat::IntGroup0IidxStatInt2),
            4 => Some(IntGroup0IidxStat::IntGroup0IidxStatInt3),
            5 => Some(IntGroup0IidxStat::IntGroup0IidxStatInt4),
            6 => Some(IntGroup0IidxStat::IntGroup0IidxStatInt5),
            7 => Some(IntGroup0IidxStat::IntGroup0IidxStatInt6),
            8 => Some(IntGroup0IidxStat::IntGroup0IidxStatInt7),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_int_group0_iidx_stat_no_intr(&self) -> bool {
        *self == IntGroup0IidxStat::IntGroup0IidxStatNoIntr
    }
    #[doc = "INT0"]
    #[inline(always)]
    pub fn is_int_group0_iidx_stat_int0(&self) -> bool {
        *self == IntGroup0IidxStat::IntGroup0IidxStatInt0
    }
    #[doc = "INT1"]
    #[inline(always)]
    pub fn is_int_group0_iidx_stat_int1(&self) -> bool {
        *self == IntGroup0IidxStat::IntGroup0IidxStatInt1
    }
    #[doc = "INT2"]
    #[inline(always)]
    pub fn is_int_group0_iidx_stat_int2(&self) -> bool {
        *self == IntGroup0IidxStat::IntGroup0IidxStatInt2
    }
    #[doc = "INT3"]
    #[inline(always)]
    pub fn is_int_group0_iidx_stat_int3(&self) -> bool {
        *self == IntGroup0IidxStat::IntGroup0IidxStatInt3
    }
    #[doc = "INT4"]
    #[inline(always)]
    pub fn is_int_group0_iidx_stat_int4(&self) -> bool {
        *self == IntGroup0IidxStat::IntGroup0IidxStatInt4
    }
    #[doc = "INT5"]
    #[inline(always)]
    pub fn is_int_group0_iidx_stat_int5(&self) -> bool {
        *self == IntGroup0IidxStat::IntGroup0IidxStatInt5
    }
    #[doc = "INT6"]
    #[inline(always)]
    pub fn is_int_group0_iidx_stat_int6(&self) -> bool {
        *self == IntGroup0IidxStat::IntGroup0IidxStatInt6
    }
    #[doc = "INT7"]
    #[inline(always)]
    pub fn is_int_group0_iidx_stat_int7(&self) -> bool {
        *self == IntGroup0IidxStat::IntGroup0IidxStatInt7
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn int_group0_iidx_stat(&self) -> IntGroup0IidxStatR {
        IntGroup0IidxStatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`int_group0_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntGroup0IidxSpec;
impl crate::RegisterSpec for IntGroup0IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_group0_iidx::R`](R) reader structure"]
impl crate::Readable for IntGroup0IidxSpec {}
#[doc = "`reset()` method sets INT_GROUP0_IIDX to value 0"]
impl crate::Resettable for IntGroup0IidxSpec {
    const RESET_VALUE: u32 = 0;
}
