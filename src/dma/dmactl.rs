#[doc = "Register `DMACTL` reader"]
pub type R = crate::R<DmactlSpec>;
#[doc = "Register `DMACTL` writer"]
pub type W = crate::W<DmactlSpec>;
#[doc = "DMA request. Software-controlled DMA start. DMAREQ is reset automatically.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmactlDmareq {
    #[doc = "0: IDLE"]
    DmactlDmareqIdle = 0,
    #[doc = "1: REQUEST"]
    DmactlDmareqRequest = 1,
}
impl From<DmactlDmareq> for bool {
    #[inline(always)]
    fn from(variant: DmactlDmareq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACTL_DMAREQ` reader - DMA request. Software-controlled DMA start. DMAREQ is reset automatically."]
pub type DmactlDmareqR = crate::BitReader<DmactlDmareq>;
impl DmactlDmareqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmactlDmareq {
        match self.bits {
            false => DmactlDmareq::DmactlDmareqIdle,
            true => DmactlDmareq::DmactlDmareqRequest,
        }
    }
    #[doc = "IDLE"]
    #[inline(always)]
    pub fn is_dmactl_dmareq_idle(&self) -> bool {
        *self == DmactlDmareq::DmactlDmareqIdle
    }
    #[doc = "REQUEST"]
    #[inline(always)]
    pub fn is_dmactl_dmareq_request(&self) -> bool {
        *self == DmactlDmareq::DmactlDmareqRequest
    }
}
#[doc = "Field `DMACTL_DMAREQ` writer - DMA request. Software-controlled DMA start. DMAREQ is reset automatically."]
pub type DmactlDmareqW<'a, REG> = crate::BitWriter<'a, REG, DmactlDmareq>;
impl<'a, REG> DmactlDmareqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "IDLE"]
    #[inline(always)]
    pub fn dmactl_dmareq_idle(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmareq::DmactlDmareqIdle)
    }
    #[doc = "REQUEST"]
    #[inline(always)]
    pub fn dmactl_dmareq_request(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmareq::DmactlDmareqRequest)
    }
}
#[doc = "DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmactlDmaen {
    #[doc = "0: DISABLE"]
    DmactlDmaenDisable = 0,
    #[doc = "1: ENABLE"]
    DmactlDmaenEnable = 1,
}
impl From<DmactlDmaen> for bool {
    #[inline(always)]
    fn from(variant: DmactlDmaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMACTL_DMAEN` reader - DMA enable"]
pub type DmactlDmaenR = crate::BitReader<DmactlDmaen>;
impl DmactlDmaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmactlDmaen {
        match self.bits {
            false => DmactlDmaen::DmactlDmaenDisable,
            true => DmactlDmaen::DmactlDmaenEnable,
        }
    }
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn is_dmactl_dmaen_disable(&self) -> bool {
        *self == DmactlDmaen::DmactlDmaenDisable
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn is_dmactl_dmaen_enable(&self) -> bool {
        *self == DmactlDmaen::DmactlDmaenEnable
    }
}
#[doc = "Field `DMACTL_DMAEN` writer - DMA enable"]
pub type DmactlDmaenW<'a, REG> = crate::BitWriter<'a, REG, DmactlDmaen>;
impl<'a, REG> DmactlDmaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DISABLE"]
    #[inline(always)]
    pub fn dmactl_dmaen_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmaen::DmactlDmaenDisable)
    }
    #[doc = "ENABLE"]
    #[inline(always)]
    pub fn dmactl_dmaen_enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmaen::DmactlDmaenEnable)
    }
}
#[doc = "Enable an early IRQ event. This can help software to react quicker to and DMA done event or allows some additional configuration before the channel is complete. Note: This register is only available in a FULL-channel configuration. Please consult the datasheet of the specific device to map which channel number has FULL or BASIC capability. In a BASIC configuration this register is a read only value and always reads as 0x0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmactlDmapreirq {
    #[doc = "0: PREIRQ_DISABLE"]
    DmactlDmapreirqPreirqDisable = 0,
    #[doc = "1: PREIRQ_1"]
    DmactlDmapreirqPreirq1 = 1,
    #[doc = "2: PREIRQ_2"]
    DmactlDmapreirqPreirq2 = 2,
    #[doc = "3: PREIRQ_4"]
    DmactlDmapreirqPreirq4 = 3,
    #[doc = "4: PREIRQ_8"]
    DmactlDmapreirqPreirq8 = 4,
    #[doc = "5: PREIRQ_32"]
    DmactlDmapreirqPreirq32 = 5,
    #[doc = "6: PREIRQ_64"]
    DmactlDmapreirqPreirq64 = 6,
    #[doc = "7: PREIRQ_HALF"]
    DmactlDmapreirqPreirqHalf = 7,
}
impl From<DmactlDmapreirq> for u8 {
    #[inline(always)]
    fn from(variant: DmactlDmapreirq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DmactlDmapreirq {
    type Ux = u8;
}
impl crate::IsEnum for DmactlDmapreirq {}
#[doc = "Field `DMACTL_DMAPREIRQ` reader - Enable an early IRQ event. This can help software to react quicker to and DMA done event or allows some additional configuration before the channel is complete. Note: This register is only available in a FULL-channel configuration. Please consult the datasheet of the specific device to map which channel number has FULL or BASIC capability. In a BASIC configuration this register is a read only value and always reads as 0x0."]
pub type DmactlDmapreirqR = crate::FieldReader<DmactlDmapreirq>;
impl DmactlDmapreirqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmactlDmapreirq {
        match self.bits {
            0 => DmactlDmapreirq::DmactlDmapreirqPreirqDisable,
            1 => DmactlDmapreirq::DmactlDmapreirqPreirq1,
            2 => DmactlDmapreirq::DmactlDmapreirqPreirq2,
            3 => DmactlDmapreirq::DmactlDmapreirqPreirq4,
            4 => DmactlDmapreirq::DmactlDmapreirqPreirq8,
            5 => DmactlDmapreirq::DmactlDmapreirqPreirq32,
            6 => DmactlDmapreirq::DmactlDmapreirqPreirq64,
            7 => DmactlDmapreirq::DmactlDmapreirqPreirqHalf,
            _ => unreachable!(),
        }
    }
    #[doc = "PREIRQ_DISABLE"]
    #[inline(always)]
    pub fn is_dmactl_dmapreirq_preirq_disable(&self) -> bool {
        *self == DmactlDmapreirq::DmactlDmapreirqPreirqDisable
    }
    #[doc = "PREIRQ_1"]
    #[inline(always)]
    pub fn is_dmactl_dmapreirq_preirq_1(&self) -> bool {
        *self == DmactlDmapreirq::DmactlDmapreirqPreirq1
    }
    #[doc = "PREIRQ_2"]
    #[inline(always)]
    pub fn is_dmactl_dmapreirq_preirq_2(&self) -> bool {
        *self == DmactlDmapreirq::DmactlDmapreirqPreirq2
    }
    #[doc = "PREIRQ_4"]
    #[inline(always)]
    pub fn is_dmactl_dmapreirq_preirq_4(&self) -> bool {
        *self == DmactlDmapreirq::DmactlDmapreirqPreirq4
    }
    #[doc = "PREIRQ_8"]
    #[inline(always)]
    pub fn is_dmactl_dmapreirq_preirq_8(&self) -> bool {
        *self == DmactlDmapreirq::DmactlDmapreirqPreirq8
    }
    #[doc = "PREIRQ_32"]
    #[inline(always)]
    pub fn is_dmactl_dmapreirq_preirq_32(&self) -> bool {
        *self == DmactlDmapreirq::DmactlDmapreirqPreirq32
    }
    #[doc = "PREIRQ_64"]
    #[inline(always)]
    pub fn is_dmactl_dmapreirq_preirq_64(&self) -> bool {
        *self == DmactlDmapreirq::DmactlDmapreirqPreirq64
    }
    #[doc = "PREIRQ_HALF"]
    #[inline(always)]
    pub fn is_dmactl_dmapreirq_preirq_half(&self) -> bool {
        *self == DmactlDmapreirq::DmactlDmapreirqPreirqHalf
    }
}
#[doc = "Field `DMACTL_DMAPREIRQ` writer - Enable an early IRQ event. This can help software to react quicker to and DMA done event or allows some additional configuration before the channel is complete. Note: This register is only available in a FULL-channel configuration. Please consult the datasheet of the specific device to map which channel number has FULL or BASIC capability. In a BASIC configuration this register is a read only value and always reads as 0x0."]
pub type DmactlDmapreirqW<'a, REG> = crate::FieldWriter<'a, REG, 3, DmactlDmapreirq, crate::Safe>;
impl<'a, REG> DmactlDmapreirqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PREIRQ_DISABLE"]
    #[inline(always)]
    pub fn dmactl_dmapreirq_preirq_disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmapreirq::DmactlDmapreirqPreirqDisable)
    }
    #[doc = "PREIRQ_1"]
    #[inline(always)]
    pub fn dmactl_dmapreirq_preirq_1(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmapreirq::DmactlDmapreirqPreirq1)
    }
    #[doc = "PREIRQ_2"]
    #[inline(always)]
    pub fn dmactl_dmapreirq_preirq_2(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmapreirq::DmactlDmapreirqPreirq2)
    }
    #[doc = "PREIRQ_4"]
    #[inline(always)]
    pub fn dmactl_dmapreirq_preirq_4(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmapreirq::DmactlDmapreirqPreirq4)
    }
    #[doc = "PREIRQ_8"]
    #[inline(always)]
    pub fn dmactl_dmapreirq_preirq_8(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmapreirq::DmactlDmapreirqPreirq8)
    }
    #[doc = "PREIRQ_32"]
    #[inline(always)]
    pub fn dmactl_dmapreirq_preirq_32(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmapreirq::DmactlDmapreirqPreirq32)
    }
    #[doc = "PREIRQ_64"]
    #[inline(always)]
    pub fn dmactl_dmapreirq_preirq_64(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmapreirq::DmactlDmapreirqPreirq64)
    }
    #[doc = "PREIRQ_HALF"]
    #[inline(always)]
    pub fn dmactl_dmapreirq_preirq_half(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmapreirq::DmactlDmapreirqPreirqHalf)
    }
}
#[doc = "DMA source width. This bit selects the source data width as a byte, half word, word or long word.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmactlDmasrcwdth {
    #[doc = "0: BYTE"]
    DmactlDmasrcwdthByte = 0,
    #[doc = "1: HALF"]
    DmactlDmasrcwdthHalf = 1,
    #[doc = "2: WORD"]
    DmactlDmasrcwdthWord = 2,
    #[doc = "3: LONG"]
    DmactlDmasrcwdthLong = 3,
}
impl From<DmactlDmasrcwdth> for u8 {
    #[inline(always)]
    fn from(variant: DmactlDmasrcwdth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DmactlDmasrcwdth {
    type Ux = u8;
}
impl crate::IsEnum for DmactlDmasrcwdth {}
#[doc = "Field `DMACTL_DMASRCWDTH` reader - DMA source width. This bit selects the source data width as a byte, half word, word or long word."]
pub type DmactlDmasrcwdthR = crate::FieldReader<DmactlDmasrcwdth>;
impl DmactlDmasrcwdthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmactlDmasrcwdth {
        match self.bits {
            0 => DmactlDmasrcwdth::DmactlDmasrcwdthByte,
            1 => DmactlDmasrcwdth::DmactlDmasrcwdthHalf,
            2 => DmactlDmasrcwdth::DmactlDmasrcwdthWord,
            3 => DmactlDmasrcwdth::DmactlDmasrcwdthLong,
            _ => unreachable!(),
        }
    }
    #[doc = "BYTE"]
    #[inline(always)]
    pub fn is_dmactl_dmasrcwdth_byte(&self) -> bool {
        *self == DmactlDmasrcwdth::DmactlDmasrcwdthByte
    }
    #[doc = "HALF"]
    #[inline(always)]
    pub fn is_dmactl_dmasrcwdth_half(&self) -> bool {
        *self == DmactlDmasrcwdth::DmactlDmasrcwdthHalf
    }
    #[doc = "WORD"]
    #[inline(always)]
    pub fn is_dmactl_dmasrcwdth_word(&self) -> bool {
        *self == DmactlDmasrcwdth::DmactlDmasrcwdthWord
    }
    #[doc = "LONG"]
    #[inline(always)]
    pub fn is_dmactl_dmasrcwdth_long(&self) -> bool {
        *self == DmactlDmasrcwdth::DmactlDmasrcwdthLong
    }
}
#[doc = "Field `DMACTL_DMASRCWDTH` writer - DMA source width. This bit selects the source data width as a byte, half word, word or long word."]
pub type DmactlDmasrcwdthW<'a, REG> = crate::FieldWriter<'a, REG, 2, DmactlDmasrcwdth, crate::Safe>;
impl<'a, REG> DmactlDmasrcwdthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "BYTE"]
    #[inline(always)]
    pub fn dmactl_dmasrcwdth_byte(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmasrcwdth::DmactlDmasrcwdthByte)
    }
    #[doc = "HALF"]
    #[inline(always)]
    pub fn dmactl_dmasrcwdth_half(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmasrcwdth::DmactlDmasrcwdthHalf)
    }
    #[doc = "WORD"]
    #[inline(always)]
    pub fn dmactl_dmasrcwdth_word(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmasrcwdth::DmactlDmasrcwdthWord)
    }
    #[doc = "LONG"]
    #[inline(always)]
    pub fn dmactl_dmasrcwdth_long(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmasrcwdth::DmactlDmasrcwdthLong)
    }
}
#[doc = "DMA destination width. This bit selects the destination as a byte, half word, word or long word.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmactlDmadstwdth {
    #[doc = "0: BYTE"]
    DmactlDmadstwdthByte = 0,
    #[doc = "1: HALF"]
    DmactlDmadstwdthHalf = 1,
    #[doc = "2: WORD"]
    DmactlDmadstwdthWord = 2,
    #[doc = "3: LONG"]
    DmactlDmadstwdthLong = 3,
}
impl From<DmactlDmadstwdth> for u8 {
    #[inline(always)]
    fn from(variant: DmactlDmadstwdth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DmactlDmadstwdth {
    type Ux = u8;
}
impl crate::IsEnum for DmactlDmadstwdth {}
#[doc = "Field `DMACTL_DMADSTWDTH` reader - DMA destination width. This bit selects the destination as a byte, half word, word or long word."]
pub type DmactlDmadstwdthR = crate::FieldReader<DmactlDmadstwdth>;
impl DmactlDmadstwdthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmactlDmadstwdth {
        match self.bits {
            0 => DmactlDmadstwdth::DmactlDmadstwdthByte,
            1 => DmactlDmadstwdth::DmactlDmadstwdthHalf,
            2 => DmactlDmadstwdth::DmactlDmadstwdthWord,
            3 => DmactlDmadstwdth::DmactlDmadstwdthLong,
            _ => unreachable!(),
        }
    }
    #[doc = "BYTE"]
    #[inline(always)]
    pub fn is_dmactl_dmadstwdth_byte(&self) -> bool {
        *self == DmactlDmadstwdth::DmactlDmadstwdthByte
    }
    #[doc = "HALF"]
    #[inline(always)]
    pub fn is_dmactl_dmadstwdth_half(&self) -> bool {
        *self == DmactlDmadstwdth::DmactlDmadstwdthHalf
    }
    #[doc = "WORD"]
    #[inline(always)]
    pub fn is_dmactl_dmadstwdth_word(&self) -> bool {
        *self == DmactlDmadstwdth::DmactlDmadstwdthWord
    }
    #[doc = "LONG"]
    #[inline(always)]
    pub fn is_dmactl_dmadstwdth_long(&self) -> bool {
        *self == DmactlDmadstwdth::DmactlDmadstwdthLong
    }
}
#[doc = "Field `DMACTL_DMADSTWDTH` writer - DMA destination width. This bit selects the destination as a byte, half word, word or long word."]
pub type DmactlDmadstwdthW<'a, REG> = crate::FieldWriter<'a, REG, 2, DmactlDmadstwdth, crate::Safe>;
impl<'a, REG> DmactlDmadstwdthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "BYTE"]
    #[inline(always)]
    pub fn dmactl_dmadstwdth_byte(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmadstwdth::DmactlDmadstwdthByte)
    }
    #[doc = "HALF"]
    #[inline(always)]
    pub fn dmactl_dmadstwdth_half(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmadstwdth::DmactlDmadstwdthHalf)
    }
    #[doc = "WORD"]
    #[inline(always)]
    pub fn dmactl_dmadstwdth_word(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmadstwdth::DmactlDmadstwdthWord)
    }
    #[doc = "LONG"]
    #[inline(always)]
    pub fn dmactl_dmadstwdth_long(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmadstwdth::DmactlDmadstwdthLong)
    }
}
#[doc = "DMA source increment. This bit selects automatic incrementing or decrementing of the source address DMASA for each transfer. The amount of change to the DMASA is based on the definitin in the DMASRCWDTH. For example an increment of 1 (+1) on a WORD transfer will increment the DMASA by 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmactlDmasrcincr {
    #[doc = "0: UNCHANGED"]
    DmactlDmasrcincrUnchanged = 0,
    #[doc = "2: DECREMENT"]
    DmactlDmasrcincrDecrement = 2,
    #[doc = "3: INCREMENT"]
    DmactlDmasrcincrIncrement = 3,
    #[doc = "8: STRIDE_2"]
    DmactlDmasrcincrStride2 = 8,
    #[doc = "9: STRIDE_3"]
    DmactlDmasrcincrStride3 = 9,
    #[doc = "10: STRIDE_4"]
    DmactlDmasrcincrStride4 = 10,
    #[doc = "11: STRIDE_5"]
    DmactlDmasrcincrStride5 = 11,
    #[doc = "12: STRIDE_6"]
    DmactlDmasrcincrStride6 = 12,
    #[doc = "13: STRIDE_7"]
    DmactlDmasrcincrStride7 = 13,
    #[doc = "14: STRIDE_8"]
    DmactlDmasrcincrStride8 = 14,
    #[doc = "15: STRIDE_9"]
    DmactlDmasrcincrStride9 = 15,
}
impl From<DmactlDmasrcincr> for u8 {
    #[inline(always)]
    fn from(variant: DmactlDmasrcincr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DmactlDmasrcincr {
    type Ux = u8;
}
impl crate::IsEnum for DmactlDmasrcincr {}
#[doc = "Field `DMACTL_DMASRCINCR` reader - DMA source increment. This bit selects automatic incrementing or decrementing of the source address DMASA for each transfer. The amount of change to the DMASA is based on the definitin in the DMASRCWDTH. For example an increment of 1 (+1) on a WORD transfer will increment the DMASA by 4."]
pub type DmactlDmasrcincrR = crate::FieldReader<DmactlDmasrcincr>;
impl DmactlDmasrcincrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DmactlDmasrcincr> {
        match self.bits {
            0 => Some(DmactlDmasrcincr::DmactlDmasrcincrUnchanged),
            2 => Some(DmactlDmasrcincr::DmactlDmasrcincrDecrement),
            3 => Some(DmactlDmasrcincr::DmactlDmasrcincrIncrement),
            8 => Some(DmactlDmasrcincr::DmactlDmasrcincrStride2),
            9 => Some(DmactlDmasrcincr::DmactlDmasrcincrStride3),
            10 => Some(DmactlDmasrcincr::DmactlDmasrcincrStride4),
            11 => Some(DmactlDmasrcincr::DmactlDmasrcincrStride5),
            12 => Some(DmactlDmasrcincr::DmactlDmasrcincrStride6),
            13 => Some(DmactlDmasrcincr::DmactlDmasrcincrStride7),
            14 => Some(DmactlDmasrcincr::DmactlDmasrcincrStride8),
            15 => Some(DmactlDmasrcincr::DmactlDmasrcincrStride9),
            _ => None,
        }
    }
    #[doc = "UNCHANGED"]
    #[inline(always)]
    pub fn is_dmactl_dmasrcincr_unchanged(&self) -> bool {
        *self == DmactlDmasrcincr::DmactlDmasrcincrUnchanged
    }
    #[doc = "DECREMENT"]
    #[inline(always)]
    pub fn is_dmactl_dmasrcincr_decrement(&self) -> bool {
        *self == DmactlDmasrcincr::DmactlDmasrcincrDecrement
    }
    #[doc = "INCREMENT"]
    #[inline(always)]
    pub fn is_dmactl_dmasrcincr_increment(&self) -> bool {
        *self == DmactlDmasrcincr::DmactlDmasrcincrIncrement
    }
    #[doc = "STRIDE_2"]
    #[inline(always)]
    pub fn is_dmactl_dmasrcincr_stride_2(&self) -> bool {
        *self == DmactlDmasrcincr::DmactlDmasrcincrStride2
    }
    #[doc = "STRIDE_3"]
    #[inline(always)]
    pub fn is_dmactl_dmasrcincr_stride_3(&self) -> bool {
        *self == DmactlDmasrcincr::DmactlDmasrcincrStride3
    }
    #[doc = "STRIDE_4"]
    #[inline(always)]
    pub fn is_dmactl_dmasrcincr_stride_4(&self) -> bool {
        *self == DmactlDmasrcincr::DmactlDmasrcincrStride4
    }
    #[doc = "STRIDE_5"]
    #[inline(always)]
    pub fn is_dmactl_dmasrcincr_stride_5(&self) -> bool {
        *self == DmactlDmasrcincr::DmactlDmasrcincrStride5
    }
    #[doc = "STRIDE_6"]
    #[inline(always)]
    pub fn is_dmactl_dmasrcincr_stride_6(&self) -> bool {
        *self == DmactlDmasrcincr::DmactlDmasrcincrStride6
    }
    #[doc = "STRIDE_7"]
    #[inline(always)]
    pub fn is_dmactl_dmasrcincr_stride_7(&self) -> bool {
        *self == DmactlDmasrcincr::DmactlDmasrcincrStride7
    }
    #[doc = "STRIDE_8"]
    #[inline(always)]
    pub fn is_dmactl_dmasrcincr_stride_8(&self) -> bool {
        *self == DmactlDmasrcincr::DmactlDmasrcincrStride8
    }
    #[doc = "STRIDE_9"]
    #[inline(always)]
    pub fn is_dmactl_dmasrcincr_stride_9(&self) -> bool {
        *self == DmactlDmasrcincr::DmactlDmasrcincrStride9
    }
}
#[doc = "Field `DMACTL_DMASRCINCR` writer - DMA source increment. This bit selects automatic incrementing or decrementing of the source address DMASA for each transfer. The amount of change to the DMASA is based on the definitin in the DMASRCWDTH. For example an increment of 1 (+1) on a WORD transfer will increment the DMASA by 4."]
pub type DmactlDmasrcincrW<'a, REG> = crate::FieldWriter<'a, REG, 4, DmactlDmasrcincr>;
impl<'a, REG> DmactlDmasrcincrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UNCHANGED"]
    #[inline(always)]
    pub fn dmactl_dmasrcincr_unchanged(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmasrcincr::DmactlDmasrcincrUnchanged)
    }
    #[doc = "DECREMENT"]
    #[inline(always)]
    pub fn dmactl_dmasrcincr_decrement(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmasrcincr::DmactlDmasrcincrDecrement)
    }
    #[doc = "INCREMENT"]
    #[inline(always)]
    pub fn dmactl_dmasrcincr_increment(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmasrcincr::DmactlDmasrcincrIncrement)
    }
    #[doc = "STRIDE_2"]
    #[inline(always)]
    pub fn dmactl_dmasrcincr_stride_2(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmasrcincr::DmactlDmasrcincrStride2)
    }
    #[doc = "STRIDE_3"]
    #[inline(always)]
    pub fn dmactl_dmasrcincr_stride_3(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmasrcincr::DmactlDmasrcincrStride3)
    }
    #[doc = "STRIDE_4"]
    #[inline(always)]
    pub fn dmactl_dmasrcincr_stride_4(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmasrcincr::DmactlDmasrcincrStride4)
    }
    #[doc = "STRIDE_5"]
    #[inline(always)]
    pub fn dmactl_dmasrcincr_stride_5(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmasrcincr::DmactlDmasrcincrStride5)
    }
    #[doc = "STRIDE_6"]
    #[inline(always)]
    pub fn dmactl_dmasrcincr_stride_6(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmasrcincr::DmactlDmasrcincrStride6)
    }
    #[doc = "STRIDE_7"]
    #[inline(always)]
    pub fn dmactl_dmasrcincr_stride_7(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmasrcincr::DmactlDmasrcincrStride7)
    }
    #[doc = "STRIDE_8"]
    #[inline(always)]
    pub fn dmactl_dmasrcincr_stride_8(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmasrcincr::DmactlDmasrcincrStride8)
    }
    #[doc = "STRIDE_9"]
    #[inline(always)]
    pub fn dmactl_dmasrcincr_stride_9(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmasrcincr::DmactlDmasrcincrStride9)
    }
}
#[doc = "DMA destination increment. This bit selects automatic incrementing or decrementing of the destination address DMADA for each transfer. The amount of change to the DMADA is based on the definitin in the DMADSTWDTH. For example an increment of 1 (+1) on a WORD transfer will increment the DMADA by 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmactlDmadstincr {
    #[doc = "0: UNCHANGED"]
    DmactlDmadstincrUnchanged = 0,
    #[doc = "2: DECREMENT"]
    DmactlDmadstincrDecrement = 2,
    #[doc = "3: INCREMENT"]
    DmactlDmadstincrIncrement = 3,
    #[doc = "8: STRIDE_2"]
    DmactlDmadstincrStride2 = 8,
    #[doc = "9: STRIDE_3"]
    DmactlDmadstincrStride3 = 9,
    #[doc = "10: STRIDE_4"]
    DmactlDmadstincrStride4 = 10,
    #[doc = "11: STRIDE_5"]
    DmactlDmadstincrStride5 = 11,
    #[doc = "12: STRIDE_6"]
    DmactlDmadstincrStride6 = 12,
    #[doc = "13: STRIDE_7"]
    DmactlDmadstincrStride7 = 13,
    #[doc = "14: STRIDE_8"]
    DmactlDmadstincrStride8 = 14,
    #[doc = "15: STRIDE_9"]
    DmactlDmadstincrStride9 = 15,
}
impl From<DmactlDmadstincr> for u8 {
    #[inline(always)]
    fn from(variant: DmactlDmadstincr) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DmactlDmadstincr {
    type Ux = u8;
}
impl crate::IsEnum for DmactlDmadstincr {}
#[doc = "Field `DMACTL_DMADSTINCR` reader - DMA destination increment. This bit selects automatic incrementing or decrementing of the destination address DMADA for each transfer. The amount of change to the DMADA is based on the definitin in the DMADSTWDTH. For example an increment of 1 (+1) on a WORD transfer will increment the DMADA by 4."]
pub type DmactlDmadstincrR = crate::FieldReader<DmactlDmadstincr>;
impl DmactlDmadstincrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DmactlDmadstincr> {
        match self.bits {
            0 => Some(DmactlDmadstincr::DmactlDmadstincrUnchanged),
            2 => Some(DmactlDmadstincr::DmactlDmadstincrDecrement),
            3 => Some(DmactlDmadstincr::DmactlDmadstincrIncrement),
            8 => Some(DmactlDmadstincr::DmactlDmadstincrStride2),
            9 => Some(DmactlDmadstincr::DmactlDmadstincrStride3),
            10 => Some(DmactlDmadstincr::DmactlDmadstincrStride4),
            11 => Some(DmactlDmadstincr::DmactlDmadstincrStride5),
            12 => Some(DmactlDmadstincr::DmactlDmadstincrStride6),
            13 => Some(DmactlDmadstincr::DmactlDmadstincrStride7),
            14 => Some(DmactlDmadstincr::DmactlDmadstincrStride8),
            15 => Some(DmactlDmadstincr::DmactlDmadstincrStride9),
            _ => None,
        }
    }
    #[doc = "UNCHANGED"]
    #[inline(always)]
    pub fn is_dmactl_dmadstincr_unchanged(&self) -> bool {
        *self == DmactlDmadstincr::DmactlDmadstincrUnchanged
    }
    #[doc = "DECREMENT"]
    #[inline(always)]
    pub fn is_dmactl_dmadstincr_decrement(&self) -> bool {
        *self == DmactlDmadstincr::DmactlDmadstincrDecrement
    }
    #[doc = "INCREMENT"]
    #[inline(always)]
    pub fn is_dmactl_dmadstincr_increment(&self) -> bool {
        *self == DmactlDmadstincr::DmactlDmadstincrIncrement
    }
    #[doc = "STRIDE_2"]
    #[inline(always)]
    pub fn is_dmactl_dmadstincr_stride_2(&self) -> bool {
        *self == DmactlDmadstincr::DmactlDmadstincrStride2
    }
    #[doc = "STRIDE_3"]
    #[inline(always)]
    pub fn is_dmactl_dmadstincr_stride_3(&self) -> bool {
        *self == DmactlDmadstincr::DmactlDmadstincrStride3
    }
    #[doc = "STRIDE_4"]
    #[inline(always)]
    pub fn is_dmactl_dmadstincr_stride_4(&self) -> bool {
        *self == DmactlDmadstincr::DmactlDmadstincrStride4
    }
    #[doc = "STRIDE_5"]
    #[inline(always)]
    pub fn is_dmactl_dmadstincr_stride_5(&self) -> bool {
        *self == DmactlDmadstincr::DmactlDmadstincrStride5
    }
    #[doc = "STRIDE_6"]
    #[inline(always)]
    pub fn is_dmactl_dmadstincr_stride_6(&self) -> bool {
        *self == DmactlDmadstincr::DmactlDmadstincrStride6
    }
    #[doc = "STRIDE_7"]
    #[inline(always)]
    pub fn is_dmactl_dmadstincr_stride_7(&self) -> bool {
        *self == DmactlDmadstincr::DmactlDmadstincrStride7
    }
    #[doc = "STRIDE_8"]
    #[inline(always)]
    pub fn is_dmactl_dmadstincr_stride_8(&self) -> bool {
        *self == DmactlDmadstincr::DmactlDmadstincrStride8
    }
    #[doc = "STRIDE_9"]
    #[inline(always)]
    pub fn is_dmactl_dmadstincr_stride_9(&self) -> bool {
        *self == DmactlDmadstincr::DmactlDmadstincrStride9
    }
}
#[doc = "Field `DMACTL_DMADSTINCR` writer - DMA destination increment. This bit selects automatic incrementing or decrementing of the destination address DMADA for each transfer. The amount of change to the DMADA is based on the definitin in the DMADSTWDTH. For example an increment of 1 (+1) on a WORD transfer will increment the DMADA by 4."]
pub type DmactlDmadstincrW<'a, REG> = crate::FieldWriter<'a, REG, 4, DmactlDmadstincr>;
impl<'a, REG> DmactlDmadstincrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "UNCHANGED"]
    #[inline(always)]
    pub fn dmactl_dmadstincr_unchanged(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmadstincr::DmactlDmadstincrUnchanged)
    }
    #[doc = "DECREMENT"]
    #[inline(always)]
    pub fn dmactl_dmadstincr_decrement(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmadstincr::DmactlDmadstincrDecrement)
    }
    #[doc = "INCREMENT"]
    #[inline(always)]
    pub fn dmactl_dmadstincr_increment(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmadstincr::DmactlDmadstincrIncrement)
    }
    #[doc = "STRIDE_2"]
    #[inline(always)]
    pub fn dmactl_dmadstincr_stride_2(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmadstincr::DmactlDmadstincrStride2)
    }
    #[doc = "STRIDE_3"]
    #[inline(always)]
    pub fn dmactl_dmadstincr_stride_3(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmadstincr::DmactlDmadstincrStride3)
    }
    #[doc = "STRIDE_4"]
    #[inline(always)]
    pub fn dmactl_dmadstincr_stride_4(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmadstincr::DmactlDmadstincrStride4)
    }
    #[doc = "STRIDE_5"]
    #[inline(always)]
    pub fn dmactl_dmadstincr_stride_5(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmadstincr::DmactlDmadstincrStride5)
    }
    #[doc = "STRIDE_6"]
    #[inline(always)]
    pub fn dmactl_dmadstincr_stride_6(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmadstincr::DmactlDmadstincrStride6)
    }
    #[doc = "STRIDE_7"]
    #[inline(always)]
    pub fn dmactl_dmadstincr_stride_7(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmadstincr::DmactlDmadstincrStride7)
    }
    #[doc = "STRIDE_8"]
    #[inline(always)]
    pub fn dmactl_dmadstincr_stride_8(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmadstincr::DmactlDmadstincrStride8)
    }
    #[doc = "STRIDE_9"]
    #[inline(always)]
    pub fn dmactl_dmadstincr_stride_9(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmadstincr::DmactlDmadstincrStride9)
    }
}
#[doc = "DMA extended mode Note: The extended transfer modes are only available in a FULL-channel configuration. Please consult the datasheet of the specific device to map which channel number has FULL or BASIC capability. In a BASIC channel configuration this register is a read-only register and reads 0x0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmactlDmaem {
    #[doc = "0: NORMAL"]
    DmactlDmaemNormal = 0,
    #[doc = "2: FILLMODE"]
    DmactlDmaemFillmode = 2,
    #[doc = "3: TABLEMODE"]
    DmactlDmaemTablemode = 3,
}
impl From<DmactlDmaem> for u8 {
    #[inline(always)]
    fn from(variant: DmactlDmaem) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DmactlDmaem {
    type Ux = u8;
}
impl crate::IsEnum for DmactlDmaem {}
#[doc = "Field `DMACTL_DMAEM` reader - DMA extended mode Note: The extended transfer modes are only available in a FULL-channel configuration. Please consult the datasheet of the specific device to map which channel number has FULL or BASIC capability. In a BASIC channel configuration this register is a read-only register and reads 0x0."]
pub type DmactlDmaemR = crate::FieldReader<DmactlDmaem>;
impl DmactlDmaemR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DmactlDmaem> {
        match self.bits {
            0 => Some(DmactlDmaem::DmactlDmaemNormal),
            2 => Some(DmactlDmaem::DmactlDmaemFillmode),
            3 => Some(DmactlDmaem::DmactlDmaemTablemode),
            _ => None,
        }
    }
    #[doc = "NORMAL"]
    #[inline(always)]
    pub fn is_dmactl_dmaem_normal(&self) -> bool {
        *self == DmactlDmaem::DmactlDmaemNormal
    }
    #[doc = "FILLMODE"]
    #[inline(always)]
    pub fn is_dmactl_dmaem_fillmode(&self) -> bool {
        *self == DmactlDmaem::DmactlDmaemFillmode
    }
    #[doc = "TABLEMODE"]
    #[inline(always)]
    pub fn is_dmactl_dmaem_tablemode(&self) -> bool {
        *self == DmactlDmaem::DmactlDmaemTablemode
    }
}
#[doc = "Field `DMACTL_DMAEM` writer - DMA extended mode Note: The extended transfer modes are only available in a FULL-channel configuration. Please consult the datasheet of the specific device to map which channel number has FULL or BASIC capability. In a BASIC channel configuration this register is a read-only register and reads 0x0."]
pub type DmactlDmaemW<'a, REG> = crate::FieldWriter<'a, REG, 2, DmactlDmaem>;
impl<'a, REG> DmactlDmaemW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NORMAL"]
    #[inline(always)]
    pub fn dmactl_dmaem_normal(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmaem::DmactlDmaemNormal)
    }
    #[doc = "FILLMODE"]
    #[inline(always)]
    pub fn dmactl_dmaem_fillmode(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmaem::DmactlDmaemFillmode)
    }
    #[doc = "TABLEMODE"]
    #[inline(always)]
    pub fn dmactl_dmaem_tablemode(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmaem::DmactlDmaemTablemode)
    }
}
#[doc = "DMA transfer mode register Note: The repeat-single (2h) and repeat-block (3h) transfer are only available in a FULL-channel configuration. Please consult the datasheet of the specific device to map which channel number has FULL or BASIC capability. In a BASIC channel configuration only the values for single (0h) and block (1h) transfer can be set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmactlDmatm {
    #[doc = "0: SINGLE"]
    DmactlDmatmSingle = 0,
    #[doc = "1: BLOCK"]
    DmactlDmatmBlock = 1,
    #[doc = "2: RPTSNGL"]
    DmactlDmatmRptsngl = 2,
    #[doc = "3: RPTBLCK"]
    DmactlDmatmRptblck = 3,
}
impl From<DmactlDmatm> for u8 {
    #[inline(always)]
    fn from(variant: DmactlDmatm) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DmactlDmatm {
    type Ux = u8;
}
impl crate::IsEnum for DmactlDmatm {}
#[doc = "Field `DMACTL_DMATM` reader - DMA transfer mode register Note: The repeat-single (2h) and repeat-block (3h) transfer are only available in a FULL-channel configuration. Please consult the datasheet of the specific device to map which channel number has FULL or BASIC capability. In a BASIC channel configuration only the values for single (0h) and block (1h) transfer can be set."]
pub type DmactlDmatmR = crate::FieldReader<DmactlDmatm>;
impl DmactlDmatmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmactlDmatm {
        match self.bits {
            0 => DmactlDmatm::DmactlDmatmSingle,
            1 => DmactlDmatm::DmactlDmatmBlock,
            2 => DmactlDmatm::DmactlDmatmRptsngl,
            3 => DmactlDmatm::DmactlDmatmRptblck,
            _ => unreachable!(),
        }
    }
    #[doc = "SINGLE"]
    #[inline(always)]
    pub fn is_dmactl_dmatm_single(&self) -> bool {
        *self == DmactlDmatm::DmactlDmatmSingle
    }
    #[doc = "BLOCK"]
    #[inline(always)]
    pub fn is_dmactl_dmatm_block(&self) -> bool {
        *self == DmactlDmatm::DmactlDmatmBlock
    }
    #[doc = "RPTSNGL"]
    #[inline(always)]
    pub fn is_dmactl_dmatm_rptsngl(&self) -> bool {
        *self == DmactlDmatm::DmactlDmatmRptsngl
    }
    #[doc = "RPTBLCK"]
    #[inline(always)]
    pub fn is_dmactl_dmatm_rptblck(&self) -> bool {
        *self == DmactlDmatm::DmactlDmatmRptblck
    }
}
#[doc = "Field `DMACTL_DMATM` writer - DMA transfer mode register Note: The repeat-single (2h) and repeat-block (3h) transfer are only available in a FULL-channel configuration. Please consult the datasheet of the specific device to map which channel number has FULL or BASIC capability. In a BASIC channel configuration only the values for single (0h) and block (1h) transfer can be set."]
pub type DmactlDmatmW<'a, REG> = crate::FieldWriter<'a, REG, 2, DmactlDmatm, crate::Safe>;
impl<'a, REG> DmactlDmatmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SINGLE"]
    #[inline(always)]
    pub fn dmactl_dmatm_single(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmatm::DmactlDmatmSingle)
    }
    #[doc = "BLOCK"]
    #[inline(always)]
    pub fn dmactl_dmatm_block(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmatm::DmactlDmatmBlock)
    }
    #[doc = "RPTSNGL"]
    #[inline(always)]
    pub fn dmactl_dmatm_rptsngl(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmatm::DmactlDmatmRptsngl)
    }
    #[doc = "RPTBLCK"]
    #[inline(always)]
    pub fn dmactl_dmatm_rptblck(self) -> &'a mut crate::W<REG> {
        self.variant(DmactlDmatm::DmactlDmatmRptblck)
    }
}
impl R {
    #[doc = "Bit 0 - DMA request. Software-controlled DMA start. DMAREQ is reset automatically."]
    #[inline(always)]
    pub fn dmactl_dmareq(&self) -> DmactlDmareqR {
        DmactlDmareqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA enable"]
    #[inline(always)]
    pub fn dmactl_dmaen(&self) -> DmactlDmaenR {
        DmactlDmaenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Enable an early IRQ event. This can help software to react quicker to and DMA done event or allows some additional configuration before the channel is complete. Note: This register is only available in a FULL-channel configuration. Please consult the datasheet of the specific device to map which channel number has FULL or BASIC capability. In a BASIC configuration this register is a read only value and always reads as 0x0."]
    #[inline(always)]
    pub fn dmactl_dmapreirq(&self) -> DmactlDmapreirqR {
        DmactlDmapreirqR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - DMA source width. This bit selects the source data width as a byte, half word, word or long word."]
    #[inline(always)]
    pub fn dmactl_dmasrcwdth(&self) -> DmactlDmasrcwdthR {
        DmactlDmasrcwdthR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - DMA destination width. This bit selects the destination as a byte, half word, word or long word."]
    #[inline(always)]
    pub fn dmactl_dmadstwdth(&self) -> DmactlDmadstwdthR {
        DmactlDmadstwdthR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:19 - DMA source increment. This bit selects automatic incrementing or decrementing of the source address DMASA for each transfer. The amount of change to the DMASA is based on the definitin in the DMASRCWDTH. For example an increment of 1 (+1) on a WORD transfer will increment the DMASA by 4."]
    #[inline(always)]
    pub fn dmactl_dmasrcincr(&self) -> DmactlDmasrcincrR {
        DmactlDmasrcincrR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - DMA destination increment. This bit selects automatic incrementing or decrementing of the destination address DMADA for each transfer. The amount of change to the DMADA is based on the definitin in the DMADSTWDTH. For example an increment of 1 (+1) on a WORD transfer will increment the DMADA by 4."]
    #[inline(always)]
    pub fn dmactl_dmadstincr(&self) -> DmactlDmadstincrR {
        DmactlDmadstincrR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - DMA extended mode Note: The extended transfer modes are only available in a FULL-channel configuration. Please consult the datasheet of the specific device to map which channel number has FULL or BASIC capability. In a BASIC channel configuration this register is a read-only register and reads 0x0."]
    #[inline(always)]
    pub fn dmactl_dmaem(&self) -> DmactlDmaemR {
        DmactlDmaemR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - DMA transfer mode register Note: The repeat-single (2h) and repeat-block (3h) transfer are only available in a FULL-channel configuration. Please consult the datasheet of the specific device to map which channel number has FULL or BASIC capability. In a BASIC channel configuration only the values for single (0h) and block (1h) transfer can be set."]
    #[inline(always)]
    pub fn dmactl_dmatm(&self) -> DmactlDmatmR {
        DmactlDmatmR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMA request. Software-controlled DMA start. DMAREQ is reset automatically."]
    #[inline(always)]
    pub fn dmactl_dmareq(&mut self) -> DmactlDmareqW<DmactlSpec> {
        DmactlDmareqW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA enable"]
    #[inline(always)]
    pub fn dmactl_dmaen(&mut self) -> DmactlDmaenW<DmactlSpec> {
        DmactlDmaenW::new(self, 1)
    }
    #[doc = "Bits 4:6 - Enable an early IRQ event. This can help software to react quicker to and DMA done event or allows some additional configuration before the channel is complete. Note: This register is only available in a FULL-channel configuration. Please consult the datasheet of the specific device to map which channel number has FULL or BASIC capability. In a BASIC configuration this register is a read only value and always reads as 0x0."]
    #[inline(always)]
    pub fn dmactl_dmapreirq(&mut self) -> DmactlDmapreirqW<DmactlSpec> {
        DmactlDmapreirqW::new(self, 4)
    }
    #[doc = "Bits 8:9 - DMA source width. This bit selects the source data width as a byte, half word, word or long word."]
    #[inline(always)]
    pub fn dmactl_dmasrcwdth(&mut self) -> DmactlDmasrcwdthW<DmactlSpec> {
        DmactlDmasrcwdthW::new(self, 8)
    }
    #[doc = "Bits 12:13 - DMA destination width. This bit selects the destination as a byte, half word, word or long word."]
    #[inline(always)]
    pub fn dmactl_dmadstwdth(&mut self) -> DmactlDmadstwdthW<DmactlSpec> {
        DmactlDmadstwdthW::new(self, 12)
    }
    #[doc = "Bits 16:19 - DMA source increment. This bit selects automatic incrementing or decrementing of the source address DMASA for each transfer. The amount of change to the DMASA is based on the definitin in the DMASRCWDTH. For example an increment of 1 (+1) on a WORD transfer will increment the DMASA by 4."]
    #[inline(always)]
    pub fn dmactl_dmasrcincr(&mut self) -> DmactlDmasrcincrW<DmactlSpec> {
        DmactlDmasrcincrW::new(self, 16)
    }
    #[doc = "Bits 20:23 - DMA destination increment. This bit selects automatic incrementing or decrementing of the destination address DMADA for each transfer. The amount of change to the DMADA is based on the definitin in the DMADSTWDTH. For example an increment of 1 (+1) on a WORD transfer will increment the DMADA by 4."]
    #[inline(always)]
    pub fn dmactl_dmadstincr(&mut self) -> DmactlDmadstincrW<DmactlSpec> {
        DmactlDmadstincrW::new(self, 20)
    }
    #[doc = "Bits 24:25 - DMA extended mode Note: The extended transfer modes are only available in a FULL-channel configuration. Please consult the datasheet of the specific device to map which channel number has FULL or BASIC capability. In a BASIC channel configuration this register is a read-only register and reads 0x0."]
    #[inline(always)]
    pub fn dmactl_dmaem(&mut self) -> DmactlDmaemW<DmactlSpec> {
        DmactlDmaemW::new(self, 24)
    }
    #[doc = "Bits 28:29 - DMA transfer mode register Note: The repeat-single (2h) and repeat-block (3h) transfer are only available in a FULL-channel configuration. Please consult the datasheet of the specific device to map which channel number has FULL or BASIC capability. In a BASIC channel configuration only the values for single (0h) and block (1h) transfer can be set."]
    #[inline(always)]
    pub fn dmactl_dmatm(&mut self) -> DmactlDmatmW<DmactlSpec> {
        DmactlDmatmW::new(self, 28)
    }
}
#[doc = "DMA Channel Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dmactl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmactl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmactlSpec;
impl crate::RegisterSpec for DmactlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmactl::R`](R) reader structure"]
impl crate::Readable for DmactlSpec {}
#[doc = "`write(|w| ..)` method takes [`dmactl::W`](W) writer structure"]
impl crate::Writable for DmactlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACTL to value 0"]
impl crate::Resettable for DmactlSpec {
    const RESET_VALUE: u32 = 0;
}
