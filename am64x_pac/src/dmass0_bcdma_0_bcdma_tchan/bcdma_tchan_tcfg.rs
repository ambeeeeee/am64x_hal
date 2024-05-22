#[doc = "Register `BCDMA_TCHAN_TCFG` reader"]
pub type R = crate::R<BcdmaTchanTcfgSpec>;
#[doc = "Register `BCDMA_TCHAN_TCFG` writer"]
pub type W = crate::W<BcdmaTchanTcfgSpec>;
#[doc = "Field `TX_NOTDPKT` reader - 8:8\\]
Specifies whether or not the channel should suppress sending the single data phase teardown packet when teardown is complete. 0 = TD packet is sent 1 = Suppress sending TD packet"]
pub type TxNotdpktR = crate::BitReader;
#[doc = "Field `TX_NOTDPKT` writer - 8:8\\]
Specifies whether or not the channel should suppress sending the single data phase teardown packet when teardown is complete. 0 = TD packet is sent 1 = Suppress sending TD packet"]
pub type TxNotdpktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_TDTYPE` reader - 9:9\\]
Specifies whether or not the channel should immediately return a teardown completion response to the default completion queue or wait until a status message is returned from the remote PSI-L paired peripheral. 0 = return immediately once all traffic is complete in BCDMA. 1 = wait until remote peer sends back a completion message."]
pub type TxTdtypeR = crate::BitReader;
#[doc = "Field `TX_TDTYPE` writer - 9:9\\]
Specifies whether or not the channel should immediately return a teardown completion response to the default completion queue or wait until a status message is returned from the remote PSI-L paired peripheral. 0 = return immediately once all traffic is complete in BCDMA. 1 = wait until remote peer sends back a completion message."]
pub type TxTdtypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_BURST_SIZE` reader - 11:10\\]
Specifies the nominal burst size and alignment for data transfers on this channel. 0 = 32 Bytes 1 = 64 Bytes All other values are reserved The optimal burst size setting is 64 Bytes to maximize utilization of the channel FIFOs."]
pub type TxBurstSizeR = crate::FieldReader;
#[doc = "Field `TX_BURST_SIZE` writer - 11:10\\]
Specifies the nominal burst size and alignment for data transfers on this channel. 0 = 32 Bytes 1 = 64 Bytes All other values are reserved The optimal burst size setting is 64 Bytes to maximize utilization of the channel FIFOs."]
pub type TxBurstSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_CHAN_TYPE` reader - 19:16\\]
Channel Type: this field controls and / or indicates the functional channel type for this channel and the work passing mechanism that the channel uses for communicating with the Host. Available channel types are as follows: 0-9 = RESERVED 10 = Channel performs Third Party Block Copy DMA transfers from memory to PSI-L using pass by reference rings. 11-15 = RESERVED"]
pub type TxChanTypeR = crate::FieldReader;
#[doc = "Field `TX_CHAN_TYPE` writer - 19:16\\]
Channel Type: this field controls and / or indicates the functional channel type for this channel and the work passing mechanism that the channel uses for communicating with the Host. Available channel types are as follows: 0-9 = RESERVED 10 = Channel performs Third Party Block Copy DMA transfers from memory to PSI-L using pass by reference rings. 11-15 = RESERVED"]
pub type TxChanTypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
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
Specifies whether or not the channel should immediately return a teardown completion response to the default completion queue or wait until a status message is returned from the remote PSI-L paired peripheral. 0 = return immediately once all traffic is complete in BCDMA. 1 = wait until remote peer sends back a completion message."]
    #[inline(always)]
    pub fn tx_tdtype(&self) -> TxTdtypeR {
        TxTdtypeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Specifies the nominal burst size and alignment for data transfers on this channel. 0 = 32 Bytes 1 = 64 Bytes All other values are reserved The optimal burst size setting is 64 Bytes to maximize utilization of the channel FIFOs."]
    #[inline(always)]
    pub fn tx_burst_size(&self) -> TxBurstSizeR {
        TxBurstSizeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Channel Type: this field controls and / or indicates the functional channel type for this channel and the work passing mechanism that the channel uses for communicating with the Host. Available channel types are as follows: 0-9 = RESERVED 10 = Channel performs Third Party Block Copy DMA transfers from memory to PSI-L using pass by reference rings. 11-15 = RESERVED"]
    #[inline(always)]
    pub fn tx_chan_type(&self) -> TxChanTypeR {
        TxChanTypeR::new(((self.bits >> 16) & 0x0f) as u8)
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
    pub fn tx_notdpkt(&mut self) -> TxNotdpktW<BcdmaTchanTcfgSpec> {
        TxNotdpktW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Specifies whether or not the channel should immediately return a teardown completion response to the default completion queue or wait until a status message is returned from the remote PSI-L paired peripheral. 0 = return immediately once all traffic is complete in BCDMA. 1 = wait until remote peer sends back a completion message."]
    #[inline(always)]
    #[must_use]
    pub fn tx_tdtype(&mut self) -> TxTdtypeW<BcdmaTchanTcfgSpec> {
        TxTdtypeW::new(self, 9)
    }
    #[doc = "Bits 10:11 - 11:10\\]
Specifies the nominal burst size and alignment for data transfers on this channel. 0 = 32 Bytes 1 = 64 Bytes All other values are reserved The optimal burst size setting is 64 Bytes to maximize utilization of the channel FIFOs."]
    #[inline(always)]
    #[must_use]
    pub fn tx_burst_size(&mut self) -> TxBurstSizeW<BcdmaTchanTcfgSpec> {
        TxBurstSizeW::new(self, 10)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Channel Type: this field controls and / or indicates the functional channel type for this channel and the work passing mechanism that the channel uses for communicating with the Host. Available channel types are as follows: 0-9 = RESERVED 10 = Channel performs Third Party Block Copy DMA transfers from memory to PSI-L using pass by reference rings. 11-15 = RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn tx_chan_type(&mut self) -> TxChanTypeW<BcdmaTchanTcfgSpec> {
        TxChanTypeW::new(self, 16)
    }
    #[doc = "Bit 31 - 31:31\\]
Pause On Error: this field controls what the channel will do if an error or exception occurs during a data transfer. This field is encoded as follows: 0 = Channel will drop current work and move on 1 = Channel will pause and wait for SW to investigate and un-pause the channel."]
    #[inline(always)]
    #[must_use]
    pub fn tx_pause_on_err(&mut self) -> TxPauseOnErrW<BcdmaTchanTcfgSpec> {
        TxPauseOnErrW::new(self, 31)
    }
}
#[doc = "The Tx Channel Configuration Register is used to initialize static mode settings for the Tx DMA channel. This register may only be written when the channel is disabled (tx_enable in realtime control reg is 0).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_tchan_tcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_tchan_tcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaTchanTcfgSpec;
impl crate::RegisterSpec for BcdmaTchanTcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_tchan_tcfg::R`](R) reader structure"]
impl crate::Readable for BcdmaTchanTcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_tchan_tcfg::W`](W) writer structure"]
impl crate::Writable for BcdmaTchanTcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_TCHAN_TCFG to value 0x0010_0400"]
impl crate::Resettable for BcdmaTchanTcfgSpec {
    const RESET_VALUE: u32 = 0x0010_0400;
}
