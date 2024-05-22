#[doc = "Register `CFG0_CHNG_DDR4_FSP_ACK_PROXY` reader"]
pub type R = crate::R<Cfg0ChngDdr4FspAckProxySpec>;
#[doc = "Register `CFG0_CHNG_DDR4_FSP_ACK_PROXY` writer"]
pub type W = crate::W<Cfg0ChngDdr4FspAckProxySpec>;
#[doc = "Field `CHNG_DDR4_FSP_ACK_ERROR_PROXY` reader - 0:0\\]
Frequency change error"]
pub type ChngDdr4FspAckErrorProxyR = crate::BitReader;
#[doc = "Field `CHNG_DDR4_FSP_ACK_ERROR_PROXY` writer - 0:0\\]
Frequency change error"]
pub type ChngDdr4FspAckErrorProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNG_DDR4_FSP_ACK_ACK_PROXY` reader - 7:7\\]
Frequency change acknowledge."]
pub type ChngDdr4FspAckAckProxyR = crate::BitReader;
#[doc = "Field `CHNG_DDR4_FSP_ACK_ACK_PROXY` writer - 7:7\\]
Frequency change acknowledge."]
pub type ChngDdr4FspAckAckProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Frequency change error"]
    #[inline(always)]
    pub fn chng_ddr4_fsp_ack_error_proxy(&self) -> ChngDdr4FspAckErrorProxyR {
        ChngDdr4FspAckErrorProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Frequency change acknowledge."]
    #[inline(always)]
    pub fn chng_ddr4_fsp_ack_ack_proxy(&self) -> ChngDdr4FspAckAckProxyR {
        ChngDdr4FspAckAckProxyR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Frequency change error"]
    #[inline(always)]
    #[must_use]
    pub fn chng_ddr4_fsp_ack_error_proxy(
        &mut self,
    ) -> ChngDdr4FspAckErrorProxyW<Cfg0ChngDdr4FspAckProxySpec> {
        ChngDdr4FspAckErrorProxyW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Frequency change acknowledge."]
    #[inline(always)]
    #[must_use]
    pub fn chng_ddr4_fsp_ack_ack_proxy(
        &mut self,
    ) -> ChngDdr4FspAckAckProxyW<Cfg0ChngDdr4FspAckProxySpec> {
        ChngDdr4FspAckAckProxyW::new(self, 7)
    }
}
#[doc = "CFG0_CHNG_DDR4_FSP_ACK_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_chng_ddr4_fsp_ack_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_chng_ddr4_fsp_ack_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0ChngDdr4FspAckProxySpec;
impl crate::RegisterSpec for Cfg0ChngDdr4FspAckProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_chng_ddr4_fsp_ack_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0ChngDdr4FspAckProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_chng_ddr4_fsp_ack_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0ChngDdr4FspAckProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_CHNG_DDR4_FSP_ACK_PROXY to value 0"]
impl crate::Resettable for Cfg0ChngDdr4FspAckProxySpec {
    const RESET_VALUE: u32 = 0;
}
