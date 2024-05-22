#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_prs0` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtCfg_RegsPrs0Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_prs0` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtCfg_RegsPrs0Spec>;
#[doc = "Field `SYNC_PORT0_COL` reader - 0:0\\]
sync_port0_col"]
pub type SyncPort0ColR = crate::BitReader;
#[doc = "Field `SYNC_PORT0_COL` writer - 0:0\\]
sync_port0_col"]
pub type SyncPort0ColW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC_PORT0_CRS` reader - 1:1\\]
sync_port0_crs"]
pub type SyncPort0CrsR = crate::BitReader;
#[doc = "Field `SYNC_PORT0_CRS` writer - 1:1\\]
sync_port0_crs"]
pub type SyncPort0CrsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
sync_port0_col"]
    #[inline(always)]
    pub fn sync_port0_col(&self) -> SyncPort0ColR {
        SyncPort0ColR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
sync_port0_crs"]
    #[inline(always)]
    pub fn sync_port0_crs(&self) -> SyncPort0CrsR {
        SyncPort0CrsR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
sync_port0_col"]
    #[inline(always)]
    #[must_use]
    pub fn sync_port0_col(&mut self) -> SyncPort0ColW<Pr1MiiRt_Pr1MiiRtCfg_RegsPrs0Spec> {
        SyncPort0ColW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
sync_port0_crs"]
    #[inline(always)]
    #[must_use]
    pub fn sync_port0_crs(&mut self) -> SyncPort0CrsW<Pr1MiiRt_Pr1MiiRtCfg_RegsPrs0Spec> {
        SyncPort0CrsW::new(self, 1)
    }
}
#[doc = "MIIPortStatus0Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_prs0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_prs0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtCfg_RegsPrs0Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtCfg_RegsPrs0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_cfg__regs_prs0::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtCfg_RegsPrs0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_cfg__regs_prs0::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtCfg_RegsPrs0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_CFG__REGS_prs0 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtCfg_RegsPrs0Spec {
    const RESET_VALUE: u32 = 0;
}
