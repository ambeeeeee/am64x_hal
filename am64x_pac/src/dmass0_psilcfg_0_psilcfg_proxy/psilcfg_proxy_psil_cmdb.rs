#[doc = "Register `PSILCFG_PROXY_PSIL_CMDB` reader"]
pub type R = crate::R<PsilcfgProxyPsilCmdbSpec>;
#[doc = "Register `PSILCFG_PROXY_PSIL_CMDB` writer"]
pub type W = crate::W<PsilcfgProxyPsilCmdbSpec>;
#[doc = "Field `PROXY_ADDRESS` reader - 15:0\\]
Word (32-bit) address within thread configuration space for transaction"]
pub type ProxyAddressR = crate::FieldReader<u16>;
#[doc = "Field `PROXY_ADDRESS` writer - 15:0\\]
Word (32-bit) address within thread configuration space for transaction"]
pub type ProxyAddressW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PROXY_BYTEN` reader - 31:28\\]
Byte enables to use for configuration read or write"]
pub type ProxyBytenR = crate::FieldReader;
#[doc = "Field `PROXY_BYTEN` writer - 31:28\\]
Byte enables to use for configuration read or write"]
pub type ProxyBytenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Word (32-bit) address within thread configuration space for transaction"]
    #[inline(always)]
    pub fn proxy_address(&self) -> ProxyAddressR {
        ProxyAddressR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Byte enables to use for configuration read or write"]
    #[inline(always)]
    pub fn proxy_byten(&self) -> ProxyBytenR {
        ProxyBytenR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Word (32-bit) address within thread configuration space for transaction"]
    #[inline(always)]
    #[must_use]
    pub fn proxy_address(&mut self) -> ProxyAddressW<PsilcfgProxyPsilCmdbSpec> {
        ProxyAddressW::new(self, 0)
    }
    #[doc = "Bits 28:31 - 31:28\\]
Byte enables to use for configuration read or write"]
    #[inline(always)]
    #[must_use]
    pub fn proxy_byten(&mut self) -> ProxyBytenW<PsilcfgProxyPsilCmdbSpec> {
        ProxyBytenW::new(self, 28)
    }
}
#[doc = "The Command Register B contains the byte enables and word address for the configuration transaction.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psilcfg_proxy_psil_cmdb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psilcfg_proxy_psil_cmdb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsilcfgProxyPsilCmdbSpec;
impl crate::RegisterSpec for PsilcfgProxyPsilCmdbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psilcfg_proxy_psil_cmdb::R`](R) reader structure"]
impl crate::Readable for PsilcfgProxyPsilCmdbSpec {}
#[doc = "`write(|w| ..)` method takes [`psilcfg_proxy_psil_cmdb::W`](W) writer structure"]
impl crate::Writable for PsilcfgProxyPsilCmdbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSILCFG_PROXY_PSIL_CMDB to value 0"]
impl crate::Resettable for PsilcfgProxyPsilCmdbSpec {
    const RESET_VALUE: u32 = 0;
}
