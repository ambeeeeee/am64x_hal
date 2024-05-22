#[doc = "Register `BCDMA_BCHAN_CFG` reader"]
pub type R = crate::R<BcdmaBchanCfgSpec>;
#[doc = "Register `BCDMA_BCHAN_CFG` writer"]
pub type W = crate::W<BcdmaBchanCfgSpec>;
#[doc = "Field `BURST_SIZE` reader - 11:10\\]
Specifies the nominal burst size and alignment for data transfers on this channel. 0 = 32 Bytes 1 = 64 Bytes All other values are reserved The optimal burst size setting is 64 Bytes to maximize utilization of the channel FIFOs."]
pub type BurstSizeR = crate::FieldReader;
#[doc = "Field `BURST_SIZE` writer - 11:10\\]
Specifies the nominal burst size and alignment for data transfers on this channel. 0 = 32 Bytes 1 = 64 Bytes All other values are reserved The optimal burst size setting is 64 Bytes to maximize utilization of the channel FIFOs."]
pub type BurstSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHAN_TYPE` reader - 19:16\\]
Channel Type: this field controls and / or indicates the functional channel type for this channel and the work passing mechanism that the channel uses for communicating with the Host. Available channel types are as follows: 0-11 = RESERVED 12 = Channel performs Third Party Block Copy DMA transfers from memory to memory using pass by reference rings. 13-15 = RESERVED"]
pub type ChanTypeR = crate::FieldReader;
#[doc = "Field `CHAN_TYPE` writer - 19:16\\]
Channel Type: this field controls and / or indicates the functional channel type for this channel and the work passing mechanism that the channel uses for communicating with the Host. Available channel types are as follows: 0-11 = RESERVED 12 = Channel performs Third Party Block Copy DMA transfers from memory to memory using pass by reference rings. 13-15 = RESERVED"]
pub type ChanTypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PAUSE_ON_ERR` reader - 31:31\\]
Pause On Error: this field controls what the channel will do if an error or exception occurs during a data transfer. This field is encoded as follows: 0 = Channel will drop current work and move on 1 = Channel will pause and wait for SW to investigate and un-pause the channel."]
pub type PauseOnErrR = crate::BitReader;
#[doc = "Field `PAUSE_ON_ERR` writer - 31:31\\]
Pause On Error: this field controls what the channel will do if an error or exception occurs during a data transfer. This field is encoded as follows: 0 = Channel will drop current work and move on 1 = Channel will pause and wait for SW to investigate and un-pause the channel."]
pub type PauseOnErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 10:11 - 11:10\\]
Specifies the nominal burst size and alignment for data transfers on this channel. 0 = 32 Bytes 1 = 64 Bytes All other values are reserved The optimal burst size setting is 64 Bytes to maximize utilization of the channel FIFOs."]
    #[inline(always)]
    pub fn burst_size(&self) -> BurstSizeR {
        BurstSizeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Channel Type: this field controls and / or indicates the functional channel type for this channel and the work passing mechanism that the channel uses for communicating with the Host. Available channel types are as follows: 0-11 = RESERVED 12 = Channel performs Third Party Block Copy DMA transfers from memory to memory using pass by reference rings. 13-15 = RESERVED"]
    #[inline(always)]
    pub fn chan_type(&self) -> ChanTypeR {
        ChanTypeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Pause On Error: this field controls what the channel will do if an error or exception occurs during a data transfer. This field is encoded as follows: 0 = Channel will drop current work and move on 1 = Channel will pause and wait for SW to investigate and un-pause the channel."]
    #[inline(always)]
    pub fn pause_on_err(&self) -> PauseOnErrR {
        PauseOnErrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 10:11 - 11:10\\]
Specifies the nominal burst size and alignment for data transfers on this channel. 0 = 32 Bytes 1 = 64 Bytes All other values are reserved The optimal burst size setting is 64 Bytes to maximize utilization of the channel FIFOs."]
    #[inline(always)]
    #[must_use]
    pub fn burst_size(&mut self) -> BurstSizeW<BcdmaBchanCfgSpec> {
        BurstSizeW::new(self, 10)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Channel Type: this field controls and / or indicates the functional channel type for this channel and the work passing mechanism that the channel uses for communicating with the Host. Available channel types are as follows: 0-11 = RESERVED 12 = Channel performs Third Party Block Copy DMA transfers from memory to memory using pass by reference rings. 13-15 = RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn chan_type(&mut self) -> ChanTypeW<BcdmaBchanCfgSpec> {
        ChanTypeW::new(self, 16)
    }
    #[doc = "Bit 31 - 31:31\\]
Pause On Error: this field controls what the channel will do if an error or exception occurs during a data transfer. This field is encoded as follows: 0 = Channel will drop current work and move on 1 = Channel will pause and wait for SW to investigate and un-pause the channel."]
    #[inline(always)]
    #[must_use]
    pub fn pause_on_err(&mut self) -> PauseOnErrW<BcdmaBchanCfgSpec> {
        PauseOnErrW::new(self, 31)
    }
}
#[doc = "The Channel Configuration Register is used to initialize static mode settings for the Block Copy DMA channel. This register may only be written when the channel is disabled (tx_enable in realtime control reg is 0).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bcdma_bchan_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bcdma_bchan_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcdmaBchanCfgSpec;
impl crate::RegisterSpec for BcdmaBchanCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bcdma_bchan_cfg::R`](R) reader structure"]
impl crate::Readable for BcdmaBchanCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`bcdma_bchan_cfg::W`](W) writer structure"]
impl crate::Writable for BcdmaBchanCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BCDMA_BCHAN_CFG to value 0x0012_0400"]
impl crate::Resettable for BcdmaBchanCfgSpec {
    const RESET_VALUE: u32 = 0x0012_0400;
}
