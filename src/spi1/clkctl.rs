#[doc = "Register `CLKCTL` reader"]
pub type R = crate::R<ClkctlSpec>;
#[doc = "Register `CLKCTL` writer"]
pub type W = crate::W<ClkctlSpec>;
#[doc = "Field `CLKCTL_SCR` reader - Serial clock divider: This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate is (SPI's functional clock frequency)/((SCR+1)*2). SCR is a value from 0-1023."]
pub type ClkctlScrR = crate::FieldReader<u16>;
#[doc = "Field `CLKCTL_SCR` writer - Serial clock divider: This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate is (SPI's functional clock frequency)/((SCR+1)*2). SCR is a value from 0-1023."]
pub type ClkctlScrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CLKCTL_DSAMPLE` reader - Delayed sampling value. In controller mode the data on the input pin will be delayed sampled by the defined clock cycles of internal functional clock hence relaxing the setup time of input data. This setting is useful in systems where the board delays and external peripheral delays are more than the input setup time of the controller. Please refer to the datasheet for values of controller input setup time and assess what DSAMPLE value meets the requirement of the system. Note: High values of DSAMPLE can cause HOLD time violations and must be factored in the calculations."]
pub type ClkctlDsampleR = crate::FieldReader;
#[doc = "Field `CLKCTL_DSAMPLE` writer - Delayed sampling value. In controller mode the data on the input pin will be delayed sampled by the defined clock cycles of internal functional clock hence relaxing the setup time of input data. This setting is useful in systems where the board delays and external peripheral delays are more than the input setup time of the controller. Please refer to the datasheet for values of controller input setup time and assess what DSAMPLE value meets the requirement of the system. Note: High values of DSAMPLE can cause HOLD time violations and must be factored in the calculations."]
pub type ClkctlDsampleW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:9 - Serial clock divider: This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate is (SPI's functional clock frequency)/((SCR+1)*2). SCR is a value from 0-1023."]
    #[inline(always)]
    pub fn clkctl_scr(&self) -> ClkctlScrR {
        ClkctlScrR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 28:31 - Delayed sampling value. In controller mode the data on the input pin will be delayed sampled by the defined clock cycles of internal functional clock hence relaxing the setup time of input data. This setting is useful in systems where the board delays and external peripheral delays are more than the input setup time of the controller. Please refer to the datasheet for values of controller input setup time and assess what DSAMPLE value meets the requirement of the system. Note: High values of DSAMPLE can cause HOLD time violations and must be factored in the calculations."]
    #[inline(always)]
    pub fn clkctl_dsample(&self) -> ClkctlDsampleR {
        ClkctlDsampleR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - Serial clock divider: This is used to generate the transmit and receive bit rate of the SPI. The SPI bit rate is (SPI's functional clock frequency)/((SCR+1)*2). SCR is a value from 0-1023."]
    #[inline(always)]
    pub fn clkctl_scr(&mut self) -> ClkctlScrW<ClkctlSpec> {
        ClkctlScrW::new(self, 0)
    }
    #[doc = "Bits 28:31 - Delayed sampling value. In controller mode the data on the input pin will be delayed sampled by the defined clock cycles of internal functional clock hence relaxing the setup time of input data. This setting is useful in systems where the board delays and external peripheral delays are more than the input setup time of the controller. Please refer to the datasheet for values of controller input setup time and assess what DSAMPLE value meets the requirement of the system. Note: High values of DSAMPLE can cause HOLD time violations and must be factored in the calculations."]
    #[inline(always)]
    pub fn clkctl_dsample(&mut self) -> ClkctlDsampleW<ClkctlSpec> {
        ClkctlDsampleW::new(self, 28)
    }
}
#[doc = "Clock prescaler and divider register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clkctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkctlSpec;
impl crate::RegisterSpec for ClkctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkctl::R`](R) reader structure"]
impl crate::Readable for ClkctlSpec {}
#[doc = "`write(|w| ..)` method takes [`clkctl::W`](W) writer structure"]
impl crate::Writable for ClkctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKCTL to value 0"]
impl crate::Resettable for ClkctlSpec {
    const RESET_VALUE: u32 = 0;
}
