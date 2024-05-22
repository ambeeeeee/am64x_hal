#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_normal_intr_sts` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_normal_intr_sts` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec>;
#[doc = "Field `CMD_COMPLETE` reader - 0:0\\]
SD Mode This bit is set when we get the end bit of the command response \\[Except Auto CMD12 and Auto CMD23\\]
Note: Command Time-out Error has higher priority than Command Complete. If both are set to 1, it can be considered that the response was not received correctly. Version 4.00 defines response check function for R1 and R5. If Response Interrupt Disable in the Transfer Mode register is set to 1, gen-eration of this interrupt is prohibited regardless of Command Complete Signal Enable. UHS-II Mode If Response Interrupt Disable is set to 0 in the UHS-II Transfer Mode register, this interrupt is generated when response packet is received.If Response Interrupt Disable is set to 1 in the UHS-II Transfer Mode register, generation of this interrupt is prohibited regardless of Com-mand Complete Signal Enable. '0' No Command Complete, '1' Command Complete"]
pub type CmdCompleteR = crate::BitReader;
#[doc = "Field `CMD_COMPLETE` writer - 0:0\\]
SD Mode This bit is set when we get the end bit of the command response \\[Except Auto CMD12 and Auto CMD23\\]
Note: Command Time-out Error has higher priority than Command Complete. If both are set to 1, it can be considered that the response was not received correctly. Version 4.00 defines response check function for R1 and R5. If Response Interrupt Disable in the Transfer Mode register is set to 1, gen-eration of this interrupt is prohibited regardless of Command Complete Signal Enable. UHS-II Mode If Response Interrupt Disable is set to 0 in the UHS-II Transfer Mode register, this interrupt is generated when response packet is received.If Response Interrupt Disable is set to 1 in the UHS-II Transfer Mode register, generation of this interrupt is prohibited regardless of Com-mand Complete Signal Enable. '0' No Command Complete, '1' Command Complete"]
pub type CmdCompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFER_COMPLETE` reader - 1:1\\]
This bit is set when a read / write transaction is completed. SD Mode Read Transaction: This bit is set at the falling edge of Read Transfer Active Status. There are two cases in which the Interrupt is generated. The first is when a data transfer is completed as specified by data length \\[After the last data has been read to the Host Sys- tem\\]. The second is when data has stopped at the block gap and completed the data transfer by setting the Stop At Block Gap Request in the Block Gap Control Register \\[After valid data has been read to the Host System\\]. Write Transaction: This bit is set at the falling edge of the DAT Line Active Status. There are two cases in which the Interrupt is generated. The first is when the last data is written to the card as specified by data length and Busy signal is released. The second is when data transfers are stopped at the block gap by setting Stop At Block Gap Request in the Block Gap Control Register and data transfers completed. \\[After valid data is written to the SD card and the busy signal is released\\]. Note: Transfer Complete has higher priority than Data Time-out Error. If both bits are set to 1, the data transfer can be considered complete. Note: While performing tuning procedure \\[Execute Tuning is set to 1\\], Transfer Complete is not set to 1 Command with Busy: This bit is set when busy is de-asserted. Refer to DAT Line Active and Command Inhibit\\[DAT\\]
in the Present State register.UHS-I mode While performing tuning procedure \\[Execute Tuning is set to 1\\], Transfer Complete is not set to 1. '0' No Data Transfer Complete, '1' Data Transfer Complete UHS-II Mode This interrupt is generated in following twocases: \\[a\\]
EBSY Completion \\[for EBSY supported commands\\]
When EBSY Wait in the UHS-II Transfer Mode register is set to 1, this bit is set when EBSY packet has been received, and all valid data have been sent to system memory in case of read operation. \\[b\\]
Stop and Continue during DCMD Data Transfer When Stop At Block Gap Request in the Block Gap Control register is set to 1 and data transfer is stopped at the Flow Control. Following is for both SD mode and UHS-II mode. The table below shows that Transfer Com-plete has higher priority than Data Timeout Error. If both bits are set to 1, execution of a command can be considered to be completed. 1 - Command execution is completed 0 - Not complete"]
pub type XferCompleteR = crate::BitReader;
#[doc = "Field `XFER_COMPLETE` writer - 1:1\\]
This bit is set when a read / write transaction is completed. SD Mode Read Transaction: This bit is set at the falling edge of Read Transfer Active Status. There are two cases in which the Interrupt is generated. The first is when a data transfer is completed as specified by data length \\[After the last data has been read to the Host Sys- tem\\]. The second is when data has stopped at the block gap and completed the data transfer by setting the Stop At Block Gap Request in the Block Gap Control Register \\[After valid data has been read to the Host System\\]. Write Transaction: This bit is set at the falling edge of the DAT Line Active Status. There are two cases in which the Interrupt is generated. The first is when the last data is written to the card as specified by data length and Busy signal is released. The second is when data transfers are stopped at the block gap by setting Stop At Block Gap Request in the Block Gap Control Register and data transfers completed. \\[After valid data is written to the SD card and the busy signal is released\\]. Note: Transfer Complete has higher priority than Data Time-out Error. If both bits are set to 1, the data transfer can be considered complete. Note: While performing tuning procedure \\[Execute Tuning is set to 1\\], Transfer Complete is not set to 1 Command with Busy: This bit is set when busy is de-asserted. Refer to DAT Line Active and Command Inhibit\\[DAT\\]
in the Present State register.UHS-I mode While performing tuning procedure \\[Execute Tuning is set to 1\\], Transfer Complete is not set to 1. '0' No Data Transfer Complete, '1' Data Transfer Complete UHS-II Mode This interrupt is generated in following twocases: \\[a\\]
EBSY Completion \\[for EBSY supported commands\\]
When EBSY Wait in the UHS-II Transfer Mode register is set to 1, this bit is set when EBSY packet has been received, and all valid data have been sent to system memory in case of read operation. \\[b\\]
Stop and Continue during DCMD Data Transfer When Stop At Block Gap Request in the Block Gap Control register is set to 1 and data transfer is stopped at the Flow Control. Following is for both SD mode and UHS-II mode. The table below shows that Transfer Com-plete has higher priority than Data Timeout Error. If both bits are set to 1, execution of a command can be considered to be completed. 1 - Command execution is completed 0 - Not complete"]
pub type XferCompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_GAP_EVENT` reader - 2:2\\]
If the Stop At Block Gap Request in the BlockGap Control Register is set, this bit is set. Read Transaction: This bit is set at the falling edge of the DAT Line Active Status \\[When the transaction is stopped at SD Bus timing. The Read Wait must be supported inorder to use this function\\]. Write Transaction: This bit is set at the falling edge of Write Transfer Active Status \\[After getting CRC status at SD Bus timing\\]. In UHS-II mode, this bit is set at FC \\[Flow Control\\]
unit basis. '0' No Block Gap Event '1' Transaction stopped at Block Gap"]
pub type BlkGapEventR = crate::BitReader;
#[doc = "Field `BLK_GAP_EVENT` writer - 2:2\\]
If the Stop At Block Gap Request in the BlockGap Control Register is set, this bit is set. Read Transaction: This bit is set at the falling edge of the DAT Line Active Status \\[When the transaction is stopped at SD Bus timing. The Read Wait must be supported inorder to use this function\\]. Write Transaction: This bit is set at the falling edge of Write Transfer Active Status \\[After getting CRC status at SD Bus timing\\]. In UHS-II mode, this bit is set at FC \\[Flow Control\\]
unit basis. '0' No Block Gap Event '1' Transaction stopped at Block Gap"]
pub type BlkGapEventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_INTERRUPT` reader - 3:3\\]
This status is set if the HC detects the Host DMA Buffer Boundary in the Block Size regiser. '0' No DMA Interrupt '1' DMA Interrupt is generated"]
pub type DmaInterruptR = crate::BitReader;
#[doc = "Field `DMA_INTERRUPT` writer - 3:3\\]
This status is set if the HC detects the Host DMA Buffer Boundary in the Block Size regiser. '0' No DMA Interrupt '1' DMA Interrupt is generated"]
pub type DmaInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_WR_READY` reader - 4:4\\]
This status is set if the Buffer Write Enable changes from 0 to 1.In UHS-II mode, this bit is set at FC \\[Flow Control\\]
unit basis. '0' Not ready to write to buffer, '1' Ready to write to buffer"]
pub type BufWrReadyR = crate::BitReader;
#[doc = "Field `BUF_WR_READY` writer - 4:4\\]
This status is set if the Buffer Write Enable changes from 0 to 1.In UHS-II mode, this bit is set at FC \\[Flow Control\\]
unit basis. '0' Not ready to write to buffer, '1' Ready to write to buffer"]
pub type BufWrReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_RD_READY` reader - 5:5\\]
This status is set if the Buffer Read Enable changes from 0 to 1. Buffer Read Ready is set to 1 for every CMD19 execution in tuning procedure.In UHS-II mode, this bit is set at FC \\[Flow Control\\]
unit basis. '0' Not ready to read buffer, '1' Ready to read buffer"]
pub type BufRdReadyR = crate::BitReader;
#[doc = "Field `BUF_RD_READY` writer - 5:5\\]
This status is set if the Buffer Read Enable changes from 0 to 1. Buffer Read Ready is set to 1 for every CMD19 execution in tuning procedure.In UHS-II mode, this bit is set at FC \\[Flow Control\\]
unit basis. '0' Not ready to read buffer, '1' Ready to read buffer"]
pub type BufRdReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INS` reader - 6:6\\]
This status is set if the Card Inserted in the Present State register changes from 0 to 1.When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State register should be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated. '0' Card State Stable or Debouncing '1' Card Inserted"]
pub type CardInsR = crate::BitReader;
#[doc = "Field `CARD_INS` writer - 6:6\\]
This status is set if the Card Inserted in the Present State register changes from 0 to 1.When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State register should be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated. '0' Card State Stable or Debouncing '1' Card Inserted"]
pub type CardInsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_REM` reader - 7:7\\]
This status is set if the Card Inserted in the Present State register changes from 1 to 0. When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State register should be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated. '0' Card State Stable or Debouncing '1' Card Removed"]
pub type CardRemR = crate::BitReader;
#[doc = "Field `CARD_REM` writer - 7:7\\]
This status is set if the Card Inserted in the Present State register changes from 1 to 0. When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State register should be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated. '0' Card State Stable or Debouncing '1' Card Removed"]
pub type CardRemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_INTR` reader - 8:8\\]
When this status has been set and the Host Driver needs to start this interrupt service, Card Interrupt Status Enable in the Normal Interrupt Status Enable register may be set to 0 in order to clear the card interrupt status latched in the Host Controller and to stop driv-ing the interrupt signal to the Host System.After completion of the card interrupt service \\[It should reset interrupt factors in the SD card and the interrupt signal may not be asserted\\],set Card Interrupt Status Enable to 1 and start sampling the interrupt signal again. Writing this bit to 1 does not clear this bit. It is cleared by resetting the SD card interrupt fac-tor. \\[1\\]
DAT\\[1\\]
Interrupt Input in SD Mode In 1-bit mode, the Host Controller shall detect the Card Interrupt without SD Clock to support wakeup. In 4-bit mode, the card interrupt sig-nal is sampled during the interrupt cycle, so there are some sample delays between the interrupt signal from the SD card and the inter-rupt to the Host System. Interrupt detected by DAT\\[1\\]
is supported when there is a card per slot. In case of UHS-I mode, switching time of Interrupt Period is relaxed for 2 clock cycles. Then Host Controller needs to delay start of interrupt sampling at least 2 clocks and should sample interrupt while Interrupt Period is sta-ble. \\[2\\]
DAT\\[2\\]
Interrupt Input in UHS-II Mode When Card Inserted in the Present State reg-ister and SD Bus Power for VDD1 in the Power Control register are set to 1, Host Con-troller configures DAT\\[2\\]
as Interrupt Input and enables pull-up of DAT\\[2\\]. DAT\\[2\\]
interrupt is asynchronous to RCLK, low level sensitive and 3.3V signal level. DAT\\[2\\]
interrupt is masked by setting Card Interrupt Status Enable to 0 in the Normal Interrupt register.When either Card Inserted or SD Bus Power for VDD1 is set to 0, Host Controller sets DAT\\[2\\]
to low. Only point to point connection is allowed between Host and Card. \\[3\\]
INT MSG in UHS-II Mode INT MSG is enabled by setting INT MSG Enable in the UHS-II Device Select register. DAT\\[2\\]
and INT MSG interrupt sources are ORed and indicated to Card Interrupt. If any bit in the UHS-II Device Interrupt Status regis- ter is set to 1, INT MSG interrupt is generated. INT MSG interrupt is cleared by writing a cor-respondent bit to 1 in the UHS-II Device Inter-rupt Status register. Masking DAT\\[2\\]
interrupt also disables INT MSG interupt due to Card Interrupt Status Enable is set to 0. SDIO Ver-sion 4.00 does not support INT MSG. '0' No Card Interrupt '1' Generate Card Interrupt"]
pub type CardIntrR = crate::BitReader;
#[doc = "Field `CARD_INTR` writer - 8:8\\]
When this status has been set and the Host Driver needs to start this interrupt service, Card Interrupt Status Enable in the Normal Interrupt Status Enable register may be set to 0 in order to clear the card interrupt status latched in the Host Controller and to stop driv-ing the interrupt signal to the Host System.After completion of the card interrupt service \\[It should reset interrupt factors in the SD card and the interrupt signal may not be asserted\\],set Card Interrupt Status Enable to 1 and start sampling the interrupt signal again. Writing this bit to 1 does not clear this bit. It is cleared by resetting the SD card interrupt fac-tor. \\[1\\]
DAT\\[1\\]
Interrupt Input in SD Mode In 1-bit mode, the Host Controller shall detect the Card Interrupt without SD Clock to support wakeup. In 4-bit mode, the card interrupt sig-nal is sampled during the interrupt cycle, so there are some sample delays between the interrupt signal from the SD card and the inter-rupt to the Host System. Interrupt detected by DAT\\[1\\]
is supported when there is a card per slot. In case of UHS-I mode, switching time of Interrupt Period is relaxed for 2 clock cycles. Then Host Controller needs to delay start of interrupt sampling at least 2 clocks and should sample interrupt while Interrupt Period is sta-ble. \\[2\\]
DAT\\[2\\]
Interrupt Input in UHS-II Mode When Card Inserted in the Present State reg-ister and SD Bus Power for VDD1 in the Power Control register are set to 1, Host Con-troller configures DAT\\[2\\]
as Interrupt Input and enables pull-up of DAT\\[2\\]. DAT\\[2\\]
interrupt is asynchronous to RCLK, low level sensitive and 3.3V signal level. DAT\\[2\\]
interrupt is masked by setting Card Interrupt Status Enable to 0 in the Normal Interrupt register.When either Card Inserted or SD Bus Power for VDD1 is set to 0, Host Controller sets DAT\\[2\\]
to low. Only point to point connection is allowed between Host and Card. \\[3\\]
INT MSG in UHS-II Mode INT MSG is enabled by setting INT MSG Enable in the UHS-II Device Select register. DAT\\[2\\]
and INT MSG interrupt sources are ORed and indicated to Card Interrupt. If any bit in the UHS-II Device Interrupt Status regis- ter is set to 1, INT MSG interrupt is generated. INT MSG interrupt is cleared by writing a cor-respondent bit to 1 in the UHS-II Device Inter-rupt Status register. Masking DAT\\[2\\]
interrupt also disables INT MSG interupt due to Card Interrupt Status Enable is set to 0. SDIO Ver-sion 4.00 does not support INT MSG. '0' No Card Interrupt '1' Generate Card Interrupt"]
pub type CardIntrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTA` reader - 9:9\\]
This status is set if INT_A is enabled and INT_A# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_A interrupt factor. NOTE : INT_A, INT_B, and INT_C are to be implemented based on the Application Requirements. By default these are not implemented as there is no specific requirement from Customers."]
pub type IntaR = crate::BitReader;
#[doc = "Field `INTA` writer - 9:9\\]
This status is set if INT_A is enabled and INT_A# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_A interrupt factor. NOTE : INT_A, INT_B, and INT_C are to be implemented based on the Application Requirements. By default these are not implemented as there is no specific requirement from Customers."]
pub type IntaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTB` reader - 10:10\\]
This status is set if INT_B is enabled and INT_B# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_B interrupt factor."]
pub type IntbR = crate::BitReader;
#[doc = "Field `INTB` writer - 10:10\\]
This status is set if INT_B is enabled and INT_B# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_B interrupt factor."]
pub type IntbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTC` reader - 11:11\\]
This status is set if INT_C is enabled and INT_C# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_C interrupt factor."]
pub type IntcR = crate::BitReader;
#[doc = "Field `INTC` writer - 11:11\\]
This status is set if INT_C is enabled and INT_C# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_C interrupt factor."]
pub type IntcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RETUNING_EVENT` reader - 12:12\\]
This status is set if Re-Tuning Request in the Present State register changes from 0 to 1. Host Controller requests Host Driver to perform re-tuning for next data transfer. Current data transfer \\[not large block count\\]
can be completed without re-tuning.In UHS-II mode, this bit is not effective '0' Re-tuning not required '1' Re-tuning should be performed"]
pub type RetuningEventR = crate::BitReader;
#[doc = "Field `RETUNING_EVENT` writer - 12:12\\]
This status is set if Re-Tuning Request in the Present State register changes from 0 to 1. Host Controller requests Host Driver to perform re-tuning for next data transfer. Current data transfer \\[not large block count\\]
can be completed without re-tuning.In UHS-II mode, this bit is not effective '0' Re-tuning not required '1' Re-tuning should be performed"]
pub type RetuningEventW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCV_BOOT_ACK` reader - 13:13\\]
This status is set if the boot acknowledge is received from device. '0' Boot ack not recieved '1' Boot ack is recieved"]
pub type RcvBootAckR = crate::BitReader;
#[doc = "Field `RCV_BOOT_ACK` writer - 13:13\\]
This status is set if the boot acknowledge is received from device. '0' Boot ack not recieved '1' Boot ack is recieved"]
pub type RcvBootAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_COMPLETE` reader - 14:14\\]
This status is set if the boot operation gets terminated. '0' Boot operation is not terminated '1' Boot operation is terminated"]
pub type BootCompleteR = crate::BitReader;
#[doc = "Field `BOOT_COMPLETE` writer - 14:14\\]
This status is set if the boot operation gets terminated. '0' Boot operation is not terminated '1' Boot operation is terminated"]
pub type BootCompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR_INTR` reader - 15:15\\]
If any of the bits in the Error Interrupt Status Register are set, then this bit is set. Therefore the HD can test for an error by checking this bit first. In UHS-II mode is enabled, if any of the bits in the UHS-II Error Interrupt Status register are set, this bit is also set. '0' No error '1' Error"]
pub type ErrorIntrR = crate::BitReader;
#[doc = "Field `ERROR_INTR` writer - 15:15\\]
If any of the bits in the Error Interrupt Status Register are set, then this bit is set. Therefore the HD can test for an error by checking this bit first. In UHS-II mode is enabled, if any of the bits in the UHS-II Error Interrupt Status register are set, this bit is also set. '0' No error '1' Error"]
pub type ErrorIntrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
SD Mode This bit is set when we get the end bit of the command response \\[Except Auto CMD12 and Auto CMD23\\]
Note: Command Time-out Error has higher priority than Command Complete. If both are set to 1, it can be considered that the response was not received correctly. Version 4.00 defines response check function for R1 and R5. If Response Interrupt Disable in the Transfer Mode register is set to 1, gen-eration of this interrupt is prohibited regardless of Command Complete Signal Enable. UHS-II Mode If Response Interrupt Disable is set to 0 in the UHS-II Transfer Mode register, this interrupt is generated when response packet is received.If Response Interrupt Disable is set to 1 in the UHS-II Transfer Mode register, generation of this interrupt is prohibited regardless of Com-mand Complete Signal Enable. '0' No Command Complete, '1' Command Complete"]
    #[inline(always)]
    pub fn cmd_complete(&self) -> CmdCompleteR {
        CmdCompleteR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit is set when a read / write transaction is completed. SD Mode Read Transaction: This bit is set at the falling edge of Read Transfer Active Status. There are two cases in which the Interrupt is generated. The first is when a data transfer is completed as specified by data length \\[After the last data has been read to the Host Sys- tem\\]. The second is when data has stopped at the block gap and completed the data transfer by setting the Stop At Block Gap Request in the Block Gap Control Register \\[After valid data has been read to the Host System\\]. Write Transaction: This bit is set at the falling edge of the DAT Line Active Status. There are two cases in which the Interrupt is generated. The first is when the last data is written to the card as specified by data length and Busy signal is released. The second is when data transfers are stopped at the block gap by setting Stop At Block Gap Request in the Block Gap Control Register and data transfers completed. \\[After valid data is written to the SD card and the busy signal is released\\]. Note: Transfer Complete has higher priority than Data Time-out Error. If both bits are set to 1, the data transfer can be considered complete. Note: While performing tuning procedure \\[Execute Tuning is set to 1\\], Transfer Complete is not set to 1 Command with Busy: This bit is set when busy is de-asserted. Refer to DAT Line Active and Command Inhibit\\[DAT\\]
in the Present State register.UHS-I mode While performing tuning procedure \\[Execute Tuning is set to 1\\], Transfer Complete is not set to 1. '0' No Data Transfer Complete, '1' Data Transfer Complete UHS-II Mode This interrupt is generated in following twocases: \\[a\\]
EBSY Completion \\[for EBSY supported commands\\]
When EBSY Wait in the UHS-II Transfer Mode register is set to 1, this bit is set when EBSY packet has been received, and all valid data have been sent to system memory in case of read operation. \\[b\\]
Stop and Continue during DCMD Data Transfer When Stop At Block Gap Request in the Block Gap Control register is set to 1 and data transfer is stopped at the Flow Control. Following is for both SD mode and UHS-II mode. The table below shows that Transfer Com-plete has higher priority than Data Timeout Error. If both bits are set to 1, execution of a command can be considered to be completed. 1 - Command execution is completed 0 - Not complete"]
    #[inline(always)]
    pub fn xfer_complete(&self) -> XferCompleteR {
        XferCompleteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
If the Stop At Block Gap Request in the BlockGap Control Register is set, this bit is set. Read Transaction: This bit is set at the falling edge of the DAT Line Active Status \\[When the transaction is stopped at SD Bus timing. The Read Wait must be supported inorder to use this function\\]. Write Transaction: This bit is set at the falling edge of Write Transfer Active Status \\[After getting CRC status at SD Bus timing\\]. In UHS-II mode, this bit is set at FC \\[Flow Control\\]
unit basis. '0' No Block Gap Event '1' Transaction stopped at Block Gap"]
    #[inline(always)]
    pub fn blk_gap_event(&self) -> BlkGapEventR {
        BlkGapEventR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
This status is set if the HC detects the Host DMA Buffer Boundary in the Block Size regiser. '0' No DMA Interrupt '1' DMA Interrupt is generated"]
    #[inline(always)]
    pub fn dma_interrupt(&self) -> DmaInterruptR {
        DmaInterruptR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
This status is set if the Buffer Write Enable changes from 0 to 1.In UHS-II mode, this bit is set at FC \\[Flow Control\\]
unit basis. '0' Not ready to write to buffer, '1' Ready to write to buffer"]
    #[inline(always)]
    pub fn buf_wr_ready(&self) -> BufWrReadyR {
        BufWrReadyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
This status is set if the Buffer Read Enable changes from 0 to 1. Buffer Read Ready is set to 1 for every CMD19 execution in tuning procedure.In UHS-II mode, this bit is set at FC \\[Flow Control\\]
unit basis. '0' Not ready to read buffer, '1' Ready to read buffer"]
    #[inline(always)]
    pub fn buf_rd_ready(&self) -> BufRdReadyR {
        BufRdReadyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
This status is set if the Card Inserted in the Present State register changes from 0 to 1.When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State register should be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated. '0' Card State Stable or Debouncing '1' Card Inserted"]
    #[inline(always)]
    pub fn card_ins(&self) -> CardInsR {
        CardInsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
This status is set if the Card Inserted in the Present State register changes from 1 to 0. When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State register should be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated. '0' Card State Stable or Debouncing '1' Card Removed"]
    #[inline(always)]
    pub fn card_rem(&self) -> CardRemR {
        CardRemR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
When this status has been set and the Host Driver needs to start this interrupt service, Card Interrupt Status Enable in the Normal Interrupt Status Enable register may be set to 0 in order to clear the card interrupt status latched in the Host Controller and to stop driv-ing the interrupt signal to the Host System.After completion of the card interrupt service \\[It should reset interrupt factors in the SD card and the interrupt signal may not be asserted\\],set Card Interrupt Status Enable to 1 and start sampling the interrupt signal again. Writing this bit to 1 does not clear this bit. It is cleared by resetting the SD card interrupt fac-tor. \\[1\\]
DAT\\[1\\]
Interrupt Input in SD Mode In 1-bit mode, the Host Controller shall detect the Card Interrupt without SD Clock to support wakeup. In 4-bit mode, the card interrupt sig-nal is sampled during the interrupt cycle, so there are some sample delays between the interrupt signal from the SD card and the inter-rupt to the Host System. Interrupt detected by DAT\\[1\\]
is supported when there is a card per slot. In case of UHS-I mode, switching time of Interrupt Period is relaxed for 2 clock cycles. Then Host Controller needs to delay start of interrupt sampling at least 2 clocks and should sample interrupt while Interrupt Period is sta-ble. \\[2\\]
DAT\\[2\\]
Interrupt Input in UHS-II Mode When Card Inserted in the Present State reg-ister and SD Bus Power for VDD1 in the Power Control register are set to 1, Host Con-troller configures DAT\\[2\\]
as Interrupt Input and enables pull-up of DAT\\[2\\]. DAT\\[2\\]
interrupt is asynchronous to RCLK, low level sensitive and 3.3V signal level. DAT\\[2\\]
interrupt is masked by setting Card Interrupt Status Enable to 0 in the Normal Interrupt register.When either Card Inserted or SD Bus Power for VDD1 is set to 0, Host Controller sets DAT\\[2\\]
to low. Only point to point connection is allowed between Host and Card. \\[3\\]
INT MSG in UHS-II Mode INT MSG is enabled by setting INT MSG Enable in the UHS-II Device Select register. DAT\\[2\\]
and INT MSG interrupt sources are ORed and indicated to Card Interrupt. If any bit in the UHS-II Device Interrupt Status regis- ter is set to 1, INT MSG interrupt is generated. INT MSG interrupt is cleared by writing a cor-respondent bit to 1 in the UHS-II Device Inter-rupt Status register. Masking DAT\\[2\\]
interrupt also disables INT MSG interupt due to Card Interrupt Status Enable is set to 0. SDIO Ver-sion 4.00 does not support INT MSG. '0' No Card Interrupt '1' Generate Card Interrupt"]
    #[inline(always)]
    pub fn card_intr(&self) -> CardIntrR {
        CardIntrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
This status is set if INT_A is enabled and INT_A# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_A interrupt factor. NOTE : INT_A, INT_B, and INT_C are to be implemented based on the Application Requirements. By default these are not implemented as there is no specific requirement from Customers."]
    #[inline(always)]
    pub fn inta(&self) -> IntaR {
        IntaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
This status is set if INT_B is enabled and INT_B# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_B interrupt factor."]
    #[inline(always)]
    pub fn intb(&self) -> IntbR {
        IntbR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
This status is set if INT_C is enabled and INT_C# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_C interrupt factor."]
    #[inline(always)]
    pub fn intc(&self) -> IntcR {
        IntcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
This status is set if Re-Tuning Request in the Present State register changes from 0 to 1. Host Controller requests Host Driver to perform re-tuning for next data transfer. Current data transfer \\[not large block count\\]
can be completed without re-tuning.In UHS-II mode, this bit is not effective '0' Re-tuning not required '1' Re-tuning should be performed"]
    #[inline(always)]
    pub fn retuning_event(&self) -> RetuningEventR {
        RetuningEventR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
This status is set if the boot acknowledge is received from device. '0' Boot ack not recieved '1' Boot ack is recieved"]
    #[inline(always)]
    pub fn rcv_boot_ack(&self) -> RcvBootAckR {
        RcvBootAckR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
This status is set if the boot operation gets terminated. '0' Boot operation is not terminated '1' Boot operation is terminated"]
    #[inline(always)]
    pub fn boot_complete(&self) -> BootCompleteR {
        BootCompleteR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
If any of the bits in the Error Interrupt Status Register are set, then this bit is set. Therefore the HD can test for an error by checking this bit first. In UHS-II mode is enabled, if any of the bits in the UHS-II Error Interrupt Status register are set, this bit is also set. '0' No error '1' Error"]
    #[inline(always)]
    pub fn error_intr(&self) -> ErrorIntrR {
        ErrorIntrR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
SD Mode This bit is set when we get the end bit of the command response \\[Except Auto CMD12 and Auto CMD23\\]
Note: Command Time-out Error has higher priority than Command Complete. If both are set to 1, it can be considered that the response was not received correctly. Version 4.00 defines response check function for R1 and R5. If Response Interrupt Disable in the Transfer Mode register is set to 1, gen-eration of this interrupt is prohibited regardless of Command Complete Signal Enable. UHS-II Mode If Response Interrupt Disable is set to 0 in the UHS-II Transfer Mode register, this interrupt is generated when response packet is received.If Response Interrupt Disable is set to 1 in the UHS-II Transfer Mode register, generation of this interrupt is prohibited regardless of Com-mand Complete Signal Enable. '0' No Command Complete, '1' Command Complete"]
    #[inline(always)]
    #[must_use]
    pub fn cmd_complete(&mut self) -> CmdCompleteW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec> {
        CmdCompleteW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit is set when a read / write transaction is completed. SD Mode Read Transaction: This bit is set at the falling edge of Read Transfer Active Status. There are two cases in which the Interrupt is generated. The first is when a data transfer is completed as specified by data length \\[After the last data has been read to the Host Sys- tem\\]. The second is when data has stopped at the block gap and completed the data transfer by setting the Stop At Block Gap Request in the Block Gap Control Register \\[After valid data has been read to the Host System\\]. Write Transaction: This bit is set at the falling edge of the DAT Line Active Status. There are two cases in which the Interrupt is generated. The first is when the last data is written to the card as specified by data length and Busy signal is released. The second is when data transfers are stopped at the block gap by setting Stop At Block Gap Request in the Block Gap Control Register and data transfers completed. \\[After valid data is written to the SD card and the busy signal is released\\]. Note: Transfer Complete has higher priority than Data Time-out Error. If both bits are set to 1, the data transfer can be considered complete. Note: While performing tuning procedure \\[Execute Tuning is set to 1\\], Transfer Complete is not set to 1 Command with Busy: This bit is set when busy is de-asserted. Refer to DAT Line Active and Command Inhibit\\[DAT\\]
in the Present State register.UHS-I mode While performing tuning procedure \\[Execute Tuning is set to 1\\], Transfer Complete is not set to 1. '0' No Data Transfer Complete, '1' Data Transfer Complete UHS-II Mode This interrupt is generated in following twocases: \\[a\\]
EBSY Completion \\[for EBSY supported commands\\]
When EBSY Wait in the UHS-II Transfer Mode register is set to 1, this bit is set when EBSY packet has been received, and all valid data have been sent to system memory in case of read operation. \\[b\\]
Stop and Continue during DCMD Data Transfer When Stop At Block Gap Request in the Block Gap Control register is set to 1 and data transfer is stopped at the Flow Control. Following is for both SD mode and UHS-II mode. The table below shows that Transfer Com-plete has higher priority than Data Timeout Error. If both bits are set to 1, execution of a command can be considered to be completed. 1 - Command execution is completed 0 - Not complete"]
    #[inline(always)]
    #[must_use]
    pub fn xfer_complete(&mut self) -> XferCompleteW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec> {
        XferCompleteW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
If the Stop At Block Gap Request in the BlockGap Control Register is set, this bit is set. Read Transaction: This bit is set at the falling edge of the DAT Line Active Status \\[When the transaction is stopped at SD Bus timing. The Read Wait must be supported inorder to use this function\\]. Write Transaction: This bit is set at the falling edge of Write Transfer Active Status \\[After getting CRC status at SD Bus timing\\]. In UHS-II mode, this bit is set at FC \\[Flow Control\\]
unit basis. '0' No Block Gap Event '1' Transaction stopped at Block Gap"]
    #[inline(always)]
    #[must_use]
    pub fn blk_gap_event(&mut self) -> BlkGapEventW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec> {
        BlkGapEventW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
This status is set if the HC detects the Host DMA Buffer Boundary in the Block Size regiser. '0' No DMA Interrupt '1' DMA Interrupt is generated"]
    #[inline(always)]
    #[must_use]
    pub fn dma_interrupt(&mut self) -> DmaInterruptW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec> {
        DmaInterruptW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
This status is set if the Buffer Write Enable changes from 0 to 1.In UHS-II mode, this bit is set at FC \\[Flow Control\\]
unit basis. '0' Not ready to write to buffer, '1' Ready to write to buffer"]
    #[inline(always)]
    #[must_use]
    pub fn buf_wr_ready(&mut self) -> BufWrReadyW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec> {
        BufWrReadyW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
This status is set if the Buffer Read Enable changes from 0 to 1. Buffer Read Ready is set to 1 for every CMD19 execution in tuning procedure.In UHS-II mode, this bit is set at FC \\[Flow Control\\]
unit basis. '0' Not ready to read buffer, '1' Ready to read buffer"]
    #[inline(always)]
    #[must_use]
    pub fn buf_rd_ready(&mut self) -> BufRdReadyW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec> {
        BufRdReadyW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
This status is set if the Card Inserted in the Present State register changes from 0 to 1.When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State register should be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated. '0' Card State Stable or Debouncing '1' Card Inserted"]
    #[inline(always)]
    #[must_use]
    pub fn card_ins(&mut self) -> CardInsW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec> {
        CardInsW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
This status is set if the Card Inserted in the Present State register changes from 1 to 0. When the HD writes this bit to 1 to clear this status the status of the Card Inserted in the Present State register should be confirmed. Because the card detect may possibly be changed when the HD clear this bit an Interrupt event may not be generated. '0' Card State Stable or Debouncing '1' Card Removed"]
    #[inline(always)]
    #[must_use]
    pub fn card_rem(&mut self) -> CardRemW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec> {
        CardRemW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
When this status has been set and the Host Driver needs to start this interrupt service, Card Interrupt Status Enable in the Normal Interrupt Status Enable register may be set to 0 in order to clear the card interrupt status latched in the Host Controller and to stop driv-ing the interrupt signal to the Host System.After completion of the card interrupt service \\[It should reset interrupt factors in the SD card and the interrupt signal may not be asserted\\],set Card Interrupt Status Enable to 1 and start sampling the interrupt signal again. Writing this bit to 1 does not clear this bit. It is cleared by resetting the SD card interrupt fac-tor. \\[1\\]
DAT\\[1\\]
Interrupt Input in SD Mode In 1-bit mode, the Host Controller shall detect the Card Interrupt without SD Clock to support wakeup. In 4-bit mode, the card interrupt sig-nal is sampled during the interrupt cycle, so there are some sample delays between the interrupt signal from the SD card and the inter-rupt to the Host System. Interrupt detected by DAT\\[1\\]
is supported when there is a card per slot. In case of UHS-I mode, switching time of Interrupt Period is relaxed for 2 clock cycles. Then Host Controller needs to delay start of interrupt sampling at least 2 clocks and should sample interrupt while Interrupt Period is sta-ble. \\[2\\]
DAT\\[2\\]
Interrupt Input in UHS-II Mode When Card Inserted in the Present State reg-ister and SD Bus Power for VDD1 in the Power Control register are set to 1, Host Con-troller configures DAT\\[2\\]
as Interrupt Input and enables pull-up of DAT\\[2\\]. DAT\\[2\\]
interrupt is asynchronous to RCLK, low level sensitive and 3.3V signal level. DAT\\[2\\]
interrupt is masked by setting Card Interrupt Status Enable to 0 in the Normal Interrupt register.When either Card Inserted or SD Bus Power for VDD1 is set to 0, Host Controller sets DAT\\[2\\]
to low. Only point to point connection is allowed between Host and Card. \\[3\\]
INT MSG in UHS-II Mode INT MSG is enabled by setting INT MSG Enable in the UHS-II Device Select register. DAT\\[2\\]
and INT MSG interrupt sources are ORed and indicated to Card Interrupt. If any bit in the UHS-II Device Interrupt Status regis- ter is set to 1, INT MSG interrupt is generated. INT MSG interrupt is cleared by writing a cor-respondent bit to 1 in the UHS-II Device Inter-rupt Status register. Masking DAT\\[2\\]
interrupt also disables INT MSG interupt due to Card Interrupt Status Enable is set to 0. SDIO Ver-sion 4.00 does not support INT MSG. '0' No Card Interrupt '1' Generate Card Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn card_intr(&mut self) -> CardIntrW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec> {
        CardIntrW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
This status is set if INT_A is enabled and INT_A# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_A interrupt factor. NOTE : INT_A, INT_B, and INT_C are to be implemented based on the Application Requirements. By default these are not implemented as there is no specific requirement from Customers."]
    #[inline(always)]
    #[must_use]
    pub fn inta(&mut self) -> IntaW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec> {
        IntaW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
This status is set if INT_B is enabled and INT_B# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_B interrupt factor."]
    #[inline(always)]
    #[must_use]
    pub fn intb(&mut self) -> IntbW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec> {
        IntbW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
This status is set if INT_C is enabled and INT_C# pin is in low level. Writing this bit to 1 does not clear this bit. It is cleared by resetting the INT_C interrupt factor."]
    #[inline(always)]
    #[must_use]
    pub fn intc(&mut self) -> IntcW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec> {
        IntcW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
This status is set if Re-Tuning Request in the Present State register changes from 0 to 1. Host Controller requests Host Driver to perform re-tuning for next data transfer. Current data transfer \\[not large block count\\]
can be completed without re-tuning.In UHS-II mode, this bit is not effective '0' Re-tuning not required '1' Re-tuning should be performed"]
    #[inline(always)]
    #[must_use]
    pub fn retuning_event(&mut self) -> RetuningEventW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec> {
        RetuningEventW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
This status is set if the boot acknowledge is received from device. '0' Boot ack not recieved '1' Boot ack is recieved"]
    #[inline(always)]
    #[must_use]
    pub fn rcv_boot_ack(&mut self) -> RcvBootAckW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec> {
        RcvBootAckW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
This status is set if the boot operation gets terminated. '0' Boot operation is not terminated '1' Boot operation is terminated"]
    #[inline(always)]
    #[must_use]
    pub fn boot_complete(&mut self) -> BootCompleteW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec> {
        BootCompleteW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
If any of the bits in the Error Interrupt Status Register are set, then this bit is set. Therefore the HD can test for an error by checking this bit first. In UHS-II mode is enabled, if any of the bits in the UHS-II Error Interrupt Status register are set, this bit is also set. '0' No error '1' Error"]
    #[inline(always)]
    #[must_use]
    pub fn error_intr(&mut self) -> ErrorIntrW<SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec> {
        ErrorIntrW::new(self, 15)
    }
}
#[doc = "This register gives the status of all the interrupts\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_normal_intr_sts::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_normal_intr_sts to value 0"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgNormalIntrStsSpec {
    const RESET_VALUE: u16 = 0;
}
