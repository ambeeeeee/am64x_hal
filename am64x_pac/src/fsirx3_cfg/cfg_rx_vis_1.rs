#[doc = "Register `CFG_RX_VIS_1` reader"]
pub type R = crate::R<CfgRxVis1Spec>;
#[doc = "Register `CFG_RX_VIS_1` writer"]
pub type W = crate::W<CfgRxVis1Spec>;
#[doc = "Field `RX_CORE_STS` reader - 3:3\\]
Receiver Core Status bitThis bit indicates the status of the receiver core. If this bit is set, the receiver should undergo a reset and subsequent resynchronization with the transmitter. This bit will be always be set when the receiver has detected and end of frame error or a frame type error. This bit can also be set if the receiver becomes corrupted due to noise on the signal lines. If the receiver has experienced a ping watchdog or frame watchdog timeout, this bit should be read to determine if the cause was due to a corrupt transaction, thus putting the receiver core into an unrecoverable state. Only a soft reset will reset the recevier core and thus reset this bit. 0h \\[R\\]
The receiver core is operating normally.1h \\[R\\]
The receiver core has entered into an error state and should be reset."]
pub type RxCoreStsR = crate::BitReader;
#[doc = "Field `RX_CORE_STS` writer - 3:3\\]
Receiver Core Status bitThis bit indicates the status of the receiver core. If this bit is set, the receiver should undergo a reset and subsequent resynchronization with the transmitter. This bit will be always be set when the receiver has detected and end of frame error or a frame type error. This bit can also be set if the receiver becomes corrupted due to noise on the signal lines. If the receiver has experienced a ping watchdog or frame watchdog timeout, this bit should be read to determine if the cause was due to a corrupt transaction, thus putting the receiver core into an unrecoverable state. Only a soft reset will reset the recevier core and thus reset this bit. 0h \\[R\\]
The receiver core is operating normally.1h \\[R\\]
The receiver core has entered into an error state and should be reset."]
pub type RxCoreStsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - 3:3\\]
Receiver Core Status bitThis bit indicates the status of the receiver core. If this bit is set, the receiver should undergo a reset and subsequent resynchronization with the transmitter. This bit will be always be set when the receiver has detected and end of frame error or a frame type error. This bit can also be set if the receiver becomes corrupted due to noise on the signal lines. If the receiver has experienced a ping watchdog or frame watchdog timeout, this bit should be read to determine if the cause was due to a corrupt transaction, thus putting the receiver core into an unrecoverable state. Only a soft reset will reset the recevier core and thus reset this bit. 0h \\[R\\]
The receiver core is operating normally.1h \\[R\\]
The receiver core has entered into an error state and should be reset."]
    #[inline(always)]
    pub fn rx_core_sts(&self) -> RxCoreStsR {
        RxCoreStsR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - 3:3\\]
Receiver Core Status bitThis bit indicates the status of the receiver core. If this bit is set, the receiver should undergo a reset and subsequent resynchronization with the transmitter. This bit will be always be set when the receiver has detected and end of frame error or a frame type error. This bit can also be set if the receiver becomes corrupted due to noise on the signal lines. If the receiver has experienced a ping watchdog or frame watchdog timeout, this bit should be read to determine if the cause was due to a corrupt transaction, thus putting the receiver core into an unrecoverable state. Only a soft reset will reset the recevier core and thus reset this bit. 0h \\[R\\]
The receiver core is operating normally.1h \\[R\\]
The receiver core has entered into an error state and should be reset."]
    #[inline(always)]
    #[must_use]
    pub fn rx_core_sts(&mut self) -> RxCoreStsW<CfgRxVis1Spec> {
        RxCoreStsW::new(self, 3)
    }
}
#[doc = "Receive debug visibility register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_vis_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_vis_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxVis1Spec;
impl crate::RegisterSpec for CfgRxVis1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_rx_vis_1::R`](R) reader structure"]
impl crate::Readable for CfgRxVis1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_vis_1::W`](W) writer structure"]
impl crate::Writable for CfgRxVis1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_RX_VIS_1 to value 0"]
impl crate::Resettable for CfgRxVis1Spec {
    const RESET_VALUE: u32 = 0;
}
