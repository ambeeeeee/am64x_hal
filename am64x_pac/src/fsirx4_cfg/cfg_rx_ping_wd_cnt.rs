#[doc = "Register `CFG_RX_PING_WD_CNT` reader"]
pub type R = crate::R<CfgRxPingWdCntSpec>;
#[doc = "Register `CFG_RX_PING_WD_CNT` writer"]
pub type W = crate::W<CfgRxPingWdCntSpec>;
#[doc = "Field `PING_WD_CNT` reader - 31:0\\]
Ping Watchdog Counter ValueThis is the 32-bit read-only register which shows the current value of the ping watchdog counter. This counter is reset to 0 in a variety of ways: A write to PING_WD_RST, a match with PING_WD_REF, or the reception of a ping frame."]
pub type PingWdCntR = crate::FieldReader<u32>;
#[doc = "Field `PING_WD_CNT` writer - 31:0\\]
Ping Watchdog Counter ValueThis is the 32-bit read-only register which shows the current value of the ping watchdog counter. This counter is reset to 0 in a variety of ways: A write to PING_WD_RST, a match with PING_WD_REF, or the reception of a ping frame."]
pub type PingWdCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Ping Watchdog Counter ValueThis is the 32-bit read-only register which shows the current value of the ping watchdog counter. This counter is reset to 0 in a variety of ways: A write to PING_WD_RST, a match with PING_WD_REF, or the reception of a ping frame."]
    #[inline(always)]
    pub fn ping_wd_cnt(&self) -> PingWdCntR {
        PingWdCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Ping Watchdog Counter ValueThis is the 32-bit read-only register which shows the current value of the ping watchdog counter. This counter is reset to 0 in a variety of ways: A write to PING_WD_RST, a match with PING_WD_REF, or the reception of a ping frame."]
    #[inline(always)]
    #[must_use]
    pub fn ping_wd_cnt(&mut self) -> PingWdCntW<CfgRxPingWdCntSpec> {
        PingWdCntW::new(self, 0)
    }
}
#[doc = "Receive pingwatchdog current count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_rx_ping_wd_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_rx_ping_wd_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgRxPingWdCntSpec;
impl crate::RegisterSpec for CfgRxPingWdCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_rx_ping_wd_cnt::R`](R) reader structure"]
impl crate::Readable for CfgRxPingWdCntSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_rx_ping_wd_cnt::W`](W) writer structure"]
impl crate::Writable for CfgRxPingWdCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_RX_PING_WD_CNT to value 0"]
impl crate::Resettable for CfgRxPingWdCntSpec {
    const RESET_VALUE: u32 = 0;
}
