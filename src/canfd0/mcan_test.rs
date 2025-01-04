#[doc = "Register `MCAN_TEST` reader"]
pub type R = crate::R<McanTestSpec>;
#[doc = "Register `MCAN_TEST` writer"]
pub type W = crate::W<McanTestSpec>;
#[doc = "Field `MCAN_TEST_LBCK` reader - Loop Back Mode 0 Reset value, Loop Back Mode is disabled 1 Loop Back Mode is enabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTestLbckR = crate::BitReader;
#[doc = "Field `MCAN_TEST_LBCK` writer - Loop Back Mode 0 Reset value, Loop Back Mode is disabled 1 Loop Back Mode is enabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTestLbckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCAN_TEST_TX` reader - Control of Transmit Pin 00 CAN TX pin controlled by the CAN Core, updated at the end of the CAN bit time 01 Sample Point can be monitored at CAN TX pin 10 Dominant ('0') level at CAN TX pin 11 Recessive ('1') at CAN TX pin Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTestTxR = crate::FieldReader;
#[doc = "Field `MCAN_TEST_TX` writer - Control of Transmit Pin 00 CAN TX pin controlled by the CAN Core, updated at the end of the CAN bit time 01 Sample Point can be monitored at CAN TX pin 10 Dominant ('0') level at CAN TX pin 11 Recessive ('1') at CAN TX pin Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
pub type McanTestTxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MCAN_TEST_RX` reader - Receive Pin. Monitors the actual value of the CAN receive pin. 0 The CAN bus is dominant (CAN RX pin = '0') 1 The CAN bus is recessive (CAN RX pin = '1')"]
pub type McanTestRxR = crate::BitReader;
impl R {
    #[doc = "Bit 4 - Loop Back Mode 0 Reset value, Loop Back Mode is disabled 1 Loop Back Mode is enabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_test_lbck(&self) -> McanTestLbckR {
        McanTestLbckR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin 00 CAN TX pin controlled by the CAN Core, updated at the end of the CAN bit time 01 Sample Point can be monitored at CAN TX pin 10 Dominant ('0') level at CAN TX pin 11 Recessive ('1') at CAN TX pin Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_test_tx(&self) -> McanTestTxR {
        McanTestTxR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Receive Pin. Monitors the actual value of the CAN receive pin. 0 The CAN bus is dominant (CAN RX pin = '0') 1 The CAN bus is recessive (CAN RX pin = '1')"]
    #[inline(always)]
    pub fn mcan_test_rx(&self) -> McanTestRxR {
        McanTestRxR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Loop Back Mode 0 Reset value, Loop Back Mode is disabled 1 Loop Back Mode is enabled Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_test_lbck(&mut self) -> McanTestLbckW<McanTestSpec> {
        McanTestLbckW::new(self, 4)
    }
    #[doc = "Bits 5:6 - Control of Transmit Pin 00 CAN TX pin controlled by the CAN Core, updated at the end of the CAN bit time 01 Sample Point can be monitored at CAN TX pin 10 Dominant ('0') level at CAN TX pin 11 Recessive ('1') at CAN TX pin Qualified Write is possible only with CCCR.CCE='1' and CCCR.INIT='1'."]
    #[inline(always)]
    pub fn mcan_test_tx(&mut self) -> McanTestTxW<McanTestSpec> {
        McanTestTxW::new(self, 5)
    }
}
#[doc = "MCAN Test Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcan_test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanTestSpec;
impl crate::RegisterSpec for McanTestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_test::R`](R) reader structure"]
impl crate::Readable for McanTestSpec {}
#[doc = "`write(|w| ..)` method takes [`mcan_test::W`](W) writer structure"]
impl crate::Writable for McanTestSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCAN_TEST to value 0"]
impl crate::Resettable for McanTestSpec {
    const RESET_VALUE: u32 = 0;
}
