#[doc = "Register `CFG0_RST_STAT_PROXY` reader"]
pub type R = crate::R<Cfg0RstStatProxySpec>;
#[doc = "Register `CFG0_RST_STAT_PROXY` writer"]
pub type W = crate::W<Cfg0RstStatProxySpec>;
#[doc = "Field `RST_STAT_MAIN_RESETSTATZ_PROXY` reader - 0:0\\]
Status of Main Domain Reset:"]
pub type RstStatMainResetstatzProxyR = crate::BitReader;
#[doc = "Field `RST_STAT_MAIN_RESETSTATZ_PROXY` writer - 0:0\\]
Status of Main Domain Reset:"]
pub type RstStatMainResetstatzProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Status of Main Domain Reset:"]
    #[inline(always)]
    pub fn rst_stat_main_resetstatz_proxy(&self) -> RstStatMainResetstatzProxyR {
        RstStatMainResetstatzProxyR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Status of Main Domain Reset:"]
    #[inline(always)]
    #[must_use]
    pub fn rst_stat_main_resetstatz_proxy(
        &mut self,
    ) -> RstStatMainResetstatzProxyW<Cfg0RstStatProxySpec> {
        RstStatMainResetstatzProxyW::new(self, 0)
    }
}
#[doc = "CFG0_RST_STAT_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_rst_stat_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_rst_stat_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0RstStatProxySpec;
impl crate::RegisterSpec for Cfg0RstStatProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_rst_stat_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0RstStatProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_rst_stat_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0RstStatProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_RST_STAT_PROXY to value 0"]
impl crate::Resettable for Cfg0RstStatProxySpec {
    const RESET_VALUE: u32 = 0;
}
