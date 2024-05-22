#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_291` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl291Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_291` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl291Spec>;
#[doc = "Field `AREF_CMD_MAX_PER_TREFI` reader - 3:0\\]
Sets the maximum number of auto-refreshes that will be executed in a TREFI period - both normal and high priority. This does not prevent refreshes generated by sub-task requests such as a self-refresh exit and enter."]
pub type ArefCmdMaxPerTrefiR = crate::FieldReader;
#[doc = "Field `AREF_CMD_MAX_PER_TREFI` writer - 3:0\\]
Sets the maximum number of auto-refreshes that will be executed in a TREFI period - both normal and high priority. This does not prevent refreshes generated by sub-task requests such as a self-refresh exit and enter."]
pub type ArefCmdMaxPerTrefiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ZQCS_OPT_THRESHOLD` reader - 10:8\\]
Number of clocks before ZQCS expires when the ZQ task will deassert its request for optimal command to command turn-around timing."]
pub type ZqcsOptThresholdR = crate::FieldReader;
#[doc = "Field `ZQCS_OPT_THRESHOLD` writer - 10:8\\]
Number of clocks before ZQCS expires when the ZQ task will deassert its request for optimal command to command turn-around timing."]
pub type ZqcsOptThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ZQ_CALSTART_NORM_THRESHOLD_F0` reader - 31:16\\]
ZQ START number of long counts until the normal priority request is asserted. This value should be scaled based on the number of ranks \\[chip selects\\]
the controller handles. The more chip selects there are, the more rotations there are to go through, and the smaller this threshold should be. FC=0"]
pub type ZqCalstartNormThresholdF0R = crate::FieldReader<u16>;
#[doc = "Field `ZQ_CALSTART_NORM_THRESHOLD_F0` writer - 31:16\\]
ZQ START number of long counts until the normal priority request is asserted. This value should be scaled based on the number of ranks \\[chip selects\\]
the controller handles. The more chip selects there are, the more rotations there are to go through, and the smaller this threshold should be. FC=0"]
pub type ZqCalstartNormThresholdF0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Sets the maximum number of auto-refreshes that will be executed in a TREFI period - both normal and high priority. This does not prevent refreshes generated by sub-task requests such as a self-refresh exit and enter."]
    #[inline(always)]
    pub fn aref_cmd_max_per_trefi(&self) -> ArefCmdMaxPerTrefiR {
        ArefCmdMaxPerTrefiR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Number of clocks before ZQCS expires when the ZQ task will deassert its request for optimal command to command turn-around timing."]
    #[inline(always)]
    pub fn zqcs_opt_threshold(&self) -> ZqcsOptThresholdR {
        ZqcsOptThresholdR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
ZQ START number of long counts until the normal priority request is asserted. This value should be scaled based on the number of ranks \\[chip selects\\]
the controller handles. The more chip selects there are, the more rotations there are to go through, and the smaller this threshold should be. FC=0"]
    #[inline(always)]
    pub fn zq_calstart_norm_threshold_f0(&self) -> ZqCalstartNormThresholdF0R {
        ZqCalstartNormThresholdF0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Sets the maximum number of auto-refreshes that will be executed in a TREFI period - both normal and high priority. This does not prevent refreshes generated by sub-task requests such as a self-refresh exit and enter."]
    #[inline(always)]
    #[must_use]
    pub fn aref_cmd_max_per_trefi(
        &mut self,
    ) -> ArefCmdMaxPerTrefiW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl291Spec> {
        ArefCmdMaxPerTrefiW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Number of clocks before ZQCS expires when the ZQ task will deassert its request for optimal command to command turn-around timing."]
    #[inline(always)]
    #[must_use]
    pub fn zqcs_opt_threshold(
        &mut self,
    ) -> ZqcsOptThresholdW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl291Spec> {
        ZqcsOptThresholdW::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
ZQ START number of long counts until the normal priority request is asserted. This value should be scaled based on the number of ranks \\[chip selects\\]
the controller handles. The more chip selects there are, the more rotations there are to go through, and the smaller this threshold should be. FC=0"]
    #[inline(always)]
    #[must_use]
    pub fn zq_calstart_norm_threshold_f0(
        &mut self,
    ) -> ZqCalstartNormThresholdF0W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl291Spec> {
        ZqCalstartNormThresholdF0W::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_291\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_291::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_291::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl291Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl291Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_291::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl291Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_291::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl291Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_291 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl291Spec {
    const RESET_VALUE: u32 = 0;
}