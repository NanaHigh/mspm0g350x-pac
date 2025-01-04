#[doc = "Register `CTL` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `CTL` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "ULP_ADCHP Enable Conversions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CtlFunc {
    #[doc = "0: NOP"]
    CtlFuncNop = 0,
    #[doc = "1: SINCOS"]
    CtlFuncSincos = 1,
    #[doc = "2: ATAN2"]
    CtlFuncAtan2 = 2,
    #[doc = "4: DIV"]
    CtlFuncDiv = 4,
    #[doc = "5: SQRT"]
    CtlFuncSqrt = 5,
    #[doc = "6: MPY32"]
    CtlFuncMpy32 = 6,
    #[doc = "7: SQUARE32"]
    CtlFuncSquare32 = 7,
    #[doc = "8: MPY64"]
    CtlFuncMpy64 = 8,
    #[doc = "9: SQUARE64"]
    CtlFuncSquare64 = 9,
    #[doc = "10: MAC"]
    CtlFuncMac = 10,
    #[doc = "11: SAC"]
    CtlFuncSac = 11,
}
impl From<CtlFunc> for u8 {
    #[inline(always)]
    fn from(variant: CtlFunc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CtlFunc {
    type Ux = u8;
}
impl crate::IsEnum for CtlFunc {}
#[doc = "Field `CTL_FUNC` reader - ULP_ADCHP Enable Conversions."]
pub type CtlFuncR = crate::FieldReader<CtlFunc>;
impl CtlFuncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CtlFunc> {
        match self.bits {
            0 => Some(CtlFunc::CtlFuncNop),
            1 => Some(CtlFunc::CtlFuncSincos),
            2 => Some(CtlFunc::CtlFuncAtan2),
            4 => Some(CtlFunc::CtlFuncDiv),
            5 => Some(CtlFunc::CtlFuncSqrt),
            6 => Some(CtlFunc::CtlFuncMpy32),
            7 => Some(CtlFunc::CtlFuncSquare32),
            8 => Some(CtlFunc::CtlFuncMpy64),
            9 => Some(CtlFunc::CtlFuncSquare64),
            10 => Some(CtlFunc::CtlFuncMac),
            11 => Some(CtlFunc::CtlFuncSac),
            _ => None,
        }
    }
    #[doc = "NOP"]
    #[inline(always)]
    pub fn is_ctl_func_nop(&self) -> bool {
        *self == CtlFunc::CtlFuncNop
    }
    #[doc = "SINCOS"]
    #[inline(always)]
    pub fn is_ctl_func_sincos(&self) -> bool {
        *self == CtlFunc::CtlFuncSincos
    }
    #[doc = "ATAN2"]
    #[inline(always)]
    pub fn is_ctl_func_atan2(&self) -> bool {
        *self == CtlFunc::CtlFuncAtan2
    }
    #[doc = "DIV"]
    #[inline(always)]
    pub fn is_ctl_func_div(&self) -> bool {
        *self == CtlFunc::CtlFuncDiv
    }
    #[doc = "SQRT"]
    #[inline(always)]
    pub fn is_ctl_func_sqrt(&self) -> bool {
        *self == CtlFunc::CtlFuncSqrt
    }
    #[doc = "MPY32"]
    #[inline(always)]
    pub fn is_ctl_func_mpy32(&self) -> bool {
        *self == CtlFunc::CtlFuncMpy32
    }
    #[doc = "SQUARE32"]
    #[inline(always)]
    pub fn is_ctl_func_square32(&self) -> bool {
        *self == CtlFunc::CtlFuncSquare32
    }
    #[doc = "MPY64"]
    #[inline(always)]
    pub fn is_ctl_func_mpy64(&self) -> bool {
        *self == CtlFunc::CtlFuncMpy64
    }
    #[doc = "SQUARE64"]
    #[inline(always)]
    pub fn is_ctl_func_square64(&self) -> bool {
        *self == CtlFunc::CtlFuncSquare64
    }
    #[doc = "MAC"]
    #[inline(always)]
    pub fn is_ctl_func_mac(&self) -> bool {
        *self == CtlFunc::CtlFuncMac
    }
    #[doc = "SAC"]
    #[inline(always)]
    pub fn is_ctl_func_sac(&self) -> bool {
        *self == CtlFunc::CtlFuncSac
    }
}
#[doc = "Field `CTL_FUNC` writer - ULP_ADCHP Enable Conversions."]
pub type CtlFuncW<'a, REG> = crate::FieldWriter<'a, REG, 5, CtlFunc>;
impl<'a, REG> CtlFuncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NOP"]
    #[inline(always)]
    pub fn ctl_func_nop(self) -> &'a mut crate::W<REG> {
        self.variant(CtlFunc::CtlFuncNop)
    }
    #[doc = "SINCOS"]
    #[inline(always)]
    pub fn ctl_func_sincos(self) -> &'a mut crate::W<REG> {
        self.variant(CtlFunc::CtlFuncSincos)
    }
    #[doc = "ATAN2"]
    #[inline(always)]
    pub fn ctl_func_atan2(self) -> &'a mut crate::W<REG> {
        self.variant(CtlFunc::CtlFuncAtan2)
    }
    #[doc = "DIV"]
    #[inline(always)]
    pub fn ctl_func_div(self) -> &'a mut crate::W<REG> {
        self.variant(CtlFunc::CtlFuncDiv)
    }
    #[doc = "SQRT"]
    #[inline(always)]
    pub fn ctl_func_sqrt(self) -> &'a mut crate::W<REG> {
        self.variant(CtlFunc::CtlFuncSqrt)
    }
    #[doc = "MPY32"]
    #[inline(always)]
    pub fn ctl_func_mpy32(self) -> &'a mut crate::W<REG> {
        self.variant(CtlFunc::CtlFuncMpy32)
    }
    #[doc = "SQUARE32"]
    #[inline(always)]
    pub fn ctl_func_square32(self) -> &'a mut crate::W<REG> {
        self.variant(CtlFunc::CtlFuncSquare32)
    }
    #[doc = "MPY64"]
    #[inline(always)]
    pub fn ctl_func_mpy64(self) -> &'a mut crate::W<REG> {
        self.variant(CtlFunc::CtlFuncMpy64)
    }
    #[doc = "SQUARE64"]
    #[inline(always)]
    pub fn ctl_func_square64(self) -> &'a mut crate::W<REG> {
        self.variant(CtlFunc::CtlFuncSquare64)
    }
    #[doc = "MAC"]
    #[inline(always)]
    pub fn ctl_func_mac(self) -> &'a mut crate::W<REG> {
        self.variant(CtlFunc::CtlFuncMac)
    }
    #[doc = "SAC"]
    #[inline(always)]
    pub fn ctl_func_sac(self) -> &'a mut crate::W<REG> {
        self.variant(CtlFunc::CtlFuncSac)
    }
}
#[doc = "Operand type, could signed or unsigned. applicable to DIV function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtlOptype {
    #[doc = "0: UNSIGNED"]
    CtlOptypeUnsigned = 0,
    #[doc = "1: SIGNED"]
    CtlOptypeSigned = 1,
}
impl From<CtlOptype> for bool {
    #[inline(always)]
    fn from(variant: CtlOptype) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL_OPTYPE` reader - Operand type, could signed or unsigned. applicable to DIV function."]
pub type CtlOptypeR = crate::BitReader<CtlOptype>;
impl CtlOptypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtlOptype {
        match self.bits {
            false => CtlOptype::CtlOptypeUnsigned,
            true => CtlOptype::CtlOptypeSigned,
        }
    }
    #[doc = "UNSIGNED"]
    #[inline(always)]
    pub fn is_ctl_optype_unsigned(&self) -> bool {
        *self == CtlOptype::CtlOptypeUnsigned
    }
    #[doc = "SIGNED"]
    #[inline(always)]
    pub fn is_ctl_optype_signed(&self) -> bool {
        *self == CtlOptype::CtlOptypeSigned
    }
}
#[doc = "Field `CTL_OPTYPE` writer - Operand type, could signed or unsigned. applicable to DIV function."]
pub type CtlOptypeW<'a, REG> = crate::BitWriter<'a, REG, CtlOptype>;
impl<'a, REG> CtlOptypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "UNSIGNED"]
    #[inline(always)]
    pub fn ctl_optype_unsigned(self) -> &'a mut crate::W<REG> {
        self.variant(CtlOptype::CtlOptypeUnsigned)
    }
    #[doc = "SIGNED"]
    #[inline(always)]
    pub fn ctl_optype_signed(self) -> &'a mut crate::W<REG> {
        self.variant(CtlOptype::CtlOptypeSigned)
    }
}
#[doc = "Indicates the fractional bits in the operands, ranges from 0 to 31. Applicable to DIV function.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CtlQval {
    #[doc = "0: Q0"]
    CtlQvalQ0 = 0,
    #[doc = "1: Q1"]
    CtlQvalQ1 = 1,
    #[doc = "2: Q2"]
    CtlQvalQ2 = 2,
    #[doc = "3: Q3"]
    CtlQvalQ3 = 3,
    #[doc = "4: Q4"]
    CtlQvalQ4 = 4,
    #[doc = "5: Q5"]
    CtlQvalQ5 = 5,
    #[doc = "6: Q6"]
    CtlQvalQ6 = 6,
    #[doc = "7: Q7"]
    CtlQvalQ7 = 7,
    #[doc = "8: Q8"]
    CtlQvalQ8 = 8,
    #[doc = "9: Q9"]
    CtlQvalQ9 = 9,
    #[doc = "10: Q10"]
    CtlQvalQ10 = 10,
    #[doc = "11: Q11"]
    CtlQvalQ11 = 11,
    #[doc = "12: Q12"]
    CtlQvalQ12 = 12,
    #[doc = "13: Q13"]
    CtlQvalQ13 = 13,
    #[doc = "14: Q14"]
    CtlQvalQ14 = 14,
    #[doc = "15: Q15"]
    CtlQvalQ15 = 15,
    #[doc = "16: Q16"]
    CtlQvalQ16 = 16,
    #[doc = "17: Q17"]
    CtlQvalQ17 = 17,
    #[doc = "18: Q18"]
    CtlQvalQ18 = 18,
    #[doc = "19: Q19"]
    CtlQvalQ19 = 19,
    #[doc = "20: Q20"]
    CtlQvalQ20 = 20,
    #[doc = "21: Q21"]
    CtlQvalQ21 = 21,
    #[doc = "22: Q22"]
    CtlQvalQ22 = 22,
    #[doc = "23: Q23"]
    CtlQvalQ23 = 23,
    #[doc = "24: Q24"]
    CtlQvalQ24 = 24,
    #[doc = "25: Q25"]
    CtlQvalQ25 = 25,
    #[doc = "26: Q26"]
    CtlQvalQ26 = 26,
    #[doc = "27: Q27"]
    CtlQvalQ27 = 27,
    #[doc = "28: Q28"]
    CtlQvalQ28 = 28,
    #[doc = "29: Q29"]
    CtlQvalQ29 = 29,
    #[doc = "30: Q30"]
    CtlQvalQ30 = 30,
    #[doc = "31: Q31"]
    CtlQvalQ31 = 31,
}
impl From<CtlQval> for u8 {
    #[inline(always)]
    fn from(variant: CtlQval) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CtlQval {
    type Ux = u8;
}
impl crate::IsEnum for CtlQval {}
#[doc = "Field `CTL_QVAL` reader - Indicates the fractional bits in the operands, ranges from 0 to 31. Applicable to DIV function."]
pub type CtlQvalR = crate::FieldReader<CtlQval>;
impl CtlQvalR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtlQval {
        match self.bits {
            0 => CtlQval::CtlQvalQ0,
            1 => CtlQval::CtlQvalQ1,
            2 => CtlQval::CtlQvalQ2,
            3 => CtlQval::CtlQvalQ3,
            4 => CtlQval::CtlQvalQ4,
            5 => CtlQval::CtlQvalQ5,
            6 => CtlQval::CtlQvalQ6,
            7 => CtlQval::CtlQvalQ7,
            8 => CtlQval::CtlQvalQ8,
            9 => CtlQval::CtlQvalQ9,
            10 => CtlQval::CtlQvalQ10,
            11 => CtlQval::CtlQvalQ11,
            12 => CtlQval::CtlQvalQ12,
            13 => CtlQval::CtlQvalQ13,
            14 => CtlQval::CtlQvalQ14,
            15 => CtlQval::CtlQvalQ15,
            16 => CtlQval::CtlQvalQ16,
            17 => CtlQval::CtlQvalQ17,
            18 => CtlQval::CtlQvalQ18,
            19 => CtlQval::CtlQvalQ19,
            20 => CtlQval::CtlQvalQ20,
            21 => CtlQval::CtlQvalQ21,
            22 => CtlQval::CtlQvalQ22,
            23 => CtlQval::CtlQvalQ23,
            24 => CtlQval::CtlQvalQ24,
            25 => CtlQval::CtlQvalQ25,
            26 => CtlQval::CtlQvalQ26,
            27 => CtlQval::CtlQvalQ27,
            28 => CtlQval::CtlQvalQ28,
            29 => CtlQval::CtlQvalQ29,
            30 => CtlQval::CtlQvalQ30,
            31 => CtlQval::CtlQvalQ31,
            _ => unreachable!(),
        }
    }
    #[doc = "Q0"]
    #[inline(always)]
    pub fn is_ctl_qval_q0(&self) -> bool {
        *self == CtlQval::CtlQvalQ0
    }
    #[doc = "Q1"]
    #[inline(always)]
    pub fn is_ctl_qval_q1(&self) -> bool {
        *self == CtlQval::CtlQvalQ1
    }
    #[doc = "Q2"]
    #[inline(always)]
    pub fn is_ctl_qval_q2(&self) -> bool {
        *self == CtlQval::CtlQvalQ2
    }
    #[doc = "Q3"]
    #[inline(always)]
    pub fn is_ctl_qval_q3(&self) -> bool {
        *self == CtlQval::CtlQvalQ3
    }
    #[doc = "Q4"]
    #[inline(always)]
    pub fn is_ctl_qval_q4(&self) -> bool {
        *self == CtlQval::CtlQvalQ4
    }
    #[doc = "Q5"]
    #[inline(always)]
    pub fn is_ctl_qval_q5(&self) -> bool {
        *self == CtlQval::CtlQvalQ5
    }
    #[doc = "Q6"]
    #[inline(always)]
    pub fn is_ctl_qval_q6(&self) -> bool {
        *self == CtlQval::CtlQvalQ6
    }
    #[doc = "Q7"]
    #[inline(always)]
    pub fn is_ctl_qval_q7(&self) -> bool {
        *self == CtlQval::CtlQvalQ7
    }
    #[doc = "Q8"]
    #[inline(always)]
    pub fn is_ctl_qval_q8(&self) -> bool {
        *self == CtlQval::CtlQvalQ8
    }
    #[doc = "Q9"]
    #[inline(always)]
    pub fn is_ctl_qval_q9(&self) -> bool {
        *self == CtlQval::CtlQvalQ9
    }
    #[doc = "Q10"]
    #[inline(always)]
    pub fn is_ctl_qval_q10(&self) -> bool {
        *self == CtlQval::CtlQvalQ10
    }
    #[doc = "Q11"]
    #[inline(always)]
    pub fn is_ctl_qval_q11(&self) -> bool {
        *self == CtlQval::CtlQvalQ11
    }
    #[doc = "Q12"]
    #[inline(always)]
    pub fn is_ctl_qval_q12(&self) -> bool {
        *self == CtlQval::CtlQvalQ12
    }
    #[doc = "Q13"]
    #[inline(always)]
    pub fn is_ctl_qval_q13(&self) -> bool {
        *self == CtlQval::CtlQvalQ13
    }
    #[doc = "Q14"]
    #[inline(always)]
    pub fn is_ctl_qval_q14(&self) -> bool {
        *self == CtlQval::CtlQvalQ14
    }
    #[doc = "Q15"]
    #[inline(always)]
    pub fn is_ctl_qval_q15(&self) -> bool {
        *self == CtlQval::CtlQvalQ15
    }
    #[doc = "Q16"]
    #[inline(always)]
    pub fn is_ctl_qval_q16(&self) -> bool {
        *self == CtlQval::CtlQvalQ16
    }
    #[doc = "Q17"]
    #[inline(always)]
    pub fn is_ctl_qval_q17(&self) -> bool {
        *self == CtlQval::CtlQvalQ17
    }
    #[doc = "Q18"]
    #[inline(always)]
    pub fn is_ctl_qval_q18(&self) -> bool {
        *self == CtlQval::CtlQvalQ18
    }
    #[doc = "Q19"]
    #[inline(always)]
    pub fn is_ctl_qval_q19(&self) -> bool {
        *self == CtlQval::CtlQvalQ19
    }
    #[doc = "Q20"]
    #[inline(always)]
    pub fn is_ctl_qval_q20(&self) -> bool {
        *self == CtlQval::CtlQvalQ20
    }
    #[doc = "Q21"]
    #[inline(always)]
    pub fn is_ctl_qval_q21(&self) -> bool {
        *self == CtlQval::CtlQvalQ21
    }
    #[doc = "Q22"]
    #[inline(always)]
    pub fn is_ctl_qval_q22(&self) -> bool {
        *self == CtlQval::CtlQvalQ22
    }
    #[doc = "Q23"]
    #[inline(always)]
    pub fn is_ctl_qval_q23(&self) -> bool {
        *self == CtlQval::CtlQvalQ23
    }
    #[doc = "Q24"]
    #[inline(always)]
    pub fn is_ctl_qval_q24(&self) -> bool {
        *self == CtlQval::CtlQvalQ24
    }
    #[doc = "Q25"]
    #[inline(always)]
    pub fn is_ctl_qval_q25(&self) -> bool {
        *self == CtlQval::CtlQvalQ25
    }
    #[doc = "Q26"]
    #[inline(always)]
    pub fn is_ctl_qval_q26(&self) -> bool {
        *self == CtlQval::CtlQvalQ26
    }
    #[doc = "Q27"]
    #[inline(always)]
    pub fn is_ctl_qval_q27(&self) -> bool {
        *self == CtlQval::CtlQvalQ27
    }
    #[doc = "Q28"]
    #[inline(always)]
    pub fn is_ctl_qval_q28(&self) -> bool {
        *self == CtlQval::CtlQvalQ28
    }
    #[doc = "Q29"]
    #[inline(always)]
    pub fn is_ctl_qval_q29(&self) -> bool {
        *self == CtlQval::CtlQvalQ29
    }
    #[doc = "Q30"]
    #[inline(always)]
    pub fn is_ctl_qval_q30(&self) -> bool {
        *self == CtlQval::CtlQvalQ30
    }
    #[doc = "Q31"]
    #[inline(always)]
    pub fn is_ctl_qval_q31(&self) -> bool {
        *self == CtlQval::CtlQvalQ31
    }
}
#[doc = "Field `CTL_QVAL` writer - Indicates the fractional bits in the operands, ranges from 0 to 31. Applicable to DIV function."]
pub type CtlQvalW<'a, REG> = crate::FieldWriter<'a, REG, 5, CtlQval, crate::Safe>;
impl<'a, REG> CtlQvalW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Q0"]
    #[inline(always)]
    pub fn ctl_qval_q0(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ0)
    }
    #[doc = "Q1"]
    #[inline(always)]
    pub fn ctl_qval_q1(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ1)
    }
    #[doc = "Q2"]
    #[inline(always)]
    pub fn ctl_qval_q2(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ2)
    }
    #[doc = "Q3"]
    #[inline(always)]
    pub fn ctl_qval_q3(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ3)
    }
    #[doc = "Q4"]
    #[inline(always)]
    pub fn ctl_qval_q4(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ4)
    }
    #[doc = "Q5"]
    #[inline(always)]
    pub fn ctl_qval_q5(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ5)
    }
    #[doc = "Q6"]
    #[inline(always)]
    pub fn ctl_qval_q6(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ6)
    }
    #[doc = "Q7"]
    #[inline(always)]
    pub fn ctl_qval_q7(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ7)
    }
    #[doc = "Q8"]
    #[inline(always)]
    pub fn ctl_qval_q8(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ8)
    }
    #[doc = "Q9"]
    #[inline(always)]
    pub fn ctl_qval_q9(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ9)
    }
    #[doc = "Q10"]
    #[inline(always)]
    pub fn ctl_qval_q10(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ10)
    }
    #[doc = "Q11"]
    #[inline(always)]
    pub fn ctl_qval_q11(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ11)
    }
    #[doc = "Q12"]
    #[inline(always)]
    pub fn ctl_qval_q12(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ12)
    }
    #[doc = "Q13"]
    #[inline(always)]
    pub fn ctl_qval_q13(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ13)
    }
    #[doc = "Q14"]
    #[inline(always)]
    pub fn ctl_qval_q14(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ14)
    }
    #[doc = "Q15"]
    #[inline(always)]
    pub fn ctl_qval_q15(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ15)
    }
    #[doc = "Q16"]
    #[inline(always)]
    pub fn ctl_qval_q16(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ16)
    }
    #[doc = "Q17"]
    #[inline(always)]
    pub fn ctl_qval_q17(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ17)
    }
    #[doc = "Q18"]
    #[inline(always)]
    pub fn ctl_qval_q18(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ18)
    }
    #[doc = "Q19"]
    #[inline(always)]
    pub fn ctl_qval_q19(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ19)
    }
    #[doc = "Q20"]
    #[inline(always)]
    pub fn ctl_qval_q20(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ20)
    }
    #[doc = "Q21"]
    #[inline(always)]
    pub fn ctl_qval_q21(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ21)
    }
    #[doc = "Q22"]
    #[inline(always)]
    pub fn ctl_qval_q22(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ22)
    }
    #[doc = "Q23"]
    #[inline(always)]
    pub fn ctl_qval_q23(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ23)
    }
    #[doc = "Q24"]
    #[inline(always)]
    pub fn ctl_qval_q24(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ24)
    }
    #[doc = "Q25"]
    #[inline(always)]
    pub fn ctl_qval_q25(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ25)
    }
    #[doc = "Q26"]
    #[inline(always)]
    pub fn ctl_qval_q26(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ26)
    }
    #[doc = "Q27"]
    #[inline(always)]
    pub fn ctl_qval_q27(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ27)
    }
    #[doc = "Q28"]
    #[inline(always)]
    pub fn ctl_qval_q28(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ28)
    }
    #[doc = "Q29"]
    #[inline(always)]
    pub fn ctl_qval_q29(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ29)
    }
    #[doc = "Q30"]
    #[inline(always)]
    pub fn ctl_qval_q30(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ30)
    }
    #[doc = "Q31"]
    #[inline(always)]
    pub fn ctl_qval_q31(self) -> &'a mut crate::W<REG> {
        self.variant(CtlQval::CtlQvalQ31)
    }
}
#[doc = "Field `CTL_SFACTOR` reader - Scaling factor. In case of SQRT function, the input operand needs to be in a range. If not it has to be scaled to 2^(+/-n). This field should be written with the value 'n'."]
pub type CtlSfactorR = crate::FieldReader;
#[doc = "Field `CTL_SFACTOR` writer - Scaling factor. In case of SQRT function, the input operand needs to be in a range. If not it has to be scaled to 2^(+/-n). This field should be written with the value 'n'."]
pub type CtlSfactorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Saturation enable This bit is shared among DIV, SQUARE32, MPY32, MAC and SAC functions. When enabled, it will make the result to saturate to maximum value in case of an overflow event When disabled, the result will overflow to an unknown value.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtlSaten {
    #[doc = "0: DISABLE"]
    CtlSatenDisable = 0,
    #[doc = "1: ENABLE"]
    CtlSatenEnable = 1,
}
impl From<CtlSaten> for bool {
    #[inline(always)]
    fn from(variant: CtlSaten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTL_SATEN` reader - Saturation enable This bit is shared among DIV, SQUARE32, MPY32, MAC and SAC functions. When enabled, it will make the result to saturate to maximum value in case of an overflow event When disabled, the result will overflow to an unknown value."]
pub type CtlSatenR = crate::BitReader<CtlSaten>;
impl CtlSatenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtlSaten {
        match self.bits {
            false => CtlSaten::CtlSatenDisable,
            true => CtlSaten::CtlSatenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_ctl_saten_disable(&self) -> bool {
        *self == CtlSaten::CtlSatenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_ctl_saten_enable(&self) -> bool {
        *self == CtlSaten::CtlSatenEnable
    }
}
#[doc = "Field `CTL_SATEN` writer - Saturation enable This bit is shared among DIV, SQUARE32, MPY32, MAC and SAC functions. When enabled, it will make the result to saturate to maximum value in case of an overflow event When disabled, the result will overflow to an unknown value."]
pub type CtlSatenW<'a, REG> = crate::BitWriter<'a, REG, CtlSaten>;
impl<'a, REG> CtlSatenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn ctl_saten_disable(self) -> &'a mut crate::W<REG> {
        self.variant(CtlSaten::CtlSatenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn ctl_saten_enable(self) -> &'a mut crate::W<REG> {
        self.variant(CtlSaten::CtlSatenEnable)
    }
}
#[doc = "Field `CTL_NUMITER` reader - Number of iterations, applicable if the function does the computations iteratively, for example sine/cosine/atan2/sqrt. Note: A value of 0 is interpreted as 31."]
pub type CtlNumiterR = crate::FieldReader;
#[doc = "Field `CTL_NUMITER` writer - Number of iterations, applicable if the function does the computations iteratively, for example sine/cosine/atan2/sqrt. Note: A value of 0 is interpreted as 31."]
pub type CtlNumiterW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - ULP_ADCHP Enable Conversions."]
    #[inline(always)]
    pub fn ctl_func(&self) -> CtlFuncR {
        CtlFuncR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Operand type, could signed or unsigned. applicable to DIV function."]
    #[inline(always)]
    pub fn ctl_optype(&self) -> CtlOptypeR {
        CtlOptypeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Indicates the fractional bits in the operands, ranges from 0 to 31. Applicable to DIV function."]
    #[inline(always)]
    pub fn ctl_qval(&self) -> CtlQvalR {
        CtlQvalR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:21 - Scaling factor. In case of SQRT function, the input operand needs to be in a range. If not it has to be scaled to 2^(+/-n). This field should be written with the value 'n'."]
    #[inline(always)]
    pub fn ctl_sfactor(&self) -> CtlSfactorR {
        CtlSfactorR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Saturation enable This bit is shared among DIV, SQUARE32, MPY32, MAC and SAC functions. When enabled, it will make the result to saturate to maximum value in case of an overflow event When disabled, the result will overflow to an unknown value."]
    #[inline(always)]
    pub fn ctl_saten(&self) -> CtlSatenR {
        CtlSatenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:28 - Number of iterations, applicable if the function does the computations iteratively, for example sine/cosine/atan2/sqrt. Note: A value of 0 is interpreted as 31."]
    #[inline(always)]
    pub fn ctl_numiter(&self) -> CtlNumiterR {
        CtlNumiterR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - ULP_ADCHP Enable Conversions."]
    #[inline(always)]
    pub fn ctl_func(&mut self) -> CtlFuncW<CtlSpec> {
        CtlFuncW::new(self, 0)
    }
    #[doc = "Bit 5 - Operand type, could signed or unsigned. applicable to DIV function."]
    #[inline(always)]
    pub fn ctl_optype(&mut self) -> CtlOptypeW<CtlSpec> {
        CtlOptypeW::new(self, 5)
    }
    #[doc = "Bits 8:12 - Indicates the fractional bits in the operands, ranges from 0 to 31. Applicable to DIV function."]
    #[inline(always)]
    pub fn ctl_qval(&mut self) -> CtlQvalW<CtlSpec> {
        CtlQvalW::new(self, 8)
    }
    #[doc = "Bits 16:21 - Scaling factor. In case of SQRT function, the input operand needs to be in a range. If not it has to be scaled to 2^(+/-n). This field should be written with the value 'n'."]
    #[inline(always)]
    pub fn ctl_sfactor(&mut self) -> CtlSfactorW<CtlSpec> {
        CtlSfactorW::new(self, 16)
    }
    #[doc = "Bit 22 - Saturation enable This bit is shared among DIV, SQUARE32, MPY32, MAC and SAC functions. When enabled, it will make the result to saturate to maximum value in case of an overflow event When disabled, the result will overflow to an unknown value."]
    #[inline(always)]
    pub fn ctl_saten(&mut self) -> CtlSatenW<CtlSpec> {
        CtlSatenW::new(self, 22)
    }
    #[doc = "Bits 24:28 - Number of iterations, applicable if the function does the computations iteratively, for example sine/cosine/atan2/sqrt. Note: A value of 0 is interpreted as 31."]
    #[inline(always)]
    pub fn ctl_numiter(&mut self) -> CtlNumiterW<CtlSpec> {
        CtlNumiterW::new(self, 24)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u32 = 0;
}
