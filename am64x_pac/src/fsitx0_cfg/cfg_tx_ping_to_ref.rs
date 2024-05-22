#[doc = "Register `CFG_TX_PING_TO_REF` reader"]
pub type R = crate::R<CfgTxPingToRefSpec>;
#[doc = "Register `CFG_TX_PING_TO_REF` writer"]
pub type W = crate::W<CfgTxPingToRefSpec>;
#[doc = "Field `TO_REF` reader - 31:0\\]
Ping Timer Reference Value.This is the 32-bit reference value for the ping timer. The timer will increment the counter starting from 0. When the reference value is reached, it will generate a timeout event, triggering a ping frame transmission. The counter will then reset to 0 and continue counting."]
pub type ToRefR = crate::FieldReader<u32>;
#[doc = "Field `TO_REF` writer - 31:0\\]
Ping Timer Reference Value.This is the 32-bit reference value for the ping timer. The timer will increment the counter starting from 0. When the reference value is reached, it will generate a timeout event, triggering a ping frame transmission. The counter will then reset to 0 and continue counting."]
pub type ToRefW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Ping Timer Reference Value.This is the 32-bit reference value for the ping timer. The timer will increment the counter starting from 0. When the reference value is reached, it will generate a timeout event, triggering a ping frame transmission. The counter will then reset to 0 and continue counting."]
    #[inline(always)]
    pub fn to_ref(&self) -> ToRefR {
        ToRefR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Ping Timer Reference Value.This is the 32-bit reference value for the ping timer. The timer will increment the counter starting from 0. When the reference value is reached, it will generate a timeout event, triggering a ping frame transmission. The counter will then reset to 0 and continue counting."]
    #[inline(always)]
    #[must_use]
    pub fn to_ref(&mut self) -> ToRefW<CfgTxPingToRefSpec> {
        ToRefW::new(self, 0)
    }
}
#[doc = "Transmit ping timeout counter reference. Protected by LOCK field in TX_LOCK_CTRL register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_tx_ping_to_ref::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_tx_ping_to_ref::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgTxPingToRefSpec;
impl crate::RegisterSpec for CfgTxPingToRefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_tx_ping_to_ref::R`](R) reader structure"]
impl crate::Readable for CfgTxPingToRefSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_tx_ping_to_ref::W`](W) writer structure"]
impl crate::Writable for CfgTxPingToRefSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_TX_PING_TO_REF to value 0"]
impl crate::Resettable for CfgTxPingToRefSpec {
    const RESET_VALUE: u32 = 0;
}
