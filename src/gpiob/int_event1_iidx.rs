#[doc = "Register `INT_EVENT1_IIDX` reader"]
pub type R = crate::R<IntEvent1IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntEvent1IidxStat {
    #[doc = "0: NO_INTR"]
    IntEvent1IidxStatNoIntr = 0,
    #[doc = "1: DIO0"]
    IntEvent1IidxStatDio0 = 1,
    #[doc = "2: DIO1"]
    IntEvent1IidxStatDio1 = 2,
    #[doc = "3: DIO2"]
    IntEvent1IidxStatDio2 = 3,
    #[doc = "4: DIO3"]
    IntEvent1IidxStatDio3 = 4,
    #[doc = "5: DIO4"]
    IntEvent1IidxStatDio4 = 5,
    #[doc = "6: DIO5"]
    IntEvent1IidxStatDio5 = 6,
    #[doc = "7: DIO6"]
    IntEvent1IidxStatDio6 = 7,
    #[doc = "8: DIO7"]
    IntEvent1IidxStatDio7 = 8,
    #[doc = "9: DIO8"]
    IntEvent1IidxStatDio8 = 9,
    #[doc = "10: DIO9"]
    IntEvent1IidxStatDio9 = 10,
    #[doc = "11: DIO10"]
    IntEvent1IidxStatDio10 = 11,
    #[doc = "12: DIO11"]
    IntEvent1IidxStatDio11 = 12,
    #[doc = "13: DIO12"]
    IntEvent1IidxStatDio12 = 13,
    #[doc = "14: DIO13"]
    IntEvent1IidxStatDio13 = 14,
    #[doc = "15: DIO14"]
    IntEvent1IidxStatDio14 = 15,
    #[doc = "16: DIO15"]
    IntEvent1IidxStatDio15 = 16,
}
impl From<IntEvent1IidxStat> for u8 {
    #[inline(always)]
    fn from(variant: IntEvent1IidxStat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntEvent1IidxStat {
    type Ux = u8;
}
impl crate::IsEnum for IntEvent1IidxStat {}
#[doc = "Field `INT_EVENT1_IIDX_STAT` reader - Interrupt index status"]
pub type IntEvent1IidxStatR = crate::FieldReader<IntEvent1IidxStat>;
impl IntEvent1IidxStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntEvent1IidxStat> {
        match self.bits {
            0 => Some(IntEvent1IidxStat::IntEvent1IidxStatNoIntr),
            1 => Some(IntEvent1IidxStat::IntEvent1IidxStatDio0),
            2 => Some(IntEvent1IidxStat::IntEvent1IidxStatDio1),
            3 => Some(IntEvent1IidxStat::IntEvent1IidxStatDio2),
            4 => Some(IntEvent1IidxStat::IntEvent1IidxStatDio3),
            5 => Some(IntEvent1IidxStat::IntEvent1IidxStatDio4),
            6 => Some(IntEvent1IidxStat::IntEvent1IidxStatDio5),
            7 => Some(IntEvent1IidxStat::IntEvent1IidxStatDio6),
            8 => Some(IntEvent1IidxStat::IntEvent1IidxStatDio7),
            9 => Some(IntEvent1IidxStat::IntEvent1IidxStatDio8),
            10 => Some(IntEvent1IidxStat::IntEvent1IidxStatDio9),
            11 => Some(IntEvent1IidxStat::IntEvent1IidxStatDio10),
            12 => Some(IntEvent1IidxStat::IntEvent1IidxStatDio11),
            13 => Some(IntEvent1IidxStat::IntEvent1IidxStatDio12),
            14 => Some(IntEvent1IidxStat::IntEvent1IidxStatDio13),
            15 => Some(IntEvent1IidxStat::IntEvent1IidxStatDio14),
            16 => Some(IntEvent1IidxStat::IntEvent1IidxStatDio15),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_no_intr(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatNoIntr
    }
    #[doc = "DIO0"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_dio0(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatDio0
    }
    #[doc = "DIO1"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_dio1(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatDio1
    }
    #[doc = "DIO2"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_dio2(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatDio2
    }
    #[doc = "DIO3"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_dio3(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatDio3
    }
    #[doc = "DIO4"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_dio4(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatDio4
    }
    #[doc = "DIO5"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_dio5(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatDio5
    }
    #[doc = "DIO6"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_dio6(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatDio6
    }
    #[doc = "DIO7"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_dio7(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatDio7
    }
    #[doc = "DIO8"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_dio8(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatDio8
    }
    #[doc = "DIO9"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_dio9(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatDio9
    }
    #[doc = "DIO10"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_dio10(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatDio10
    }
    #[doc = "DIO11"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_dio11(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatDio11
    }
    #[doc = "DIO12"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_dio12(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatDio12
    }
    #[doc = "DIO13"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_dio13(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatDio13
    }
    #[doc = "DIO14"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_dio14(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatDio14
    }
    #[doc = "DIO15"]
    #[inline(always)]
    pub fn is_int_event1_iidx_stat_dio15(&self) -> bool {
        *self == IntEvent1IidxStat::IntEvent1IidxStatDio15
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn int_event1_iidx_stat(&self) -> IntEvent1IidxStatR {
        IntEvent1IidxStatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`int_event1_iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEvent1IidxSpec;
impl crate::RegisterSpec for IntEvent1IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_event1_iidx::R`](R) reader structure"]
impl crate::Readable for IntEvent1IidxSpec {}
#[doc = "`reset()` method sets INT_EVENT1_IIDX to value 0"]
impl crate::Resettable for IntEvent1IidxSpec {
    const RESET_VALUE: u32 = 0;
}
