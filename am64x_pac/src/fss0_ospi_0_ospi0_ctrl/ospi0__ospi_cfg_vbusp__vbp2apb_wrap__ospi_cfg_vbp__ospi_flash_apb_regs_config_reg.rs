#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_config_reg` reader"]
pub type R = crate::R<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec>;
#[doc = "Register `OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_config_reg` writer"]
pub type W = crate::W<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec>;
#[doc = "Field `ENB_SPI_FLD` reader - 0:0\\]
Octal-SPI Enable: 0 : disable the Octal-SPI, once current transfer of the data word \\[FF_W\\]
is complete. 1 : enable the Octal-SPI, when spi_enable = 0, all output enables are inactive and all pins are set to input mode."]
pub type EnbSpiFldR = crate::BitReader;
#[doc = "Field `ENB_SPI_FLD` writer - 0:0\\]
Octal-SPI Enable: 0 : disable the Octal-SPI, once current transfer of the data word \\[FF_W\\]
is complete. 1 : enable the Octal-SPI, when spi_enable = 0, all output enables are inactive and all pins are set to input mode."]
pub type EnbSpiFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_CLK_POL_FLD` reader - 1:1\\]
Clock polarity outside SPI word: 0 : the SPI clock is quiescent low 1 : the SPI clock is quiescent high"]
pub type SelClkPolFldR = crate::BitReader;
#[doc = "Field `SEL_CLK_POL_FLD` writer - 1:1\\]
Clock polarity outside SPI word: 0 : the SPI clock is quiescent low 1 : the SPI clock is quiescent high"]
pub type SelClkPolFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEL_CLK_PHASE_FLD` reader - 2:2\\]
Select Clock Phase: Selects whether the clock is in an active or inactive phase outside the SPI word. 0 : the SPI clock is active outside the word 1 : the SPI clock is inactive outside the word"]
pub type SelClkPhaseFldR = crate::BitReader;
#[doc = "Field `SEL_CLK_PHASE_FLD` writer - 2:2\\]
Select Clock Phase: Selects whether the clock is in an active or inactive phase outside the SPI word. 0 : the SPI clock is active outside the word 1 : the SPI clock is inactive outside the word"]
pub type SelClkPhaseFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_MODE_ENABLE_FLD` reader - 3:3\\]
PHY mode enable: When enabled, the controller is informed that PHY Module is to be used for handling SPI transfers. This bit is relevant only for configuration with PHY Module."]
pub type PhyModeEnableFldR = crate::BitReader;
#[doc = "Field `PHY_MODE_ENABLE_FLD` writer - 3:3\\]
PHY mode enable: When enabled, the controller is informed that PHY Module is to be used for handling SPI transfers. This bit is relevant only for configuration with PHY Module."]
pub type PhyModeEnableFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOLD_PIN_FLD` reader - 4:4\\]
Set to drive the HOLD pin of the FLASH device and reset for de-activation of the HOLD pin feature"]
pub type HoldPinFldR = crate::BitReader;
#[doc = "Field `HOLD_PIN_FLD` writer - 4:4\\]
Set to drive the HOLD pin of the FLASH device and reset for de-activation of the HOLD pin feature"]
pub type HoldPinFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_PIN_FLD` reader - 5:5\\]
Set to drive the RESET pin of the FLASH device and reset for de-activation of the RESET pin feature"]
pub type ResetPinFldR = crate::BitReader;
#[doc = "Field `RESET_PIN_FLD` writer - 5:5\\]
Set to drive the RESET pin of the FLASH device and reset for de-activation of the RESET pin feature"]
pub type ResetPinFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_CFG_FLD` reader - 6:6\\]
RESET pin configuration: 0 = RESET feature on DQ3 pin of the device 1 = RESET feature on dedicated pin of the device \\[controlling of 5th bit influences on reset_out output\\]"]
pub type ResetCfgFldR = crate::BitReader;
#[doc = "Field `RESET_CFG_FLD` writer - 6:6\\]
RESET pin configuration: 0 = RESET feature on DQ3 pin of the device 1 = RESET feature on dedicated pin of the device \\[controlling of 5th bit influences on reset_out output\\]"]
pub type ResetCfgFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENB_DIR_ACC_CTLR_FLD` reader - 7:7\\]
Enable Direct Access Controller: 0 : disable the Direct Access Controller once current transfer of the data word \\[FF_W\\]
is complete. 1 : enable the Direct Access Controller When the Direct Access Controller and Indirect Access Controller are both disabled, all AHB requested are completed with an error response."]
pub type EnbDirAccCtlrFldR = crate::BitReader;
#[doc = "Field `ENB_DIR_ACC_CTLR_FLD` writer - 7:7\\]
Enable Direct Access Controller: 0 : disable the Direct Access Controller once current transfer of the data word \\[FF_W\\]
is complete. 1 : enable the Direct Access Controller When the Direct Access Controller and Indirect Access Controller are both disabled, all AHB requested are completed with an error response."]
pub type EnbDirAccCtlrFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENB_LEGACY_IP_MODE_FLD` reader - 8:8\\]
Legacy IP Mode Enable: 0 : Use Direct Access Controller/Indirect Access Controller 1 : legacy Mode is enabled. In this mode, any write to the controller via the AHB interface is serialized and sent to the FLASH device. Any valid AHB read will pop the internal RX-FIFO, retrieving data that was forwarded by the external FLASH device on the SPI lines,4,2 or 1 byte transfers are permitted and controlled via the HSIZE input."]
pub type EnbLegacyIpModeFldR = crate::BitReader;
#[doc = "Field `ENB_LEGACY_IP_MODE_FLD` writer - 8:8\\]
Legacy IP Mode Enable: 0 : Use Direct Access Controller/Indirect Access Controller 1 : legacy Mode is enabled. In this mode, any write to the controller via the AHB interface is serialized and sent to the FLASH device. Any valid AHB read will pop the internal RX-FIFO, retrieving data that was forwarded by the external FLASH device on the SPI lines,4,2 or 1 byte transfers are permitted and controlled via the HSIZE input."]
pub type EnbLegacyIpModeFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERIPH_SEL_DEC_FLD` reader - 9:9\\]
Peripheral select decode: 0 : only 1 of 4 selects n_ss_out\\[3:0\\]
is active 1 : allow external 4-to-16 decode \\[n_ss_out = ss\\]"]
pub type PeriphSelDecFldR = crate::BitReader;
#[doc = "Field `PERIPH_SEL_DEC_FLD` writer - 9:9\\]
Peripheral select decode: 0 : only 1 of 4 selects n_ss_out\\[3:0\\]
is active 1 : allow external 4-to-16 decode \\[n_ss_out = ss\\]"]
pub type PeriphSelDecFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERIPH_CS_LINES_FLD` reader - 13:10\\]
Peripheral Chip Select Lines: Peripheral chip select lines If pdec = 0, ss\\[3:0\\]
are output thus: ss\\[3:0\\]
n_ss_out\\[3:0\\]
xxx0 1110 xx01 1101 x011 1011 0111 0111 1111 1111 \\[no peripheral selected\\]
else ss\\[3:0\\]
directly drives n_ss_out\\[3:0\\]"]
pub type PeriphCsLinesFldR = crate::FieldReader;
#[doc = "Field `PERIPH_CS_LINES_FLD` writer - 13:10\\]
Peripheral Chip Select Lines: Peripheral chip select lines If pdec = 0, ss\\[3:0\\]
are output thus: ss\\[3:0\\]
n_ss_out\\[3:0\\]
xxx0 1110 xx01 1101 x011 1011 0111 0111 1111 1111 \\[no peripheral selected\\]
else ss\\[3:0\\]
directly drives n_ss_out\\[3:0\\]"]
pub type PeriphCsLinesFldW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WR_PROT_FLASH_FLD` reader - 14:14\\]
Write Protect Flash Pin: Set to drive the Write Protect pin of the FLASH device. This is resynchronized to the generated memory clock as necessary."]
pub type WrProtFlashFldR = crate::BitReader;
#[doc = "Field `WR_PROT_FLASH_FLD` writer - 14:14\\]
Write Protect Flash Pin: Set to drive the Write Protect pin of the FLASH device. This is resynchronized to the generated memory clock as necessary."]
pub type WrProtFlashFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENB_DMA_IF_FLD` reader - 15:15\\]
Enable DMA Peripheral Interface: Set to 1 to enable the DMA handshaking logic. When enabled the controller will trigger DMA transfer requests via the DMA peripheral interface. Set to 0 to disable"]
pub type EnbDmaIfFldR = crate::BitReader;
#[doc = "Field `ENB_DMA_IF_FLD` writer - 15:15\\]
Enable DMA Peripheral Interface: Set to 1 to enable the DMA handshaking logic. When enabled the controller will trigger DMA transfer requests via the DMA peripheral interface. Set to 0 to disable"]
pub type EnbDmaIfFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENB_AHB_ADDR_REMAP_FLD` reader - 16:16\\]
Enable AHB Address Re-mapping: \\[Direct Access Mode Only\\]
When set to 1, the incoming AHB address will be adapted and sent to the FLASH device as \\[address + N\\], where N is the value stored in the remap address register."]
pub type EnbAhbAddrRemapFldR = crate::BitReader;
#[doc = "Field `ENB_AHB_ADDR_REMAP_FLD` writer - 16:16\\]
Enable AHB Address Re-mapping: \\[Direct Access Mode Only\\]
When set to 1, the incoming AHB address will be adapted and sent to the FLASH device as \\[address + N\\], where N is the value stored in the remap address register."]
pub type EnbAhbAddrRemapFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENTER_XIP_MODE_FLD` reader - 17:17\\]
Enter XIP Mode on next READ: Value=0 : If XIP is enabled, then setting to 0 will cause the controller to exit XIP mode on the next READ instruction. Value=1 : If XIP is disabled, then setting to ?1? will inform the controller that the device is ready to enter XIP on the next READ instruction. The controller will therefore send the appropriate command sequence, including mode bits to cause the device to enter XIP mode. Use this register after the controller has ensured the FLASH device has been configured to be ready to enter XIP mode. Note : To exit XIP mode, this bit should be set to 0. This will take effect in the attached device only AFTER the next READ instruction is executed. Software should therefore ensure that at least one READ instruction is requested after resetting this bit before it can be sure XIP mode in the device is exited."]
pub type EnterXipModeFldR = crate::BitReader;
#[doc = "Field `ENTER_XIP_MODE_FLD` writer - 17:17\\]
Enter XIP Mode on next READ: Value=0 : If XIP is enabled, then setting to 0 will cause the controller to exit XIP mode on the next READ instruction. Value=1 : If XIP is disabled, then setting to ?1? will inform the controller that the device is ready to enter XIP on the next READ instruction. The controller will therefore send the appropriate command sequence, including mode bits to cause the device to enter XIP mode. Use this register after the controller has ensured the FLASH device has been configured to be ready to enter XIP mode. Note : To exit XIP mode, this bit should be set to 0. This will take effect in the attached device only AFTER the next READ instruction is executed. Software should therefore ensure that at least one READ instruction is requested after resetting this bit before it can be sure XIP mode in the device is exited."]
pub type EnterXipModeFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENTER_XIP_MODE_IMM_FLD` reader - 18:18\\]
Enter XIP Mode immediately: Value=0 : If XIP is enabled, then setting to 0 will cause the controller to exit XIP mode on the next READ instruction. Value=1 : Operate the device in XIP mode immediately Use this register when the external device wakes up in XIP mode \\[as per the contents of its non- volatile configuration register\\]. The controller will assume the next READ instruction will be passed to the device as an XIP instruction, and therefore will not require the READ opcode to be transferred. Note: To exit XIP mode, this bit should be set to 0. This will take effect in the attached device only after the next READ instruction is executed. Software therefore should ensure that at least one READ instruction is requested after resetting this bit in order to be sure that XIP mode is exited."]
pub type EnterXipModeImmFldR = crate::BitReader;
#[doc = "Field `ENTER_XIP_MODE_IMM_FLD` writer - 18:18\\]
Enter XIP Mode immediately: Value=0 : If XIP is enabled, then setting to 0 will cause the controller to exit XIP mode on the next READ instruction. Value=1 : Operate the device in XIP mode immediately Use this register when the external device wakes up in XIP mode \\[as per the contents of its non- volatile configuration register\\]. The controller will assume the next READ instruction will be passed to the device as an XIP instruction, and therefore will not require the READ opcode to be transferred. Note: To exit XIP mode, this bit should be set to 0. This will take effect in the attached device only after the next READ instruction is executed. Software therefore should ensure that at least one READ instruction is requested after resetting this bit in order to be sure that XIP mode is exited."]
pub type EnterXipModeImmFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTR_BAUD_DIV_FLD` reader - 22:19\\]
Master Mode Baud Rate Divisor: SPI baud rate = \\[master reference clock\\]
baud_rate_divisor"]
pub type MstrBaudDivFldR = crate::FieldReader;
#[doc = "Field `MSTR_BAUD_DIV_FLD` writer - 22:19\\]
Master Mode Baud Rate Divisor: SPI baud rate = \\[master reference clock\\]
baud_rate_divisor"]
pub type MstrBaudDivFldW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ENABLE_AHB_DECODER_FLD` reader - 23:23\\]
Enable AHB Decoder: Value=0 : Active slave is selected based on Peripheral Chip Select Lines \\[bits \\[13:10\\]\\]. Value=1 Active slave is selected based on actual AHB address \\[the partition for each device is calculated with respect to bits \\[28:21\\]
of Device Size Configuration Register\\]"]
pub type EnableAhbDecoderFldR = crate::BitReader;
#[doc = "Field `ENABLE_AHB_DECODER_FLD` writer - 23:23\\]
Enable AHB Decoder: Value=0 : Active slave is selected based on Peripheral Chip Select Lines \\[bits \\[13:10\\]\\]. Value=1 Active slave is selected based on actual AHB address \\[the partition for each device is calculated with respect to bits \\[28:21\\]
of Device Size Configuration Register\\]"]
pub type EnableAhbDecoderFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_DTR_PROTOCOL_FLD` reader - 24:24\\]
Enable DTR Protocol: This bit should be set if device is configured to work in DTR protocol."]
pub type EnableDtrProtocolFldR = crate::BitReader;
#[doc = "Field `ENABLE_DTR_PROTOCOL_FLD` writer - 24:24\\]
Enable DTR Protocol: This bit should be set if device is configured to work in DTR protocol."]
pub type EnableDtrProtocolFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIPELINE_PHY_FLD` reader - 25:25\\]
Pipeline PHY Mode enable: This bit is relevant only for configuration with PHY Module. It should be asserted to 1 between consecutive PHY pipeline reads transfers and de-asserted to 0 otherwise."]
pub type PipelinePhyFldR = crate::BitReader;
#[doc = "Field `PIPELINE_PHY_FLD` writer - 25:25\\]
Pipeline PHY Mode enable: This bit is relevant only for configuration with PHY Module. It should be asserted to 1 between consecutive PHY pipeline reads transfers and de-asserted to 0 otherwise."]
pub type PipelinePhyFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_ENABLE_FLD` reader - 29:29\\]
CRC enable bit This bit is to be set in case the target Flash Device supports CRC \\[Macronix MX25\\]. It is applicable for Octal DDR Protocol only so should be set back to low if the device is configured to work in another SPI Mode."]
pub type CrcEnableFldR = crate::BitReader;
#[doc = "Field `CRC_ENABLE_FLD` writer - 29:29\\]
CRC enable bit This bit is to be set in case the target Flash Device supports CRC \\[Macronix MX25\\]. It is applicable for Octal DDR Protocol only so should be set back to low if the device is configured to work in another SPI Mode."]
pub type CrcEnableFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUAL_BYTE_OPCODE_EN_FLD` reader - 30:30\\]
Dual-byte Opcode Mode enable bit This bit is to be set in case the target Flash Device supports dual byte opcode \\[i.e. Macronix MX25\\]. It is applicable for Octal I/O Mode or Protocol only so should be set back to low if the device is configured to work in another SPI Mode. If enabled, the supplementing bytes are taken from Opcode Extension Register \\[Lower\\]
and from Opcode Extension Register \\[Upper\\]."]
pub type DualByteOpcodeEnFldR = crate::BitReader;
#[doc = "Field `DUAL_BYTE_OPCODE_EN_FLD` writer - 30:30\\]
Dual-byte Opcode Mode enable bit This bit is to be set in case the target Flash Device supports dual byte opcode \\[i.e. Macronix MX25\\]. It is applicable for Octal I/O Mode or Protocol only so should be set back to low if the device is configured to work in another SPI Mode. If enabled, the supplementing bytes are taken from Opcode Extension Register \\[Lower\\]
and from Opcode Extension Register \\[Upper\\]."]
pub type DualByteOpcodeEnFldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLE_FLD` reader - 31:31\\]
Serial interface and low level SPI pipeline is IDLE: This is a STATUS read-only bit. Note this is a retimed signal, so there will be some inherent delay on the generation of this status signal."]
pub type IdleFldR = crate::BitReader;
#[doc = "Field `IDLE_FLD` writer - 31:31\\]
Serial interface and low level SPI pipeline is IDLE: This is a STATUS read-only bit. Note this is a retimed signal, so there will be some inherent delay on the generation of this status signal."]
pub type IdleFldW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Octal-SPI Enable: 0 : disable the Octal-SPI, once current transfer of the data word \\[FF_W\\]
is complete. 1 : enable the Octal-SPI, when spi_enable = 0, all output enables are inactive and all pins are set to input mode."]
    #[inline(always)]
    pub fn enb_spi_fld(&self) -> EnbSpiFldR {
        EnbSpiFldR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clock polarity outside SPI word: 0 : the SPI clock is quiescent low 1 : the SPI clock is quiescent high"]
    #[inline(always)]
    pub fn sel_clk_pol_fld(&self) -> SelClkPolFldR {
        SelClkPolFldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Select Clock Phase: Selects whether the clock is in an active or inactive phase outside the SPI word. 0 : the SPI clock is active outside the word 1 : the SPI clock is inactive outside the word"]
    #[inline(always)]
    pub fn sel_clk_phase_fld(&self) -> SelClkPhaseFldR {
        SelClkPhaseFldR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
PHY mode enable: When enabled, the controller is informed that PHY Module is to be used for handling SPI transfers. This bit is relevant only for configuration with PHY Module."]
    #[inline(always)]
    pub fn phy_mode_enable_fld(&self) -> PhyModeEnableFldR {
        PhyModeEnableFldR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Set to drive the HOLD pin of the FLASH device and reset for de-activation of the HOLD pin feature"]
    #[inline(always)]
    pub fn hold_pin_fld(&self) -> HoldPinFldR {
        HoldPinFldR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Set to drive the RESET pin of the FLASH device and reset for de-activation of the RESET pin feature"]
    #[inline(always)]
    pub fn reset_pin_fld(&self) -> ResetPinFldR {
        ResetPinFldR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
RESET pin configuration: 0 = RESET feature on DQ3 pin of the device 1 = RESET feature on dedicated pin of the device \\[controlling of 5th bit influences on reset_out output\\]"]
    #[inline(always)]
    pub fn reset_cfg_fld(&self) -> ResetCfgFldR {
        ResetCfgFldR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable Direct Access Controller: 0 : disable the Direct Access Controller once current transfer of the data word \\[FF_W\\]
is complete. 1 : enable the Direct Access Controller When the Direct Access Controller and Indirect Access Controller are both disabled, all AHB requested are completed with an error response."]
    #[inline(always)]
    pub fn enb_dir_acc_ctlr_fld(&self) -> EnbDirAccCtlrFldR {
        EnbDirAccCtlrFldR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Legacy IP Mode Enable: 0 : Use Direct Access Controller/Indirect Access Controller 1 : legacy Mode is enabled. In this mode, any write to the controller via the AHB interface is serialized and sent to the FLASH device. Any valid AHB read will pop the internal RX-FIFO, retrieving data that was forwarded by the external FLASH device on the SPI lines,4,2 or 1 byte transfers are permitted and controlled via the HSIZE input."]
    #[inline(always)]
    pub fn enb_legacy_ip_mode_fld(&self) -> EnbLegacyIpModeFldR {
        EnbLegacyIpModeFldR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Peripheral select decode: 0 : only 1 of 4 selects n_ss_out\\[3:0\\]
is active 1 : allow external 4-to-16 decode \\[n_ss_out = ss\\]"]
    #[inline(always)]
    pub fn periph_sel_dec_fld(&self) -> PeriphSelDecFldR {
        PeriphSelDecFldR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - 13:10\\]
Peripheral Chip Select Lines: Peripheral chip select lines If pdec = 0, ss\\[3:0\\]
are output thus: ss\\[3:0\\]
n_ss_out\\[3:0\\]
xxx0 1110 xx01 1101 x011 1011 0111 0111 1111 1111 \\[no peripheral selected\\]
else ss\\[3:0\\]
directly drives n_ss_out\\[3:0\\]"]
    #[inline(always)]
    pub fn periph_cs_lines_fld(&self) -> PeriphCsLinesFldR {
        PeriphCsLinesFldR::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - 14:14\\]
Write Protect Flash Pin: Set to drive the Write Protect pin of the FLASH device. This is resynchronized to the generated memory clock as necessary."]
    #[inline(always)]
    pub fn wr_prot_flash_fld(&self) -> WrProtFlashFldR {
        WrProtFlashFldR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Enable DMA Peripheral Interface: Set to 1 to enable the DMA handshaking logic. When enabled the controller will trigger DMA transfer requests via the DMA peripheral interface. Set to 0 to disable"]
    #[inline(always)]
    pub fn enb_dma_if_fld(&self) -> EnbDmaIfFldR {
        EnbDmaIfFldR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable AHB Address Re-mapping: \\[Direct Access Mode Only\\]
When set to 1, the incoming AHB address will be adapted and sent to the FLASH device as \\[address + N\\], where N is the value stored in the remap address register."]
    #[inline(always)]
    pub fn enb_ahb_addr_remap_fld(&self) -> EnbAhbAddrRemapFldR {
        EnbAhbAddrRemapFldR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Enter XIP Mode on next READ: Value=0 : If XIP is enabled, then setting to 0 will cause the controller to exit XIP mode on the next READ instruction. Value=1 : If XIP is disabled, then setting to ?1? will inform the controller that the device is ready to enter XIP on the next READ instruction. The controller will therefore send the appropriate command sequence, including mode bits to cause the device to enter XIP mode. Use this register after the controller has ensured the FLASH device has been configured to be ready to enter XIP mode. Note : To exit XIP mode, this bit should be set to 0. This will take effect in the attached device only AFTER the next READ instruction is executed. Software should therefore ensure that at least one READ instruction is requested after resetting this bit before it can be sure XIP mode in the device is exited."]
    #[inline(always)]
    pub fn enter_xip_mode_fld(&self) -> EnterXipModeFldR {
        EnterXipModeFldR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Enter XIP Mode immediately: Value=0 : If XIP is enabled, then setting to 0 will cause the controller to exit XIP mode on the next READ instruction. Value=1 : Operate the device in XIP mode immediately Use this register when the external device wakes up in XIP mode \\[as per the contents of its non- volatile configuration register\\]. The controller will assume the next READ instruction will be passed to the device as an XIP instruction, and therefore will not require the READ opcode to be transferred. Note: To exit XIP mode, this bit should be set to 0. This will take effect in the attached device only after the next READ instruction is executed. Software therefore should ensure that at least one READ instruction is requested after resetting this bit in order to be sure that XIP mode is exited."]
    #[inline(always)]
    pub fn enter_xip_mode_imm_fld(&self) -> EnterXipModeImmFldR {
        EnterXipModeImmFldR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - 22:19\\]
Master Mode Baud Rate Divisor: SPI baud rate = \\[master reference clock\\]
baud_rate_divisor"]
    #[inline(always)]
    pub fn mstr_baud_div_fld(&self) -> MstrBaudDivFldR {
        MstrBaudDivFldR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
Enable AHB Decoder: Value=0 : Active slave is selected based on Peripheral Chip Select Lines \\[bits \\[13:10\\]\\]. Value=1 Active slave is selected based on actual AHB address \\[the partition for each device is calculated with respect to bits \\[28:21\\]
of Device Size Configuration Register\\]"]
    #[inline(always)]
    pub fn enable_ahb_decoder_fld(&self) -> EnableAhbDecoderFldR {
        EnableAhbDecoderFldR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable DTR Protocol: This bit should be set if device is configured to work in DTR protocol."]
    #[inline(always)]
    pub fn enable_dtr_protocol_fld(&self) -> EnableDtrProtocolFldR {
        EnableDtrProtocolFldR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
Pipeline PHY Mode enable: This bit is relevant only for configuration with PHY Module. It should be asserted to 1 between consecutive PHY pipeline reads transfers and de-asserted to 0 otherwise."]
    #[inline(always)]
    pub fn pipeline_phy_fld(&self) -> PipelinePhyFldR {
        PipelinePhyFldR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - 29:29\\]
CRC enable bit This bit is to be set in case the target Flash Device supports CRC \\[Macronix MX25\\]. It is applicable for Octal DDR Protocol only so should be set back to low if the device is configured to work in another SPI Mode."]
    #[inline(always)]
    pub fn crc_enable_fld(&self) -> CrcEnableFldR {
        CrcEnableFldR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Dual-byte Opcode Mode enable bit This bit is to be set in case the target Flash Device supports dual byte opcode \\[i.e. Macronix MX25\\]. It is applicable for Octal I/O Mode or Protocol only so should be set back to low if the device is configured to work in another SPI Mode. If enabled, the supplementing bytes are taken from Opcode Extension Register \\[Lower\\]
and from Opcode Extension Register \\[Upper\\]."]
    #[inline(always)]
    pub fn dual_byte_opcode_en_fld(&self) -> DualByteOpcodeEnFldR {
        DualByteOpcodeEnFldR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Serial interface and low level SPI pipeline is IDLE: This is a STATUS read-only bit. Note this is a retimed signal, so there will be some inherent delay on the generation of this status signal."]
    #[inline(always)]
    pub fn idle_fld(&self) -> IdleFldR {
        IdleFldR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Octal-SPI Enable: 0 : disable the Octal-SPI, once current transfer of the data word \\[FF_W\\]
is complete. 1 : enable the Octal-SPI, when spi_enable = 0, all output enables are inactive and all pins are set to input mode."]
    #[inline(always)]
    #[must_use]
    pub fn enb_spi_fld(
        &mut self,
    ) -> EnbSpiFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec> {
        EnbSpiFldW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clock polarity outside SPI word: 0 : the SPI clock is quiescent low 1 : the SPI clock is quiescent high"]
    #[inline(always)]
    #[must_use]
    pub fn sel_clk_pol_fld(
        &mut self,
    ) -> SelClkPolFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec>
    {
        SelClkPolFldW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Select Clock Phase: Selects whether the clock is in an active or inactive phase outside the SPI word. 0 : the SPI clock is active outside the word 1 : the SPI clock is inactive outside the word"]
    #[inline(always)]
    #[must_use]
    pub fn sel_clk_phase_fld(
        &mut self,
    ) -> SelClkPhaseFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec>
    {
        SelClkPhaseFldW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
PHY mode enable: When enabled, the controller is informed that PHY Module is to be used for handling SPI transfers. This bit is relevant only for configuration with PHY Module."]
    #[inline(always)]
    #[must_use]
    pub fn phy_mode_enable_fld(
        &mut self,
    ) -> PhyModeEnableFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec>
    {
        PhyModeEnableFldW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Set to drive the HOLD pin of the FLASH device and reset for de-activation of the HOLD pin feature"]
    #[inline(always)]
    #[must_use]
    pub fn hold_pin_fld(
        &mut self,
    ) -> HoldPinFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec> {
        HoldPinFldW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Set to drive the RESET pin of the FLASH device and reset for de-activation of the RESET pin feature"]
    #[inline(always)]
    #[must_use]
    pub fn reset_pin_fld(
        &mut self,
    ) -> ResetPinFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec> {
        ResetPinFldW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
RESET pin configuration: 0 = RESET feature on DQ3 pin of the device 1 = RESET feature on dedicated pin of the device \\[controlling of 5th bit influences on reset_out output\\]"]
    #[inline(always)]
    #[must_use]
    pub fn reset_cfg_fld(
        &mut self,
    ) -> ResetCfgFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec> {
        ResetCfgFldW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Enable Direct Access Controller: 0 : disable the Direct Access Controller once current transfer of the data word \\[FF_W\\]
is complete. 1 : enable the Direct Access Controller When the Direct Access Controller and Indirect Access Controller are both disabled, all AHB requested are completed with an error response."]
    #[inline(always)]
    #[must_use]
    pub fn enb_dir_acc_ctlr_fld(
        &mut self,
    ) -> EnbDirAccCtlrFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec>
    {
        EnbDirAccCtlrFldW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
Legacy IP Mode Enable: 0 : Use Direct Access Controller/Indirect Access Controller 1 : legacy Mode is enabled. In this mode, any write to the controller via the AHB interface is serialized and sent to the FLASH device. Any valid AHB read will pop the internal RX-FIFO, retrieving data that was forwarded by the external FLASH device on the SPI lines,4,2 or 1 byte transfers are permitted and controlled via the HSIZE input."]
    #[inline(always)]
    #[must_use]
    pub fn enb_legacy_ip_mode_fld(
        &mut self,
    ) -> EnbLegacyIpModeFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec>
    {
        EnbLegacyIpModeFldW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Peripheral select decode: 0 : only 1 of 4 selects n_ss_out\\[3:0\\]
is active 1 : allow external 4-to-16 decode \\[n_ss_out = ss\\]"]
    #[inline(always)]
    #[must_use]
    pub fn periph_sel_dec_fld(
        &mut self,
    ) -> PeriphSelDecFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec>
    {
        PeriphSelDecFldW::new(self, 9)
    }
    #[doc = "Bits 10:13 - 13:10\\]
Peripheral Chip Select Lines: Peripheral chip select lines If pdec = 0, ss\\[3:0\\]
are output thus: ss\\[3:0\\]
n_ss_out\\[3:0\\]
xxx0 1110 xx01 1101 x011 1011 0111 0111 1111 1111 \\[no peripheral selected\\]
else ss\\[3:0\\]
directly drives n_ss_out\\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn periph_cs_lines_fld(
        &mut self,
    ) -> PeriphCsLinesFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec>
    {
        PeriphCsLinesFldW::new(self, 10)
    }
    #[doc = "Bit 14 - 14:14\\]
Write Protect Flash Pin: Set to drive the Write Protect pin of the FLASH device. This is resynchronized to the generated memory clock as necessary."]
    #[inline(always)]
    #[must_use]
    pub fn wr_prot_flash_fld(
        &mut self,
    ) -> WrProtFlashFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec>
    {
        WrProtFlashFldW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
Enable DMA Peripheral Interface: Set to 1 to enable the DMA handshaking logic. When enabled the controller will trigger DMA transfer requests via the DMA peripheral interface. Set to 0 to disable"]
    #[inline(always)]
    #[must_use]
    pub fn enb_dma_if_fld(
        &mut self,
    ) -> EnbDmaIfFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec> {
        EnbDmaIfFldW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
Enable AHB Address Re-mapping: \\[Direct Access Mode Only\\]
When set to 1, the incoming AHB address will be adapted and sent to the FLASH device as \\[address + N\\], where N is the value stored in the remap address register."]
    #[inline(always)]
    #[must_use]
    pub fn enb_ahb_addr_remap_fld(
        &mut self,
    ) -> EnbAhbAddrRemapFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec>
    {
        EnbAhbAddrRemapFldW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Enter XIP Mode on next READ: Value=0 : If XIP is enabled, then setting to 0 will cause the controller to exit XIP mode on the next READ instruction. Value=1 : If XIP is disabled, then setting to ?1? will inform the controller that the device is ready to enter XIP on the next READ instruction. The controller will therefore send the appropriate command sequence, including mode bits to cause the device to enter XIP mode. Use this register after the controller has ensured the FLASH device has been configured to be ready to enter XIP mode. Note : To exit XIP mode, this bit should be set to 0. This will take effect in the attached device only AFTER the next READ instruction is executed. Software should therefore ensure that at least one READ instruction is requested after resetting this bit before it can be sure XIP mode in the device is exited."]
    #[inline(always)]
    #[must_use]
    pub fn enter_xip_mode_fld(
        &mut self,
    ) -> EnterXipModeFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec>
    {
        EnterXipModeFldW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Enter XIP Mode immediately: Value=0 : If XIP is enabled, then setting to 0 will cause the controller to exit XIP mode on the next READ instruction. Value=1 : Operate the device in XIP mode immediately Use this register when the external device wakes up in XIP mode \\[as per the contents of its non- volatile configuration register\\]. The controller will assume the next READ instruction will be passed to the device as an XIP instruction, and therefore will not require the READ opcode to be transferred. Note: To exit XIP mode, this bit should be set to 0. This will take effect in the attached device only after the next READ instruction is executed. Software therefore should ensure that at least one READ instruction is requested after resetting this bit in order to be sure that XIP mode is exited."]
    #[inline(always)]
    #[must_use]
    pub fn enter_xip_mode_imm_fld(
        &mut self,
    ) -> EnterXipModeImmFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec>
    {
        EnterXipModeImmFldW::new(self, 18)
    }
    #[doc = "Bits 19:22 - 22:19\\]
Master Mode Baud Rate Divisor: SPI baud rate = \\[master reference clock\\]
baud_rate_divisor"]
    #[inline(always)]
    #[must_use]
    pub fn mstr_baud_div_fld(
        &mut self,
    ) -> MstrBaudDivFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec>
    {
        MstrBaudDivFldW::new(self, 19)
    }
    #[doc = "Bit 23 - 23:23\\]
Enable AHB Decoder: Value=0 : Active slave is selected based on Peripheral Chip Select Lines \\[bits \\[13:10\\]\\]. Value=1 Active slave is selected based on actual AHB address \\[the partition for each device is calculated with respect to bits \\[28:21\\]
of Device Size Configuration Register\\]"]
    #[inline(always)]
    #[must_use]
    pub fn enable_ahb_decoder_fld(
        &mut self,
    ) -> EnableAhbDecoderFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec>
    {
        EnableAhbDecoderFldW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
Enable DTR Protocol: This bit should be set if device is configured to work in DTR protocol."]
    #[inline(always)]
    #[must_use]
    pub fn enable_dtr_protocol_fld(
        &mut self,
    ) -> EnableDtrProtocolFldW<
        Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec,
    > {
        EnableDtrProtocolFldW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
Pipeline PHY Mode enable: This bit is relevant only for configuration with PHY Module. It should be asserted to 1 between consecutive PHY pipeline reads transfers and de-asserted to 0 otherwise."]
    #[inline(always)]
    #[must_use]
    pub fn pipeline_phy_fld(
        &mut self,
    ) -> PipelinePhyFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec>
    {
        PipelinePhyFldW::new(self, 25)
    }
    #[doc = "Bit 29 - 29:29\\]
CRC enable bit This bit is to be set in case the target Flash Device supports CRC \\[Macronix MX25\\]. It is applicable for Octal DDR Protocol only so should be set back to low if the device is configured to work in another SPI Mode."]
    #[inline(always)]
    #[must_use]
    pub fn crc_enable_fld(
        &mut self,
    ) -> CrcEnableFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec>
    {
        CrcEnableFldW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Dual-byte Opcode Mode enable bit This bit is to be set in case the target Flash Device supports dual byte opcode \\[i.e. Macronix MX25\\]. It is applicable for Octal I/O Mode or Protocol only so should be set back to low if the device is configured to work in another SPI Mode. If enabled, the supplementing bytes are taken from Opcode Extension Register \\[Lower\\]
and from Opcode Extension Register \\[Upper\\]."]
    #[inline(always)]
    #[must_use]
    pub fn dual_byte_opcode_en_fld(
        &mut self,
    ) -> DualByteOpcodeEnFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec>
    {
        DualByteOpcodeEnFldW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Serial interface and low level SPI pipeline is IDLE: This is a STATUS read-only bit. Note this is a retimed signal, so there will be some inherent delay on the generation of this status signal."]
    #[inline(always)]
    #[must_use]
    pub fn idle_fld(
        &mut self,
    ) -> IdleFldW<Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec> {
        IdleFldW::new(self, 31)
    }
}
#[doc = "Octal-SPI Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_config_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_config_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec;
impl crate::RegisterSpec
    for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec
{
    type Ux = u32;
}
#[doc = "`read()` method returns [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_config_reg::R`](R) reader structure"]
impl crate::Readable for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ospi0__ospi_cfg_vbusp__vbp2apb_wrap__ospi_cfg_vbp__ospi_flash_apb_regs_config_reg::W`](W) writer structure"]
impl crate::Writable for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSPI0__OSPI_CFG_VBUSP__VBP2APB_WRAP__OSPI_CFG_VBP__OSPI_FLASH_APB_REGS_config_reg to value 0x80a8_0081"]
impl crate::Resettable for Ospi0_OspiCfgVbusp_Vbp2apbWrap_OspiCfgVbp_OspiFlashApbRegsConfigRegSpec {
    const RESET_VALUE: u32 = 0x80a8_0081;
}
