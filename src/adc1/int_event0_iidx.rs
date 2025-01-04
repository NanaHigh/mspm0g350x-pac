#[doc = "Register `INT_EVENT0_IIDX` reader"]
pub type R = crate::R<IntEvent0IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum IntEvent0IidxStat {
    #[doc = "0: NO_INTR"]
    IntEvent0IidxStatNoIntr = 0,
    #[doc = "1: OVIFG"]
    IntEvent0IidxStatOvifg = 1,
    #[doc = "2: TOVIFG"]
    IntEvent0IidxStatTovifg = 2,
    #[doc = "3: HIGHIFG"]
    IntEvent0IidxStatHighifg = 3,
    #[doc = "4: LOWIFG"]
    IntEvent0IidxStatLowifg = 4,
    #[doc = "5: INIFG"]
    IntEvent0IidxStatInifg = 5,
    #[doc = "6: DMADONE"]
    IntEvent0IidxStatDmadone = 6,
    #[doc = "7: UVIFG"]
    IntEvent0IidxStatUvifg = 7,
    #[doc = "9: MEMRESIFG0"]
    IntEvent0IidxStatMemresifg0 = 9,
    #[doc = "10: MEMRESIFG1"]
    IntEvent0IidxStatMemresifg1 = 10,
    #[doc = "11: MEMRESIFG2"]
    IntEvent0IidxStatMemresifg2 = 11,
    #[doc = "12: MEMRESIFG3"]
    IntEvent0IidxStatMemresifg3 = 12,
    #[doc = "13: MEMRESIFG4"]
    IntEvent0IidxStatMemresifg4 = 13,
    #[doc = "14: MEMRESIFG5"]
    IntEvent0IidxStatMemresifg5 = 14,
    #[doc = "15: MEMRESIFG6"]
    IntEvent0IidxStatMemresifg6 = 15,
    #[doc = "16: MEMRESIFG7"]
    IntEvent0IidxStatMemresifg7 = 16,
    #[doc = "17: MEMRESIFG8"]
    IntEvent0IidxStatMemresifg8 = 17,
    #[doc = "18: MEMRESIFG9"]
    IntEvent0IidxStatMemresifg9 = 18,
    #[doc = "19: MEMRESIFG10"]
    IntEvent0IidxStatMemresifg10 = 19,
    #[doc = "20: MEMRESIFG11"]
    IntEvent0IidxStatMemresifg11 = 20,
    #[doc = "21: MEMRESIFG12"]
    IntEvent0IidxStatMemresifg12 = 21,
    #[doc = "22: MEMRESIFG13"]
    IntEvent0IidxStatMemresifg13 = 22,
    #[doc = "23: MEMRESIFG14"]
    IntEvent0IidxStatMemresifg14 = 23,
    #[doc = "24: MEMRESIFG15"]
    IntEvent0IidxStatMemresifg15 = 24,
    #[doc = "25: MEMRESIFG16"]
    IntEvent0IidxStatMemresifg16 = 25,
    #[doc = "26: MEMRESIFG17"]
    IntEvent0IidxStatMemresifg17 = 26,
    #[doc = "27: MEMRESIFG18"]
    IntEvent0IidxStatMemresifg18 = 27,
    #[doc = "28: MEMRESIFG19"]
    IntEvent0IidxStatMemresifg19 = 28,
    #[doc = "29: MEMRESIFG20"]
    IntEvent0IidxStatMemresifg20 = 29,
    #[doc = "30: MEMRESIFG21"]
    IntEvent0IidxStatMemresifg21 = 30,
    #[doc = "31: MEMRESIFG22"]
    IntEvent0IidxStatMemresifg22 = 31,
    #[doc = "32: MEMRESIFG23"]
    IntEvent0IidxStatMemresifg23 = 32,
}
impl From<IntEvent0IidxStat> for u16 {
    #[inline(always)]
    fn from(variant: IntEvent0IidxStat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntEvent0IidxStat {
    type Ux = u16;
}
impl crate::IsEnum for IntEvent0IidxStat {}
#[doc = "Field `INT_EVENT0_IIDX_STAT` reader - Interrupt index status"]
pub type IntEvent0IidxStatR = crate::FieldReader<IntEvent0IidxStat>;
impl IntEvent0IidxStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntEvent0IidxStat> {
        match self.bits {
            0 => Some(IntEvent0IidxStat::IntEvent0IidxStatNoIntr),
            1 => Some(IntEvent0IidxStat::IntEvent0IidxStatOvifg),
            2 => Some(IntEvent0IidxStat::IntEvent0IidxStatTovifg),
            3 => Some(IntEvent0IidxStat::IntEvent0IidxStatHighifg),
            4 => Some(IntEvent0IidxStat::IntEvent0IidxStatLowifg),
            5 => Some(IntEvent0IidxStat::IntEvent0IidxStatInifg),
            6 => Some(IntEvent0IidxStat::IntEvent0IidxStatDmadone),
            7 => Some(IntEvent0IidxStat::IntEvent0IidxStatUvifg),
            9 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg0),
            10 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg1),
            11 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg2),
            12 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg3),
            13 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg4),
            14 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg5),
            15 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg6),
            16 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg7),
            17 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg8),
            18 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg9),
            19 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg10),
            20 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg11),
            21 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg12),
            22 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg13),
            23 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg14),
            24 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg15),
            25 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg16),
            26 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg17),
            27 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg18),
            28 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg19),
            29 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg20),
            30 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg21),
            31 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg22),
            32 => Some(IntEvent0IidxStat::IntEvent0IidxStatMemresifg23),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_no_intr(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatNoIntr
    }
    #[doc = "OVIFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_ovifg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatOvifg
    }
    #[doc = "TOVIFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_tovifg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatTovifg
    }
    #[doc = "HIGHIFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_highifg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatHighifg
    }
    #[doc = "LOWIFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_lowifg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatLowifg
    }
    #[doc = "INIFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_inifg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatInifg
    }
    #[doc = "DMADONE"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dmadone(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDmadone
    }
    #[doc = "UVIFG"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_uvifg(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatUvifg
    }
    #[doc = "MEMRESIFG0"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg0(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg0
    }
    #[doc = "MEMRESIFG1"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg1(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg1
    }
    #[doc = "MEMRESIFG2"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg2(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg2
    }
    #[doc = "MEMRESIFG3"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg3(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg3
    }
    #[doc = "MEMRESIFG4"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg4(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg4
    }
    #[doc = "MEMRESIFG5"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg5(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg5
    }
    #[doc = "MEMRESIFG6"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg6(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg6
    }
    #[doc = "MEMRESIFG7"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg7(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg7
    }
    #[doc = "MEMRESIFG8"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg8(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg8
    }
    #[doc = "MEMRESIFG9"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg9(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg9
    }
    #[doc = "MEMRESIFG10"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg10(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg10
    }
    #[doc = "MEMRESIFG11"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg11(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg11
    }
    #[doc = "MEMRESIFG12"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg12(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg12
    }
    #[doc = "MEMRESIFG13"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg13(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg13
    }
    #[doc = "MEMRESIFG14"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg14(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg14
    }
    #[doc = "MEMRESIFG15"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg15(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg15
    }
    #[doc = "MEMRESIFG16"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg16(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg16
    }
    #[doc = "MEMRESIFG17"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg17(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg17
    }
    #[doc = "MEMRESIFG18"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg18(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg18
    }
    #[doc = "MEMRESIFG19"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg19(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg19
    }
    #[doc = "MEMRESIFG20"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg20(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg20
    }
    #[doc = "MEMRESIFG21"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg21(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg21
    }
    #[doc = "MEMRESIFG22"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg22(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg22
    }
    #[doc = "MEMRESIFG23"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_memresifg23(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatMemresifg23
    }
}
impl R {
    #[doc = "Bits 0:9 - Interrupt index status"]
    #[inline(always)]
    pub fn int_event0_iidx_stat(&self) -> IntEvent0IidxStatR {
        IntEvent0IidxStatR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event0_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent0IidxSpec;
impl crate::RegisterSpec for IntEvent0IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event0_iidx::R`](R) reader structure"]
impl crate::Readable for IntEvent0IidxSpec {}
#[doc = "`reset()` method sets INT_EVENT0_IIDX to value 0"]
impl crate::Resettable for IntEvent0IidxSpec {
    const RESET_VALUE: u32 = 0;
}
