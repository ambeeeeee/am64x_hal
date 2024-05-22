#[doc = "Register `CFG_TX_MASTER_CTRL` reader"]
pub type R = crate::R<CfgTxMasterCtrlSpec>;
#[doc = "Register `CFG_TX_MASTER_CTRL` writer"]
pub type W = crate::W<CfgTxMasterCtrlSpec>;
#[doc = "Field `CORE_RST` reader - 0:0\\]
Transmitter Master Core Reset bitThis bit controls the transmitter master core reset. In order to send any frame, this bit must be cleared. 0h \\[R/W\\]
= Transmitter core is not in reset and can transmit frames.1h \\[R/W\\]
= Transmitter core is held in reset. Note: The KEY field must contatin 0xA5 for any write to this bit to take effect."]
pub type CoreRstR = crate::BitReader;
#[doc = "Field `CORE_RST` writer - 0:0\\]
Transmitter Master Core Reset bitThis bit controls the transmitter master core reset. In order to send any frame, this bit must be cleared. 0h \\[R/W\\]
= Transmitter core is not in reset and can transmit frames.1h \\[R/W\\]
= Transmitter core is held in reset. Note: The KEY field must contatin 0xA5 for any write to this bit to take effect."]
pub type CoreRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLUSH` reader - 1:1\\]
Flush Operation Start bitThis bit will cause the transmitter to initiate a flush pattern of a single toggle on the TXD0 and TXD1 followed by five full cycles of TXCLK. This bit should be written only when the CORE_RST bit is 0 and the clock to the Transmitter core is turned on. 0h \\[R/W\\]
= Clear this bit.1h \\[R/W\\]
= Setting this bit will Initiate flush sequence. To properly execute a flush sequence, Set FLUSH to 1, wait for five TXCLK cycles then clear FLUSH to 0. Note: The KEY field must contain 0xA5 for any write to this bit to take effect. The software must keep this bit set to 1 for at least five TXCLK cycles before setting it back to 0."]
pub type FlushR = crate::BitReader;
#[doc = "Field `FLUSH` writer - 1:1\\]
Flush Operation Start bitThis bit will cause the transmitter to initiate a flush pattern of a single toggle on the TXD0 and TXD1 followed by five full cycles of TXCLK. This bit should be written only when the CORE_RST bit is 0 and the clock to the Transmitter core is turned on. 0h \\[R/W\\]
= Clear this bit.1h \\[R/W\\]
= Setting this bit will Initiate flush sequence. To properly execute a flush sequence, Set FLUSH to 1, wait for five TXCLK cycles then clear FLUSH to 0. Note: The KEY field must contain 0xA5 for any write to this bit to take effect. The software must keep this bit set to 1 for at least five TXCLK cycles before setting it back to 0."]
pub type FlushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` reader - 15:8\\]
Write KeyIn order to write to any bit in this register, 0xA5 must be written to this field at the same time. Otherwise, writes are ignored. The key is cleared immediately after writing, so it must be written again for every change to this register."]
pub type KeyR = crate::FieldReader;
#[doc = "Field `KEY` writer - 15:8\\]
Write KeyIn order to write to any bit in this register, 0xA5 must be written to this field at the same time. Otherwise, writes are ignored. The key is cleared immediately after writing, so it must be written again for every change to this register."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Transmitter Master Core Reset bitThis bit controls the transmitter master core reset. In order to send any frame, this bit must be cleared. 0h \\[R/W\\]
= Transmitter core is not in reset and can transmit frames.1h \\[R/W\\]
= Transmitter core is held in reset. Note: The KEY field must contatin 0xA5 for any write to this bit to take effect."]
    #[inline(always)]
    pub fn core_rst(&self) -> CoreRstR {
        CoreRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Flush Operation Start bitThis bit will cause the transmitter to initiate a flush pattern of a single toggle on the TXD0 and TXD1 followed by five full cycles of TXCLK. This bit should be written only when the CORE_RST bit is 0 and the clock to the Transmitter core is turned on. 0h \\[R/W\\]
= Clear this bit.1h \\[R/W\\]
= Setting this bit will Initiate flush sequence. To properly execute a flush sequence, Set FLUSH to 1, wait for five TXCLK cycles then clear FLUSH to 0. Note: The KEY field must contain 0xA5 for any write to this bit to take effect. The software must keep this bit set to 1 for at least five TXCLK cycles before setting it back to 0."]
    #[inline(always)]
    pub fn flush(&self) -> FlushR {
        FlushR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Write KeyIn order to write to any bit in this register, 0xA5 must be written to this field at the same time. Otherwise, writes are ignored. The key is cleared immediately after writing, so it must be written again for every change to this register."]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Transmitter Master Core Reset bitThis bit controls the transmitter master core reset. In order to send any frame, this bit must be cleared. 0h \\[R/W\\]
= Transmitter core is not in reset and can transmit frames.1h \\[R/W\\]
= Transmitter core is held in reset. Note: The KEY field must contatin 0xA5 for any write to this bit to take effect."]
    #[inline(always)]
    #[must_use]
    pub fn core_rst(&mut self) -> CoreRstW<CfgTxMasterCtrlSpec> {
        CoreRstW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Flush Operation Start bitThis bit will cause the transmitter to initiate a flush pattern of a single toggle on the TXD0 and TXD1 followed by five full cycles of TXCLK. This bit should be written only when the CORE_RST bit is 0 and the clock to the Transmitter core is turned on. 0h \\[R/W\\]
= Clear this bit.1h \\[R/W\\]
= Setting this bit will Initiate flush sequence. To properly execute a flush sequence, Set FLUSH to 1, wait for five TXCLK cycles then clear FLUSH to 0. Note: The KEY field must contain 0xA5 for any write to this bit to take effect. The software must keep this bit set to 1 for at least five TXCLK cycles before setting it back to 0."]
    #[inline(always)]
    #[must_use]
    pub fn flush(&mut self) -> FlushW<CfgTxMasterCtrlSpec> {
        FlushW::new(self, 1)
    }
    #[doc = "Bits 8:15 - 15:8\\]
Write KeyIn order to write to any bit in this register, 0xA5 must be written to this field at the same time. Otherwise, writes are ignored. The key is cleared immediately after writing, so it must be written again for every change to this register."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KeyW<CfgTxMasterCtrlSpec> {
        KeyW::new(self, 8)
    }
}
#[doc = "Transmit master control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_master_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_master_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTxMasterCtrlSpec;
impl crate::RegisterSpec for CfgTxMasterCtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_tx_master_ctrl::R`](R) reader structure"]
impl crate::Readable for CfgTxMasterCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tx_master_ctrl::W`](W) writer structure"]
impl crate::Writable for CfgTxMasterCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_TX_MASTER_CTRL to value 0"]
impl crate::Resettable for CfgTxMasterCtrlSpec {
    const RESET_VALUE: u16 = 0;
}
