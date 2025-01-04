#[doc = "Register `INT_EVENT2_IIDX` reader"]
pub type R = crate::R<IntEvent2IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum IntEvent2IidxStat {
    #[doc = "0: NO_INTR"]
    IntEvent2IidxStatNoIntr = 0,
    #[doc = "9: MEMRESIFG0"]
    IntEvent2IidxStatMemresifg0 = 9,
    #[doc = "10: MEMRESIFG1"]
    IntEvent2IidxStatMemresifg1 = 10,
    #[doc = "11: MEMRESIFG2"]
    IntEvent2IidxStatMemresifg2 = 11,
    #[doc = "12: MEMRESIFG3"]
    IntEvent2IidxStatMemresifg3 = 12,
    #[doc = "13: MEMRESIFG4"]
    IntEvent2IidxStatMemresifg4 = 13,
    #[doc = "14: MEMRESIFG5"]
    IntEvent2IidxStatMemresifg5 = 14,
    #[doc = "15: MEMRESIFG6"]
    IntEvent2IidxStatMemresifg6 = 15,
    #[doc = "16: MEMRESIFG7"]
    IntEvent2IidxStatMemresifg7 = 16,
    #[doc = "17: MEMRESIFG8"]
    IntEvent2IidxStatMemresifg8 = 17,
    #[doc = "18: MEMRESIFG9"]
    IntEvent2IidxStatMemresifg9 = 18,
    #[doc = "19: MEMRESIFG10"]
    IntEvent2IidxStatMemresifg10 = 19,
    #[doc = "20: MEMRESIFG11"]
    IntEvent2IidxStatMemresifg11 = 20,
    #[doc = "21: MEMRESIFG12"]
    IntEvent2IidxStatMemresifg12 = 21,
    #[doc = "22: MEMRESIFG13"]
    IntEvent2IidxStatMemresifg13 = 22,
    #[doc = "23: MEMRESIFG14"]
    IntEvent2IidxStatMemresifg14 = 23,
    #[doc = "24: MEMRESIFG15"]
    IntEvent2IidxStatMemresifg15 = 24,
    #[doc = "25: MEMRESIFG16"]
    IntEvent2IidxStatMemresifg16 = 25,
    #[doc = "26: MEMRESIFG17"]
    IntEvent2IidxStatMemresifg17 = 26,
    #[doc = "27: MEMRESIFG18"]
    IntEvent2IidxStatMemresifg18 = 27,
    #[doc = "28: MEMRESIFG19"]
    IntEvent2IidxStatMemresifg19 = 28,
    #[doc = "29: MEMRESIFG20"]
    IntEvent2IidxStatMemresifg20 = 29,
    #[doc = "30: MEMRESIFG21"]
    IntEvent2IidxStatMemresifg21 = 30,
    #[doc = "31: MEMRESIFG22"]
    IntEvent2IidxStatMemresifg22 = 31,
    #[doc = "32: MEMRESIFG23"]
    IntEvent2IidxStatMemresifg23 = 32,
}
impl From<IntEvent2IidxStat> for u16 {
    #[inline(always)]
    fn from(variant: IntEvent2IidxStat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntEvent2IidxStat {
    type Ux = u16;
}
impl crate::IsEnum for IntEvent2IidxStat {}
#[doc = "Field `INT_EVENT2_IIDX_STAT` reader - Interrupt index status"]
pub type IntEvent2IidxStatR = crate::FieldReader<IntEvent2IidxStat>;
impl IntEvent2IidxStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntEvent2IidxStat> {
        match self.bits {
            0 => Some(IntEvent2IidxStat::IntEvent2IidxStatNoIntr),
            9 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg0),
            10 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg1),
            11 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg2),
            12 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg3),
            13 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg4),
            14 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg5),
            15 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg6),
            16 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg7),
            17 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg8),
            18 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg9),
            19 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg10),
            20 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg11),
            21 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg12),
            22 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg13),
            23 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg14),
            24 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg15),
            25 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg16),
            26 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg17),
            27 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg18),
            28 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg19),
            29 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg20),
            30 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg21),
            31 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg22),
            32 => Some(IntEvent2IidxStat::IntEvent2IidxStatMemresifg23),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_no_intr(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatNoIntr
    }
    #[doc = "MEMRESIFG0"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg0(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg0
    }
    #[doc = "MEMRESIFG1"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg1(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg1
    }
    #[doc = "MEMRESIFG2"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg2(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg2
    }
    #[doc = "MEMRESIFG3"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg3(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg3
    }
    #[doc = "MEMRESIFG4"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg4(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg4
    }
    #[doc = "MEMRESIFG5"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg5(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg5
    }
    #[doc = "MEMRESIFG6"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg6(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg6
    }
    #[doc = "MEMRESIFG7"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg7(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg7
    }
    #[doc = "MEMRESIFG8"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg8(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg8
    }
    #[doc = "MEMRESIFG9"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg9(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg9
    }
    #[doc = "MEMRESIFG10"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg10(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg10
    }
    #[doc = "MEMRESIFG11"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg11(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg11
    }
    #[doc = "MEMRESIFG12"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg12(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg12
    }
    #[doc = "MEMRESIFG13"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg13(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg13
    }
    #[doc = "MEMRESIFG14"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg14(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg14
    }
    #[doc = "MEMRESIFG15"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg15(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg15
    }
    #[doc = "MEMRESIFG16"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg16(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg16
    }
    #[doc = "MEMRESIFG17"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg17(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg17
    }
    #[doc = "MEMRESIFG18"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg18(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg18
    }
    #[doc = "MEMRESIFG19"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg19(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg19
    }
    #[doc = "MEMRESIFG20"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg20(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg20
    }
    #[doc = "MEMRESIFG21"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg21(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg21
    }
    #[doc = "MEMRESIFG22"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg22(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg22
    }
    #[doc = "MEMRESIFG23"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_memresifg23(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatMemresifg23
    }
}
impl R {
    #[doc = "Bits 0:9 - Interrupt index status"]
    #[inline(always)]
    pub fn int_event2_iidx_stat(&self) -> IntEvent2IidxStatR {
        IntEvent2IidxStatR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event2_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent2IidxSpec;
impl crate::RegisterSpec for IntEvent2IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event2_iidx::R`](R) reader structure"]
impl crate::Readable for IntEvent2IidxSpec {}
#[doc = "`reset()` method sets INT_EVENT2_IIDX to value 0"]
impl crate::Resettable for IntEvent2IidxSpec {
    const RESET_VALUE: u32 = 0;
}
