#[doc = "Register `IIDX` reader"]
pub type R = crate::R<IidxSpec>;
#[doc = "The SYSCTL interrupt index (IIDX) register generates a value corresponding to the highest priority pending interrupt source. This value may be used as an address offset for fast, deterministic handling in the interrupt service routine. A read of the IIDX register will clear the corresponding interrupt status in the RIS and MIS registers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IidxStat {
    #[doc = "0: NO_INTR"]
    IidxStatNoIntr = 0,
    #[doc = "1: LFOSCGOOD"]
    IidxStatLfoscgood = 1,
    #[doc = "2: ANACLKERR"]
    IidxStatAnaclkerr = 2,
    #[doc = "3: FLASHSEC"]
    IidxStatFlashsec = 3,
    #[doc = "4: SRAMSEC"]
    IidxStatSramsec = 4,
    #[doc = "5: LFXTGOOD"]
    IidxStatLfxtgood = 5,
    #[doc = "6: HFCLKGOOD"]
    IidxStatHfclkgood = 6,
    #[doc = "7: SYSPLLGOOD"]
    IidxStatSyspllgood = 7,
    #[doc = "8: HSCLKGOOD"]
    IidxStatHsclkgood = 8,
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
#[doc = "Field `IIDX_STAT` reader - The SYSCTL interrupt index (IIDX) register generates a value corresponding to the highest priority pending interrupt source. This value may be used as an address offset for fast, deterministic handling in the interrupt service routine. A read of the IIDX register will clear the corresponding interrupt status in the RIS and MIS registers."]
pub type IidxStatR = crate::FieldReader<IidxStat>;
impl IidxStatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IidxStat> {
        match self.bits {
            0 => Some(IidxStat::IidxStatNoIntr),
            1 => Some(IidxStat::IidxStatLfoscgood),
            2 => Some(IidxStat::IidxStatAnaclkerr),
            3 => Some(IidxStat::IidxStatFlashsec),
            4 => Some(IidxStat::IidxStatSramsec),
            5 => Some(IidxStat::IidxStatLfxtgood),
            6 => Some(IidxStat::IidxStatHfclkgood),
            7 => Some(IidxStat::IidxStatSyspllgood),
            8 => Some(IidxStat::IidxStatHsclkgood),
            _ => None,
        }
    }
    #[doc = "NO_INTR"]
    #[inline(always)]
    pub fn is_iidx_stat_no_intr(&self) -> bool {
        *self == IidxStat::IidxStatNoIntr
    }
    #[doc = "LFOSCGOOD"]
    #[inline(always)]
    pub fn is_iidx_stat_lfoscgood(&self) -> bool {
        *self == IidxStat::IidxStatLfoscgood
    }
    #[doc = "ANACLKERR"]
    #[inline(always)]
    pub fn is_iidx_stat_anaclkerr(&self) -> bool {
        *self == IidxStat::IidxStatAnaclkerr
    }
    #[doc = "FLASHSEC"]
    #[inline(always)]
    pub fn is_iidx_stat_flashsec(&self) -> bool {
        *self == IidxStat::IidxStatFlashsec
    }
    #[doc = "SRAMSEC"]
    #[inline(always)]
    pub fn is_iidx_stat_sramsec(&self) -> bool {
        *self == IidxStat::IidxStatSramsec
    }
    #[doc = "LFXTGOOD"]
    #[inline(always)]
    pub fn is_iidx_stat_lfxtgood(&self) -> bool {
        *self == IidxStat::IidxStatLfxtgood
    }
    #[doc = "HFCLKGOOD"]
    #[inline(always)]
    pub fn is_iidx_stat_hfclkgood(&self) -> bool {
        *self == IidxStat::IidxStatHfclkgood
    }
    #[doc = "SYSPLLGOOD"]
    #[inline(always)]
    pub fn is_iidx_stat_syspllgood(&self) -> bool {
        *self == IidxStat::IidxStatSyspllgood
    }
    #[doc = "HSCLKGOOD"]
    #[inline(always)]
    pub fn is_iidx_stat_hsclkgood(&self) -> bool {
        *self == IidxStat::IidxStatHsclkgood
    }
}
impl R {
    #[doc = "Bits 0:3 - The SYSCTL interrupt index (IIDX) register generates a value corresponding to the highest priority pending interrupt source. This value may be used as an address offset for fast, deterministic handling in the interrupt service routine. A read of the IIDX register will clear the corresponding interrupt status in the RIS and MIS registers."]
    #[inline(always)]
    pub fn iidx_stat(&self) -> IidxStatR {
        IidxStatR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "SYSCTL interrupt index\n\nYou can [`read`](crate::Reg::read) this register and get [`iidx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
