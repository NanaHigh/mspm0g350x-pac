#[doc = "Register `IIDX` reader"]
pub type R = crate::R<IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IidxStat {
    #[doc = "0: NO_INTR"]
    IidxStatNoIntr = 0,
    #[doc = "1: DMACH0"]
    IidxStatDmach0 = 1,
    #[doc = "2: DMACH1"]
    IidxStatDmach1 = 2,
    #[doc = "3: DMACH2"]
    IidxStatDmach2 = 3,
    #[doc = "4: DMACH3"]
    IidxStatDmach3 = 4,
    #[doc = "5: DMACH4"]
    IidxStatDmach4 = 5,
    #[doc = "6: DMACH5"]
    IidxStatDmach5 = 6,
    #[doc = "7: DMACH6"]
    IidxStatDmach6 = 7,
    #[doc = "8: DMACH7"]
    IidxStatDmach7 = 8,
    #[doc = "9: DMACH8"]
    IidxStatDmach8 = 9,
    #[doc = "10: DMACH9"]
    IidxStatDmach9 = 10,
    #[doc = "11: DMACH10"]
    IidxStatDmach10 = 11,
    #[doc = "12: DMACH11"]
    IidxStatDmach11 = 12,
    #[doc = "13: DMACH12"]
    IidxStatDmach12 = 13,
    #[doc = "14: DMACH13"]
    IidxStatDmach13 = 14,
    #[doc = "15: DMACH14"]
    IidxStatDmach14 = 15,
    #[doc = "16: DMACH15"]
    IidxStatDmach15 = 16,
    #[doc = "17: PREIRQCH0"]
    IidxStatPreirqch0 = 17,
    #[doc = "18: PREIRQCH1"]
    IidxStatPreirqch1 = 18,
    #[doc = "19: PREIRQCH2"]
    IidxStatPreirqch2 = 19,
    #[doc = "20: PREIRQCH3"]
    IidxStatPreirqch3 = 20,
    #[doc = "21: PREIRQCH4"]
    IidxStatPreirqch4 = 21,
    #[doc = "22: PREIRQCH5"]
    IidxStatPreirqch5 = 22,
    #[doc = "23: PREIRQCH6"]
    IidxStatPreirqch6 = 23,
    #[doc = "24: PREIRQCH7"]
    IidxStatPreirqch7 = 24,
    #[doc = "25: ADDRERR"]
    IidxStatAddrerr = 25,
    #[doc = "26: DATAERR"]
    IidxStatDataerr = 26,
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
            1 => Some(IidxStat::IidxStatDmach0),
            2 => Some(IidxStat::IidxStatDmach1),
            3 => Some(IidxStat::IidxStatDmach2),
            4 => Some(IidxStat::IidxStatDmach3),
            5 => Some(IidxStat::IidxStatDmach4),
            6 => Some(IidxStat::IidxStatDmach5),
            7 => Some(IidxStat::IidxStatDmach6),
            8 => Some(IidxStat::IidxStatDmach7),
            9 => Some(IidxStat::IidxStatDmach8),
            10 => Some(IidxStat::IidxStatDmach9),
            11 => Some(IidxStat::IidxStatDmach10),
            12 => Some(IidxStat::IidxStatDmach11),
            13 => Some(IidxStat::IidxStatDmach12),
            14 => Some(IidxStat::IidxStatDmach13),
            15 => Some(IidxStat::IidxStatDmach14),
            16 => Some(IidxStat::IidxStatDmach15),
            17 => Some(IidxStat::IidxStatPreirqch0),
            18 => Some(IidxStat::IidxStatPreirqch1),
            19 => Some(IidxStat::IidxStatPreirqch2),
            20 => Some(IidxStat::IidxStatPreirqch3),
            21 => Some(IidxStat::IidxStatPreirqch4),
            22 => Some(IidxStat::IidxStatPreirqch5),
            23 => Some(IidxStat::IidxStatPreirqch6),
            24 => Some(IidxStat::IidxStatPreirqch7),
            25 => Some(IidxStat::IidxStatAddrerr),
            26 => Some(IidxStat::IidxStatDataerr),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_iidx_stat_no_intr(&self) -> bool {
        *self == IidxStat::IidxStatNoIntr
    }
    #[doc = "DMACH0"]
    #[inline(always)]
    pub fn is_iidx_stat_dmach0(&self) -> bool {
        *self == IidxStat::IidxStatDmach0
    }
    #[doc = "DMACH1"]
    #[inline(always)]
    pub fn is_iidx_stat_dmach1(&self) -> bool {
        *self == IidxStat::IidxStatDmach1
    }
    #[doc = "DMACH2"]
    #[inline(always)]
    pub fn is_iidx_stat_dmach2(&self) -> bool {
        *self == IidxStat::IidxStatDmach2
    }
    #[doc = "DMACH3"]
    #[inline(always)]
    pub fn is_iidx_stat_dmach3(&self) -> bool {
        *self == IidxStat::IidxStatDmach3
    }
    #[doc = "DMACH4"]
    #[inline(always)]
    pub fn is_iidx_stat_dmach4(&self) -> bool {
        *self == IidxStat::IidxStatDmach4
    }
    #[doc = "DMACH5"]
    #[inline(always)]
    pub fn is_iidx_stat_dmach5(&self) -> bool {
        *self == IidxStat::IidxStatDmach5
    }
    #[doc = "DMACH6"]
    #[inline(always)]
    pub fn is_iidx_stat_dmach6(&self) -> bool {
        *self == IidxStat::IidxStatDmach6
    }
    #[doc = "DMACH7"]
    #[inline(always)]
    pub fn is_iidx_stat_dmach7(&self) -> bool {
        *self == IidxStat::IidxStatDmach7
    }
    #[doc = "DMACH8"]
    #[inline(always)]
    pub fn is_iidx_stat_dmach8(&self) -> bool {
        *self == IidxStat::IidxStatDmach8
    }
    #[doc = "DMACH9"]
    #[inline(always)]
    pub fn is_iidx_stat_dmach9(&self) -> bool {
        *self == IidxStat::IidxStatDmach9
    }
    #[doc = "DMACH10"]
    #[inline(always)]
    pub fn is_iidx_stat_dmach10(&self) -> bool {
        *self == IidxStat::IidxStatDmach10
    }
    #[doc = "DMACH11"]
    #[inline(always)]
    pub fn is_iidx_stat_dmach11(&self) -> bool {
        *self == IidxStat::IidxStatDmach11
    }
    #[doc = "DMACH12"]
    #[inline(always)]
    pub fn is_iidx_stat_dmach12(&self) -> bool {
        *self == IidxStat::IidxStatDmach12
    }
    #[doc = "DMACH13"]
    #[inline(always)]
    pub fn is_iidx_stat_dmach13(&self) -> bool {
        *self == IidxStat::IidxStatDmach13
    }
    #[doc = "DMACH14"]
    #[inline(always)]
    pub fn is_iidx_stat_dmach14(&self) -> bool {
        *self == IidxStat::IidxStatDmach14
    }
    #[doc = "DMACH15"]
    #[inline(always)]
    pub fn is_iidx_stat_dmach15(&self) -> bool {
        *self == IidxStat::IidxStatDmach15
    }
    #[doc = "PREIRQCH0"]
    #[inline(always)]
    pub fn is_iidx_stat_preirqch0(&self) -> bool {
        *self == IidxStat::IidxStatPreirqch0
    }
    #[doc = "PREIRQCH1"]
    #[inline(always)]
    pub fn is_iidx_stat_preirqch1(&self) -> bool {
        *self == IidxStat::IidxStatPreirqch1
    }
    #[doc = "PREIRQCH2"]
    #[inline(always)]
    pub fn is_iidx_stat_preirqch2(&self) -> bool {
        *self == IidxStat::IidxStatPreirqch2
    }
    #[doc = "PREIRQCH3"]
    #[inline(always)]
    pub fn is_iidx_stat_preirqch3(&self) -> bool {
        *self == IidxStat::IidxStatPreirqch3
    }
    #[doc = "PREIRQCH4"]
    #[inline(always)]
    pub fn is_iidx_stat_preirqch4(&self) -> bool {
        *self == IidxStat::IidxStatPreirqch4
    }
    #[doc = "PREIRQCH5"]
    #[inline(always)]
    pub fn is_iidx_stat_preirqch5(&self) -> bool {
        *self == IidxStat::IidxStatPreirqch5
    }
    #[doc = "PREIRQCH6"]
    #[inline(always)]
    pub fn is_iidx_stat_preirqch6(&self) -> bool {
        *self == IidxStat::IidxStatPreirqch6
    }
    #[doc = "PREIRQCH7"]
    #[inline(always)]
    pub fn is_iidx_stat_preirqch7(&self) -> bool {
        *self == IidxStat::IidxStatPreirqch7
    }
    #[doc = "ADDRERR"]
    #[inline(always)]
    pub fn is_iidx_stat_addrerr(&self) -> bool {
        *self == IidxStat::IidxStatAddrerr
    }
    #[doc = "DATAERR"]
    #[inline(always)]
    pub fn is_iidx_stat_dataerr(&self) -> bool {
        *self == IidxStat::IidxStatDataerr
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
