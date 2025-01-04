#[doc = "Register `DMAMASK` reader"]
pub type R = crate::R<DmamaskSpec>;
#[doc = "Register `DMAMASK` writer"]
pub type W = crate::W<DmamaskSpec>;
#[doc = "DMA is allowed to modify DOUT0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout0 {
    #[doc = "0: DISABLE"]
    DmamaskDout0Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout0Enable = 1,
}
impl From<DmamaskDout0> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT0` reader - DMA is allowed to modify DOUT0"]
pub type DmamaskDout0R = crate::BitReader<DmamaskDout0>;
impl DmamaskDout0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout0 {
        match self.bits {
            false => DmamaskDout0::DmamaskDout0Disable,
            true => DmamaskDout0::DmamaskDout0Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout0_disable(&self) -> bool {
        *self == DmamaskDout0::DmamaskDout0Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout0_enable(&self) -> bool {
        *self == DmamaskDout0::DmamaskDout0Enable
    }
}
#[doc = "Field `DMAMASK_DOUT0` writer - DMA is allowed to modify DOUT0"]
pub type DmamaskDout0W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout0>;
impl<'a, REG> DmamaskDout0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout0_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout0::DmamaskDout0Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout0_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout0::DmamaskDout0Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout1 {
    #[doc = "0: DISABLE"]
    DmamaskDout1Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout1Enable = 1,
}
impl From<DmamaskDout1> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT1` reader - DMA is allowed to modify DOUT1"]
pub type DmamaskDout1R = crate::BitReader<DmamaskDout1>;
impl DmamaskDout1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout1 {
        match self.bits {
            false => DmamaskDout1::DmamaskDout1Disable,
            true => DmamaskDout1::DmamaskDout1Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout1_disable(&self) -> bool {
        *self == DmamaskDout1::DmamaskDout1Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout1_enable(&self) -> bool {
        *self == DmamaskDout1::DmamaskDout1Enable
    }
}
#[doc = "Field `DMAMASK_DOUT1` writer - DMA is allowed to modify DOUT1"]
pub type DmamaskDout1W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout1>;
impl<'a, REG> DmamaskDout1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout1_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout1::DmamaskDout1Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout1_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout1::DmamaskDout1Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout2 {
    #[doc = "0: DISABLE"]
    DmamaskDout2Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout2Enable = 1,
}
impl From<DmamaskDout2> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT2` reader - DMA is allowed to modify DOUT2"]
pub type DmamaskDout2R = crate::BitReader<DmamaskDout2>;
impl DmamaskDout2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout2 {
        match self.bits {
            false => DmamaskDout2::DmamaskDout2Disable,
            true => DmamaskDout2::DmamaskDout2Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout2_disable(&self) -> bool {
        *self == DmamaskDout2::DmamaskDout2Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout2_enable(&self) -> bool {
        *self == DmamaskDout2::DmamaskDout2Enable
    }
}
#[doc = "Field `DMAMASK_DOUT2` writer - DMA is allowed to modify DOUT2"]
pub type DmamaskDout2W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout2>;
impl<'a, REG> DmamaskDout2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout2_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout2::DmamaskDout2Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout2_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout2::DmamaskDout2Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout3 {
    #[doc = "0: DISABLE"]
    DmamaskDout3Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout3Enable = 1,
}
impl From<DmamaskDout3> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT3` reader - DMA is allowed to modify DOUT3"]
pub type DmamaskDout3R = crate::BitReader<DmamaskDout3>;
impl DmamaskDout3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout3 {
        match self.bits {
            false => DmamaskDout3::DmamaskDout3Disable,
            true => DmamaskDout3::DmamaskDout3Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout3_disable(&self) -> bool {
        *self == DmamaskDout3::DmamaskDout3Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout3_enable(&self) -> bool {
        *self == DmamaskDout3::DmamaskDout3Enable
    }
}
#[doc = "Field `DMAMASK_DOUT3` writer - DMA is allowed to modify DOUT3"]
pub type DmamaskDout3W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout3>;
impl<'a, REG> DmamaskDout3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout3_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout3::DmamaskDout3Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout3_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout3::DmamaskDout3Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout4 {
    #[doc = "0: DISABLE"]
    DmamaskDout4Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout4Enable = 1,
}
impl From<DmamaskDout4> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT4` reader - DMA is allowed to modify DOUT4"]
pub type DmamaskDout4R = crate::BitReader<DmamaskDout4>;
impl DmamaskDout4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout4 {
        match self.bits {
            false => DmamaskDout4::DmamaskDout4Disable,
            true => DmamaskDout4::DmamaskDout4Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout4_disable(&self) -> bool {
        *self == DmamaskDout4::DmamaskDout4Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout4_enable(&self) -> bool {
        *self == DmamaskDout4::DmamaskDout4Enable
    }
}
#[doc = "Field `DMAMASK_DOUT4` writer - DMA is allowed to modify DOUT4"]
pub type DmamaskDout4W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout4>;
impl<'a, REG> DmamaskDout4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout4_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout4::DmamaskDout4Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout4_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout4::DmamaskDout4Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout5 {
    #[doc = "0: DISABLE"]
    DmamaskDout5Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout5Enable = 1,
}
impl From<DmamaskDout5> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT5` reader - DMA is allowed to modify DOUT5"]
pub type DmamaskDout5R = crate::BitReader<DmamaskDout5>;
impl DmamaskDout5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout5 {
        match self.bits {
            false => DmamaskDout5::DmamaskDout5Disable,
            true => DmamaskDout5::DmamaskDout5Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout5_disable(&self) -> bool {
        *self == DmamaskDout5::DmamaskDout5Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout5_enable(&self) -> bool {
        *self == DmamaskDout5::DmamaskDout5Enable
    }
}
#[doc = "Field `DMAMASK_DOUT5` writer - DMA is allowed to modify DOUT5"]
pub type DmamaskDout5W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout5>;
impl<'a, REG> DmamaskDout5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout5_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout5::DmamaskDout5Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout5_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout5::DmamaskDout5Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout6 {
    #[doc = "0: DISABLE"]
    DmamaskDout6Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout6Enable = 1,
}
impl From<DmamaskDout6> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT6` reader - DMA is allowed to modify DOUT6"]
pub type DmamaskDout6R = crate::BitReader<DmamaskDout6>;
impl DmamaskDout6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout6 {
        match self.bits {
            false => DmamaskDout6::DmamaskDout6Disable,
            true => DmamaskDout6::DmamaskDout6Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout6_disable(&self) -> bool {
        *self == DmamaskDout6::DmamaskDout6Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout6_enable(&self) -> bool {
        *self == DmamaskDout6::DmamaskDout6Enable
    }
}
#[doc = "Field `DMAMASK_DOUT6` writer - DMA is allowed to modify DOUT6"]
pub type DmamaskDout6W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout6>;
impl<'a, REG> DmamaskDout6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout6_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout6::DmamaskDout6Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout6_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout6::DmamaskDout6Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout7 {
    #[doc = "0: DISABLE"]
    DmamaskDout7Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout7Enable = 1,
}
impl From<DmamaskDout7> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT7` reader - DMA is allowed to modify DOUT7"]
pub type DmamaskDout7R = crate::BitReader<DmamaskDout7>;
impl DmamaskDout7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout7 {
        match self.bits {
            false => DmamaskDout7::DmamaskDout7Disable,
            true => DmamaskDout7::DmamaskDout7Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout7_disable(&self) -> bool {
        *self == DmamaskDout7::DmamaskDout7Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout7_enable(&self) -> bool {
        *self == DmamaskDout7::DmamaskDout7Enable
    }
}
#[doc = "Field `DMAMASK_DOUT7` writer - DMA is allowed to modify DOUT7"]
pub type DmamaskDout7W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout7>;
impl<'a, REG> DmamaskDout7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout7_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout7::DmamaskDout7Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout7_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout7::DmamaskDout7Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout8 {
    #[doc = "0: DISABLE"]
    DmamaskDout8Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout8Enable = 1,
}
impl From<DmamaskDout8> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT8` reader - DMA is allowed to modify DOUT8"]
pub type DmamaskDout8R = crate::BitReader<DmamaskDout8>;
impl DmamaskDout8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout8 {
        match self.bits {
            false => DmamaskDout8::DmamaskDout8Disable,
            true => DmamaskDout8::DmamaskDout8Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout8_disable(&self) -> bool {
        *self == DmamaskDout8::DmamaskDout8Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout8_enable(&self) -> bool {
        *self == DmamaskDout8::DmamaskDout8Enable
    }
}
#[doc = "Field `DMAMASK_DOUT8` writer - DMA is allowed to modify DOUT8"]
pub type DmamaskDout8W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout8>;
impl<'a, REG> DmamaskDout8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout8_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout8::DmamaskDout8Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout8_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout8::DmamaskDout8Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout9 {
    #[doc = "0: DISABLE"]
    DmamaskDout9Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout9Enable = 1,
}
impl From<DmamaskDout9> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT9` reader - DMA is allowed to modify DOUT9"]
pub type DmamaskDout9R = crate::BitReader<DmamaskDout9>;
impl DmamaskDout9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout9 {
        match self.bits {
            false => DmamaskDout9::DmamaskDout9Disable,
            true => DmamaskDout9::DmamaskDout9Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout9_disable(&self) -> bool {
        *self == DmamaskDout9::DmamaskDout9Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout9_enable(&self) -> bool {
        *self == DmamaskDout9::DmamaskDout9Enable
    }
}
#[doc = "Field `DMAMASK_DOUT9` writer - DMA is allowed to modify DOUT9"]
pub type DmamaskDout9W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout9>;
impl<'a, REG> DmamaskDout9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout9_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout9::DmamaskDout9Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout9_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout9::DmamaskDout9Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout10 {
    #[doc = "0: DISABLE"]
    DmamaskDout10Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout10Enable = 1,
}
impl From<DmamaskDout10> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT10` reader - DMA is allowed to modify DOUT10"]
pub type DmamaskDout10R = crate::BitReader<DmamaskDout10>;
impl DmamaskDout10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout10 {
        match self.bits {
            false => DmamaskDout10::DmamaskDout10Disable,
            true => DmamaskDout10::DmamaskDout10Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout10_disable(&self) -> bool {
        *self == DmamaskDout10::DmamaskDout10Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout10_enable(&self) -> bool {
        *self == DmamaskDout10::DmamaskDout10Enable
    }
}
#[doc = "Field `DMAMASK_DOUT10` writer - DMA is allowed to modify DOUT10"]
pub type DmamaskDout10W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout10>;
impl<'a, REG> DmamaskDout10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout10_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout10::DmamaskDout10Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout10_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout10::DmamaskDout10Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout11 {
    #[doc = "0: DISABLE"]
    DmamaskDout11Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout11Enable = 1,
}
impl From<DmamaskDout11> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT11` reader - DMA is allowed to modify DOUT11"]
pub type DmamaskDout11R = crate::BitReader<DmamaskDout11>;
impl DmamaskDout11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout11 {
        match self.bits {
            false => DmamaskDout11::DmamaskDout11Disable,
            true => DmamaskDout11::DmamaskDout11Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout11_disable(&self) -> bool {
        *self == DmamaskDout11::DmamaskDout11Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout11_enable(&self) -> bool {
        *self == DmamaskDout11::DmamaskDout11Enable
    }
}
#[doc = "Field `DMAMASK_DOUT11` writer - DMA is allowed to modify DOUT11"]
pub type DmamaskDout11W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout11>;
impl<'a, REG> DmamaskDout11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout11_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout11::DmamaskDout11Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout11_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout11::DmamaskDout11Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout12 {
    #[doc = "0: DISABLE"]
    DmamaskDout12Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout12Enable = 1,
}
impl From<DmamaskDout12> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT12` reader - DMA is allowed to modify DOUT12"]
pub type DmamaskDout12R = crate::BitReader<DmamaskDout12>;
impl DmamaskDout12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout12 {
        match self.bits {
            false => DmamaskDout12::DmamaskDout12Disable,
            true => DmamaskDout12::DmamaskDout12Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout12_disable(&self) -> bool {
        *self == DmamaskDout12::DmamaskDout12Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout12_enable(&self) -> bool {
        *self == DmamaskDout12::DmamaskDout12Enable
    }
}
#[doc = "Field `DMAMASK_DOUT12` writer - DMA is allowed to modify DOUT12"]
pub type DmamaskDout12W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout12>;
impl<'a, REG> DmamaskDout12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout12_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout12::DmamaskDout12Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout12_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout12::DmamaskDout12Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout13 {
    #[doc = "0: DISABLE"]
    DmamaskDout13Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout13Enable = 1,
}
impl From<DmamaskDout13> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT13` reader - DMA is allowed to modify DOUT13"]
pub type DmamaskDout13R = crate::BitReader<DmamaskDout13>;
impl DmamaskDout13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout13 {
        match self.bits {
            false => DmamaskDout13::DmamaskDout13Disable,
            true => DmamaskDout13::DmamaskDout13Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout13_disable(&self) -> bool {
        *self == DmamaskDout13::DmamaskDout13Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout13_enable(&self) -> bool {
        *self == DmamaskDout13::DmamaskDout13Enable
    }
}
#[doc = "Field `DMAMASK_DOUT13` writer - DMA is allowed to modify DOUT13"]
pub type DmamaskDout13W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout13>;
impl<'a, REG> DmamaskDout13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout13_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout13::DmamaskDout13Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout13_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout13::DmamaskDout13Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout14 {
    #[doc = "0: DISABLE"]
    DmamaskDout14Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout14Enable = 1,
}
impl From<DmamaskDout14> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT14` reader - DMA is allowed to modify DOUT14"]
pub type DmamaskDout14R = crate::BitReader<DmamaskDout14>;
impl DmamaskDout14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout14 {
        match self.bits {
            false => DmamaskDout14::DmamaskDout14Disable,
            true => DmamaskDout14::DmamaskDout14Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout14_disable(&self) -> bool {
        *self == DmamaskDout14::DmamaskDout14Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout14_enable(&self) -> bool {
        *self == DmamaskDout14::DmamaskDout14Enable
    }
}
#[doc = "Field `DMAMASK_DOUT14` writer - DMA is allowed to modify DOUT14"]
pub type DmamaskDout14W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout14>;
impl<'a, REG> DmamaskDout14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout14_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout14::DmamaskDout14Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout14_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout14::DmamaskDout14Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout15 {
    #[doc = "0: DISABLE"]
    DmamaskDout15Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout15Enable = 1,
}
impl From<DmamaskDout15> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT15` reader - DMA is allowed to modify DOUT15"]
pub type DmamaskDout15R = crate::BitReader<DmamaskDout15>;
impl DmamaskDout15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout15 {
        match self.bits {
            false => DmamaskDout15::DmamaskDout15Disable,
            true => DmamaskDout15::DmamaskDout15Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout15_disable(&self) -> bool {
        *self == DmamaskDout15::DmamaskDout15Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout15_enable(&self) -> bool {
        *self == DmamaskDout15::DmamaskDout15Enable
    }
}
#[doc = "Field `DMAMASK_DOUT15` writer - DMA is allowed to modify DOUT15"]
pub type DmamaskDout15W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout15>;
impl<'a, REG> DmamaskDout15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout15_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout15::DmamaskDout15Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout15_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout15::DmamaskDout15Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout16 {
    #[doc = "0: DISABLE"]
    DmamaskDout16Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout16Enable = 1,
}
impl From<DmamaskDout16> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout16) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT16` reader - DMA is allowed to modify DOUT16"]
pub type DmamaskDout16R = crate::BitReader<DmamaskDout16>;
impl DmamaskDout16R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout16 {
        match self.bits {
            false => DmamaskDout16::DmamaskDout16Disable,
            true => DmamaskDout16::DmamaskDout16Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout16_disable(&self) -> bool {
        *self == DmamaskDout16::DmamaskDout16Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout16_enable(&self) -> bool {
        *self == DmamaskDout16::DmamaskDout16Enable
    }
}
#[doc = "Field `DMAMASK_DOUT16` writer - DMA is allowed to modify DOUT16"]
pub type DmamaskDout16W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout16>;
impl<'a, REG> DmamaskDout16W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout16_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout16::DmamaskDout16Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout16_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout16::DmamaskDout16Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout17 {
    #[doc = "0: DISABLE"]
    DmamaskDout17Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout17Enable = 1,
}
impl From<DmamaskDout17> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout17) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT17` reader - DMA is allowed to modify DOUT17"]
pub type DmamaskDout17R = crate::BitReader<DmamaskDout17>;
impl DmamaskDout17R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout17 {
        match self.bits {
            false => DmamaskDout17::DmamaskDout17Disable,
            true => DmamaskDout17::DmamaskDout17Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout17_disable(&self) -> bool {
        *self == DmamaskDout17::DmamaskDout17Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout17_enable(&self) -> bool {
        *self == DmamaskDout17::DmamaskDout17Enable
    }
}
#[doc = "Field `DMAMASK_DOUT17` writer - DMA is allowed to modify DOUT17"]
pub type DmamaskDout17W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout17>;
impl<'a, REG> DmamaskDout17W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout17_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout17::DmamaskDout17Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout17_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout17::DmamaskDout17Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout18 {
    #[doc = "0: DISABLE"]
    DmamaskDout18Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout18Enable = 1,
}
impl From<DmamaskDout18> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout18) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT18` reader - DMA is allowed to modify DOUT18"]
pub type DmamaskDout18R = crate::BitReader<DmamaskDout18>;
impl DmamaskDout18R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout18 {
        match self.bits {
            false => DmamaskDout18::DmamaskDout18Disable,
            true => DmamaskDout18::DmamaskDout18Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout18_disable(&self) -> bool {
        *self == DmamaskDout18::DmamaskDout18Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout18_enable(&self) -> bool {
        *self == DmamaskDout18::DmamaskDout18Enable
    }
}
#[doc = "Field `DMAMASK_DOUT18` writer - DMA is allowed to modify DOUT18"]
pub type DmamaskDout18W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout18>;
impl<'a, REG> DmamaskDout18W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout18_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout18::DmamaskDout18Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout18_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout18::DmamaskDout18Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout19 {
    #[doc = "0: DISABLE"]
    DmamaskDout19Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout19Enable = 1,
}
impl From<DmamaskDout19> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout19) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT19` reader - DMA is allowed to modify DOUT19"]
pub type DmamaskDout19R = crate::BitReader<DmamaskDout19>;
impl DmamaskDout19R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout19 {
        match self.bits {
            false => DmamaskDout19::DmamaskDout19Disable,
            true => DmamaskDout19::DmamaskDout19Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout19_disable(&self) -> bool {
        *self == DmamaskDout19::DmamaskDout19Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout19_enable(&self) -> bool {
        *self == DmamaskDout19::DmamaskDout19Enable
    }
}
#[doc = "Field `DMAMASK_DOUT19` writer - DMA is allowed to modify DOUT19"]
pub type DmamaskDout19W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout19>;
impl<'a, REG> DmamaskDout19W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout19_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout19::DmamaskDout19Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout19_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout19::DmamaskDout19Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout20 {
    #[doc = "0: DISABLE"]
    DmamaskDout20Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout20Enable = 1,
}
impl From<DmamaskDout20> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout20) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT20` reader - DMA is allowed to modify DOUT20"]
pub type DmamaskDout20R = crate::BitReader<DmamaskDout20>;
impl DmamaskDout20R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout20 {
        match self.bits {
            false => DmamaskDout20::DmamaskDout20Disable,
            true => DmamaskDout20::DmamaskDout20Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout20_disable(&self) -> bool {
        *self == DmamaskDout20::DmamaskDout20Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout20_enable(&self) -> bool {
        *self == DmamaskDout20::DmamaskDout20Enable
    }
}
#[doc = "Field `DMAMASK_DOUT20` writer - DMA is allowed to modify DOUT20"]
pub type DmamaskDout20W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout20>;
impl<'a, REG> DmamaskDout20W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout20_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout20::DmamaskDout20Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout20_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout20::DmamaskDout20Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout21 {
    #[doc = "0: DISABLE"]
    DmamaskDout21Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout21Enable = 1,
}
impl From<DmamaskDout21> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout21) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT21` reader - DMA is allowed to modify DOUT21"]
pub type DmamaskDout21R = crate::BitReader<DmamaskDout21>;
impl DmamaskDout21R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout21 {
        match self.bits {
            false => DmamaskDout21::DmamaskDout21Disable,
            true => DmamaskDout21::DmamaskDout21Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout21_disable(&self) -> bool {
        *self == DmamaskDout21::DmamaskDout21Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout21_enable(&self) -> bool {
        *self == DmamaskDout21::DmamaskDout21Enable
    }
}
#[doc = "Field `DMAMASK_DOUT21` writer - DMA is allowed to modify DOUT21"]
pub type DmamaskDout21W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout21>;
impl<'a, REG> DmamaskDout21W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout21_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout21::DmamaskDout21Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout21_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout21::DmamaskDout21Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout22 {
    #[doc = "0: DISABLE"]
    DmamaskDout22Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout22Enable = 1,
}
impl From<DmamaskDout22> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout22) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT22` reader - DMA is allowed to modify DOUT22"]
pub type DmamaskDout22R = crate::BitReader<DmamaskDout22>;
impl DmamaskDout22R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout22 {
        match self.bits {
            false => DmamaskDout22::DmamaskDout22Disable,
            true => DmamaskDout22::DmamaskDout22Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout22_disable(&self) -> bool {
        *self == DmamaskDout22::DmamaskDout22Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout22_enable(&self) -> bool {
        *self == DmamaskDout22::DmamaskDout22Enable
    }
}
#[doc = "Field `DMAMASK_DOUT22` writer - DMA is allowed to modify DOUT22"]
pub type DmamaskDout22W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout22>;
impl<'a, REG> DmamaskDout22W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout22_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout22::DmamaskDout22Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout22_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout22::DmamaskDout22Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout23 {
    #[doc = "0: DISABLE"]
    DmamaskDout23Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout23Enable = 1,
}
impl From<DmamaskDout23> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout23) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT23` reader - DMA is allowed to modify DOUT23"]
pub type DmamaskDout23R = crate::BitReader<DmamaskDout23>;
impl DmamaskDout23R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout23 {
        match self.bits {
            false => DmamaskDout23::DmamaskDout23Disable,
            true => DmamaskDout23::DmamaskDout23Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout23_disable(&self) -> bool {
        *self == DmamaskDout23::DmamaskDout23Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout23_enable(&self) -> bool {
        *self == DmamaskDout23::DmamaskDout23Enable
    }
}
#[doc = "Field `DMAMASK_DOUT23` writer - DMA is allowed to modify DOUT23"]
pub type DmamaskDout23W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout23>;
impl<'a, REG> DmamaskDout23W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout23_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout23::DmamaskDout23Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout23_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout23::DmamaskDout23Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout24 {
    #[doc = "0: DISABLE"]
    DmamaskDout24Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout24Enable = 1,
}
impl From<DmamaskDout24> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout24) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT24` reader - DMA is allowed to modify DOUT24"]
pub type DmamaskDout24R = crate::BitReader<DmamaskDout24>;
impl DmamaskDout24R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout24 {
        match self.bits {
            false => DmamaskDout24::DmamaskDout24Disable,
            true => DmamaskDout24::DmamaskDout24Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout24_disable(&self) -> bool {
        *self == DmamaskDout24::DmamaskDout24Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout24_enable(&self) -> bool {
        *self == DmamaskDout24::DmamaskDout24Enable
    }
}
#[doc = "Field `DMAMASK_DOUT24` writer - DMA is allowed to modify DOUT24"]
pub type DmamaskDout24W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout24>;
impl<'a, REG> DmamaskDout24W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout24_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout24::DmamaskDout24Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout24_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout24::DmamaskDout24Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout25 {
    #[doc = "0: DISABLE"]
    DmamaskDout25Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout25Enable = 1,
}
impl From<DmamaskDout25> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout25) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT25` reader - DMA is allowed to modify DOUT25"]
pub type DmamaskDout25R = crate::BitReader<DmamaskDout25>;
impl DmamaskDout25R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout25 {
        match self.bits {
            false => DmamaskDout25::DmamaskDout25Disable,
            true => DmamaskDout25::DmamaskDout25Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout25_disable(&self) -> bool {
        *self == DmamaskDout25::DmamaskDout25Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout25_enable(&self) -> bool {
        *self == DmamaskDout25::DmamaskDout25Enable
    }
}
#[doc = "Field `DMAMASK_DOUT25` writer - DMA is allowed to modify DOUT25"]
pub type DmamaskDout25W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout25>;
impl<'a, REG> DmamaskDout25W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout25_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout25::DmamaskDout25Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout25_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout25::DmamaskDout25Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout26 {
    #[doc = "0: DISABLE"]
    DmamaskDout26Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout26Enable = 1,
}
impl From<DmamaskDout26> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout26) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT26` reader - DMA is allowed to modify DOUT26"]
pub type DmamaskDout26R = crate::BitReader<DmamaskDout26>;
impl DmamaskDout26R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout26 {
        match self.bits {
            false => DmamaskDout26::DmamaskDout26Disable,
            true => DmamaskDout26::DmamaskDout26Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout26_disable(&self) -> bool {
        *self == DmamaskDout26::DmamaskDout26Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout26_enable(&self) -> bool {
        *self == DmamaskDout26::DmamaskDout26Enable
    }
}
#[doc = "Field `DMAMASK_DOUT26` writer - DMA is allowed to modify DOUT26"]
pub type DmamaskDout26W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout26>;
impl<'a, REG> DmamaskDout26W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout26_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout26::DmamaskDout26Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout26_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout26::DmamaskDout26Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout27 {
    #[doc = "0: DISABLE"]
    DmamaskDout27Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout27Enable = 1,
}
impl From<DmamaskDout27> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout27) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT27` reader - DMA is allowed to modify DOUT27"]
pub type DmamaskDout27R = crate::BitReader<DmamaskDout27>;
impl DmamaskDout27R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout27 {
        match self.bits {
            false => DmamaskDout27::DmamaskDout27Disable,
            true => DmamaskDout27::DmamaskDout27Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout27_disable(&self) -> bool {
        *self == DmamaskDout27::DmamaskDout27Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout27_enable(&self) -> bool {
        *self == DmamaskDout27::DmamaskDout27Enable
    }
}
#[doc = "Field `DMAMASK_DOUT27` writer - DMA is allowed to modify DOUT27"]
pub type DmamaskDout27W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout27>;
impl<'a, REG> DmamaskDout27W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout27_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout27::DmamaskDout27Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout27_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout27::DmamaskDout27Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout28 {
    #[doc = "0: DISABLE"]
    DmamaskDout28Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout28Enable = 1,
}
impl From<DmamaskDout28> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout28) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT28` reader - DMA is allowed to modify DOUT28"]
pub type DmamaskDout28R = crate::BitReader<DmamaskDout28>;
impl DmamaskDout28R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout28 {
        match self.bits {
            false => DmamaskDout28::DmamaskDout28Disable,
            true => DmamaskDout28::DmamaskDout28Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout28_disable(&self) -> bool {
        *self == DmamaskDout28::DmamaskDout28Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout28_enable(&self) -> bool {
        *self == DmamaskDout28::DmamaskDout28Enable
    }
}
#[doc = "Field `DMAMASK_DOUT28` writer - DMA is allowed to modify DOUT28"]
pub type DmamaskDout28W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout28>;
impl<'a, REG> DmamaskDout28W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout28_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout28::DmamaskDout28Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout28_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout28::DmamaskDout28Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout29 {
    #[doc = "0: DISABLE"]
    DmamaskDout29Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout29Enable = 1,
}
impl From<DmamaskDout29> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout29) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT29` reader - DMA is allowed to modify DOUT29"]
pub type DmamaskDout29R = crate::BitReader<DmamaskDout29>;
impl DmamaskDout29R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout29 {
        match self.bits {
            false => DmamaskDout29::DmamaskDout29Disable,
            true => DmamaskDout29::DmamaskDout29Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout29_disable(&self) -> bool {
        *self == DmamaskDout29::DmamaskDout29Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout29_enable(&self) -> bool {
        *self == DmamaskDout29::DmamaskDout29Enable
    }
}
#[doc = "Field `DMAMASK_DOUT29` writer - DMA is allowed to modify DOUT29"]
pub type DmamaskDout29W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout29>;
impl<'a, REG> DmamaskDout29W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout29_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout29::DmamaskDout29Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout29_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout29::DmamaskDout29Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout30 {
    #[doc = "0: DISABLE"]
    DmamaskDout30Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout30Enable = 1,
}
impl From<DmamaskDout30> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout30) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT30` reader - DMA is allowed to modify DOUT30"]
pub type DmamaskDout30R = crate::BitReader<DmamaskDout30>;
impl DmamaskDout30R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout30 {
        match self.bits {
            false => DmamaskDout30::DmamaskDout30Disable,
            true => DmamaskDout30::DmamaskDout30Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout30_disable(&self) -> bool {
        *self == DmamaskDout30::DmamaskDout30Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout30_enable(&self) -> bool {
        *self == DmamaskDout30::DmamaskDout30Enable
    }
}
#[doc = "Field `DMAMASK_DOUT30` writer - DMA is allowed to modify DOUT30"]
pub type DmamaskDout30W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout30>;
impl<'a, REG> DmamaskDout30W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout30_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout30::DmamaskDout30Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout30_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout30::DmamaskDout30Enable)
    }
}
#[doc = "DMA is allowed to modify DOUT31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmamaskDout31 {
    #[doc = "0: DISABLE"]
    DmamaskDout31Disable = 0,
    #[doc = "1: ENABLE"]
    DmamaskDout31Enable = 1,
}
impl From<DmamaskDout31> for bool {
    #[inline(always)]
    fn from(variant: DmamaskDout31) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAMASK_DOUT31` reader - DMA is allowed to modify DOUT31"]
pub type DmamaskDout31R = crate::BitReader<DmamaskDout31>;
impl DmamaskDout31R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmamaskDout31 {
        match self.bits {
            false => DmamaskDout31::DmamaskDout31Disable,
            true => DmamaskDout31::DmamaskDout31Enable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout31_disable(&self) -> bool {
        *self == DmamaskDout31::DmamaskDout31Disable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmamask_dout31_enable(&self) -> bool {
        *self == DmamaskDout31::DmamaskDout31Enable
    }
}
#[doc = "Field `DMAMASK_DOUT31` writer - DMA is allowed to modify DOUT31"]
pub type DmamaskDout31W<'a, REG> = crate::BitWriter<'a, REG, DmamaskDout31>;
impl<'a, REG> DmamaskDout31W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmamask_dout31_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout31::DmamaskDout31Disable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmamask_dout31_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmamaskDout31::DmamaskDout31Enable)
    }
}
impl R {
    #[doc = "Bit 0 - DMA is allowed to modify DOUT0"]
    #[inline(always)]
    pub fn dmamask_dout0(&self) -> DmamaskDout0R {
        DmamaskDout0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA is allowed to modify DOUT1"]
    #[inline(always)]
    pub fn dmamask_dout1(&self) -> DmamaskDout1R {
        DmamaskDout1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA is allowed to modify DOUT2"]
    #[inline(always)]
    pub fn dmamask_dout2(&self) -> DmamaskDout2R {
        DmamaskDout2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA is allowed to modify DOUT3"]
    #[inline(always)]
    pub fn dmamask_dout3(&self) -> DmamaskDout3R {
        DmamaskDout3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA is allowed to modify DOUT4"]
    #[inline(always)]
    pub fn dmamask_dout4(&self) -> DmamaskDout4R {
        DmamaskDout4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA is allowed to modify DOUT5"]
    #[inline(always)]
    pub fn dmamask_dout5(&self) -> DmamaskDout5R {
        DmamaskDout5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA is allowed to modify DOUT6"]
    #[inline(always)]
    pub fn dmamask_dout6(&self) -> DmamaskDout6R {
        DmamaskDout6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA is allowed to modify DOUT7"]
    #[inline(always)]
    pub fn dmamask_dout7(&self) -> DmamaskDout7R {
        DmamaskDout7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA is allowed to modify DOUT8"]
    #[inline(always)]
    pub fn dmamask_dout8(&self) -> DmamaskDout8R {
        DmamaskDout8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMA is allowed to modify DOUT9"]
    #[inline(always)]
    pub fn dmamask_dout9(&self) -> DmamaskDout9R {
        DmamaskDout9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMA is allowed to modify DOUT10"]
    #[inline(always)]
    pub fn dmamask_dout10(&self) -> DmamaskDout10R {
        DmamaskDout10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMA is allowed to modify DOUT11"]
    #[inline(always)]
    pub fn dmamask_dout11(&self) -> DmamaskDout11R {
        DmamaskDout11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA is allowed to modify DOUT12"]
    #[inline(always)]
    pub fn dmamask_dout12(&self) -> DmamaskDout12R {
        DmamaskDout12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA is allowed to modify DOUT13"]
    #[inline(always)]
    pub fn dmamask_dout13(&self) -> DmamaskDout13R {
        DmamaskDout13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA is allowed to modify DOUT14"]
    #[inline(always)]
    pub fn dmamask_dout14(&self) -> DmamaskDout14R {
        DmamaskDout14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA is allowed to modify DOUT15"]
    #[inline(always)]
    pub fn dmamask_dout15(&self) -> DmamaskDout15R {
        DmamaskDout15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA is allowed to modify DOUT16"]
    #[inline(always)]
    pub fn dmamask_dout16(&self) -> DmamaskDout16R {
        DmamaskDout16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA is allowed to modify DOUT17"]
    #[inline(always)]
    pub fn dmamask_dout17(&self) -> DmamaskDout17R {
        DmamaskDout17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMA is allowed to modify DOUT18"]
    #[inline(always)]
    pub fn dmamask_dout18(&self) -> DmamaskDout18R {
        DmamaskDout18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DMA is allowed to modify DOUT19"]
    #[inline(always)]
    pub fn dmamask_dout19(&self) -> DmamaskDout19R {
        DmamaskDout19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DMA is allowed to modify DOUT20"]
    #[inline(always)]
    pub fn dmamask_dout20(&self) -> DmamaskDout20R {
        DmamaskDout20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA is allowed to modify DOUT21"]
    #[inline(always)]
    pub fn dmamask_dout21(&self) -> DmamaskDout21R {
        DmamaskDout21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA is allowed to modify DOUT22"]
    #[inline(always)]
    pub fn dmamask_dout22(&self) -> DmamaskDout22R {
        DmamaskDout22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DMA is allowed to modify DOUT23"]
    #[inline(always)]
    pub fn dmamask_dout23(&self) -> DmamaskDout23R {
        DmamaskDout23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMA is allowed to modify DOUT24"]
    #[inline(always)]
    pub fn dmamask_dout24(&self) -> DmamaskDout24R {
        DmamaskDout24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMA is allowed to modify DOUT25"]
    #[inline(always)]
    pub fn dmamask_dout25(&self) -> DmamaskDout25R {
        DmamaskDout25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DMA is allowed to modify DOUT26"]
    #[inline(always)]
    pub fn dmamask_dout26(&self) -> DmamaskDout26R {
        DmamaskDout26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DMA is allowed to modify DOUT27"]
    #[inline(always)]
    pub fn dmamask_dout27(&self) -> DmamaskDout27R {
        DmamaskDout27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA is allowed to modify DOUT28"]
    #[inline(always)]
    pub fn dmamask_dout28(&self) -> DmamaskDout28R {
        DmamaskDout28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMA is allowed to modify DOUT29"]
    #[inline(always)]
    pub fn dmamask_dout29(&self) -> DmamaskDout29R {
        DmamaskDout29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA is allowed to modify DOUT30"]
    #[inline(always)]
    pub fn dmamask_dout30(&self) -> DmamaskDout30R {
        DmamaskDout30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA is allowed to modify DOUT31"]
    #[inline(always)]
    pub fn dmamask_dout31(&self) -> DmamaskDout31R {
        DmamaskDout31R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA is allowed to modify DOUT0"]
    #[inline(always)]
    pub fn dmamask_dout0(&mut self) -> DmamaskDout0W<DmamaskSpec> {
        DmamaskDout0W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA is allowed to modify DOUT1"]
    #[inline(always)]
    pub fn dmamask_dout1(&mut self) -> DmamaskDout1W<DmamaskSpec> {
        DmamaskDout1W::new(self, 1)
    }
    #[doc = "Bit 2 - DMA is allowed to modify DOUT2"]
    #[inline(always)]
    pub fn dmamask_dout2(&mut self) -> DmamaskDout2W<DmamaskSpec> {
        DmamaskDout2W::new(self, 2)
    }
    #[doc = "Bit 3 - DMA is allowed to modify DOUT3"]
    #[inline(always)]
    pub fn dmamask_dout3(&mut self) -> DmamaskDout3W<DmamaskSpec> {
        DmamaskDout3W::new(self, 3)
    }
    #[doc = "Bit 4 - DMA is allowed to modify DOUT4"]
    #[inline(always)]
    pub fn dmamask_dout4(&mut self) -> DmamaskDout4W<DmamaskSpec> {
        DmamaskDout4W::new(self, 4)
    }
    #[doc = "Bit 5 - DMA is allowed to modify DOUT5"]
    #[inline(always)]
    pub fn dmamask_dout5(&mut self) -> DmamaskDout5W<DmamaskSpec> {
        DmamaskDout5W::new(self, 5)
    }
    #[doc = "Bit 6 - DMA is allowed to modify DOUT6"]
    #[inline(always)]
    pub fn dmamask_dout6(&mut self) -> DmamaskDout6W<DmamaskSpec> {
        DmamaskDout6W::new(self, 6)
    }
    #[doc = "Bit 7 - DMA is allowed to modify DOUT7"]
    #[inline(always)]
    pub fn dmamask_dout7(&mut self) -> DmamaskDout7W<DmamaskSpec> {
        DmamaskDout7W::new(self, 7)
    }
    #[doc = "Bit 8 - DMA is allowed to modify DOUT8"]
    #[inline(always)]
    pub fn dmamask_dout8(&mut self) -> DmamaskDout8W<DmamaskSpec> {
        DmamaskDout8W::new(self, 8)
    }
    #[doc = "Bit 9 - DMA is allowed to modify DOUT9"]
    #[inline(always)]
    pub fn dmamask_dout9(&mut self) -> DmamaskDout9W<DmamaskSpec> {
        DmamaskDout9W::new(self, 9)
    }
    #[doc = "Bit 10 - DMA is allowed to modify DOUT10"]
    #[inline(always)]
    pub fn dmamask_dout10(&mut self) -> DmamaskDout10W<DmamaskSpec> {
        DmamaskDout10W::new(self, 10)
    }
    #[doc = "Bit 11 - DMA is allowed to modify DOUT11"]
    #[inline(always)]
    pub fn dmamask_dout11(&mut self) -> DmamaskDout11W<DmamaskSpec> {
        DmamaskDout11W::new(self, 11)
    }
    #[doc = "Bit 12 - DMA is allowed to modify DOUT12"]
    #[inline(always)]
    pub fn dmamask_dout12(&mut self) -> DmamaskDout12W<DmamaskSpec> {
        DmamaskDout12W::new(self, 12)
    }
    #[doc = "Bit 13 - DMA is allowed to modify DOUT13"]
    #[inline(always)]
    pub fn dmamask_dout13(&mut self) -> DmamaskDout13W<DmamaskSpec> {
        DmamaskDout13W::new(self, 13)
    }
    #[doc = "Bit 14 - DMA is allowed to modify DOUT14"]
    #[inline(always)]
    pub fn dmamask_dout14(&mut self) -> DmamaskDout14W<DmamaskSpec> {
        DmamaskDout14W::new(self, 14)
    }
    #[doc = "Bit 15 - DMA is allowed to modify DOUT15"]
    #[inline(always)]
    pub fn dmamask_dout15(&mut self) -> DmamaskDout15W<DmamaskSpec> {
        DmamaskDout15W::new(self, 15)
    }
    #[doc = "Bit 16 - DMA is allowed to modify DOUT16"]
    #[inline(always)]
    pub fn dmamask_dout16(&mut self) -> DmamaskDout16W<DmamaskSpec> {
        DmamaskDout16W::new(self, 16)
    }
    #[doc = "Bit 17 - DMA is allowed to modify DOUT17"]
    #[inline(always)]
    pub fn dmamask_dout17(&mut self) -> DmamaskDout17W<DmamaskSpec> {
        DmamaskDout17W::new(self, 17)
    }
    #[doc = "Bit 18 - DMA is allowed to modify DOUT18"]
    #[inline(always)]
    pub fn dmamask_dout18(&mut self) -> DmamaskDout18W<DmamaskSpec> {
        DmamaskDout18W::new(self, 18)
    }
    #[doc = "Bit 19 - DMA is allowed to modify DOUT19"]
    #[inline(always)]
    pub fn dmamask_dout19(&mut self) -> DmamaskDout19W<DmamaskSpec> {
        DmamaskDout19W::new(self, 19)
    }
    #[doc = "Bit 20 - DMA is allowed to modify DOUT20"]
    #[inline(always)]
    pub fn dmamask_dout20(&mut self) -> DmamaskDout20W<DmamaskSpec> {
        DmamaskDout20W::new(self, 20)
    }
    #[doc = "Bit 21 - DMA is allowed to modify DOUT21"]
    #[inline(always)]
    pub fn dmamask_dout21(&mut self) -> DmamaskDout21W<DmamaskSpec> {
        DmamaskDout21W::new(self, 21)
    }
    #[doc = "Bit 22 - DMA is allowed to modify DOUT22"]
    #[inline(always)]
    pub fn dmamask_dout22(&mut self) -> DmamaskDout22W<DmamaskSpec> {
        DmamaskDout22W::new(self, 22)
    }
    #[doc = "Bit 23 - DMA is allowed to modify DOUT23"]
    #[inline(always)]
    pub fn dmamask_dout23(&mut self) -> DmamaskDout23W<DmamaskSpec> {
        DmamaskDout23W::new(self, 23)
    }
    #[doc = "Bit 24 - DMA is allowed to modify DOUT24"]
    #[inline(always)]
    pub fn dmamask_dout24(&mut self) -> DmamaskDout24W<DmamaskSpec> {
        DmamaskDout24W::new(self, 24)
    }
    #[doc = "Bit 25 - DMA is allowed to modify DOUT25"]
    #[inline(always)]
    pub fn dmamask_dout25(&mut self) -> DmamaskDout25W<DmamaskSpec> {
        DmamaskDout25W::new(self, 25)
    }
    #[doc = "Bit 26 - DMA is allowed to modify DOUT26"]
    #[inline(always)]
    pub fn dmamask_dout26(&mut self) -> DmamaskDout26W<DmamaskSpec> {
        DmamaskDout26W::new(self, 26)
    }
    #[doc = "Bit 27 - DMA is allowed to modify DOUT27"]
    #[inline(always)]
    pub fn dmamask_dout27(&mut self) -> DmamaskDout27W<DmamaskSpec> {
        DmamaskDout27W::new(self, 27)
    }
    #[doc = "Bit 28 - DMA is allowed to modify DOUT28"]
    #[inline(always)]
    pub fn dmamask_dout28(&mut self) -> DmamaskDout28W<DmamaskSpec> {
        DmamaskDout28W::new(self, 28)
    }
    #[doc = "Bit 29 - DMA is allowed to modify DOUT29"]
    #[inline(always)]
    pub fn dmamask_dout29(&mut self) -> DmamaskDout29W<DmamaskSpec> {
        DmamaskDout29W::new(self, 29)
    }
    #[doc = "Bit 30 - DMA is allowed to modify DOUT30"]
    #[inline(always)]
    pub fn dmamask_dout30(&mut self) -> DmamaskDout30W<DmamaskSpec> {
        DmamaskDout30W::new(self, 30)
    }
    #[doc = "Bit 31 - DMA is allowed to modify DOUT31"]
    #[inline(always)]
    pub fn dmamask_dout31(&mut self) -> DmamaskDout31W<DmamaskSpec> {
        DmamaskDout31W::new(self, 31)
    }
}
#[doc = "DMA Write MASK\n\nYou can [`read`](crate::Reg::read) this register and get [`dmamask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmamaskSpec;
impl crate::RegisterSpec for DmamaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmamask::R`](R) reader structure"]
impl crate::Readable for DmamaskSpec {}
#[doc = "`write(|w| ..)` method takes [`dmamask::W`](W) writer structure"]
impl crate::Writable for DmamaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAMASK to value 0"]
impl crate::Resettable for DmamaskSpec {
    const RESET_VALUE: u32 = 0;
}
