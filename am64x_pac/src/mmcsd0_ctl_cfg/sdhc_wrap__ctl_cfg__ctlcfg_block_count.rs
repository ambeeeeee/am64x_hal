#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_block_count` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgBlockCountSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_block_count` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgBlockCountSpec>;
#[doc = "Field `XFER_BLK_CNT` reader - 15:0\\]
Host Controller Version 4.10 extends block count to 32-bit \\[Refer to Section 1.15\\].Selection of either 16-bit Block Count register or 32-bit Block Count register is defined as follows: \\[1\\]
If Host Version 4 Enable in the Host Control 2 register is set to 0 or 16-bit Block Count register is set to non-zero, 16-bit Block Count register is selected \\[2\\]
If Host Version 4 Enable is set to 1 and 16-bit Block Count register is set to zero, 32-bit Block Count register is selected.Use of 16-bit/32-bit Block Count register is enabled when Block Count Enable in the Transfer Mode register is set to 1 and is valid only for multiple block transfers. The Host Driver shall set this register to a value between 1 and the maximum block count.The Host Controller decrements the block count after each block transfer and stops when the count reaches zero. Setting the block count to 0 results in no data blocks is transferred.This register should be accessed only when no transaction is executing \\[i.e., after transactions are stopped\\]. During data transfer,read operations on this register may return an invalid value and write operations are ignored."]
pub type XferBlkCntR = crate::FieldReader<u16>;
#[doc = "Field `XFER_BLK_CNT` writer - 15:0\\]
Host Controller Version 4.10 extends block count to 32-bit \\[Refer to Section 1.15\\].Selection of either 16-bit Block Count register or 32-bit Block Count register is defined as follows: \\[1\\]
If Host Version 4 Enable in the Host Control 2 register is set to 0 or 16-bit Block Count register is set to non-zero, 16-bit Block Count register is selected \\[2\\]
If Host Version 4 Enable is set to 1 and 16-bit Block Count register is set to zero, 32-bit Block Count register is selected.Use of 16-bit/32-bit Block Count register is enabled when Block Count Enable in the Transfer Mode register is set to 1 and is valid only for multiple block transfers. The Host Driver shall set this register to a value between 1 and the maximum block count.The Host Controller decrements the block count after each block transfer and stops when the count reaches zero. Setting the block count to 0 results in no data blocks is transferred.This register should be accessed only when no transaction is executing \\[i.e., after transactions are stopped\\]. During data transfer,read operations on this register may return an invalid value and write operations are ignored."]
pub type XferBlkCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Host Controller Version 4.10 extends block count to 32-bit \\[Refer to Section 1.15\\].Selection of either 16-bit Block Count register or 32-bit Block Count register is defined as follows: \\[1\\]
If Host Version 4 Enable in the Host Control 2 register is set to 0 or 16-bit Block Count register is set to non-zero, 16-bit Block Count register is selected \\[2\\]
If Host Version 4 Enable is set to 1 and 16-bit Block Count register is set to zero, 32-bit Block Count register is selected.Use of 16-bit/32-bit Block Count register is enabled when Block Count Enable in the Transfer Mode register is set to 1 and is valid only for multiple block transfers. The Host Driver shall set this register to a value between 1 and the maximum block count.The Host Controller decrements the block count after each block transfer and stops when the count reaches zero. Setting the block count to 0 results in no data blocks is transferred.This register should be accessed only when no transaction is executing \\[i.e., after transactions are stopped\\]. During data transfer,read operations on this register may return an invalid value and write operations are ignored."]
    #[inline(always)]
    pub fn xfer_blk_cnt(&self) -> XferBlkCntR {
        XferBlkCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Host Controller Version 4.10 extends block count to 32-bit \\[Refer to Section 1.15\\].Selection of either 16-bit Block Count register or 32-bit Block Count register is defined as follows: \\[1\\]
If Host Version 4 Enable in the Host Control 2 register is set to 0 or 16-bit Block Count register is set to non-zero, 16-bit Block Count register is selected \\[2\\]
If Host Version 4 Enable is set to 1 and 16-bit Block Count register is set to zero, 32-bit Block Count register is selected.Use of 16-bit/32-bit Block Count register is enabled when Block Count Enable in the Transfer Mode register is set to 1 and is valid only for multiple block transfers. The Host Driver shall set this register to a value between 1 and the maximum block count.The Host Controller decrements the block count after each block transfer and stops when the count reaches zero. Setting the block count to 0 results in no data blocks is transferred.This register should be accessed only when no transaction is executing \\[i.e., after transactions are stopped\\]. During data transfer,read operations on this register may return an invalid value and write operations are ignored."]
    #[inline(always)]
    #[must_use]
    pub fn xfer_blk_cnt(&mut self) -> XferBlkCntW<SdhcWrap_CtlCfg_CtlcfgBlockCountSpec> {
        XferBlkCntW::new(self, 0)
    }
}
#[doc = "This register is used to configure the number of data blocks\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_block_count::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_block_count::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgBlockCountSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgBlockCountSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_block_count::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgBlockCountSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_block_count::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgBlockCountSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_block_count to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgBlockCountSpec {
    const RESET_VALUE: u16 = 0;
}
