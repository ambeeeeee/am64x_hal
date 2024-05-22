#[doc = "Register `CFG0_CBA_ERR_STAT_PROXY` reader"]
pub type R = crate::R<Cfg0CbaErrStatProxySpec>;
#[doc = "Register `CFG0_CBA_ERR_STAT_PROXY` writer"]
pub type W = crate::W<Cfg0CbaErrStatProxySpec>;
#[doc = "Field `CBA_ERR_STAT_MAIN_CBA_ERR_PROXY` reader - "]
pub type CbaErrStatMainCbaErrProxyR = crate::BitReader;
#[doc = "Field `CBA_ERR_STAT_MAIN_CBA_ERR_PROXY` writer - "]
pub type CbaErrStatMainCbaErrProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBA_ERR_STAT_MAIN_INFRA_CBA_ERR_PROXY` reader - "]
pub type CbaErrStatMainInfraCbaErrProxyR = crate::BitReader;
#[doc = "Field `CBA_ERR_STAT_MAIN_INFRA_CBA_ERR_PROXY` writer - "]
pub type CbaErrStatMainInfraCbaErrProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBA_ERR_STAT_MCU_CBA_ERR_PROXY` reader - "]
pub type CbaErrStatMcuCbaErrProxyR = crate::BitReader;
#[doc = "Field `CBA_ERR_STAT_MCU_CBA_ERR_PROXY` writer - "]
pub type CbaErrStatMcuCbaErrProxyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBA_ERR_STAT_DBG_CBA_ERR_PROXY` reader - "]
pub type CbaErrStatDbgCbaErrProxyR = crate::BitReader;
#[doc = "Field `CBA_ERR_STAT_DBG_CBA_ERR_PROXY` writer - "]
pub type CbaErrStatDbgCbaErrProxyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cba_err_stat_main_cba_err_proxy(&self) -> CbaErrStatMainCbaErrProxyR {
        CbaErrStatMainCbaErrProxyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cba_err_stat_main_infra_cba_err_proxy(&self) -> CbaErrStatMainInfraCbaErrProxyR {
        CbaErrStatMainInfraCbaErrProxyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cba_err_stat_mcu_cba_err_proxy(&self) -> CbaErrStatMcuCbaErrProxyR {
        CbaErrStatMcuCbaErrProxyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cba_err_stat_dbg_cba_err_proxy(&self) -> CbaErrStatDbgCbaErrProxyR {
        CbaErrStatDbgCbaErrProxyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cba_err_stat_main_cba_err_proxy(
        &mut self,
    ) -> CbaErrStatMainCbaErrProxyW<Cfg0CbaErrStatProxySpec> {
        CbaErrStatMainCbaErrProxyW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cba_err_stat_main_infra_cba_err_proxy(
        &mut self,
    ) -> CbaErrStatMainInfraCbaErrProxyW<Cfg0CbaErrStatProxySpec> {
        CbaErrStatMainInfraCbaErrProxyW::new(self, 1)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cba_err_stat_mcu_cba_err_proxy(
        &mut self,
    ) -> CbaErrStatMcuCbaErrProxyW<Cfg0CbaErrStatProxySpec> {
        CbaErrStatMcuCbaErrProxyW::new(self, 16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn cba_err_stat_dbg_cba_err_proxy(
        &mut self,
    ) -> CbaErrStatDbgCbaErrProxyW<Cfg0CbaErrStatProxySpec> {
        CbaErrStatDbgCbaErrProxyW::new(self, 31)
    }
}
#[doc = "CFG0_CBA_ERR_STAT_PROXY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_cba_err_stat_proxy::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_cba_err_stat_proxy::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0CbaErrStatProxySpec;
impl crate::RegisterSpec for Cfg0CbaErrStatProxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_cba_err_stat_proxy::R`](R) reader structure"]
impl crate::Readable for Cfg0CbaErrStatProxySpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_cba_err_stat_proxy::W`](W) writer structure"]
impl crate::Writable for Cfg0CbaErrStatProxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_CBA_ERR_STAT_PROXY to value 0"]
impl crate::Resettable for Cfg0CbaErrStatProxySpec {
    const RESET_VALUE: u32 = 0;
}
