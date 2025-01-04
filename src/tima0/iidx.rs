#[doc = "Register `IIDX` reader"]
pub type R = crate::R<IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IidxStat {
    #[doc = "0: NO_INTR"]
    IidxStatNoIntr = 0,
    #[doc = "1: Z"]
    IidxStatZ = 1,
    #[doc = "2: L"]
    IidxStatL = 2,
    #[doc = "5: CCD0"]
    IidxStatCcd0 = 5,
    #[doc = "6: CCD1"]
    IidxStatCcd1 = 6,
    #[doc = "7: CCD2"]
    IidxStatCcd2 = 7,
    #[doc = "8: CCD3"]
    IidxStatCcd3 = 8,
    #[doc = "9: CCU0"]
    IidxStatCcu0 = 9,
    #[doc = "10: CCU1"]
    IidxStatCcu1 = 10,
    #[doc = "11: CCU2"]
    IidxStatCcu2 = 11,
    #[doc = "12: CCU3"]
    IidxStatCcu3 = 12,
    #[doc = "13: CCD4"]
    IidxStatCcd4 = 13,
    #[doc = "14: CCD5"]
    IidxStatCcd5 = 14,
    #[doc = "15: CCU4"]
    IidxStatCcu4 = 15,
    #[doc = "16: CCU5"]
    IidxStatCcu5 = 16,
    #[doc = "25: F"]
    IidxStatF = 25,
    #[doc = "26: TOV"]
    IidxStatTov = 26,
    #[doc = "27: REPC"]
    IidxStatRepc = 27,
    #[doc = "28: DC"]
    IidxStatDc = 28,
    #[doc = "29: QEIERR"]
    IidxStatQeierr = 29,
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
            1 => Some(IidxStat::IidxStatZ),
            2 => Some(IidxStat::IidxStatL),
            5 => Some(IidxStat::IidxStatCcd0),
            6 => Some(IidxStat::IidxStatCcd1),
            7 => Some(IidxStat::IidxStatCcd2),
            8 => Some(IidxStat::IidxStatCcd3),
            9 => Some(IidxStat::IidxStatCcu0),
            10 => Some(IidxStat::IidxStatCcu1),
            11 => Some(IidxStat::IidxStatCcu2),
            12 => Some(IidxStat::IidxStatCcu3),
            13 => Some(IidxStat::IidxStatCcd4),
            14 => Some(IidxStat::IidxStatCcd5),
            15 => Some(IidxStat::IidxStatCcu4),
            16 => Some(IidxStat::IidxStatCcu5),
            25 => Some(IidxStat::IidxStatF),
            26 => Some(IidxStat::IidxStatTov),
            27 => Some(IidxStat::IidxStatRepc),
            28 => Some(IidxStat::IidxStatDc),
            29 => Some(IidxStat::IidxStatQeierr),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_iidx_stat_no_intr(&self) -> bool {
        *self == IidxStat::IidxStatNoIntr
    }
    #[doc = "Z"]
    #[inline(always)]
    pub fn is_iidx_stat_z(&self) -> bool {
        *self == IidxStat::IidxStatZ
    }
    #[doc = "L"]
    #[inline(always)]
    pub fn is_iidx_stat_l(&self) -> bool {
        *self == IidxStat::IidxStatL
    }
    #[doc = "CCD0"]
    #[inline(always)]
    pub fn is_iidx_stat_ccd0(&self) -> bool {
        *self == IidxStat::IidxStatCcd0
    }
    #[doc = "CCD1"]
    #[inline(always)]
    pub fn is_iidx_stat_ccd1(&self) -> bool {
        *self == IidxStat::IidxStatCcd1
    }
    #[doc = "CCD2"]
    #[inline(always)]
    pub fn is_iidx_stat_ccd2(&self) -> bool {
        *self == IidxStat::IidxStatCcd2
    }
    #[doc = "CCD3"]
    #[inline(always)]
    pub fn is_iidx_stat_ccd3(&self) -> bool {
        *self == IidxStat::IidxStatCcd3
    }
    #[doc = "CCU0"]
    #[inline(always)]
    pub fn is_iidx_stat_ccu0(&self) -> bool {
        *self == IidxStat::IidxStatCcu0
    }
    #[doc = "CCU1"]
    #[inline(always)]
    pub fn is_iidx_stat_ccu1(&self) -> bool {
        *self == IidxStat::IidxStatCcu1
    }
    #[doc = "CCU2"]
    #[inline(always)]
    pub fn is_iidx_stat_ccu2(&self) -> bool {
        *self == IidxStat::IidxStatCcu2
    }
    #[doc = "CCU3"]
    #[inline(always)]
    pub fn is_iidx_stat_ccu3(&self) -> bool {
        *self == IidxStat::IidxStatCcu3
    }
    #[doc = "CCD4"]
    #[inline(always)]
    pub fn is_iidx_stat_ccd4(&self) -> bool {
        *self == IidxStat::IidxStatCcd4
    }
    #[doc = "CCD5"]
    #[inline(always)]
    pub fn is_iidx_stat_ccd5(&self) -> bool {
        *self == IidxStat::IidxStatCcd5
    }
    #[doc = "CCU4"]
    #[inline(always)]
    pub fn is_iidx_stat_ccu4(&self) -> bool {
        *self == IidxStat::IidxStatCcu4
    }
    #[doc = "CCU5"]
    #[inline(always)]
    pub fn is_iidx_stat_ccu5(&self) -> bool {
        *self == IidxStat::IidxStatCcu5
    }
    #[doc = "F"]
    #[inline(always)]
    pub fn is_iidx_stat_f(&self) -> bool {
        *self == IidxStat::IidxStatF
    }
    #[doc = "TOV"]
    #[inline(always)]
    pub fn is_iidx_stat_tov(&self) -> bool {
        *self == IidxStat::IidxStatTov
    }
    #[doc = "REPC"]
    #[inline(always)]
    pub fn is_iidx_stat_repc(&self) -> bool {
        *self == IidxStat::IidxStatRepc
    }
    #[doc = "DC"]
    #[inline(always)]
    pub fn is_iidx_stat_dc(&self) -> bool {
        *self == IidxStat::IidxStatDc
    }
    #[doc = "QEIERR"]
    #[inline(always)]
    pub fn is_iidx_stat_qeierr(&self) -> bool {
        *self == IidxStat::IidxStatQeierr
    }
}
impl R {
    #[doc = "Bits 0:7 - Interrupt index status"]
    #[inline(always)]
    pub fn iidx_stat(&self) -> IidxStatR {
        IidxStatR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
