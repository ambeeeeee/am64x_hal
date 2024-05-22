#[doc = "Register `CFG_RX_PING_TAG_CMP` reader"]
pub type R = crate::R<CfgRxPingTagCmpSpec>;
#[doc = "Register `CFG_RX_PING_TAG_CMP` writer"]
pub type W = crate::W<CfgRxPingTagCmpSpec>;
#[doc = "Field `TAG_REF` reader - 3:0\\]
Ping Tag ReferenceThe reference tag to check against when comparing the TAG_MASK and the incoming ping tag. This reference value is used only for ping frames."]
pub type TagRefR = crate::FieldReader;
#[doc = "Field `TAG_REF` writer - 3:0\\]
Ping Tag ReferenceThe reference tag to check against when comparing the TAG_MASK and the incoming ping tag. This reference value is used only for ping frames."]
pub type TagRefW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TAG_MASK` reader - 7:4\\]
Ping Tag MaskAny bit position in this register set to 0 will be used in the comparison of the incoming ping frame tag and the value stored in TAG_REF. A bit position set to 1 will be ignored in the tag comparison. This mask value is used only for ping frames."]
pub type TagMaskR = crate::FieldReader;
#[doc = "Field `TAG_MASK` writer - 7:4\\]
Ping Tag MaskAny bit position in this register set to 0 will be used in the comparison of the incoming ping frame tag and the value stored in TAG_REF. A bit position set to 1 will be ignored in the tag comparison. This mask value is used only for ping frames."]
pub type TagMaskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CMP_EN` reader - 8:8\\]
Ping Tag Compare Enable bitSet this bit to enable the comparison of an incoming ping tag and the value stored in the ping tag reference. A match caused by the comparison of TAG_MASK, TAG_REF, and the incoming ping tag will trigger a ping frame tag match event. 0h \\[R/W\\]
Ping tag comparison is disabled.1h \\[R/W\\]
Ping tag comparison is enabled."]
pub type CmpEnR = crate::BitReader;
#[doc = "Field `CMP_EN` writer - 8:8\\]
Ping Tag Compare Enable bitSet this bit to enable the comparison of an incoming ping tag and the value stored in the ping tag reference. A match caused by the comparison of TAG_MASK, TAG_REF, and the incoming ping tag will trigger a ping frame tag match event. 0h \\[R/W\\]
Ping tag comparison is disabled.1h \\[R/W\\]
Ping tag comparison is enabled."]
pub type CmpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BROADCAST_EN` reader - 9:9\\]
Broadcast Enable bitThis will enable the reception of a ping frame broadcast. When this bit is set, bit 3 of the received tag will be treated as a broadcast notification. If bit 3 of the received tag is set to 1, a ping tag match event will be triggered regardless of the. A match caused by the comparison of TAG_MASK and TAG_REF will still be considered a match and the ping tag match event will be triggered as normal This bit only takes effect only if CMP_EN is set to 1. 0h \\[R/W\\]
Broadcast frame match disabled.1h \\[R/W\\]
Broadcast frame match enabled."]
pub type BroadcastEnR = crate::BitReader;
#[doc = "Field `BROADCAST_EN` writer - 9:9\\]
Broadcast Enable bitThis will enable the reception of a ping frame broadcast. When this bit is set, bit 3 of the received tag will be treated as a broadcast notification. If bit 3 of the received tag is set to 1, a ping tag match event will be triggered regardless of the. A match caused by the comparison of TAG_MASK and TAG_REF will still be considered a match and the ping tag match event will be triggered as normal This bit only takes effect only if CMP_EN is set to 1. 0h \\[R/W\\]
Broadcast frame match disabled.1h \\[R/W\\]
Broadcast frame match enabled."]
pub type BroadcastEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Ping Tag ReferenceThe reference tag to check against when comparing the TAG_MASK and the incoming ping tag. This reference value is used only for ping frames."]
    #[inline(always)]
    pub fn tag_ref(&self) -> TagRefR {
        TagRefR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Ping Tag MaskAny bit position in this register set to 0 will be used in the comparison of the incoming ping frame tag and the value stored in TAG_REF. A bit position set to 1 will be ignored in the tag comparison. This mask value is used only for ping frames."]
    #[inline(always)]
    pub fn tag_mask(&self) -> TagMaskR {
        TagMaskR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Ping Tag Compare Enable bitSet this bit to enable the comparison of an incoming ping tag and the value stored in the ping tag reference. A match caused by the comparison of TAG_MASK, TAG_REF, and the incoming ping tag will trigger a ping frame tag match event. 0h \\[R/W\\]
Ping tag comparison is disabled.1h \\[R/W\\]
Ping tag comparison is enabled."]
    #[inline(always)]
    pub fn cmp_en(&self) -> CmpEnR {
        CmpEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Broadcast Enable bitThis will enable the reception of a ping frame broadcast. When this bit is set, bit 3 of the received tag will be treated as a broadcast notification. If bit 3 of the received tag is set to 1, a ping tag match event will be triggered regardless of the. A match caused by the comparison of TAG_MASK and TAG_REF will still be considered a match and the ping tag match event will be triggered as normal This bit only takes effect only if CMP_EN is set to 1. 0h \\[R/W\\]
Broadcast frame match disabled.1h \\[R/W\\]
Broadcast frame match enabled."]
    #[inline(always)]
    pub fn broadcast_en(&self) -> BroadcastEnR {
        BroadcastEnR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Ping Tag ReferenceThe reference tag to check against when comparing the TAG_MASK and the incoming ping tag. This reference value is used only for ping frames."]
    #[inline(always)]
    #[must_use]
    pub fn tag_ref(&mut self) -> TagRefW<CfgRxPingTagCmpSpec> {
        TagRefW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
Ping Tag MaskAny bit position in this register set to 0 will be used in the comparison of the incoming ping frame tag and the value stored in TAG_REF. A bit position set to 1 will be ignored in the tag comparison. This mask value is used only for ping frames."]
    #[inline(always)]
    #[must_use]
    pub fn tag_mask(&mut self) -> TagMaskW<CfgRxPingTagCmpSpec> {
        TagMaskW::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
Ping Tag Compare Enable bitSet this bit to enable the comparison of an incoming ping tag and the value stored in the ping tag reference. A match caused by the comparison of TAG_MASK, TAG_REF, and the incoming ping tag will trigger a ping frame tag match event. 0h \\[R/W\\]
Ping tag comparison is disabled.1h \\[R/W\\]
Ping tag comparison is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cmp_en(&mut self) -> CmpEnW<CfgRxPingTagCmpSpec> {
        CmpEnW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Broadcast Enable bitThis will enable the reception of a ping frame broadcast. When this bit is set, bit 3 of the received tag will be treated as a broadcast notification. If bit 3 of the received tag is set to 1, a ping tag match event will be triggered regardless of the. A match caused by the comparison of TAG_MASK and TAG_REF will still be considered a match and the ping tag match event will be triggered as normal This bit only takes effect only if CMP_EN is set to 1. 0h \\[R/W\\]
Broadcast frame match disabled.1h \\[R/W\\]
Broadcast frame match enabled."]
    #[inline(always)]
    #[must_use]
    pub fn broadcast_en(&mut self) -> BroadcastEnW<CfgRxPingTagCmpSpec> {
        BroadcastEnW::new(self, 9)
    }
}
#[doc = "Receive ping tag compare register. Protected by LOCK field in RX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_ping_tag_cmp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_ping_tag_cmp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxPingTagCmpSpec;
impl crate::RegisterSpec for CfgRxPingTagCmpSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_rx_ping_tag_cmp::R`](R) reader structure"]
impl crate::Readable for CfgRxPingTagCmpSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_ping_tag_cmp::W`](W) writer structure"]
impl crate::Writable for CfgRxPingTagCmpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_RX_PING_TAG_CMP to value 0"]
impl crate::Resettable for CfgRxPingTagCmpSpec {
    const RESET_VALUE: u16 = 0;
}
