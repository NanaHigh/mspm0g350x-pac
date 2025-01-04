#[doc = "Register `MEMCTL[%s]` reader"]
pub type R = crate::R<MemctlSpec>;
#[doc = "Register `MEMCTL[%s]` writer"]
pub type W = crate::W<MemctlSpec>;
#[doc = "Input channel select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MemctlChansel {
    #[doc = "0: CHAN_0"]
    MemctlChanselChan0 = 0,
    #[doc = "1: CHAN_1"]
    MemctlChanselChan1 = 1,
    #[doc = "2: CHAN_2"]
    MemctlChanselChan2 = 2,
    #[doc = "3: CHAN_3"]
    MemctlChanselChan3 = 3,
    #[doc = "4: CHAN_4"]
    MemctlChanselChan4 = 4,
    #[doc = "5: CHAN_5"]
    MemctlChanselChan5 = 5,
    #[doc = "6: CHAN_6"]
    MemctlChanselChan6 = 6,
    #[doc = "7: CHAN_7"]
    MemctlChanselChan7 = 7,
    #[doc = "8: CHAN_8"]
    MemctlChanselChan8 = 8,
    #[doc = "9: CHAN_9"]
    MemctlChanselChan9 = 9,
    #[doc = "10: CHAN_10"]
    MemctlChanselChan10 = 10,
    #[doc = "11: CHAN_11"]
    MemctlChanselChan11 = 11,
    #[doc = "12: CHAN_12"]
    MemctlChanselChan12 = 12,
    #[doc = "13: CHAN_13"]
    MemctlChanselChan13 = 13,
    #[doc = "14: CHAN_14"]
    MemctlChanselChan14 = 14,
    #[doc = "15: CHAN_15"]
    MemctlChanselChan15 = 15,
    #[doc = "16: CHAN_16"]
    MemctlChanselChan16 = 16,
    #[doc = "17: CHAN_17"]
    MemctlChanselChan17 = 17,
    #[doc = "18: CHAN_18"]
    MemctlChanselChan18 = 18,
    #[doc = "19: CHAN_19"]
    MemctlChanselChan19 = 19,
    #[doc = "20: CHAN_20"]
    MemctlChanselChan20 = 20,
    #[doc = "21: CHAN_21"]
    MemctlChanselChan21 = 21,
    #[doc = "22: CHAN_22"]
    MemctlChanselChan22 = 22,
    #[doc = "23: CHAN_23"]
    MemctlChanselChan23 = 23,
    #[doc = "24: CHAN_24"]
    MemctlChanselChan24 = 24,
    #[doc = "25: CHAN_25"]
    MemctlChanselChan25 = 25,
    #[doc = "26: CHAN_26"]
    MemctlChanselChan26 = 26,
    #[doc = "27: CHAN_27"]
    MemctlChanselChan27 = 27,
    #[doc = "28: CHAN_28"]
    MemctlChanselChan28 = 28,
    #[doc = "29: CHAN_29"]
    MemctlChanselChan29 = 29,
    #[doc = "30: CHAN_30"]
    MemctlChanselChan30 = 30,
    #[doc = "31: CHAN_31"]
    MemctlChanselChan31 = 31,
}
impl From<MemctlChansel> for u8 {
    #[inline(always)]
    fn from(variant: MemctlChansel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MemctlChansel {
    type Ux = u8;
}
impl crate::IsEnum for MemctlChansel {}
#[doc = "Field `MEMCTL_CHANSEL` reader - Input channel select."]
pub type MemctlChanselR = crate::FieldReader<MemctlChansel>;
impl MemctlChanselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MemctlChansel {
        match self.bits {
            0 => MemctlChansel::MemctlChanselChan0,
            1 => MemctlChansel::MemctlChanselChan1,
            2 => MemctlChansel::MemctlChanselChan2,
            3 => MemctlChansel::MemctlChanselChan3,
            4 => MemctlChansel::MemctlChanselChan4,
            5 => MemctlChansel::MemctlChanselChan5,
            6 => MemctlChansel::MemctlChanselChan6,
            7 => MemctlChansel::MemctlChanselChan7,
            8 => MemctlChansel::MemctlChanselChan8,
            9 => MemctlChansel::MemctlChanselChan9,
            10 => MemctlChansel::MemctlChanselChan10,
            11 => MemctlChansel::MemctlChanselChan11,
            12 => MemctlChansel::MemctlChanselChan12,
            13 => MemctlChansel::MemctlChanselChan13,
            14 => MemctlChansel::MemctlChanselChan14,
            15 => MemctlChansel::MemctlChanselChan15,
            16 => MemctlChansel::MemctlChanselChan16,
            17 => MemctlChansel::MemctlChanselChan17,
            18 => MemctlChansel::MemctlChanselChan18,
            19 => MemctlChansel::MemctlChanselChan19,
            20 => MemctlChansel::MemctlChanselChan20,
            21 => MemctlChansel::MemctlChanselChan21,
            22 => MemctlChansel::MemctlChanselChan22,
            23 => MemctlChansel::MemctlChanselChan23,
            24 => MemctlChansel::MemctlChanselChan24,
            25 => MemctlChansel::MemctlChanselChan25,
            26 => MemctlChansel::MemctlChanselChan26,
            27 => MemctlChansel::MemctlChanselChan27,
            28 => MemctlChansel::MemctlChanselChan28,
            29 => MemctlChansel::MemctlChanselChan29,
            30 => MemctlChansel::MemctlChanselChan30,
            31 => MemctlChansel::MemctlChanselChan31,
            _ => unreachable!(),
        }
    }
    #[doc = "CHAN_0"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_0(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan0
    }
    #[doc = "CHAN_1"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_1(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan1
    }
    #[doc = "CHAN_2"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_2(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan2
    }
    #[doc = "CHAN_3"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_3(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan3
    }
    #[doc = "CHAN_4"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_4(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan4
    }
    #[doc = "CHAN_5"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_5(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan5
    }
    #[doc = "CHAN_6"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_6(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan6
    }
    #[doc = "CHAN_7"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_7(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan7
    }
    #[doc = "CHAN_8"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_8(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan8
    }
    #[doc = "CHAN_9"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_9(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan9
    }
    #[doc = "CHAN_10"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_10(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan10
    }
    #[doc = "CHAN_11"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_11(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan11
    }
    #[doc = "CHAN_12"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_12(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan12
    }
    #[doc = "CHAN_13"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_13(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan13
    }
    #[doc = "CHAN_14"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_14(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan14
    }
    #[doc = "CHAN_15"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_15(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan15
    }
    #[doc = "CHAN_16"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_16(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan16
    }
    #[doc = "CHAN_17"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_17(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan17
    }
    #[doc = "CHAN_18"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_18(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan18
    }
    #[doc = "CHAN_19"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_19(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan19
    }
    #[doc = "CHAN_20"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_20(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan20
    }
    #[doc = "CHAN_21"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_21(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan21
    }
    #[doc = "CHAN_22"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_22(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan22
    }
    #[doc = "CHAN_23"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_23(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan23
    }
    #[doc = "CHAN_24"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_24(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan24
    }
    #[doc = "CHAN_25"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_25(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan25
    }
    #[doc = "CHAN_26"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_26(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan26
    }
    #[doc = "CHAN_27"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_27(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan27
    }
    #[doc = "CHAN_28"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_28(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan28
    }
    #[doc = "CHAN_29"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_29(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan29
    }
    #[doc = "CHAN_30"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_30(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan30
    }
    #[doc = "CHAN_31"]
    #[inline(always)]
    pub fn is_memctl_chansel_chan_31(&self) -> bool {
        *self == MemctlChansel::MemctlChanselChan31
    }
}
#[doc = "Field `MEMCTL_CHANSEL` writer - Input channel select."]
pub type MemctlChanselW<'a, REG> = crate::FieldWriter<'a, REG, 5, MemctlChansel, crate::Safe>;
impl<'a, REG> MemctlChanselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CHAN_0"]
    #[inline(always)]
    pub fn memctl_chansel_chan_0(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan0)
    }
    #[doc = "CHAN_1"]
    #[inline(always)]
    pub fn memctl_chansel_chan_1(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan1)
    }
    #[doc = "CHAN_2"]
    #[inline(always)]
    pub fn memctl_chansel_chan_2(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan2)
    }
    #[doc = "CHAN_3"]
    #[inline(always)]
    pub fn memctl_chansel_chan_3(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan3)
    }
    #[doc = "CHAN_4"]
    #[inline(always)]
    pub fn memctl_chansel_chan_4(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan4)
    }
    #[doc = "CHAN_5"]
    #[inline(always)]
    pub fn memctl_chansel_chan_5(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan5)
    }
    #[doc = "CHAN_6"]
    #[inline(always)]
    pub fn memctl_chansel_chan_6(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan6)
    }
    #[doc = "CHAN_7"]
    #[inline(always)]
    pub fn memctl_chansel_chan_7(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan7)
    }
    #[doc = "CHAN_8"]
    #[inline(always)]
    pub fn memctl_chansel_chan_8(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan8)
    }
    #[doc = "CHAN_9"]
    #[inline(always)]
    pub fn memctl_chansel_chan_9(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan9)
    }
    #[doc = "CHAN_10"]
    #[inline(always)]
    pub fn memctl_chansel_chan_10(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan10)
    }
    #[doc = "CHAN_11"]
    #[inline(always)]
    pub fn memctl_chansel_chan_11(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan11)
    }
    #[doc = "CHAN_12"]
    #[inline(always)]
    pub fn memctl_chansel_chan_12(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan12)
    }
    #[doc = "CHAN_13"]
    #[inline(always)]
    pub fn memctl_chansel_chan_13(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan13)
    }
    #[doc = "CHAN_14"]
    #[inline(always)]
    pub fn memctl_chansel_chan_14(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan14)
    }
    #[doc = "CHAN_15"]
    #[inline(always)]
    pub fn memctl_chansel_chan_15(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan15)
    }
    #[doc = "CHAN_16"]
    #[inline(always)]
    pub fn memctl_chansel_chan_16(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan16)
    }
    #[doc = "CHAN_17"]
    #[inline(always)]
    pub fn memctl_chansel_chan_17(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan17)
    }
    #[doc = "CHAN_18"]
    #[inline(always)]
    pub fn memctl_chansel_chan_18(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan18)
    }
    #[doc = "CHAN_19"]
    #[inline(always)]
    pub fn memctl_chansel_chan_19(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan19)
    }
    #[doc = "CHAN_20"]
    #[inline(always)]
    pub fn memctl_chansel_chan_20(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan20)
    }
    #[doc = "CHAN_21"]
    #[inline(always)]
    pub fn memctl_chansel_chan_21(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan21)
    }
    #[doc = "CHAN_22"]
    #[inline(always)]
    pub fn memctl_chansel_chan_22(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan22)
    }
    #[doc = "CHAN_23"]
    #[inline(always)]
    pub fn memctl_chansel_chan_23(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan23)
    }
    #[doc = "CHAN_24"]
    #[inline(always)]
    pub fn memctl_chansel_chan_24(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan24)
    }
    #[doc = "CHAN_25"]
    #[inline(always)]
    pub fn memctl_chansel_chan_25(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan25)
    }
    #[doc = "CHAN_26"]
    #[inline(always)]
    pub fn memctl_chansel_chan_26(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan26)
    }
    #[doc = "CHAN_27"]
    #[inline(always)]
    pub fn memctl_chansel_chan_27(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan27)
    }
    #[doc = "CHAN_28"]
    #[inline(always)]
    pub fn memctl_chansel_chan_28(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan28)
    }
    #[doc = "CHAN_29"]
    #[inline(always)]
    pub fn memctl_chansel_chan_29(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan29)
    }
    #[doc = "CHAN_30"]
    #[inline(always)]
    pub fn memctl_chansel_chan_30(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan30)
    }
    #[doc = "CHAN_31"]
    #[inline(always)]
    pub fn memctl_chansel_chan_31(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlChansel::MemctlChanselChan31)
    }
}
#[doc = "Voltage reference selection. VEREFM must be connected to on-board ground when external reference option is selected. Note: Writing value 0x3 defaults to INTREF.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MemctlVrsel {
    #[doc = "0: VDDA"]
    MemctlVrselVdda = 0,
    #[doc = "1: EXTREF"]
    MemctlVrselExtref = 1,
    #[doc = "2: INTREF"]
    MemctlVrselIntref = 2,
}
impl From<MemctlVrsel> for u8 {
    #[inline(always)]
    fn from(variant: MemctlVrsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MemctlVrsel {
    type Ux = u8;
}
impl crate::IsEnum for MemctlVrsel {}
#[doc = "Field `MEMCTL_VRSEL` reader - Voltage reference selection. VEREFM must be connected to on-board ground when external reference option is selected. Note: Writing value 0x3 defaults to INTREF."]
pub type MemctlVrselR = crate::FieldReader<MemctlVrsel>;
impl MemctlVrselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MemctlVrsel> {
        match self.bits {
            0 => Some(MemctlVrsel::MemctlVrselVdda),
            1 => Some(MemctlVrsel::MemctlVrselExtref),
            2 => Some(MemctlVrsel::MemctlVrselIntref),
            _ => None,
        }
    }
    #[doc = "VDDA"]
    #[inline(always)]
    pub fn is_memctl_vrsel_vdda(&self) -> bool {
        *self == MemctlVrsel::MemctlVrselVdda
    }
    #[doc = "EXTREF"]
    #[inline(always)]
    pub fn is_memctl_vrsel_extref(&self) -> bool {
        *self == MemctlVrsel::MemctlVrselExtref
    }
    #[doc = "INTREF"]
    #[inline(always)]
    pub fn is_memctl_vrsel_intref(&self) -> bool {
        *self == MemctlVrsel::MemctlVrselIntref
    }
}
#[doc = "Field `MEMCTL_VRSEL` writer - Voltage reference selection. VEREFM must be connected to on-board ground when external reference option is selected. Note: Writing value 0x3 defaults to INTREF."]
pub type MemctlVrselW<'a, REG> = crate::FieldWriter<'a, REG, 2, MemctlVrsel>;
impl<'a, REG> MemctlVrselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "VDDA"]
    #[inline(always)]
    pub fn memctl_vrsel_vdda(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlVrsel::MemctlVrselVdda)
    }
    #[doc = "EXTREF"]
    #[inline(always)]
    pub fn memctl_vrsel_extref(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlVrsel::MemctlVrselExtref)
    }
    #[doc = "INTREF"]
    #[inline(always)]
    pub fn memctl_vrsel_intref(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlVrsel::MemctlVrselIntref)
    }
}
#[doc = "Selects the source of sample timer period between SCOMP0 and SCOMP1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemctlStime {
    #[doc = "0: SEL_SCOMP0"]
    MemctlStimeSelScomp0 = 0,
    #[doc = "1: SEL_SCOMP1"]
    MemctlStimeSelScomp1 = 1,
}
impl From<MemctlStime> for bool {
    #[inline(always)]
    fn from(variant: MemctlStime) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMCTL_STIME` reader - Selects the source of sample timer period between SCOMP0 and SCOMP1."]
pub type MemctlStimeR = crate::BitReader<MemctlStime>;
impl MemctlStimeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MemctlStime {
        match self.bits {
            false => MemctlStime::MemctlStimeSelScomp0,
            true => MemctlStime::MemctlStimeSelScomp1,
        }
    }
    #[doc = "SEL_SCOMP0"]
    #[inline(always)]
    pub fn is_memctl_stime_sel_scomp0(&self) -> bool {
        *self == MemctlStime::MemctlStimeSelScomp0
    }
    #[doc = "SEL_SCOMP1"]
    #[inline(always)]
    pub fn is_memctl_stime_sel_scomp1(&self) -> bool {
        *self == MemctlStime::MemctlStimeSelScomp1
    }
}
#[doc = "Field `MEMCTL_STIME` writer - Selects the source of sample timer period between SCOMP0 and SCOMP1."]
pub type MemctlStimeW<'a, REG> = crate::BitWriter<'a, REG, MemctlStime>;
impl<'a, REG> MemctlStimeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SEL_SCOMP0"]
    #[inline(always)]
    pub fn memctl_stime_sel_scomp0(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlStime::MemctlStimeSelScomp0)
    }
    #[doc = "SEL_SCOMP1"]
    #[inline(always)]
    pub fn memctl_stime_sel_scomp1(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlStime::MemctlStimeSelScomp1)
    }
}
#[doc = "Enable hardware averaging.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemctlAvgen {
    #[doc = "0: DISABLE"]
    MemctlAvgenDisable = 0,
    #[doc = "1: ENABLE"]
    MemctlAvgenEnable = 1,
}
impl From<MemctlAvgen> for bool {
    #[inline(always)]
    fn from(variant: MemctlAvgen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMCTL_AVGEN` reader - Enable hardware averaging."]
pub type MemctlAvgenR = crate::BitReader<MemctlAvgen>;
impl MemctlAvgenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MemctlAvgen {
        match self.bits {
            false => MemctlAvgen::MemctlAvgenDisable,
            true => MemctlAvgen::MemctlAvgenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_memctl_avgen_disable(&self) -> bool {
        *self == MemctlAvgen::MemctlAvgenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_memctl_avgen_enable(&self) -> bool {
        *self == MemctlAvgen::MemctlAvgenEnable
    }
}
#[doc = "Field `MEMCTL_AVGEN` writer - Enable hardware averaging."]
pub type MemctlAvgenW<'a, REG> = crate::BitWriter<'a, REG, MemctlAvgen>;
impl<'a, REG> MemctlAvgenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn memctl_avgen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlAvgen::MemctlAvgenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn memctl_avgen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlAvgen::MemctlAvgenEnable)
    }
}
#[doc = "Enable burn out current source.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemctlBcsen {
    #[doc = "0: DISABLE"]
    MemctlBcsenDisable = 0,
    #[doc = "1: ENABLE"]
    MemctlBcsenEnable = 1,
}
impl From<MemctlBcsen> for bool {
    #[inline(always)]
    fn from(variant: MemctlBcsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMCTL_BCSEN` reader - Enable burn out current source."]
pub type MemctlBcsenR = crate::BitReader<MemctlBcsen>;
impl MemctlBcsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MemctlBcsen {
        match self.bits {
            false => MemctlBcsen::MemctlBcsenDisable,
            true => MemctlBcsen::MemctlBcsenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_memctl_bcsen_disable(&self) -> bool {
        *self == MemctlBcsen::MemctlBcsenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_memctl_bcsen_enable(&self) -> bool {
        *self == MemctlBcsen::MemctlBcsenEnable
    }
}
#[doc = "Field `MEMCTL_BCSEN` writer - Enable burn out current source."]
pub type MemctlBcsenW<'a, REG> = crate::BitWriter<'a, REG, MemctlBcsen>;
impl<'a, REG> MemctlBcsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn memctl_bcsen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlBcsen::MemctlBcsenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn memctl_bcsen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlBcsen::MemctlBcsenEnable)
    }
}
#[doc = "Trigger policy. Indicates if a trigger will be needed to step to the next MEMCTL in the sequence or to perform next conversion in the case of repeat single channel conversions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemctlTrig {
    #[doc = "0: AUTO_NEXT"]
    MemctlTrigAutoNext = 0,
    #[doc = "1: TRIGGER_NEXT"]
    MemctlTrigTriggerNext = 1,
}
impl From<MemctlTrig> for bool {
    #[inline(always)]
    fn from(variant: MemctlTrig) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMCTL_TRIG` reader - Trigger policy. Indicates if a trigger will be needed to step to the next MEMCTL in the sequence or to perform next conversion in the case of repeat single channel conversions."]
pub type MemctlTrigR = crate::BitReader<MemctlTrig>;
impl MemctlTrigR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MemctlTrig {
        match self.bits {
            false => MemctlTrig::MemctlTrigAutoNext,
            true => MemctlTrig::MemctlTrigTriggerNext,
        }
    }
    #[doc = "AUTO_NEXT"]
    #[inline(always)]
    pub fn is_memctl_trig_auto_next(&self) -> bool {
        *self == MemctlTrig::MemctlTrigAutoNext
    }
    #[doc = "TRIGGER_NEXT"]
    #[inline(always)]
    pub fn is_memctl_trig_trigger_next(&self) -> bool {
        *self == MemctlTrig::MemctlTrigTriggerNext
    }
}
#[doc = "Field `MEMCTL_TRIG` writer - Trigger policy. Indicates if a trigger will be needed to step to the next MEMCTL in the sequence or to perform next conversion in the case of repeat single channel conversions."]
pub type MemctlTrigW<'a, REG> = crate::BitWriter<'a, REG, MemctlTrig>;
impl<'a, REG> MemctlTrigW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AUTO_NEXT"]
    #[inline(always)]
    pub fn memctl_trig_auto_next(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlTrig::MemctlTrigAutoNext)
    }
    #[doc = "TRIGGER_NEXT"]
    #[inline(always)]
    pub fn memctl_trig_trigger_next(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlTrig::MemctlTrigTriggerNext)
    }
}
#[doc = "Enable window comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemctlWincomp {
    #[doc = "0: DISABLE"]
    MemctlWincompDisable = 0,
    #[doc = "1: ENABLE"]
    MemctlWincompEnable = 1,
}
impl From<MemctlWincomp> for bool {
    #[inline(always)]
    fn from(variant: MemctlWincomp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEMCTL_WINCOMP` reader - Enable window comparator."]
pub type MemctlWincompR = crate::BitReader<MemctlWincomp>;
impl MemctlWincompR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MemctlWincomp {
        match self.bits {
            false => MemctlWincomp::MemctlWincompDisable,
            true => MemctlWincomp::MemctlWincompEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_memctl_wincomp_disable(&self) -> bool {
        *self == MemctlWincomp::MemctlWincompDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_memctl_wincomp_enable(&self) -> bool {
        *self == MemctlWincomp::MemctlWincompEnable
    }
}
#[doc = "Field `MEMCTL_WINCOMP` writer - Enable window comparator."]
pub type MemctlWincompW<'a, REG> = crate::BitWriter<'a, REG, MemctlWincomp>;
impl<'a, REG> MemctlWincompW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn memctl_wincomp_disable(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlWincomp::MemctlWincompDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn memctl_wincomp_enable(self) -> &'a mut crate::W<REG> {
        self.variant(MemctlWincomp::MemctlWincompEnable)
    }
}
impl R {
    #[doc = "Bits 0:4 - Input channel select."]
    #[inline(always)]
    pub fn memctl_chansel(&self) -> MemctlChanselR {
        MemctlChanselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - Voltage reference selection. VEREFM must be connected to on-board ground when external reference option is selected. Note: Writing value 0x3 defaults to INTREF."]
    #[inline(always)]
    pub fn memctl_vrsel(&self) -> MemctlVrselR {
        MemctlVrselR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Selects the source of sample timer period between SCOMP0 and SCOMP1."]
    #[inline(always)]
    pub fn memctl_stime(&self) -> MemctlStimeR {
        MemctlStimeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable hardware averaging."]
    #[inline(always)]
    pub fn memctl_avgen(&self) -> MemctlAvgenR {
        MemctlAvgenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable burn out current source."]
    #[inline(always)]
    pub fn memctl_bcsen(&self) -> MemctlBcsenR {
        MemctlBcsenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Trigger policy. Indicates if a trigger will be needed to step to the next MEMCTL in the sequence or to perform next conversion in the case of repeat single channel conversions."]
    #[inline(always)]
    pub fn memctl_trig(&self) -> MemctlTrigR {
        MemctlTrigR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable window comparator."]
    #[inline(always)]
    pub fn memctl_wincomp(&self) -> MemctlWincompR {
        MemctlWincompR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel select."]
    #[inline(always)]
    pub fn memctl_chansel(&mut self) -> MemctlChanselW<MemctlSpec> {
        MemctlChanselW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Voltage reference selection. VEREFM must be connected to on-board ground when external reference option is selected. Note: Writing value 0x3 defaults to INTREF."]
    #[inline(always)]
    pub fn memctl_vrsel(&mut self) -> MemctlVrselW<MemctlSpec> {
        MemctlVrselW::new(self, 8)
    }
    #[doc = "Bit 12 - Selects the source of sample timer period between SCOMP0 and SCOMP1."]
    #[inline(always)]
    pub fn memctl_stime(&mut self) -> MemctlStimeW<MemctlSpec> {
        MemctlStimeW::new(self, 12)
    }
    #[doc = "Bit 16 - Enable hardware averaging."]
    #[inline(always)]
    pub fn memctl_avgen(&mut self) -> MemctlAvgenW<MemctlSpec> {
        MemctlAvgenW::new(self, 16)
    }
    #[doc = "Bit 20 - Enable burn out current source."]
    #[inline(always)]
    pub fn memctl_bcsen(&mut self) -> MemctlBcsenW<MemctlSpec> {
        MemctlBcsenW::new(self, 20)
    }
    #[doc = "Bit 24 - Trigger policy. Indicates if a trigger will be needed to step to the next MEMCTL in the sequence or to perform next conversion in the case of repeat single channel conversions."]
    #[inline(always)]
    pub fn memctl_trig(&mut self) -> MemctlTrigW<MemctlSpec> {
        MemctlTrigW::new(self, 24)
    }
    #[doc = "Bit 28 - Enable window comparator."]
    #[inline(always)]
    pub fn memctl_wincomp(&mut self) -> MemctlWincompW<MemctlSpec> {
        MemctlWincompW::new(self, 28)
    }
}
#[doc = "Conversion Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`memctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemctlSpec;
impl crate::RegisterSpec for MemctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memctl::R`](R) reader structure"]
impl crate::Readable for MemctlSpec {}
#[doc = "`write(|w| ..)` method takes [`memctl::W`](W) writer structure"]
impl crate::Writable for MemctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEMCTL[%s]
to value 0"]
impl crate::Resettable for MemctlSpec {
    const RESET_VALUE: u32 = 0;
}
