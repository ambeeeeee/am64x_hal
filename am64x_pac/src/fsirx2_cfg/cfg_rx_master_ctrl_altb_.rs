#[doc = "Register `CFG_RX_MASTER_CTRL_ALTB_` reader"]
pub type R = crate::R<CfgRxMasterCtrlAltb_Spec>;
#[doc = "Register `CFG_RX_MASTER_CTRL_ALTB_` writer"]
pub type W = crate::W<CfgRxMasterCtrlAltb_Spec>;
#[doc = "Field `CORE_RST` reader - 0:0\\]
Receiver Master Core Reset bitThis bit controls the receiver master core reset. In order to receive any frame, this bit must be cleared.Note: For reset to take affect, the FSI RX module must be held in reset for at least 4 SYSCLK cycles. 0h \\[R/W\\]
= Receiver core is not in reset and can receive frames.1h \\[R/W\\]
= Receiver core is held in reset. Note: The KEY field must contatin 0xA5 for any write to this bit to take effect."]
pub type CoreRstR = crate::BitReader;
#[doc = "Field `CORE_RST` writer - 0:0\\]
Receiver Master Core Reset bitThis bit controls the receiver master core reset. In order to receive any frame, this bit must be cleared.Note: For reset to take affect, the FSI RX module must be held in reset for at least 4 SYSCLK cycles. 0h \\[R/W\\]
= Receiver core is not in reset and can receive frames.1h \\[R/W\\]
= Receiver core is held in reset. Note: The KEY field must contatin 0xA5 for any write to this bit to take effect."]
pub type CoreRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_LOOPBACK` reader - 1:1\\]
Internal Loopback Enable bitThis bit enables the internal loopback functionality of the FSI receiver. By enabling this bit, a mux will select the signals coming directly from the corresponding FSI transmitter module rather than from the pins. 0h \\[R/W\\]
= Internal loopback is disabled. The FSI RX module will receive signals coming from the pins.1h \\[R/W\\]
= Internal loopback is enabled. The FSI RX module will receive signals from the directly from FSI TX module rather than the pins. Note: The KEY field must contatin 0xA5 for any write to this bit to take effect."]
pub type IntLoopbackR = crate::BitReader;
#[doc = "Field `INT_LOOPBACK` writer - 1:1\\]
Internal Loopback Enable bitThis bit enables the internal loopback functionality of the FSI receiver. By enabling this bit, a mux will select the signals coming directly from the corresponding FSI transmitter module rather than from the pins. 0h \\[R/W\\]
= Internal loopback is disabled. The FSI RX module will receive signals coming from the pins.1h \\[R/W\\]
= Internal loopback is enabled. The FSI RX module will receive signals from the directly from FSI TX module rather than the pins. Note: The KEY field must contatin 0xA5 for any write to this bit to take effect."]
pub type IntLoopbackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_PAIRING` reader - 2:2\\]
Clock Pairing for SPI-like Behavior Enable bitThis bit enables the internal clock pairing with the FSI TX module. This feature internally connects the TXCLK to RXCLK allowing the FSI TX module, acting as a SPI master, to clock data into the receiver and out of the transmitter like a standard SPI module. This configuration is valid when the Module is in SPI mode only \\[RX_OPER_CTRL.SPI_MODE = 1\\]
0h \\[R/W\\]
= SPI clock pairing is not enabled.1h \\[R/W\\]
= SPI clock pairing is enabled. The RXCLK will be internally connected to the TXCLK of the corresponding FSI module. Note: The KEY field must contatin 0xA5 for any write to this bit to take effect."]
pub type SpiPairingR = crate::BitReader;
#[doc = "Field `SPI_PAIRING` writer - 2:2\\]
Clock Pairing for SPI-like Behavior Enable bitThis bit enables the internal clock pairing with the FSI TX module. This feature internally connects the TXCLK to RXCLK allowing the FSI TX module, acting as a SPI master, to clock data into the receiver and out of the transmitter like a standard SPI module. This configuration is valid when the Module is in SPI mode only \\[RX_OPER_CTRL.SPI_MODE = 1\\]
0h \\[R/W\\]
= SPI clock pairing is not enabled.1h \\[R/W\\]
= SPI clock pairing is enabled. The RXCLK will be internally connected to the TXCLK of the corresponding FSI module. Note: The KEY field must contatin 0xA5 for any write to this bit to take effect."]
pub type SpiPairingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUT_ISOLATE` reader - 3:3\\]
When set to 1, the FSI RX inputs \\[RXCLK, RXD0 and RXD1\\]
will be isolated from what is driven from the device pins and will be held at inactive level of '1'.This isolation facilitates the user to switch the RX inputs to a different set of device pins and hence any potential glitch that could occur during the process of switching will not affect the RX module itself."]
pub type InputIsolateR = crate::BitReader;
#[doc = "Field `INPUT_ISOLATE` writer - 3:3\\]
When set to 1, the FSI RX inputs \\[RXCLK, RXD0 and RXD1\\]
will be isolated from what is driven from the device pins and will be held at inactive level of '1'.This isolation facilitates the user to switch the RX inputs to a different set of device pins and hence any potential glitch that could occur during the process of switching will not affect the RX module itself."]
pub type InputIsolateW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` reader - 15:8\\]
Write Key.In order to write to this register, 0xA5 must be written to this field at the same time. Otherwise, writes are ignored. The key is cleared immediately after writing, so it must be written again for every change to this register."]
pub type KeyR = crate::FieldReader;
#[doc = "Field `KEY` writer - 15:8\\]
Write Key.In order to write to this register, 0xA5 must be written to this field at the same time. Otherwise, writes are ignored. The key is cleared immediately after writing, so it must be written again for every change to this register."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Receiver Master Core Reset bitThis bit controls the receiver master core reset. In order to receive any frame, this bit must be cleared.Note: For reset to take affect, the FSI RX module must be held in reset for at least 4 SYSCLK cycles. 0h \\[R/W\\]
= Receiver core is not in reset and can receive frames.1h \\[R/W\\]
= Receiver core is held in reset. Note: The KEY field must contatin 0xA5 for any write to this bit to take effect."]
    #[inline(always)]
    pub fn core_rst(&self) -> CoreRstR {
        CoreRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal Loopback Enable bitThis bit enables the internal loopback functionality of the FSI receiver. By enabling this bit, a mux will select the signals coming directly from the corresponding FSI transmitter module rather than from the pins. 0h \\[R/W\\]
= Internal loopback is disabled. The FSI RX module will receive signals coming from the pins.1h \\[R/W\\]
= Internal loopback is enabled. The FSI RX module will receive signals from the directly from FSI TX module rather than the pins. Note: The KEY field must contatin 0xA5 for any write to this bit to take effect."]
    #[inline(always)]
    pub fn int_loopback(&self) -> IntLoopbackR {
        IntLoopbackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Clock Pairing for SPI-like Behavior Enable bitThis bit enables the internal clock pairing with the FSI TX module. This feature internally connects the TXCLK to RXCLK allowing the FSI TX module, acting as a SPI master, to clock data into the receiver and out of the transmitter like a standard SPI module. This configuration is valid when the Module is in SPI mode only \\[RX_OPER_CTRL.SPI_MODE = 1\\]
0h \\[R/W\\]
= SPI clock pairing is not enabled.1h \\[R/W\\]
= SPI clock pairing is enabled. The RXCLK will be internally connected to the TXCLK of the corresponding FSI module. Note: The KEY field must contatin 0xA5 for any write to this bit to take effect."]
    #[inline(always)]
    pub fn spi_pairing(&self) -> SpiPairingR {
        SpiPairingR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
When set to 1, the FSI RX inputs \\[RXCLK, RXD0 and RXD1\\]
will be isolated from what is driven from the device pins and will be held at inactive level of '1'.This isolation facilitates the user to switch the RX inputs to a different set of device pins and hence any potential glitch that could occur during the process of switching will not affect the RX module itself."]
    #[inline(always)]
    pub fn input_isolate(&self) -> InputIsolateR {
        InputIsolateR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Write Key.In order to write to this register, 0xA5 must be written to this field at the same time. Otherwise, writes are ignored. The key is cleared immediately after writing, so it must be written again for every change to this register."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Receiver Master Core Reset bitThis bit controls the receiver master core reset. In order to receive any frame, this bit must be cleared.Note: For reset to take affect, the FSI RX module must be held in reset for at least 4 SYSCLK cycles. 0h \\[R/W\\]
= Receiver core is not in reset and can receive frames.1h \\[R/W\\]
= Receiver core is held in reset. Note: The KEY field must contatin 0xA5 for any write to this bit to take effect."]
    #[inline(always)]
    #[must_use]
    pub fn core_rst(&mut self) -> CoreRstW<CfgRxMasterCtrlAltb_Spec> {
        CoreRstW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Internal Loopback Enable bitThis bit enables the internal loopback functionality of the FSI receiver. By enabling this bit, a mux will select the signals coming directly from the corresponding FSI transmitter module rather than from the pins. 0h \\[R/W\\]
= Internal loopback is disabled. The FSI RX module will receive signals coming from the pins.1h \\[R/W\\]
= Internal loopback is enabled. The FSI RX module will receive signals from the directly from FSI TX module rather than the pins. Note: The KEY field must contatin 0xA5 for any write to this bit to take effect."]
    #[inline(always)]
    #[must_use]
    pub fn int_loopback(&mut self) -> IntLoopbackW<CfgRxMasterCtrlAltb_Spec> {
        IntLoopbackW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Clock Pairing for SPI-like Behavior Enable bitThis bit enables the internal clock pairing with the FSI TX module. This feature internally connects the TXCLK to RXCLK allowing the FSI TX module, acting as a SPI master, to clock data into the receiver and out of the transmitter like a standard SPI module. This configuration is valid when the Module is in SPI mode only \\[RX_OPER_CTRL.SPI_MODE = 1\\]
0h \\[R/W\\]
= SPI clock pairing is not enabled.1h \\[R/W\\]
= SPI clock pairing is enabled. The RXCLK will be internally connected to the TXCLK of the corresponding FSI module. Note: The KEY field must contatin 0xA5 for any write to this bit to take effect."]
    #[inline(always)]
    #[must_use]
    pub fn spi_pairing(&mut self) -> SpiPairingW<CfgRxMasterCtrlAltb_Spec> {
        SpiPairingW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
When set to 1, the FSI RX inputs \\[RXCLK, RXD0 and RXD1\\]
will be isolated from what is driven from the device pins and will be held at inactive level of '1'.This isolation facilitates the user to switch the RX inputs to a different set of device pins and hence any potential glitch that could occur during the process of switching will not affect the RX module itself."]
    #[inline(always)]
    #[must_use]
    pub fn input_isolate(&mut self) -> InputIsolateW<CfgRxMasterCtrlAltb_Spec> {
        InputIsolateW::new(self, 3)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Write Key.In order to write to this register, 0xA5 must be written to this field at the same time. Otherwise, writes are ignored. The key is cleared immediately after writing, so it must be written again for every change to this register."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CfgRxMasterCtrlAltb_Spec> {
        KeyW::new(self, 8)
    }
}
#[doc = "Receive master control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_master_ctrl_altb_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_master_ctrl_altb_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxMasterCtrlAltb_Spec;
impl crate::RegisterSpec for CfgRxMasterCtrlAltb_Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_rx_master_ctrl_altb_::R`](R) reader structure"]
impl crate::Readable for CfgRxMasterCtrlAltb_Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_master_ctrl_altb_::W`](W) writer structure"]
impl crate::Writable for CfgRxMasterCtrlAltb_Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_RX_MASTER_CTRL_ALTB_ to value 0"]
impl crate::Resettable for CfgRxMasterCtrlAltb_Spec {
    const RESET_VALUE: u16 = 0;
}
