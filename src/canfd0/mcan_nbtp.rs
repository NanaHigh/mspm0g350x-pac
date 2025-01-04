#[doc = "Register `MCAN_NBTP` reader"]
pub type R = crate::R<McanNbtpSpec>;
#[doc = "Register `MCAN_NBTP` writer"]
pub type W = crate::W<McanNbtpSpec>;
#[doc = "Field `MCAN_NBTP_NTSEG2` reader - Nominal Time Segment After Sample Point. Valid values are 1 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanNbtpNtseg2R = crate::FieldReader;
#[doc = "Field `MCAN_NBTP_NTSEG2` writer - Nominal Time Segment After Sample Point. Valid values are 1 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanNbtpNtseg2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MCAN_NBTP_NTSEG1` reader - Nominal Time Segment Before Sample Point. Valid values are 1 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanNbtpNtseg1R = crate::FieldReader;
#[doc = "Field `MCAN_NBTP_NTSEG1` writer - Nominal Time Segment Before Sample Point. Valid values are 1 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanNbtpNtseg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MCAN_NBTP_NBRP` reader - Nominal Bit Rate Prescaler. The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanNbtpNbrpR = crate::FieldReader<u16>;
#[doc = "Field `MCAN_NBTP_NBRP` writer - Nominal Bit Rate Prescaler. The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanNbtpNbrpW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `MCAN_NBTP_NSJW` reader - Nominal (Re)Synchronization Jump Width. Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanNbtpNsjwR = crate::FieldReader;
#[doc = "Field `MCAN_NBTP_NSJW` writer - Nominal (Re)Synchronization Jump Width. Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanNbtpNsjwW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Nominal Time Segment After Sample Point. Valid values are 1 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_nbtp_ntseg2(&self) -> McanNbtpNtseg2R {
        McanNbtpNtseg2R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - Nominal Time Segment Before Sample Point. Valid values are 1 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_nbtp_ntseg1(&self) -> McanNbtpNtseg1R {
        McanNbtpNtseg1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:24 - Nominal Bit Rate Prescaler. The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_nbtp_nbrp(&self) -> McanNbtpNbrpR {
        McanNbtpNbrpR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bits 25:31 - Nominal (Re)Synchronization Jump Width. Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_nbtp_nsjw(&self) -> McanNbtpNsjwR {
        McanNbtpNsjwR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Nominal Time Segment After Sample Point. Valid values are 1 to 127. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_nbtp_ntseg2(&mut self) -> McanNbtpNtseg2W<McanNbtpSpec> {
        McanNbtpNtseg2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Nominal Time Segment Before Sample Point. Valid values are 1 to 255. The actual interpretation by the hardware of this value is such that one more than the programmed value is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_nbtp_ntseg1(&mut self) -> McanNbtpNtseg1W<McanNbtpSpec> {
        McanNbtpNtseg1W::new(self, 8)
    }
    #[doc = "Bits 16:24 - Nominal Bit Rate Prescaler. The value by which the oscillator frequency is divided for generating the bit time quanta. The bit time is built up from a multiple of this quanta. Valid values for the Bit Rate Prescaler are 0 to 511. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_nbtp_nbrp(&mut self) -> McanNbtpNbrpW<McanNbtpSpec> {
        McanNbtpNbrpW::new(self, 16)
    }
    #[doc = "Bits 25:31 - Nominal (Re)Synchronization Jump Width. Valid values are 0 to 127. The actual interpretation by the hardware of this value is such that one more than the value programmed here is used. Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_nbtp_nsjw(&mut self) -> McanNbtpNsjwW<McanNbtpSpec> {
        McanNbtpNsjwW::new(self, 25)
    }
}
#[doc = "MCAN Nominal Bit Timing and Prescaler Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_nbtp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_nbtp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanNbtpSpec;
impl crate::RegisterSpec for McanNbtpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_nbtp::R`](R) reader structure"]
impl crate::Readable for McanNbtpSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_nbtp::W`](W) writer structure"]
impl crate::Writable for McanNbtpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_NBTP to value 0x0600_0a03"]
impl crate::Resettable for McanNbtpSpec {
    const RESET_VALUE: u32 = 0x0600_0a03;
}
