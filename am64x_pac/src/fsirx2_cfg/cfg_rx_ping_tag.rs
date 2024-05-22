#[doc = "Register `CFG_RX_PING_TAG` reader"]
pub type R = crate::R<CfgRxPingTagSpec>;
#[doc = "Register `CFG_RX_PING_TAG` writer"]
pub type W = crate::W<CfgRxPingTagSpec>;
#[doc = "Field `ZERO` reader - 0:0\\]
Zero bitThis bit will always read as 0. This is intentionally provided to create a 32-bit offset if required. Using the PING_TAG and ZERO bits of this register \\[bits 4:0\\], application software can directly index into an array of 32-bit data."]
pub type ZeroR = crate::BitReader;
#[doc = "Field `ZERO` writer - 0:0\\]
Zero bitThis bit will always read as 0. This is intentionally provided to create a 32-bit offset if required. Using the PING_TAG and ZERO bits of this register \\[bits 4:0\\], application software can directly index into an array of 32-bit data."]
pub type ZeroW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PING_TAG` reader - 4:1\\]
Received Ping Frame TagThis field contains the 4-bit frame tag from the last successfully received ping frame. This is intentionally shifted into bits 4:1 so that the register can be used as a 32-bit address index based on the received tag."]
pub type PingTagR = crate::FieldReader;
#[doc = "Field `PING_TAG` writer - 4:1\\]
Received Ping Frame TagThis field contains the 4-bit frame tag from the last successfully received ping frame. This is intentionally shifted into bits 4:1 so that the register can be used as a 32-bit address index based on the received tag."]
pub type PingTagW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Zero bitThis bit will always read as 0. This is intentionally provided to create a 32-bit offset if required. Using the PING_TAG and ZERO bits of this register \\[bits 4:0\\], application software can directly index into an array of 32-bit data."]
    #[inline(always)]
    pub fn zero(&self) -> ZeroR {
        ZeroR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - 4:1\\]
Received Ping Frame TagThis field contains the 4-bit frame tag from the last successfully received ping frame. This is intentionally shifted into bits 4:1 so that the register can be used as a 32-bit address index based on the received tag."]
    #[inline(always)]
    pub fn ping_tag(&self) -> PingTagR {
        PingTagR::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Zero bitThis bit will always read as 0. This is intentionally provided to create a 32-bit offset if required. Using the PING_TAG and ZERO bits of this register \\[bits 4:0\\], application software can directly index into an array of 32-bit data."]
    #[inline(always)]
    #[must_use]
    pub fn zero(&mut self) -> ZeroW<CfgRxPingTagSpec> {
        ZeroW::new(self, 0)
    }
    #[doc = "Bits 1:4 - 4:1\\]
Received Ping Frame TagThis field contains the 4-bit frame tag from the last successfully received ping frame. This is intentionally shifted into bits 4:1 so that the register can be used as a 32-bit address index based on the received tag."]
    #[inline(always)]
    #[must_use]
    pub fn ping_tag(&mut self) -> PingTagW<CfgRxPingTagSpec> {
        PingTagW::new(self, 1)
    }
}
#[doc = "Receive ping tag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_ping_tag::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_ping_tag::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxPingTagSpec;
impl crate::RegisterSpec for CfgRxPingTagSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_rx_ping_tag::R`](R) reader structure"]
impl crate::Readable for CfgRxPingTagSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_ping_tag::W`](W) writer structure"]
impl crate::Writable for CfgRxPingTagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_RX_PING_TAG to value 0"]
impl crate::Resettable for CfgRxPingTagSpec {
    const RESET_VALUE: u16 = 0;
}
