#[doc = "Register `CFG_RX_FRAME_INFO` reader"]
pub type R = crate::R<CfgRxFrameInfoSpec>;
#[doc = "Register `CFG_RX_FRAME_INFO` writer"]
pub type W = crate::W<CfgRxFrameInfoSpec>;
#[doc = "Field `FRAME_TYPE` reader - 3:0\\]
Received Frame TypeThis field indicates the type of frame that was successfully received last. 0000b \\[R/W\\]
= A ping frame was received0100b \\[R/W\\]
= A DATA_1_WORD frame was received \\[16-bit data\\].0101b \\[R/W\\]
= A DATA_2_WORD frame was received \\[32-bit data\\].0110b \\[R/W\\]
= A DATA_4_WORD frame was received \\[64-bit data\\]. 0111b \\[R/W\\]
= A DATA_6_WORD frame was received \\[96-bit data\\].0011b \\[R/W\\]
= A DATA_N_WORD frame was received. The N_WORD field will determine the number of words \\[1 to 16\\]
to be sent. The number of words received must equal the value programmed in RX_OPER_CTRL.N_WORDS. 1111b \\[R/W\\]
= An error frame was received. This frame can be used during error conditions or any condition where the transmitter wants to signal the receiver for attention. However, the user software is at liberty to use this for any purpose. 0001b, 0010b, and 1000b through 1110b are Reserved and should not be used."]
pub type FrameTypeR = crate::FieldReader;
#[doc = "Field `FRAME_TYPE` writer - 3:0\\]
Received Frame TypeThis field indicates the type of frame that was successfully received last. 0000b \\[R/W\\]
= A ping frame was received0100b \\[R/W\\]
= A DATA_1_WORD frame was received \\[16-bit data\\].0101b \\[R/W\\]
= A DATA_2_WORD frame was received \\[32-bit data\\].0110b \\[R/W\\]
= A DATA_4_WORD frame was received \\[64-bit data\\]. 0111b \\[R/W\\]
= A DATA_6_WORD frame was received \\[96-bit data\\].0011b \\[R/W\\]
= A DATA_N_WORD frame was received. The N_WORD field will determine the number of words \\[1 to 16\\]
to be sent. The number of words received must equal the value programmed in RX_OPER_CTRL.N_WORDS. 1111b \\[R/W\\]
= An error frame was received. This frame can be used during error conditions or any condition where the transmitter wants to signal the receiver for attention. However, the user software is at liberty to use this for any purpose. 0001b, 0010b, and 1000b through 1110b are Reserved and should not be used."]
pub type FrameTypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Received Frame TypeThis field indicates the type of frame that was successfully received last. 0000b \\[R/W\\]
= A ping frame was received0100b \\[R/W\\]
= A DATA_1_WORD frame was received \\[16-bit data\\].0101b \\[R/W\\]
= A DATA_2_WORD frame was received \\[32-bit data\\].0110b \\[R/W\\]
= A DATA_4_WORD frame was received \\[64-bit data\\]. 0111b \\[R/W\\]
= A DATA_6_WORD frame was received \\[96-bit data\\].0011b \\[R/W\\]
= A DATA_N_WORD frame was received. The N_WORD field will determine the number of words \\[1 to 16\\]
to be sent. The number of words received must equal the value programmed in RX_OPER_CTRL.N_WORDS. 1111b \\[R/W\\]
= An error frame was received. This frame can be used during error conditions or any condition where the transmitter wants to signal the receiver for attention. However, the user software is at liberty to use this for any purpose. 0001b, 0010b, and 1000b through 1110b are Reserved and should not be used."]
    #[inline(always)]
    pub fn frame_type(&self) -> FrameTypeR {
        FrameTypeR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Received Frame TypeThis field indicates the type of frame that was successfully received last. 0000b \\[R/W\\]
= A ping frame was received0100b \\[R/W\\]
= A DATA_1_WORD frame was received \\[16-bit data\\].0101b \\[R/W\\]
= A DATA_2_WORD frame was received \\[32-bit data\\].0110b \\[R/W\\]
= A DATA_4_WORD frame was received \\[64-bit data\\]. 0111b \\[R/W\\]
= A DATA_6_WORD frame was received \\[96-bit data\\].0011b \\[R/W\\]
= A DATA_N_WORD frame was received. The N_WORD field will determine the number of words \\[1 to 16\\]
to be sent. The number of words received must equal the value programmed in RX_OPER_CTRL.N_WORDS. 1111b \\[R/W\\]
= An error frame was received. This frame can be used during error conditions or any condition where the transmitter wants to signal the receiver for attention. However, the user software is at liberty to use this for any purpose. 0001b, 0010b, and 1000b through 1110b are Reserved and should not be used."]
    #[inline(always)]
    #[must_use]
    pub fn frame_type(&mut self) -> FrameTypeW<CfgRxFrameInfoSpec> {
        FrameTypeW::new(self, 0)
    }
}
#[doc = "Receive frame control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_frame_info::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_frame_info::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxFrameInfoSpec;
impl crate::RegisterSpec for CfgRxFrameInfoSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_rx_frame_info::R`](R) reader structure"]
impl crate::Readable for CfgRxFrameInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_frame_info::W`](W) writer structure"]
impl crate::Writable for CfgRxFrameInfoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_RX_FRAME_INFO to value 0"]
impl crate::Resettable for CfgRxFrameInfoSpec {
    const RESET_VALUE: u16 = 0;
}
