#[doc = "Register `CFG0_CHNG_DDR4_FSP_REQ_PROXY` reader"]
pub type R = crate::R<Cfg0ChngDdr4FspReqProxySpec>;
#[doc = "Register `CFG0_CHNG_DDR4_FSP_REQ_PROXY` writer"]
pub type W = crate::W<Cfg0ChngDdr4FspReqProxySpec>;
#[doc = "Field `CHNG_DDR4_FSP_REQ_REQ_TYPE_PROXY` reader - 1:0\\]
Frequency request type"]
pub type ChngDdr4FspReqReqTypeProxyR = crate::FieldReader;
#[doc = "Field `CHNG_DDR4_FSP_REQ_REQ_TYPE_PROXY` writer - 1:0\\]
Frequency request type"]
pub type ChngDdr4FspReqReqTypeProxyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHNG_DDR4_FSP_REQ_REQ_PROXY` reader - 8:8\\]
Initiate FSP frequency change"]
pub type ChngDdr4FspReqReqProxyR = crate::BitReader;
#[doc = "Field `CHNG_DDR4_FSP_REQ_REQ_PROXY` writer - 8:8\\]
Initiate FSP frequency change"]
pub type ChngDdr4FspReqReqProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Frequency request type"]
    #[inline(always)]
    pub fn chng_ddr4_fsp_req_req_type_proxy(&self) -> ChngDdr4FspReqReqTypeProxyR {
        ChngDdr4FspReqReqTypeProxyR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Initiate FSP frequency change"]
    #[inline(always)]
    pub fn chng_ddr4_fsp_req_req_proxy(&self) -> ChngDdr4FspReqReqProxyR {
        ChngDdr4FspReqReqProxyR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Frequency request type"]
    #[inline(always)]
    #[must_use]
    pub fn chng_ddr4_fsp_req_req_type_proxy(
        &mut self,
    ) -> ChngDdr4FspReqReqTypeProxyW<Cfg0ChngDdr4FspReqProxySpec> {
        ChngDdr4FspReqReqTypeProxyW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Initiate FSP frequency change"]
    #[inline(always)]
    #[must_use]
    pub fn chng_ddr4_fsp_req_req_proxy(
        &mut self,
    ) -> ChngDdr4FspReqReqProxyW<Cfg0ChngDdr4FspReqProxySpec> {
        ChngDdr4FspReqReqProxyW::new(self, 8)
    }
}
#[doc = "CFG0_CHNG_DDR4_FSP_REQ_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_chng_ddr4_fsp_req_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_chng_ddr4_fsp_req_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0ChngDdr4FspReqProxySpec;
impl crate::RegisterSpec for Cfg0ChngDdr4FspReqProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_chng_ddr4_fsp_req_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0ChngDdr4FspReqProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_chng_ddr4_fsp_req_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0ChngDdr4FspReqProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_CHNG_DDR4_FSP_REQ_PROXY to value 0"]
impl crate::Resettable for Cfg0ChngDdr4FspReqProxySpec {
    const RESET_VALUE: u32 = 0;
}
