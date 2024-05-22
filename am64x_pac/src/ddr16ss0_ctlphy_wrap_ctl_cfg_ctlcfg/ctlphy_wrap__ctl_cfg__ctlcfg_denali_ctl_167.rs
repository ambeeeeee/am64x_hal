#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_167` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl167Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_167` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl167Spec>;
#[doc = "Field `TDFI_LP_RESP` reader - 2:0\\]
Defines the DFI tLP_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_lp_req assertion and a dfi_lp_ack assertion."]
pub type TdfiLpRespR = crate::FieldReader;
#[doc = "Field `TDFI_LP_RESP` writer - 2:0\\]
Defines the DFI tLP_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_lp_req assertion and a dfi_lp_ack assertion."]
pub type TdfiLpRespW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_STATE` reader - 14:8\\]
Low power state status parameter. Bits \\[5:0\\]
indicate the current low power state and bit \\[6\\]
set indicates that status bits are valid. READ-ONLY"]
pub type LpStateR = crate::FieldReader;
#[doc = "Field `LP_STATE` writer - 14:8\\]
Low power state status parameter. Bits \\[5:0\\]
indicate the current low power state and bit \\[6\\]
set indicates that status bits are valid. READ-ONLY"]
pub type LpStateW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `LP_AUTO_ENTRY_EN` reader - 19:16\\]
Enable auto entry into each of the low power states when the associated idle timer expires. Bit \\[0\\]
controls power-down, bit \\[1\\]
controls self-refresh long or self-refresh power-down long, bit \\[2\\]
controls self-refresh long with memory and controller clock gating or self-refresh power-down long with memory and controller clock gating, and bit \\[3\\]
controls self-refresh short or self-refresh power-down short. Set each bit to 1 to enable."]
pub type LpAutoEntryEnR = crate::FieldReader;
#[doc = "Field `LP_AUTO_ENTRY_EN` writer - 19:16\\]
Enable auto entry into each of the low power states when the associated idle timer expires. Bit \\[0\\]
controls power-down, bit \\[1\\]
controls self-refresh long or self-refresh power-down long, bit \\[2\\]
controls self-refresh long with memory and controller clock gating or self-refresh power-down long with memory and controller clock gating, and bit \\[3\\]
controls self-refresh short or self-refresh power-down short. Set each bit to 1 to enable."]
pub type LpAutoEntryEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LP_AUTO_EXIT_EN` reader - 27:24\\]
Enable auto exit from each of the low power states when a read or write command enters the command queue. Bit \\[0\\]
controls power-down, bit \\[1\\]
controls self-refresh long or self-refresh power-down long, bit \\[2\\]
controls self-refresh long with memory and controller clock gating or self-refresh power-down long with memory and controller clock gating, and bit \\[3\\]
controls self-refresh short or self-refresh power-down short. Set each bit to 1 to enable."]
pub type LpAutoExitEnR = crate::FieldReader;
#[doc = "Field `LP_AUTO_EXIT_EN` writer - 27:24\\]
Enable auto exit from each of the low power states when a read or write command enters the command queue. Bit \\[0\\]
controls power-down, bit \\[1\\]
controls self-refresh long or self-refresh power-down long, bit \\[2\\]
controls self-refresh long with memory and controller clock gating or self-refresh power-down long with memory and controller clock gating, and bit \\[3\\]
controls self-refresh short or self-refresh power-down short. Set each bit to 1 to enable."]
pub type LpAutoExitEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Defines the DFI tLP_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_lp_req assertion and a dfi_lp_ack assertion."]
    #[inline(always)]
    pub fn tdfi_lp_resp(&self) -> TdfiLpRespR {
        TdfiLpRespR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Low power state status parameter. Bits \\[5:0\\]
indicate the current low power state and bit \\[6\\]
set indicates that status bits are valid. READ-ONLY"]
    #[inline(always)]
    pub fn lp_state(&self) -> LpStateR {
        LpStateR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Enable auto entry into each of the low power states when the associated idle timer expires. Bit \\[0\\]
controls power-down, bit \\[1\\]
controls self-refresh long or self-refresh power-down long, bit \\[2\\]
controls self-refresh long with memory and controller clock gating or self-refresh power-down long with memory and controller clock gating, and bit \\[3\\]
controls self-refresh short or self-refresh power-down short. Set each bit to 1 to enable."]
    #[inline(always)]
    pub fn lp_auto_entry_en(&self) -> LpAutoEntryEnR {
        LpAutoEntryEnR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Enable auto exit from each of the low power states when a read or write command enters the command queue. Bit \\[0\\]
controls power-down, bit \\[1\\]
controls self-refresh long or self-refresh power-down long, bit \\[2\\]
controls self-refresh long with memory and controller clock gating or self-refresh power-down long with memory and controller clock gating, and bit \\[3\\]
controls self-refresh short or self-refresh power-down short. Set each bit to 1 to enable."]
    #[inline(always)]
    pub fn lp_auto_exit_en(&self) -> LpAutoExitEnR {
        LpAutoExitEnR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Defines the DFI tLP_RESP timing parameter \\[in DFI clocks\\], the maximum cycles between a dfi_lp_req assertion and a dfi_lp_ack assertion."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_lp_resp(&mut self) -> TdfiLpRespW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl167Spec> {
        TdfiLpRespW::new(self, 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Low power state status parameter. Bits \\[5:0\\]
indicate the current low power state and bit \\[6\\]
set indicates that status bits are valid. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn lp_state(&mut self) -> LpStateW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl167Spec> {
        LpStateW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Enable auto entry into each of the low power states when the associated idle timer expires. Bit \\[0\\]
controls power-down, bit \\[1\\]
controls self-refresh long or self-refresh power-down long, bit \\[2\\]
controls self-refresh long with memory and controller clock gating or self-refresh power-down long with memory and controller clock gating, and bit \\[3\\]
controls self-refresh short or self-refresh power-down short. Set each bit to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_auto_entry_en(&mut self) -> LpAutoEntryEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl167Spec> {
        LpAutoEntryEnW::new(self, 16)
    }
    #[doc = "Bits 24:27 - 27:24\\]
Enable auto exit from each of the low power states when a read or write command enters the command queue. Bit \\[0\\]
controls power-down, bit \\[1\\]
controls self-refresh long or self-refresh power-down long, bit \\[2\\]
controls self-refresh long with memory and controller clock gating or self-refresh power-down long with memory and controller clock gating, and bit \\[3\\]
controls self-refresh short or self-refresh power-down short. Set each bit to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_auto_exit_en(&mut self) -> LpAutoExitEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl167Spec> {
        LpAutoExitEnW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_167\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_167::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_167::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl167Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl167Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_167::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl167Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_167::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl167Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_167 to value 0x6400"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl167Spec {
    const RESET_VALUE: u32 = 0x6400;
}
