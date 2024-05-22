#[doc = "Register `PKTDMA_TCHANRT_TRT_PEER13` reader"]
pub type R = crate::R<PktdmaTchanrtTrtPeer13Spec>;
#[doc = "Register `PKTDMA_TCHANRT_TRT_PEER13` writer"]
pub type W = crate::W<PktdmaTchanrtTrtPeer13Spec>;
#[doc = "Field `PEER_DATA` reader - 31:0\\]
Peer realtime register data (varies by paired peer)."]
pub type PeerDataR = crate::FieldReader<u32>;
#[doc = "Field `PEER_DATA` writer - 31:0\\]
Peer realtime register data (varies by paired peer)."]
pub type PeerDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Peer realtime register data (varies by paired peer)."]
    #[inline(always)]
    pub fn peer_data(&self) -> PeerDataR {
        PeerDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Peer realtime register data (varies by paired peer)."]
    #[inline(always)]
    #[must_use]
    pub fn peer_data(&mut self) -> PeerDataW<PktdmaTchanrtTrtPeer13Spec> {
        PeerDataW::new(self, 0)
    }
}
#[doc = "This register provides access to the remote peer's realtime register at 0x40D.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchanrt_trt_peer13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchanrt_trt_peer13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PktdmaTchanrtTrtPeer13Spec;
impl crate::RegisterSpec for PktdmaTchanrtTrtPeer13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktdma_tchanrt_trt_peer13::R`](R) reader structure"]
impl crate::Readable for PktdmaTchanrtTrtPeer13Spec {}
#[doc = "`write(|w| ..)` method takes [`pktdma_tchanrt_trt_peer13::W`](W) writer structure"]
impl crate::Writable for PktdmaTchanrtTrtPeer13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTDMA_TCHANRT_TRT_PEER13 to value 0"]
impl crate::Resettable for PktdmaTchanrtTrtPeer13Spec {
    const RESET_VALUE: u32 = 0;
}
