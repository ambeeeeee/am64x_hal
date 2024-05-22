#[doc = "Register `CFG_RX_FRAME_TAG_UDATA` reader"]
pub type R = crate::R<CfgRxFrameTagUdataSpec>;
#[doc = "Register `CFG_RX_FRAME_TAG_UDATA` writer"]
pub type W = crate::W<CfgRxFrameTagUdataSpec>;
#[doc = "Field `ZERO` reader - 0:0\\]
Zero bitThis bit will always read as 0. This is intentionally provided to create a 32-bit offset if required. Using the FRAME_TAG and ZERO bits of this register \\[bits 4:0\\], application software can directly index into an array of 32-bit data."]
pub type ZeroR = crate::BitReader;
#[doc = "Field `ZERO` writer - 0:0\\]
Zero bitThis bit will always read as 0. This is intentionally provided to create a 32-bit offset if required. Using the FRAME_TAG and ZERO bits of this register \\[bits 4:0\\], application software can directly index into an array of 32-bit data."]
pub type ZeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAME_TAG` reader - 4:1\\]
Received Frame TagThis field contains the 4-bit frame tag from the last successfully received frame. This is intentionally shifted into bits 4:1 so that the register can be used as a 32-bit address index based on the received tag."]
pub type FrameTagR = crate::FieldReader;
#[doc = "Field `FRAME_TAG` writer - 4:1\\]
Received Frame TagThis field contains the 4-bit frame tag from the last successfully received frame. This is intentionally shifted into bits 4:1 so that the register can be used as a 32-bit address index based on the received tag."]
pub type FrameTagW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USER_DATA` reader - 15:8\\]
Received User DataThis field contains the 8-bit user data field of the last successfully received frame."]
pub type UserDataR = crate::FieldReader;
#[doc = "Field `USER_DATA` writer - 15:8\\]
Received User DataThis field contains the 8-bit user data field of the last successfully received frame."]
pub type UserDataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Zero bitThis bit will always read as 0. This is intentionally provided to create a 32-bit offset if required. Using the FRAME_TAG and ZERO bits of this register \\[bits 4:0\\], application software can directly index into an array of 32-bit data."]
    #[inline(always)]
    pub fn zero(&self) -> ZeroR {
        ZeroR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - 4:1\\]
Received Frame TagThis field contains the 4-bit frame tag from the last successfully received frame. This is intentionally shifted into bits 4:1 so that the register can be used as a 32-bit address index based on the received tag."]
    #[inline(always)]
    pub fn frame_tag(&self) -> FrameTagR {
        FrameTagR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Received User DataThis field contains the 8-bit user data field of the last successfully received frame."]
    #[inline(always)]
    pub fn user_data(&self) -> UserDataR {
        UserDataR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Zero bitThis bit will always read as 0. This is intentionally provided to create a 32-bit offset if required. Using the FRAME_TAG and ZERO bits of this register \\[bits 4:0\\], application software can directly index into an array of 32-bit data."]
    #[inline(always)]
    #[must_use]
    pub fn zero(&mut self) -> ZeroW<CfgRxFrameTagUdataSpec> {
        ZeroW::new(self, 0)
    }
    #[doc = "Bits 1:4 - 4:1\\]
Received Frame TagThis field contains the 4-bit frame tag from the last successfully received frame. This is intentionally shifted into bits 4:1 so that the register can be used as a 32-bit address index based on the received tag."]
    #[inline(always)]
    #[must_use]
    pub fn frame_tag(&mut self) -> FrameTagW<CfgRxFrameTagUdataSpec> {
        FrameTagW::new(self, 1)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Received User DataThis field contains the 8-bit user data field of the last successfully received frame."]
    #[inline(always)]
    #[must_use]
    pub fn user_data(&mut self) -> UserDataW<CfgRxFrameTagUdataSpec> {
        UserDataW::new(self, 8)
    }
}
#[doc = "Receive frame tag and user data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_frame_tag_udata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_frame_tag_udata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxFrameTagUdataSpec;
impl crate::RegisterSpec for CfgRxFrameTagUdataSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_rx_frame_tag_udata::R`](R) reader structure"]
impl crate::Readable for CfgRxFrameTagUdataSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_frame_tag_udata::W`](W) writer structure"]
impl crate::Writable for CfgRxFrameTagUdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_RX_FRAME_TAG_UDATA to value 0"]
impl crate::Resettable for CfgRxFrameTagUdataSpec {
    const RESET_VALUE: u16 = 0;
}
