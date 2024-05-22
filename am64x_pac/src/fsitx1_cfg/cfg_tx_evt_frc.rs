#[doc = "Register `CFG_TX_EVT_FRC` reader"]
pub type R = crate::R<CfgTxEvtFrcSpec>;
#[doc = "Register `CFG_TX_EVT_FRC` writer"]
pub type W = crate::W<CfgTxEvtFrcSpec>;
#[doc = "Field `FRAME_DONE` reader - 0:0\\]
Frame Done Flag Force bitThis bit will cause the corresponding bit in the TX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding flag bit in the TX_EVT_STS Register."]
pub type FrameDoneR = crate::BitReader;
#[doc = "Field `FRAME_DONE` writer - 0:0\\]
Frame Done Flag Force bitThis bit will cause the corresponding bit in the TX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding flag bit in the TX_EVT_STS Register."]
pub type FrameDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_UNDERRUN` reader - 1:1\\]
Buffer Underrun Flag Force bitThis bit will cause the corresponding bit in the TX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding flag bit in the TX_EVT_STS Register."]
pub type BufUnderrunR = crate::BitReader;
#[doc = "Field `BUF_UNDERRUN` writer - 1:1\\]
Buffer Underrun Flag Force bitThis bit will cause the corresponding bit in the TX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding flag bit in the TX_EVT_STS Register."]
pub type BufUnderrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUF_OVERRUN` reader - 2:2\\]
Buffer Overrun Flag Force bitThis bit will cause the corresponding bit in the TX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[R/W\\]
= Writing a 0 to this bit will have no effect.1h \\[R/W\\]
= Force the corresponding flag bit in the TX_EVT_STS Register."]
pub type BufOverrunR = crate::BitReader;
#[doc = "Field `BUF_OVERRUN` writer - 2:2\\]
Buffer Overrun Flag Force bitThis bit will cause the corresponding bit in the TX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[R/W\\]
= Writing a 0 to this bit will have no effect.1h \\[R/W\\]
= Force the corresponding flag bit in the TX_EVT_STS Register."]
pub type BufOverrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PING_TRIGGERED` reader - 3:3\\]
Ping Frame Triggered Flag Force bitThis bit will cause the corresponding bit in the TX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding flag bit in the TX_EVT_STS Register."]
pub type PingTriggeredR = crate::BitReader;
#[doc = "Field `PING_TRIGGERED` writer - 3:3\\]
Ping Frame Triggered Flag Force bitThis bit will cause the corresponding bit in the TX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding flag bit in the TX_EVT_STS Register."]
pub type PingTriggeredW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Frame Done Flag Force bitThis bit will cause the corresponding bit in the TX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding flag bit in the TX_EVT_STS Register."]
    #[inline(always)]
    pub fn frame_done(&self) -> FrameDoneR {
        FrameDoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Buffer Underrun Flag Force bitThis bit will cause the corresponding bit in the TX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding flag bit in the TX_EVT_STS Register."]
    #[inline(always)]
    pub fn buf_underrun(&self) -> BufUnderrunR {
        BufUnderrunR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Buffer Overrun Flag Force bitThis bit will cause the corresponding bit in the TX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[R/W\\]
= Writing a 0 to this bit will have no effect.1h \\[R/W\\]
= Force the corresponding flag bit in the TX_EVT_STS Register."]
    #[inline(always)]
    pub fn buf_overrun(&self) -> BufOverrunR {
        BufOverrunR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Ping Frame Triggered Flag Force bitThis bit will cause the corresponding bit in the TX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding flag bit in the TX_EVT_STS Register."]
    #[inline(always)]
    pub fn ping_triggered(&self) -> PingTriggeredR {
        PingTriggeredR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Frame Done Flag Force bitThis bit will cause the corresponding bit in the TX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding flag bit in the TX_EVT_STS Register."]
    #[inline(always)]
    #[must_use]
    pub fn frame_done(&mut self) -> FrameDoneW<CfgTxEvtFrcSpec> {
        FrameDoneW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Buffer Underrun Flag Force bitThis bit will cause the corresponding bit in the TX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding flag bit in the TX_EVT_STS Register."]
    #[inline(always)]
    #[must_use]
    pub fn buf_underrun(&mut self) -> BufUnderrunW<CfgTxEvtFrcSpec> {
        BufUnderrunW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Buffer Overrun Flag Force bitThis bit will cause the corresponding bit in the TX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[R/W\\]
= Writing a 0 to this bit will have no effect.1h \\[R/W\\]
= Force the corresponding flag bit in the TX_EVT_STS Register."]
    #[inline(always)]
    #[must_use]
    pub fn buf_overrun(&mut self) -> BufOverrunW<CfgTxEvtFrcSpec> {
        BufOverrunW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Ping Frame Triggered Flag Force bitThis bit will cause the corresponding bit in the TX_EVT_STS register to get set. The purpose of this register is to allow software to simulate the effect of the event and test the associated software/ISR. 0h \\[W\\]
= Writing a 0 to this bit will have no effect.1h \\[W\\]
= Force the corresponding flag bit in the TX_EVT_STS Register."]
    #[inline(always)]
    #[must_use]
    pub fn ping_triggered(&mut self) -> PingTriggeredW<CfgTxEvtFrcSpec> {
        PingTriggeredW::new(self, 3)
    }
}
#[doc = "Transmit event and error flag force register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_evt_frc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_evt_frc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTxEvtFrcSpec;
impl crate::RegisterSpec for CfgTxEvtFrcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_tx_evt_frc::R`](R) reader structure"]
impl crate::Readable for CfgTxEvtFrcSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tx_evt_frc::W`](W) writer structure"]
impl crate::Writable for CfgTxEvtFrcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_TX_EVT_FRC to value 0"]
impl crate::Resettable for CfgTxEvtFrcSpec {
    const RESET_VALUE: u16 = 0;
}
