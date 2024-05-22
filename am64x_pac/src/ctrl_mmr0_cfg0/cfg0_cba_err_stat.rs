#[doc = "Register `CFG0_CBA_ERR_STAT` reader"]
pub type R = crate::R<Cfg0CbaErrStatSpec>;
#[doc = "Register `CFG0_CBA_ERR_STAT` writer"]
pub type W = crate::W<Cfg0CbaErrStatSpec>;
#[doc = "Field `CBA_ERR_STAT_MAIN_CBA_ERR` reader - "]
pub type CbaErrStatMainCbaErrR = crate::BitReader;
#[doc = "Field `CBA_ERR_STAT_MAIN_CBA_ERR` writer - "]
pub type CbaErrStatMainCbaErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBA_ERR_STAT_MAIN_INFRA_CBA_ERR` reader - "]
pub type CbaErrStatMainInfraCbaErrR = crate::BitReader;
#[doc = "Field `CBA_ERR_STAT_MAIN_INFRA_CBA_ERR` writer - "]
pub type CbaErrStatMainInfraCbaErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBA_ERR_STAT_MCU_CBA_ERR` reader - "]
pub type CbaErrStatMcuCbaErrR = crate::BitReader;
#[doc = "Field `CBA_ERR_STAT_MCU_CBA_ERR` writer - "]
pub type CbaErrStatMcuCbaErrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBA_ERR_STAT_DBG_CBA_ERR` reader - "]
pub type CbaErrStatDbgCbaErrR = crate::BitReader;
#[doc = "Field `CBA_ERR_STAT_DBG_CBA_ERR` writer - "]
pub type CbaErrStatDbgCbaErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cba_err_stat_main_cba_err(&self) -> CbaErrStatMainCbaErrR {
        CbaErrStatMainCbaErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cba_err_stat_main_infra_cba_err(&self) -> CbaErrStatMainInfraCbaErrR {
        CbaErrStatMainInfraCbaErrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cba_err_stat_mcu_cba_err(&self) -> CbaErrStatMcuCbaErrR {
        CbaErrStatMcuCbaErrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn cba_err_stat_dbg_cba_err(&self) -> CbaErrStatDbgCbaErrR {
        CbaErrStatDbgCbaErrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cba_err_stat_main_cba_err(&mut self) -> CbaErrStatMainCbaErrW<Cfg0CbaErrStatSpec> {
        CbaErrStatMainCbaErrW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cba_err_stat_main_infra_cba_err(
        &mut self,
    ) -> CbaErrStatMainInfraCbaErrW<Cfg0CbaErrStatSpec> {
        CbaErrStatMainInfraCbaErrW::new(self, 1)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cba_err_stat_mcu_cba_err(&mut self) -> CbaErrStatMcuCbaErrW<Cfg0CbaErrStatSpec> {
        CbaErrStatMcuCbaErrW::new(self, 16)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn cba_err_stat_dbg_cba_err(&mut self) -> CbaErrStatDbgCbaErrW<Cfg0CbaErrStatSpec> {
        CbaErrStatDbgCbaErrW::new(self, 31)
    }
}
#[doc = "CFG0_CBA_ERR_STAT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_cba_err_stat::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_cba_err_stat::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0CbaErrStatSpec;
impl crate::RegisterSpec for Cfg0CbaErrStatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_cba_err_stat::R`](R) reader structure"]
impl crate::Readable for Cfg0CbaErrStatSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_cba_err_stat::W`](W) writer structure"]
impl crate::Writable for Cfg0CbaErrStatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_CBA_ERR_STAT to value 0"]
impl crate::Resettable for Cfg0CbaErrStatSpec {
    const RESET_VALUE: u32 = 0;
}
