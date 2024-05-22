#[doc = "Register `BCDMA_RCHAN_RCFG` reader"]
pub type R = crate::R<BcdmaRchanRcfgSpec>;
#[doc = "Register `BCDMA_RCHAN_RCFG` writer"]
pub type W = crate::W<BcdmaRchanRcfgSpec>;
#[doc = "Field `RX_BURST_SIZE` reader - 11:10\\]
Specifies the nominal burst size and alignment for data transfers on this channel. 0 = 32 Bytes 1 = 64 Bytes All other values are reserved The optimal burst size setting is 64 Bytes to maximize utilization of the channel FIFOs."]
pub type RxBurstSizeR = crate::FieldReader;
#[doc = "Field `RX_BURST_SIZE` writer - 11:10\\]
Specifies the nominal burst size and alignment for data transfers on this channel. 0 = 32 Bytes 1 = 64 Bytes All other values are reserved The optimal burst size setting is 64 Bytes to maximize utilization of the channel FIFOs."]
pub type RxBurstSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX_IGNORE_LONG` reader - 14:14\\]
This field controls whether or not long packets will be treated as exceptions or ignored for the channel. This field is only used when the channel is in split UTC mode. The values are encoded as follows: 0 = Long packets are treated as exceptions and handled appropriately. 1 = Long packets are ignored and the next TR will be fetched even if the current TR is marked or interpreted as EOP."]
pub type RxIgnoreLongR = crate::BitReader;
#[doc = "Field `RX_IGNORE_LONG` writer - 14:14\\]
This field controls whether or not long packets will be treated as exceptions or ignored for the channel. This field is only used when the channel is in split UTC mode. The values are encoded as follows: 0 = Long packets are treated as exceptions and handled appropriately. 1 = Long packets are ignored and the next TR will be fetched even if the current TR is marked or interpreted as EOP."]
pub type RxIgnoreLongW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CHAN_TYPE` reader - 19:16\\]
Rx Channel Type: this field controls and / or indicates the functional channel type for this channel and the work passing mechanism that the channel uses for communicating with the Host. Available channel types are as follows: 0-9 = RESERVED 10 = Channel performs Third Party Block Copy DMA transfers from PSI-L to memory using pass by reference rings. 11-15 = RESERVED"]
pub type RxChanTypeR = crate::FieldReader;
#[doc = "Field `RX_CHAN_TYPE` writer - 19:16\\]
Rx Channel Type: this field controls and / or indicates the functional channel type for this channel and the work passing mechanism that the channel uses for communicating with the Host. Available channel types are as follows: 0-9 = RESERVED 10 = Channel performs Third Party Block Copy DMA transfers from PSI-L to memory using pass by reference rings. 11-15 = RESERVED"]
pub type RxChanTypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RX_PAUSE_ON_ERR` reader - 31:31\\]
Pause On Error: this field controls what the channel will do if an error or exception occurs during a data transfer. This field is encoded as follows: 0 = Channel will drop current work and move on 1 = Channel will pause and wait for SW to investigate and un-pause the channel."]
pub type RxPauseOnErrR = crate::BitReader;
#[doc = "Field `RX_PAUSE_ON_ERR` writer - 31:31\\]
Pause On Error: this field controls what the channel will do if an error or exception occurs during a data transfer. This field is encoded as follows: 0 = Channel will drop current work and move on 1 = Channel will pause and wait for SW to investigate and un-pause the channel."]
pub type RxPauseOnErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 10:11 - 11:10\\]
Specifies the nominal burst size and alignment for data transfers on this channel. 0 = 32 Bytes 1 = 64 Bytes All other values are reserved The optimal burst size setting is 64 Bytes to maximize utilization of the channel FIFOs."]
    #[inline(always)]
    pub fn rx_burst_size(&self) -> RxBurstSizeR {
        RxBurstSizeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
This field controls whether or not long packets will be treated as exceptions or ignored for the channel. This field is only used when the channel is in split UTC mode. The values are encoded as follows: 0 = Long packets are treated as exceptions and handled appropriately. 1 = Long packets are ignored and the next TR will be fetched even if the current TR is marked or interpreted as EOP."]
    #[inline(always)]
    pub fn rx_ignore_long(&self) -> RxIgnoreLongR {
        RxIgnoreLongR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Rx Channel Type: this field controls and / or indicates the functional channel type for this channel and the work passing mechanism that the channel uses for communicating with the Host. Available channel types are as follows: 0-9 = RESERVED 10 = Channel performs Third Party Block Copy DMA transfers from PSI-L to memory using pass by reference rings. 11-15 = RESERVED"]
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
Specifies the nominal burst size and alignment for data transfers on this channel. 0 = 32 Bytes 1 = 64 Bytes All other values are reserved The optimal burst size setting is 64 Bytes to maximize utilization of the channel FIFOs."]
    #[inline(always)]
    #[must_use]
    pub fn rx_burst_size(&mut self) -> RxBurstSizeW<BcdmaRchanRcfgSpec> {
        RxBurstSizeW::new(self, 10)
    }
    #[doc = "Bit 14 - 14:14\\]
This field controls whether or not long packets will be treated as exceptions or ignored for the channel. This field is only used when the channel is in split UTC mode. The values are encoded as follows: 0 = Long packets are treated as exceptions and handled appropriately. 1 = Long packets are ignored and the next TR will be fetched even if the current TR is marked or interpreted as EOP."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ignore_long(&mut self) -> RxIgnoreLongW<BcdmaRchanRcfgSpec> {
        RxIgnoreLongW::new(self, 14)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Rx Channel Type: this field controls and / or indicates the functional channel type for this channel and the work passing mechanism that the channel uses for communicating with the Host. Available channel types are as follows: 0-9 = RESERVED 10 = Channel performs Third Party Block Copy DMA transfers from PSI-L to memory using pass by reference rings. 11-15 = RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn rx_chan_type(&mut self) -> RxChanTypeW<BcdmaRchanRcfgSpec> {
        RxChanTypeW::new(self, 16)
    }
    #[doc = "Bit 31 - 31:31\\]
Pause On Error: this field controls what the channel will do if an error or exception occurs during a data transfer. This field is encoded as follows: 0 = Channel will drop current work and move on 1 = Channel will pause and wait for SW to investigate and un-pause the channel."]
    #[inline(always)]
    #[must_use]
    pub fn rx_pause_on_err(&mut self) -> RxPauseOnErrW<BcdmaRchanRcfgSpec> {
        RxPauseOnErrW::new(self, 31)
    }
}
#[doc = "The Rx Channel Configuration Register is used to initialize static mode settings for the Rx DMA channel. This register may only be written when the channel is disabled (rx_enable in realtime control reg is 0).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_rchan_rcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_rchan_rcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaRchanRcfgSpec;
impl crate::RegisterSpec for BcdmaRchanRcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_rchan_rcfg::R`](R) reader structure"]
impl crate::Readable for BcdmaRchanRcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_rchan_rcfg::W`](W) writer structure"]
impl crate::Writable for BcdmaRchanRcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_RCHAN_RCFG to value 0x0010_0400"]
impl crate::Resettable for BcdmaRchanRcfgSpec {
    const RESET_VALUE: u32 = 0x0010_0400;
}
