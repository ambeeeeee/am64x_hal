#[doc = "Register `PSILCFG_PROXY_PSIL_TO` reader"]
pub type R = crate::R<PsilcfgProxyPsilToSpec>;
#[doc = "Register `PSILCFG_PROXY_PSIL_TO` writer"]
pub type W = crate::W<PsilcfgProxyPsilToSpec>;
#[doc = "Field `TOUT_CNT` reader - 15:0\\]
Timeout period. Specifies how many cycles to wait before closing up a conifiguration read or write transaction and asserting the tout bit"]
pub type ToutCntR = crate::FieldReader<u16>;
#[doc = "Field `TOUT_CNT` writer - 15:0\\]
Timeout period. Specifies how many cycles to wait before closing up a conifiguration read or write transaction and asserting the tout bit"]
pub type ToutCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TOUT` reader - 31:31\\]
Timeout occurred. When set indicates that a timeout has occurred on a config access. Once set, this bit is persistent until manually cleared"]
pub type ToutR = crate::BitReader;
#[doc = "Field `TOUT` writer - 31:31\\]
Timeout occurred. When set indicates that a timeout has occurred on a config access. Once set, this bit is persistent until manually cleared"]
pub type ToutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Timeout period. Specifies how many cycles to wait before closing up a conifiguration read or write transaction and asserting the tout bit"]
    #[inline(always)]
    pub fn tout_cnt(&self) -> ToutCntR {
        ToutCntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - 31:31\\]
Timeout occurred. When set indicates that a timeout has occurred on a config access. Once set, this bit is persistent until manually cleared"]
    #[inline(always)]
    pub fn tout(&self) -> ToutR {
        ToutR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Timeout period. Specifies how many cycles to wait before closing up a conifiguration read or write transaction and asserting the tout bit"]
    #[inline(always)]
    #[must_use]
    pub fn tout_cnt(&mut self) -> ToutCntW<PsilcfgProxyPsilToSpec> {
        ToutCntW::new(self, 0)
    }
    #[doc = "Bit 31 - 31:31\\]
Timeout occurred. When set indicates that a timeout has occurred on a config access. Once set, this bit is persistent until manually cleared"]
    #[inline(always)]
    #[must_use]
    pub fn tout(&mut self) -> ToutW<PsilcfgProxyPsilToSpec> {
        ToutW::new(self, 31)
    }
}
#[doc = "The PSI-L proxy timeout register controls the timeout watchdog and reports timeout occurrances on PSI-L configuration transactions issued by the built in PSI-L proxy.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`psilcfg_proxy_psil_to::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`psilcfg_proxy_psil_to::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsilcfgProxyPsilToSpec;
impl crate::RegisterSpec for PsilcfgProxyPsilToSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psilcfg_proxy_psil_to::R`](R) reader structure"]
impl crate::Readable for PsilcfgProxyPsilToSpec {}
#[doc = "`write(|w| ..)` method takes [`psilcfg_proxy_psil_to::W`](W) writer structure"]
impl crate::Writable for PsilcfgProxyPsilToSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PSILCFG_PROXY_PSIL_TO to value 0x1024"]
impl crate::Resettable for PsilcfgProxyPsilToSpec {
    const RESET_VALUE: u32 = 0x1024;
}
