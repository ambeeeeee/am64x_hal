#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_error_intr_sts` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_error_intr_sts` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsSpec>;
#[doc = "Field `CMD_TIMEOUT` reader - 0:0\\]
Occurs only if the no response is returned within 64 SDCLK cycles from the end bit of the command. If the HC detects a CMD line conflict, in which case Command CRC Error shall also be set. This bit shall be set without waiting for 64 SDCLK cycles because the command will be aborted by the HC."]
pub type CmdTimeoutR = crate::BitReader;
#[doc = "Field `CMD_TIMEOUT` writer - 0:0\\]
Occurs only if the no response is returned within 64 SDCLK cycles from the end bit of the command. If the HC detects a CMD line conflict, in which case Command CRC Error shall also be set. This bit shall be set without waiting for 64 SDCLK cycles because the command will be aborted by the HC."]
pub type CmdTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_CRC` reader - 1:1\\]
Command CRC Error is generated in two cases. 1. If a response is returned and the Command Time-out Error is set to 0, this bit is set to 1 when detecting a CRT error in the command response 2. The HC detects a CMD line conflict by monitoring the CMD line when a command is issued. If the HC drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SDCLK edge, then the HC shall abort the command \\[Stop driving CMD line\\]
and set this bit to 1. The Command Timeout Error shall also be set to 1 to distinguish CMD line conflict."]
pub type CmdCrcR = crate::BitReader;
#[doc = "Field `CMD_CRC` writer - 1:1\\]
Command CRC Error is generated in two cases. 1. If a response is returned and the Command Time-out Error is set to 0, this bit is set to 1 when detecting a CRT error in the command response 2. The HC detects a CMD line conflict by monitoring the CMD line when a command is issued. If the HC drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SDCLK edge, then the HC shall abort the command \\[Stop driving CMD line\\]
and set this bit to 1. The Command Timeout Error shall also be set to 1 to distinguish CMD line conflict."]
pub type CmdCrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_ENDBIT` reader - 2:2\\]
Occurs when detecting that the end bit of a command response is 0."]
pub type CmdEndbitR = crate::BitReader;
#[doc = "Field `CMD_ENDBIT` writer - 2:2\\]
Occurs when detecting that the end bit of a command response is 0."]
pub type CmdEndbitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_INDEX` reader - 3:3\\]
Occurs if a Command Index error occurs in the Command Response."]
pub type CmdIndexR = crate::BitReader;
#[doc = "Field `CMD_INDEX` writer - 3:3\\]
Occurs if a Command Index error occurs in the Command Response."]
pub type CmdIndexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_TIMEOUT` reader - 4:4\\]
Occurs when detecting one of following timeout conditions: 1. Busy Timeout for R1b, R5b type. 2. Busy Timeout after Write CRC status 3. Write CRC status Timeout 4. Read Data Timeout."]
pub type DataTimeoutR = crate::BitReader;
#[doc = "Field `DATA_TIMEOUT` writer - 4:4\\]
Occurs when detecting one of following timeout conditions: 1. Busy Timeout for R1b, R5b type. 2. Busy Timeout after Write CRC status 3. Write CRC status Timeout 4. Read Data Timeout."]
pub type DataTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_CRC` reader - 5:5\\]
Occurs when detecting CRC error when transferring read data which uses the DAT line or when detecting the Write CRC Status having a value of other than 010."]
pub type DataCrcR = crate::BitReader;
#[doc = "Field `DATA_CRC` writer - 5:5\\]
Occurs when detecting CRC error when transferring read data which uses the DAT line or when detecting the Write CRC Status having a value of other than 010."]
pub type DataCrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_ENDBIT` reader - 6:6\\]
Occurs when detecting 0 at the end bit position of read data which uses the DAT line or the end bit position of the CRC status."]
pub type DataEndbitR = crate::BitReader;
#[doc = "Field `DATA_ENDBIT` writer - 6:6\\]
Occurs when detecting 0 at the end bit position of read data which uses the DAT line or the end bit position of the CRC status."]
pub type DataEndbitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CURR_LIMIT` reader - 7:7\\]
By setting the SD Bus Power bit in the Power Control Register, the HC is requested to supply power for the SD Bus. If the HC supports the Current Limit Function, it can be protected from an Illegal card by stopping power supply to the card in which case this bit indicates a failure status. Reading 1 means the HC is not supplying power to SD card due to some failure. Reading 0 means that the HC is supplying power and no error has occurred. This bit shall always set to be 0, if the HC does not support this function."]
pub type CurrLimitR = crate::BitReader;
#[doc = "Field `CURR_LIMIT` writer - 7:7\\]
By setting the SD Bus Power bit in the Power Control Register, the HC is requested to supply power for the SD Bus. If the HC supports the Current Limit Function, it can be protected from an Illegal card by stopping power supply to the card in which case this bit indicates a failure status. Reading 1 means the HC is not supplying power to SD card due to some failure. Reading 0 means that the HC is supplying power and no error has occurred. This bit shall always set to be 0, if the HC does not support this function."]
pub type CurrLimitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_CMD` reader - 8:8\\]
Auto CMD12 and Auto CMD23 use this error status.This bit is set when detecting that any of the bits D00 to D05 in Auto CMD Error Status register has changed from 0 to 1. D07 is effective in case of Auto CMD12. Auto CMD Error Status register is valid while this bit is set to 1 and may be cleared with clearing of this bit \\[another implementation is also allowed\\]."]
pub type AutoCmdR = crate::BitReader;
#[doc = "Field `AUTO_CMD` writer - 8:8\\]
Auto CMD12 and Auto CMD23 use this error status.This bit is set when detecting that any of the bits D00 to D05 in Auto CMD Error Status register has changed from 0 to 1. D07 is effective in case of Auto CMD12. Auto CMD Error Status register is valid while this bit is set to 1 and may be cleared with clearing of this bit \\[another implementation is also allowed\\]."]
pub type AutoCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADMA` reader - 9:9\\]
This bit is set when the Host Controller detects errors during ADMA based data transfer. The state of the ADMA at an error occurrence is saved in the ADMA Error Status Register."]
pub type AdmaR = crate::BitReader;
#[doc = "Field `ADMA` writer - 9:9\\]
This bit is set when the Host Controller detects errors during ADMA based data transfer. The state of the ADMA at an error occurrence is saved in the ADMA Error Status Register."]
pub type AdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TUNING` reader - 10:10\\]
This bit is set when an unrecoverable error is detected in a tuning circuit except during tuning procedure \\[Occurrence of an error during tuning procedure is indicated by Sampling Select\\]. By detecting Tuning Error, Host Driver needs to abort a command executing and perform tuning.To reset tuning circuit, Sampling Clock shall be set to 0 before executing tuning procedure. The Tuning Error is higher priority than the other error interrupts generated during data transfer. By detecting Turning Error, the Host Driver should discard data transferred by a current read/write command and retry data transfer after the Host Controller retrieved from tuning circuit error."]
pub type TuningR = crate::BitReader;
#[doc = "Field `TUNING` writer - 10:10\\]
This bit is set when an unrecoverable error is detected in a tuning circuit except during tuning procedure \\[Occurrence of an error during tuning procedure is indicated by Sampling Select\\]. By detecting Tuning Error, Host Driver needs to abort a command executing and perform tuning.To reset tuning circuit, Sampling Clock shall be set to 0 before executing tuning procedure. The Tuning Error is higher priority than the other error interrupts generated during data transfer. By detecting Turning Error, the Host Driver should discard data transferred by a current read/write command and retry data transfer after the Host Controller retrieved from tuning circuit error."]
pub type TuningW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESP` reader - 11:11\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response, this bit is set to 1.If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response."]
pub type RespR = crate::BitReader;
#[doc = "Field `RESP` writer - 11:11\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response, this bit is set to 1.If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response."]
pub type RespW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOST` reader - 12:12\\]
Occurs when detecting ERROR in m_hresp\\[dma transaction\\]"]
pub type HostR = crate::BitReader;
#[doc = "Field `HOST` writer - 12:12\\]
Occurs when detecting ERROR in m_hresp\\[dma transaction\\]"]
pub type HostW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Occurs only if the no response is returned within 64 SDCLK cycles from the end bit of the command. If the HC detects a CMD line conflict, in which case Command CRC Error shall also be set. This bit shall be set without waiting for 64 SDCLK cycles because the command will be aborted by the HC."]
    #[inline(always)]
    pub fn cmd_timeout(&self) -> CmdTimeoutR {
        CmdTimeoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Command CRC Error is generated in two cases. 1. If a response is returned and the Command Time-out Error is set to 0, this bit is set to 1 when detecting a CRT error in the command response 2. The HC detects a CMD line conflict by monitoring the CMD line when a command is issued. If the HC drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SDCLK edge, then the HC shall abort the command \\[Stop driving CMD line\\]
and set this bit to 1. The Command Timeout Error shall also be set to 1 to distinguish CMD line conflict."]
    #[inline(always)]
    pub fn cmd_crc(&self) -> CmdCrcR {
        CmdCrcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Occurs when detecting that the end bit of a command response is 0."]
    #[inline(always)]
    pub fn cmd_endbit(&self) -> CmdEndbitR {
        CmdEndbitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Occurs if a Command Index error occurs in the Command Response."]
    #[inline(always)]
    pub fn cmd_index(&self) -> CmdIndexR {
        CmdIndexR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Occurs when detecting one of following timeout conditions: 1. Busy Timeout for R1b, R5b type. 2. Busy Timeout after Write CRC status 3. Write CRC status Timeout 4. Read Data Timeout."]
    #[inline(always)]
    pub fn data_timeout(&self) -> DataTimeoutR {
        DataTimeoutR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Occurs when detecting CRC error when transferring read data which uses the DAT line or when detecting the Write CRC Status having a value of other than 010."]
    #[inline(always)]
    pub fn data_crc(&self) -> DataCrcR {
        DataCrcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Occurs when detecting 0 at the end bit position of read data which uses the DAT line or the end bit position of the CRC status."]
    #[inline(always)]
    pub fn data_endbit(&self) -> DataEndbitR {
        DataEndbitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
By setting the SD Bus Power bit in the Power Control Register, the HC is requested to supply power for the SD Bus. If the HC supports the Current Limit Function, it can be protected from an Illegal card by stopping power supply to the card in which case this bit indicates a failure status. Reading 1 means the HC is not supplying power to SD card due to some failure. Reading 0 means that the HC is supplying power and no error has occurred. This bit shall always set to be 0, if the HC does not support this function."]
    #[inline(always)]
    pub fn curr_limit(&self) -> CurrLimitR {
        CurrLimitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Auto CMD12 and Auto CMD23 use this error status.This bit is set when detecting that any of the bits D00 to D05 in Auto CMD Error Status register has changed from 0 to 1. D07 is effective in case of Auto CMD12. Auto CMD Error Status register is valid while this bit is set to 1 and may be cleared with clearing of this bit \\[another implementation is also allowed\\]."]
    #[inline(always)]
    pub fn auto_cmd(&self) -> AutoCmdR {
        AutoCmdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
This bit is set when the Host Controller detects errors during ADMA based data transfer. The state of the ADMA at an error occurrence is saved in the ADMA Error Status Register."]
    #[inline(always)]
    pub fn adma(&self) -> AdmaR {
        AdmaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
This bit is set when an unrecoverable error is detected in a tuning circuit except during tuning procedure \\[Occurrence of an error during tuning procedure is indicated by Sampling Select\\]. By detecting Tuning Error, Host Driver needs to abort a command executing and perform tuning.To reset tuning circuit, Sampling Clock shall be set to 0 before executing tuning procedure. The Tuning Error is higher priority than the other error interrupts generated during data transfer. By detecting Turning Error, the Host Driver should discard data transferred by a current read/write command and retry data transfer after the Host Controller retrieved from tuning circuit error."]
    #[inline(always)]
    pub fn tuning(&self) -> TuningR {
        TuningR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response, this bit is set to 1.If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response."]
    #[inline(always)]
    pub fn resp(&self) -> RespR {
        RespR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Occurs when detecting ERROR in m_hresp\\[dma transaction\\]"]
    #[inline(always)]
    pub fn host(&self) -> HostR {
        HostR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Occurs only if the no response is returned within 64 SDCLK cycles from the end bit of the command. If the HC detects a CMD line conflict, in which case Command CRC Error shall also be set. This bit shall be set without waiting for 64 SDCLK cycles because the command will be aborted by the HC."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_timeout(&mut self) -> CmdTimeoutW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsSpec> {
        CmdTimeoutW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Command CRC Error is generated in two cases. 1. If a response is returned and the Command Time-out Error is set to 0, this bit is set to 1 when detecting a CRT error in the command response 2. The HC detects a CMD line conflict by monitoring the CMD line when a command is issued. If the HC drives the CMD line to 1 level, but detects 0 level on the CMD line at the next SDCLK edge, then the HC shall abort the command \\[Stop driving CMD line\\]
and set this bit to 1. The Command Timeout Error shall also be set to 1 to distinguish CMD line conflict."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crc(&mut self) -> CmdCrcW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsSpec> {
        CmdCrcW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Occurs when detecting that the end bit of a command response is 0."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_endbit(&mut self) -> CmdEndbitW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsSpec> {
        CmdEndbitW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Occurs if a Command Index error occurs in the Command Response."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_index(&mut self) -> CmdIndexW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsSpec> {
        CmdIndexW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Occurs when detecting one of following timeout conditions: 1. Busy Timeout for R1b, R5b type. 2. Busy Timeout after Write CRC status 3. Write CRC status Timeout 4. Read Data Timeout."]
    #[inline(always)]
    #[must_use]
    pub fn data_timeout(&mut self) -> DataTimeoutW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsSpec> {
        DataTimeoutW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Occurs when detecting CRC error when transferring read data which uses the DAT line or when detecting the Write CRC Status having a value of other than 010."]
    #[inline(always)]
    #[must_use]
    pub fn data_crc(&mut self) -> DataCrcW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsSpec> {
        DataCrcW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Occurs when detecting 0 at the end bit position of read data which uses the DAT line or the end bit position of the CRC status."]
    #[inline(always)]
    #[must_use]
    pub fn data_endbit(&mut self) -> DataEndbitW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsSpec> {
        DataEndbitW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
By setting the SD Bus Power bit in the Power Control Register, the HC is requested to supply power for the SD Bus. If the HC supports the Current Limit Function, it can be protected from an Illegal card by stopping power supply to the card in which case this bit indicates a failure status. Reading 1 means the HC is not supplying power to SD card due to some failure. Reading 0 means that the HC is supplying power and no error has occurred. This bit shall always set to be 0, if the HC does not support this function."]
    #[inline(always)]
    #[must_use]
    pub fn curr_limit(&mut self) -> CurrLimitW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsSpec> {
        CurrLimitW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Auto CMD12 and Auto CMD23 use this error status.This bit is set when detecting that any of the bits D00 to D05 in Auto CMD Error Status register has changed from 0 to 1. D07 is effective in case of Auto CMD12. Auto CMD Error Status register is valid while this bit is set to 1 and may be cleared with clearing of this bit \\[another implementation is also allowed\\]."]
    #[inline(always)]
    #[must_use]
    pub fn auto_cmd(&mut self) -> AutoCmdW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsSpec> {
        AutoCmdW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
This bit is set when the Host Controller detects errors during ADMA based data transfer. The state of the ADMA at an error occurrence is saved in the ADMA Error Status Register."]
    #[inline(always)]
    #[must_use]
    pub fn adma(&mut self) -> AdmaW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsSpec> {
        AdmaW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
This bit is set when an unrecoverable error is detected in a tuning circuit except during tuning procedure \\[Occurrence of an error during tuning procedure is indicated by Sampling Select\\]. By detecting Tuning Error, Host Driver needs to abort a command executing and perform tuning.To reset tuning circuit, Sampling Clock shall be set to 0 before executing tuning procedure. The Tuning Error is higher priority than the other error interrupts generated during data transfer. By detecting Turning Error, the Host Driver should discard data transferred by a current read/write command and retry data transfer after the Host Controller retrieved from tuning circuit error."]
    #[inline(always)]
    #[must_use]
    pub fn tuning(&mut self) -> TuningW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsSpec> {
        TuningW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response. If an error is detected in a response, this bit is set to 1.If Response Error Check Enable is set to 1 in the Transfer Mode register, Host Controller Checks R1 or R5 response."]
    #[inline(always)]
    #[must_use]
    pub fn resp(&mut self) -> RespW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsSpec> {
        RespW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Occurs when detecting ERROR in m_hresp\\[dma transaction\\]"]
    #[inline(always)]
    #[must_use]
    pub fn host(&mut self) -> HostW<SdhcWrap_CtlCfg_CtlcfgErrorIntrStsSpec> {
        HostW::new(self, 12)
    }
}
#[doc = "This register gives the status of the error interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgErrorIntrStsSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgErrorIntrStsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgErrorIntrStsSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_error_intr_sts::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgErrorIntrStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_error_intr_sts to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgErrorIntrStsSpec {
    const RESET_VALUE: u16 = 0;
}
