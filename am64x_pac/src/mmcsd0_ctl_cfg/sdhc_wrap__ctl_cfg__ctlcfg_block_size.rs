#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_block_size` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgBlockSizeSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_block_size` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgBlockSizeSpec>;
#[doc = "Field `XFER_BLK_SIZE` reader - 11:0\\]
This field specifies the block size for block data transfers for CMD17, CMD18, CMD24, CMD25 and CMD53. It can be accessed only if no transaction is executing \\[i.e after a transaction has stopped\\]. Read operations during transfer return an invalid value and write operations shall be ignored."]
pub type XferBlkSizeR = crate::FieldReader<u16>;
#[doc = "Field `XFER_BLK_SIZE` writer - 11:0\\]
This field specifies the block size for block data transfers for CMD17, CMD18, CMD24, CMD25 and CMD53. It can be accessed only if no transaction is executing \\[i.e after a transaction has stopped\\]. Read operations during transfer return an invalid value and write operations shall be ignored."]
pub type XferBlkSizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `SDMA_BUF_SIZE` reader - 14:12\\]
To perform long DMA transfer, System Address register shall be updated at every system boundary during DMA transfer. These bits specify the size of contiguous buffer in the system memory. The DMA transfer shall wait at the every boundary specified by these fields and the HC generates the DMA Interrupt to request the HD to update the System Address register. These bits shall support when the DMA Support in the Capabilities register is set to 1 and this function is active when the DMA Enable in the Transfer Mode register is set to 1."]
pub type SdmaBufSizeR = crate::FieldReader;
#[doc = "Field `SDMA_BUF_SIZE` writer - 14:12\\]
To perform long DMA transfer, System Address register shall be updated at every system boundary during DMA transfer. These bits specify the size of contiguous buffer in the system memory. The DMA transfer shall wait at the every boundary specified by these fields and the HC generates the DMA Interrupt to request the HD to update the System Address register. These bits shall support when the DMA Support in the Capabilities register is set to 1 and this function is active when the DMA Enable in the Transfer Mode register is set to 1."]
pub type SdmaBufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
This field specifies the block size for block data transfers for CMD17, CMD18, CMD24, CMD25 and CMD53. It can be accessed only if no transaction is executing \\[i.e after a transaction has stopped\\]. Read operations during transfer return an invalid value and write operations shall be ignored."]
    #[inline(always)]
    pub fn xfer_blk_size(&self) -> XferBlkSizeR {
        XferBlkSizeR::new(self.bits & 0x0fff)
    }
    #[doc = "Bits 12:14 - 14:12\\]
To perform long DMA transfer, System Address register shall be updated at every system boundary during DMA transfer. These bits specify the size of contiguous buffer in the system memory. The DMA transfer shall wait at the every boundary specified by these fields and the HC generates the DMA Interrupt to request the HD to update the System Address register. These bits shall support when the DMA Support in the Capabilities register is set to 1 and this function is active when the DMA Enable in the Transfer Mode register is set to 1."]
    #[inline(always)]
    pub fn sdma_buf_size(&self) -> SdmaBufSizeR {
        SdmaBufSizeR::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
This field specifies the block size for block data transfers for CMD17, CMD18, CMD24, CMD25 and CMD53. It can be accessed only if no transaction is executing \\[i.e after a transaction has stopped\\]. Read operations during transfer return an invalid value and write operations shall be ignored."]
    #[inline(always)]
    #[must_use]
    pub fn xfer_blk_size(&mut self) -> XferBlkSizeW<SdhcWrap_CtlCfg_CtlcfgBlockSizeSpec> {
        XferBlkSizeW::new(self, 0)
    }
    #[doc = "Bits 12:14 - 14:12\\]
To perform long DMA transfer, System Address register shall be updated at every system boundary during DMA transfer. These bits specify the size of contiguous buffer in the system memory. The DMA transfer shall wait at the every boundary specified by these fields and the HC generates the DMA Interrupt to request the HD to update the System Address register. These bits shall support when the DMA Support in the Capabilities register is set to 1 and this function is active when the DMA Enable in the Transfer Mode register is set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn sdma_buf_size(&mut self) -> SdmaBufSizeW<SdhcWrap_CtlCfg_CtlcfgBlockSizeSpec> {
        SdmaBufSizeW::new(self, 12)
    }
}
#[doc = "This register is used to configure the number of bytes in a data block\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_block_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_block_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgBlockSizeSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgBlockSizeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_block_size::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgBlockSizeSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_block_size::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgBlockSizeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_block_size to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgBlockSizeSpec {
    const RESET_VALUE: u16 = 0;
}
