#[doc = "Register `CFG_TX_PING_TAG` reader"]
pub type R = crate::R<CfgTxPingTagSpec>;
#[doc = "Register `CFG_TX_PING_TAG` writer"]
pub type W = crate::W<CfgTxPingTagSpec>;
#[doc = "Field `TAG` reader - 3:0\\]
Ping Frame TagThis field contains a 4-bit tag which will be sent in any ping frame that is initiated by an external trigger or the ping timer. This field is user-defined and can be set based on the application requirement. If a ping frame is generated manually, the transmitted tag will be from TX_FRAME_TAG_UDATA.FRAME_TAG, not this value."]
pub type TagR = crate::FieldReader;
#[doc = "Field `TAG` writer - 3:0\\]
Ping Frame TagThis field contains a 4-bit tag which will be sent in any ping frame that is initiated by an external trigger or the ping timer. This field is user-defined and can be set based on the application requirement. If a ping frame is generated manually, the transmitted tag will be from TX_FRAME_TAG_UDATA.FRAME_TAG, not this value."]
pub type TagW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Ping Frame TagThis field contains a 4-bit tag which will be sent in any ping frame that is initiated by an external trigger or the ping timer. This field is user-defined and can be set based on the application requirement. If a ping frame is generated manually, the transmitted tag will be from TX_FRAME_TAG_UDATA.FRAME_TAG, not this value."]
    #[inline(always)]
    pub fn tag(&self) -> TagR {
        TagR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Ping Frame TagThis field contains a 4-bit tag which will be sent in any ping frame that is initiated by an external trigger or the ping timer. This field is user-defined and can be set based on the application requirement. If a ping frame is generated manually, the transmitted tag will be from TX_FRAME_TAG_UDATA.FRAME_TAG, not this value."]
    #[inline(always)]
    #[must_use]
    pub fn tag(&mut self) -> TagW<CfgTxPingTagSpec> {
        TagW::new(self, 0)
    }
}
#[doc = "Transmit ping tag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_ping_tag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_ping_tag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTxPingTagSpec;
impl crate::RegisterSpec for CfgTxPingTagSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_tx_ping_tag::R`](R) reader structure"]
impl crate::Readable for CfgTxPingTagSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tx_ping_tag::W`](W) writer structure"]
impl crate::Writable for CfgTxPingTagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_TX_PING_TAG to value 0"]
impl crate::Resettable for CfgTxPingTagSpec {
    const RESET_VALUE: u16 = 0;
}
