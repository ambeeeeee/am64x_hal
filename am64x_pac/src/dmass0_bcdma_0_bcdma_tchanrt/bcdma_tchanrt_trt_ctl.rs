#[doc = "Register `BCDMA_TCHANRT_TRT_CTL` reader"]
pub type R = crate::R<BcdmaTchanrtTrtCtlSpec>;
#[doc = "Register `BCDMA_TCHANRT_TRT_CTL` writer"]
pub type W = crate::W<BcdmaTchanrtTrtCtlSpec>;
#[doc = "Field `TX_ERROR` reader - 0:0\\]
Channel error: This bit will be set anytime an error has occurred on the channel. This bit is cleared by writing back a 0."]
pub type TxErrorR = crate::BitReader;
#[doc = "Field `TX_ERROR` writer - 0:0\\]
Channel error: This bit will be set anytime an error has occurred on the channel. This bit is cleared by writing back a 0."]
pub type TxErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FORCED_TEARDOWN` reader - 28:28\\]
Channel forced teardown: Setting this bit will cause the channel to stop waiting on trigger events. When this bit is set, the implementation may choose to bypass data transfers and event generation. This bit is a modifier to the normal tx_teardown and is intended to flush the channel to recover any descriptor or TR references which are currently being held by the BCDMA even if the trigger source is no longer functioning. Uso fo this bit is considered a 'catastrophic' condition and it is assumed that SW will need to perform some re-initialization in the system to re-align events, data buffers, etc. This bit should be set in addition to the tx_teardown bit in order to cause a forced teardown. This field will remain set after a channel teardown is complete."]
pub type TxForcedTeardownR = crate::BitReader;
#[doc = "Field `TX_FORCED_TEARDOWN` writer - 28:28\\]
Channel forced teardown: Setting this bit will cause the channel to stop waiting on trigger events. When this bit is set, the implementation may choose to bypass data transfers and event generation. This bit is a modifier to the normal tx_teardown and is intended to flush the channel to recover any descriptor or TR references which are currently being held by the BCDMA even if the trigger source is no longer functioning. Uso fo this bit is considered a 'catastrophic' condition and it is assumed that SW will need to perform some re-initialization in the system to re-align events, data buffers, etc. This bit should be set in addition to the tx_teardown bit in order to cause a forced teardown. This field will remain set after a channel teardown is complete."]
pub type TxForcedTeardownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_PAUSE` reader - 29:29\\]
Channel pause: Setting this bit will cause the channel to pause processing immediately."]
pub type TxPauseR = crate::BitReader;
#[doc = "Field `TX_PAUSE` writer - 29:29\\]
Channel pause: Setting this bit will cause the channel to pause processing immediately."]
pub type TxPauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_TEARDOWN` reader - 30:30\\]
Channel teardown: Setting this bit will request the channel to be torn down. This field will remain set after a channel teardown is complete."]
pub type TxTeardownR = crate::BitReader;
#[doc = "Field `TX_TEARDOWN` writer - 30:30\\]
Channel teardown: Setting this bit will request the channel to be torn down. This field will remain set after a channel teardown is complete."]
pub type TxTeardownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_ENABLE` reader - 31:31\\]
This field enables or disables the channel. Disabling a channel halts operation on the channel after the current block transfer is completed. Disabling a channel in the middle of a packet transfer may result in underflow conditions in the attached application block and data loss. When a channel is disabled, the implementation may choose to reset all state for the channel. The pause bit should be asserted instead of clearing enable directly if the intent is to temporarily pause the channel. This field is encoded as follows: 0 = channel is disabled 1 = channel is enabled This field will be cleared by HW after a teardown is requested to indicate that the channel teardown is complete."]
pub type TxEnableR = crate::BitReader;
#[doc = "Field `TX_ENABLE` writer - 31:31\\]
This field enables or disables the channel. Disabling a channel halts operation on the channel after the current block transfer is completed. Disabling a channel in the middle of a packet transfer may result in underflow conditions in the attached application block and data loss. When a channel is disabled, the implementation may choose to reset all state for the channel. The pause bit should be asserted instead of clearing enable directly if the intent is to temporarily pause the channel. This field is encoded as follows: 0 = channel is disabled 1 = channel is enabled This field will be cleared by HW after a teardown is requested to indicate that the channel teardown is complete."]
pub type TxEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Channel error: This bit will be set anytime an error has occurred on the channel. This bit is cleared by writing back a 0."]
    #[inline(always)]
    pub fn tx_error(&self) -> TxErrorR {
        TxErrorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Channel forced teardown: Setting this bit will cause the channel to stop waiting on trigger events. When this bit is set, the implementation may choose to bypass data transfers and event generation. This bit is a modifier to the normal tx_teardown and is intended to flush the channel to recover any descriptor or TR references which are currently being held by the BCDMA even if the trigger source is no longer functioning. Uso fo this bit is considered a 'catastrophic' condition and it is assumed that SW will need to perform some re-initialization in the system to re-align events, data buffers, etc. This bit should be set in addition to the tx_teardown bit in order to cause a forced teardown. This field will remain set after a channel teardown is complete."]
    #[inline(always)]
    pub fn tx_forced_teardown(&self) -> TxForcedTeardownR {
        TxForcedTeardownR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Channel pause: Setting this bit will cause the channel to pause processing immediately."]
    #[inline(always)]
    pub fn tx_pause(&self) -> TxPauseR {
        TxPauseR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Channel teardown: Setting this bit will request the channel to be torn down. This field will remain set after a channel teardown is complete."]
    #[inline(always)]
    pub fn tx_teardown(&self) -> TxTeardownR {
        TxTeardownR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
This field enables or disables the channel. Disabling a channel halts operation on the channel after the current block transfer is completed. Disabling a channel in the middle of a packet transfer may result in underflow conditions in the attached application block and data loss. When a channel is disabled, the implementation may choose to reset all state for the channel. The pause bit should be asserted instead of clearing enable directly if the intent is to temporarily pause the channel. This field is encoded as follows: 0 = channel is disabled 1 = channel is enabled This field will be cleared by HW after a teardown is requested to indicate that the channel teardown is complete."]
    #[inline(always)]
    pub fn tx_enable(&self) -> TxEnableR {
        TxEnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Channel error: This bit will be set anytime an error has occurred on the channel. This bit is cleared by writing back a 0."]
    #[inline(always)]
    #[must_use]
    pub fn tx_error(&mut self) -> TxErrorW<BcdmaTchanrtTrtCtlSpec> {
        TxErrorW::new(self, 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Channel forced teardown: Setting this bit will cause the channel to stop waiting on trigger events. When this bit is set, the implementation may choose to bypass data transfers and event generation. This bit is a modifier to the normal tx_teardown and is intended to flush the channel to recover any descriptor or TR references which are currently being held by the BCDMA even if the trigger source is no longer functioning. Uso fo this bit is considered a 'catastrophic' condition and it is assumed that SW will need to perform some re-initialization in the system to re-align events, data buffers, etc. This bit should be set in addition to the tx_teardown bit in order to cause a forced teardown. This field will remain set after a channel teardown is complete."]
    #[inline(always)]
    #[must_use]
    pub fn tx_forced_teardown(&mut self) -> TxForcedTeardownW<BcdmaTchanrtTrtCtlSpec> {
        TxForcedTeardownW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Channel pause: Setting this bit will cause the channel to pause processing immediately."]
    #[inline(always)]
    #[must_use]
    pub fn tx_pause(&mut self) -> TxPauseW<BcdmaTchanrtTrtCtlSpec> {
        TxPauseW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Channel teardown: Setting this bit will request the channel to be torn down. This field will remain set after a channel teardown is complete."]
    #[inline(always)]
    #[must_use]
    pub fn tx_teardown(&mut self) -> TxTeardownW<BcdmaTchanrtTrtCtlSpec> {
        TxTeardownW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
This field enables or disables the channel. Disabling a channel halts operation on the channel after the current block transfer is completed. Disabling a channel in the middle of a packet transfer may result in underflow conditions in the attached application block and data loss. When a channel is disabled, the implementation may choose to reset all state for the channel. The pause bit should be asserted instead of clearing enable directly if the intent is to temporarily pause the channel. This field is encoded as follows: 0 = channel is disabled 1 = channel is enabled This field will be cleared by HW after a teardown is requested to indicate that the channel teardown is complete."]
    #[inline(always)]
    #[must_use]
    pub fn tx_enable(&mut self) -> TxEnableW<BcdmaTchanrtTrtCtlSpec> {
        TxEnableW::new(self, 31)
    }
}
#[doc = "The Tx Channel Realtime Control Register contains real-time control and status information for the Tx DMA channel. The fields in this register can safely be changed while the channel is in operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaTchanrtTrtCtlSpec;
impl crate::RegisterSpec for BcdmaTchanrtTrtCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_tchanrt_trt_ctl::R`](R) reader structure"]
impl crate::Readable for BcdmaTchanrtTrtCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_tchanrt_trt_ctl::W`](W) writer structure"]
impl crate::Writable for BcdmaTchanrtTrtCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_TCHANRT_TRT_CTL to value 0"]
impl crate::Resettable for BcdmaTchanrtTrtCtlSpec {
    const RESET_VALUE: u32 = 0;
}
