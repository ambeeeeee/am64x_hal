#[doc = "Register `PSILCFG_PROXY_PSIL_RDATA` reader"]
pub type R = crate::R<PsilcfgProxyPsilRdataSpec>;
#[doc = "Register `PSILCFG_PROXY_PSIL_RDATA` writer"]
pub type W = crate::W<PsilcfgProxyPsilRdataSpec>;
#[doc = "Field `PROXY_RDATA` reader - 31:0\\]
Configuration data word that was read"]
pub type ProxyRdataR = crate::FieldReader<u32>;
#[doc = "Field `PROXY_RDATA` writer - 31:0\\]
Configuration data word that was read"]
pub type ProxyRdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Configuration data word that was read"]
    #[inline(always)]
    pub fn proxy_rdata(&self) -> ProxyRdataR {
        ProxyRdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Configuration data word that was read"]
    #[inline(always)]
    #[must_use]
    pub fn proxy_rdata(&mut self) -> ProxyRdataW<PsilcfgProxyPsilRdataSpec> {
        ProxyRdataW::new(self, 0)
    }
}
#[doc = "The Read Data Register contains the data which which was read back during the configuration transaction.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psilcfg_proxy_psil_rdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psilcfg_proxy_psil_rdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsilcfgProxyPsilRdataSpec;
impl crate::RegisterSpec for PsilcfgProxyPsilRdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psilcfg_proxy_psil_rdata::R`](R) reader structure"]
impl crate::Readable for PsilcfgProxyPsilRdataSpec {}
#[doc = "`write(|w| ..)` method takes [`psilcfg_proxy_psil_rdata::W`](W) writer structure"]
impl crate::Writable for PsilcfgProxyPsilRdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSILCFG_PROXY_PSIL_RDATA to value 0"]
impl crate::Resettable for PsilcfgProxyPsilRdataSpec {
    const RESET_VALUE: u32 = 0;
}
