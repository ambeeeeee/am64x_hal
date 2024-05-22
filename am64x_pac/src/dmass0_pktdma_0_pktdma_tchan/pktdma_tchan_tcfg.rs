#[doc = "Register `PKTDMA_TCHAN_TCFG` reader"]
pub type R = crate::R<PktdmaTchanTcfgSpec>;
#[doc = "Register `PKTDMA_TCHAN_TCFG` writer"]
pub type W = crate::W<PktdmaTchanTcfgSpec>;
#[doc = "Field `TX_NOTDPKT` reader - 8:8\\]
Specifies whether or not the channel should suppress sending the single data phase teardown packet when teardown is complete. 0 = TD packet is sent 1 = Suppress sending TD packet"]
pub type TxNotdpktR = crate::BitReader;
#[doc = "Field `TX_NOTDPKT` writer - 8:8\\]
Specifies whether or not the channel should suppress sending the single data phase teardown packet when teardown is complete. 0 = TD packet is sent 1 = Suppress sending TD packet"]
pub type TxNotdpktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_TDTYPE` reader - 9:9\\]
Specifies whether or not the channel should immediately return a teardown completion response to the default completion queue or wait until a status message is returned from the remote PSI-L paired peripheral. 0 = return immediately once all traffic is complete in PKTDMA. 1 = wait until remote peer sends back a completion message."]
pub type TxTdtypeR = crate::BitReader;
#[doc = "Field `TX_TDTYPE` writer - 9:9\\]
Specifies whether or not the channel should immediately return a teardown completion response to the default completion queue or wait until a status message is returned from the remote PSI-L paired peripheral. 0 = return immediately once all traffic is complete in PKTDMA. 1 = wait until remote peer sends back a completion message."]
pub type TxTdtypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_BURST_SIZE` reader - 11:10\\]
Specifies the nominal burst size and alignment for data transfers on this channel. 0,1 = 64 Bytes 2 = 128 Bytes All other values are reserved"]
pub type TxBurstSizeR = crate::FieldReader;
#[doc = "Field `TX_BURST_SIZE` writer - 11:10\\]
Specifies the nominal burst size and alignment for data transfers on this channel. 0,1 = 64 Bytes 2 = 128 Bytes All other values are reserved"]
pub type TxBurstSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_CHAN_TYPE` reader - 19:16\\]
Tx Channel Type: this field controls and / or indicates the functional channel type for this channel and the work passing mechanism that the channel uses for communicating with the Host. Available channel types are as follows: 2 = Channel performs packet oriented data transfers using pass by reference rings. Channels configured in this mode can only use Host and Monolithic descriptors and the pointers to those descriptors are passed from/to SW using rings in the Ring Accelerator."]
pub type TxChanTypeR = crate::FieldReader;
#[doc = "Field `TX_CHAN_TYPE` writer - 19:16\\]
Tx Channel Type: this field controls and / or indicates the functional channel type for this channel and the work passing mechanism that the channel uses for communicating with the Host. Available channel types are as follows: 2 = Channel performs packet oriented data transfers using pass by reference rings. Channels configured in this mode can only use Host and Monolithic descriptors and the pointers to those descriptors are passed from/to SW using rings in the Ring Accelerator."]
pub type TxChanTypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TX_FILT_PSWORDS` reader - 29:29\\]
This field controls whether or not the DMA controller will pass the protocol specific words (if present) from the descriptor to the back end application. This field is encoded as follows: 0=DMA controller will pass PS words if present in descriptor 1=DMA controller will filter PS words."]
pub type TxFiltPswordsR = crate::BitReader;
#[doc = "Field `TX_FILT_PSWORDS` writer - 29:29\\]
This field controls whether or not the DMA controller will pass the protocol specific words (if present) from the descriptor to the back end application. This field is encoded as follows: 0=DMA controller will pass PS words if present in descriptor 1=DMA controller will filter PS words."]
pub type TxFiltPswordsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_FILT_EINFO` reader - 30:30\\]
This field controls whether or not the DMA controller will pass the extended packet information fields (if present) from the descriptor to the back end application. This field is encoded as follows: 0=DMA controller will pass extended packet info words if they are present in the descriptor 1=DMA controller will filter extended packet info words."]
pub type TxFiltEinfoR = crate::BitReader;
#[doc = "Field `TX_FILT_EINFO` writer - 30:30\\]
This field controls whether or not the DMA controller will pass the extended packet information fields (if present) from the descriptor to the back end application. This field is encoded as follows: 0=DMA controller will pass extended packet info words if they are present in the descriptor 1=DMA controller will filter extended packet info words."]
pub type TxFiltEinfoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_PAUSE_ON_ERR` reader - 31:31\\]
Pause On Error: this field controls what the channel will do if an error or exception occurs during a data transfer. This field is encoded as follows: 0 = Channel will drop current work and move on 1 = Channel will pause and wait for SW to investigate and un-pause the channel."]
pub type TxPauseOnErrR = crate::BitReader;
#[doc = "Field `TX_PAUSE_ON_ERR` writer - 31:31\\]
Pause On Error: this field controls what the channel will do if an error or exception occurs during a data transfer. This field is encoded as follows: 0 = Channel will drop current work and move on 1 = Channel will pause and wait for SW to investigate and un-pause the channel."]
pub type TxPauseOnErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - 8:8\\]
Specifies whether or not the channel should suppress sending the single data phase teardown packet when teardown is complete. 0 = TD packet is sent 1 = Suppress sending TD packet"]
    #[inline(always)]
    pub fn tx_notdpkt(&self) -> TxNotdpktR {
        TxNotdpktR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Specifies whether or not the channel should immediately return a teardown completion response to the default completion queue or wait until a status message is returned from the remote PSI-L paired peripheral. 0 = return immediately once all traffic is complete in PKTDMA. 1 = wait until remote peer sends back a completion message."]
    #[inline(always)]
    pub fn tx_tdtype(&self) -> TxTdtypeR {
        TxTdtypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Specifies the nominal burst size and alignment for data transfers on this channel. 0,1 = 64 Bytes 2 = 128 Bytes All other values are reserved"]
    #[inline(always)]
    pub fn tx_burst_size(&self) -> TxBurstSizeR {
        TxBurstSizeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Tx Channel Type: this field controls and / or indicates the functional channel type for this channel and the work passing mechanism that the channel uses for communicating with the Host. Available channel types are as follows: 2 = Channel performs packet oriented data transfers using pass by reference rings. Channels configured in this mode can only use Host and Monolithic descriptors and the pointers to those descriptors are passed from/to SW using rings in the Ring Accelerator."]
    #[inline(always)]
    pub fn tx_chan_type(&self) -> TxChanTypeR {
        TxChanTypeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - 29:29\\]
This field controls whether or not the DMA controller will pass the protocol specific words (if present) from the descriptor to the back end application. This field is encoded as follows: 0=DMA controller will pass PS words if present in descriptor 1=DMA controller will filter PS words."]
    #[inline(always)]
    pub fn tx_filt_pswords(&self) -> TxFiltPswordsR {
        TxFiltPswordsR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
This field controls whether or not the DMA controller will pass the extended packet information fields (if present) from the descriptor to the back end application. This field is encoded as follows: 0=DMA controller will pass extended packet info words if they are present in the descriptor 1=DMA controller will filter extended packet info words."]
    #[inline(always)]
    pub fn tx_filt_einfo(&self) -> TxFiltEinfoR {
        TxFiltEinfoR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Pause On Error: this field controls what the channel will do if an error or exception occurs during a data transfer. This field is encoded as follows: 0 = Channel will drop current work and move on 1 = Channel will pause and wait for SW to investigate and un-pause the channel."]
    #[inline(always)]
    pub fn tx_pause_on_err(&self) -> TxPauseOnErrR {
        TxPauseOnErrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - 8:8\\]
Specifies whether or not the channel should suppress sending the single data phase teardown packet when teardown is complete. 0 = TD packet is sent 1 = Suppress sending TD packet"]
    #[inline(always)]
    #[must_use]
    pub fn tx_notdpkt(&mut self) -> TxNotdpktW<PktdmaTchanTcfgSpec> {
        TxNotdpktW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Specifies whether or not the channel should immediately return a teardown completion response to the default completion queue or wait until a status message is returned from the remote PSI-L paired peripheral. 0 = return immediately once all traffic is complete in PKTDMA. 1 = wait until remote peer sends back a completion message."]
    #[inline(always)]
    #[must_use]
    pub fn tx_tdtype(&mut self) -> TxTdtypeW<PktdmaTchanTcfgSpec> {
        TxTdtypeW::new(self, 9)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Specifies the nominal burst size and alignment for data transfers on this channel. 0,1 = 64 Bytes 2 = 128 Bytes All other values are reserved"]
    #[inline(always)]
    #[must_use]
    pub fn tx_burst_size(&mut self) -> TxBurstSizeW<PktdmaTchanTcfgSpec> {
        TxBurstSizeW::new(self, 10)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Tx Channel Type: this field controls and / or indicates the functional channel type for this channel and the work passing mechanism that the channel uses for communicating with the Host. Available channel types are as follows: 2 = Channel performs packet oriented data transfers using pass by reference rings. Channels configured in this mode can only use Host and Monolithic descriptors and the pointers to those descriptors are passed from/to SW using rings in the Ring Accelerator."]
    #[inline(always)]
    #[must_use]
    pub fn tx_chan_type(&mut self) -> TxChanTypeW<PktdmaTchanTcfgSpec> {
        TxChanTypeW::new(self, 16)
    }
    #[doc = "Bit 29 - 29:29\\]
This field controls whether or not the DMA controller will pass the protocol specific words (if present) from the descriptor to the back end application. This field is encoded as follows: 0=DMA controller will pass PS words if present in descriptor 1=DMA controller will filter PS words."]
    #[inline(always)]
    #[must_use]
    pub fn tx_filt_pswords(&mut self) -> TxFiltPswordsW<PktdmaTchanTcfgSpec> {
        TxFiltPswordsW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
This field controls whether or not the DMA controller will pass the extended packet information fields (if present) from the descriptor to the back end application. This field is encoded as follows: 0=DMA controller will pass extended packet info words if they are present in the descriptor 1=DMA controller will filter extended packet info words."]
    #[inline(always)]
    #[must_use]
    pub fn tx_filt_einfo(&mut self) -> TxFiltEinfoW<PktdmaTchanTcfgSpec> {
        TxFiltEinfoW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Pause On Error: this field controls what the channel will do if an error or exception occurs during a data transfer. This field is encoded as follows: 0 = Channel will drop current work and move on 1 = Channel will pause and wait for SW to investigate and un-pause the channel."]
    #[inline(always)]
    #[must_use]
    pub fn tx_pause_on_err(&mut self) -> TxPauseOnErrW<PktdmaTchanTcfgSpec> {
        TxPauseOnErrW::new(self, 31)
    }
}
#[doc = "The Tx Channel Configuration Register is used to initialize static mode settings for the Tx DMA channel. This register may only be written when the channel is disabled (tx_enable in realtime control reg is 0).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_tchan_tcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_tchan_tcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PktdmaTchanTcfgSpec;
impl crate::RegisterSpec for PktdmaTchanTcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktdma_tchan_tcfg::R`](R) reader structure"]
impl crate::Readable for PktdmaTchanTcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pktdma_tchan_tcfg::W`](W) writer structure"]
impl crate::Writable for PktdmaTchanTcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTDMA_TCHAN_TCFG to value 0x0002_0400"]
impl crate::Resettable for PktdmaTchanTcfgSpec {
    const RESET_VALUE: u32 = 0x0002_0400;
}
