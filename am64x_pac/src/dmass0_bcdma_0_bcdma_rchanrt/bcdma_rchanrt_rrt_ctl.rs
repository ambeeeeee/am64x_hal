#[doc = "Register `BCDMA_RCHANRT_RRT_CTL` reader"]
pub type R = crate::R<BcdmaRchanrtRrtCtlSpec>;
#[doc = "Register `BCDMA_RCHANRT_RRT_CTL` writer"]
pub type W = crate::W<BcdmaRchanrtRrtCtlSpec>;
#[doc = "Field `RX_ERROR` reader - 0:0\\]
Channel error: This bit will be set anytime an error has occurred on the channel. This bit is cleared when the channel is disabled and re-enabled."]
pub type RxErrorR = crate::BitReader;
#[doc = "Field `RX_ERROR` writer - 0:0\\]
Channel error: This bit will be set anytime an error has occurred on the channel. This bit is cleared when the channel is disabled and re-enabled."]
pub type RxErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_STARVATION` reader - 1:1\\]
Rx starvation. This bit is set if the port receives a packet and the ring is empty. The bit clears when the doorbell is written with a positive value."]
pub type RxStarvationR = crate::BitReader;
#[doc = "Field `RX_STARVATION` writer - 1:1\\]
Rx starvation. This bit is set if the port receives a packet and the ring is empty. The bit clears when the doorbell is written with a positive value."]
pub type RxStarvationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FORCED_TEARDOWN` reader - 28:28\\]
Channel forced teardown: Setting this bit will cause the channel to stop waiting on trigger events. When this bit is set, the implementation may choose to bypass data transfers and event generation. This bit is a modifier to the normal tx_teardown and is intended to flush the channel to recover any descriptor or TR references which are currently being held by the BCDMA even if the trigger source is no longer functioning. Uso fo this bit is considered a 'catastrophic' condition and it is assumed that SW will need to perform some re-initialization in the system to re-align events, data buffers, etc. This bit should be set in addition to the tx_teardown bit in order to cause a forced teardown. This field will remain set after a channel teardown is complete."]
pub type RxForcedTeardownR = crate::BitReader;
#[doc = "Field `RX_FORCED_TEARDOWN` writer - 28:28\\]
Channel forced teardown: Setting this bit will cause the channel to stop waiting on trigger events. When this bit is set, the implementation may choose to bypass data transfers and event generation. This bit is a modifier to the normal tx_teardown and is intended to flush the channel to recover any descriptor or TR references which are currently being held by the BCDMA even if the trigger source is no longer functioning. Uso fo this bit is considered a 'catastrophic' condition and it is assumed that SW will need to perform some re-initialization in the system to re-align events, data buffers, etc. This bit should be set in addition to the tx_teardown bit in order to cause a forced teardown. This field will remain set after a channel teardown is complete."]
pub type RxForcedTeardownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PAUSE` reader - 29:29\\]
Channel pause: Setting this bit will cause the channel to pause processing immediately."]
pub type RxPauseR = crate::BitReader;
#[doc = "Field `RX_PAUSE` writer - 29:29\\]
Channel pause: Setting this bit will cause the channel to pause processing immediately."]
pub type RxPauseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_TEARDOWN` reader - 30:30\\]
This field indicates whether or not an Rx teardown operation is complete. This field should be cleared when a channel is initialized. This field will be set after a channel teardown is complete."]
pub type RxTeardownR = crate::BitReader;
#[doc = "Field `RX_TEARDOWN` writer - 30:30\\]
This field indicates whether or not an Rx teardown operation is complete. This field should be cleared when a channel is initialized. This field will be set after a channel teardown is complete."]
pub type RxTeardownW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_ENABLE` reader - 31:31\\]
This field enables or disables the channel. Disabling a channel halts operation on the channel after the current block transfer is completed. Disabling a channel in the middle of a packet transfer may result in overflow conditions in the attached application and data loss. When a channel is disabled, the implementation may choose to reset all state for the channel. The pause bit shoudl be asserted instead of clearing enable directly if the intent is to temporarily pause the channel. This field is encoded as follows: 0 = channel is disabled 1 = channel is enabled This field will be cleared by HW after a teardown is requested to indicate tha the channel teardown is complete. If the host is enabling a channel that is just being set up, the host must initialize all of the other channel configuration fields before setting this bit."]
pub type RxEnableR = crate::BitReader;
#[doc = "Field `RX_ENABLE` writer - 31:31\\]
This field enables or disables the channel. Disabling a channel halts operation on the channel after the current block transfer is completed. Disabling a channel in the middle of a packet transfer may result in overflow conditions in the attached application and data loss. When a channel is disabled, the implementation may choose to reset all state for the channel. The pause bit shoudl be asserted instead of clearing enable directly if the intent is to temporarily pause the channel. This field is encoded as follows: 0 = channel is disabled 1 = channel is enabled This field will be cleared by HW after a teardown is requested to indicate tha the channel teardown is complete. If the host is enabling a channel that is just being set up, the host must initialize all of the other channel configuration fields before setting this bit."]
pub type RxEnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Channel error: This bit will be set anytime an error has occurred on the channel. This bit is cleared when the channel is disabled and re-enabled."]
    #[inline(always)]
    pub fn rx_error(&self) -> RxErrorR {
        RxErrorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Rx starvation. This bit is set if the port receives a packet and the ring is empty. The bit clears when the doorbell is written with a positive value."]
    #[inline(always)]
    pub fn rx_starvation(&self) -> RxStarvationR {
        RxStarvationR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
Channel forced teardown: Setting this bit will cause the channel to stop waiting on trigger events. When this bit is set, the implementation may choose to bypass data transfers and event generation. This bit is a modifier to the normal tx_teardown and is intended to flush the channel to recover any descriptor or TR references which are currently being held by the BCDMA even if the trigger source is no longer functioning. Uso fo this bit is considered a 'catastrophic' condition and it is assumed that SW will need to perform some re-initialization in the system to re-align events, data buffers, etc. This bit should be set in addition to the tx_teardown bit in order to cause a forced teardown. This field will remain set after a channel teardown is complete."]
    #[inline(always)]
    pub fn rx_forced_teardown(&self) -> RxForcedTeardownR {
        RxForcedTeardownR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Channel pause: Setting this bit will cause the channel to pause processing immediately."]
    #[inline(always)]
    pub fn rx_pause(&self) -> RxPauseR {
        RxPauseR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
This field indicates whether or not an Rx teardown operation is complete. This field should be cleared when a channel is initialized. This field will be set after a channel teardown is complete."]
    #[inline(always)]
    pub fn rx_teardown(&self) -> RxTeardownR {
        RxTeardownR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
This field enables or disables the channel. Disabling a channel halts operation on the channel after the current block transfer is completed. Disabling a channel in the middle of a packet transfer may result in overflow conditions in the attached application and data loss. When a channel is disabled, the implementation may choose to reset all state for the channel. The pause bit shoudl be asserted instead of clearing enable directly if the intent is to temporarily pause the channel. This field is encoded as follows: 0 = channel is disabled 1 = channel is enabled This field will be cleared by HW after a teardown is requested to indicate tha the channel teardown is complete. If the host is enabling a channel that is just being set up, the host must initialize all of the other channel configuration fields before setting this bit."]
    #[inline(always)]
    pub fn rx_enable(&self) -> RxEnableR {
        RxEnableR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Channel error: This bit will be set anytime an error has occurred on the channel. This bit is cleared when the channel is disabled and re-enabled."]
    #[inline(always)]
    #[must_use]
    pub fn rx_error(&mut self) -> RxErrorW<BcdmaRchanrtRrtCtlSpec> {
        RxErrorW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Rx starvation. This bit is set if the port receives a packet and the ring is empty. The bit clears when the doorbell is written with a positive value."]
    #[inline(always)]
    #[must_use]
    pub fn rx_starvation(&mut self) -> RxStarvationW<BcdmaRchanrtRrtCtlSpec> {
        RxStarvationW::new(self, 1)
    }
    #[doc = "Bit 28 - 28:28\\]
Channel forced teardown: Setting this bit will cause the channel to stop waiting on trigger events. When this bit is set, the implementation may choose to bypass data transfers and event generation. This bit is a modifier to the normal tx_teardown and is intended to flush the channel to recover any descriptor or TR references which are currently being held by the BCDMA even if the trigger source is no longer functioning. Uso fo this bit is considered a 'catastrophic' condition and it is assumed that SW will need to perform some re-initialization in the system to re-align events, data buffers, etc. This bit should be set in addition to the tx_teardown bit in order to cause a forced teardown. This field will remain set after a channel teardown is complete."]
    #[inline(always)]
    #[must_use]
    pub fn rx_forced_teardown(&mut self) -> RxForcedTeardownW<BcdmaRchanrtRrtCtlSpec> {
        RxForcedTeardownW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Channel pause: Setting this bit will cause the channel to pause processing immediately."]
    #[inline(always)]
    #[must_use]
    pub fn rx_pause(&mut self) -> RxPauseW<BcdmaRchanrtRrtCtlSpec> {
        RxPauseW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
This field indicates whether or not an Rx teardown operation is complete. This field should be cleared when a channel is initialized. This field will be set after a channel teardown is complete."]
    #[inline(always)]
    #[must_use]
    pub fn rx_teardown(&mut self) -> RxTeardownW<BcdmaRchanrtRrtCtlSpec> {
        RxTeardownW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
This field enables or disables the channel. Disabling a channel halts operation on the channel after the current block transfer is completed. Disabling a channel in the middle of a packet transfer may result in overflow conditions in the attached application and data loss. When a channel is disabled, the implementation may choose to reset all state for the channel. The pause bit shoudl be asserted instead of clearing enable directly if the intent is to temporarily pause the channel. This field is encoded as follows: 0 = channel is disabled 1 = channel is enabled This field will be cleared by HW after a teardown is requested to indicate tha the channel teardown is complete. If the host is enabling a channel that is just being set up, the host must initialize all of the other channel configuration fields before setting this bit."]
    #[inline(always)]
    #[must_use]
    pub fn rx_enable(&mut self) -> RxEnableW<BcdmaRchanrtRrtCtlSpec> {
        RxEnableW::new(self, 31)
    }
}
#[doc = "The Rx Channel Realtime Control Register contains real-time control and status information for the Rx DMA channel. The fields in this register can safely be changed while the channel is in operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchanrt_rrt_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchanrt_rrt_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaRchanrtRrtCtlSpec;
impl crate::RegisterSpec for BcdmaRchanrtRrtCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_rchanrt_rrt_ctl::R`](R) reader structure"]
impl crate::Readable for BcdmaRchanrtRrtCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_rchanrt_rrt_ctl::W`](W) writer structure"]
impl crate::Writable for BcdmaRchanrtRrtCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_RCHANRT_RRT_CTL to value 0"]
impl crate::Resettable for BcdmaRchanrtRrtCtlSpec {
    const RESET_VALUE: u32 = 0;
}
