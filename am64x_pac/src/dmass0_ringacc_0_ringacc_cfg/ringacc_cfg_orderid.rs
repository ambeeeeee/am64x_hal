#[doc = "Register `RINGACC_CFG_ORDERID` reader"]
pub type R = crate::R<RingaccCfgOrderidSpec>;
#[doc = "Register `RINGACC_CFG_ORDERID` writer"]
pub type W = crate::W<RingaccCfgOrderidSpec>;
#[doc = "Field `ORDERID` reader - 3:0\\]
Defines the bus orderid value for this ring or queue."]
pub type OrderidR = crate::FieldReader;
#[doc = "Field `ORDERID` writer - 3:0\\]
Defines the bus orderid value for this ring or queue."]
pub type OrderidW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REPLACE` reader - 4:4\\]
Indicates to replace the bus orderid value for this ring or queue with the orderid MMR field. This allows control over the orderid value when it must be restricted due to the topology for QoS reasons. 0 = bypass and use the orderid from the source transaction for the destination transaction. 1 = use the orderid MMR field value for the destination transaction."]
pub type ReplaceR = crate::BitReader;
#[doc = "Field `REPLACE` writer - 4:4\\]
Indicates to replace the bus orderid value for this ring or queue with the orderid MMR field. This allows control over the orderid value when it must be restricted due to the topology for QoS reasons. 0 = bypass and use the orderid from the source transaction for the destination transaction. 1 = use the orderid MMR field value for the destination transaction."]
pub type ReplaceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the bus orderid value for this ring or queue."]
    #[inline(always)]
    pub fn orderid(&self) -> OrderidR {
        OrderidR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates to replace the bus orderid value for this ring or queue with the orderid MMR field. This allows control over the orderid value when it must be restricted due to the topology for QoS reasons. 0 = bypass and use the orderid from the source transaction for the destination transaction. 1 = use the orderid MMR field value for the destination transaction."]
    #[inline(always)]
    pub fn replace(&self) -> ReplaceR {
        ReplaceR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Defines the bus orderid value for this ring or queue."]
    #[inline(always)]
    #[must_use]
    pub fn orderid(&mut self) -> OrderidW<RingaccCfgOrderidSpec> {
        OrderidW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Indicates to replace the bus orderid value for this ring or queue with the orderid MMR field. This allows control over the orderid value when it must be restricted due to the topology for QoS reasons. 0 = bypass and use the orderid from the source transaction for the destination transaction. 1 = use the orderid MMR field value for the destination transaction."]
    #[inline(always)]
    #[must_use]
    pub fn replace(&mut self) -> ReplaceW<RingaccCfgOrderidSpec> {
        ReplaceW::new(self, 4)
    }
}
#[doc = "The Ring OrderID Register contains the bus orderid value for the ring memory access.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ringacc_cfg_orderid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ringacc_cfg_orderid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RingaccCfgOrderidSpec;
impl crate::RegisterSpec for RingaccCfgOrderidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ringacc_cfg_orderid::R`](R) reader structure"]
impl crate::Readable for RingaccCfgOrderidSpec {}
#[doc = "`write(|w| ..)` method takes [`ringacc_cfg_orderid::W`](W) writer structure"]
impl crate::Writable for RingaccCfgOrderidSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RINGACC_CFG_ORDERID to value 0"]
impl crate::Resettable for RingaccCfgOrderidSpec {
    const RESET_VALUE: u32 = 0;
}
