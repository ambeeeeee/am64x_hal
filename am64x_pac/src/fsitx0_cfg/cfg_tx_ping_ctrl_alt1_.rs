#[doc = "Register `CFG_TX_PING_CTRL_ALT1_` reader"]
pub type R = crate::R<CfgTxPingCtrlAlt1_Spec>;
#[doc = "Register `CFG_TX_PING_CTRL_ALT1_` writer"]
pub type W = crate::W<CfgTxPingCtrlAlt1_Spec>;
#[doc = "Field `CNT_RST` reader - 0:0\\]
Ping Counter Reset bitThis bit will reset the the ping counter to 0. This bit will always be read as 0. 0h \\[R/W\\]
= Writing a 0 to this bit has no effect.1h \\[R/W\\]
= The ping counter will be reset to 0."]
pub type CntRstR = crate::BitReader;
#[doc = "Field `CNT_RST` writer - 0:0\\]
Ping Counter Reset bitThis bit will reset the the ping counter to 0. This bit will always be read as 0. 0h \\[R/W\\]
= Writing a 0 to this bit has no effect.1h \\[R/W\\]
= The ping counter will be reset to 0."]
pub type CntRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_EN` reader - 1:1\\]
Ping Timer Enable bitThis bit will enable the ping timer for generating periodic ping frames. 0h \\[R/W\\]
= The ping timer is disabled and will not generate ping frames.1h \\[R/W\\]
= The ping timer is enabled and can be used to generate ping frames.Once the timer count reaches the value set by the TX_PING_TO_REF register, it will initiate a ping frame transmission. Note: If the ping timer is used, EXT_TRIG_EN should not be set as it will override this function."]
pub type TimerEnR = crate::BitReader;
#[doc = "Field `TIMER_EN` writer - 1:1\\]
Ping Timer Enable bitThis bit will enable the ping timer for generating periodic ping frames. 0h \\[R/W\\]
= The ping timer is disabled and will not generate ping frames.1h \\[R/W\\]
= The ping timer is enabled and can be used to generate ping frames.Once the timer count reaches the value set by the TX_PING_TO_REF register, it will initiate a ping frame transmission. Note: If the ping timer is used, EXT_TRIG_EN should not be set as it will override this function."]
pub type TimerEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_TRIG_EN` reader - 2:2\\]
External Trigger Enable bitThis bit will allow the external trigger logic to generate a ping frame. 0h \\[R/W\\]
= External triggers will not be used to generate ping frames.1h \\[R/W\\]
= The selected external trigger \\[selected by EXT_TRIG_SEL bits\\]
will be able to generate a ping frame. The ping timer will be ignored if this bit is set."]
pub type ExtTrigEnR = crate::BitReader;
#[doc = "Field `EXT_TRIG_EN` writer - 2:2\\]
External Trigger Enable bitThis bit will allow the external trigger logic to generate a ping frame. 0h \\[R/W\\]
= External triggers will not be used to generate ping frames.1h \\[R/W\\]
= The selected external trigger \\[selected by EXT_TRIG_SEL bits\\]
will be able to generate a ping frame. The ping timer will be ignored if this bit is set."]
pub type ExtTrigEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_TRIG_SEL` reader - 8:3\\]
External Trigger Select bitsThis bitfield will select one of the 64 external trigger inputs to as the source to generate a ping frame. A ping frame will only be generated if the EXT_TRIG_EN bit is set. 0h \\[R/W\\]
= Trigger 1 will be used to generate a ping frame.1h \\[R/W\\]
= Trigger 2 will be used to generate a ping frame...3Fh \\[R/W\\]
= Trigger 64 will be used to generate a ping frame."]
pub type ExtTrigSelR = crate::FieldReader;
#[doc = "Field `EXT_TRIG_SEL` writer - 8:3\\]
External Trigger Select bitsThis bitfield will select one of the 64 external trigger inputs to as the source to generate a ping frame. A ping frame will only be generated if the EXT_TRIG_EN bit is set. 0h \\[R/W\\]
= Trigger 1 will be used to generate a ping frame.1h \\[R/W\\]
= Trigger 2 will be used to generate a ping frame...3Fh \\[R/W\\]
= Trigger 64 will be used to generate a ping frame."]
pub type ExtTrigSelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Ping Counter Reset bitThis bit will reset the the ping counter to 0. This bit will always be read as 0. 0h \\[R/W\\]
= Writing a 0 to this bit has no effect.1h \\[R/W\\]
= The ping counter will be reset to 0."]
    #[inline(always)]
    pub fn cnt_rst(&self) -> CntRstR {
        CntRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Ping Timer Enable bitThis bit will enable the ping timer for generating periodic ping frames. 0h \\[R/W\\]
= The ping timer is disabled and will not generate ping frames.1h \\[R/W\\]
= The ping timer is enabled and can be used to generate ping frames.Once the timer count reaches the value set by the TX_PING_TO_REF register, it will initiate a ping frame transmission. Note: If the ping timer is used, EXT_TRIG_EN should not be set as it will override this function."]
    #[inline(always)]
    pub fn timer_en(&self) -> TimerEnR {
        TimerEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
External Trigger Enable bitThis bit will allow the external trigger logic to generate a ping frame. 0h \\[R/W\\]
= External triggers will not be used to generate ping frames.1h \\[R/W\\]
= The selected external trigger \\[selected by EXT_TRIG_SEL bits\\]
will be able to generate a ping frame. The ping timer will be ignored if this bit is set."]
    #[inline(always)]
    pub fn ext_trig_en(&self) -> ExtTrigEnR {
        ExtTrigEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:8 - 8:3\\]
External Trigger Select bitsThis bitfield will select one of the 64 external trigger inputs to as the source to generate a ping frame. A ping frame will only be generated if the EXT_TRIG_EN bit is set. 0h \\[R/W\\]
= Trigger 1 will be used to generate a ping frame.1h \\[R/W\\]
= Trigger 2 will be used to generate a ping frame...3Fh \\[R/W\\]
= Trigger 64 will be used to generate a ping frame."]
    #[inline(always)]
    pub fn ext_trig_sel(&self) -> ExtTrigSelR {
        ExtTrigSelR::new(((self.bits >> 3) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Ping Counter Reset bitThis bit will reset the the ping counter to 0. This bit will always be read as 0. 0h \\[R/W\\]
= Writing a 0 to this bit has no effect.1h \\[R/W\\]
= The ping counter will be reset to 0."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_rst(&mut self) -> CntRstW<CfgTxPingCtrlAlt1_Spec> {
        CntRstW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Ping Timer Enable bitThis bit will enable the ping timer for generating periodic ping frames. 0h \\[R/W\\]
= The ping timer is disabled and will not generate ping frames.1h \\[R/W\\]
= The ping timer is enabled and can be used to generate ping frames.Once the timer count reaches the value set by the TX_PING_TO_REF register, it will initiate a ping frame transmission. Note: If the ping timer is used, EXT_TRIG_EN should not be set as it will override this function."]
    #[inline(always)]
    #[must_use]
    pub fn timer_en(&mut self) -> TimerEnW<CfgTxPingCtrlAlt1_Spec> {
        TimerEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
External Trigger Enable bitThis bit will allow the external trigger logic to generate a ping frame. 0h \\[R/W\\]
= External triggers will not be used to generate ping frames.1h \\[R/W\\]
= The selected external trigger \\[selected by EXT_TRIG_SEL bits\\]
will be able to generate a ping frame. The ping timer will be ignored if this bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn ext_trig_en(&mut self) -> ExtTrigEnW<CfgTxPingCtrlAlt1_Spec> {
        ExtTrigEnW::new(self, 2)
    }
    #[doc = "Bits 3:8 - 8:3\\]
External Trigger Select bitsThis bitfield will select one of the 64 external trigger inputs to as the source to generate a ping frame. A ping frame will only be generated if the EXT_TRIG_EN bit is set. 0h \\[R/W\\]
= Trigger 1 will be used to generate a ping frame.1h \\[R/W\\]
= Trigger 2 will be used to generate a ping frame...3Fh \\[R/W\\]
= Trigger 64 will be used to generate a ping frame."]
    #[inline(always)]
    #[must_use]
    pub fn ext_trig_sel(&mut self) -> ExtTrigSelW<CfgTxPingCtrlAlt1_Spec> {
        ExtTrigSelW::new(self, 3)
    }
}
#[doc = "Transmit ping control register. Protected by LOCK field in TX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_ping_ctrl_alt1_::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_ping_ctrl_alt1_::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTxPingCtrlAlt1_Spec;
impl crate::RegisterSpec for CfgTxPingCtrlAlt1_Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_tx_ping_ctrl_alt1_::R`](R) reader structure"]
impl crate::Readable for CfgTxPingCtrlAlt1_Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tx_ping_ctrl_alt1_::W`](W) writer structure"]
impl crate::Writable for CfgTxPingCtrlAlt1_Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_TX_PING_CTRL_ALT1_ to value 0"]
impl crate::Resettable for CfgTxPingCtrlAlt1_Spec {
    const RESET_VALUE: u16 = 0;
}
