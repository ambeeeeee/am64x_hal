#[doc = "Register `CFG0_DDR4_FSP_CLKCHNG_ACK_PROXY` reader"]
pub type R = crate::R<Cfg0Ddr4FspClkchngAckProxySpec>;
#[doc = "Register `CFG0_DDR4_FSP_CLKCHNG_ACK_PROXY` writer"]
pub type W = crate::W<Cfg0Ddr4FspClkchngAckProxySpec>;
#[doc = "Field `DDR4_FSP_CLKCHNG_ACK_ACK_PROXY` reader - 0:0\\]
DDR FSP clock change ackowledge"]
pub type Ddr4FspClkchngAckAckProxyR = crate::BitReader;
#[doc = "Field `DDR4_FSP_CLKCHNG_ACK_ACK_PROXY` writer - 0:0\\]
DDR FSP clock change ackowledge"]
pub type Ddr4FspClkchngAckAckProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DDR FSP clock change ackowledge"]
    #[inline(always)]
    pub fn ddr4_fsp_clkchng_ack_ack_proxy(&self) -> Ddr4FspClkchngAckAckProxyR {
        Ddr4FspClkchngAckAckProxyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DDR FSP clock change ackowledge"]
    #[inline(always)]
    #[must_use]
    pub fn ddr4_fsp_clkchng_ack_ack_proxy(
        &mut self,
    ) -> Ddr4FspClkchngAckAckProxyW<Cfg0Ddr4FspClkchngAckProxySpec> {
        Ddr4FspClkchngAckAckProxyW::new(self, 0)
    }
}
#[doc = "CFG0_DDR4_FSP_CLKCHNG_ACK_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_ddr4_fsp_clkchng_ack_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_ddr4_fsp_clkchng_ack_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Ddr4FspClkchngAckProxySpec;
impl crate::RegisterSpec for Cfg0Ddr4FspClkchngAckProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_ddr4_fsp_clkchng_ack_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Ddr4FspClkchngAckProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_ddr4_fsp_clkchng_ack_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Ddr4FspClkchngAckProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_DDR4_FSP_CLKCHNG_ACK_PROXY to value 0"]
impl crate::Resettable for Cfg0Ddr4FspClkchngAckProxySpec {
    const RESET_VALUE: u32 = 0;
}
