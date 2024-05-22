#[doc = "Register `CFG0_DDR4_FSP_CLKCHNG_REQ` reader"]
pub type R = crate::R<Cfg0Ddr4FspClkchngReqSpec>;
#[doc = "Register `CFG0_DDR4_FSP_CLKCHNG_REQ` writer"]
pub type W = crate::W<Cfg0Ddr4FspClkchngReqSpec>;
#[doc = "Field `DDR4_FSP_CLKCHNG_REQ_REQ_TYPE` reader - 1:0\\]
Frequency request type"]
pub type Ddr4FspClkchngReqReqTypeR = crate::FieldReader;
#[doc = "Field `DDR4_FSP_CLKCHNG_REQ_REQ_TYPE` writer - 1:0\\]
Frequency request type"]
pub type Ddr4FspClkchngReqReqTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DDR4_FSP_CLKCHNG_REQ_REQ` reader - 7:7\\]
DDR Controller FSP clock change request"]
pub type Ddr4FspClkchngReqReqR = crate::BitReader;
#[doc = "Field `DDR4_FSP_CLKCHNG_REQ_REQ` writer - 7:7\\]
DDR Controller FSP clock change request"]
pub type Ddr4FspClkchngReqReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Frequency request type"]
    #[inline(always)]
    pub fn ddr4_fsp_clkchng_req_req_type(&self) -> Ddr4FspClkchngReqReqTypeR {
        Ddr4FspClkchngReqReqTypeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
DDR Controller FSP clock change request"]
    #[inline(always)]
    pub fn ddr4_fsp_clkchng_req_req(&self) -> Ddr4FspClkchngReqReqR {
        Ddr4FspClkchngReqReqR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Frequency request type"]
    #[inline(always)]
    #[must_use]
    pub fn ddr4_fsp_clkchng_req_req_type(
        &mut self,
    ) -> Ddr4FspClkchngReqReqTypeW<Cfg0Ddr4FspClkchngReqSpec> {
        Ddr4FspClkchngReqReqTypeW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
DDR Controller FSP clock change request"]
    #[inline(always)]
    #[must_use]
    pub fn ddr4_fsp_clkchng_req_req(&mut self) -> Ddr4FspClkchngReqReqW<Cfg0Ddr4FspClkchngReqSpec> {
        Ddr4FspClkchngReqReqW::new(self, 7)
    }
}
#[doc = "CFG0_DDR4_FSP_CLKCHNG_REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_ddr4_fsp_clkchng_req::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_ddr4_fsp_clkchng_req::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0Ddr4FspClkchngReqSpec;
impl crate::RegisterSpec for Cfg0Ddr4FspClkchngReqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_ddr4_fsp_clkchng_req::R`](R) reader structure"]
impl crate::Readable for Cfg0Ddr4FspClkchngReqSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_ddr4_fsp_clkchng_req::W`](W) writer structure"]
impl crate::Writable for Cfg0Ddr4FspClkchngReqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_DDR4_FSP_CLKCHNG_REQ to value 0"]
impl crate::Resettable for Cfg0Ddr4FspClkchngReqSpec {
    const RESET_VALUE: u32 = 0;
}
