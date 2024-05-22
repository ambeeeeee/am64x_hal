#[doc = "Register `CFG_TX_FRAME_CTRL` reader"]
pub type R = crate::R<CfgTxFrameCtrlSpec>;
#[doc = "Register `CFG_TX_FRAME_CTRL` writer"]
pub type W = crate::W<CfgTxFrameCtrlSpec>;
#[doc = "Field `FRAME_TYPE` reader - 3:0\\]
Transmit Frame TypeThis field determines the type of frame that will be transmitted next. 0000b \\[R/W\\]
= Ping Frame. This frame can be sent either by software or automatically by hardware.0100b \\[R/W\\]
= DATA_1_WORD Frame. One word data frame \\[16-bit data\\].0101b \\[R/W\\]
= DATA_2_WORD Frame. Two word data frame \\[32-bit data\\].0110b \\[R/W\\]
= DATA_4_WORD Frame. Four word data frame \\[64-bit data\\]. 0111b \\[R/W\\]
= DATA_6_WORD Frame. Six word data frame \\[96-bit data\\].0011b \\[R/W\\]
= DATA_N_WORD Frame. The N_WORDS field will determine the number of words \\[1 to 16\\]
to be sent. Both the transmitter and receiver must have the same value programmed.1111b \\[R/W\\]
= Error Frame. This frame can be used during error conditions or any condition where the transmitter wants to notify the receiver of a high priorty status. However, the user software is at liberty to use this for any purpose. 0001b, 0010b, and 1000b through 1110b are Reserved and should not be used."]
pub type FrameTypeR = crate::FieldReader;
#[doc = "Field `FRAME_TYPE` writer - 3:0\\]
Transmit Frame TypeThis field determines the type of frame that will be transmitted next. 0000b \\[R/W\\]
= Ping Frame. This frame can be sent either by software or automatically by hardware.0100b \\[R/W\\]
= DATA_1_WORD Frame. One word data frame \\[16-bit data\\].0101b \\[R/W\\]
= DATA_2_WORD Frame. Two word data frame \\[32-bit data\\].0110b \\[R/W\\]
= DATA_4_WORD Frame. Four word data frame \\[64-bit data\\]. 0111b \\[R/W\\]
= DATA_6_WORD Frame. Six word data frame \\[96-bit data\\].0011b \\[R/W\\]
= DATA_N_WORD Frame. The N_WORDS field will determine the number of words \\[1 to 16\\]
to be sent. Both the transmitter and receiver must have the same value programmed.1111b \\[R/W\\]
= Error Frame. This frame can be used during error conditions or any condition where the transmitter wants to notify the receiver of a high priorty status. However, the user software is at liberty to use this for any purpose. 0001b, 0010b, and 1000b through 1110b are Reserved and should not be used."]
pub type FrameTypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `N_WORDS` reader - 7:4\\]
Number of Words to be TransmittedThis field defines the number of words which will be transmitted in a DATA_N_WORD frame. This is a user-defined field that must match the corresponding field in the receiver. Set this bitfield to be one less than the number of words to be transmitted. 0h \\[R/W\\]
= 1 data word frame \\[16-bit data\\].1h \\[R/W\\]
= 2 data word frame \\[32-bit data\\]...Fh \\[R/W\\]
= 16 data word frame \\[256-bit data\\]."]
pub type NWordsR = crate::FieldReader;
#[doc = "Field `N_WORDS` writer - 7:4\\]
Number of Words to be TransmittedThis field defines the number of words which will be transmitted in a DATA_N_WORD frame. This is a user-defined field that must match the corresponding field in the receiver. Set this bitfield to be one less than the number of words to be transmitted. 0h \\[R/W\\]
= 1 data word frame \\[16-bit data\\].1h \\[R/W\\]
= 2 data word frame \\[32-bit data\\]...Fh \\[R/W\\]
= 16 data word frame \\[256-bit data\\]."]
pub type NWordsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `START` reader - 15:15\\]
Start Transmission bitThis bit will cause the FSI to start transmitting the next frame. 0h \\[R/W\\]
= Writing a 0 to this bit will have no effect.1h \\[R/W\\]
= Start the next transmission. This bit will be cleared by hardware."]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - 15:15\\]
Start Transmission bitThis bit will cause the FSI to start transmitting the next frame. 0h \\[R/W\\]
= Writing a 0 to this bit will have no effect.1h \\[R/W\\]
= Start the next transmission. This bit will be cleared by hardware."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Transmit Frame TypeThis field determines the type of frame that will be transmitted next. 0000b \\[R/W\\]
= Ping Frame. This frame can be sent either by software or automatically by hardware.0100b \\[R/W\\]
= DATA_1_WORD Frame. One word data frame \\[16-bit data\\].0101b \\[R/W\\]
= DATA_2_WORD Frame. Two word data frame \\[32-bit data\\].0110b \\[R/W\\]
= DATA_4_WORD Frame. Four word data frame \\[64-bit data\\]. 0111b \\[R/W\\]
= DATA_6_WORD Frame. Six word data frame \\[96-bit data\\].0011b \\[R/W\\]
= DATA_N_WORD Frame. The N_WORDS field will determine the number of words \\[1 to 16\\]
to be sent. Both the transmitter and receiver must have the same value programmed.1111b \\[R/W\\]
= Error Frame. This frame can be used during error conditions or any condition where the transmitter wants to notify the receiver of a high priorty status. However, the user software is at liberty to use this for any purpose. 0001b, 0010b, and 1000b through 1110b are Reserved and should not be used."]
    #[inline(always)]
    pub fn frame_type(&self) -> FrameTypeR {
        FrameTypeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Number of Words to be TransmittedThis field defines the number of words which will be transmitted in a DATA_N_WORD frame. This is a user-defined field that must match the corresponding field in the receiver. Set this bitfield to be one less than the number of words to be transmitted. 0h \\[R/W\\]
= 1 data word frame \\[16-bit data\\].1h \\[R/W\\]
= 2 data word frame \\[32-bit data\\]...Fh \\[R/W\\]
= 16 data word frame \\[256-bit data\\]."]
    #[inline(always)]
    pub fn n_words(&self) -> NWordsR {
        NWordsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Start Transmission bitThis bit will cause the FSI to start transmitting the next frame. 0h \\[R/W\\]
= Writing a 0 to this bit will have no effect.1h \\[R/W\\]
= Start the next transmission. This bit will be cleared by hardware."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Transmit Frame TypeThis field determines the type of frame that will be transmitted next. 0000b \\[R/W\\]
= Ping Frame. This frame can be sent either by software or automatically by hardware.0100b \\[R/W\\]
= DATA_1_WORD Frame. One word data frame \\[16-bit data\\].0101b \\[R/W\\]
= DATA_2_WORD Frame. Two word data frame \\[32-bit data\\].0110b \\[R/W\\]
= DATA_4_WORD Frame. Four word data frame \\[64-bit data\\]. 0111b \\[R/W\\]
= DATA_6_WORD Frame. Six word data frame \\[96-bit data\\].0011b \\[R/W\\]
= DATA_N_WORD Frame. The N_WORDS field will determine the number of words \\[1 to 16\\]
to be sent. Both the transmitter and receiver must have the same value programmed.1111b \\[R/W\\]
= Error Frame. This frame can be used during error conditions or any condition where the transmitter wants to notify the receiver of a high priorty status. However, the user software is at liberty to use this for any purpose. 0001b, 0010b, and 1000b through 1110b are Reserved and should not be used."]
    #[inline(always)]
    #[must_use]
    pub fn frame_type(&mut self) -> FrameTypeW<CfgTxFrameCtrlSpec> {
        FrameTypeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Number of Words to be TransmittedThis field defines the number of words which will be transmitted in a DATA_N_WORD frame. This is a user-defined field that must match the corresponding field in the receiver. Set this bitfield to be one less than the number of words to be transmitted. 0h \\[R/W\\]
= 1 data word frame \\[16-bit data\\].1h \\[R/W\\]
= 2 data word frame \\[32-bit data\\]...Fh \\[R/W\\]
= 16 data word frame \\[256-bit data\\]."]
    #[inline(always)]
    #[must_use]
    pub fn n_words(&mut self) -> NWordsW<CfgTxFrameCtrlSpec> {
        NWordsW::new(self, 4)
    }
    #[doc = "Bit 15 - 15:15\\]
Start Transmission bitThis bit will cause the FSI to start transmitting the next frame. 0h \\[R/W\\]
= Writing a 0 to this bit will have no effect.1h \\[R/W\\]
= Start the next transmission. This bit will be cleared by hardware."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<CfgTxFrameCtrlSpec> {
        StartW::new(self, 15)
    }
}
#[doc = "Transmit frame control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_frame_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_frame_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTxFrameCtrlSpec;
impl crate::RegisterSpec for CfgTxFrameCtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_tx_frame_ctrl::R`](R) reader structure"]
impl crate::Readable for CfgTxFrameCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tx_frame_ctrl::W`](W) writer structure"]
impl crate::Writable for CfgTxFrameCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_TX_FRAME_CTRL to value 0"]
impl crate::Resettable for CfgTxFrameCtrlSpec {
    const RESET_VALUE: u16 = 0;
}
