#[doc = "Register `INT_EVENT0_IIDX` reader"]
pub type R = crate::R<IntEvent0IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntEvent0IidxStat {
    #[doc = "0: NO_INTR"]
    IntEvent0IidxStatNoIntr = 0,
    #[doc = "1: DIO0"]
    IntEvent0IidxStatDio0 = 1,
    #[doc = "2: DIO1"]
    IntEvent0IidxStatDio1 = 2,
    #[doc = "3: DIO2"]
    IntEvent0IidxStatDio2 = 3,
    #[doc = "4: DIO3"]
    IntEvent0IidxStatDio3 = 4,
    #[doc = "5: DIO4"]
    IntEvent0IidxStatDio4 = 5,
    #[doc = "6: DIO5"]
    IntEvent0IidxStatDio5 = 6,
    #[doc = "7: DIO6"]
    IntEvent0IidxStatDio6 = 7,
    #[doc = "8: DIO7"]
    IntEvent0IidxStatDio7 = 8,
    #[doc = "9: DIO8"]
    IntEvent0IidxStatDio8 = 9,
    #[doc = "10: DIO9"]
    IntEvent0IidxStatDio9 = 10,
    #[doc = "11: DIO10"]
    IntEvent0IidxStatDio10 = 11,
    #[doc = "12: DIO11"]
    IntEvent0IidxStatDio11 = 12,
    #[doc = "13: DIO12"]
    IntEvent0IidxStatDio12 = 13,
    #[doc = "14: DIO13"]
    IntEvent0IidxStatDio13 = 14,
    #[doc = "15: DIO14"]
    IntEvent0IidxStatDio14 = 15,
    #[doc = "16: DIO15"]
    IntEvent0IidxStatDio15 = 16,
    #[doc = "17: DIO16"]
    IntEvent0IidxStatDio16 = 17,
    #[doc = "18: DIO17"]
    IntEvent0IidxStatDio17 = 18,
    #[doc = "19: DIO18"]
    IntEvent0IidxStatDio18 = 19,
    #[doc = "20: DIO19"]
    IntEvent0IidxStatDio19 = 20,
    #[doc = "21: DIO20"]
    IntEvent0IidxStatDio20 = 21,
    #[doc = "22: DIO21"]
    IntEvent0IidxStatDio21 = 22,
    #[doc = "23: DIO22"]
    IntEvent0IidxStatDio22 = 23,
    #[doc = "24: DIO23"]
    IntEvent0IidxStatDio23 = 24,
    #[doc = "25: DIO24"]
    IntEvent0IidxStatDio24 = 25,
    #[doc = "26: DIO25"]
    IntEvent0IidxStatDio25 = 26,
    #[doc = "27: DIO26"]
    IntEvent0IidxStatDio26 = 27,
    #[doc = "28: DIO27"]
    IntEvent0IidxStatDio27 = 28,
    #[doc = "29: DIO28"]
    IntEvent0IidxStatDio28 = 29,
    #[doc = "30: DIO29"]
    IntEvent0IidxStatDio29 = 30,
    #[doc = "31: DIO30"]
    IntEvent0IidxStatDio30 = 31,
    #[doc = "32: DIO31"]
    IntEvent0IidxStatDio31 = 32,
}
impl From<IntEvent0IidxStat> for u8 {
    #[inline(always)]
    fn from(variant: IntEvent0IidxStat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntEvent0IidxStat {
    type Ux = u8;
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
            1 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio0),
            2 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio1),
            3 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio2),
            4 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio3),
            5 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio4),
            6 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio5),
            7 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio6),
            8 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio7),
            9 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio8),
            10 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio9),
            11 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio10),
            12 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio11),
            13 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio12),
            14 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio13),
            15 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio14),
            16 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio15),
            17 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio16),
            18 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio17),
            19 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio18),
            20 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio19),
            21 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio20),
            22 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio21),
            23 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio22),
            24 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio23),
            25 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio24),
            26 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio25),
            27 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio26),
            28 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio27),
            29 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio28),
            30 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio29),
            31 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio30),
            32 => Some(IntEvent0IidxStat::IntEvent0IidxStatDio31),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_no_intr(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatNoIntr
    }
    #[doc = "DIO0"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio0(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio0
    }
    #[doc = "DIO1"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio1(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio1
    }
    #[doc = "DIO2"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio2(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio2
    }
    #[doc = "DIO3"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio3(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio3
    }
    #[doc = "DIO4"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio4(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio4
    }
    #[doc = "DIO5"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio5(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio5
    }
    #[doc = "DIO6"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio6(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio6
    }
    #[doc = "DIO7"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio7(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio7
    }
    #[doc = "DIO8"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio8(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio8
    }
    #[doc = "DIO9"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio9(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio9
    }
    #[doc = "DIO10"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio10(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio10
    }
    #[doc = "DIO11"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio11(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio11
    }
    #[doc = "DIO12"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio12(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio12
    }
    #[doc = "DIO13"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio13(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio13
    }
    #[doc = "DIO14"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio14(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio14
    }
    #[doc = "DIO15"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio15(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio15
    }
    #[doc = "DIO16"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio16(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio16
    }
    #[doc = "DIO17"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio17(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio17
    }
    #[doc = "DIO18"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio18(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio18
    }
    #[doc = "DIO19"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio19(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio19
    }
    #[doc = "DIO20"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio20(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio20
    }
    #[doc = "DIO21"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio21(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio21
    }
    #[doc = "DIO22"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio22(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio22
    }
    #[doc = "DIO23"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio23(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio23
    }
    #[doc = "DIO24"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio24(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio24
    }
    #[doc = "DIO25"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio25(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio25
    }
    #[doc = "DIO26"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio26(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio26
    }
    #[doc = "DIO27"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio27(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio27
    }
    #[doc = "DIO28"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio28(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio28
    }
    #[doc = "DIO29"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio29(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio29
    }
    #[doc = "DIO30"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio30(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio30
    }
    #[doc = "DIO31"]
    #[inline(always)]
    pub fn is_int_event0_iidx_stat_dio31(&self) -> bool {
        *self == IntEvent0IidxStat::IntEvent0IidxStatDio31
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn int_event0_iidx_stat(&self) -> IntEvent0IidxStatR {
        IntEvent0IidxStatR::new((self.bits & 0xff) as u8)
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
