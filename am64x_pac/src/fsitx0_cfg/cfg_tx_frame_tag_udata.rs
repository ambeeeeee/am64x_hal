#[doc = "Register `CFG_TX_FRAME_TAG_UDATA` reader"]
pub type R = crate::R<CfgTxFrameTagUdataSpec>;
#[doc = "Register `CFG_TX_FRAME_TAG_UDATA` writer"]
pub type W = crate::W<CfgTxFrameTagUdataSpec>;
#[doc = "Field `FRAME_TAG` reader - 3:0\\]
This will be used only for software initiated transmissions.Frame tag bitsThis is a user-defined value that will be loaded into the frame tag phase of the next transmission. The receiver may use the frame tag for any application need. This value will not impact any hardware behavior For external triggers do not use this register. Use the TX_PING_TAG register instead."]
pub type FrameTagR = crate::FieldReader;
#[doc = "Field `FRAME_TAG` writer - 3:0\\]
This will be used only for software initiated transmissions.Frame tag bitsThis is a user-defined value that will be loaded into the frame tag phase of the next transmission. The receiver may use the frame tag for any application need. This value will not impact any hardware behavior For external triggers do not use this register. Use the TX_PING_TAG register instead."]
pub type FrameTagW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USER_DATA` reader - 15:8\\]
User Data bitsThis is a user-defined value that will be loaded into the the user data phase of the frame. This 8-bit value can be used by the receiver for any application need. This value will not impact any hardware behavior."]
pub type UserDataR = crate::FieldReader;
#[doc = "Field `USER_DATA` writer - 15:8\\]
User Data bitsThis is a user-defined value that will be loaded into the the user data phase of the frame. This 8-bit value can be used by the receiver for any application need. This value will not impact any hardware behavior."]
pub type UserDataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
This will be used only for software initiated transmissions.Frame tag bitsThis is a user-defined value that will be loaded into the frame tag phase of the next transmission. The receiver may use the frame tag for any application need. This value will not impact any hardware behavior For external triggers do not use this register. Use the TX_PING_TAG register instead."]
    #[inline(always)]
    pub fn frame_tag(&self) -> FrameTagR {
        FrameTagR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
User Data bitsThis is a user-defined value that will be loaded into the the user data phase of the frame. This 8-bit value can be used by the receiver for any application need. This value will not impact any hardware behavior."]
    #[inline(always)]
    pub fn user_data(&self) -> UserDataR {
        UserDataR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
This will be used only for software initiated transmissions.Frame tag bitsThis is a user-defined value that will be loaded into the frame tag phase of the next transmission. The receiver may use the frame tag for any application need. This value will not impact any hardware behavior For external triggers do not use this register. Use the TX_PING_TAG register instead."]
    #[inline(always)]
    #[must_use]
    pub fn frame_tag(&mut self) -> FrameTagW<CfgTxFrameTagUdataSpec> {
        FrameTagW::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
User Data bitsThis is a user-defined value that will be loaded into the the user data phase of the frame. This 8-bit value can be used by the receiver for any application need. This value will not impact any hardware behavior."]
    #[inline(always)]
    #[must_use]
    pub fn user_data(&mut self) -> UserDataW<CfgTxFrameTagUdataSpec> {
        UserDataW::new(self, 8)
    }
}
#[doc = "Transmit frame tag and user data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_frame_tag_udata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_frame_tag_udata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTxFrameTagUdataSpec;
impl crate::RegisterSpec for CfgTxFrameTagUdataSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_tx_frame_tag_udata::R`](R) reader structure"]
impl crate::Readable for CfgTxFrameTagUdataSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tx_frame_tag_udata::W`](W) writer structure"]
impl crate::Writable for CfgTxFrameTagUdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_TX_FRAME_TAG_UDATA to value 0"]
impl crate::Resettable for CfgTxFrameTagUdataSpec {
    const RESET_VALUE: u16 = 0;
}
