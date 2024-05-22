#[doc = "Register `BCDMA_TCHANRT_TRT_PEER1` reader"]
pub type R = crate::R<BcdmaTchanrtTrtPeer1Spec>;
#[doc = "Register `BCDMA_TCHANRT_TRT_PEER1` writer"]
pub type W = crate::W<BcdmaTchanrtTrtPeer1Spec>;
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
    pub fn peer_data(&mut self) -> PeerDataW<BcdmaTchanrtTrtPeer1Spec> {
        PeerDataW::new(self, 0)
    }
}
#[doc = "This register provides access to the remote peer's realtime register at 0x401.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchanrt_trt_peer1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchanrt_trt_peer1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaTchanrtTrtPeer1Spec;
impl crate::RegisterSpec for BcdmaTchanrtTrtPeer1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_tchanrt_trt_peer1::R`](R) reader structure"]
impl crate::Readable for BcdmaTchanrtTrtPeer1Spec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_tchanrt_trt_peer1::W`](W) writer structure"]
impl crate::Writable for BcdmaTchanrtTrtPeer1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_TCHANRT_TRT_PEER1 to value 0"]
impl crate::Resettable for BcdmaTchanrtTrtPeer1Spec {
    const RESET_VALUE: u32 = 0;
}
