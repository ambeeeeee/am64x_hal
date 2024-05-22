#[doc = "Register `PKTDMA_GCFG_RFLOWFWSTAT` reader"]
pub type R = crate::R<PktdmaGcfgRflowfwstatSpec>;
#[doc = "Register `PKTDMA_GCFG_RFLOWFWSTAT` writer"]
pub type W = crate::W<PktdmaGcfgRflowfwstatSpec>;
#[doc = "Field `CHANNEL` reader - 8:0\\]
This is the channel number on which the trapped packet was received"]
pub type ChannelR = crate::FieldReader<u16>;
#[doc = "Field `CHANNEL` writer - 8:0\\]
This is the channel number on which the trapped packet was received"]
pub type ChannelW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `FLOWID` reader - 29:16\\]
This is the flow ID that was received on the trapped packet"]
pub type FlowidR = crate::FieldReader<u16>;
#[doc = "Field `FLOWID` writer - 29:16\\]
This is the flow ID that was received on the trapped packet"]
pub type FlowidW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `PEND` reader - 31:31\\]
This bit is set whenever the Flow ID firewall detects a Flow ID is out of range for an incoming packet. Once this bit is set, the remaining fields in this register will not be modified. SW is required to write this bit to 0 to allow another exception to be captured."]
pub type PendR = crate::BitReader;
#[doc = "Field `PEND` writer - 31:31\\]
This bit is set whenever the Flow ID firewall detects a Flow ID is out of range for an incoming packet. Once this bit is set, the remaining fields in this register will not be modified. SW is required to write this bit to 0 to allow another exception to be captured."]
pub type PendW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - 8:0\\]
This is the channel number on which the trapped packet was received"]
    #[inline(always)]
    pub fn channel(&self) -> ChannelR {
        ChannelR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:29 - 29:16\\]
This is the flow ID that was received on the trapped packet"]
    #[inline(always)]
    pub fn flowid(&self) -> FlowidR {
        FlowidR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
This bit is set whenever the Flow ID firewall detects a Flow ID is out of range for an incoming packet. Once this bit is set, the remaining fields in this register will not be modified. SW is required to write this bit to 0 to allow another exception to be captured."]
    #[inline(always)]
    pub fn pend(&self) -> PendR {
        PendR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - 8:0\\]
This is the channel number on which the trapped packet was received"]
    #[inline(always)]
    #[must_use]
    pub fn channel(&mut self) -> ChannelW<PktdmaGcfgRflowfwstatSpec> {
        ChannelW::new(self, 0)
    }
    #[doc = "Bits 16:29 - 29:16\\]
This is the flow ID that was received on the trapped packet"]
    #[inline(always)]
    #[must_use]
    pub fn flowid(&mut self) -> FlowidW<PktdmaGcfgRflowfwstatSpec> {
        FlowidW::new(self, 16)
    }
    #[doc = "Bit 31 - 31:31\\]
This bit is set whenever the Flow ID firewall detects a Flow ID is out of range for an incoming packet. Once this bit is set, the remaining fields in this register will not be modified. SW is required to write this bit to 0 to allow another exception to be captured."]
    #[inline(always)]
    #[must_use]
    pub fn pend(&mut self) -> PendW<PktdmaGcfgRflowfwstatSpec> {
        PendW::new(self, 31)
    }
}
#[doc = "The Rx Flow FW Status Register 0 captures information about the thread/channel and received flow ID which failed a range check. Values in this register will remain persistent once an exception has been detected until the pend bit is written back to 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_gcfg_rflowfwstat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_gcfg_rflowfwstat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PktdmaGcfgRflowfwstatSpec;
impl crate::RegisterSpec for PktdmaGcfgRflowfwstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktdma_gcfg_rflowfwstat::R`](R) reader structure"]
impl crate::Readable for PktdmaGcfgRflowfwstatSpec {}
#[doc = "`write(|w| ..)` method takes [`pktdma_gcfg_rflowfwstat::W`](W) writer structure"]
impl crate::Writable for PktdmaGcfgRflowfwstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTDMA_GCFG_RFLOWFWSTAT to value 0"]
impl crate::Resettable for PktdmaGcfgRflowfwstatSpec {
    const RESET_VALUE: u32 = 0;
}
