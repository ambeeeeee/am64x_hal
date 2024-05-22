#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_transfer_mode` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgTransferModeSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_transfer_mode` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgTransferModeSpec>;
#[doc = "Field `DMA_ENA` reader - 0:0\\]
DMA can be enabled only if DMA Support bit in the Capabilities register is set. If this bit is set to 1, a DMA operation shall begin when the HD writes to the upper byte of Command register \\[00Fh\\]."]
pub type DmaEnaR = crate::BitReader;
#[doc = "Field `DMA_ENA` writer - 0:0\\]
DMA can be enabled only if DMA Support bit in the Capabilities register is set. If this bit is set to 1, a DMA operation shall begin when the HD writes to the upper byte of Command register \\[00Fh\\]."]
pub type DmaEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_CNT_ENA` reader - 1:1\\]
This bit is used to enable the Block count register, which is only relevant for multiple block transfers. When this bit is 0, the Block Count register is disabled, which is useful in executing an infinite transfer."]
pub type BlkCntEnaR = crate::BitReader;
#[doc = "Field `BLK_CNT_ENA` writer - 1:1\\]
This bit is used to enable the Block count register, which is only relevant for multiple block transfers. When this bit is 0, the Block Count register is disabled, which is useful in executing an infinite transfer."]
pub type BlkCntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CMD_ENA` reader - 3:2\\]
There are three methods to stop Multiple-block read and write operation. \\[1\\]
Auto CMD12 Enable: Multiple-block read and write commands for memory require CMD12 to stop the operation. When this field is set to 01b, the Host Controller issues CMD12 automatically when last block transfer is completed. Auto CMD12 error is indicated to the Auto CMD Error Status register. The Host Driver shall not set this bit if the command does not require CMD12. When Host Version 4 Enable =0, CMD12 is issued when 16-bit Block Count is expired. When Host Version 4 Enable =1, CMD12 is issued when 16-bit Block Count or 32-bit Block Count is expired. \\[2\\]
Auto CMD23 Enable: When this bit field is set to 10b, the Host Controller issues a CMD23 automatically before issuing a command specified in the Command register. The Host Controller Version 3.00 and later shall support this function. The following conditions are required to use the Auto CMD23. The following conditions are required to use the Auto CMD23. Auto CMD23 Supported \\[Host Controller Version is 3.00 or later\\]. A memory card that supports CMD23 \\[SCR\\[33\\]=1\\]. If DMA is used, it shall be ADMA. Only when CMD18 or CMD25 is issued. Auto CMD23 can be used with or without ADMA. By writing the Command register, the Host Controller issues a CMD23 first and then issues a command specified by the Command Index in Com-mand register. If response errors of CMD23 are detected, the second command is not issued. A CMD23 error is indicated in the Auto CMD Error Status register.32-bit block count value for CMD23 is set to 32-bit Block Count \\[SDMA System Address\\]
register. \\[3\\]
Auto CMD Auto Select \\[Version 4.10\\]
As CMD23 is optional for SD memory card except UHS 104 card, if card supports CMD23, Auto CMD 23 should be used instead of Auto CMD12. Host Controller Version 4.10 defines this Auto CMD Auto Select mode. Selection of Auto CMD depends on setting of CMD23 Enable in the Host Control 2 register which indicates whether card supports CMD23. If CMD23 Enable =1, Auto CMD23 is used and if CMD23 Enable =0, Auto CMD12 is used. In case of Version 4.10 or later, use of Auto CMD Auto Select is recommended rather than use of Auto CMD12 Enable or Auto CMD23 Enable."]
pub type AutoCmdEnaR = crate::FieldReader;
#[doc = "Field `AUTO_CMD_ENA` writer - 3:2\\]
There are three methods to stop Multiple-block read and write operation. \\[1\\]
Auto CMD12 Enable: Multiple-block read and write commands for memory require CMD12 to stop the operation. When this field is set to 01b, the Host Controller issues CMD12 automatically when last block transfer is completed. Auto CMD12 error is indicated to the Auto CMD Error Status register. The Host Driver shall not set this bit if the command does not require CMD12. When Host Version 4 Enable =0, CMD12 is issued when 16-bit Block Count is expired. When Host Version 4 Enable =1, CMD12 is issued when 16-bit Block Count or 32-bit Block Count is expired. \\[2\\]
Auto CMD23 Enable: When this bit field is set to 10b, the Host Controller issues a CMD23 automatically before issuing a command specified in the Command register. The Host Controller Version 3.00 and later shall support this function. The following conditions are required to use the Auto CMD23. The following conditions are required to use the Auto CMD23. Auto CMD23 Supported \\[Host Controller Version is 3.00 or later\\]. A memory card that supports CMD23 \\[SCR\\[33\\]=1\\]. If DMA is used, it shall be ADMA. Only when CMD18 or CMD25 is issued. Auto CMD23 can be used with or without ADMA. By writing the Command register, the Host Controller issues a CMD23 first and then issues a command specified by the Command Index in Com-mand register. If response errors of CMD23 are detected, the second command is not issued. A CMD23 error is indicated in the Auto CMD Error Status register.32-bit block count value for CMD23 is set to 32-bit Block Count \\[SDMA System Address\\]
register. \\[3\\]
Auto CMD Auto Select \\[Version 4.10\\]
As CMD23 is optional for SD memory card except UHS 104 card, if card supports CMD23, Auto CMD 23 should be used instead of Auto CMD12. Host Controller Version 4.10 defines this Auto CMD Auto Select mode. Selection of Auto CMD depends on setting of CMD23 Enable in the Host Control 2 register which indicates whether card supports CMD23. If CMD23 Enable =1, Auto CMD23 is used and if CMD23 Enable =0, Auto CMD12 is used. In case of Version 4.10 or later, use of Auto CMD Auto Select is recommended rather than use of Auto CMD12 Enable or Auto CMD23 Enable."]
pub type AutoCmdEnaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DATA_XFER_DIR` reader - 4:4\\]
This bit defines the direction of data transfers."]
pub type DataXferDirR = crate::BitReader;
#[doc = "Field `DATA_XFER_DIR` writer - 4:4\\]
This bit defines the direction of data transfers."]
pub type DataXferDirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULTI_BLK_SEL` reader - 5:5\\]
This bit enables multiple block data transfers."]
pub type MultiBlkSelR = crate::BitReader;
#[doc = "Field `MULTI_BLK_SEL` writer - 5:5\\]
This bit enables multiple block data transfers."]
pub type MultiBlkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_TYPE` reader - 6:6\\]
When response error check is enabled, this bit selects either R1 or R5 response types. Two types of response checks are supported: R1 for memory and R5 for SDIO. Error Statuses Checked in R1 .Bit31 OUT_OF_RANGE .Bit30 ADDRESS_ERROR .Bit29 BLOCK_LEN_ERROR .Bit26 WP_VIOLATION .Bit25 CARD_IS_LOCKED .Bit23 COM_CRC_ERROR .Bit21 CARD_ECC_FAILED .Bit20 CC_ERROR .Bit19 ERROR Response Flags Checked in R5 .Bit07 COM_CRC_ERROR .Bit03 ERROR .Bit01 FUNCTION_NUMBER .Bit00 OUT_OF_RANGE"]
pub type RespTypeR = crate::BitReader;
#[doc = "Field `RESP_TYPE` writer - 6:6\\]
When response error check is enabled, this bit selects either R1 or R5 response types. Two types of response checks are supported: R1 for memory and R5 for SDIO. Error Statuses Checked in R1 .Bit31 OUT_OF_RANGE .Bit30 ADDRESS_ERROR .Bit29 BLOCK_LEN_ERROR .Bit26 WP_VIOLATION .Bit25 CARD_IS_LOCKED .Bit23 COM_CRC_ERROR .Bit21 CARD_ECC_FAILED .Bit20 CC_ERROR .Bit19 ERROR Response Flags Checked in R5 .Bit07 COM_CRC_ERROR .Bit03 ERROR .Bit01 FUNCTION_NUMBER .Bit00 OUT_OF_RANGE"]
pub type RespTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_ERR_CHK_ENA` reader - 7:7\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver. Only R1 or R5 can be checked.If Host Driver checks response error, this bit is set to 0 and Response Interrupt Disable is set to 0.If Host Controller checks response error, sets this bit to 1 and sets Response Interrupt Disable to 1. Response Type R1 / R5 selects either R1 or R5 response type. If an error is detected, Response Error Interrupt is generated in the Response Error Interrupt Status register."]
pub type RespErrChkEnaR = crate::BitReader;
#[doc = "Field `RESP_ERR_CHK_ENA` writer - 7:7\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver. Only R1 or R5 can be checked.If Host Driver checks response error, this bit is set to 0 and Response Interrupt Disable is set to 0.If Host Controller checks response error, sets this bit to 1 and sets Response Interrupt Disable to 1. Response Type R1 / R5 selects either R1 or R5 response type. If an error is detected, Response Error Interrupt is generated in the Response Error Interrupt Status register."]
pub type RespErrChkEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_INTR_DIS` reader - 8:8\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver. Only R1 or R5 can be checked. If Host Driver checks response error,sets this bit to 0 and waits Command Complete Interrupt and then checks the response register. If Host Controller checks response error, sets this bit to 1 and sets Response Error Check Enable to 1. Command Complete Interrupt is disabled by this bit regardless of Command Complete Signal Enable."]
pub type RespIntrDisR = crate::BitReader;
#[doc = "Field `RESP_INTR_DIS` writer - 8:8\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver. Only R1 or R5 can be checked. If Host Driver checks response error,sets this bit to 0 and waits Command Complete Interrupt and then checks the response register. If Host Controller checks response error, sets this bit to 1 and sets Response Error Check Enable to 1. Command Complete Interrupt is disabled by this bit regardless of Command Complete Signal Enable."]
pub type RespIntrDisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DMA can be enabled only if DMA Support bit in the Capabilities register is set. If this bit is set to 1, a DMA operation shall begin when the HD writes to the upper byte of Command register \\[00Fh\\]."]
    #[inline(always)]
    pub fn dma_ena(&self) -> DmaEnaR {
        DmaEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit is used to enable the Block count register, which is only relevant for multiple block transfers. When this bit is 0, the Block Count register is disabled, which is useful in executing an infinite transfer."]
    #[inline(always)]
    pub fn blk_cnt_ena(&self) -> BlkCntEnaR {
        BlkCntEnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
There are three methods to stop Multiple-block read and write operation. \\[1\\]
Auto CMD12 Enable: Multiple-block read and write commands for memory require CMD12 to stop the operation. When this field is set to 01b, the Host Controller issues CMD12 automatically when last block transfer is completed. Auto CMD12 error is indicated to the Auto CMD Error Status register. The Host Driver shall not set this bit if the command does not require CMD12. When Host Version 4 Enable =0, CMD12 is issued when 16-bit Block Count is expired. When Host Version 4 Enable =1, CMD12 is issued when 16-bit Block Count or 32-bit Block Count is expired. \\[2\\]
Auto CMD23 Enable: When this bit field is set to 10b, the Host Controller issues a CMD23 automatically before issuing a command specified in the Command register. The Host Controller Version 3.00 and later shall support this function. The following conditions are required to use the Auto CMD23. The following conditions are required to use the Auto CMD23. Auto CMD23 Supported \\[Host Controller Version is 3.00 or later\\]. A memory card that supports CMD23 \\[SCR\\[33\\]=1\\]. If DMA is used, it shall be ADMA. Only when CMD18 or CMD25 is issued. Auto CMD23 can be used with or without ADMA. By writing the Command register, the Host Controller issues a CMD23 first and then issues a command specified by the Command Index in Com-mand register. If response errors of CMD23 are detected, the second command is not issued. A CMD23 error is indicated in the Auto CMD Error Status register.32-bit block count value for CMD23 is set to 32-bit Block Count \\[SDMA System Address\\]
register. \\[3\\]
Auto CMD Auto Select \\[Version 4.10\\]
As CMD23 is optional for SD memory card except UHS 104 card, if card supports CMD23, Auto CMD 23 should be used instead of Auto CMD12. Host Controller Version 4.10 defines this Auto CMD Auto Select mode. Selection of Auto CMD depends on setting of CMD23 Enable in the Host Control 2 register which indicates whether card supports CMD23. If CMD23 Enable =1, Auto CMD23 is used and if CMD23 Enable =0, Auto CMD12 is used. In case of Version 4.10 or later, use of Auto CMD Auto Select is recommended rather than use of Auto CMD12 Enable or Auto CMD23 Enable."]
    #[inline(always)]
    pub fn auto_cmd_ena(&self) -> AutoCmdEnaR {
        AutoCmdEnaR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
This bit defines the direction of data transfers."]
    #[inline(always)]
    pub fn data_xfer_dir(&self) -> DataXferDirR {
        DataXferDirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
This bit enables multiple block data transfers."]
    #[inline(always)]
    pub fn multi_blk_sel(&self) -> MultiBlkSelR {
        MultiBlkSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
When response error check is enabled, this bit selects either R1 or R5 response types. Two types of response checks are supported: R1 for memory and R5 for SDIO. Error Statuses Checked in R1 .Bit31 OUT_OF_RANGE .Bit30 ADDRESS_ERROR .Bit29 BLOCK_LEN_ERROR .Bit26 WP_VIOLATION .Bit25 CARD_IS_LOCKED .Bit23 COM_CRC_ERROR .Bit21 CARD_ECC_FAILED .Bit20 CC_ERROR .Bit19 ERROR Response Flags Checked in R5 .Bit07 COM_CRC_ERROR .Bit03 ERROR .Bit01 FUNCTION_NUMBER .Bit00 OUT_OF_RANGE"]
    #[inline(always)]
    pub fn resp_type(&self) -> RespTypeR {
        RespTypeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver. Only R1 or R5 can be checked.If Host Driver checks response error, this bit is set to 0 and Response Interrupt Disable is set to 0.If Host Controller checks response error, sets this bit to 1 and sets Response Interrupt Disable to 1. Response Type R1 / R5 selects either R1 or R5 response type. If an error is detected, Response Error Interrupt is generated in the Response Error Interrupt Status register."]
    #[inline(always)]
    pub fn resp_err_chk_ena(&self) -> RespErrChkEnaR {
        RespErrChkEnaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver. Only R1 or R5 can be checked. If Host Driver checks response error,sets this bit to 0 and waits Command Complete Interrupt and then checks the response register. If Host Controller checks response error, sets this bit to 1 and sets Response Error Check Enable to 1. Command Complete Interrupt is disabled by this bit regardless of Command Complete Signal Enable."]
    #[inline(always)]
    pub fn resp_intr_dis(&self) -> RespIntrDisR {
        RespIntrDisR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DMA can be enabled only if DMA Support bit in the Capabilities register is set. If this bit is set to 1, a DMA operation shall begin when the HD writes to the upper byte of Command register \\[00Fh\\]."]
    #[inline(always)]
    #[must_use]
    pub fn dma_ena(&mut self) -> DmaEnaW<SdhcWrap_CtlCfg_CtlcfgTransferModeSpec> {
        DmaEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit is used to enable the Block count register, which is only relevant for multiple block transfers. When this bit is 0, the Block Count register is disabled, which is useful in executing an infinite transfer."]
    #[inline(always)]
    #[must_use]
    pub fn blk_cnt_ena(&mut self) -> BlkCntEnaW<SdhcWrap_CtlCfg_CtlcfgTransferModeSpec> {
        BlkCntEnaW::new(self, 1)
    }
    #[doc = "Bits 2:3 - 3:2\\]
There are three methods to stop Multiple-block read and write operation. \\[1\\]
Auto CMD12 Enable: Multiple-block read and write commands for memory require CMD12 to stop the operation. When this field is set to 01b, the Host Controller issues CMD12 automatically when last block transfer is completed. Auto CMD12 error is indicated to the Auto CMD Error Status register. The Host Driver shall not set this bit if the command does not require CMD12. When Host Version 4 Enable =0, CMD12 is issued when 16-bit Block Count is expired. When Host Version 4 Enable =1, CMD12 is issued when 16-bit Block Count or 32-bit Block Count is expired. \\[2\\]
Auto CMD23 Enable: When this bit field is set to 10b, the Host Controller issues a CMD23 automatically before issuing a command specified in the Command register. The Host Controller Version 3.00 and later shall support this function. The following conditions are required to use the Auto CMD23. The following conditions are required to use the Auto CMD23. Auto CMD23 Supported \\[Host Controller Version is 3.00 or later\\]. A memory card that supports CMD23 \\[SCR\\[33\\]=1\\]. If DMA is used, it shall be ADMA. Only when CMD18 or CMD25 is issued. Auto CMD23 can be used with or without ADMA. By writing the Command register, the Host Controller issues a CMD23 first and then issues a command specified by the Command Index in Com-mand register. If response errors of CMD23 are detected, the second command is not issued. A CMD23 error is indicated in the Auto CMD Error Status register.32-bit block count value for CMD23 is set to 32-bit Block Count \\[SDMA System Address\\]
register. \\[3\\]
Auto CMD Auto Select \\[Version 4.10\\]
As CMD23 is optional for SD memory card except UHS 104 card, if card supports CMD23, Auto CMD 23 should be used instead of Auto CMD12. Host Controller Version 4.10 defines this Auto CMD Auto Select mode. Selection of Auto CMD depends on setting of CMD23 Enable in the Host Control 2 register which indicates whether card supports CMD23. If CMD23 Enable =1, Auto CMD23 is used and if CMD23 Enable =0, Auto CMD12 is used. In case of Version 4.10 or later, use of Auto CMD Auto Select is recommended rather than use of Auto CMD12 Enable or Auto CMD23 Enable."]
    #[inline(always)]
    #[must_use]
    pub fn auto_cmd_ena(&mut self) -> AutoCmdEnaW<SdhcWrap_CtlCfg_CtlcfgTransferModeSpec> {
        AutoCmdEnaW::new(self, 2)
    }
    #[doc = "Bit 4 - 4:4\\]
This bit defines the direction of data transfers."]
    #[inline(always)]
    #[must_use]
    pub fn data_xfer_dir(&mut self) -> DataXferDirW<SdhcWrap_CtlCfg_CtlcfgTransferModeSpec> {
        DataXferDirW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
This bit enables multiple block data transfers."]
    #[inline(always)]
    #[must_use]
    pub fn multi_blk_sel(&mut self) -> MultiBlkSelW<SdhcWrap_CtlCfg_CtlcfgTransferModeSpec> {
        MultiBlkSelW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
When response error check is enabled, this bit selects either R1 or R5 response types. Two types of response checks are supported: R1 for memory and R5 for SDIO. Error Statuses Checked in R1 .Bit31 OUT_OF_RANGE .Bit30 ADDRESS_ERROR .Bit29 BLOCK_LEN_ERROR .Bit26 WP_VIOLATION .Bit25 CARD_IS_LOCKED .Bit23 COM_CRC_ERROR .Bit21 CARD_ECC_FAILED .Bit20 CC_ERROR .Bit19 ERROR Response Flags Checked in R5 .Bit07 COM_CRC_ERROR .Bit03 ERROR .Bit01 FUNCTION_NUMBER .Bit00 OUT_OF_RANGE"]
    #[inline(always)]
    #[must_use]
    pub fn resp_type(&mut self) -> RespTypeW<SdhcWrap_CtlCfg_CtlcfgTransferModeSpec> {
        RespTypeW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver. Only R1 or R5 can be checked.If Host Driver checks response error, this bit is set to 0 and Response Interrupt Disable is set to 0.If Host Controller checks response error, sets this bit to 1 and sets Response Interrupt Disable to 1. Response Type R1 / R5 selects either R1 or R5 response type. If an error is detected, Response Error Interrupt is generated in the Response Error Interrupt Status register."]
    #[inline(always)]
    #[must_use]
    pub fn resp_err_chk_ena(&mut self) -> RespErrChkEnaW<SdhcWrap_CtlCfg_CtlcfgTransferModeSpec> {
        RespErrChkEnaW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver. Only R1 or R5 can be checked. If Host Driver checks response error,sets this bit to 0 and waits Command Complete Interrupt and then checks the response register. If Host Controller checks response error, sets this bit to 1 and sets Response Error Check Enable to 1. Command Complete Interrupt is disabled by this bit regardless of Command Complete Signal Enable."]
    #[inline(always)]
    #[must_use]
    pub fn resp_intr_dis(&mut self) -> RespIntrDisW<SdhcWrap_CtlCfg_CtlcfgTransferModeSpec> {
        RespIntrDisW::new(self, 8)
    }
}
#[doc = "This register is used to control the operations of data transfers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_transfer_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_transfer_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgTransferModeSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgTransferModeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_transfer_mode::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgTransferModeSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_transfer_mode::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgTransferModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_transfer_mode to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgTransferModeSpec {
    const RESET_VALUE: u16 = 0;
}
