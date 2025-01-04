#[doc = "Register `CTL2` reader"]
pub type R = crate::R<Ctl2Spec>;
#[doc = "Register `CTL2` writer"]
pub type W = crate::W<Ctl2Spec>;
#[doc = "This bit enables the FIFO and the FIFO hardware control state machine.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl2Fifoen {
    #[doc = "0: CLR"]
    Ctl2FifoenClr = 0,
    #[doc = "1: SET"]
    Ctl2FifoenSet = 1,
}
impl From<Ctl2Fifoen> for bool {
    #[inline(always)]
    fn from(variant: Ctl2Fifoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL2_FIFOEN` reader - This bit enables the FIFO and the FIFO hardware control state machine."]
pub type Ctl2FifoenR = crate::BitReader<Ctl2Fifoen>;
impl Ctl2FifoenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl2Fifoen {
        match self.bits {
            false => Ctl2Fifoen::Ctl2FifoenClr,
            true => Ctl2Fifoen::Ctl2FifoenSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ctl2_fifoen_clr(&self) -> bool {
        *self == Ctl2Fifoen::Ctl2FifoenClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ctl2_fifoen_set(&self) -> bool {
        *self == Ctl2Fifoen::Ctl2FifoenSet
    }
}
#[doc = "Field `CTL2_FIFOEN` writer - This bit enables the FIFO and the FIFO hardware control state machine."]
pub type Ctl2FifoenW<'a, REG> = crate::BitWriter<'a, REG, Ctl2Fifoen>;
impl<'a, REG> Ctl2FifoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn ctl2_fifoen_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Fifoen::Ctl2FifoenClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn ctl2_fifoen_set(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Fifoen::Ctl2FifoenSet)
    }
}
#[doc = "These bits determine the FIFO threshold. In case of DMA based operation, DAC generates new DMA trigger when the number of empty locations in FIFO match the selected FIFO threshold level. In case of CPU based operation, the FIFO threshold bits are don't care and FIFO level is directly indicated through the respective bits in the RIS register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl2Fifoth {
    #[doc = "0: LOW"]
    Ctl2FifothLow = 0,
    #[doc = "1: MED"]
    Ctl2FifothMed = 1,
    #[doc = "2: HIGH"]
    Ctl2FifothHigh = 2,
    #[doc = "3: SPARE"]
    Ctl2FifothSpare = 3,
}
impl From<Ctl2Fifoth> for u8 {
    #[inline(always)]
    fn from(variant: Ctl2Fifoth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl2Fifoth {
    type Ux = u8;
}
impl crate::IsEnum for Ctl2Fifoth {}
#[doc = "Field `CTL2_FIFOTH` reader - These bits determine the FIFO threshold. In case of DMA based operation, DAC generates new DMA trigger when the number of empty locations in FIFO match the selected FIFO threshold level. In case of CPU based operation, the FIFO threshold bits are don't care and FIFO level is directly indicated through the respective bits in the RIS register."]
pub type Ctl2FifothR = crate::FieldReader<Ctl2Fifoth>;
impl Ctl2FifothR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl2Fifoth {
        match self.bits {
            0 => Ctl2Fifoth::Ctl2FifothLow,
            1 => Ctl2Fifoth::Ctl2FifothMed,
            2 => Ctl2Fifoth::Ctl2FifothHigh,
            3 => Ctl2Fifoth::Ctl2FifothSpare,
            _ => unreachable!(),
        }
    }
    #[doc = "LOW"]
    #[inline(always)]
    pub fn is_ctl2_fifoth_low(&self) -> bool {
        *self == Ctl2Fifoth::Ctl2FifothLow
    }
    #[doc = "MED"]
    #[inline(always)]
    pub fn is_ctl2_fifoth_med(&self) -> bool {
        *self == Ctl2Fifoth::Ctl2FifothMed
    }
    #[doc = "HIGH"]
    #[inline(always)]
    pub fn is_ctl2_fifoth_high(&self) -> bool {
        *self == Ctl2Fifoth::Ctl2FifothHigh
    }
    #[doc = "SPARE"]
    #[inline(always)]
    pub fn is_ctl2_fifoth_spare(&self) -> bool {
        *self == Ctl2Fifoth::Ctl2FifothSpare
    }
}
#[doc = "Field `CTL2_FIFOTH` writer - These bits determine the FIFO threshold. In case of DMA based operation, DAC generates new DMA trigger when the number of empty locations in FIFO match the selected FIFO threshold level. In case of CPU based operation, the FIFO threshold bits are don't care and FIFO level is directly indicated through the respective bits in the RIS register."]
pub type Ctl2FifothW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctl2Fifoth, crate::Safe>;
impl<'a, REG> Ctl2FifothW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "LOW"]
    #[inline(always)]
    pub fn ctl2_fifoth_low(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Fifoth::Ctl2FifothLow)
    }
    #[doc = "MED"]
    #[inline(always)]
    pub fn ctl2_fifoth_med(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Fifoth::Ctl2FifothMed)
    }
    #[doc = "HIGH"]
    #[inline(always)]
    pub fn ctl2_fifoth_high(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Fifoth::Ctl2FifothHigh)
    }
    #[doc = "SPARE"]
    #[inline(always)]
    pub fn ctl2_fifoth_spare(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Fifoth::Ctl2FifothSpare)
    }
}
#[doc = "These bits select the source for FIFO read trigger. When the selected FIFO read trigger is asserted, the data from FIFO (as indicated by read pointer) is moved into internal DAC data register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctl2Fifotrigsel {
    #[doc = "0: STIM"]
    Ctl2FifotrigselStim = 0,
    #[doc = "1: TRIG0"]
    Ctl2FifotrigselTrig0 = 1,
    #[doc = "2: TRIG1"]
    Ctl2FifotrigselTrig1 = 2,
    #[doc = "3: SPARE"]
    Ctl2FifotrigselSpare = 3,
}
impl From<Ctl2Fifotrigsel> for u8 {
    #[inline(always)]
    fn from(variant: Ctl2Fifotrigsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctl2Fifotrigsel {
    type Ux = u8;
}
impl crate::IsEnum for Ctl2Fifotrigsel {}
#[doc = "Field `CTL2_FIFOTRIGSEL` reader - These bits select the source for FIFO read trigger. When the selected FIFO read trigger is asserted, the data from FIFO (as indicated by read pointer) is moved into internal DAC data register."]
pub type Ctl2FifotrigselR = crate::FieldReader<Ctl2Fifotrigsel>;
impl Ctl2FifotrigselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl2Fifotrigsel {
        match self.bits {
            0 => Ctl2Fifotrigsel::Ctl2FifotrigselStim,
            1 => Ctl2Fifotrigsel::Ctl2FifotrigselTrig0,
            2 => Ctl2Fifotrigsel::Ctl2FifotrigselTrig1,
            3 => Ctl2Fifotrigsel::Ctl2FifotrigselSpare,
            _ => unreachable!(),
        }
    }
    #[doc = "STIM"]
    #[inline(always)]
    pub fn is_ctl2_fifotrigsel_stim(&self) -> bool {
        *self == Ctl2Fifotrigsel::Ctl2FifotrigselStim
    }
    #[doc = "TRIG0"]
    #[inline(always)]
    pub fn is_ctl2_fifotrigsel_trig0(&self) -> bool {
        *self == Ctl2Fifotrigsel::Ctl2FifotrigselTrig0
    }
    #[doc = "TRIG1"]
    #[inline(always)]
    pub fn is_ctl2_fifotrigsel_trig1(&self) -> bool {
        *self == Ctl2Fifotrigsel::Ctl2FifotrigselTrig1
    }
    #[doc = "SPARE"]
    #[inline(always)]
    pub fn is_ctl2_fifotrigsel_spare(&self) -> bool {
        *self == Ctl2Fifotrigsel::Ctl2FifotrigselSpare
    }
}
#[doc = "Field `CTL2_FIFOTRIGSEL` writer - These bits select the source for FIFO read trigger. When the selected FIFO read trigger is asserted, the data from FIFO (as indicated by read pointer) is moved into internal DAC data register."]
pub type Ctl2FifotrigselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctl2Fifotrigsel, crate::Safe>;
impl<'a, REG> Ctl2FifotrigselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "STIM"]
    #[inline(always)]
    pub fn ctl2_fifotrigsel_stim(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Fifotrigsel::Ctl2FifotrigselStim)
    }
    #[doc = "TRIG0"]
    #[inline(always)]
    pub fn ctl2_fifotrigsel_trig0(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Fifotrigsel::Ctl2FifotrigselTrig0)
    }
    #[doc = "TRIG1"]
    #[inline(always)]
    pub fn ctl2_fifotrigsel_trig1(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Fifotrigsel::Ctl2FifotrigselTrig1)
    }
    #[doc = "SPARE"]
    #[inline(always)]
    pub fn ctl2_fifotrigsel_spare(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Fifotrigsel::Ctl2FifotrigselSpare)
    }
}
#[doc = "This bit enables the DMA trigger generation mechanism. When this bit is set along with FIFOEN, the DMA trigger is generated based on the empty FIFO locations qualified by FIFOTH settings. This bit needs to be cleared by SW to stop further DMA triggers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctl2Dmatrigen {
    #[doc = "0: CLR"]
    Ctl2DmatrigenClr = 0,
    #[doc = "1: SET"]
    Ctl2DmatrigenSet = 1,
}
impl From<Ctl2Dmatrigen> for bool {
    #[inline(always)]
    fn from(variant: Ctl2Dmatrigen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL2_DMATRIGEN` reader - This bit enables the DMA trigger generation mechanism. When this bit is set along with FIFOEN, the DMA trigger is generated based on the empty FIFO locations qualified by FIFOTH settings. This bit needs to be cleared by SW to stop further DMA triggers"]
pub type Ctl2DmatrigenR = crate::BitReader<Ctl2Dmatrigen>;
impl Ctl2DmatrigenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctl2Dmatrigen {
        match self.bits {
            false => Ctl2Dmatrigen::Ctl2DmatrigenClr,
            true => Ctl2Dmatrigen::Ctl2DmatrigenSet,
        }
    }
    #[doc = "CLR"]
    #[inline(always)]
    pub fn is_ctl2_dmatrigen_clr(&self) -> bool {
        *self == Ctl2Dmatrigen::Ctl2DmatrigenClr
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn is_ctl2_dmatrigen_set(&self) -> bool {
        *self == Ctl2Dmatrigen::Ctl2DmatrigenSet
    }
}
#[doc = "Field `CTL2_DMATRIGEN` writer - This bit enables the DMA trigger generation mechanism. When this bit is set along with FIFOEN, the DMA trigger is generated based on the empty FIFO locations qualified by FIFOTH settings. This bit needs to be cleared by SW to stop further DMA triggers"]
pub type Ctl2DmatrigenW<'a, REG> = crate::BitWriter<'a, REG, Ctl2Dmatrigen>;
impl<'a, REG> Ctl2DmatrigenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLR"]
    #[inline(always)]
    pub fn ctl2_dmatrigen_clr(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Dmatrigen::Ctl2DmatrigenClr)
    }
    #[doc = "SET"]
    #[inline(always)]
    pub fn ctl2_dmatrigen_set(self) -> &'a mut crate::W<REG> {
        self.variant(Ctl2Dmatrigen::Ctl2DmatrigenSet)
    }
}
impl R {
    #[doc = "Bit 0 - This bit enables the FIFO and the FIFO hardware control state machine."]
    #[inline(always)]
    pub fn ctl2_fifoen(&self) -> Ctl2FifoenR {
        Ctl2FifoenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - These bits determine the FIFO threshold. In case of DMA based operation, DAC generates new DMA trigger when the number of empty locations in FIFO match the selected FIFO threshold level. In case of CPU based operation, the FIFO threshold bits are don't care and FIFO level is directly indicated through the respective bits in the RIS register."]
    #[inline(always)]
    pub fn ctl2_fifoth(&self) -> Ctl2FifothR {
        Ctl2FifothR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - These bits select the source for FIFO read trigger. When the selected FIFO read trigger is asserted, the data from FIFO (as indicated by read pointer) is moved into internal DAC data register."]
    #[inline(always)]
    pub fn ctl2_fifotrigsel(&self) -> Ctl2FifotrigselR {
        Ctl2FifotrigselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - This bit enables the DMA trigger generation mechanism. When this bit is set along with FIFOEN, the DMA trigger is generated based on the empty FIFO locations qualified by FIFOTH settings. This bit needs to be cleared by SW to stop further DMA triggers"]
    #[inline(always)]
    pub fn ctl2_dmatrigen(&self) -> Ctl2DmatrigenR {
        Ctl2DmatrigenR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit enables the FIFO and the FIFO hardware control state machine."]
    #[inline(always)]
    pub fn ctl2_fifoen(&mut self) -> Ctl2FifoenW<Ctl2Spec> {
        Ctl2FifoenW::new(self, 0)
    }
    #[doc = "Bits 8:9 - These bits determine the FIFO threshold. In case of DMA based operation, DAC generates new DMA trigger when the number of empty locations in FIFO match the selected FIFO threshold level. In case of CPU based operation, the FIFO threshold bits are don't care and FIFO level is directly indicated through the respective bits in the RIS register."]
    #[inline(always)]
    pub fn ctl2_fifoth(&mut self) -> Ctl2FifothW<Ctl2Spec> {
        Ctl2FifothW::new(self, 8)
    }
    #[doc = "Bits 16:17 - These bits select the source for FIFO read trigger. When the selected FIFO read trigger is asserted, the data from FIFO (as indicated by read pointer) is moved into internal DAC data register."]
    #[inline(always)]
    pub fn ctl2_fifotrigsel(&mut self) -> Ctl2FifotrigselW<Ctl2Spec> {
        Ctl2FifotrigselW::new(self, 16)
    }
    #[doc = "Bit 24 - This bit enables the DMA trigger generation mechanism. When this bit is set along with FIFOEN, the DMA trigger is generated based on the empty FIFO locations qualified by FIFOTH settings. This bit needs to be cleared by SW to stop further DMA triggers"]
    #[inline(always)]
    pub fn ctl2_dmatrigen(&mut self) -> Ctl2DmatrigenW<Ctl2Spec> {
        Ctl2DmatrigenW::new(self, 24)
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
