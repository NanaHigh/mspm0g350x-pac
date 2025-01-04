#[doc = "Register `MCAN_RXF0C` reader"]
pub type R = crate::R<McanRxf0cSpec>;
#[doc = "Register `MCAN_RXF0C` writer"]
pub type W = crate::W<McanRxf0cSpec>;
#[doc = "Field `MCAN_RXF0C_F0SA` reader - Rx FIFO 0 Start Address. Start address of Rx FIFO 0 in Message RAM (32-bit word address). Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanRxf0cF0saR = crate::FieldReader<u16>;
#[doc = "Field `MCAN_RXF0C_F0SA` writer - Rx FIFO 0 Start Address. Start address of Rx FIFO 0 in Message RAM (32-bit word address). Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanRxf0cF0saW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `MCAN_RXF0C_F0S` reader - Rx FIFO 0 Size. The Rx FIFO 0 elements are indexed from 0 to F0S-1. 0 No Rx FIFO 0 1-64 Number of Rx FIFO 0 elements &gt;64 Values greater than 64 are interpreted as 64 Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanRxf0cF0sR = crate::FieldReader;
#[doc = "Field `MCAN_RXF0C_F0S` writer - Rx FIFO 0 Size. The Rx FIFO 0 elements are indexed from 0 to F0S-1. 0 No Rx FIFO 0 1-64 Number of Rx FIFO 0 elements &gt;64 Values greater than 64 are interpreted as 64 Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanRxf0cF0sW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MCAN_RXF0C_F0WM` reader - Rx FIFO 0 Watermark 0 Watermark interrupt disabled 1-64 Level for Rx FIFO 0 watermark interrupt (IR.RF0W) &gt;64 Watermark interrupt disabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanRxf0cF0wmR = crate::FieldReader;
#[doc = "Field `MCAN_RXF0C_F0WM` writer - Rx FIFO 0 Watermark 0 Watermark interrupt disabled 1-64 Level for Rx FIFO 0 watermark interrupt (IR.RF0W) &gt;64 Watermark interrupt disabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanRxf0cF0wmW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MCAN_RXF0C_F0OM` reader - FIFO 0 Operation Mode. FIFO 0 can be operated in blocking or in overwrite mode. 0 FIFO 0 blocking mode 1 FIFO 0 overwrite mode Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanRxf0cF0omR = crate::BitReader;
#[doc = "Field `MCAN_RXF0C_F0OM` writer - FIFO 0 Operation Mode. FIFO 0 can be operated in blocking or in overwrite mode. 0 FIFO 0 blocking mode 1 FIFO 0 overwrite mode Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanRxf0cF0omW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:15 - Rx FIFO 0 Start Address. Start address of Rx FIFO 0 in Message RAM (32-bit word address). Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_rxf0c_f0sa(&self) -> McanRxf0cF0saR {
        McanRxf0cF0saR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - Rx FIFO 0 Size. The Rx FIFO 0 elements are indexed from 0 to F0S-1. 0 No Rx FIFO 0 1-64 Number of Rx FIFO 0 elements &gt;64 Values greater than 64 are interpreted as 64 Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_rxf0c_f0s(&self) -> McanRxf0cF0sR {
        McanRxf0cF0sR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Rx FIFO 0 Watermark 0 Watermark interrupt disabled 1-64 Level for Rx FIFO 0 watermark interrupt (IR.RF0W) &gt;64 Watermark interrupt disabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_rxf0c_f0wm(&self) -> McanRxf0cF0wmR {
        McanRxf0cF0wmR::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - FIFO 0 Operation Mode. FIFO 0 can be operated in blocking or in overwrite mode. 0 FIFO 0 blocking mode 1 FIFO 0 overwrite mode Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_rxf0c_f0om(&self) -> McanRxf0cF0omR {
        McanRxf0cF0omR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:15 - Rx FIFO 0 Start Address. Start address of Rx FIFO 0 in Message RAM (32-bit word address). Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_rxf0c_f0sa(&mut self) -> McanRxf0cF0saW<McanRxf0cSpec> {
        McanRxf0cF0saW::new(self, 2)
    }
    #[doc = "Bits 16:22 - Rx FIFO 0 Size. The Rx FIFO 0 elements are indexed from 0 to F0S-1. 0 No Rx FIFO 0 1-64 Number of Rx FIFO 0 elements &gt;64 Values greater than 64 are interpreted as 64 Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_rxf0c_f0s(&mut self) -> McanRxf0cF0sW<McanRxf0cSpec> {
        McanRxf0cF0sW::new(self, 16)
    }
    #[doc = "Bits 24:30 - Rx FIFO 0 Watermark 0 Watermark interrupt disabled 1-64 Level for Rx FIFO 0 watermark interrupt (IR.RF0W) &gt;64 Watermark interrupt disabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_rxf0c_f0wm(&mut self) -> McanRxf0cF0wmW<McanRxf0cSpec> {
        McanRxf0cF0wmW::new(self, 24)
    }
    #[doc = "Bit 31 - FIFO 0 Operation Mode. FIFO 0 can be operated in blocking or in overwrite mode. 0 FIFO 0 blocking mode 1 FIFO 0 overwrite mode Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_rxf0c_f0om(&mut self) -> McanRxf0cF0omW<McanRxf0cSpec> {
        McanRxf0cF0omW::new(self, 31)
    }
}
#[doc = "MCAN Rx FIFO 0 Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_rxf0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_rxf0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanRxf0cSpec;
impl crate::RegisterSpec for McanRxf0cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_rxf0c::R`](R) reader structure"]
impl crate::Readable for McanRxf0cSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_rxf0c::W`](W) writer structure"]
impl crate::Writable for McanRxf0cSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_RXF0C to value 0"]
impl crate::Resettable for McanRxf0cSpec {
    const RESET_VALUE: u32 = 0;
}
