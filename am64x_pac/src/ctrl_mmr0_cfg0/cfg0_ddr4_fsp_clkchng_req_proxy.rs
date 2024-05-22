#[doc = "Register `CFG0_DDR4_FSP_CLKCHNG_REQ_PROXY` reader"]
pub type R = crate::R<Cfg0Ddr4FspClkchngReqProxySpec>;
#[doc = "Register `CFG0_DDR4_FSP_CLKCHNG_REQ_PROXY` writer"]
pub type W = crate::W<Cfg0Ddr4FspClkchngReqProxySpec>;
#[doc = "Field `DDR4_FSP_CLKCHNG_REQ_REQ_TYPE_PROXY` reader - 1:0\\]
Frequency request type"]
pub type Ddr4FspClkchngReqReqTypeProxyR = crate::FieldReader;
#[doc = "Field `DDR4_FSP_CLKCHNG_REQ_REQ_TYPE_PROXY` writer - 1:0\\]
Frequency request type"]
pub type Ddr4FspClkchngReqReqTypeProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DDR4_FSP_CLKCHNG_REQ_REQ_PROXY` reader - 7:7\\]
DDR Controller FSP clock change request"]
pub type Ddr4FspClkchngReqReqProxyR = crate::BitReader;
#[doc = "Field `DDR4_FSP_CLKCHNG_REQ_REQ_PROXY` writer - 7:7\\]
DDR Controller FSP clock change request"]
pub type Ddr4FspClkchngReqReqProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Frequency request type"]
    #[inline(always)]
    pub fn ddr4_fsp_clkchng_req_req_type_proxy(&self) -> Ddr4FspClkchngReqReqTypeProxyR {
        Ddr4FspClkchngReqReqTypeProxyR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
DDR Controller FSP clock change request"]
    #[inline(always)]
    pub fn ddr4_fsp_clkchng_req_req_proxy(&self) -> Ddr4FspClkchngReqReqProxyR {
        Ddr4FspClkchngReqReqProxyR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Frequency request type"]
    #[inline(always)]
    #[must_use]
    pub fn ddr4_fsp_clkchng_req_req_type_proxy(
        &mut self,
    ) -> Ddr4FspClkchngReqReqTypeProxyW<Cfg0Ddr4FspClkchngReqProxySpec> {
        Ddr4FspClkchngReqReqTypeProxyW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
DDR Controller FSP clock change request"]
    #[inline(always)]
    #[must_use]
    pub fn ddr4_fsp_clkchng_req_req_proxy(
        &mut self,
    ) -> Ddr4FspClkchngReqReqProxyW<Cfg0Ddr4FspClkchngReqProxySpec> {
        Ddr4FspClkchngReqReqProxyW::new(self, 7)
    }
}
#[doc = "CFG0_DDR4_FSP_CLKCHNG_REQ_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_ddr4_fsp_clkchng_req_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_ddr4_fsp_clkchng_req_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Ddr4FspClkchngReqProxySpec;
impl crate::RegisterSpec for Cfg0Ddr4FspClkchngReqProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_ddr4_fsp_clkchng_req_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0Ddr4FspClkchngReqProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_ddr4_fsp_clkchng_req_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0Ddr4FspClkchngReqProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_DDR4_FSP_CLKCHNG_REQ_PROXY to value 0"]
impl crate::Resettable for Cfg0Ddr4FspClkchngReqProxySpec {
    const RESET_VALUE: u32 = 0;
}
