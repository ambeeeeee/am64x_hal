#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_107` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl107Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_107` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl107Spec>;
#[doc = "Field `DFS_CMD` reader - 4:0\\]
lt Currently not supported gt DFS software command request interface. Bit \\[0\\]
sends the DFS exit request when set. Bit \\[1\\]
sends the DFS enter request when set. Bit \\[2\\]
tells the controller to gate the memory clock before handing control to the PHY during a DFS operation. Bits \\[4"]
pub type DfsCmdR = crate::FieldReader;
#[doc = "Field `DFS_CMD` writer - 4:0\\]
lt Currently not supported gt DFS software command request interface. Bit \\[0\\]
sends the DFS exit request when set. Bit \\[1\\]
sends the DFS enter request when set. Bit \\[2\\]
tells the controller to gate the memory clock before handing control to the PHY during a DFS operation. Bits \\[4"]
pub type DfsCmdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DFS_STATUS` reader - 14:8\\]
Contains status and interrupt information related to DFS. Bit \\[0\\]
set indicates that the DFS request from the hardware interface was ignored because param_dfs_enable was zero or because another HWI-initiated DFS operation was already in progress. Bit \\[1\\]
set indicates that a DFS operation initiated by the hardware interface was terminated because the PHY did not deassert dfi_init_complete within tDFI_INIT_START after the controller asserted dfi_init_start. Bit \\[2\\]
set indicates that a hardware interface initiated DFS operation completed successfully. Bit \\[3\\]
set indicates that the DFS command on the software interface was ignored either because param_dfs_enable was zero or because another SW-initiated DFS operation was already in progress. Bit \\[4\\]
set indicates that a DFS operation initiated by the software interface was terminated because the PHY did not deassert dfi_init_complete within tDFI_INIT_START after the controller asserted dfi_init_start. Bit \\[5\\]
set indicates that a software initiated DFS operation completed successfully. Bit \\[6\\]
set indicates that the DFS logic entered the software wait state after a DFS operation completed. Clear this bit to allow the logic to exit this wait state."]
pub type DfsStatusR = crate::FieldReader;
#[doc = "Field `DFS_STATUS` writer - 14:8\\]
Contains status and interrupt information related to DFS. Bit \\[0\\]
set indicates that the DFS request from the hardware interface was ignored because param_dfs_enable was zero or because another HWI-initiated DFS operation was already in progress. Bit \\[1\\]
set indicates that a DFS operation initiated by the hardware interface was terminated because the PHY did not deassert dfi_init_complete within tDFI_INIT_START after the controller asserted dfi_init_start. Bit \\[2\\]
set indicates that a hardware interface initiated DFS operation completed successfully. Bit \\[3\\]
set indicates that the DFS command on the software interface was ignored either because param_dfs_enable was zero or because another SW-initiated DFS operation was already in progress. Bit \\[4\\]
set indicates that a DFS operation initiated by the software interface was terminated because the PHY did not deassert dfi_init_complete within tDFI_INIT_START after the controller asserted dfi_init_start. Bit \\[5\\]
set indicates that a software initiated DFS operation completed successfully. Bit \\[6\\]
set indicates that the DFS logic entered the software wait state after a DFS operation completed. Clear this bit to allow the logic to exit this wait state."]
pub type DfsStatusW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DFS_ZQ_EN` reader - 16:16\\]
Enables ZQ calibration across all ranks during a DFS exit. Set to 1 to enable. Not valid when operating in ZQ background mode."]
pub type DfsZqEnR = crate::BitReader;
#[doc = "Field `DFS_ZQ_EN` writer - 16:16\\]
Enables ZQ calibration across all ranks during a DFS exit. Set to 1 to enable. Not valid when operating in ZQ background mode."]
pub type DfsZqEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
lt Currently not supported gt DFS software command request interface. Bit \\[0\\]
sends the DFS exit request when set. Bit \\[1\\]
sends the DFS enter request when set. Bit \\[2\\]
tells the controller to gate the memory clock before handing control to the PHY during a DFS operation. Bits \\[4"]
    #[inline(always)]
    pub fn dfs_cmd(&self) -> DfsCmdR {
        DfsCmdR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Contains status and interrupt information related to DFS. Bit \\[0\\]
set indicates that the DFS request from the hardware interface was ignored because param_dfs_enable was zero or because another HWI-initiated DFS operation was already in progress. Bit \\[1\\]
set indicates that a DFS operation initiated by the hardware interface was terminated because the PHY did not deassert dfi_init_complete within tDFI_INIT_START after the controller asserted dfi_init_start. Bit \\[2\\]
set indicates that a hardware interface initiated DFS operation completed successfully. Bit \\[3\\]
set indicates that the DFS command on the software interface was ignored either because param_dfs_enable was zero or because another SW-initiated DFS operation was already in progress. Bit \\[4\\]
set indicates that a DFS operation initiated by the software interface was terminated because the PHY did not deassert dfi_init_complete within tDFI_INIT_START after the controller asserted dfi_init_start. Bit \\[5\\]
set indicates that a software initiated DFS operation completed successfully. Bit \\[6\\]
set indicates that the DFS logic entered the software wait state after a DFS operation completed. Clear this bit to allow the logic to exit this wait state."]
    #[inline(always)]
    pub fn dfs_status(&self) -> DfsStatusR {
        DfsStatusR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enables ZQ calibration across all ranks during a DFS exit. Set to 1 to enable. Not valid when operating in ZQ background mode."]
    #[inline(always)]
    pub fn dfs_zq_en(&self) -> DfsZqEnR {
        DfsZqEnR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
lt Currently not supported gt DFS software command request interface. Bit \\[0\\]
sends the DFS exit request when set. Bit \\[1\\]
sends the DFS enter request when set. Bit \\[2\\]
tells the controller to gate the memory clock before handing control to the PHY during a DFS operation. Bits \\[4"]
    #[inline(always)]
    #[must_use]
    pub fn dfs_cmd(&mut self) -> DfsCmdW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl107Spec> {
        DfsCmdW::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Contains status and interrupt information related to DFS. Bit \\[0\\]
set indicates that the DFS request from the hardware interface was ignored because param_dfs_enable was zero or because another HWI-initiated DFS operation was already in progress. Bit \\[1\\]
set indicates that a DFS operation initiated by the hardware interface was terminated because the PHY did not deassert dfi_init_complete within tDFI_INIT_START after the controller asserted dfi_init_start. Bit \\[2\\]
set indicates that a hardware interface initiated DFS operation completed successfully. Bit \\[3\\]
set indicates that the DFS command on the software interface was ignored either because param_dfs_enable was zero or because another SW-initiated DFS operation was already in progress. Bit \\[4\\]
set indicates that a DFS operation initiated by the software interface was terminated because the PHY did not deassert dfi_init_complete within tDFI_INIT_START after the controller asserted dfi_init_start. Bit \\[5\\]
set indicates that a software initiated DFS operation completed successfully. Bit \\[6\\]
set indicates that the DFS logic entered the software wait state after a DFS operation completed. Clear this bit to allow the logic to exit this wait state."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_status(&mut self) -> DfsStatusW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl107Spec> {
        DfsStatusW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Enables ZQ calibration across all ranks during a DFS exit. Set to 1 to enable. Not valid when operating in ZQ background mode."]
    #[inline(always)]
    #[must_use]
    pub fn dfs_zq_en(&mut self) -> DfsZqEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl107Spec> {
        DfsZqEnW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_107\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_107::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_107::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl107Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl107Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_107::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl107Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_107::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl107Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_107 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl107Spec {
    const RESET_VALUE: u32 = 0;
}
