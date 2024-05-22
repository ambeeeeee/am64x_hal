#[doc = "Register `CFG0_DDR4_FSP_CLKCHNG_ACK` reader"]
pub type R = crate::R<Cfg0Ddr4FspClkchngAckSpec>;
#[doc = "Register `CFG0_DDR4_FSP_CLKCHNG_ACK` writer"]
pub type W = crate::W<Cfg0Ddr4FspClkchngAckSpec>;
#[doc = "Field `DDR4_FSP_CLKCHNG_ACK_ACK` reader - 0:0\\]
DDR FSP clock change ackowledge"]
pub type Ddr4FspClkchngAckAckR = crate::BitReader;
#[doc = "Field `DDR4_FSP_CLKCHNG_ACK_ACK` writer - 0:0\\]
DDR FSP clock change ackowledge"]
pub type Ddr4FspClkchngAckAckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DDR FSP clock change ackowledge"]
    #[inline(always)]
    pub fn ddr4_fsp_clkchng_ack_ack(&self) -> Ddr4FspClkchngAckAckR {
        Ddr4FspClkchngAckAckR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DDR FSP clock change ackowledge"]
    #[inline(always)]
    #[must_use]
    pub fn ddr4_fsp_clkchng_ack_ack(&mut self) -> Ddr4FspClkchngAckAckW<Cfg0Ddr4FspClkchngAckSpec> {
        Ddr4FspClkchngAckAckW::new(self, 0)
    }
}
#[doc = "CFG0_DDR4_FSP_CLKCHNG_ACK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_ddr4_fsp_clkchng_ack::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_ddr4_fsp_clkchng_ack::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Ddr4FspClkchngAckSpec;
impl crate::RegisterSpec for Cfg0Ddr4FspClkchngAckSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_ddr4_fsp_clkchng_ack::R`](R) reader structure"]
impl crate::Readable for Cfg0Ddr4FspClkchngAckSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_ddr4_fsp_clkchng_ack::W`](W) writer structure"]
impl crate::Writable for Cfg0Ddr4FspClkchngAckSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_DDR4_FSP_CLKCHNG_ACK to value 0"]
impl crate::Resettable for Cfg0Ddr4FspClkchngAckSpec {
    const RESET_VALUE: u32 = 0;
}
