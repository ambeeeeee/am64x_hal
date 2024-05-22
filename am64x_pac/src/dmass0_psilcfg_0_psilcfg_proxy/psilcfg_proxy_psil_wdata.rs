#[doc = "Register `PSILCFG_PROXY_PSIL_WDATA` reader"]
pub type R = crate::R<PsilcfgProxyPsilWdataSpec>;
#[doc = "Register `PSILCFG_PROXY_PSIL_WDATA` writer"]
pub type W = crate::W<PsilcfgProxyPsilWdataSpec>;
#[doc = "Field `PROXY_WDATA` reader - 31:0\\]
Configuration data word to be written"]
pub type ProxyWdataR = crate::FieldReader<u32>;
#[doc = "Field `PROXY_WDATA` writer - 31:0\\]
Configuration data word to be written"]
pub type ProxyWdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Configuration data word to be written"]
    #[inline(always)]
    pub fn proxy_wdata(&self) -> ProxyWdataR {
        ProxyWdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Configuration data word to be written"]
    #[inline(always)]
    #[must_use]
    pub fn proxy_wdata(&mut self) -> ProxyWdataW<PsilcfgProxyPsilWdataSpec> {
        ProxyWdataW::new(self, 0)
    }
}
#[doc = "The Write Data Register contains the data which is to be written during the configuration transaction.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psilcfg_proxy_psil_wdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psilcfg_proxy_psil_wdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsilcfgProxyPsilWdataSpec;
impl crate::RegisterSpec for PsilcfgProxyPsilWdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psilcfg_proxy_psil_wdata::R`](R) reader structure"]
impl crate::Readable for PsilcfgProxyPsilWdataSpec {}
#[doc = "`write(|w| ..)` method takes [`psilcfg_proxy_psil_wdata::W`](W) writer structure"]
impl crate::Writable for PsilcfgProxyPsilWdataSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSILCFG_PROXY_PSIL_WDATA to value 0"]
impl crate::Resettable for PsilcfgProxyPsilWdataSpec {
    const RESET_VALUE: u32 = 0;
}
