#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Sets the TRNG mode through a command. The mode will not be updated until the previous command is done, as indicated by IRQ_CMD_DONE. 00 --&gt; OFF 01 --&gt; PWRUP_DIG 10 --&gt; PWRUP_ANA 11 --&gt; NORM_FUNC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CtlCmd {
    #[doc = "0: PWR_OFF"]
    CtlCmdPwrOff = 0,
    #[doc = "1: PWRUP_DIG"]
    CtlCmdPwrupDig = 1,
    #[doc = "2: PWRUP_ANA"]
    CtlCmdPwrupAna = 2,
    #[doc = "3: NORM_FUNC"]
    CtlCmdNormFunc = 3,
}
impl From<CtlCmd> for u8 {
    #[inline(always)]
    fn from(variant: CtlCmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CtlCmd {
    type Ux = u8;
}
impl crate::IsEnum for CtlCmd {}
#[doc = "Field `CTL_CMD` reader - Sets the TRNG mode through a command. The mode will not be updated until the previous command is done, as indicated by IRQ_CMD_DONE. 00 --&gt; OFF 01 --&gt; PWRUP_DIG 10 --&gt; PWRUP_ANA 11 --&gt; NORM_FUNC"]
pub type CtlCmdR = crate::FieldReader<CtlCmd>;
impl CtlCmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtlCmd {
        match self.bits {
            0 => CtlCmd::CtlCmdPwrOff,
            1 => CtlCmd::CtlCmdPwrupDig,
            2 => CtlCmd::CtlCmdPwrupAna,
            3 => CtlCmd::CtlCmdNormFunc,
            _ => unreachable!(),
        }
    }
    #[doc = "PWR_OFF"]
    #[inline(always)]
    pub fn is_ctl_cmd_pwr_off(&self) -> bool {
        *self == CtlCmd::CtlCmdPwrOff
    }
    #[doc = "PWRUP_DIG"]
    #[inline(always)]
    pub fn is_ctl_cmd_pwrup_dig(&self) -> bool {
        *self == CtlCmd::CtlCmdPwrupDig
    }
    #[doc = "PWRUP_ANA"]
    #[inline(always)]
    pub fn is_ctl_cmd_pwrup_ana(&self) -> bool {
        *self == CtlCmd::CtlCmdPwrupAna
    }
    #[doc = "NORM_FUNC"]
    #[inline(always)]
    pub fn is_ctl_cmd_norm_func(&self) -> bool {
        *self == CtlCmd::CtlCmdNormFunc
    }
}
#[doc = "Field `CTL_CMD` writer - Sets the TRNG mode through a command. The mode will not be updated until the previous command is done, as indicated by IRQ_CMD_DONE. 00 --&gt; OFF 01 --&gt; PWRUP_DIG 10 --&gt; PWRUP_ANA 11 --&gt; NORM_FUNC"]
pub type CtlCmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, CtlCmd, crate::Safe>;
impl<'a, REG> CtlCmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PWR_OFF"]
    #[inline(always)]
    pub fn ctl_cmd_pwr_off(self) -> &'a mut crate::W<REG> {
        self.variant(CtlCmd::CtlCmdPwrOff)
    }
    #[doc = "PWRUP_DIG"]
    #[inline(always)]
    pub fn ctl_cmd_pwrup_dig(self) -> &'a mut crate::W<REG> {
        self.variant(CtlCmd::CtlCmdPwrupDig)
    }
    #[doc = "PWRUP_ANA"]
    #[inline(always)]
    pub fn ctl_cmd_pwrup_ana(self) -> &'a mut crate::W<REG> {
        self.variant(CtlCmd::CtlCmdPwrupAna)
    }
    #[doc = "NORM_FUNC"]
    #[inline(always)]
    pub fn ctl_cmd_norm_func(self) -> &'a mut crate::W<REG> {
        self.variant(CtlCmd::CtlCmdNormFunc)
    }
}
#[doc = "Field `CTL_DECIM_RATE` reader - Set decimation rate. Decimate by n 0x0 = Decimation by 1 (no decimation) 0x1 = Decimation by 2 (Skip every other sample) 0x7 = Decimation by 8 (Take every 8th sample)"]
pub type CtlDecimRateR = crate::FieldReader;
#[doc = "Field `CTL_DECIM_RATE` writer - Set decimation rate. Decimate by n 0x0 = Decimation by 1 (no decimation) 0x1 = Decimation by 2 (Skip every other sample) 0x7 = Decimation by 8 (Take every 8th sample)"]
pub type CtlDecimRateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CTL_PWRUP_CLKDIV` reader - When '1', the powerup sequence will take twice as long (i.e., clock frequency halved)"]
pub type CtlPwrupClkdivR = crate::BitReader;
#[doc = "Field `CTL_PWRUP_CLKDIV` writer - When '1', the powerup sequence will take twice as long (i.e., clock frequency halved)"]
pub type CtlPwrupClkdivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTL_PWRUP_PCHRG_CFG` reader - Configure PCHARGE sequence length b00 = Disabled b01 = 20 us PCHARGE b10 = 30 us PCHARGE (default) b11 = 40 us PCHARGE"]
pub type CtlPwrupPchrgCfgR = crate::FieldReader;
#[doc = "Field `CTL_PWRUP_PCHRG_CFG` writer - Configure PCHARGE sequence length b00 = Disabled b01 = 20 us PCHARGE b10 = 30 us PCHARGE (default) b11 = 40 us PCHARGE"]
pub type CtlPwrupPchrgCfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CTL_PWRUP_PSTART_CFG` reader - Configure pusle startup sequence length b00 = Disabled b01 = rise at 10us, fall at 50us b10 = rise at 10us, fall at 70us (default) b11 = rise at 10us, fall at 90us"]
pub type CtlPwrupPstartCfgR = crate::FieldReader;
#[doc = "Field `CTL_PWRUP_PSTART_CFG` writer - Configure pusle startup sequence length b00 = Disabled b01 = rise at 10us, fall at 50us b10 = rise at 10us, fall at 70us (default) b11 = rise at 10us, fall at 90us"]
pub type CtlPwrupPstartCfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Sets the TRNG mode through a command. The mode will not be updated until the previous command is done, as indicated by IRQ_CMD_DONE. 00 --&gt; OFF 01 --&gt; PWRUP_DIG 10 --&gt; PWRUP_ANA 11 --&gt; NORM_FUNC"]
    #[inline(always)]
    pub fn ctl_cmd(&self) -> CtlCmdR {
        CtlCmdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:10 - Set decimation rate. Decimate by n 0x0 = Decimation by 1 (no decimation) 0x1 = Decimation by 2 (Skip every other sample) 0x7 = Decimation by 8 (Take every 8th sample)"]
    #[inline(always)]
    pub fn ctl_decim_rate(&self) -> CtlDecimRateR {
        CtlDecimRateR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - When '1', the powerup sequence will take twice as long (i.e., clock frequency halved)"]
    #[inline(always)]
    pub fn ctl_pwrup_clkdiv(&self) -> CtlPwrupClkdivR {
        CtlPwrupClkdivR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Configure PCHARGE sequence length b00 = Disabled b01 = 20 us PCHARGE b10 = 30 us PCHARGE (default) b11 = 40 us PCHARGE"]
    #[inline(always)]
    pub fn ctl_pwrup_pchrg_cfg(&self) -> CtlPwrupPchrgCfgR {
        CtlPwrupPchrgCfgR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:20 - Configure pusle startup sequence length b00 = Disabled b01 = rise at 10us, fall at 50us b10 = rise at 10us, fall at 70us (default) b11 = rise at 10us, fall at 90us"]
    #[inline(always)]
    pub fn ctl_pwrup_pstart_cfg(&self) -> CtlPwrupPstartCfgR {
        CtlPwrupPstartCfgR::new(((self.bits >> 19) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Sets the TRNG mode through a command. The mode will not be updated until the previous command is done, as indicated by IRQ_CMD_DONE. 00 --&gt; OFF 01 --&gt; PWRUP_DIG 10 --&gt; PWRUP_ANA 11 --&gt; NORM_FUNC"]
    #[inline(always)]
    pub fn ctl_cmd(&mut self) -> CtlCmdW<CtlSpec> {
        CtlCmdW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Set decimation rate. Decimate by n 0x0 = Decimation by 1 (no decimation) 0x1 = Decimation by 2 (Skip every other sample) 0x7 = Decimation by 8 (Take every 8th sample)"]
    #[inline(always)]
    pub fn ctl_decim_rate(&mut self) -> CtlDecimRateW<CtlSpec> {
        CtlDecimRateW::new(self, 8)
    }
    #[doc = "Bit 16 - When '1', the powerup sequence will take twice as long (i.e., clock frequency halved)"]
    #[inline(always)]
    pub fn ctl_pwrup_clkdiv(&mut self) -> CtlPwrupClkdivW<CtlSpec> {
        CtlPwrupClkdivW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Configure PCHARGE sequence length b00 = Disabled b01 = 20 us PCHARGE b10 = 30 us PCHARGE (default) b11 = 40 us PCHARGE"]
    #[inline(always)]
    pub fn ctl_pwrup_pchrg_cfg(&mut self) -> CtlPwrupPchrgCfgW<CtlSpec> {
        CtlPwrupPchrgCfgW::new(self, 17)
    }
    #[doc = "Bits 19:20 - Configure pusle startup sequence length b00 = Disabled b01 = rise at 10us, fall at 50us b10 = rise at 10us, fall at 70us (default) b11 = rise at 10us, fall at 90us"]
    #[inline(always)]
    pub fn ctl_pwrup_pstart_cfg(&mut self) -> CtlPwrupPstartCfgW<CtlSpec> {
        CtlPwrupPstartCfgW::new(self, 19)
    }
}
#[doc = "Controls the command and decimation rate\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTL to value 0x0014_0000"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0x0014_0000;
}
