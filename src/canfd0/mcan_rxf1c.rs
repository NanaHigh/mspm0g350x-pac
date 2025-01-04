#[doc = "Register `MCAN_RXF1C` reader"]
pub type R = crate::R<McanRxf1cSpec>;
#[doc = "Register `MCAN_RXF1C` writer"]
pub type W = crate::W<McanRxf1cSpec>;
#[doc = "Field `MCAN_RXF1C_F1SA` reader - Rx FIFO 1 Start Address Start address of Rx FIFO 1 in Message RAM (32-bit word address)."]
pub type McanRxf1cF1saR = crate::FieldReader<u16>;
#[doc = "Field `MCAN_RXF1C_F1SA` writer - Rx FIFO 1 Start Address Start address of Rx FIFO 1 in Message RAM (32-bit word address)."]
pub type McanRxf1cF1saW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `MCAN_RXF1C_F1S` reader - Rx FIFO 1 Size. The Rx FIFO 1 elements are indexed from 0 to F1S - 1. 0 No Rx FIFO 1 1-64 Number of Rx FIFO 1 elements &gt;64 Values greater than 64 are interpreted as 64 Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanRxf1cF1sR = crate::FieldReader;
#[doc = "Field `MCAN_RXF1C_F1S` writer - Rx FIFO 1 Size. The Rx FIFO 1 elements are indexed from 0 to F1S - 1. 0 No Rx FIFO 1 1-64 Number of Rx FIFO 1 elements &gt;64 Values greater than 64 are interpreted as 64 Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanRxf1cF1sW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MCAN_RXF1C_F1WM` reader - Rx FIFO 1 Watermark 0 Watermark interrupt disabled 1-64 Level for Rx FIFO 1 watermark interrupt (IR.RF1W) &gt;64 Watermark interrupt disabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanRxf1cF1wmR = crate::FieldReader;
#[doc = "Field `MCAN_RXF1C_F1WM` writer - Rx FIFO 1 Watermark 0 Watermark interrupt disabled 1-64 Level for Rx FIFO 1 watermark interrupt (IR.RF1W) &gt;64 Watermark interrupt disabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanRxf1cF1wmW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MCAN_RXF1C_F1OM` reader - FIFO 1 Operation Mode. FIFO 1 can be operated in blocking or in overwrite mode. 0 FIFO 1 blocking mode 1 FIFO 1 overwrite mode Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanRxf1cF1omR = crate::BitReader;
#[doc = "Field `MCAN_RXF1C_F1OM` writer - FIFO 1 Operation Mode. FIFO 1 can be operated in blocking or in overwrite mode. 0 FIFO 1 blocking mode 1 FIFO 1 overwrite mode Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanRxf1cF1omW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:15 - Rx FIFO 1 Start Address Start address of Rx FIFO 1 in Message RAM (32-bit word address)."]
    #[inline(always)]
    pub fn mcan_rxf1c_f1sa(&self) -> McanRxf1cF1saR {
        McanRxf1cF1saR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - Rx FIFO 1 Size. The Rx FIFO 1 elements are indexed from 0 to F1S - 1. 0 No Rx FIFO 1 1-64 Number of Rx FIFO 1 elements &gt;64 Values greater than 64 are interpreted as 64 Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_rxf1c_f1s(&self) -> McanRxf1cF1sR {
        McanRxf1cF1sR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Rx FIFO 1 Watermark 0 Watermark interrupt disabled 1-64 Level for Rx FIFO 1 watermark interrupt (IR.RF1W) &gt;64 Watermark interrupt disabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_rxf1c_f1wm(&self) -> McanRxf1cF1wmR {
        McanRxf1cF1wmR::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - FIFO 1 Operation Mode. FIFO 1 can be operated in blocking or in overwrite mode. 0 FIFO 1 blocking mode 1 FIFO 1 overwrite mode Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_rxf1c_f1om(&self) -> McanRxf1cF1omR {
        McanRxf1cF1omR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - Rx FIFO 1 Start Address Start address of Rx FIFO 1 in Message RAM (32-bit word address)."]
    #[inline(always)]
    pub fn mcan_rxf1c_f1sa(&mut self) -> McanRxf1cF1saW<McanRxf1cSpec> {
        McanRxf1cF1saW::new(self, 2)
    }
    #[doc = "Bits 16:22 - Rx FIFO 1 Size. The Rx FIFO 1 elements are indexed from 0 to F1S - 1. 0 No Rx FIFO 1 1-64 Number of Rx FIFO 1 elements &gt;64 Values greater than 64 are interpreted as 64 Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_rxf1c_f1s(&mut self) -> McanRxf1cF1sW<McanRxf1cSpec> {
        McanRxf1cF1sW::new(self, 16)
    }
    #[doc = "Bits 24:30 - Rx FIFO 1 Watermark 0 Watermark interrupt disabled 1-64 Level for Rx FIFO 1 watermark interrupt (IR.RF1W) &gt;64 Watermark interrupt disabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_rxf1c_f1wm(&mut self) -> McanRxf1cF1wmW<McanRxf1cSpec> {
        McanRxf1cF1wmW::new(self, 24)
    }
    #[doc = "Bit 31 - FIFO 1 Operation Mode. FIFO 1 can be operated in blocking or in overwrite mode. 0 FIFO 1 blocking mode 1 FIFO 1 overwrite mode Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_rxf1c_f1om(&mut self) -> McanRxf1cF1omW<McanRxf1cSpec> {
        McanRxf1cF1omW::new(self, 31)
    }
}
#[doc = "MCAN Rx FIFO 1 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_rxf1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_rxf1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanRxf1cSpec;
impl crate::RegisterSpec for McanRxf1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_rxf1c::R`](R) reader structure"]
impl crate::Readable for McanRxf1cSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_rxf1c::W`](W) writer structure"]
impl crate::Writable for McanRxf1cSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_RXF1C to value 0"]
impl crate::Resettable for McanRxf1cSpec {
    const RESET_VALUE: u32 = 0;
}
