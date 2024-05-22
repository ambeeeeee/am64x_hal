#[doc = "Register `PKTDMA_RFLOW_RFA` reader"]
pub type R = crate::R<PktdmaRflowRfaSpec>;
#[doc = "Register `PKTDMA_RFLOW_RFA` writer"]
pub type W = crate::W<PktdmaRflowRfaSpec>;
#[doc = "Field `RX_SOP_OFFSET` reader - 24:16\\]
Rx Start of Packet Offset: This field specifies the number of bytes that are to be skipped in the SOP buffer before beginning to write the payload or protocol specific bytes(if they are in the sop buffer). This value must be less than the minimum size of a buffer in the system. Valid values are 0 - 255 bytes. The primary purpose of this field is to ensure that space is left in the descriptor to place the protocol specific information without overwriting or being overwritten by the Rx data. The secondary purpose of this field is to allow space to be left prior to the data in the descriptor in case header information needs to be added as the packet is passed thorough the system."]
pub type RxSopOffsetR = crate::FieldReader<u16>;
#[doc = "Field `RX_SOP_OFFSET` writer - 24:16\\]
Rx Start of Packet Offset: This field specifies the number of bytes that are to be skipped in the SOP buffer before beginning to write the payload or protocol specific bytes(if they are in the sop buffer). This value must be less than the minimum size of a buffer in the system. Valid values are 0 - 255 bytes. The primary purpose of this field is to ensure that space is left in the descriptor to place the protocol specific information without overwriting or being overwritten by the Rx data. The secondary purpose of this field is to allow space to be left prior to the data in the descriptor in case header information needs to be added as the packet is passed thorough the system."]
pub type RxSopOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `RX_ERROR_HANDLING` reader - 28:28\\]
Rx Error Handling Mode: This bit controls the error handling mode for the flow and is only used when channel errors (i.e. descriptor starvation) occurs. 0 = Starvation errors result in dropping packet and incrementing dropped packet count. 1 = Starvation errors result in the channel waiting until descriptors are added to the free queue before the channel will be scheduled."]
pub type RxErrorHandlingR = crate::BitReader;
#[doc = "Field `RX_ERROR_HANDLING` writer - 28:28\\]
Rx Error Handling Mode: This bit controls the error handling mode for the flow and is only used when channel errors (i.e. descriptor starvation) occurs. 0 = Starvation errors result in dropping packet and incrementing dropped packet count. 1 = Starvation errors result in the channel waiting until descriptors are added to the free queue before the channel will be scheduled."]
pub type RxErrorHandlingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_PSINFO_PRESENT` reader - 29:29\\]
Rx PS Words Present: This bit controls whether or not the Protocol Specific words will be present in the Rx Packet Descriptor. If this bit is clear, the port DMA will set the PS word count to 0 in the PD and will drop any PS words that are presented from the back end application. If this bit is set, the port DMA will set the PS word count to the value given by the back end application and will copy the PS words from the back end application to the location"]
pub type RxPsinfoPresentR = crate::BitReader;
#[doc = "Field `RX_PSINFO_PRESENT` writer - 29:29\\]
Rx PS Words Present: This bit controls whether or not the Protocol Specific words will be present in the Rx Packet Descriptor. If this bit is clear, the port DMA will set the PS word count to 0 in the PD and will drop any PS words that are presented from the back end application. If this bit is set, the port DMA will set the PS word count to the value given by the back end application and will copy the PS words from the back end application to the location"]
pub type RxPsinfoPresentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_EINFO_PRESENT` reader - 30:30\\]
Rx Extended Packet Info Block Present: This bit controls whether or not the Extended Packet Info Block will be present in the Rx Packet Descriptor. If this bit is clear, the port DMA will clear the Extended Packet Info Present bit in the PD and will drop any Timestamp or SW Data words that are presented from the back end application. If this bit is set, the port DMA will set the Extended Packet Info Block Present bit in the PD and will copy any Timestamp or SW Data words that are presented across the Rx streaming interface into the Extended Packet Info Block words in the descriptor. If no Timestamp or SW Data words are presented from the back end application, the port DMA will overwrite the fields in the PD with zeroes."]
pub type RxEinfoPresentR = crate::BitReader;
#[doc = "Field `RX_EINFO_PRESENT` writer - 30:30\\]
Rx Extended Packet Info Block Present: This bit controls whether or not the Extended Packet Info Block will be present in the Rx Packet Descriptor. If this bit is clear, the port DMA will clear the Extended Packet Info Present bit in the PD and will drop any Timestamp or SW Data words that are presented from the back end application. If this bit is set, the port DMA will set the Extended Packet Info Block Present bit in the PD and will copy any Timestamp or SW Data words that are presented across the Rx streaming interface into the Extended Packet Info Block words in the descriptor. If no Timestamp or SW Data words are presented from the back end application, the port DMA will overwrite the fields in the PD with zeroes."]
pub type RxEinfoPresentW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:24 - 24:16\\]
Rx Start of Packet Offset: This field specifies the number of bytes that are to be skipped in the SOP buffer before beginning to write the payload or protocol specific bytes(if they are in the sop buffer). This value must be less than the minimum size of a buffer in the system. Valid values are 0 - 255 bytes. The primary purpose of this field is to ensure that space is left in the descriptor to place the protocol specific information without overwriting or being overwritten by the Rx data. The secondary purpose of this field is to allow space to be left prior to the data in the descriptor in case header information needs to be added as the packet is passed thorough the system."]
    #[inline(always)]
    pub fn rx_sop_offset(&self) -> RxSopOffsetR {
        RxSopOffsetR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 28 - 28:28\\]
