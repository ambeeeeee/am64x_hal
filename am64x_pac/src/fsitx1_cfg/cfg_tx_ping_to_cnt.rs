#[doc = "Register `CFG_TX_PING_TO_CNT` reader"]
pub type R = crate::R<CfgTxPingToCntSpec>;
#[doc = "Register `CFG_TX_PING_TO_CNT` writer"]
pub type W = crate::W<CfgTxPingToCntSpec>;
#[doc = "Field `TO_CNT` reader - 31:0\\]
Ping Timer Counter ValueThis register contains the current value of the ping timer counter. After reset, this counter will increment until it reaches the reference value \\[TX_PING_TO_REF\\], at which point it generates a ping frame transmission. After this point, the counter will reset to 0 and continue counting. This is a free-running counter"]
pub type ToCntR = crate::FieldReader<u32>;
#[doc = "Field `TO_CNT` writer - 31:0\\]
Ping Timer Counter ValueThis register contains the current value of the ping timer counter. After reset, this counter will increment until it reaches the reference value \\[TX_PING_TO_REF\\], at which point it generates a ping frame transmission. After this point, the counter will reset to 0 and continue counting. This is a free-running counter"]
pub type ToCntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Ping Timer Counter ValueThis register contains the current value of the ping timer counter. After reset, this counter will increment until it reaches the reference value \\[TX_PING_TO_REF\\], at which point it generates a ping frame transmission. After this point, the counter will reset to 0 and continue counting. This is a free-running counter"]
    #[inline(always)]
    pub fn to_cnt(&self) -> ToCntR {
        ToCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Ping Timer Counter ValueThis register contains the current value of the ping timer counter. After reset, this counter will increment until it reaches the reference value \\[TX_PING_TO_REF\\], at which point it generates a ping frame transmission. After this point, the counter will reset to 0 and continue counting. This is a free-running counter"]
    #[inline(always)]
    #[must_use]
    pub fn to_cnt(&mut self) -> ToCntW<CfgTxPingToCntSpec> {
        ToCntW::new(self, 0)
    }
}
#[doc = "Transmit ping timeout current count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_ping_to_cnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_ping_to_cnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTxPingToCntSpec;
impl crate::RegisterSpec for CfgTxPingToCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_tx_ping_to_cnt::R`](R) reader structure"]
impl crate::Readable for CfgTxPingToCntSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tx_ping_to_cnt::W`](W) writer structure"]
impl crate::Writable for CfgTxPingToCntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TX_PING_TO_CNT to value 0"]
impl crate::Resettable for CfgTxPingToCntSpec {
    const RESET_VALUE: u32 = 0;
}
