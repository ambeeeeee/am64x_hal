#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_prs1` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtCfg_RegsPrs1Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_CFG__REGS_prs1` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtCfg_RegsPrs1Spec>;
#[doc = "Field `SYNC_PORT1_COL` reader - 0:0\\]
sync_port1_col"]
pub type SyncPort1ColR = crate::BitReader;
#[doc = "Field `SYNC_PORT1_COL` writer - 0:0\\]
sync_port1_col"]
pub type SyncPort1ColW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNC_PORT1_CRS` reader - 1:1\\]
sync_port1_crs"]
pub type SyncPort1CrsR = crate::BitReader;
#[doc = "Field `SYNC_PORT1_CRS` writer - 1:1\\]
sync_port1_crs"]
pub type SyncPort1CrsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
sync_port1_col"]
    #[inline(always)]
    pub fn sync_port1_col(&self) -> SyncPort1ColR {
        SyncPort1ColR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
sync_port1_crs"]
    #[inline(always)]
    pub fn sync_port1_crs(&self) -> SyncPort1CrsR {
        SyncPort1CrsR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
sync_port1_col"]
    #[inline(always)]
    #[must_use]
    pub fn sync_port1_col(&mut self) -> SyncPort1ColW<Pr1MiiRt_Pr1MiiRtCfg_RegsPrs1Spec> {
        SyncPort1ColW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
sync_port1_crs"]
    #[inline(always)]
    #[must_use]
    pub fn sync_port1_crs(&mut self) -> SyncPort1CrsW<Pr1MiiRt_Pr1MiiRtCfg_RegsPrs1Spec> {
        SyncPort1CrsW::new(self, 1)
    }
}
#[doc = "MIIPortStatus1Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_cfg__regs_prs1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_cfg__regs_prs1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtCfg_RegsPrs1Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtCfg_RegsPrs1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_cfg__regs_prs1::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtCfg_RegsPrs1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_cfg__regs_prs1::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtCfg_RegsPrs1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_CFG__REGS_prs1 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtCfg_RegsPrs1Spec {
    const RESET_VALUE: u32 = 0;
}
