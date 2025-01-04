#[doc = "Register `RSTCAUSE` reader"]
pub type R = crate::R<RstcauseSpec>;
#[doc = "ID is a read-to-clear field which indicates the lowest level reset cause since the last read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RstcauseId {
    #[doc = "0: NORST"]
    RstcauseIdNorst = 0,
    #[doc = "1: PORHWFAIL"]
    RstcauseIdPorhwfail = 1,
    #[doc = "2: POREXNRST"]
    RstcauseIdPorexnrst = 2,
    #[doc = "3: PORSW"]
    RstcauseIdPorsw = 3,
    #[doc = "4: BORSUPPLY"]
    RstcauseIdBorsupply = 4,
    #[doc = "5: BORWAKESHUTDN"]
    RstcauseIdBorwakeshutdn = 5,
    #[doc = "8: BOOTNONPMUPARITY"]
    RstcauseIdBootnonpmuparity = 8,
    #[doc = "9: BOOTCLKFAIL"]
    RstcauseIdBootclkfail = 9,
    #[doc = "10: BOOTSW"]
    RstcauseIdBootsw = 10,
    #[doc = "12: BOOTEXNRST"]
    RstcauseIdBootexnrst = 12,
    #[doc = "16: SYSBSLEXIT"]
    RstcauseIdSysbslexit = 16,
    #[doc = "17: SYSBSLENTRY"]
    RstcauseIdSysbslentry = 17,
    #[doc = "18: SYSWWDT0"]
    RstcauseIdSyswwdt0 = 18,
    #[doc = "19: SYSWWDT1"]
    RstcauseIdSyswwdt1 = 19,
    #[doc = "20: SYSFLASHECC"]
    RstcauseIdSysflashecc = 20,
    #[doc = "21: SYSCPULOCK"]
    RstcauseIdSyscpulock = 21,
    #[doc = "26: SYSDBG"]
    RstcauseIdSysdbg = 26,
    #[doc = "27: SYSSW"]
    RstcauseIdSyssw = 27,
    #[doc = "28: CPUDBG"]
    RstcauseIdCpudbg = 28,
    #[doc = "29: CPUSW"]
    RstcauseIdCpusw = 29,
}
impl From<RstcauseId> for u8 {
    #[inline(always)]
    fn from(variant: RstcauseId) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RstcauseId {
    type Ux = u8;
}
impl crate::IsEnum for RstcauseId {}
#[doc = "Field `RSTCAUSE_ID` reader - ID is a read-to-clear field which indicates the lowest level reset cause since the last read."]
pub type RstcauseIdR = crate::FieldReader<RstcauseId>;
impl RstcauseIdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<RstcauseId> {
        match self.bits {
            0 => Some(RstcauseId::RstcauseIdNorst),
            1 => Some(RstcauseId::RstcauseIdPorhwfail),
            2 => Some(RstcauseId::RstcauseIdPorexnrst),
            3 => Some(RstcauseId::RstcauseIdPorsw),
            4 => Some(RstcauseId::RstcauseIdBorsupply),
            5 => Some(RstcauseId::RstcauseIdBorwakeshutdn),
            8 => Some(RstcauseId::RstcauseIdBootnonpmuparity),
            9 => Some(RstcauseId::RstcauseIdBootclkfail),
            10 => Some(RstcauseId::RstcauseIdBootsw),
            12 => Some(RstcauseId::RstcauseIdBootexnrst),
            16 => Some(RstcauseId::RstcauseIdSysbslexit),
            17 => Some(RstcauseId::RstcauseIdSysbslentry),
            18 => Some(RstcauseId::RstcauseIdSyswwdt0),
            19 => Some(RstcauseId::RstcauseIdSyswwdt1),
            20 => Some(RstcauseId::RstcauseIdSysflashecc),
            21 => Some(RstcauseId::RstcauseIdSyscpulock),
            26 => Some(RstcauseId::RstcauseIdSysdbg),
            27 => Some(RstcauseId::RstcauseIdSyssw),
            28 => Some(RstcauseId::RstcauseIdCpudbg),
            29 => Some(RstcauseId::RstcauseIdCpusw),
            _ => None,
        }
    }
    #[doc = "NORST"]
    #[inline(always)]
    pub fn is_rstcause_id_norst(&self) -> bool {
        *self == RstcauseId::RstcauseIdNorst
    }
    #[doc = "PORHWFAIL"]
    #[inline(always)]
    pub fn is_rstcause_id_porhwfail(&self) -> bool {
        *self == RstcauseId::RstcauseIdPorhwfail
    }
    #[doc = "POREXNRST"]
    #[inline(always)]
    pub fn is_rstcause_id_porexnrst(&self) -> bool {
        *self == RstcauseId::RstcauseIdPorexnrst
    }
    #[doc = "PORSW"]
    #[inline(always)]
    pub fn is_rstcause_id_porsw(&self) -> bool {
        *self == RstcauseId::RstcauseIdPorsw
    }
    #[doc = "BORSUPPLY"]
    #[inline(always)]
    pub fn is_rstcause_id_borsupply(&self) -> bool {
        *self == RstcauseId::RstcauseIdBorsupply
    }
    #[doc = "BORWAKESHUTDN"]
    #[inline(always)]
    pub fn is_rstcause_id_borwakeshutdn(&self) -> bool {
        *self == RstcauseId::RstcauseIdBorwakeshutdn
    }
    #[doc = "BOOTNONPMUPARITY"]
    #[inline(always)]
    pub fn is_rstcause_id_bootnonpmuparity(&self) -> bool {
        *self == RstcauseId::RstcauseIdBootnonpmuparity
    }
    #[doc = "BOOTCLKFAIL"]
    #[inline(always)]
    pub fn is_rstcause_id_bootclkfail(&self) -> bool {
        *self == RstcauseId::RstcauseIdBootclkfail
    }
    #[doc = "BOOTSW"]
    #[inline(always)]
    pub fn is_rstcause_id_bootsw(&self) -> bool {
        *self == RstcauseId::RstcauseIdBootsw
    }
    #[doc = "BOOTEXNRST"]
    #[inline(always)]
    pub fn is_rstcause_id_bootexnrst(&self) -> bool {
        *self == RstcauseId::RstcauseIdBootexnrst
    }
    #[doc = "SYSBSLEXIT"]
    #[inline(always)]
    pub fn is_rstcause_id_sysbslexit(&self) -> bool {
        *self == RstcauseId::RstcauseIdSysbslexit
    }
    #[doc = "SYSBSLENTRY"]
    #[inline(always)]
    pub fn is_rstcause_id_sysbslentry(&self) -> bool {
        *self == RstcauseId::RstcauseIdSysbslentry
    }
    #[doc = "SYSWWDT0"]
    #[inline(always)]
    pub fn is_rstcause_id_syswwdt0(&self) -> bool {
        *self == RstcauseId::RstcauseIdSyswwdt0
    }
    #[doc = "SYSWWDT1"]
    #[inline(always)]
    pub fn is_rstcause_id_syswwdt1(&self) -> bool {
        *self == RstcauseId::RstcauseIdSyswwdt1
    }
    #[doc = "SYSFLASHECC"]
    #[inline(always)]
    pub fn is_rstcause_id_sysflashecc(&self) -> bool {
        *self == RstcauseId::RstcauseIdSysflashecc
    }
    #[doc = "SYSCPULOCK"]
    #[inline(always)]
    pub fn is_rstcause_id_syscpulock(&self) -> bool {
        *self == RstcauseId::RstcauseIdSyscpulock
    }
    #[doc = "SYSDBG"]
    #[inline(always)]
    pub fn is_rstcause_id_sysdbg(&self) -> bool {
        *self == RstcauseId::RstcauseIdSysdbg
    }
    #[doc = "SYSSW"]
    #[inline(always)]
    pub fn is_rstcause_id_syssw(&self) -> bool {
        *self == RstcauseId::RstcauseIdSyssw
    }
    #[doc = "CPUDBG"]
    #[inline(always)]
    pub fn is_rstcause_id_cpudbg(&self) -> bool {
        *self == RstcauseId::RstcauseIdCpudbg
    }
    #[doc = "CPUSW"]
    #[inline(always)]
    pub fn is_rstcause_id_cpusw(&self) -> bool {
        *self == RstcauseId::RstcauseIdCpusw
    }
}
impl R {
    #[doc = "Bits 0:4 - ID is a read-to-clear field which indicates the lowest level reset cause since the last read."]
    #[inline(always)]
    pub fn rstcause_id(&self) -> RstcauseIdR {
        RstcauseIdR::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Reset cause\n\nYou can [`read`](crate::Reg::read) this register and get [`rstcause::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstcauseSpec;
impl crate::RegisterSpec for RstcauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstcause::R`](R) reader structure"]
impl crate::Readable for RstcauseSpec {}
#[doc = "`reset()` method sets RSTCAUSE to value 0"]
impl crate::Resettable for RstcauseSpec {
    const RESET_VALUE: u32 = 0;
}
