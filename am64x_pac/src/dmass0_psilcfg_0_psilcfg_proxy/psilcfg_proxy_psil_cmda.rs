#[doc = "Register `PSILCFG_PROXY_PSIL_CMDA` reader"]
pub type R = crate::R<PsilcfgProxyPsilCmdaSpec>;
#[doc = "Register `PSILCFG_PROXY_PSIL_CMDA` writer"]
pub type W = crate::W<PsilcfgProxyPsilCmdaSpec>;
#[doc = "Field `PROXY_THREAD_ID` reader - 15:0\\]
Thread ID to which configuration read or write is being sent"]
pub type ProxyThreadIdR = crate::FieldReader<u16>;
#[doc = "Field `PROXY_THREAD_ID` writer - 15:0\\]
Thread ID to which configuration read or write is being sent"]
pub type ProxyThreadIdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PROXY_TOUT` reader - 29:29\\]
Indication that a timeout occurred. This bit should be written to 0 on each new transaction."]
pub type ProxyToutR = crate::BitReader;
#[doc = "Field `PROXY_TOUT` writer - 29:29\\]
Indication that a timeout occurred. This bit should be written to 0 on each new transaction."]
pub type ProxyToutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROXY_DIR` reader - 30:30\\]
Direction of configuration transaction"]
pub type ProxyDirR = crate::BitReader;
#[doc = "Field `PROXY_DIR` writer - 30:30\\]
Direction of configuration transaction"]
pub type ProxyDirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROXY_BUSY` reader - 31:31\\]
Indication that a configuration read or write is in progress"]
pub type ProxyBusyR = crate::BitReader;
#[doc = "Field `PROXY_BUSY` writer - 31:31\\]
Indication that a configuration read or write is in progress"]
pub type ProxyBusyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Thread ID to which configuration read or write is being sent"]
    #[inline(always)]
    pub fn proxy_thread_id(&self) -> ProxyThreadIdR {
        ProxyThreadIdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 29 - 29:29\\]
Indication that a timeout occurred. This bit should be written to 0 on each new transaction."]
    #[inline(always)]
    pub fn proxy_tout(&self) -> ProxyToutR {
        ProxyToutR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 30:30\\]
Direction of configuration transaction"]
    #[inline(always)]
    pub fn proxy_dir(&self) -> ProxyDirR {
        ProxyDirR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Indication that a configuration read or write is in progress"]
    #[inline(always)]
    pub fn proxy_busy(&self) -> ProxyBusyR {
        ProxyBusyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Thread ID to which configuration read or write is being sent"]
    #[inline(always)]
    #[must_use]
    pub fn proxy_thread_id(&mut self) -> ProxyThreadIdW<PsilcfgProxyPsilCmdaSpec> {
        ProxyThreadIdW::new(self, 0)
    }
    #[doc = "Bit 29 - 29:29\\]
Indication that a timeout occurred. This bit should be written to 0 on each new transaction."]
    #[inline(always)]
    #[must_use]
    pub fn proxy_tout(&mut self) -> ProxyToutW<PsilcfgProxyPsilCmdaSpec> {
        ProxyToutW::new(self, 29)
    }
    #[doc = "Bit 30 - 30:30\\]
Direction of configuration transaction"]
    #[inline(always)]
    #[must_use]
    pub fn proxy_dir(&mut self) -> ProxyDirW<PsilcfgProxyPsilCmdaSpec> {
        ProxyDirW::new(self, 30)
    }
    #[doc = "Bit 31 - 31:31\\]
Indication that a configuration read or write is in progress"]
    #[inline(always)]
    #[must_use]
    pub fn proxy_busy(&mut self) -> ProxyBusyW<PsilcfgProxyPsilCmdaSpec> {
        ProxyBusyW::new(self, 31)
    }
}
#[doc = "The Command Register A contains the busy indicator, direction, and thread number for the configuration transaction.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psilcfg_proxy_psil_cmda::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psilcfg_proxy_psil_cmda::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsilcfgProxyPsilCmdaSpec;
impl crate::RegisterSpec for PsilcfgProxyPsilCmdaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psilcfg_proxy_psil_cmda::R`](R) reader structure"]
impl crate::Readable for PsilcfgProxyPsilCmdaSpec {}
#[doc = "`write(|w| ..)` method takes [`psilcfg_proxy_psil_cmda::W`](W) writer structure"]
impl crate::Writable for PsilcfgProxyPsilCmdaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSILCFG_PROXY_PSIL_CMDA to value 0"]
impl crate::Resettable for PsilcfgProxyPsilCmdaSpec {
    const RESET_VALUE: u32 = 0;
}
