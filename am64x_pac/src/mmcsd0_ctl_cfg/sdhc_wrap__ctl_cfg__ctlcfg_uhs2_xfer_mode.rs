#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_xfer_mode` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgUhs2XferModeSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_xfer_mode` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgUhs2XferModeSpec>;
#[doc = "Field `DMA_ENA` reader - 0:0\\]
This bit selects whether DMA is used or not and is effective to a command with data transfer. One of DMA types is selected by DMA Select in the Host Control 1 register."]
pub type DmaEnaR = crate::BitReader;
#[doc = "Field `DMA_ENA` writer - 0:0\\]
This bit selects whether DMA is used or not and is effective to a command with data transfer. One of DMA types is selected by DMA Select in the Host Control 1 register."]
pub type DmaEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_CNT_ENA` reader - 1:1\\]
This bit specifies whether data transfer usesUHS-II Block Count register. If this bit is set to 1, data transfer is terminated by Block Count. Setting to UHS-II Block Count register shall be equivalent to TLEN in UHS-II Command Packet reg-ister."]
pub type BlkCntEnaR = crate::BitReader;
#[doc = "Field `BLK_CNT_ENA` writer - 1:1\\]
This bit specifies whether data transfer usesUHS-II Block Count register. If this bit is set to 1, data transfer is terminated by Block Count. Setting to UHS-II Block Count register shall be equivalent to TLEN in UHS-II Command Packet reg-ister."]
pub type BlkCntEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_XFER_DIR` reader - 4:4\\]
This bit specifies direction of data trans-fer when Data Present is set to 1. This bit is effective to a command with data transfer. 0 - Read \\[Card to Host\\]
1 - Write \\[Host to Card\\]"]
pub type DataXferDirR = crate::BitReader;
#[doc = "Field `DATA_XFER_DIR` writer - 4:4\\]
This bit specifies direction of data trans-fer when Data Present is set to 1. This bit is effective to a command with data transfer. 0 - Read \\[Card to Host\\]
1 - Write \\[Host to Card\\]"]
pub type DataXferDirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYTE_MODE` reader - 5:5\\]
This bit specifies whether data transfer is in byte mode or block mode when Data Present is set to 1. This bit is effective to a command with data trans-fer."]
pub type ByteModeR = crate::BitReader;
#[doc = "Field `BYTE_MODE` writer - 5:5\\]
This bit specifies whether data transfer is in byte mode or block mode when Data Present is set to 1. This bit is effective to a command with data trans-fer."]
pub type ByteModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_TYPE` reader - 6:6\\]
When response error check is enabled, this bit selects either R1 or R5 response types. Two types of response checks are supported: R1 for memory and R5 for SDIO. Error Statuses Checked in R1 Bit31 OUT_OF_RANGE Bit30 ADDRESS_ERROR Bit29 BLOCK_LEN_ERROR Bit26 WP_VIOLATION Bit25 CARD_IS_LOCKED Bit23 COM_CRC_ERROR Bit21 CARD_ECC_FAILED Bit20 CC_ERROR Bit19 ERROR Response Flags Checked in R5 .Bit07 COM_CRC_ERROR .Bit03 ERROR .Bit01 FUNCTION_NUMBER .Bit00 OUT_OF_RANGE"]
pub type RespTypeR = crate::BitReader;
#[doc = "Field `RESP_TYPE` writer - 6:6\\]
When response error check is enabled, this bit selects either R1 or R5 response types. Two types of response checks are supported: R1 for memory and R5 for SDIO. Error Statuses Checked in R1 Bit31 OUT_OF_RANGE Bit30 ADDRESS_ERROR Bit29 BLOCK_LEN_ERROR Bit26 WP_VIOLATION Bit25 CARD_IS_LOCKED Bit23 COM_CRC_ERROR Bit21 CARD_ECC_FAILED Bit20 CC_ERROR Bit19 ERROR Response Flags Checked in R5 .Bit07 COM_CRC_ERROR .Bit03 ERROR .Bit01 FUNCTION_NUMBER .Bit00 OUT_OF_RANGE"]
pub type RespTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_ERR_CHK_ENA` reader - 7:7\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver.Only R1 or R5 can be checked. If Host Driver checks response error, this bit is set to 0 and Response Inter-rupt Disable is set to 0. If Host Control-ler checks response error, sets this bit to 1 and sets Response Interrupt Dis-able to 1. Response Type R1 / R5 selects either R1 or R5 response type. If an error is detected, RES Packet Error Interrupt is generated in the UHS-II Error Interrupt Status register."]
pub type RespErrChkEnaR = crate::BitReader;
#[doc = "Field `RESP_ERR_CHK_ENA` writer - 7:7\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver.Only R1 or R5 can be checked. If Host Driver checks response error, this bit is set to 0 and Response Inter-rupt Disable is set to 0. If Host Control-ler checks response error, sets this bit to 1 and sets Response Interrupt Dis-able to 1. Response Type R1 / R5 selects either R1 or R5 response type. If an error is detected, RES Packet Error Interrupt is generated in the UHS-II Error Interrupt Status register."]
pub type RespErrChkEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP_INTR_DIS` reader - 8:8\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver. Only R1 or R5 can be checked. If Host Driver checks response error, sets this bit to 0 and waits Command Complete Interrupt and then check the response register. If Host Controller checks response error, sets this bit to 1 and sets Response Error Check Enable to 1. Command Complete Interrupt is disabled by this bit regardless of Com-mand Complete Signal Enable."]
pub type RespIntrDisR = crate::BitReader;
#[doc = "Field `RESP_INTR_DIS` writer - 8:8\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver. Only R1 or R5 can be checked. If Host Driver checks response error, sets this bit to 0 and waits Command Complete Interrupt and then check the response register. If Host Controller checks response error, sets this bit to 1 and sets Response Error Check Enable to 1. Command Complete Interrupt is disabled by this bit regardless of Com-mand Complete Signal Enable."]
pub type RespIntrDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EBSY_WAIT` reader - 14:14\\]
This bit is set when issuing a command which is accompanied by EBSY packet to indicate end of command execution. Busy is expected for CCMD with R1b/R5b type and DCMD with data transfer.If this bit is set to 1, Host Controller waits receiving of EBSY packet and on receiving EBSY packet, Transfer Com-plete in the Normal Interrupt Status reg-ister is set to 1 to indicate end ofbusy. If an error is indicated in EBSY packet \\[ex. Memory Error\\], EBSY Error in the UHS-II Error Interrupt Status register is set to 1. Setting of EBSY Error also sets Error Interrupt to 1 in the Normal Inter-rupt Status register. Error Interrupt and Transfer Complete shall be set together. '1' Wait EBSY, '0' Issue a command without busy."]
pub type EbsyWaitR = crate::BitReader;
#[doc = "Field `EBSY_WAIT` writer - 14:14\\]
This bit is set when issuing a command which is accompanied by EBSY packet to indicate end of command execution. Busy is expected for CCMD with R1b/R5b type and DCMD with data transfer.If this bit is set to 1, Host Controller waits receiving of EBSY packet and on receiving EBSY packet, Transfer Com-plete in the Normal Interrupt Status reg-ister is set to 1 to indicate end ofbusy. If an error is indicated in EBSY packet \\[ex. Memory Error\\], EBSY Error in the UHS-II Error Interrupt Status register is set to 1. Setting of EBSY Error also sets Error Interrupt to 1 in the Normal Inter-rupt Status register. Error Interrupt and Transfer Complete shall be set together. '1' Wait EBSY, '0' Issue a command without busy."]
pub type EbsyWaitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUPLEX_SELECT` reader - 15:15\\]
Use of 2 lane half duplex mode is determined by Host Driver."]
pub type DuplexSelectR = crate::BitReader;
#[doc = "Field `DUPLEX_SELECT` writer - 15:15\\]
Use of 2 lane half duplex mode is determined by Host Driver."]
pub type DuplexSelectW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit selects whether DMA is used or not and is effective to a command with data transfer. One of DMA types is selected by DMA Select in the Host Control 1 register."]
    #[inline(always)]
    pub fn dma_ena(&self) -> DmaEnaR {
        DmaEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit specifies whether data transfer usesUHS-II Block Count register. If this bit is set to 1, data transfer is terminated by Block Count. Setting to UHS-II Block Count register shall be equivalent to TLEN in UHS-II Command Packet reg-ister."]
    #[inline(always)]
    pub fn blk_cnt_ena(&self) -> BlkCntEnaR {
        BlkCntEnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
This bit specifies direction of data trans-fer when Data Present is set to 1. This bit is effective to a command with data transfer. 0 - Read \\[Card to Host\\]
1 - Write \\[Host to Card\\]"]
    #[inline(always)]
    pub fn data_xfer_dir(&self) -> DataXferDirR {
        DataXferDirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
This bit specifies whether data transfer is in byte mode or block mode when Data Present is set to 1. This bit is effective to a command with data trans-fer."]
    #[inline(always)]
    pub fn byte_mode(&self) -> ByteModeR {
        ByteModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
When response error check is enabled, this bit selects either R1 or R5 response types. Two types of response checks are supported: R1 for memory and R5 for SDIO. Error Statuses Checked in R1 Bit31 OUT_OF_RANGE Bit30 ADDRESS_ERROR Bit29 BLOCK_LEN_ERROR Bit26 WP_VIOLATION Bit25 CARD_IS_LOCKED Bit23 COM_CRC_ERROR Bit21 CARD_ECC_FAILED Bit20 CC_ERROR Bit19 ERROR Response Flags Checked in R5 .Bit07 COM_CRC_ERROR .Bit03 ERROR .Bit01 FUNCTION_NUMBER .Bit00 OUT_OF_RANGE"]
    #[inline(always)]
    pub fn resp_type(&self) -> RespTypeR {
        RespTypeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver.Only R1 or R5 can be checked. If Host Driver checks response error, this bit is set to 0 and Response Inter-rupt Disable is set to 0. If Host Control-ler checks response error, sets this bit to 1 and sets Response Interrupt Dis-able to 1. Response Type R1 / R5 selects either R1 or R5 response type. If an error is detected, RES Packet Error Interrupt is generated in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    pub fn resp_err_chk_ena(&self) -> RespErrChkEnaR {
        RespErrChkEnaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver. Only R1 or R5 can be checked. If Host Driver checks response error, sets this bit to 0 and waits Command Complete Interrupt and then check the response register. If Host Controller checks response error, sets this bit to 1 and sets Response Error Check Enable to 1. Command Complete Interrupt is disabled by this bit regardless of Com-mand Complete Signal Enable."]
    #[inline(always)]
    pub fn resp_intr_dis(&self) -> RespIntrDisR {
        RespIntrDisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
This bit is set when issuing a command which is accompanied by EBSY packet to indicate end of command execution. Busy is expected for CCMD with R1b/R5b type and DCMD with data transfer.If this bit is set to 1, Host Controller waits receiving of EBSY packet and on receiving EBSY packet, Transfer Com-plete in the Normal Interrupt Status reg-ister is set to 1 to indicate end ofbusy. If an error is indicated in EBSY packet \\[ex. Memory Error\\], EBSY Error in the UHS-II Error Interrupt Status register is set to 1. Setting of EBSY Error also sets Error Interrupt to 1 in the Normal Inter-rupt Status register. Error Interrupt and Transfer Complete shall be set together. '1' Wait EBSY, '0' Issue a command without busy."]
    #[inline(always)]
    pub fn ebsy_wait(&self) -> EbsyWaitR {
        EbsyWaitR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Use of 2 lane half duplex mode is determined by Host Driver."]
    #[inline(always)]
    pub fn duplex_select(&self) -> DuplexSelectR {
        DuplexSelectR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit selects whether DMA is used or not and is effective to a command with data transfer. One of DMA types is selected by DMA Select in the Host Control 1 register."]
    #[inline(always)]
    #[must_use]
    pub fn dma_ena(&mut self) -> DmaEnaW<SdhcWrap_CtlCfg_CtlcfgUhs2XferModeSpec> {
        DmaEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit specifies whether data transfer usesUHS-II Block Count register. If this bit is set to 1, data transfer is terminated by Block Count. Setting to UHS-II Block Count register shall be equivalent to TLEN in UHS-II Command Packet reg-ister."]
    #[inline(always)]
    #[must_use]
    pub fn blk_cnt_ena(&mut self) -> BlkCntEnaW<SdhcWrap_CtlCfg_CtlcfgUhs2XferModeSpec> {
        BlkCntEnaW::new(self, 1)
    }
    #[doc = "Bit 4 - 4:4\\]
This bit specifies direction of data trans-fer when Data Present is set to 1. This bit is effective to a command with data transfer. 0 - Read \\[Card to Host\\]
1 - Write \\[Host to Card\\]"]
    #[inline(always)]
    #[must_use]
    pub fn data_xfer_dir(&mut self) -> DataXferDirW<SdhcWrap_CtlCfg_CtlcfgUhs2XferModeSpec> {
        DataXferDirW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
This bit specifies whether data transfer is in byte mode or block mode when Data Present is set to 1. This bit is effective to a command with data trans-fer."]
    #[inline(always)]
    #[must_use]
    pub fn byte_mode(&mut self) -> ByteModeW<SdhcWrap_CtlCfg_CtlcfgUhs2XferModeSpec> {
        ByteModeW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
When response error check is enabled, this bit selects either R1 or R5 response types. Two types of response checks are supported: R1 for memory and R5 for SDIO. Error Statuses Checked in R1 Bit31 OUT_OF_RANGE Bit30 ADDRESS_ERROR Bit29 BLOCK_LEN_ERROR Bit26 WP_VIOLATION Bit25 CARD_IS_LOCKED Bit23 COM_CRC_ERROR Bit21 CARD_ECC_FAILED Bit20 CC_ERROR Bit19 ERROR Response Flags Checked in R5 .Bit07 COM_CRC_ERROR .Bit03 ERROR .Bit01 FUNCTION_NUMBER .Bit00 OUT_OF_RANGE"]
    #[inline(always)]
    #[must_use]
    pub fn resp_type(&mut self) -> RespTypeW<SdhcWrap_CtlCfg_CtlcfgUhs2XferModeSpec> {
        RespTypeW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver.Only R1 or R5 can be checked. If Host Driver checks response error, this bit is set to 0 and Response Inter-rupt Disable is set to 0. If Host Control-ler checks response error, sets this bit to 1 and sets Response Interrupt Dis-able to 1. Response Type R1 / R5 selects either R1 or R5 response type. If an error is detected, RES Packet Error Interrupt is generated in the UHS-II Error Interrupt Status register."]
    #[inline(always)]
    #[must_use]
    pub fn resp_err_chk_ena(&mut self) -> RespErrChkEnaW<SdhcWrap_CtlCfg_CtlcfgUhs2XferModeSpec> {
        RespErrChkEnaW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver. Only R1 or R5 can be checked. If Host Driver checks response error, sets this bit to 0 and waits Command Complete Interrupt and then check the response register. If Host Controller checks response error, sets this bit to 1 and sets Response Error Check Enable to 1. Command Complete Interrupt is disabled by this bit regardless of Com-mand Complete Signal Enable."]
    #[inline(always)]
    #[must_use]
    pub fn resp_intr_dis(&mut self) -> RespIntrDisW<SdhcWrap_CtlCfg_CtlcfgUhs2XferModeSpec> {
        RespIntrDisW::new(self, 8)
    }
    #[doc = "Bit 14 - 14:14\\]
This bit is set when issuing a command which is accompanied by EBSY packet to indicate end of command execution. Busy is expected for CCMD with R1b/R5b type and DCMD with data transfer.If this bit is set to 1, Host Controller waits receiving of EBSY packet and on receiving EBSY packet, Transfer Com-plete in the Normal Interrupt Status reg-ister is set to 1 to indicate end ofbusy. If an error is indicated in EBSY packet \\[ex. Memory Error\\], EBSY Error in the UHS-II Error Interrupt Status register is set to 1. Setting of EBSY Error also sets Error Interrupt to 1 in the Normal Inter-rupt Status register. Error Interrupt and Transfer Complete shall be set together. '1' Wait EBSY, '0' Issue a command without busy."]
    #[inline(always)]
    #[must_use]
    pub fn ebsy_wait(&mut self) -> EbsyWaitW<SdhcWrap_CtlCfg_CtlcfgUhs2XferModeSpec> {
        EbsyWaitW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Use of 2 lane half duplex mode is determined by Host Driver."]
    #[inline(always)]
    #[must_use]
    pub fn duplex_select(&mut self) -> DuplexSelectW<SdhcWrap_CtlCfg_CtlcfgUhs2XferModeSpec> {
        DuplexSelectW::new(self, 15)
    }
}
#[doc = "This register is used to control the operations of data transfers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_xfer_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_xfer_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgUhs2XferModeSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgUhs2XferModeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_xfer_mode::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgUhs2XferModeSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_uhs2_xfer_mode::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgUhs2XferModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_uhs2_xfer_mode to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgUhs2XferModeSpec {
    const RESET_VALUE: u16 = 0;
}
