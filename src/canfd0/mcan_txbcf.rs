#[doc = "Register `MCAN_TXBCF` reader"]
pub type R = crate::R<McanTxbcfSpec>;
#[doc = "Field `MCAN_TXBCF_CF0` reader - Cancellation Finished 0. Each Tx Buffer has its own Cancellation Finished bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a '1' to the corresponding bit of register TXBAR. 0 No transmit buffer cancellation 1 Transmit buffer cancellation finished"]
pub type McanTxbcfCf0R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF1` reader - Cancellation Finished 1. See description for bit 0."]
pub type McanTxbcfCf1R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF2` reader - Cancellation Finished 2. See description for bit 0."]
pub type McanTxbcfCf2R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF3` reader - Cancellation Finished 3. See description for bit 0."]
pub type McanTxbcfCf3R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF4` reader - Cancellation Finished 4. See description for bit 0."]
pub type McanTxbcfCf4R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF5` reader - Cancellation Finished 5. See description for bit 0."]
pub type McanTxbcfCf5R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF6` reader - Cancellation Finished 6. See description for bit 0."]
pub type McanTxbcfCf6R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF7` reader - Cancellation Finished 7. See description for bit 0."]
pub type McanTxbcfCf7R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF8` reader - Cancellation Finished 8. See description for bit 0."]
pub type McanTxbcfCf8R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF9` reader - Cancellation Finished 9. See description for bit 0."]
pub type McanTxbcfCf9R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF10` reader - Cancellation Finished 10. See description for bit 0."]
pub type McanTxbcfCf10R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF11` reader - Cancellation Finished 11. See description for bit 0."]
pub type McanTxbcfCf11R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF12` reader - Cancellation Finished 12. See description for bit 0."]
pub type McanTxbcfCf12R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF13` reader - Cancellation Finished 13. See description for bit 0."]
pub type McanTxbcfCf13R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF14` reader - Cancellation Finished 14. See description for bit 0."]
pub type McanTxbcfCf14R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF15` reader - Cancellation Finished 15. See description for bit 0."]
pub type McanTxbcfCf15R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF16` reader - Cancellation Finished 16. See description for bit 0."]
pub type McanTxbcfCf16R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF17` reader - Cancellation Finished 17. See description for bit 0."]
pub type McanTxbcfCf17R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF18` reader - Cancellation Finished 18. See description for bit 0."]
pub type McanTxbcfCf18R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF19` reader - Cancellation Finished 19. See description for bit 0."]
pub type McanTxbcfCf19R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF20` reader - Cancellation Finished 20. See description for bit 0."]
pub type McanTxbcfCf20R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF21` reader - Cancellation Finished 21. See description for bit 0."]
pub type McanTxbcfCf21R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF22` reader - Cancellation Finished 22. See description for bit 0."]
pub type McanTxbcfCf22R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF23` reader - Cancellation Finished 23. See description for bit 0."]
pub type McanTxbcfCf23R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF24` reader - Cancellation Finished 24. See description for bit 0."]
pub type McanTxbcfCf24R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF25` reader - Cancellation Finished 25. See description for bit 0."]
pub type McanTxbcfCf25R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF26` reader - Cancellation Finished 26. See description for bit 0."]
pub type McanTxbcfCf26R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF27` reader - Cancellation Finished 27. See description for bit 0."]
pub type McanTxbcfCf27R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF28` reader - Cancellation Finished 28. See description for bit 0."]
pub type McanTxbcfCf28R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF29` reader - Cancellation Finished 29. See description for bit 0."]
pub type McanTxbcfCf29R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF30` reader - Cancellation Finished 30. See description for bit 0."]
pub type McanTxbcfCf30R = crate::BitReader;
#[doc = "Field `MCAN_TXBCF_CF31` reader - Cancellation Finished 31. See description for bit 0."]
pub type McanTxbcfCf31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Cancellation Finished 0. Each Tx Buffer has its own Cancellation Finished bit. The bits are set when the corresponding TXBRP bit is cleared after a cancellation was requested via TXBCR. In case the corresponding TXBRP bit was not set at the point of cancellation, CF is set immediately. The bits are reset when a new transmission is requested by writing a '1' to the corresponding bit of register TXBAR. 0 No transmit buffer cancellation 1 Transmit buffer cancellation finished"]
    #[inline(always)]
    pub fn mcan_txbcf_cf0(&self) -> McanTxbcfCf0R {
        McanTxbcfCf0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cancellation Finished 1. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf1(&self) -> McanTxbcfCf1R {
        McanTxbcfCf1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Cancellation Finished 2. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf2(&self) -> McanTxbcfCf2R {
        McanTxbcfCf2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Cancellation Finished 3. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf3(&self) -> McanTxbcfCf3R {
        McanTxbcfCf3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Cancellation Finished 4. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf4(&self) -> McanTxbcfCf4R {
        McanTxbcfCf4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Cancellation Finished 5. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf5(&self) -> McanTxbcfCf5R {
        McanTxbcfCf5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Cancellation Finished 6. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf6(&self) -> McanTxbcfCf6R {
        McanTxbcfCf6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Cancellation Finished 7. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf7(&self) -> McanTxbcfCf7R {
        McanTxbcfCf7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Cancellation Finished 8. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf8(&self) -> McanTxbcfCf8R {
        McanTxbcfCf8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Cancellation Finished 9. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf9(&self) -> McanTxbcfCf9R {
        McanTxbcfCf9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Cancellation Finished 10. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf10(&self) -> McanTxbcfCf10R {
        McanTxbcfCf10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Cancellation Finished 11. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf11(&self) -> McanTxbcfCf11R {
        McanTxbcfCf11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Cancellation Finished 12. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf12(&self) -> McanTxbcfCf12R {
        McanTxbcfCf12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Cancellation Finished 13. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf13(&self) -> McanTxbcfCf13R {
        McanTxbcfCf13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Cancellation Finished 14. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf14(&self) -> McanTxbcfCf14R {
        McanTxbcfCf14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Cancellation Finished 15. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf15(&self) -> McanTxbcfCf15R {
        McanTxbcfCf15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Cancellation Finished 16. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf16(&self) -> McanTxbcfCf16R {
        McanTxbcfCf16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Cancellation Finished 17. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf17(&self) -> McanTxbcfCf17R {
        McanTxbcfCf17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Cancellation Finished 18. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf18(&self) -> McanTxbcfCf18R {
        McanTxbcfCf18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Cancellation Finished 19. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf19(&self) -> McanTxbcfCf19R {
        McanTxbcfCf19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Cancellation Finished 20. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf20(&self) -> McanTxbcfCf20R {
        McanTxbcfCf20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Cancellation Finished 21. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf21(&self) -> McanTxbcfCf21R {
        McanTxbcfCf21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Cancellation Finished 22. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf22(&self) -> McanTxbcfCf22R {
        McanTxbcfCf22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Cancellation Finished 23. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf23(&self) -> McanTxbcfCf23R {
        McanTxbcfCf23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Cancellation Finished 24. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf24(&self) -> McanTxbcfCf24R {
        McanTxbcfCf24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Cancellation Finished 25. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf25(&self) -> McanTxbcfCf25R {
        McanTxbcfCf25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Cancellation Finished 26. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf26(&self) -> McanTxbcfCf26R {
        McanTxbcfCf26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Cancellation Finished 27. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf27(&self) -> McanTxbcfCf27R {
        McanTxbcfCf27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Cancellation Finished 28. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf28(&self) -> McanTxbcfCf28R {
        McanTxbcfCf28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Cancellation Finished 29. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf29(&self) -> McanTxbcfCf29R {
        McanTxbcfCf29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Cancellation Finished 30. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf30(&self) -> McanTxbcfCf30R {
        McanTxbcfCf30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Cancellation Finished 31. See description for bit 0."]
    #[inline(always)]
    pub fn mcan_txbcf_cf31(&self) -> McanTxbcfCf31R {
        McanTxbcfCf31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "MCAN Tx Buffer Cancellation Finished\n\nYou can [`read`](crate::Reg::read) this register and get [`mcan_txbcf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McanTxbcfSpec;
impl crate::RegisterSpec for McanTxbcfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcan_txbcf::R`](R) reader structure"]
impl crate::Readable for McanTxbcfSpec {}
#[doc = "`reset()` method sets MCAN_TXBCF to value 0"]
impl crate::Resettable for McanTxbcfSpec {
    const RESET_VALUE: u32 = 0;
}
