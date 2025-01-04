#[doc = "Register `CTL2` reader"]
pub type R = crate::R<Ctl2Spec>;
#[doc = "Register `CTL2` writer"]
pub type W = crate::W<Ctl2Spec>;
#[doc = "Field `CTL2_SHCYCLE` reader - Sample and Hold cycle count Total cycles of module clock for sample and hold phase when VREF is working in sample and hold mode in STANDBY to save power. This field should be greater than HCYCLE field. The difference between this field and HCYCLE gives the number of cycles of sample phase. Please refer VREF section of datasheet for recommended values of sample and hold times."]
pub type Ctl2ShcycleR = crate::FieldReader<u16>;
#[doc = "Field `CTL2_SHCYCLE` writer - Sample and Hold cycle count Total cycles of module clock for sample and hold phase when VREF is working in sample and hold mode in STANDBY to save power. This field should be greater than HCYCLE field. The difference between this field and HCYCLE gives the number of cycles of sample phase. Please refer VREF section of datasheet for recommended values of sample and hold times."]
pub type Ctl2ShcycleW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CTL2_HCYCLE` reader - Hold cycle count Total cycles of module clock for hold phase when VREF is working in sample and hold mode in STANDBY to save power. Please refer VREF section of datasheet for recommended values of sample and hold times."]
pub type Ctl2HcycleR = crate::FieldReader<u16>;
#[doc = "Field `CTL2_HCYCLE` writer - Hold cycle count Total cycles of module clock for hold phase when VREF is working in sample and hold mode in STANDBY to save power. Please refer VREF section of datasheet for recommended values of sample and hold times."]
pub type Ctl2HcycleW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Sample and Hold cycle count Total cycles of module clock for sample and hold phase when VREF is working in sample and hold mode in STANDBY to save power. This field should be greater than HCYCLE field. The difference between this field and HCYCLE gives the number of cycles of sample phase. Please refer VREF section of datasheet for recommended values of sample and hold times."]
    #[inline(always)]
    pub fn ctl2_shcycle(&self) -> Ctl2ShcycleR {
        Ctl2ShcycleR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Hold cycle count Total cycles of module clock for hold phase when VREF is working in sample and hold mode in STANDBY to save power. Please refer VREF section of datasheet for recommended values of sample and hold times."]
    #[inline(always)]
    pub fn ctl2_hcycle(&self) -> Ctl2HcycleR {
        Ctl2HcycleR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sample and Hold cycle count Total cycles of module clock for sample and hold phase when VREF is working in sample and hold mode in STANDBY to save power. This field should be greater than HCYCLE field. The difference between this field and HCYCLE gives the number of cycles of sample phase. Please refer VREF section of datasheet for recommended values of sample and hold times."]
    #[inline(always)]
    pub fn ctl2_shcycle(&mut self) -> Ctl2ShcycleW<Ctl2Spec> {
        Ctl2ShcycleW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Hold cycle count Total cycles of module clock for hold phase when VREF is working in sample and hold mode in STANDBY to save power. Please refer VREF section of datasheet for recommended values of sample and hold times."]
    #[inline(always)]
    pub fn ctl2_hcycle(&mut self) -> Ctl2HcycleW<Ctl2Spec> {
        Ctl2HcycleW::new(self, 16)
    }
}
#[doc = "Control 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl2Spec;
impl crate::RegisterSpec for Ctl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl2::R`](R) reader structure"]
impl crate::Readable for Ctl2Spec {}
#[doc = "`write(|w| ..)` method takes [`ctl2::W`](W) writer structure"]
impl crate::Writable for Ctl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL2 to value 0"]
impl crate::Resettable for Ctl2Spec {
    const RESET_VALUE: u32 = 0;
}
