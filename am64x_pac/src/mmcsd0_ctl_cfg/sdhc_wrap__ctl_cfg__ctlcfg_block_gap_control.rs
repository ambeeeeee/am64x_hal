#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_block_gap_control` reader"]
pub type R = crate::R<SdhcWrap_CtlCfg_CtlcfgBlockGapControlSpec>;
#[doc = "Register `SDHC_WRAP__CTL_CFG__CTLCFG_block_gap_control` writer"]
pub type W = crate::W<SdhcWrap_CtlCfg_CtlcfgBlockGapControlSpec>;
#[doc = "Field `STOP_AT_BLK_GAP` reader - 0:0\\]
This bit is used to stop executing a transaction at the next block gap for non- DMA,SDMA and ADMA transfers. Until the transfer complete is set to 1, indicating a transfer completion the HD shall leave this bit set to 1. Clearing both the Stop At Block Gap Request and Continue Request shall not cause the transaction to restart. Read Wait is used to stop the read transaction at the block gap. The HC shall honour Stop At Block Gap Request for write transfers, but for read transfers it requires that the SD card support Read Wait. Therefore the HD shall not set this bit during read transfers unless the SD card supports Read Wait and has set Read Wait Control to 1. In case of write transfers in which the HD writes data to the Buffer Data Port register, the HD shall set this bit after all block data is written. If this bit is set to 1, the HD shall not write data to Buffer data port register. This bit affects Read Transfer Active, Write Transfer Active, DAT line active and Command Inhibit \\[DAT\\]
in the Present State register. In case of UHS-II, a transaction can be stopped at the boundary of DATA Burst \\[Flow Control basis\\]. Host Control-ler waits for sending Flow Control MSG until Continue Request is set to 1. '0' Transfer '1' Stop"]
pub type StopAtBlkGapR = crate::BitReader;
#[doc = "Field `STOP_AT_BLK_GAP` writer - 0:0\\]
This bit is used to stop executing a transaction at the next block gap for non- DMA,SDMA and ADMA transfers. Until the transfer complete is set to 1, indicating a transfer completion the HD shall leave this bit set to 1. Clearing both the Stop At Block Gap Request and Continue Request shall not cause the transaction to restart. Read Wait is used to stop the read transaction at the block gap. The HC shall honour Stop At Block Gap Request for write transfers, but for read transfers it requires that the SD card support Read Wait. Therefore the HD shall not set this bit during read transfers unless the SD card supports Read Wait and has set Read Wait Control to 1. In case of write transfers in which the HD writes data to the Buffer Data Port register, the HD shall set this bit after all block data is written. If this bit is set to 1, the HD shall not write data to Buffer data port register. This bit affects Read Transfer Active, Write Transfer Active, DAT line active and Command Inhibit \\[DAT\\]
in the Present State register. In case of UHS-II, a transaction can be stopped at the boundary of DATA Burst \\[Flow Control basis\\]. Host Control-ler waits for sending Flow Control MSG until Continue Request is set to 1. '0' Transfer '1' Stop"]
pub type StopAtBlkGapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTINUE` reader - 1:1\\]
This bit is used to restart a transaction which was stopped using the Stop At Block Gap Request. To cancel stop at the block gap, set Stop At block Gap Request to 0 and set this bit to restart the transfer. The Host Controller automatically clears this bit when the transaction re-starts. If Stop At Block Gap Request is set to 1, any write to this bit is ignored. In SD mode, this bit is cleared in either of the following cases: \\[1\\]
In the case of a read transaction, the DAT Line Active changes from 0 to 1 as a read transaction restarts. \\[2\\]
In the case of a write transaction, the Write transfer active changes from 0 to 1 as the write transaction restarts. Therefore it is not necessary for Host driver to set this bit to 0. If Stop At Block Gap Request is set to 1, any write to this bit is ignored. '0' Ignore '1' Restart"]
pub type ContinueR = crate::BitReader;
#[doc = "Field `CONTINUE` writer - 1:1\\]
This bit is used to restart a transaction which was stopped using the Stop At Block Gap Request. To cancel stop at the block gap, set Stop At block Gap Request to 0 and set this bit to restart the transfer. The Host Controller automatically clears this bit when the transaction re-starts. If Stop At Block Gap Request is set to 1, any write to this bit is ignored. In SD mode, this bit is cleared in either of the following cases: \\[1\\]
In the case of a read transaction, the DAT Line Active changes from 0 to 1 as a read transaction restarts. \\[2\\]
In the case of a write transaction, the Write transfer active changes from 0 to 1 as the write transaction restarts. Therefore it is not necessary for Host driver to set this bit to 0. If Stop At Block Gap Request is set to 1, any write to this bit is ignored. '0' Ignore '1' Restart"]
pub type ContinueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDWAIT_CTRL` reader - 2:2\\]
The read wait function is optional for SDIO cards. If the card supports read wait, set this bit to enable use of the read wait protocol to stop read data using DAT\\[2\\]
line. Otherwise the HC has to stop the SD clock to hold read data, which restricts commands generation. When the HD detects an SD card insertion, it shall set this bit according to the CCCR of the SDIO card. If the card does not support read wait, this bit shall never be set to 1 otherwise DAT line conflict may occur. If this bit is set to 0, Suspend / Resume cannot be supported. In UHS-II mode, Read Wait is disabled and DAT\\[2\\]
line is used for Interrupt Signal from UHS-II Card."]
pub type RdwaitCtrlR = crate::BitReader;
#[doc = "Field `RDWAIT_CTRL` writer - 2:2\\]
The read wait function is optional for SDIO cards. If the card supports read wait, set this bit to enable use of the read wait protocol to stop read data using DAT\\[2\\]
line. Otherwise the HC has to stop the SD clock to hold read data, which restricts commands generation. When the HD detects an SD card insertion, it shall set this bit according to the CCCR of the SDIO card. If the card does not support read wait, this bit shall never be set to 1 otherwise DAT line conflict may occur. If this bit is set to 0, Suspend / Resume cannot be supported. In UHS-II mode, Read Wait is disabled and DAT\\[2\\]
line is used for Interrupt Signal from UHS-II Card."]
pub type RdwaitCtrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTRPT_AT_BLK_GAP` reader - 3:3\\]
This bit is valid only in 4-bit mode of the SDIO card and selects a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. If the SD card cannot signal an interrupt during a multiple block transfer, this bit should be set to 0. When the HD detects an SD card insertion, it shall set this bit according to the CCCR of the SDIO card."]
pub type IntrptAtBlkGapR = crate::BitReader;
#[doc = "Field `INTRPT_AT_BLK_GAP` writer - 3:3\\]
This bit is valid only in 4-bit mode of the SDIO card and selects a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. If the SD card cannot signal an interrupt during a multiple block transfer, this bit should be set to 0. When the HD detects an SD card insertion, it shall set this bit according to the CCCR of the SDIO card."]
pub type IntrptAtBlkGapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MODE` reader - 4:4\\]
SPI mode enable bit."]
pub type SpiModeR = crate::BitReader;
#[doc = "Field `SPI_MODE` writer - 4:4\\]
SPI mode enable bit."]
pub type SpiModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_ENABLE` reader - 5:5\\]
To start boot code access."]
pub type BootEnableR = crate::BitReader;
#[doc = "Field `BOOT_ENABLE` writer - 5:5\\]
To start boot code access."]
pub type BootEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALT_BOOT_MODE` reader - 6:6\\]
To start boot code access in alternative mode."]
pub type AltBootModeR = crate::BitReader;
#[doc = "Field `ALT_BOOT_MODE` writer - 6:6\\]
To start boot code access in alternative mode."]
pub type AltBootModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOT_ACK_ENA` reader - 7:7\\]
To check for the boot acknowledge in boot operation."]
pub type BootAckEnaR = crate::BitReader;
#[doc = "Field `BOOT_ACK_ENA` writer - 7:7\\]
To check for the boot acknowledge in boot operation."]
pub type BootAckEnaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit is used to stop executing a transaction at the next block gap for non- DMA,SDMA and ADMA transfers. Until the transfer complete is set to 1, indicating a transfer completion the HD shall leave this bit set to 1. Clearing both the Stop At Block Gap Request and Continue Request shall not cause the transaction to restart. Read Wait is used to stop the read transaction at the block gap. The HC shall honour Stop At Block Gap Request for write transfers, but for read transfers it requires that the SD card support Read Wait. Therefore the HD shall not set this bit during read transfers unless the SD card supports Read Wait and has set Read Wait Control to 1. In case of write transfers in which the HD writes data to the Buffer Data Port register, the HD shall set this bit after all block data is written. If this bit is set to 1, the HD shall not write data to Buffer data port register. This bit affects Read Transfer Active, Write Transfer Active, DAT line active and Command Inhibit \\[DAT\\]
in the Present State register. In case of UHS-II, a transaction can be stopped at the boundary of DATA Burst \\[Flow Control basis\\]. Host Control-ler waits for sending Flow Control MSG until Continue Request is set to 1. '0' Transfer '1' Stop"]
    #[inline(always)]
    pub fn stop_at_blk_gap(&self) -> StopAtBlkGapR {
        StopAtBlkGapR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit is used to restart a transaction which was stopped using the Stop At Block Gap Request. To cancel stop at the block gap, set Stop At block Gap Request to 0 and set this bit to restart the transfer. The Host Controller automatically clears this bit when the transaction re-starts. If Stop At Block Gap Request is set to 1, any write to this bit is ignored. In SD mode, this bit is cleared in either of the following cases: \\[1\\]
In the case of a read transaction, the DAT Line Active changes from 0 to 1 as a read transaction restarts. \\[2\\]
In the case of a write transaction, the Write transfer active changes from 0 to 1 as the write transaction restarts. Therefore it is not necessary for Host driver to set this bit to 0. If Stop At Block Gap Request is set to 1, any write to this bit is ignored. '0' Ignore '1' Restart"]
    #[inline(always)]
    pub fn continue_(&self) -> ContinueR {
        ContinueR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
The read wait function is optional for SDIO cards. If the card supports read wait, set this bit to enable use of the read wait protocol to stop read data using DAT\\[2\\]
line. Otherwise the HC has to stop the SD clock to hold read data, which restricts commands generation. When the HD detects an SD card insertion, it shall set this bit according to the CCCR of the SDIO card. If the card does not support read wait, this bit shall never be set to 1 otherwise DAT line conflict may occur. If this bit is set to 0, Suspend / Resume cannot be supported. In UHS-II mode, Read Wait is disabled and DAT\\[2\\]
line is used for Interrupt Signal from UHS-II Card."]
    #[inline(always)]
    pub fn rdwait_ctrl(&self) -> RdwaitCtrlR {
        RdwaitCtrlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
This bit is valid only in 4-bit mode of the SDIO card and selects a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. If the SD card cannot signal an interrupt during a multiple block transfer, this bit should be set to 0. When the HD detects an SD card insertion, it shall set this bit according to the CCCR of the SDIO card."]
    #[inline(always)]
    pub fn intrpt_at_blk_gap(&self) -> IntrptAtBlkGapR {
        IntrptAtBlkGapR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
SPI mode enable bit."]
    #[inline(always)]
    pub fn spi_mode(&self) -> SpiModeR {
        SpiModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
To start boot code access."]
    #[inline(always)]
    pub fn boot_enable(&self) -> BootEnableR {
        BootEnableR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
To start boot code access in alternative mode."]
    #[inline(always)]
    pub fn alt_boot_mode(&self) -> AltBootModeR {
        AltBootModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
To check for the boot acknowledge in boot operation."]
    #[inline(always)]
    pub fn boot_ack_ena(&self) -> BootAckEnaR {
        BootAckEnaR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit is used to stop executing a transaction at the next block gap for non- DMA,SDMA and ADMA transfers. Until the transfer complete is set to 1, indicating a transfer completion the HD shall leave this bit set to 1. Clearing both the Stop At Block Gap Request and Continue Request shall not cause the transaction to restart. Read Wait is used to stop the read transaction at the block gap. The HC shall honour Stop At Block Gap Request for write transfers, but for read transfers it requires that the SD card support Read Wait. Therefore the HD shall not set this bit during read transfers unless the SD card supports Read Wait and has set Read Wait Control to 1. In case of write transfers in which the HD writes data to the Buffer Data Port register, the HD shall set this bit after all block data is written. If this bit is set to 1, the HD shall not write data to Buffer data port register. This bit affects Read Transfer Active, Write Transfer Active, DAT line active and Command Inhibit \\[DAT\\]
in the Present State register. In case of UHS-II, a transaction can be stopped at the boundary of DATA Burst \\[Flow Control basis\\]. Host Control-ler waits for sending Flow Control MSG until Continue Request is set to 1. '0' Transfer '1' Stop"]
    #[inline(always)]
    #[must_use]
    pub fn stop_at_blk_gap(&mut self) -> StopAtBlkGapW<SdhcWrap_CtlCfg_CtlcfgBlockGapControlSpec> {
        StopAtBlkGapW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
This bit is used to restart a transaction which was stopped using the Stop At Block Gap Request. To cancel stop at the block gap, set Stop At block Gap Request to 0 and set this bit to restart the transfer. The Host Controller automatically clears this bit when the transaction re-starts. If Stop At Block Gap Request is set to 1, any write to this bit is ignored. In SD mode, this bit is cleared in either of the following cases: \\[1\\]
In the case of a read transaction, the DAT Line Active changes from 0 to 1 as a read transaction restarts. \\[2\\]
In the case of a write transaction, the Write transfer active changes from 0 to 1 as the write transaction restarts. Therefore it is not necessary for Host driver to set this bit to 0. If Stop At Block Gap Request is set to 1, any write to this bit is ignored. '0' Ignore '1' Restart"]
    #[inline(always)]
    #[must_use]
    pub fn continue_(&mut self) -> ContinueW<SdhcWrap_CtlCfg_CtlcfgBlockGapControlSpec> {
        ContinueW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
The read wait function is optional for SDIO cards. If the card supports read wait, set this bit to enable use of the read wait protocol to stop read data using DAT\\[2\\]
line. Otherwise the HC has to stop the SD clock to hold read data, which restricts commands generation. When the HD detects an SD card insertion, it shall set this bit according to the CCCR of the SDIO card. If the card does not support read wait, this bit shall never be set to 1 otherwise DAT line conflict may occur. If this bit is set to 0, Suspend / Resume cannot be supported. In UHS-II mode, Read Wait is disabled and DAT\\[2\\]
line is used for Interrupt Signal from UHS-II Card."]
    #[inline(always)]
    #[must_use]
    pub fn rdwait_ctrl(&mut self) -> RdwaitCtrlW<SdhcWrap_CtlCfg_CtlcfgBlockGapControlSpec> {
        RdwaitCtrlW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
This bit is valid only in 4-bit mode of the SDIO card and selects a sample point in the interrupt cycle. Setting to 1 enables interrupt detection at the block gap for a multiple block transfer. If the SD card cannot signal an interrupt during a multiple block transfer, this bit should be set to 0. When the HD detects an SD card insertion, it shall set this bit according to the CCCR of the SDIO card."]
    #[inline(always)]
    #[must_use]
    pub fn intrpt_at_blk_gap(
        &mut self,
    ) -> IntrptAtBlkGapW<SdhcWrap_CtlCfg_CtlcfgBlockGapControlSpec> {
        IntrptAtBlkGapW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
SPI mode enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mode(&mut self) -> SpiModeW<SdhcWrap_CtlCfg_CtlcfgBlockGapControlSpec> {
        SpiModeW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
To start boot code access."]
    #[inline(always)]
    #[must_use]
    pub fn boot_enable(&mut self) -> BootEnableW<SdhcWrap_CtlCfg_CtlcfgBlockGapControlSpec> {
        BootEnableW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
To start boot code access in alternative mode."]
    #[inline(always)]
    #[must_use]
    pub fn alt_boot_mode(&mut self) -> AltBootModeW<SdhcWrap_CtlCfg_CtlcfgBlockGapControlSpec> {
        AltBootModeW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
To check for the boot acknowledge in boot operation."]
    #[inline(always)]
    #[must_use]
    pub fn boot_ack_ena(&mut self) -> BootAckEnaW<SdhcWrap_CtlCfg_CtlcfgBlockGapControlSpec> {
        BootAckEnaW::new(self, 7)
    }
}
#[doc = "This register is used to program the block gap request, read wait control and interrupt at block gap\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdhc_wrap__ctl_cfg__ctlcfg_block_gap_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdhc_wrap__ctl_cfg__ctlcfg_block_gap_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdhcWrap_CtlCfg_CtlcfgBlockGapControlSpec;
impl crate::RegisterSpec for SdhcWrap_CtlCfg_CtlcfgBlockGapControlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`sdhc_wrap__ctl_cfg__ctlcfg_block_gap_control::R`](R) reader structure"]
impl crate::Readable for SdhcWrap_CtlCfg_CtlcfgBlockGapControlSpec {}
#[doc = "`write(|w| ..)` method takes [`sdhc_wrap__ctl_cfg__ctlcfg_block_gap_control::W`](W) writer structure"]
impl crate::Writable for SdhcWrap_CtlCfg_CtlcfgBlockGapControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SDHC_WRAP__CTL_CFG__CTLCFG_block_gap_control to value 0x80"]
impl crate::Resettable for SdhcWrap_CtlCfg_CtlcfgBlockGapControlSpec {
    const RESET_VALUE: u8 = 0x80;
}
