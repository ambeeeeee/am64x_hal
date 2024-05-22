#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_command` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgCommandSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_command` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgCommandSpec>;
#[doc = "Field `RESP_TYPE_SEL` reader - 1:0\\]
Response Type Select."]
pub type RespTypeSelR = crate::FieldReader;
#[doc = "Field `RESP_TYPE_SEL` writer - 1:0\\]
Response Type Select."]
pub type RespTypeSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SUB_CMD` reader - 2:2\\]
This bit is added from Version 4.10 to distinguish a main command or sub command \\[Refer to Section 1.17\\]. When issuing a main com-mand, this bit is set to 0 and when issuing a sub command, this bit is set to 1. Setting of this bit is checked by Sub Command Status in the Present State register.Host Driver manages whether main or sub command. Host Controller does not refer to this bit to issue a command."]
pub type SubCmdR = crate::BitReader;
#[doc = "Field `SUB_CMD` writer - 2:2\\]
This bit is added from Version 4.10 to distinguish a main command or sub command \\[Refer to Section 1.17\\]. When issuing a main com-mand, this bit is set to 0 and when issuing a sub command, this bit is set to 1. Setting of this bit is checked by Sub Command Status in the Present State register.Host Driver manages whether main or sub command. Host Controller does not refer to this bit to issue a command."]
pub type SubCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_CRC_CHK_ENA` reader - 3:3\\]
If this bit is set to 1, the HC shall check the CRC field in the response. If an error is detected, it is reported as a Command CRC Error. If this bit is set to 0, the CRC field is not checked."]
pub type CmdCrcChkEnaR = crate::BitReader;
#[doc = "Field `CMD_CRC_CHK_ENA` writer - 3:3\\]
If this bit is set to 1, the HC shall check the CRC field in the response. If an error is detected, it is reported as a Command CRC Error. If this bit is set to 0, the CRC field is not checked."]
pub type CmdCrcChkEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_INDEX_CHK_ENA` reader - 4:4\\]
If this bit is set to 1, the HC shall check the index field in the response to see if it has the same value as the command index. If it is not, it is reported as a Command Index Error. If this bit is set to 0, the Index field is not checked."]
pub type CmdIndexChkEnaR = crate::BitReader;
#[doc = "Field `CMD_INDEX_CHK_ENA` writer - 4:4\\]
If this bit is set to 1, the HC shall check the index field in the response to see if it has the same value as the command index. If it is not, it is reported as a Command Index Error. If this bit is set to 0, the Index field is not checked."]
pub type CmdIndexChkEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_PRESENT` reader - 5:5\\]
This bit is set to 1 to indicate that data is present and shall be transferred using the DAT line. If is set to 0 for the following: 1. Commands using only CMD line \\[ex. CMD52\\]. 2. Commands with no data transferbut using busy signal on DAT\\[0\\]line \\[R1b or R5b ex. CMD38\\]. 3. Resume Command."]
pub type DataPresentR = crate::BitReader;
#[doc = "Field `DATA_PRESENT` writer - 5:5\\]
This bit is set to 1 to indicate that data is present and shall be transferred using the DAT line. If is set to 0 for the following: 1. Commands using only CMD line \\[ex. CMD52\\]. 2. Commands with no data transferbut using busy signal on DAT\\[0\\]line \\[R1b or R5b ex. CMD38\\]. 3. Resume Command."]
pub type DataPresentW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_TYPE` reader - 7:6\\]
There are three types of special commands. Suspend, Resume andAbort. These bits shall bet set to 00b for all other commands. Suspend Command: If the Suspend command succeeds, the HC shall assume the SD Bus has been released and that it is possible to issue the next command which uses the DAT line. The HC shall de-assert Read Wait for read transactions and stop checking busy for write transactions. The Interrupt cycle shall start, in 4-bit mode. If the Suspend command fails, the HC shall maintain its current state. and the HD shall restart the transfer by setting Continue Request in the Block Gap Control Register. Resume Command: The HD re-starts the data transfer by restoring the registers in the range of 000-00Dh. The HC shall check for busy before starting write transfers. Abort Command: If this command is set when exe- cuting a read transfer, the HC shall stop reads to the buffer. If this command is set when executing a writetransfer, the HC shall stop driving the DAT line. After issuing the Abort command, the HD should issue a software reset."]
pub type CmdTypeR = crate::FieldReader;
#[doc = "Field `CMD_TYPE` writer - 7:6\\]
There are three types of special commands. Suspend, Resume andAbort. These bits shall bet set to 00b for all other commands. Suspend Command: If the Suspend command succeeds, the HC shall assume the SD Bus has been released and that it is possible to issue the next command which uses the DAT line. The HC shall de-assert Read Wait for read transactions and stop checking busy for write transactions. The Interrupt cycle shall start, in 4-bit mode. If the Suspend command fails, the HC shall maintain its current state. and the HD shall restart the transfer by setting Continue Request in the Block Gap Control Register. Resume Command: The HD re-starts the data transfer by restoring the registers in the range of 000-00Dh. The HC shall check for busy before starting write transfers. Abort Command: If this command is set when exe- cuting a read transfer, the HC shall stop reads to the buffer. If this command is set when executing a writetransfer, the HC shall stop driving the DAT line. After issuing the Abort command, the HD should issue a software reset."]
pub type CmdTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMD_INDEX` reader - 13:8\\]
This bit shall be set to the command number \\[CMD0-63, ACMD0-63\\]."]
pub type CmdIndexR = crate::FieldReader;
#[doc = "Field `CMD_INDEX` writer - 13:8\\]
This bit shall be set to the command number \\[CMD0-63, ACMD0-63\\]."]
pub type CmdIndexW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Response Type Select."]
    #[inline(always)]
    pub fn resp_type_sel(&self) -> RespTypeSelR {
        RespTypeSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit is added from Version 4.10 to distinguish a main command or sub command \\[Refer to Section 1.17\\]. When issuing a main com-mand, this bit is set to 0 and when issuing a sub command, this bit is set to 1. Setting of this bit is checked by Sub Command Status in the Present State register.Host Driver manages whether main or sub command. Host Controller does not refer to this bit to issue a command."]
    #[inline(always)]
    pub fn sub_cmd(&self) -> SubCmdR {
        SubCmdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
If this bit is set to 1, the HC shall check the CRC field in the response. If an error is detected, it is reported as a Command CRC Error. If this bit is set to 0, the CRC field is not checked."]
    #[inline(always)]
    pub fn cmd_crc_chk_ena(&self) -> CmdCrcChkEnaR {
        CmdCrcChkEnaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
If this bit is set to 1, the HC shall check the index field in the response to see if it has the same value as the command index. If it is not, it is reported as a Command Index Error. If this bit is set to 0, the Index field is not checked."]
    #[inline(always)]
    pub fn cmd_index_chk_ena(&self) -> CmdIndexChkEnaR {
        CmdIndexChkEnaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
This bit is set to 1 to indicate that data is present and shall be transferred using the DAT line. If is set to 0 for the following: 1. Commands using only CMD line \\[ex. CMD52\\]. 2. Commands with no data transferbut using busy signal on DAT\\[0\\]line \\[R1b or R5b ex. CMD38\\]. 3. Resume Command."]
    #[inline(always)]
    pub fn data_present(&self) -> DataPresentR {
        DataPresentR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
There are three types of special commands. Suspend, Resume andAbort. These bits shall bet set to 00b for all other commands. Suspend Command: If the Suspend command succeeds, the HC shall assume the SD Bus has been released and that it is possible to issue the next command which uses the DAT line. The HC shall de-assert Read Wait for read transactions and stop checking busy for write transactions. The Interrupt cycle shall start, in 4-bit mode. If the Suspend command fails, the HC shall maintain its current state. and the HD shall restart the transfer by setting Continue Request in the Block Gap Control Register. Resume Command: The HD re-starts the data transfer by restoring the registers in the range of 000-00Dh. The HC shall check for busy before starting write transfers. Abort Command: If this command is set when exe- cuting a read transfer, the HC shall stop reads to the buffer. If this command is set when executing a writetransfer, the HC shall stop driving the DAT line. After issuing the Abort command, the HD should issue a software reset."]
    #[inline(always)]
    pub fn cmd_type(&self) -> CmdTypeR {
        CmdTypeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
This bit shall be set to the command number \\[CMD0-63, ACMD0-63\\]."]
    #[inline(always)]
    pub fn cmd_index(&self) -> CmdIndexR {
        CmdIndexR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Response Type Select."]
    #[inline(always)]
    #[must_use]
    pub fn resp_type_sel(&mut self) -> RespTypeSelW<SdhcWrap_CtlCfg_CtlcfgCommandSpec> {
        RespTypeSelW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit is added from Version 4.10 to distinguish a main command or sub command \\[Refer to Section 1.17\\]. When issuing a main com-mand, this bit is set to 0 and when issuing a sub command, this bit is set to 1. Setting of this bit is checked by Sub Command Status in the Present State register.Host Driver manages whether main or sub command. Host Controller does not refer to this bit to issue a command."]
    #[inline(always)]
    #[must_use]
    pub fn sub_cmd(&mut self) -> SubCmdW<SdhcWrap_CtlCfg_CtlcfgCommandSpec> {
        SubCmdW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
If this bit is set to 1, the HC shall check the CRC field in the response. If an error is detected, it is reported as a Command CRC Error. If this bit is set to 0, the CRC field is not checked."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_crc_chk_ena(&mut self) -> CmdCrcChkEnaW<SdhcWrap_CtlCfg_CtlcfgCommandSpec> {
        CmdCrcChkEnaW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
If this bit is set to 1, the HC shall check the index field in the response to see if it has the same value as the command index. If it is not, it is reported as a Command Index Error. If this bit is set to 0, the Index field is not checked."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_index_chk_ena(&mut self) -> CmdIndexChkEnaW<SdhcWrap_CtlCfg_CtlcfgCommandSpec> {
        CmdIndexChkEnaW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
This bit is set to 1 to indicate that data is present and shall be transferred using the DAT line. If is set to 0 for the following: 1. Commands using only CMD line \\[ex. CMD52\\]. 2. Commands with no data transferbut using busy signal on DAT\\[0\\]line \\[R1b or R5b ex. CMD38\\]. 3. Resume Command."]
    #[inline(always)]
    #[must_use]
    pub fn data_present(&mut self) -> DataPresentW<SdhcWrap_CtlCfg_CtlcfgCommandSpec> {
        DataPresentW::new(self, 5)
    }
    #[doc = "Bits 6:7 - 7:6\\]
There are three types of special commands. Suspend, Resume andAbort. These bits shall bet set to 00b for all other commands. Suspend Command: If the Suspend command succeeds, the HC shall assume the SD Bus has been released and that it is possible to issue the next command which uses the DAT line. The HC shall de-assert Read Wait for read transactions and stop checking busy for write transactions. The Interrupt cycle shall start, in 4-bit mode. If the Suspend command fails, the HC shall maintain its current state. and the HD shall restart the transfer by setting Continue Request in the Block Gap Control Register. Resume Command: The HD re-starts the data transfer by restoring the registers in the range of 000-00Dh. The HC shall check for busy before starting write transfers. Abort Command: If this command is set when exe- cuting a read transfer, the HC shall stop reads to the buffer. If this command is set when executing a writetransfer, the HC shall stop driving the DAT line. After issuing the Abort command, the HD should issue a software reset."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_type(&mut self) -> CmdTypeW<SdhcWrap_CtlCfg_CtlcfgCommandSpec> {
        CmdTypeW::new(self, 6)
    }
    #[doc = "Bits 8:13 - 13:8\\]
This bit shall be set to the command number \\[CMD0-63, ACMD0-63\\]."]
    #[inline(always)]
    #[must_use]
    pub fn cmd_index(&mut self) -> CmdIndexW<SdhcWrap_CtlCfg_CtlcfgCommandSpec> {
        CmdIndexW::new(self, 8)
    }
}
#[doc = "This register is used to program the Command for host controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_command::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_command::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgCommandSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgCommandSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_command::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgCommandSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_command::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgCommandSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_command to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgCommandSpec {
    const RESET_VALUE: u16 = 0;
}
