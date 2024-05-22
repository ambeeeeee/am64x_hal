#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_block_count` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgUhs2BlockCountSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_block_count` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgUhs2BlockCountSpec>;
#[doc = "Field `XFER_BLK_COUNT` reader - 31:0\\]
This register is effective when Data Present is set to 1 in UHS-II Command register and is enabled when Block Count Enable is set to 1 and Block / Byte Mode is set to 0 in the UHS-II Transfer Mode register. Data transfer stops when the count reaches zero. Setting the block count to 0 results in no data blocks is transferred. This register should be accessed only when no transaction is executing \\[i.e.,after transactions are stopped\\]. During data transfer, read operations on this register may return an invalid value and write operations are ignored. 00000000h - Stop Count 00000001h - 1 block 00000002h - 2 blocks ..... ..... FFFFFFFFh - 4G blocks -1."]
pub type XferBlkCountR = crate::FieldReader<u32>;
#[doc = "Field `XFER_BLK_COUNT` writer - 31:0\\]
This register is effective when Data Present is set to 1 in UHS-II Command register and is enabled when Block Count Enable is set to 1 and Block / Byte Mode is set to 0 in the UHS-II Transfer Mode register. Data transfer stops when the count reaches zero. Setting the block count to 0 results in no data blocks is transferred. This register should be accessed only when no transaction is executing \\[i.e.,after transactions are stopped\\]. During data transfer, read operations on this register may return an invalid value and write operations are ignored. 00000000h - Stop Count 00000001h - 1 block 00000002h - 2 blocks ..... ..... FFFFFFFFh - 4G blocks -1."]
pub type XferBlkCountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register is effective when Data Present is set to 1 in UHS-II Command register and is enabled when Block Count Enable is set to 1 and Block / Byte Mode is set to 0 in the UHS-II Transfer Mode register. Data transfer stops when the count reaches zero. Setting the block count to 0 results in no data blocks is transferred. This register should be accessed only when no transaction is executing \\[i.e.,after transactions are stopped\\]. During data transfer, read operations on this register may return an invalid value and write operations are ignored. 00000000h - Stop Count 00000001h - 1 block 00000002h - 2 blocks ..... ..... FFFFFFFFh - 4G blocks -1."]
    #[inline(always)]
    pub fn xfer_blk_count(&self) -> XferBlkCountR {
        XferBlkCountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register is effective when Data Present is set to 1 in UHS-II Command register and is enabled when Block Count Enable is set to 1 and Block / Byte Mode is set to 0 in the UHS-II Transfer Mode register. Data transfer stops when the count reaches zero. Setting the block count to 0 results in no data blocks is transferred. This register should be accessed only when no transaction is executing \\[i.e.,after transactions are stopped\\]. During data transfer, read operations on this register may return an invalid value and write operations are ignored. 00000000h - Stop Count 00000001h - 1 block 00000002h - 2 blocks ..... ..... FFFFFFFFh - 4G blocks -1."]
    #[inline(always)]
    #[must_use]
    pub fn xfer_blk_count(&mut self) -> XferBlkCountW<SdhcWrap_CtlCfg_CtlcfgUhs2BlockCountSpec> {
        XferBlkCountW::new(self, 0)
    }
}
#[doc = "This register is used to configure the number of data blocks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_count::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_count::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgUhs2BlockCountSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgUhs2BlockCountSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_count::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgUhs2BlockCountSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_block_count::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgUhs2BlockCountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_block_count to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgUhs2BlockCountSpec {
    const RESET_VALUE: u32 = 0;
}
