#[doc = "Register `TIMEOUT_CTL` reader"]
pub type R = crate::R<TimeoutCtlSpec>;
#[doc = "Register `TIMEOUT_CTL` writer"]
pub type W = crate::W<TimeoutCtlSpec>;
#[doc = "Field `TIMEOUT_CTL_TCNTLA` reader - Timeout counter A load value Counter A is used for SCL low detection. This field contains the upper 8 bits of a 12-bit pre-load value for the Timeout A count. NOTE: The value of CNTLA must be greater than 1h. Each count is equal to 520 times the timeout period of functional clock. For example, with 8MHz functional clock and a 100KHz operating I2C clock, one timeout period will be equal to (1 / 8MHz) * 520 or 65 us."]
pub type TimeoutCtlTcntlaR = crate::FieldReader;
#[doc = "Field `TIMEOUT_CTL_TCNTLA` writer - Timeout counter A load value Counter A is used for SCL low detection. This field contains the upper 8 bits of a 12-bit pre-load value for the Timeout A count. NOTE: The value of CNTLA must be greater than 1h. Each count is equal to 520 times the timeout period of functional clock. For example, with 8MHz functional clock and a 100KHz operating I2C clock, one timeout period will be equal to (1 / 8MHz) * 520 or 65 us."]
pub type TimeoutCtlTcntlaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Timeout Counter A Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeoutCtlTcntaen {
    #[doc = "0: DISABLE"]
    TimeoutCtlTcntaenDisable = 0,
    #[doc = "1: ENABLE"]
    TimeoutCtlTcntaenEnable = 1,
}
impl From<TimeoutCtlTcntaen> for bool {
    #[inline(always)]
    fn from(variant: TimeoutCtlTcntaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT_CTL_TCNTAEN` reader - Timeout Counter A Enable"]
pub type TimeoutCtlTcntaenR = crate::BitReader<TimeoutCtlTcntaen>;
impl TimeoutCtlTcntaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimeoutCtlTcntaen {
        match self.bits {
            false => TimeoutCtlTcntaen::TimeoutCtlTcntaenDisable,
            true => TimeoutCtlTcntaen::TimeoutCtlTcntaenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_timeout_ctl_tcntaen_disable(&self) -> bool {
        *self == TimeoutCtlTcntaen::TimeoutCtlTcntaenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_timeout_ctl_tcntaen_enable(&self) -> bool {
        *self == TimeoutCtlTcntaen::TimeoutCtlTcntaenEnable
    }
}
#[doc = "Field `TIMEOUT_CTL_TCNTAEN` writer - Timeout Counter A Enable"]
pub type TimeoutCtlTcntaenW<'a, REG> = crate::BitWriter<'a, REG, TimeoutCtlTcntaen>;
impl<'a, REG> TimeoutCtlTcntaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn timeout_ctl_tcntaen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutCtlTcntaen::TimeoutCtlTcntaenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn timeout_ctl_tcntaen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutCtlTcntaen::TimeoutCtlTcntaenEnable)
    }
}
#[doc = "Field `TIMEOUT_CTL_TCNTLB` reader - Timeout Count B Load: Counter B is used for SCL High Detection. This field contains the upper 8 bits of a 12-bit pre-load value for the Timeout B count. NOTE: The value of CNTLB must be greater than 1h. Each count is equal to 1* clock period. For example, with 10MHz functional clock one timeout period will be equal to1*100ns."]
pub type TimeoutCtlTcntlbR = crate::FieldReader;
#[doc = "Field `TIMEOUT_CTL_TCNTLB` writer - Timeout Count B Load: Counter B is used for SCL High Detection. This field contains the upper 8 bits of a 12-bit pre-load value for the Timeout B count. NOTE: The value of CNTLB must be greater than 1h. Each count is equal to 1* clock period. For example, with 10MHz functional clock one timeout period will be equal to1*100ns."]
pub type TimeoutCtlTcntlbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Timeout Counter B Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeoutCtlTcntben {
    #[doc = "0: DISABLE"]
    TimeoutCtlTcntbenDisable = 0,
    #[doc = "1: ENABLE"]
    TimeoutCtlTcntbenEnable = 1,
}
impl From<TimeoutCtlTcntben> for bool {
    #[inline(always)]
    fn from(variant: TimeoutCtlTcntben) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT_CTL_TCNTBEN` reader - Timeout Counter B Enable"]
pub type TimeoutCtlTcntbenR = crate::BitReader<TimeoutCtlTcntben>;
impl TimeoutCtlTcntbenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimeoutCtlTcntben {
        match self.bits {
            false => TimeoutCtlTcntben::TimeoutCtlTcntbenDisable,
            true => TimeoutCtlTcntben::TimeoutCtlTcntbenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_timeout_ctl_tcntben_disable(&self) -> bool {
        *self == TimeoutCtlTcntben::TimeoutCtlTcntbenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_timeout_ctl_tcntben_enable(&self) -> bool {
        *self == TimeoutCtlTcntben::TimeoutCtlTcntbenEnable
    }
}
#[doc = "Field `TIMEOUT_CTL_TCNTBEN` writer - Timeout Counter B Enable"]
pub type TimeoutCtlTcntbenW<'a, REG> = crate::BitWriter<'a, REG, TimeoutCtlTcntben>;
impl<'a, REG> TimeoutCtlTcntbenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn timeout_ctl_tcntben_disable(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutCtlTcntben::TimeoutCtlTcntbenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn timeout_ctl_tcntben_enable(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutCtlTcntben::TimeoutCtlTcntbenEnable)
    }
}
impl R {
    #[doc = "Bits 0:7 - Timeout counter A load value Counter A is used for SCL low detection. This field contains the upper 8 bits of a 12-bit pre-load value for the Timeout A count. NOTE: The value of CNTLA must be greater than 1h. Each count is equal to 520 times the timeout period of functional clock. For example, with 8MHz functional clock and a 100KHz operating I2C clock, one timeout period will be equal to (1 / 8MHz) * 520 or 65 us."]
    #[inline(always)]
    pub fn timeout_ctl_tcntla(&self) -> TimeoutCtlTcntlaR {
        TimeoutCtlTcntlaR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - Timeout Counter A Enable"]
    #[inline(always)]
    pub fn timeout_ctl_tcntaen(&self) -> TimeoutCtlTcntaenR {
        TimeoutCtlTcntaenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Timeout Count B Load: Counter B is used for SCL High Detection. This field contains the upper 8 bits of a 12-bit pre-load value for the Timeout B count. NOTE: The value of CNTLB must be greater than 1h. Each count is equal to 1* clock period. For example, with 10MHz functional clock one timeout period will be equal to1*100ns."]
    #[inline(always)]
    pub fn timeout_ctl_tcntlb(&self) -> TimeoutCtlTcntlbR {
        TimeoutCtlTcntlbR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Timeout Counter B Enable"]
    #[inline(always)]
    pub fn timeout_ctl_tcntben(&self) -> TimeoutCtlTcntbenR {
        TimeoutCtlTcntbenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timeout counter A load value Counter A is used for SCL low detection. This field contains the upper 8 bits of a 12-bit pre-load value for the Timeout A count. NOTE: The value of CNTLA must be greater than 1h. Each count is equal to 520 times the timeout period of functional clock. For example, with 8MHz functional clock and a 100KHz operating I2C clock, one timeout period will be equal to (1 / 8MHz) * 520 or 65 us."]
    #[inline(always)]
    pub fn timeout_ctl_tcntla(&mut self) -> TimeoutCtlTcntlaW<TimeoutCtlSpec> {
        TimeoutCtlTcntlaW::new(self, 0)
    }
    #[doc = "Bit 15 - Timeout Counter A Enable"]
    #[inline(always)]
    pub fn timeout_ctl_tcntaen(&mut self) -> TimeoutCtlTcntaenW<TimeoutCtlSpec> {
        TimeoutCtlTcntaenW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Timeout Count B Load: Counter B is used for SCL High Detection. This field contains the upper 8 bits of a 12-bit pre-load value for the Timeout B count. NOTE: The value of CNTLB must be greater than 1h. Each count is equal to 1* clock period. For example, with 10MHz functional clock one timeout period will be equal to1*100ns."]
    #[inline(always)]
    pub fn timeout_ctl_tcntlb(&mut self) -> TimeoutCtlTcntlbW<TimeoutCtlSpec> {
        TimeoutCtlTcntlbW::new(self, 16)
    }
    #[doc = "Bit 31 - Timeout Counter B Enable"]
    #[inline(always)]
    pub fn timeout_ctl_tcntben(&mut self) -> TimeoutCtlTcntbenW<TimeoutCtlSpec> {
        TimeoutCtlTcntbenW::new(self, 31)
    }
}
#[doc = "I2C Timeout Count Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimeoutCtlSpec;
impl crate::RegisterSpec for TimeoutCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout_ctl::R`](R) reader structure"]
impl crate::Readable for TimeoutCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`timeout_ctl::W`](W) writer structure"]
impl crate::Writable for TimeoutCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMEOUT_CTL to value 0x0002_0002"]
impl crate::Resettable for TimeoutCtlSpec {
    const RESET_VALUE: u32 = 0x0002_0002;
}
