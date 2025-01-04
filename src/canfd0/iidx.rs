#[doc = "Register `IIDX` reader"]
pub type R = crate::R<IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IidxStat {
    #[doc = "0: NO_INTR"]
    IidxStatNoIntr = 0,
    #[doc = "1: INTL0"]
    IidxStatIntl0 = 1,
    #[doc = "2: INTL1"]
    IidxStatIntl1 = 2,
    #[doc = "3: SEC"]
    IidxStatSec = 3,
    #[doc = "4: DED"]
    IidxStatDed = 4,
    #[doc = "5: EXT_TS_CNTR_OVFL"]
    IidxStatExtTsCntrOvfl = 5,
    #[doc = "6: WAKEUP"]
    IidxStatWakeup = 6,
}
impl From<IidxStat> for u8 {
    #[inline(always)]
    fn from(variant: IidxStat) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IidxStat {
    type Ux = u8;
}
impl crate::IsEnum for IidxStat {}
#[doc = "Field `IIDX_STAT` reader - Interrupt index status"]
pub type IidxStatR = crate::FieldReader<IidxStat>;
impl IidxStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IidxStat> {
        match self.bits {
            0 => Some(IidxStat::IidxStatNoIntr),
            1 => Some(IidxStat::IidxStatIntl0),
            2 => Some(IidxStat::IidxStatIntl1),
            3 => Some(IidxStat::IidxStatSec),
            4 => Some(IidxStat::IidxStatDed),
            5 => Some(IidxStat::IidxStatExtTsCntrOvfl),
            6 => Some(IidxStat::IidxStatWakeup),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_iidx_stat_no_intr(&self) -> bool {
        *self == IidxStat::IidxStatNoIntr
    }
    #[doc = "INTL0"]
    #[inline(always)]
    pub fn is_iidx_stat_intl0(&self) -> bool {
        *self == IidxStat::IidxStatIntl0
    }
    #[doc = "INTL1"]
    #[inline(always)]
    pub fn is_iidx_stat_intl1(&self) -> bool {
        *self == IidxStat::IidxStatIntl1
    }
    #[doc = "SEC"]
    #[inline(always)]
    pub fn is_iidx_stat_sec(&self) -> bool {
        *self == IidxStat::IidxStatSec
    }
    #[doc = "DED"]
    #[inline(always)]
    pub fn is_iidx_stat_ded(&self) -> bool {
        *self == IidxStat::IidxStatDed
    }
    #[doc = "EXT_TS_CNTR_OVFL"]
    #[inline(always)]
    pub fn is_iidx_stat_ext_ts_cntr_ovfl(&self) -> bool {
        *self == IidxStat::IidxStatExtTsCntrOvfl
    }
    #[doc = "WAKEUP"]
    #[inline(always)]
    pub fn is_iidx_stat_wakeup(&self) -> bool {
        *self == IidxStat::IidxStatWakeup
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn iidx_stat(&self) -> IidxStatR {
        IidxStatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IidxSpec;
impl crate::RegisterSpec for IidxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iidx::R`](R) reader structure"]
impl crate::Readable for IidxSpec {}
#[doc = "`reset()` method sets IIDX to value 0"]
impl crate::Resettable for IidxSpec {
    const RESET_VALUE: u32 = 0;
}