Rx Error Handling Mode: This bit controls the error handling mode for the flow and is only used when channel errors (i.e. descriptor starvation) occurs. 0 = Starvation errors result in dropping packet and incrementing dropped packet count. 1 = Starvation errors result in the channel waiting until descriptors are added to the free queue before the channel will be scheduled."]
    #[inline(always)]
    pub fn rx_error_handling(&self) -> RxErrorHandlingR {
        RxErrorHandlingR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Rx PS Words Present: This bit controls whether or not the Protocol Specific words will be present in the Rx Packet Descriptor. If this bit is clear, the port DMA will set the PS word count to 0 in the PD and will drop any PS words that are presented from the back end application. If this bit is set, the port DMA will set the PS word count to the value given by the back end application and will copy the PS words from the back end application to the location"]
    #[inline(always)]
    pub fn rx_psinfo_present(&self) -> RxPsinfoPresentR {
        RxPsinfoPresentR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Rx Extended Packet Info Block Present: This bit controls whether or not the Extended Packet Info Block will be present in the Rx Packet Descriptor. If this bit is clear, the port DMA will clear the Extended Packet Info Present bit in the PD and will drop any Timestamp or SW Data words that are presented from the back end application. If this bit is set, the port DMA will set the Extended Packet Info Block Present bit in the PD and will copy any Timestamp or SW Data words that are presented across the Rx streaming interface into the Extended Packet Info Block words in the descriptor. If no Timestamp or SW Data words are presented from the back end application, the port DMA will overwrite the fields in the PD with zeroes."]
    #[inline(always)]
    pub fn rx_einfo_present(&self) -> RxEinfoPresentR {
        RxEinfoPresentR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:24 - 24:16\\]
Rx Start of Packet Offset: This field specifies the number of bytes that are to be skipped in the SOP buffer before beginning to write the payload or protocol specific bytes(if they are in the sop buffer). This value must be less than the minimum size of a buffer in the system. Valid values are 0 - 255 bytes. The primary purpose of this field is to ensure that space is left in the descriptor to place the protocol specific information without overwriting or being overwritten by the Rx data. The secondary purpose of this field is to allow space to be left prior to the data in the descriptor in case header information needs to be added as the packet is passed thorough the system."]
    #[inline(always)]
    #[must_use]
    pub fn rx_sop_offset(&mut self) -> RxSopOffsetW<PktdmaRflowRfaSpec> {
        RxSopOffsetW::new(self, 16)
    }
    #[doc = "Bit 28 - 28:28\\]
Rx Error Handling Mode: This bit controls the error handling mode for the flow and is only used when channel errors (i.e. descriptor starvation) occurs. 0 = Starvation errors result in dropping packet and incrementing dropped packet count. 1 = Starvation errors result in the channel waiting until descriptors are added to the free queue before the channel will be scheduled."]
    #[inline(always)]
    #[must_use]
    pub fn rx_error_handling(&mut self) -> RxErrorHandlingW<PktdmaRflowRfaSpec> {
        RxErrorHandlingW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
Rx PS Words Present: This bit controls whether or not the Protocol Specific words will be present in the Rx Packet Descriptor. If this bit is clear, the port DMA will set the PS word count to 0 in the PD and will drop any PS words that are presented from the back end application. If this bit is set, the port DMA will set the PS word count to the value given by the back end application and will copy the PS words from the back end application to the location"]
    #[inline(always)]
    #[must_use]
    pub fn rx_psinfo_present(&mut self) -> RxPsinfoPresentW<PktdmaRflowRfaSpec> {
        RxPsinfoPresentW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Rx Extended Packet Info Block Present: This bit controls whether or not the Extended Packet Info Block will be present in the Rx Packet Descriptor. If this bit is clear, the port DMA will clear the Extended Packet Info Present bit in the PD and will drop any Timestamp or SW Data words that are presented from the back end application. If this bit is set, the port DMA will set the Extended Packet Info Block Present bit in the PD and will copy any Timestamp or SW Data words that are presented across the Rx streaming interface into the Extended Packet Info Block words in the descriptor. If no Timestamp or SW Data words are presented from the back end application, the port DMA will overwrite the fields in the PD with zeroes."]
    #[inline(always)]
    #[must_use]
    pub fn rx_einfo_present(&mut self) -> RxEinfoPresentW<PktdmaRflowRfaSpec> {
        RxEinfoPresentW::new(self, 30)
    }
}
#[doc = "The Rx Flow N Configuration Register A contains static configuration information for the Rx DMA flow. The fields in this register can only be changed when all of the DMA channels that use this flow have been disabled. The fields in this register are as follows:\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pktdma_rflow_rfa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pktdma_rflow_rfa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PktdmaRflowRfaSpec;
impl crate::RegisterSpec for PktdmaRflowRfaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pktdma_rflow_rfa::R`](R) reader structure"]
impl crate::Readable for PktdmaRflowRfaSpec {}
#[doc = "`write(|w| ..)` method takes [`pktdma_rflow_rfa::W`](W) writer structure"]
impl crate::Writable for PktdmaRflowRfaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PKTDMA_RFLOW_RFA to value 0"]
impl crate::Resettable for PktdmaRflowRfaSpec {
    const RESET_VALUE: u32 = 0;
}
