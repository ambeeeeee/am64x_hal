#[doc = "Register `PKTDMA_RCHAN_RCFG` reader"]
pub type R = crate::R<PktdmaRchanRcfgSpec>;
#[doc = "Register `PKTDMA_RCHAN_RCFG` writer"]
pub type W = crate::W<PktdmaRchanRcfgSpec>;
#[doc = "Field `RX_BURST_SIZE` reader - 11:10\\]
Specifies the nominal burst size and alignment for data transfers on this channel. 0,1 = 64 Bytes 2 = 128 Bytes All other values are reserved"]
pub type RxBurstSizeR = crate::FieldReader;
#[doc = "Field `RX_BURST_SIZE` writer - 11:10\\]
Specifies the nominal burst size and alignment for data transfers on this channel. 0,1 = 64 Bytes 2 = 128 Bytes All other values are reserved"]
pub type RxBurstSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX_CHAN_TYPE` reader - 19:16\\]
Rx Channel Type: this field controls and / or indicates the functional channel type for this channel and the work passing mechanism that the channel uses for communicating with the Host. Available channel types are as follows: 2 = Channel performs packet oriented data transfers using pass by reference rings. Channels configured in this mode can only use Host and Monolithic descriptors and the pointers to those descriptors are passed from/to SW using rings in the Ring Accelerator. 3 = Channel performs packet oriented data transfers using pass by reference rings with single buffer packet mode enabled. Channels configured in this mode can only use Host descriptors and each descriptor will be processed as an independent packet (no buffer chaining). This is the only packet oriented mode that can be used with data sources that are infinite streams (no EOP)"]
pub type RxChanTypeR = crate::FieldReader;
#[doc = "Field `RX_CHAN_TYPE` writer - 19:16\\]
Rx Channel Type: this field controls and / or indicates the functional channel type for this channel and the work passing mechanism that the channel uses for communicating with the Host. Available channel types are as follows: 2 = Channel performs packet oriented data transfers using pass by reference rings. Channels configured in this mode can only use Host and Monolithic descriptors and the pointers to those descriptors are passed from/to SW using rings in the Ring Accelerator. 3 = Channel performs packet oriented data transfers using pass by reference rings with single buffer packet mode enabled. Channels configured in this mode can only use Host descriptors and each descriptor will be processed as an independent packet (no buffer chaining). This is the only packet oriented mode that can be used with data sources that are infinite streams (no EOP)"]
pub type RxChanTypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RX_PAUSE_ON_ERR` reader - 31:31\\]
Pause On Error: this field controls what the channel will do if an error or exception occurs during a data transfer. This field is encoded as follows: 0 = Channel will drop current work and move on 1 = Channel will pause and wait for SW to investigate and un-pause the channel."]
pub type RxPauseOnErrR = crate::BitReader;
#[doc = "Field `RX_PAUSE_ON_ERR` writer - 31:31\\]
Pause On Error: this field controls what the channel will do if an error or exception occurs during a data transfer. This field is encoded as follows: 0 = Channel will drop current work and move on 1 = Channel will pause and wait for SW to investigate and un-pause the channel."]
pub type RxPauseOnErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 10:11 - 11:10\\]
Specifies the nominal burst size and alignment for data transfers on this channel. 0,1 = 64 Bytes 2 = 128 Bytes All other values are reserved"]
    #[inline(always)]
    pub fn rx_burst_size(&self) -> RxBurstSizeR {
        RxBurstSizeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Rx Channel Type: this field controls and / or indicates the functional channel type for this channel and the work passing mechanism that the channel uses for communicating with the Host. Available channel types are as follows: 2 = Channel performs packet oriented data transfers using pass by reference rings. Channels configured in this mode can only use Host and Monolithic descriptors and the pointers to those descriptors are passed from/to SW using rings in the Ring Accelerator. 3 = Channel performs packet oriented data transfers using pass by reference rings with single buffer packet mode enabled. Channels configured in this mode can only use Host descriptors and each descriptor will be processed as an independent packet (no buffer chaining). This is the only packet oriented mode that can be used with data sources that are infinite streams (no EOP)"]
    #[inline(always)]
    pub fn rx_chan_type(&self) -> RxChanTypeR {
        RxChanTypeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Pause On Error: this field controls what the channel will do if an error or exception occurs during a data transfer. This field is encoded as follows: 0 = Channel will drop current work and move on 1 = Channel will pause and wait for SW to investigate and un-pause the channel."]
    #[inline(always)]
    pub fn rx_pause_on_err(&self) -> RxPauseOnErrR {
        RxPauseOnErrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 10:11 - 11:10\\]
Specifies the nominal burst size and alignment for data transfers on this channel. 0,1 = 64 Bytes 2 = 128 Bytes All other values are reserved"]
    #[inline(always)]
    #[must_use]
    pub fn rx_burst_size(&mut self) -> RxBurstSizeW<PktdmaRchanRcfgSpec> {
        RxBurstSizeW::new(self, 10)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Rx Channel Type: this field controls and / or indicates the functional channel type for this channel and the work passing mechanism that the channel uses for communicating with the Host. Available channel types are as follows: 2 = Channel performs packet oriented data transfers using pass by reference rings. Channels configured in this mode can only use Host and Monolithic descriptors and the pointers to those descriptors are passed from/to SW using rings in the Ring Accelerator. 3 = Channel performs packet oriented data transfers using pass by reference rings with single buffer packet mode enabled. Channels configured in this mode can only use Host descriptors and each descriptor will be processed as an independent packet (no buffer chaining). This is the only packet oriented mode that can be used with data sources that are infinite streams (no EOP)"]
    #[inline(always)]
    #[must_use]
    pub fn rx_chan_type(&mut self) -> RxChanTypeW<PktdmaRchanRcfgSpec> {
        RxChanTypeW::new(self, 16)
    }
    #[doc = "Bit 31 - 31:31\\]
Pause On Error: this field controls what the channel will do if an error or exception occurs during a data transfer. This field is encoded as follows: 0 = Channel will drop current work and move on 1 = Channel will pause and wait for SW to investigate and un-pause the channel."]
    #[inline(always)]
    #[must_use]
    pub fn rx_pause_on_err(&mut self) -> RxPauseOnErrW<PktdmaRchanRcfgSpec> {
        RxPauseOnErrW::new(self, 31)
    }
}
#[doc = "The Rx Channel Configuration Register is used to initialize static mode settings for the Rx DMA channel. This register may only be written when the channel is disabled (rx_enable in realtime control reg is 0).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rchan_rcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rchan_rcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PktdmaRchanRcfgSpec;
impl crate::RegisterSpec for PktdmaRchanRcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktdma_rchan_rcfg::R`](R) reader structure"]
impl crate::Readable for PktdmaRchanRcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pktdma_rchan_rcfg::W`](W) writer structure"]
impl crate::Writable for PktdmaRchanRcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTDMA_RCHAN_RCFG to value 0x0002_0400"]
impl crate::Resettable for PktdmaRchanRcfgSpec {
    const RESET_VALUE: u32 = 0x0002_0400;
}
