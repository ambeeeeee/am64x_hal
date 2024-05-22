#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_289` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl289Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_289` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl289Spec>;
#[doc = "Field `BIST_ERR_COUNT` reader - 11:0\\]
Indicates the number of BIST errors found when the BIST_TEST_MODE parameter is programmed to 1, 2 or 3. READ-ONLY"]
pub type BistErrCountR = crate::FieldReader<u16>;
#[doc = "Field `BIST_ERR_COUNT` writer - 11:0\\]
Indicates the number of BIST errors found when the BIST_TEST_MODE parameter is programmed to 1, 2 or 3. READ-ONLY"]
pub type BistErrCountW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIST_RET_STATE_EXIT` reader - 16:16\\]
Exit self-refresh or idle retention state, used when the BIST_TEST_MODE parameter is programmed to 2 or 3. Set to 1 to trigger. WRITE-ONLY"]
pub type BistRetStateExitR = crate::BitReader;
#[doc = "Field `BIST_RET_STATE_EXIT` writer - 16:16\\]
Exit self-refresh or idle retention state, used when the BIST_TEST_MODE parameter is programmed to 2 or 3. Set to 1 to trigger. WRITE-ONLY"]
pub type BistRetStateExitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LONG_COUNT_MASK` reader - 28:24\\]
Reduces the length of the long counter from 1024 cycles. The only supported values are 0x00 \\[1024 cycles\\], 0x10 \\[512 clocks\\], 0x18 \\[256 clocks\\], 0x1C \\[128 clocks\\], 0x1E \\[64 clocks\\]
and 0x1F \\[32 clocks\\]."]
pub type LongCountMaskR = crate::FieldReader;
#[doc = "Field `LONG_COUNT_MASK` writer - 28:24\\]
Reduces the length of the long counter from 1024 cycles. The only supported values are 0x00 \\[1024 cycles\\], 0x10 \\[512 clocks\\], 0x18 \\[256 clocks\\], 0x1C \\[128 clocks\\], 0x1E \\[64 clocks\\]
and 0x1F \\[32 clocks\\]."]
pub type LongCountMaskW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
Indicates the number of BIST errors found when the BIST_TEST_MODE parameter is programmed to 1, 2 or 3. READ-ONLY"]
    #[inline(always)]
    pub fn bist_err_count(&self) -> BistErrCountR {
        BistErrCountR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - 16:16\\]
Exit self-refresh or idle retention state, used when the BIST_TEST_MODE parameter is programmed to 2 or 3. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    pub fn bist_ret_state_exit(&self) -> BistRetStateExitR {
        BistRetStateExitR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Reduces the length of the long counter from 1024 cycles. The only supported values are 0x00 \\[1024 cycles\\], 0x10 \\[512 clocks\\], 0x18 \\[256 clocks\\], 0x1C \\[128 clocks\\], 0x1E \\[64 clocks\\]
and 0x1F \\[32 clocks\\]."]
    #[inline(always)]
    pub fn long_count_mask(&self) -> LongCountMaskR {
        LongCountMaskR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
Indicates the number of BIST errors found when the BIST_TEST_MODE parameter is programmed to 1, 2 or 3. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn bist_err_count(&mut self) -> BistErrCountW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl289Spec> {
        BistErrCountW::new(self, 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Exit self-refresh or idle retention state, used when the BIST_TEST_MODE parameter is programmed to 2 or 3. Set to 1 to trigger. WRITE-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn bist_ret_state_exit(
        &mut self,
    ) -> BistRetStateExitW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl289Spec> {
        BistRetStateExitW::new(self, 16)
    }
    #[doc = "Bits 24:28 - 28:24\\]
Reduces the length of the long counter from 1024 cycles. The only supported values are 0x00 \\[1024 cycles\\], 0x10 \\[512 clocks\\], 0x18 \\[256 clocks\\], 0x1C \\[128 clocks\\], 0x1E \\[64 clocks\\]
and 0x1F \\[32 clocks\\]."]
    #[inline(always)]
    #[must_use]
    pub fn long_count_mask(&mut self) -> LongCountMaskW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl289Spec> {
        LongCountMaskW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_289\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_289::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_289::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl289Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl289Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_289::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl289Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_289::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl289Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_289 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl289Spec {
    const RESET_VALUE: u32 = 0;
}
