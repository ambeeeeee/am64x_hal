#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_presentstate` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_presentstate` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec>;
#[doc = "Field `INHIBIT_CMD` reader - 0:0\\]
SD Mode If this bit is 0, it indicates the CMD line is not in use and the HC can issue a SD command using the CMD line. This bit is set immediately after the Command register \\[00Fh\\]
is written. This bit is cleared when the command response is received. Even if the Command Inhibit \\[DAT\\]
is set to 1, Commands using only the CMD line can be issued if this bit is 0. Changing from 1 to 0 gener- ates a Command complete interrupt in the Normal Interrupt Status register. If the HC cannot issuethe command because of a command conflict error or because of Command Not Issued By Auto CMD12 Error, this bit shall remain 1 and the Com- mand Complete is not set. Status issuing Auto CMD12 is not read from this bit. Auto CMD12 and Auto CMD23 consist of two responses. In this case, this bit is not cleared by the response of CMD12 or CMD23 but cleared by the response of a read/write command. Status issuing Auto CMD12 is not read from this bit. So if a command is issued during Auto CMD12 opera- tion, Host Controller shall manage to issue two commands: CMD12 and a command set by Com-mand register. UHS-II Mode This bit is 0 means that a command packet can be issued by the Host Controller. While this bit is set to 1, which means the Host Controller is not ready to issue a next command, Host Driver shall not write the registers from UHS-II BlockSize \\[Offset 080h\\]
to the UHS-II Command \\[Offset 09Eh\\]. Changing from 1 to 0 generates a Command Complete Interrupt in the Normal Interrupt Status-register. 1- Host Controller is not ready to issue a com-mand 0 - Host Controller is ready to issue a command Version 4.10 adds a new control to prevent error statuses from overwriting by receipt of a next com-mand. This status keeps indicating 1 while any of response error statuses is set to 1 \\[as described in Section 1.17\\], Command Not Issued by Error in this register is set to 1 or Command Not Issued by Auto CMD12 Error in the Auto CMD Error Status register is set to 1. Software Reset For CMD Lineis used to clear the error statuses above and this status."]
pub type InhibitCmdR = crate::BitReader;
#[doc = "Field `INHIBIT_CMD` writer - 0:0\\]
SD Mode If this bit is 0, it indicates the CMD line is not in use and the HC can issue a SD command using the CMD line. This bit is set immediately after the Command register \\[00Fh\\]
is written. This bit is cleared when the command response is received. Even if the Command Inhibit \\[DAT\\]
is set to 1, Commands using only the CMD line can be issued if this bit is 0. Changing from 1 to 0 gener- ates a Command complete interrupt in the Normal Interrupt Status register. If the HC cannot issuethe command because of a command conflict error or because of Command Not Issued By Auto CMD12 Error, this bit shall remain 1 and the Com- mand Complete is not set. Status issuing Auto CMD12 is not read from this bit. Auto CMD12 and Auto CMD23 consist of two responses. In this case, this bit is not cleared by the response of CMD12 or CMD23 but cleared by the response of a read/write command. Status issuing Auto CMD12 is not read from this bit. So if a command is issued during Auto CMD12 opera- tion, Host Controller shall manage to issue two commands: CMD12 and a command set by Com-mand register. UHS-II Mode This bit is 0 means that a command packet can be issued by the Host Controller. While this bit is set to 1, which means the Host Controller is not ready to issue a next command, Host Driver shall not write the registers from UHS-II BlockSize \\[Offset 080h\\]
to the UHS-II Command \\[Offset 09Eh\\]. Changing from 1 to 0 generates a Command Complete Interrupt in the Normal Interrupt Status-register. 1- Host Controller is not ready to issue a com-mand 0 - Host Controller is ready to issue a command Version 4.10 adds a new control to prevent error statuses from overwriting by receipt of a next com-mand. This status keeps indicating 1 while any of response error statuses is set to 1 \\[as described in Section 1.17\\], Command Not Issued by Error in this register is set to 1 or Command Not Issued by Auto CMD12 Error in the Auto CMD Error Status register is set to 1. Software Reset For CMD Lineis used to clear the error statuses above and this status."]
pub type InhibitCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INHIBIT_DAT` reader - 1:1\\]
This status bit is generated if either the DAT Line Active or the Read transfer Active is set to 1. If this bit is 0, it indicates the HC can issue the next SD command. Commands with busy signal belong to Command Inhibit \\[DAT\\]
\\[ex. R1b, R5b type\\]. Changing from 1 to 0 generates a Transfer Complete interrupt in the Normal interrupt status register. Note: The SD Host Driver can save registers in the range of 000-00Dh for a suspend transaction after this bit has changed from 1 to 0. '0' Can issue command which uses the DAT line '1' Cannot issue command which uses the DATline"]
pub type InhibitDatR = crate::BitReader;
#[doc = "Field `INHIBIT_DAT` writer - 1:1\\]
This status bit is generated if either the DAT Line Active or the Read transfer Active is set to 1. If this bit is 0, it indicates the HC can issue the next SD command. Commands with busy signal belong to Command Inhibit \\[DAT\\]
\\[ex. R1b, R5b type\\]. Changing from 1 to 0 generates a Transfer Complete interrupt in the Normal interrupt status register. Note: The SD Host Driver can save registers in the range of 000-00Dh for a suspend transaction after this bit has changed from 1 to 0. '0' Can issue command which uses the DAT line '1' Cannot issue command which uses the DATline"]
pub type InhibitDatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_LINE_ACTIVE` reader - 2:2\\]
This bit indicates whether one of the DAT line on SD bus is in use."]
pub type DataLineActiveR = crate::BitReader;
#[doc = "Field `DATA_LINE_ACTIVE` writer - 2:2\\]
This bit indicates whether one of the DAT line on SD bus is in use."]
pub type DataLineActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETUNING_REQ` reader - 3:3\\]
Host Controller may request Host Driver to execute re-tuning sequence by setting this bit when the data window is shifted by temperature drift and a tuned sampling point does not have a good margin to receive correct data. This bit is cleared when a command is issued with setting Execute Tuning in the Host Control 2 register. Changing of this bit from 0 to 1 generates Re-Tuning Event. Refer to Normal Interrupt registers for more detail. This bit isn't set to 1 if Sampling Clock Select in the Host Control 2 register is set to 0 \\[using fixed sampling clock\\]."]
pub type RetuningReqR = crate::BitReader;
#[doc = "Field `RETUNING_REQ` writer - 3:3\\]
Host Controller may request Host Driver to execute re-tuning sequence by setting this bit when the data window is shifted by temperature drift and a tuned sampling point does not have a good margin to receive correct data. This bit is cleared when a command is issued with setting Execute Tuning in the Host Control 2 register. Changing of this bit from 0 to 1 generates Re-Tuning Event. Refer to Normal Interrupt registers for more detail. This bit isn't set to 1 if Sampling Clock Select in the Host Control 2 register is set to 0 \\[using fixed sampling clock\\]."]
pub type RetuningReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIF_DAT4IN` reader - 4:4\\]
This status is used to check DAT line level to recover from errors, and for debugging."]
pub type SdifDat4inR = crate::BitReader;
#[doc = "Field `SDIF_DAT4IN` writer - 4:4\\]
This status is used to check DAT line level to recover from errors, and for debugging."]
pub type SdifDat4inW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIF_DAT5IN` reader - 5:5\\]
This status is used to check DAT line level to recover from errors, and for debugging."]
pub type SdifDat5inR = crate::BitReader;
#[doc = "Field `SDIF_DAT5IN` writer - 5:5\\]
This status is used to check DAT line level to recover from errors, and for debugging."]
pub type SdifDat5inW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIF_DAT6IN` reader - 6:6\\]
This status is used to check DAT line level to recover from errors, and for debugging."]
pub type SdifDat6inR = crate::BitReader;
#[doc = "Field `SDIF_DAT6IN` writer - 6:6\\]
This status is used to check DAT line level to recover from errors, and for debugging."]
pub type SdifDat6inW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIF_DAT7IN` reader - 7:7\\]
This status is used to check DAT line level to recover from errors, and for debugging."]
pub type SdifDat7inR = crate::BitReader;
#[doc = "Field `SDIF_DAT7IN` writer - 7:7\\]
This status is used to check DAT line level to recover from errors, and for debugging."]
pub type SdifDat7inW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_XFER_ACTIVE` reader - 8:8\\]
This status indicates a write transfer is active. If this bit is 0, it means no valid write data exists in the HC. This bit is set in either of the following cases: After the end bit of the write command. When writing a 1 to Continue Request in the Block Gap Control register to restart a write transfer. This bit is cleared in either of the following cases: After getting the CRC status of the last data block as specified by the transfer count \\[Single or Multiple\\]
After getting a CRC status of any block where data transmission is about to be stopped by a Stop At Block Gap Request. During a write transaction, a Block Gap Event interrupt is generated when this bit is changed to 0, as a result of the Stop At Block Gap Request being set. This status is useful for the HD in determining when to issue commands during write busy."]
pub type WrXferActiveR = crate::BitReader;
#[doc = "Field `WR_XFER_ACTIVE` writer - 8:8\\]
This status indicates a write transfer is active. If this bit is 0, it means no valid write data exists in the HC. This bit is set in either of the following cases: After the end bit of the write command. When writing a 1 to Continue Request in the Block Gap Control register to restart a write transfer. This bit is cleared in either of the following cases: After getting the CRC status of the last data block as specified by the transfer count \\[Single or Multiple\\]
After getting a CRC status of any block where data transmission is about to be stopped by a Stop At Block Gap Request. During a write transaction, a Block Gap Event interrupt is generated when this bit is changed to 0, as a result of the Stop At Block Gap Request being set. This status is useful for the HD in determining when to issue commands during write busy."]
pub type WrXferActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_XFER_ACTIVE` reader - 9:9\\]
This status is used for detecting completion of a read transfer. This bit is set to 1 for either of the following conditions: After the end bit of the read command. When writing a 1 to continue Request in the Block Gap Control register to restart a read transfer. This bit is cleared to 0 for either of the following conditions: When the last data block as specified by block length is transferred to the system. When all valid data blocks have been transferred to the system and no current block transfers are being sent as a result of the Stop At Block Gap Request set to 1. A transfer complete interrupt is generated when this bit changes to 0. '0' No valid data '1' Transferring data"]
pub type RdXferActiveR = crate::BitReader;
#[doc = "Field `RD_XFER_ACTIVE` writer - 9:9\\]
This status is used for detecting completion of a read transfer. This bit is set to 1 for either of the following conditions: After the end bit of the read command. When writing a 1 to continue Request in the Block Gap Control register to restart a read transfer. This bit is cleared to 0 for either of the following conditions: When the last data block as specified by block length is transferred to the system. When all valid data blocks have been transferred to the system and no current block transfers are being sent as a result of the Stop At Block Gap Request set to 1. A transfer complete interrupt is generated when this bit changes to 0. '0' No valid data '1' Transferring data"]
pub type RdXferActiveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_WR_ENA` reader - 10:10\\]
This status is used for non-DMA write transfers.This read only flag indicates if space is available for write data. If this bit is 1, data can be written to the buffer. A change of this bit from 1 to 0 occurs when all the block data is written to the buffer. A change of this bit from 0 to 1 occurs when top of block data can be written to the buffer and generates the Buffer Write Ready Interrupt."]
pub type BufWrEnaR = crate::BitReader;
#[doc = "Field `BUF_WR_ENA` writer - 10:10\\]
This status is used for non-DMA write transfers.This read only flag indicates if space is available for write data. If this bit is 1, data can be written to the buffer. A change of this bit from 1 to 0 occurs when all the block data is written to the buffer. A change of this bit from 0 to 1 occurs when top of block data can be written to the buffer and generates the Buffer Write Ready Interrupt."]
pub type BufWrEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_RD_ENA` reader - 11:11\\]
This status is used for non-DMA read transfers.This read only flag indicates that valid data exists in the host side buffer status. If this bit is 1, readable data exists in the buffer. A change of this bit from 1 to 0 occurs when all the block data is read from the buffer. A change of this bit from 0 to 1 occurs when all the block data is ready in the buffer and generates the Buffer Read Ready Interrupt."]
pub type BufRdEnaR = crate::BitReader;
#[doc = "Field `BUF_RD_ENA` writer - 11:11\\]
This status is used for non-DMA read transfers.This read only flag indicates that valid data exists in the host side buffer status. If this bit is 1, readable data exists in the buffer. A change of this bit from 1 to 0 occurs when all the block data is read from the buffer. A change of this bit from 0 to 1 occurs when all the block data is ready in the buffer and generates the Buffer Read Ready Interrupt."]
pub type BufRdEnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INSERTED` reader - 16:16\\]
This bit indicates whether a card has been inserted. Changing from 0 to 1 generates a Card Insertion interrupt in the Normal Interrupt Status register and changing from 1 to 0 generates a Card Removal Interrupt in the Normal Interrupt Status register. The Software Reset For All in the Software Reset register shall not affect this bit. If a Card is removed while its power is on and its clock is oscillating, the HC shall clear SD Bus Power in the Power Control register and SD Clock Enable in the Clock control register. In addition the HD should clear the HC by the Software Reset For All in Software register. The card detect is active regardless of the SD Bus Power. '0' Reset or Debouncing or No Card '1' Card Inserted"]
pub type CardInsertedR = crate::BitReader;
#[doc = "Field `CARD_INSERTED` writer - 16:16\\]
This bit indicates whether a card has been inserted. Changing from 0 to 1 generates a Card Insertion interrupt in the Normal Interrupt Status register and changing from 1 to 0 generates a Card Removal Interrupt in the Normal Interrupt Status register. The Software Reset For All in the Software Reset register shall not affect this bit. If a Card is removed while its power is on and its clock is oscillating, the HC shall clear SD Bus Power in the Power Control register and SD Clock Enable in the Clock control register. In addition the HD should clear the HC by the Software Reset For All in Software register. The card detect is active regardless of the SD Bus Power. '0' Reset or Debouncing or No Card '1' Card Inserted"]
pub type CardInsertedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_STATE_STABLE` reader - 17:17\\]
This bit is used for testing. If it is 0, the Card Detect Pin Level is not stable. If this bit is set to 1,it means the Card Detect Pin Level is stable. The Software Reset For All in the Software Reset Register shall not affect this bit. '0' Reset of Debouncing '1' No Card or Inserted"]
pub type CardStateStableR = crate::BitReader;
#[doc = "Field `CARD_STATE_STABLE` writer - 17:17\\]
This bit is used for testing. If it is 0, the Card Detect Pin Level is not stable. If this bit is set to 1,it means the Card Detect Pin Level is stable. The Software Reset For All in the Software Reset Register shall not affect this bit. '0' Reset of Debouncing '1' No Card or Inserted"]
pub type CardStateStableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_DETECT` reader - 18:18\\]
This bit reflects the inverse value of the SDCD# pin. '0' No Card present \\[SDCD# = 1\\]
'1' Card present \\[SDCD# = 0\\]"]
pub type CardDetectR = crate::BitReader;
#[doc = "Field `CARD_DETECT` writer - 18:18\\]
This bit reflects the inverse value of the SDCD# pin. '0' No Card present \\[SDCD# = 1\\]
'1' Card present \\[SDCD# = 0\\]"]
pub type CardDetectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_PROTECT` reader - 19:19\\]
The Write Protect Switch is supported for memory and combo cards.This bit reflects the SDWP# pin."]
pub type WriteProtectR = crate::BitReader;
#[doc = "Field `WRITE_PROTECT` writer - 19:19\\]
The Write Protect Switch is supported for memory and combo cards.This bit reflects the SDWP# pin."]
pub type WriteProtectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIF_DAT0IN` reader - 20:20\\]
This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[0\\]."]
pub type SdifDat0inR = crate::BitReader;
#[doc = "Field `SDIF_DAT0IN` writer - 20:20\\]
This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[0\\]."]
pub type SdifDat0inW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIF_DAT1IN` reader - 21:21\\]
This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[1\\]."]
pub type SdifDat1inR = crate::BitReader;
#[doc = "Field `SDIF_DAT1IN` writer - 21:21\\]
This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[1\\]."]
pub type SdifDat1inW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIF_DAT2IN` reader - 22:22\\]
This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[2\\]."]
pub type SdifDat2inR = crate::BitReader;
#[doc = "Field `SDIF_DAT2IN` writer - 22:22\\]
This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[2\\]."]
pub type SdifDat2inW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIF_DAT3IN` reader - 23:23\\]
This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[3\\]."]
pub type SdifDat3inR = crate::BitReader;
#[doc = "Field `SDIF_DAT3IN` writer - 23:23\\]
This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[3\\]."]
pub type SdifDat3inW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIF_CMDIN` reader - 24:24\\]
This status is used to check DAT line level to recover from errors, and for debugging."]
pub type SdifCmdinR = crate::BitReader;
#[doc = "Field `SDIF_CMDIN` writer - 24:24\\]
This status is used to check DAT line level to recover from errors, and for debugging."]
pub type SdifCmdinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_NOT_ISS_BY_ERR` reader - 27:27\\]
Setting of this status indicates that a command cannot be issued due to an error except Auto CMD12 error. \\[Equivalent error status by Auto CMD12 error is defined as Command Not Issued By Auto CMD12 Error in the Auto CMD Error Status register.\\]
This status is set to 1 when Host Controller cannot issue a command after setting Command register or UHS-II Command register.Refer to Section 3.10 about 2L-HD error case inUHS-II mode.Sub Command Status \\[D28\\]
indicates which command is not issued \\[main or sub\\]. 1 - Command cannot be issued 0 - No error for issuing a command"]
pub type CmdNotIssByErrR = crate::BitReader;
#[doc = "Field `CMD_NOT_ISS_BY_ERR` writer - 27:27\\]
Setting of this status indicates that a command cannot be issued due to an error except Auto CMD12 error. \\[Equivalent error status by Auto CMD12 error is defined as Command Not Issued By Auto CMD12 Error in the Auto CMD Error Status register.\\]
This status is set to 1 when Host Controller cannot issue a command after setting Command register or UHS-II Command register.Refer to Section 3.10 about 2L-HD error case inUHS-II mode.Sub Command Status \\[D28\\]
indicates which command is not issued \\[main or sub\\]. 1 - Command cannot be issued 0 - No error for issuing a command"]
pub type CmdNotIssByErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUB_COMMAND_STS` reader - 28:28\\]
The Command register and Response register are commonly used for main command and sub command. This status is used to distinguish which response error statuses, main command or sub command, indicated in the Error Interrupt Status register or in the UHS-II Error Interrupt Status register. Refer to Section 1.17 about details of response error statuses. Just before reading of this register, the Sub Command Flag of the Command register or the UHS-II Command register is copied to this status. This status is effective not only when Response Error interrupt is generated but also when data error interrupt is generated with Command Not Issued by Error \\[D27 of this register\\]
or Auto CMD Error interrupt is generated with Command Not Issued by Error by Auto CMD12 in the Auto CMD Error Status register. 1 - Sub Command Status 0 - Main Command Status"]
pub type SubCommandStsR = crate::BitReader;
#[doc = "Field `SUB_COMMAND_STS` writer - 28:28\\]
The Command register and Response register are commonly used for main command and sub command. This status is used to distinguish which response error statuses, main command or sub command, indicated in the Error Interrupt Status register or in the UHS-II Error Interrupt Status register. Refer to Section 1.17 about details of response error statuses. Just before reading of this register, the Sub Command Flag of the Command register or the UHS-II Command register is copied to this status. This status is effective not only when Response Error interrupt is generated but also when data error interrupt is generated with Command Not Issued by Error \\[D27 of this register\\]
or Auto CMD Error interrupt is generated with Command Not Issued by Error by Auto CMD12 in the Auto CMD Error Status register. 1 - Sub Command Status 0 - Main Command Status"]
pub type SubCommandStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UHS2_DORMANT` reader - 29:29\\]
This status indicates whether UHS-II Ianes enterDormant state. This function is enabled by setting UHS-II Interface Enable to 1 in the Host Control 2 register. On issuing GO_DORMAT_STATE com-mand, Go Dormant Command \\[111b\\]; is set to Command type in the UHS-II Command register.This command type acts as a trigger to enterlanes into dormant state. Host Controller provides STB.H and EIDL on D0 lane and waits for receiv-ing STB.H and EIDL on D1 lane. This bit is set to 1 after the time of T_DMT_ENTRY \\[750 RCLK cycle\\]
or more from detecting EIDL on D1 lane. RCLK may be stopped in dormant state, by setting SD Clock Enable to 0 in the Clock Control register while In Dormant State bit is set to 1. On writingClock Control register with setting SD Clock Enable to 1, Host Controller wakes lanes to exit Dormant State and In Dormant State is set to 0. In case of the card enters Hibernate Mode \\[RCLK is stopped\\], Host Driver may turn off VDD1 by clearing SD Bus Power for VDD1 bit in the Power Control register. Host Controller shall turn off VDD1 after stopping RCLK. This bit is cleared by when Host Controller drives STB.L on D0 lane, UHS-II Interface Enable is set to 0 or executesHost full reset."]
pub type Uhs2DormantR = crate::BitReader;
#[doc = "Field `UHS2_DORMANT` writer - 29:29\\]
This status indicates whether UHS-II Ianes enterDormant state. This function is enabled by setting UHS-II Interface Enable to 1 in the Host Control 2 register. On issuing GO_DORMAT_STATE com-mand, Go Dormant Command \\[111b\\]; is set to Command type in the UHS-II Command register.This command type acts as a trigger to enterlanes into dormant state. Host Controller provides STB.H and EIDL on D0 lane and waits for receiv-ing STB.H and EIDL on D1 lane. This bit is set to 1 after the time of T_DMT_ENTRY \\[750 RCLK cycle\\]
or more from detecting EIDL on D1 lane. RCLK may be stopped in dormant state, by setting SD Clock Enable to 0 in the Clock Control register while In Dormant State bit is set to 1. On writingClock Control register with setting SD Clock Enable to 1, Host Controller wakes lanes to exit Dormant State and In Dormant State is set to 0. In case of the card enters Hibernate Mode \\[RCLK is stopped\\], Host Driver may turn off VDD1 by clearing SD Bus Power for VDD1 bit in the Power Control register. Host Controller shall turn off VDD1 after stopping RCLK. This bit is cleared by when Host Controller drives STB.L on D0 lane, UHS-II Interface Enable is set to 0 or executesHost full reset."]
pub type Uhs2DormantW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UHS2_IF_LANE_SYNC` reader - 30:30\\]
This status indicates whether lane is synchronized in UHS-II mode. This status is enabled by setting UHS-II Interface Enable to 1 in the Host Control 2 register. On detecting UHS-II Interface \\[D31=1\\], Host Controller provides SYN LSS on D0 lane and waits for receiving SYN LSS on D1 lane. If SYN LSS is detected on D1 lane, Host Controller pro-vides LIDL LSS on D0 lane and waits for receiving LIDL LSS on D1 lane. In case of Version 4.00, this bit indicates comple-tion of Device PHY Initialization by detecting LIDL LSS on D1 lane.From Version 4.10, Host Controller may imple-ment a specific PHY verification method and PHY Initialization Failure can be indicated by keeping this bit to 0 even LIDL LSS is detected on D1 lane.Host Driver detects PHY Initialization Failure by timeout.This bit is cleared by when D0 lane is set to EIDL,UHS-II Interface Enable is set to 0 or executes Host full reset."]
pub type Uhs2IfLaneSyncR = crate::BitReader;
#[doc = "Field `UHS2_IF_LANE_SYNC` writer - 30:30\\]
This status indicates whether lane is synchronized in UHS-II mode. This status is enabled by setting UHS-II Interface Enable to 1 in the Host Control 2 register. On detecting UHS-II Interface \\[D31=1\\], Host Controller provides SYN LSS on D0 lane and waits for receiving SYN LSS on D1 lane. If SYN LSS is detected on D1 lane, Host Controller pro-vides LIDL LSS on D0 lane and waits for receiving LIDL LSS on D1 lane. In case of Version 4.00, this bit indicates comple-tion of Device PHY Initialization by detecting LIDL LSS on D1 lane.From Version 4.10, Host Controller may imple-ment a specific PHY verification method and PHY Initialization Failure can be indicated by keeping this bit to 0 even LIDL LSS is detected on D1 lane.Host Driver detects PHY Initialization Failure by timeout.This bit is cleared by when D0 lane is set to EIDL,UHS-II Interface Enable is set to 0 or executes Host full reset."]
pub type Uhs2IfLaneSyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UHS2_IF_DETECTION` reader - 31:31\\]
This status indicates whether a card supports UHS-II IF. This status is enabled by setting UHS-II Interface Enable to 1 in the Host Control 2 regis-ter. UHS-II interface initialization is activated by setting SD Clock Enable in the Clock Control reg-ister. Host Controller drives STB.L on D0 lanefrom EIDL state and waits for receiving STB.L on D1 lane. This bit is set to 1 if STB.L is detected on D1 lane. Host Controller shall compensate latency from setting SD Clock Enable to output STB.L on D0 lane when reading this status \\[Refer to Figure 3-35 about details of this method\\]. This bit may be read any time after setting SD Clock Enable for faster UHS-II IF detection but Host Driver shall check this status at least 200us period from set- ting SD Clock Enable until detecting UHS-II IF. After UHS-II IF is detected, this bit is cleared by when EIDL is detected on D0 lane, UHS-II Inter-face Enable is set to 0 or Host full reset is exe-cuted. '0' UHS-II IF is not detected '1' UHS-II IF is detected"]
pub type Uhs2IfDetectionR = crate::BitReader;
#[doc = "Field `UHS2_IF_DETECTION` writer - 31:31\\]
This status indicates whether a card supports UHS-II IF. This status is enabled by setting UHS-II Interface Enable to 1 in the Host Control 2 regis-ter. UHS-II interface initialization is activated by setting SD Clock Enable in the Clock Control reg-ister. Host Controller drives STB.L on D0 lanefrom EIDL state and waits for receiving STB.L on D1 lane. This bit is set to 1 if STB.L is detected on D1 lane. Host Controller shall compensate latency from setting SD Clock Enable to output STB.L on D0 lane when reading this status \\[Refer to Figure 3-35 about details of this method\\]. This bit may be read any time after setting SD Clock Enable for faster UHS-II IF detection but Host Driver shall check this status at least 200us period from set- ting SD Clock Enable until detecting UHS-II IF. After UHS-II IF is detected, this bit is cleared by when EIDL is detected on D0 lane, UHS-II Inter-face Enable is set to 0 or Host full reset is exe-cuted. '0' UHS-II IF is not detected '1' UHS-II IF is detected"]
pub type Uhs2IfDetectionW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SD Mode If this bit is 0, it indicates the CMD line is not in use and the HC can issue a SD command using the CMD line. This bit is set immediately after the Command register \\[00Fh\\]
is written. This bit is cleared when the command response is received. Even if the Command Inhibit \\[DAT\\]
is set to 1, Commands using only the CMD line can be issued if this bit is 0. Changing from 1 to 0 gener- ates a Command complete interrupt in the Normal Interrupt Status register. If the HC cannot issuethe command because of a command conflict error or because of Command Not Issued By Auto CMD12 Error, this bit shall remain 1 and the Com- mand Complete is not set. Status issuing Auto CMD12 is not read from this bit. Auto CMD12 and Auto CMD23 consist of two responses. In this case, this bit is not cleared by the response of CMD12 or CMD23 but cleared by the response of a read/write command. Status issuing Auto CMD12 is not read from this bit. So if a command is issued during Auto CMD12 opera- tion, Host Controller shall manage to issue two commands: CMD12 and a command set by Com-mand register. UHS-II Mode This bit is 0 means that a command packet can be issued by the Host Controller. While this bit is set to 1, which means the Host Controller is not ready to issue a next command, Host Driver shall not write the registers from UHS-II BlockSize \\[Offset 080h\\]
to the UHS-II Command \\[Offset 09Eh\\]. Changing from 1 to 0 generates a Command Complete Interrupt in the Normal Interrupt Status-register. 1- Host Controller is not ready to issue a com-mand 0 - Host Controller is ready to issue a command Version 4.10 adds a new control to prevent error statuses from overwriting by receipt of a next com-mand. This status keeps indicating 1 while any of response error statuses is set to 1 \\[as described in Section 1.17\\], Command Not Issued by Error in this register is set to 1 or Command Not Issued by Auto CMD12 Error in the Auto CMD Error Status register is set to 1. Software Reset For CMD Lineis used to clear the error statuses above and this status."]
    #[inline(always)]
    pub fn inhibit_cmd(&self) -> InhibitCmdR {
        InhibitCmdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This status bit is generated if either the DAT Line Active or the Read transfer Active is set to 1. If this bit is 0, it indicates the HC can issue the next SD command. Commands with busy signal belong to Command Inhibit \\[DAT\\]
\\[ex. R1b, R5b type\\]. Changing from 1 to 0 generates a Transfer Complete interrupt in the Normal interrupt status register. Note: The SD Host Driver can save registers in the range of 000-00Dh for a suspend transaction after this bit has changed from 1 to 0. '0' Can issue command which uses the DAT line '1' Cannot issue command which uses the DATline"]
    #[inline(always)]
    pub fn inhibit_dat(&self) -> InhibitDatR {
        InhibitDatR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit indicates whether one of the DAT line on SD bus is in use."]
    #[inline(always)]
    pub fn data_line_active(&self) -> DataLineActiveR {
        DataLineActiveR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Host Controller may request Host Driver to execute re-tuning sequence by setting this bit when the data window is shifted by temperature drift and a tuned sampling point does not have a good margin to receive correct data. This bit is cleared when a command is issued with setting Execute Tuning in the Host Control 2 register. Changing of this bit from 0 to 1 generates Re-Tuning Event. Refer to Normal Interrupt registers for more detail. This bit isn't set to 1 if Sampling Clock Select in the Host Control 2 register is set to 0 \\[using fixed sampling clock\\]."]
    #[inline(always)]
    pub fn retuning_req(&self) -> RetuningReqR {
        RetuningReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
This status is used to check DAT line level to recover from errors, and for debugging."]
    #[inline(always)]
    pub fn sdif_dat4in(&self) -> SdifDat4inR {
        SdifDat4inR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
This status is used to check DAT line level to recover from errors, and for debugging."]
    #[inline(always)]
    pub fn sdif_dat5in(&self) -> SdifDat5inR {
        SdifDat5inR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
This status is used to check DAT line level to recover from errors, and for debugging."]
    #[inline(always)]
    pub fn sdif_dat6in(&self) -> SdifDat6inR {
        SdifDat6inR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
This status is used to check DAT line level to recover from errors, and for debugging."]
    #[inline(always)]
    pub fn sdif_dat7in(&self) -> SdifDat7inR {
        SdifDat7inR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
This status indicates a write transfer is active. If this bit is 0, it means no valid write data exists in the HC. This bit is set in either of the following cases: After the end bit of the write command. When writing a 1 to Continue Request in the Block Gap Control register to restart a write transfer. This bit is cleared in either of the following cases: After getting the CRC status of the last data block as specified by the transfer count \\[Single or Multiple\\]
After getting a CRC status of any block where data transmission is about to be stopped by a Stop At Block Gap Request. During a write transaction, a Block Gap Event interrupt is generated when this bit is changed to 0, as a result of the Stop At Block Gap Request being set. This status is useful for the HD in determining when to issue commands during write busy."]
    #[inline(always)]
    pub fn wr_xfer_active(&self) -> WrXferActiveR {
        WrXferActiveR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
This status is used for detecting completion of a read transfer. This bit is set to 1 for either of the following conditions: After the end bit of the read command. When writing a 1 to continue Request in the Block Gap Control register to restart a read transfer. This bit is cleared to 0 for either of the following conditions: When the last data block as specified by block length is transferred to the system. When all valid data blocks have been transferred to the system and no current block transfers are being sent as a result of the Stop At Block Gap Request set to 1. A transfer complete interrupt is generated when this bit changes to 0. '0' No valid data '1' Transferring data"]
    #[inline(always)]
    pub fn rd_xfer_active(&self) -> RdXferActiveR {
        RdXferActiveR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
This status is used for non-DMA write transfers.This read only flag indicates if space is available for write data. If this bit is 1, data can be written to the buffer. A change of this bit from 1 to 0 occurs when all the block data is written to the buffer. A change of this bit from 0 to 1 occurs when top of block data can be written to the buffer and generates the Buffer Write Ready Interrupt."]
    #[inline(always)]
    pub fn buf_wr_ena(&self) -> BufWrEnaR {
        BufWrEnaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
This status is used for non-DMA read transfers.This read only flag indicates that valid data exists in the host side buffer status. If this bit is 1, readable data exists in the buffer. A change of this bit from 1 to 0 occurs when all the block data is read from the buffer. A change of this bit from 0 to 1 occurs when all the block data is ready in the buffer and generates the Buffer Read Ready Interrupt."]
    #[inline(always)]
    pub fn buf_rd_ena(&self) -> BufRdEnaR {
        BufRdEnaR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
This bit indicates whether a card has been inserted. Changing from 0 to 1 generates a Card Insertion interrupt in the Normal Interrupt Status register and changing from 1 to 0 generates a Card Removal Interrupt in the Normal Interrupt Status register. The Software Reset For All in the Software Reset register shall not affect this bit. If a Card is removed while its power is on and its clock is oscillating, the HC shall clear SD Bus Power in the Power Control register and SD Clock Enable in the Clock control register. In addition the HD should clear the HC by the Software Reset For All in Software register. The card detect is active regardless of the SD Bus Power. '0' Reset or Debouncing or No Card '1' Card Inserted"]
    #[inline(always)]
    pub fn card_inserted(&self) -> CardInsertedR {
        CardInsertedR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
This bit is used for testing. If it is 0, the Card Detect Pin Level is not stable. If this bit is set to 1,it means the Card Detect Pin Level is stable. The Software Reset For All in the Software Reset Register shall not affect this bit. '0' Reset of Debouncing '1' No Card or Inserted"]
    #[inline(always)]
    pub fn card_state_stable(&self) -> CardStateStableR {
        CardStateStableR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
This bit reflects the inverse value of the SDCD# pin. '0' No Card present \\[SDCD# = 1\\]
'1' Card present \\[SDCD# = 0\\]"]
    #[inline(always)]
    pub fn card_detect(&self) -> CardDetectR {
        CardDetectR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
The Write Protect Switch is supported for memory and combo cards.This bit reflects the SDWP# pin."]
    #[inline(always)]
    pub fn write_protect(&self) -> WriteProtectR {
        WriteProtectR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[0\\]."]
    #[inline(always)]
    pub fn sdif_dat0in(&self) -> SdifDat0inR {
        SdifDat0inR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[1\\]."]
    #[inline(always)]
    pub fn sdif_dat1in(&self) -> SdifDat1inR {
        SdifDat1inR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[2\\]."]
    #[inline(always)]
    pub fn sdif_dat2in(&self) -> SdifDat2inR {
        SdifDat2inR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[3\\]."]
    #[inline(always)]
    pub fn sdif_dat3in(&self) -> SdifDat3inR {
        SdifDat3inR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
This status is used to check DAT line level to recover from errors, and for debugging."]
    #[inline(always)]
    pub fn sdif_cmdin(&self) -> SdifCmdinR {
        SdifCmdinR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
Setting of this status indicates that a command cannot be issued due to an error except Auto CMD12 error. \\[Equivalent error status by Auto CMD12 error is defined as Command Not Issued By Auto CMD12 Error in the Auto CMD Error Status register.\\]
This status is set to 1 when Host Controller cannot issue a command after setting Command register or UHS-II Command register.Refer to Section 3.10 about 2L-HD error case inUHS-II mode.Sub Command Status \\[D28\\]
indicates which command is not issued \\[main or sub\\]. 1 - Command cannot be issued 0 - No error for issuing a command"]
    #[inline(always)]
    pub fn cmd_not_iss_by_err(&self) -> CmdNotIssByErrR {
        CmdNotIssByErrR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 28:28\\]
The Command register and Response register are commonly used for main command and sub command. This status is used to distinguish which response error statuses, main command or sub command, indicated in the Error Interrupt Status register or in the UHS-II Error Interrupt Status register. Refer to Section 1.17 about details of response error statuses. Just before reading of this register, the Sub Command Flag of the Command register or the UHS-II Command register is copied to this status. This status is effective not only when Response Error interrupt is generated but also when data error interrupt is generated with Command Not Issued by Error \\[D27 of this register\\]
or Auto CMD Error interrupt is generated with Command Not Issued by Error by Auto CMD12 in the Auto CMD Error Status register. 1 - Sub Command Status 0 - Main Command Status"]
    #[inline(always)]
    pub fn sub_command_sts(&self) -> SubCommandStsR {
        SubCommandStsR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
This status indicates whether UHS-II Ianes enterDormant state. This function is enabled by setting UHS-II Interface Enable to 1 in the Host Control 2 register. On issuing GO_DORMAT_STATE com-mand, Go Dormant Command \\[111b\\]; is set to Command type in the UHS-II Command register.This command type acts as a trigger to enterlanes into dormant state. Host Controller provides STB.H and EIDL on D0 lane and waits for receiv-ing STB.H and EIDL on D1 lane. This bit is set to 1 after the time of T_DMT_ENTRY \\[750 RCLK cycle\\]
or more from detecting EIDL on D1 lane. RCLK may be stopped in dormant state, by setting SD Clock Enable to 0 in the Clock Control register while In Dormant State bit is set to 1. On writingClock Control register with setting SD Clock Enable to 1, Host Controller wakes lanes to exit Dormant State and In Dormant State is set to 0. In case of the card enters Hibernate Mode \\[RCLK is stopped\\], Host Driver may turn off VDD1 by clearing SD Bus Power for VDD1 bit in the Power Control register. Host Controller shall turn off VDD1 after stopping RCLK. This bit is cleared by when Host Controller drives STB.L on D0 lane, UHS-II Interface Enable is set to 0 or executesHost full reset."]
    #[inline(always)]
    pub fn uhs2_dormant(&self) -> Uhs2DormantR {
        Uhs2DormantR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
This status indicates whether lane is synchronized in UHS-II mode. This status is enabled by setting UHS-II Interface Enable to 1 in the Host Control 2 register. On detecting UHS-II Interface \\[D31=1\\], Host Controller provides SYN LSS on D0 lane and waits for receiving SYN LSS on D1 lane. If SYN LSS is detected on D1 lane, Host Controller pro-vides LIDL LSS on D0 lane and waits for receiving LIDL LSS on D1 lane. In case of Version 4.00, this bit indicates comple-tion of Device PHY Initialization by detecting LIDL LSS on D1 lane.From Version 4.10, Host Controller may imple-ment a specific PHY verification method and PHY Initialization Failure can be indicated by keeping this bit to 0 even LIDL LSS is detected on D1 lane.Host Driver detects PHY Initialization Failure by timeout.This bit is cleared by when D0 lane is set to EIDL,UHS-II Interface Enable is set to 0 or executes Host full reset."]
    #[inline(always)]
    pub fn uhs2_if_lane_sync(&self) -> Uhs2IfLaneSyncR {
        Uhs2IfLaneSyncR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
This status indicates whether a card supports UHS-II IF. This status is enabled by setting UHS-II Interface Enable to 1 in the Host Control 2 regis-ter. UHS-II interface initialization is activated by setting SD Clock Enable in the Clock Control reg-ister. Host Controller drives STB.L on D0 lanefrom EIDL state and waits for receiving STB.L on D1 lane. This bit is set to 1 if STB.L is detected on D1 lane. Host Controller shall compensate latency from setting SD Clock Enable to output STB.L on D0 lane when reading this status \\[Refer to Figure 3-35 about details of this method\\]. This bit may be read any time after setting SD Clock Enable for faster UHS-II IF detection but Host Driver shall check this status at least 200us period from set- ting SD Clock Enable until detecting UHS-II IF. After UHS-II IF is detected, this bit is cleared by when EIDL is detected on D0 lane, UHS-II Inter-face Enable is set to 0 or Host full reset is exe-cuted. '0' UHS-II IF is not detected '1' UHS-II IF is detected"]
    #[inline(always)]
    pub fn uhs2_if_detection(&self) -> Uhs2IfDetectionR {
        Uhs2IfDetectionR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SD Mode If this bit is 0, it indicates the CMD line is not in use and the HC can issue a SD command using the CMD line. This bit is set immediately after the Command register \\[00Fh\\]
is written. This bit is cleared when the command response is received. Even if the Command Inhibit \\[DAT\\]
is set to 1, Commands using only the CMD line can be issued if this bit is 0. Changing from 1 to 0 gener- ates a Command complete interrupt in the Normal Interrupt Status register. If the HC cannot issuethe command because of a command conflict error or because of Command Not Issued By Auto CMD12 Error, this bit shall remain 1 and the Com- mand Complete is not set. Status issuing Auto CMD12 is not read from this bit. Auto CMD12 and Auto CMD23 consist of two responses. In this case, this bit is not cleared by the response of CMD12 or CMD23 but cleared by the response of a read/write command. Status issuing Auto CMD12 is not read from this bit. So if a command is issued during Auto CMD12 opera- tion, Host Controller shall manage to issue two commands: CMD12 and a command set by Com-mand register. UHS-II Mode This bit is 0 means that a command packet can be issued by the Host Controller. While this bit is set to 1, which means the Host Controller is not ready to issue a next command, Host Driver shall not write the registers from UHS-II BlockSize \\[Offset 080h\\]
to the UHS-II Command \\[Offset 09Eh\\]. Changing from 1 to 0 generates a Command Complete Interrupt in the Normal Interrupt Status-register. 1- Host Controller is not ready to issue a com-mand 0 - Host Controller is ready to issue a command Version 4.10 adds a new control to prevent error statuses from overwriting by receipt of a next com-mand. This status keeps indicating 1 while any of response error statuses is set to 1 \\[as described in Section 1.17\\], Command Not Issued by Error in this register is set to 1 or Command Not Issued by Auto CMD12 Error in the Auto CMD Error Status register is set to 1. Software Reset For CMD Lineis used to clear the error statuses above and this status."]
    #[inline(always)]
    #[must_use]
    pub fn inhibit_cmd(&mut self) -> InhibitCmdW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        InhibitCmdW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This status bit is generated if either the DAT Line Active or the Read transfer Active is set to 1. If this bit is 0, it indicates the HC can issue the next SD command. Commands with busy signal belong to Command Inhibit \\[DAT\\]
\\[ex. R1b, R5b type\\]. Changing from 1 to 0 generates a Transfer Complete interrupt in the Normal interrupt status register. Note: The SD Host Driver can save registers in the range of 000-00Dh for a suspend transaction after this bit has changed from 1 to 0. '0' Can issue command which uses the DAT line '1' Cannot issue command which uses the DATline"]
    #[inline(always)]
    #[must_use]
    pub fn inhibit_dat(&mut self) -> InhibitDatW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        InhibitDatW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
This bit indicates whether one of the DAT line on SD bus is in use."]
    #[inline(always)]
    #[must_use]
    pub fn data_line_active(&mut self) -> DataLineActiveW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        DataLineActiveW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Host Controller may request Host Driver to execute re-tuning sequence by setting this bit when the data window is shifted by temperature drift and a tuned sampling point does not have a good margin to receive correct data. This bit is cleared when a command is issued with setting Execute Tuning in the Host Control 2 register. Changing of this bit from 0 to 1 generates Re-Tuning Event. Refer to Normal Interrupt registers for more detail. This bit isn't set to 1 if Sampling Clock Select in the Host Control 2 register is set to 0 \\[using fixed sampling clock\\]."]
    #[inline(always)]
    #[must_use]
    pub fn retuning_req(&mut self) -> RetuningReqW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        RetuningReqW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
This status is used to check DAT line level to recover from errors, and for debugging."]
    #[inline(always)]
    #[must_use]
    pub fn sdif_dat4in(&mut self) -> SdifDat4inW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        SdifDat4inW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
This status is used to check DAT line level to recover from errors, and for debugging."]
    #[inline(always)]
    #[must_use]
    pub fn sdif_dat5in(&mut self) -> SdifDat5inW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        SdifDat5inW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
This status is used to check DAT line level to recover from errors, and for debugging."]
    #[inline(always)]
    #[must_use]
    pub fn sdif_dat6in(&mut self) -> SdifDat6inW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        SdifDat6inW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
This status is used to check DAT line level to recover from errors, and for debugging."]
    #[inline(always)]
    #[must_use]
    pub fn sdif_dat7in(&mut self) -> SdifDat7inW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        SdifDat7inW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
This status indicates a write transfer is active. If this bit is 0, it means no valid write data exists in the HC. This bit is set in either of the following cases: After the end bit of the write command. When writing a 1 to Continue Request in the Block Gap Control register to restart a write transfer. This bit is cleared in either of the following cases: After getting the CRC status of the last data block as specified by the transfer count \\[Single or Multiple\\]
After getting a CRC status of any block where data transmission is about to be stopped by a Stop At Block Gap Request. During a write transaction, a Block Gap Event interrupt is generated when this bit is changed to 0, as a result of the Stop At Block Gap Request being set. This status is useful for the HD in determining when to issue commands during write busy."]
    #[inline(always)]
    #[must_use]
    pub fn wr_xfer_active(&mut self) -> WrXferActiveW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        WrXferActiveW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
This status is used for detecting completion of a read transfer. This bit is set to 1 for either of the following conditions: After the end bit of the read command. When writing a 1 to continue Request in the Block Gap Control register to restart a read transfer. This bit is cleared to 0 for either of the following conditions: When the last data block as specified by block length is transferred to the system. When all valid data blocks have been transferred to the system and no current block transfers are being sent as a result of the Stop At Block Gap Request set to 1. A transfer complete interrupt is generated when this bit changes to 0. '0' No valid data '1' Transferring data"]
    #[inline(always)]
    #[must_use]
    pub fn rd_xfer_active(&mut self) -> RdXferActiveW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        RdXferActiveW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
This status is used for non-DMA write transfers.This read only flag indicates if space is available for write data. If this bit is 1, data can be written to the buffer. A change of this bit from 1 to 0 occurs when all the block data is written to the buffer. A change of this bit from 0 to 1 occurs when top of block data can be written to the buffer and generates the Buffer Write Ready Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn buf_wr_ena(&mut self) -> BufWrEnaW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        BufWrEnaW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
This status is used for non-DMA read transfers.This read only flag indicates that valid data exists in the host side buffer status. If this bit is 1, readable data exists in the buffer. A change of this bit from 1 to 0 occurs when all the block data is read from the buffer. A change of this bit from 0 to 1 occurs when all the block data is ready in the buffer and generates the Buffer Read Ready Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn buf_rd_ena(&mut self) -> BufRdEnaW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        BufRdEnaW::new(self, 11)
    }
    #[doc = "Bit 16 - 16:16\\]
This bit indicates whether a card has been inserted. Changing from 0 to 1 generates a Card Insertion interrupt in the Normal Interrupt Status register and changing from 1 to 0 generates a Card Removal Interrupt in the Normal Interrupt Status register. The Software Reset For All in the Software Reset register shall not affect this bit. If a Card is removed while its power is on and its clock is oscillating, the HC shall clear SD Bus Power in the Power Control register and SD Clock Enable in the Clock control register. In addition the HD should clear the HC by the Software Reset For All in Software register. The card detect is active regardless of the SD Bus Power. '0' Reset or Debouncing or No Card '1' Card Inserted"]
    #[inline(always)]
    #[must_use]
    pub fn card_inserted(&mut self) -> CardInsertedW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        CardInsertedW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
This bit is used for testing. If it is 0, the Card Detect Pin Level is not stable. If this bit is set to 1,it means the Card Detect Pin Level is stable. The Software Reset For All in the Software Reset Register shall not affect this bit. '0' Reset of Debouncing '1' No Card or Inserted"]
    #[inline(always)]
    #[must_use]
    pub fn card_state_stable(
        &mut self,
    ) -> CardStateStableW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        CardStateStableW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
This bit reflects the inverse value of the SDCD# pin. '0' No Card present \\[SDCD# = 1\\]
'1' Card present \\[SDCD# = 0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn card_detect(&mut self) -> CardDetectW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        CardDetectW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
The Write Protect Switch is supported for memory and combo cards.This bit reflects the SDWP# pin."]
    #[inline(always)]
    #[must_use]
    pub fn write_protect(&mut self) -> WriteProtectW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        WriteProtectW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[0\\]."]
    #[inline(always)]
    #[must_use]
    pub fn sdif_dat0in(&mut self) -> SdifDat0inW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        SdifDat0inW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[1\\]."]
    #[inline(always)]
    #[must_use]
    pub fn sdif_dat1in(&mut self) -> SdifDat1inW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        SdifDat1inW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[2\\]."]
    #[inline(always)]
    #[must_use]
    pub fn sdif_dat2in(&mut self) -> SdifDat2inW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        SdifDat2inW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
This status is used to check DAT line level to recover from errors, and for debugging. This is especially useful in detecting the busy signal level from DAT\\[3\\]."]
    #[inline(always)]
    #[must_use]
    pub fn sdif_dat3in(&mut self) -> SdifDat3inW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        SdifDat3inW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
This status is used to check DAT line level to recover from errors, and for debugging."]
    #[inline(always)]
    #[must_use]
    pub fn sdif_cmdin(&mut self) -> SdifCmdinW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        SdifCmdinW::new(self, 24)
    }
    #[doc = "Bit 27 - 27:27\\]
Setting of this status indicates that a command cannot be issued due to an error except Auto CMD12 error. \\[Equivalent error status by Auto CMD12 error is defined as Command Not Issued By Auto CMD12 Error in the Auto CMD Error Status register.\\]
This status is set to 1 when Host Controller cannot issue a command after setting Command register or UHS-II Command register.Refer to Section 3.10 about 2L-HD error case inUHS-II mode.Sub Command Status \\[D28\\]
indicates which command is not issued \\[main or sub\\]. 1 - Command cannot be issued 0 - No error for issuing a command"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_not_iss_by_err(
        &mut self,
    ) -> CmdNotIssByErrW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        CmdNotIssByErrW::new(self, 27)
    }
    #[doc = "Bit 28 - 28:28\\]
The Command register and Response register are commonly used for main command and sub command. This status is used to distinguish which response error statuses, main command or sub command, indicated in the Error Interrupt Status register or in the UHS-II Error Interrupt Status register. Refer to Section 1.17 about details of response error statuses. Just before reading of this register, the Sub Command Flag of the Command register or the UHS-II Command register is copied to this status. This status is effective not only when Response Error interrupt is generated but also when data error interrupt is generated with Command Not Issued by Error \\[D27 of this register\\]
or Auto CMD Error interrupt is generated with Command Not Issued by Error by Auto CMD12 in the Auto CMD Error Status register. 1 - Sub Command Status 0 - Main Command Status"]
    #[inline(always)]
    #[must_use]
    pub fn sub_command_sts(&mut self) -> SubCommandStsW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        SubCommandStsW::new(self, 28)
    }
    #[doc = "Bit 29 - 29:29\\]
This status indicates whether UHS-II Ianes enterDormant state. This function is enabled by setting UHS-II Interface Enable to 1 in the Host Control 2 register. On issuing GO_DORMAT_STATE com-mand, Go Dormant Command \\[111b\\]; is set to Command type in the UHS-II Command register.This command type acts as a trigger to enterlanes into dormant state. Host Controller provides STB.H and EIDL on D0 lane and waits for receiv-ing STB.H and EIDL on D1 lane. This bit is set to 1 after the time of T_DMT_ENTRY \\[750 RCLK cycle\\]
or more from detecting EIDL on D1 lane. RCLK may be stopped in dormant state, by setting SD Clock Enable to 0 in the Clock Control register while In Dormant State bit is set to 1. On writingClock Control register with setting SD Clock Enable to 1, Host Controller wakes lanes to exit Dormant State and In Dormant State is set to 0. In case of the card enters Hibernate Mode \\[RCLK is stopped\\], Host Driver may turn off VDD1 by clearing SD Bus Power for VDD1 bit in the Power Control register. Host Controller shall turn off VDD1 after stopping RCLK. This bit is cleared by when Host Controller drives STB.L on D0 lane, UHS-II Interface Enable is set to 0 or executesHost full reset."]
    #[inline(always)]
    #[must_use]
    pub fn uhs2_dormant(&mut self) -> Uhs2DormantW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        Uhs2DormantW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
This status indicates whether lane is synchronized in UHS-II mode. This status is enabled by setting UHS-II Interface Enable to 1 in the Host Control 2 register. On detecting UHS-II Interface \\[D31=1\\], Host Controller provides SYN LSS on D0 lane and waits for receiving SYN LSS on D1 lane. If SYN LSS is detected on D1 lane, Host Controller pro-vides LIDL LSS on D0 lane and waits for receiving LIDL LSS on D1 lane. In case of Version 4.00, this bit indicates comple-tion of Device PHY Initialization by detecting LIDL LSS on D1 lane.From Version 4.10, Host Controller may imple-ment a specific PHY verification method and PHY Initialization Failure can be indicated by keeping this bit to 0 even LIDL LSS is detected on D1 lane.Host Driver detects PHY Initialization Failure by timeout.This bit is cleared by when D0 lane is set to EIDL,UHS-II Interface Enable is set to 0 or executes Host full reset."]
    #[inline(always)]
    #[must_use]
    pub fn uhs2_if_lane_sync(&mut self) -> Uhs2IfLaneSyncW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        Uhs2IfLaneSyncW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
This status indicates whether a card supports UHS-II IF. This status is enabled by setting UHS-II Interface Enable to 1 in the Host Control 2 regis-ter. UHS-II interface initialization is activated by setting SD Clock Enable in the Clock Control reg-ister. Host Controller drives STB.L on D0 lanefrom EIDL state and waits for receiving STB.L on D1 lane. This bit is set to 1 if STB.L is detected on D1 lane. Host Controller shall compensate latency from setting SD Clock Enable to output STB.L on D0 lane when reading this status \\[Refer to Figure 3-35 about details of this method\\]. This bit may be read any time after setting SD Clock Enable for faster UHS-II IF detection but Host Driver shall check this status at least 200us period from set- ting SD Clock Enable until detecting UHS-II IF. After UHS-II IF is detected, this bit is cleared by when EIDL is detected on D0 lane, UHS-II Inter-face Enable is set to 0 or Host full reset is exe-cuted. '0' UHS-II IF is not detected '1' UHS-II IF is detected"]
    #[inline(always)]
    #[must_use]
    pub fn uhs2_if_detection(
        &mut self,
    ) -> Uhs2IfDetectionW<SdhcWrap_CtlCfg_CtlcfgPresentstateSpec> {
        Uhs2IfDetectionW::new(self, 31)
    }
}
#[doc = "The Host Driver can get status of the Host Controller from this 32-bit read-only register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_presentstate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_presentstate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgPresentstateSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgPresentstateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_presentstate::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgPresentstateSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_presentstate::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgPresentstateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_presentstate to value 0x01f0_0000"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgPresentstateSpec {
    const RESET_VALUE: u32 = 0x01f0_0000;
}
