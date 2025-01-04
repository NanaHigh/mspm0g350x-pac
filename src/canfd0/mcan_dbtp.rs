#[doc = "Register `MCAN_DBTP` reader"]
pub type R = crate::R<McanDbtpSpec>;
#[doc = "Register `MCAN_DBTP` writer"]
pub type W = crate::W<McanDbtpSpec>;
#[doc = "Field `MCAN_DBTP_DSJW` reader - Data Resynchronization Jump Width. Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanDbtpDsjwR = crate::FieldReader;
#[doc = "Field `MCAN_DBTP_DSJW` writer - Data Resynchronization Jump Width. Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanDbtpDsjwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCAN_DBTP_DTSEG2` reader - Data Time Segment After Sample Point. Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanDbtpDtseg2R = crate::FieldReader;
#[doc = "Field `MCAN_DBTP_DTSEG2` writer - Data Time Segment After Sample Point. Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanDbtpDtseg2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCAN_DBTP_DTSEG1` reader - Data Time Segment Before Sample Point. Valid values are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanDbtpDtseg1R = crate::FieldReader;
#[doc = "Field `MCAN_DBTP_DTSEG1` writer - Data Time Segment Before Sample Point. Valid values are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanDbtpDtseg1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MCAN_DBTP_DBRP` reader - Data Bit Rate Prescaler. The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanDbtpDbrpR = crate::FieldReader;
#[doc = "Field `MCAN_DBTP_DBRP` writer - Data Bit Rate Prescaler. The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanDbtpDbrpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `MCAN_DBTP_TDC` reader - Transmitter Delay Compensation 0 Transmitter Delay Compensation disabled 1 Transmitter Delay Compensation enabled +I107"]
pub type McanDbtpTdcR = crate::BitReader;
#[doc = "Field `MCAN_DBTP_TDC` writer - Transmitter Delay Compensation 0 Transmitter Delay Compensation disabled 1 Transmitter Delay Compensation enabled +I107"]
pub type McanDbtpTdcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Data Resynchronization Jump Width. Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_dbtp_dsjw(&self) -> McanDbtpDsjwR {
        McanDbtpDsjwR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Data Time Segment After Sample Point. Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_dbtp_dtseg2(&self) -> McanDbtpDtseg2R {
        McanDbtpDtseg2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Data Time Segment Before Sample Point. Valid values are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_dbtp_dtseg1(&self) -> McanDbtpDtseg1R {
        McanDbtpDtseg1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Data Bit Rate Prescaler. The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_dbtp_dbrp(&self) -> McanDbtpDbrpR {
        McanDbtpDbrpR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Transmitter Delay Compensation 0 Transmitter Delay Compensation disabled 1 Transmitter Delay Compensation enabled +I107"]
    #[inline(always)]
    pub fn mcan_dbtp_tdc(&self) -> McanDbtpTdcR {
        McanDbtpTdcR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Resynchronization Jump Width. Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_dbtp_dsjw(&mut self) -> McanDbtpDsjwW<McanDbtpSpec> {
        McanDbtpDsjwW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Data Time Segment After Sample Point. Valid values are 0 to 15. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_dbtp_dtseg2(&mut self) -> McanDbtpDtseg2W<McanDbtpSpec> {
        McanDbtpDtseg2W::new(self, 4)
    }
    #[doc = "Bits 8:12 - Data Time Segment Before Sample Point. Valid values are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_dbtp_dtseg1(&mut self) -> McanDbtpDtseg1W<McanDbtpSpec> {
        McanDbtpDtseg1W::new(self, 8)
    }
    #[doc = "Bits 16:20 - Data Bit Rate Prescaler. The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 31. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_dbtp_dbrp(&mut self) -> McanDbtpDbrpW<McanDbtpSpec> {
        McanDbtpDbrpW::new(self, 16)
    }
    #[doc = "Bit 23 - Transmitter Delay Compensation 0 Transmitter Delay Compensation disabled 1 Transmitter Delay Compensation enabled +I107"]
    #[inline(always)]
    pub fn mcan_dbtp_tdc(&mut self) -> McanDbtpTdcW<McanDbtpSpec> {
        McanDbtpTdcW::new(self, 23)
    }
}
#[doc = "MCAN Data Bit Timing and Prescaler Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_dbtp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_dbtp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanDbtpSpec;
impl crate::RegisterSpec for McanDbtpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_dbtp::R`](R) reader structure"]
impl crate::Readable for McanDbtpSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_dbtp::W`](W) writer structure"]
impl crate::Writable for McanDbtpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_DBTP to value 0x0a33"]
impl crate::Resettable for McanDbtpSpec {
    const RESET_VALUE: u32 = 0x0a33;
}
