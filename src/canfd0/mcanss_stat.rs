#[doc = "Register `MCANSS_STAT` reader"]
pub type R = crate::R<McanssStatSpec>;
#[doc = "Field `MCANSS_STAT_RESET` reader - Soft Reset Status. 0 Not in reset 1 Reset is in progress"]
pub type McanssStatResetR = crate::BitReader;
#[doc = "Field `MCANSS_STAT_MEM_INIT_DONE` reader - Memory Initialization Done. 0 Message RAM initialization is in progress 1 Message RAM is initialized for use"]
pub type McanssStatMemInitDoneR = crate::BitReader;
#[doc = "Field `MCANSS_STAT_ENABLE_FDOE` reader - Flexible Datarate Operation Enable. Determines whether CAN FD operation may be enabled via the MCAN core CCCR.FDOE bit (bit 8) or if only standard CAN operation is possible with this instance of the MCAN. 0 MCAN is only capable of standard CAN communication 1 MCAN may be configured to perform CAN FD communication"]
pub type McanssStatEnableFdoeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Soft Reset Status. 0 Not in reset 1 Reset is in progress"]
    #[inline(always)]
    pub fn mcanss_stat_reset(&self) -> McanssStatResetR {
        McanssStatResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Memory Initialization Done. 0 Message RAM initialization is in progress 1 Message RAM is initialized for use"]
    #[inline(always)]
    pub fn mcanss_stat_mem_init_done(&self) -> McanssStatMemInitDoneR {
        McanssStatMemInitDoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flexible Datarate Operation Enable. Determines whether CAN FD operation may be enabled via the MCAN core CCCR.FDOE bit (bit 8) or if only standard CAN operation is possible with this instance of the MCAN. 0 MCAN is only capable of standard CAN communication 1 MCAN may be configured to perform CAN FD communication"]
    #[inline(always)]
    pub fn mcanss_stat_enable_fdoe(&self) -> McanssStatEnableFdoeR {
        McanssStatEnableFdoeR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "MCAN Subsystem Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcanss_stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanssStatSpec;
impl crate::RegisterSpec for McanssStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcanss_stat::R`](R) reader structure"]
impl crate::Readable for McanssStatSpec {}
#[doc = "`reset()` method sets MCANSS_STAT to value 0"]
impl crate::Resettable for McanssStatSpec {
    const RESET_VALUE: u32 = 0;
}
