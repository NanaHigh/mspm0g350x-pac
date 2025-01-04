#[doc = "Register `INT_EVENT2_IIDX` reader"]
pub type R = crate::R<IntEvent2IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntEvent2IidxStat {
    #[doc = "0: NO_INTR"]
    IntEvent2IidxStatNoIntr = 0,
    #[doc = "1: DIO16"]
    IntEvent2IidxStatDio16 = 1,
    #[doc = "2: DIO17"]
    IntEvent2IidxStatDio17 = 2,
    #[doc = "3: DIO18"]
    IntEvent2IidxStatDio18 = 3,
    #[doc = "4: DIO19"]
    IntEvent2IidxStatDio19 = 4,
    #[doc = "5: DIO20"]
    IntEvent2IidxStatDio20 = 5,
    #[doc = "6: DIO21"]
    IntEvent2IidxStatDio21 = 6,
    #[doc = "7: DIO22"]
    IntEvent2IidxStatDio22 = 7,
    #[doc = "8: DIO23"]
    IntEvent2IidxStatDio23 = 8,
    #[doc = "9: DIO24"]
    IntEvent2IidxStatDio24 = 9,
    #[doc = "10: DIO25"]
    IntEvent2IidxStatDio25 = 10,
    #[doc = "11: DIO26"]
    IntEvent2IidxStatDio26 = 11,
    #[doc = "12: DIO27"]
    IntEvent2IidxStatDio27 = 12,
    #[doc = "13: DIO28"]
    IntEvent2IidxStatDio28 = 13,
    #[doc = "14: DIO29"]
    IntEvent2IidxStatDio29 = 14,
    #[doc = "15: DIO30"]
    IntEvent2IidxStatDio30 = 15,
    #[doc = "16: DIO31"]
    IntEvent2IidxStatDio31 = 16,
}
impl From<IntEvent2IidxStat> for u8 {
    #[inline(always)]
    fn from(variant: IntEvent2IidxStat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntEvent2IidxStat {
    type Ux = u8;
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
            1 => Some(IntEvent2IidxStat::IntEvent2IidxStatDio16),
            2 => Some(IntEvent2IidxStat::IntEvent2IidxStatDio17),
            3 => Some(IntEvent2IidxStat::IntEvent2IidxStatDio18),
            4 => Some(IntEvent2IidxStat::IntEvent2IidxStatDio19),
            5 => Some(IntEvent2IidxStat::IntEvent2IidxStatDio20),
            6 => Some(IntEvent2IidxStat::IntEvent2IidxStatDio21),
            7 => Some(IntEvent2IidxStat::IntEvent2IidxStatDio22),
            8 => Some(IntEvent2IidxStat::IntEvent2IidxStatDio23),
            9 => Some(IntEvent2IidxStat::IntEvent2IidxStatDio24),
            10 => Some(IntEvent2IidxStat::IntEvent2IidxStatDio25),
            11 => Some(IntEvent2IidxStat::IntEvent2IidxStatDio26),
            12 => Some(IntEvent2IidxStat::IntEvent2IidxStatDio27),
            13 => Some(IntEvent2IidxStat::IntEvent2IidxStatDio28),
            14 => Some(IntEvent2IidxStat::IntEvent2IidxStatDio29),
            15 => Some(IntEvent2IidxStat::IntEvent2IidxStatDio30),
            16 => Some(IntEvent2IidxStat::IntEvent2IidxStatDio31),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_no_intr(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatNoIntr
    }
    #[doc = "DIO16"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_dio16(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatDio16
    }
    #[doc = "DIO17"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_dio17(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatDio17
    }
    #[doc = "DIO18"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_dio18(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatDio18
    }
    #[doc = "DIO19"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_dio19(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatDio19
    }
    #[doc = "DIO20"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_dio20(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatDio20
    }
    #[doc = "DIO21"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_dio21(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatDio21
    }
    #[doc = "DIO22"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_dio22(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatDio22
    }
    #[doc = "DIO23"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_dio23(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatDio23
    }
    #[doc = "DIO24"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_dio24(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatDio24
    }
    #[doc = "DIO25"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_dio25(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatDio25
    }
    #[doc = "DIO26"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_dio26(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatDio26
    }
    #[doc = "DIO27"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_dio27(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatDio27
    }
    #[doc = "DIO28"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_dio28(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatDio28
    }
    #[doc = "DIO29"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_dio29(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatDio29
    }
    #[doc = "DIO30"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_dio30(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatDio30
    }
    #[doc = "DIO31"]
    #[inline(always)]
    pub fn is_int_event2_iidx_stat_dio31(&self) -> bool {
        *self == IntEvent2IidxStat::IntEvent2IidxStatDio31
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn int_event2_iidx_stat(&self) -> IntEvent2IidxStatR {
        IntEvent2IidxStatR::new((self.bits & 0xff) as u8)
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
