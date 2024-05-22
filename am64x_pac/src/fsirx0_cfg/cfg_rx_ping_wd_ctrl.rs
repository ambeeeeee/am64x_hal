#[doc = "Register `CFG_RX_PING_WD_CTRL` reader"]
pub type R = crate::R<CfgRxPingWdCtrlSpec>;
#[doc = "Register `CFG_RX_PING_WD_CTRL` writer"]
pub type W = crate::W<CfgRxPingWdCtrlSpec>;
#[doc = "Field `PING_WD_RST` reader - 0:0\\]
Ping Watchdog Counter Reset bitThis bit will reset the ping watchdog counter to 0. This bit will always be read as 0. 0h \\[R/W\\]
= Writing a 0 to this bit has no effect.1h \\[W\\]
= The ping watchdog counter will be reset to 0."]
pub type PingWdRstR = crate::BitReader;
#[doc = "Field `PING_WD_RST` writer - 0:0\\]
Ping Watchdog Counter Reset bitThis bit will reset the ping watchdog counter to 0. This bit will always be read as 0. 0h \\[R/W\\]
= Writing a 0 to this bit has no effect.1h \\[W\\]
= The ping watchdog counter will be reset to 0."]
pub type PingWdRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PING_WD_EN` reader - 1:1\\]
Ping Watchdog Counter Enable bitThis bit will enable or disable the ping watchdog counter. The counter \\[RX_PING_WD_CNT\\]
will begin counting from 0 when it is enabled. When the reference value \\[RX_PING_WD_REF\\]
is reached, it will generate a ping watchdog timeout event \\[RX_EVT_STS.PING_WD_TO\\]
and the counter value will reset to 0, and resume counting 0h \\[R/W\\]
= The ping watchdog counter is disabled and not running.1h \\[R/W\\]
= The ping watchdog counter logic is enabled and running."]
pub type PingWdEnR = crate::BitReader;
#[doc = "Field `PING_WD_EN` writer - 1:1\\]
Ping Watchdog Counter Enable bitThis bit will enable or disable the ping watchdog counter. The counter \\[RX_PING_WD_CNT\\]
will begin counting from 0 when it is enabled. When the reference value \\[RX_PING_WD_REF\\]
is reached, it will generate a ping watchdog timeout event \\[RX_EVT_STS.PING_WD_TO\\]
and the counter value will reset to 0, and resume counting 0h \\[R/W\\]
= The ping watchdog counter is disabled and not running.1h \\[R/W\\]
= The ping watchdog counter logic is enabled and running."]
pub type PingWdEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Ping Watchdog Counter Reset bitThis bit will reset the ping watchdog counter to 0. This bit will always be read as 0. 0h \\[R/W\\]
= Writing a 0 to this bit has no effect.1h \\[W\\]
= The ping watchdog counter will be reset to 0."]
    #[inline(always)]
    pub fn ping_wd_rst(&self) -> PingWdRstR {
        PingWdRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Ping Watchdog Counter Enable bitThis bit will enable or disable the ping watchdog counter. The counter \\[RX_PING_WD_CNT\\]
will begin counting from 0 when it is enabled. When the reference value \\[RX_PING_WD_REF\\]
is reached, it will generate a ping watchdog timeout event \\[RX_EVT_STS.PING_WD_TO\\]
and the counter value will reset to 0, and resume counting 0h \\[R/W\\]
= The ping watchdog counter is disabled and not running.1h \\[R/W\\]
= The ping watchdog counter logic is enabled and running."]
    #[inline(always)]
    pub fn ping_wd_en(&self) -> PingWdEnR {
        PingWdEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Ping Watchdog Counter Reset bitThis bit will reset the ping watchdog counter to 0. This bit will always be read as 0. 0h \\[R/W\\]
= Writing a 0 to this bit has no effect.1h \\[W\\]
= The ping watchdog counter will be reset to 0."]
    #[inline(always)]
    #[must_use]
    pub fn ping_wd_rst(&mut self) -> PingWdRstW<CfgRxPingWdCtrlSpec> {
        PingWdRstW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Ping Watchdog Counter Enable bitThis bit will enable or disable the ping watchdog counter. The counter \\[RX_PING_WD_CNT\\]
will begin counting from 0 when it is enabled. When the reference value \\[RX_PING_WD_REF\\]
is reached, it will generate a ping watchdog timeout event \\[RX_EVT_STS.PING_WD_TO\\]
and the counter value will reset to 0, and resume counting 0h \\[R/W\\]
= The ping watchdog counter is disabled and not running.1h \\[R/W\\]
= The ping watchdog counter logic is enabled and running."]
    #[inline(always)]
    #[must_use]
    pub fn ping_wd_en(&mut self) -> PingWdEnW<CfgRxPingWdCtrlSpec> {
        PingWdEnW::new(self, 1)
    }
}
#[doc = "Receive ping watchdog control register. Protected by LOCK field in RX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_ping_wd_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_ping_wd_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxPingWdCtrlSpec;
impl crate::RegisterSpec for CfgRxPingWdCtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cfg_rx_ping_wd_ctrl::R`](R) reader structure"]
impl crate::Readable for CfgRxPingWdCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_ping_wd_ctrl::W`](W) writer structure"]
impl crate::Writable for CfgRxPingWdCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CFG_RX_PING_WD_CTRL to value 0"]
impl crate::Resettable for CfgRxPingWdCtrlSpec {
    const RESET_VALUE: u16 = 0;
}
