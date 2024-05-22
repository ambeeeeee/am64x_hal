#[doc = "Register `CFG0_CHNG_DDR4_FSP_REQ` reader"]
pub type R = crate::R<Cfg0ChngDdr4FspReqSpec>;
#[doc = "Register `CFG0_CHNG_DDR4_FSP_REQ` writer"]
pub type W = crate::W<Cfg0ChngDdr4FspReqSpec>;
#[doc = "Field `CHNG_DDR4_FSP_REQ_REQ_TYPE` reader - 1:0\\]
Frequency request type"]
pub type ChngDdr4FspReqReqTypeR = crate::FieldReader;
#[doc = "Field `CHNG_DDR4_FSP_REQ_REQ_TYPE` writer - 1:0\\]
Frequency request type"]
pub type ChngDdr4FspReqReqTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHNG_DDR4_FSP_REQ_REQ` reader - 8:8\\]
Initiate FSP frequency change"]
pub type ChngDdr4FspReqReqR = crate::BitReader;
#[doc = "Field `CHNG_DDR4_FSP_REQ_REQ` writer - 8:8\\]
Initiate FSP frequency change"]
pub type ChngDdr4FspReqReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
Frequency request type"]
    #[inline(always)]
    pub fn chng_ddr4_fsp_req_req_type(&self) -> ChngDdr4FspReqReqTypeR {
        ChngDdr4FspReqReqTypeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Initiate FSP frequency change"]
    #[inline(always)]
    pub fn chng_ddr4_fsp_req_req(&self) -> ChngDdr4FspReqReqR {
        ChngDdr4FspReqReqR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
Frequency request type"]
    #[inline(always)]
    #[must_use]
    pub fn chng_ddr4_fsp_req_req_type(&mut self) -> ChngDdr4FspReqReqTypeW<Cfg0ChngDdr4FspReqSpec> {
        ChngDdr4FspReqReqTypeW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Initiate FSP frequency change"]
    #[inline(always)]
    #[must_use]
    pub fn chng_ddr4_fsp_req_req(&mut self) -> ChngDdr4FspReqReqW<Cfg0ChngDdr4FspReqSpec> {
        ChngDdr4FspReqReqW::new(self, 8)
    }
}
#[doc = "CFG0_CHNG_DDR4_FSP_REQ\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_chng_ddr4_fsp_req::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_chng_ddr4_fsp_req::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0ChngDdr4FspReqSpec;
impl crate::RegisterSpec for Cfg0ChngDdr4FspReqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_chng_ddr4_fsp_req::R`](R) reader structure"]
impl crate::Readable for Cfg0ChngDdr4FspReqSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_chng_ddr4_fsp_req::W`](W) writer structure"]
impl crate::Writable for Cfg0ChngDdr4FspReqSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_CHNG_DDR4_FSP_REQ to value 0"]
impl crate::Resettable for Cfg0ChngDdr4FspReqSpec {
    const RESET_VALUE: u32 = 0;
}
