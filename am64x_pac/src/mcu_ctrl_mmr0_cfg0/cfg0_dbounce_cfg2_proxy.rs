#[doc = "Register `CFG0_DBOUNCE_CFG2_PROXY` reader"]
pub type R = crate::R<Cfg0DbounceCfg2ProxySpec>;
#[doc = "Register `CFG0_DBOUNCE_CFG2_PROXY` writer"]
pub type W = crate::W<Cfg0DbounceCfg2ProxySpec>;
#[doc = "Field `DBOUNCE_CFG2_DB_CFG_PROXY` reader - 5:0\\]
Configures the debounce period used for I/Os with debounce_sel2 actived. See the AM64xx IO Integration Spec for Detail (https://pds.design.ti.com/cgi-bin/viewdocs?pds_bgid=21&amp;docconfigid=736&amp;filtertitle=2622&amp;filteritem=13912)"]
pub type DbounceCfg2DbCfgProxyR = crate::FieldReader;
#[doc = "Field `DBOUNCE_CFG2_DB_CFG_PROXY` writer - 5:0\\]
Configures the debounce period used for I/Os with debounce_sel2 actived. See the AM64xx IO Integration Spec for Detail (https://pds.design.ti.com/cgi-bin/viewdocs?pds_bgid=21&amp;docconfigid=736&amp;filtertitle=2622&amp;filteritem=13912)"]
pub type DbounceCfg2DbCfgProxyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Configures the debounce period used for I/Os with debounce_sel2 actived. See the AM64xx IO Integration Spec for Detail (https://pds.design.ti.com/cgi-bin/viewdocs?pds_bgid=21&amp;docconfigid=736&amp;filtertitle=2622&amp;filteritem=13912)"]
    #[inline(always)]
    pub fn dbounce_cfg2_db_cfg_proxy(&self) -> DbounceCfg2DbCfgProxyR {
        DbounceCfg2DbCfgProxyR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Configures the debounce period used for I/Os with debounce_sel2 actived. See the AM64xx IO Integration Spec for Detail (https://pds.design.ti.com/cgi-bin/viewdocs?pds_bgid=21&amp;docconfigid=736&amp;filtertitle=2622&amp;filteritem=13912)"]
    #[inline(always)]
    #[must_use]
    pub fn dbounce_cfg2_db_cfg_proxy(
        &mut self,
    ) -> DbounceCfg2DbCfgProxyW<Cfg0DbounceCfg2ProxySpec> {
        DbounceCfg2DbCfgProxyW::new(self, 0)
    }
}
#[doc = "CFG0_DBOUNCE_CFG2_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_dbounce_cfg2_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_dbounce_cfg2_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0DbounceCfg2ProxySpec;
impl crate::RegisterSpec for Cfg0DbounceCfg2ProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_dbounce_cfg2_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0DbounceCfg2ProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_dbounce_cfg2_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0DbounceCfg2ProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_DBOUNCE_CFG2_PROXY to value 0"]
impl crate::Resettable for Cfg0DbounceCfg2ProxySpec {
    const RESET_VALUE: u32 = 0;
}
