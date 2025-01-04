#[doc = "Register `IIDX` reader"]
pub type R = crate::R<IidxSpec>;
#[doc = "Interrupt index status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IidxStat {
    #[doc = "0: NO_INTR"]
    IidxStatNoIntr = 0,
    #[doc = "1: OUTRDYIFG"]
    IidxStatOutrdyifg = 1,
    #[doc = "2: COMPIFG"]
    IidxStatCompifg = 2,
    #[doc = "3: COMPINVIFG"]
    IidxStatCompinvifg = 3,
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
    pub const fn variant(&self) -> IidxStat {
        match self.bits {
            0 => IidxStat::IidxStatNoIntr,
            1 => IidxStat::IidxStatOutrdyifg,
            2 => IidxStat::IidxStatCompifg,
            3 => IidxStat::IidxStatCompinvifg,
            _ => unreachable!(),
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_iidx_stat_no_intr(&self) -> bool {
        *self == IidxStat::IidxStatNoIntr
    }
    #[doc = "OUTRDYIFG"]
    #[inline(always)]
    pub fn is_iidx_stat_outrdyifg(&self) -> bool {
        *self == IidxStat::IidxStatOutrdyifg
    }
    #[doc = "COMPIFG"]
    #[inline(always)]
    pub fn is_iidx_stat_compifg(&self) -> bool {
        *self == IidxStat::IidxStatCompifg
    }
    #[doc = "COMPINVIFG"]
    #[inline(always)]
    pub fn is_iidx_stat_compinvifg(&self) -> bool {
        *self == IidxStat::IidxStatCompinvifg
    }
}
impl R {
    #[doc = "Bits 0:1 - Interrupt index status"]
    #[inline(always)]
    pub fn iidx_stat(&self) -> IidxStatR {
        IidxStatR::new((self.bits & 3) as u8)
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
