#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_block_size` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgUhs2BlockSizeSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_block_size` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgUhs2BlockSizeSpec>;
#[doc = "Field `XFER_BLK_SIZE` reader - 11:0\\]
This register specifies the block size of data packet. SD Memory Card uses a fixed block size of 512 bytes. Vari-able block size may be used for SDIO. The maximum value is 2048 Bytes because CRC16 covers up to 2048 bytes. This register is effective when Data Present is set to 1 in UHS-II Command register."]
pub type XferBlkSizeR = crate::FieldReader<u16>;
#[doc = "Field `XFER_BLK_SIZE` writer - 11:0\\]
This register specifies the block size of data packet. SD Memory Card uses a fixed block size of 512 bytes. Vari-able block size may be used for SDIO. The maximum value is 2048 Bytes because CRC16 covers up to 2048 bytes. This register is effective when Data Present is set to 1 in UHS-II Command register."]
pub type XferBlkSizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SDMA_BUF_BOUNDARY` reader - 14:12\\]
When system memory is managed by paging, SDMA data transfer is performed in unit of paging. A page size of sys-tem memory management is set to this field. Host Controller generates the DMA Interrupt at the page boundary and requests the Host Driver to update the ADMA System Address register. SDMA waits until the ADMA System Address register is written. At the end of transfer, the Host Controller may issue or may not issue DMA Interrupt. In particular, DMA Interrupt shall not be issued after Transfer Complete Interrupt is issued. These bits shall be supported when the SDMA Support in the Capabilities register is set to 1 and this function is active when the DMA Enable in the UHS-II Transfer Mode register is set to 1. ADMA does not use this field."]
pub type SdmaBufBoundaryR = crate::FieldReader;
#[doc = "Field `SDMA_BUF_BOUNDARY` writer - 14:12\\]
When system memory is managed by paging, SDMA data transfer is performed in unit of paging. A page size of sys-tem memory management is set to this field. Host Controller generates the DMA Interrupt at the page boundary and requests the Host Driver to update the ADMA System Address register. SDMA waits until the ADMA System Address register is written. At the end of transfer, the Host Controller may issue or may not issue DMA Interrupt. In particular, DMA Interrupt shall not be issued after Transfer Complete Interrupt is issued. These bits shall be supported when the SDMA Support in the Capabilities register is set to 1 and this function is active when the DMA Enable in the UHS-II Transfer Mode register is set to 1. ADMA does not use this field."]
pub type SdmaBufBoundaryW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
This register specifies the block size of data packet. SD Memory Card uses a fixed block size of 512 bytes. Vari-able block size may be used for SDIO. The maximum value is 2048 Bytes because CRC16 covers up to 2048 bytes. This register is effective when Data Present is set to 1 in UHS-II Command register."]
    #[inline(always)]
    pub fn xfer_blk_size(&self) -> XferBlkSizeR {
        XferBlkSizeR::new(self.bits & 0x0fff)
    }
    #[doc = "Bits 12:14 - 14:12\\]
When system memory is managed by paging, SDMA data transfer is performed in unit of paging. A page size of sys-tem memory management is set to this field. Host Controller generates the DMA Interrupt at the page boundary and requests the Host Driver to update the ADMA System Address register. SDMA waits until the ADMA System Address register is written. At the end of transfer, the Host Controller may issue or may not issue DMA Interrupt. In particular, DMA Interrupt shall not be issued after Transfer Complete Interrupt is issued. These bits shall be supported when the SDMA Support in the Capabilities register is set to 1 and this function is active when the DMA Enable in the UHS-II Transfer Mode register is set to 1. ADMA does not use this field."]
    #[inline(always)]
    pub fn sdma_buf_boundary(&self) -> SdmaBufBoundaryR {
        SdmaBufBoundaryR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
This register specifies the block size of data packet. SD Memory Card uses a fixed block size of 512 bytes. Vari-able block size may be used for SDIO. The maximum value is 2048 Bytes because CRC16 covers up to 2048 bytes. This register is effective when Data Present is set to 1 in UHS-II Command register."]
    #[inline(always)]
    #[must_use]
    pub fn xfer_blk_size(&mut self) -> XferBlkSizeW<SdhcWrap_CtlCfg_CtlcfgUhs2BlockSizeSpec> {
        XferBlkSizeW::new(self, 0)
    }
    #[doc = "Bits 12:14 - 14:12\\]
When system memory is managed by paging, SDMA data transfer is performed in unit of paging. A page size of sys-tem memory management is set to this field. Host Controller generates the DMA Interrupt at the page boundary and requests the Host Driver to update the ADMA System Address register. SDMA waits until the ADMA System Address register is written. At the end of transfer, the Host Controller may issue or may not issue DMA Interrupt. In particular, DMA Interrupt shall not be issued after Transfer Complete Interrupt is issued. These bits shall be supported when the SDMA Support in the Capabilities register is set to 1 and this function is active when the DMA Enable in the UHS-II Transfer Mode register is set to 1. ADMA does not use this field."]
    #[inline(always)]
    #[must_use]
    pub fn sdma_buf_boundary(
        &mut self,
    ) -> SdmaBufBoundaryW<SdhcWrap_CtlCfg_CtlcfgUhs2BlockSizeSpec> {
        SdmaBufBoundaryW::new(self, 12)
    }
}
#[doc = "This register is used to configure the number of bytes in a data block\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgUhs2BlockSizeSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgUhs2BlockSizeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_size::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgUhs2BlockSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_size::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgUhs2BlockSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_block_size to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgUhs2BlockSizeSpec {
    const RESET_VALUE: u16 = 0;
}
