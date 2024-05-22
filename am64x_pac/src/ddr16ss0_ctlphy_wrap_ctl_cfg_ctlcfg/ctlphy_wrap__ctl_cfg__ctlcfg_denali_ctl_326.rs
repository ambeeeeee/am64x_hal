#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_326` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl326Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_326` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl326Spec>;
#[doc = "Field `NUM_Q_ENTRIES_ACT_DISABLE` reader - 4:0\\]
Number of queue entries in which ACT requests will be disabled. Programming to X will disable ACT requests from the X entries lowest in the command queue."]
pub type NumQEntriesActDisableR = crate::FieldReader;
#[doc = "Field `NUM_Q_ENTRIES_ACT_DISABLE` writer - 4:0\\]
Number of queue entries in which ACT requests will be disabled. Programming to X will disable ACT requests from the X entries lowest in the command queue."]
pub type NumQEntriesActDisableW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SWAP_EN` reader - 8:8\\]
Enable command swapping logic in execution unit. Set to 1 to enable."]
pub type SwapEnR = crate::BitReader;
#[doc = "Field `SWAP_EN` writer - 8:8\\]
Enable command swapping logic in execution unit. Set to 1 to enable."]
pub type SwapEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_RD_INTERLEAVE` reader - 16:16\\]
Disable read data interleaving for commands from the same port, regardless of the requestor ID."]
pub type DisableRdInterleaveR = crate::BitReader;
#[doc = "Field `DISABLE_RD_INTERLEAVE` writer - 16:16\\]
Disable read data interleaving for commands from the same port, regardless of the requestor ID."]
pub type DisableRdInterleaveW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INHIBIT_DRAM_CMD` reader - 25:24\\]
Inhibit command types from being executed from the command queue. Clear to 0 to enable any command, program to 1 to inhibit read/write and bank commands, program to 2 to inhibit MRR and peripheral MRR commands, or program to 3 to inhibit MRR and read/write commands."]
pub type InhibitDramCmdR = crate::FieldReader;
#[doc = "Field `INHIBIT_DRAM_CMD` writer - 25:24\\]
Inhibit command types from being executed from the command queue. Clear to 0 to enable any command, program to 1 to inhibit read/write and bank commands, program to 2 to inhibit MRR and peripheral MRR commands, or program to 3 to inhibit MRR and read/write commands."]
pub type InhibitDramCmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Number of queue entries in which ACT requests will be disabled. Programming to X will disable ACT requests from the X entries lowest in the command queue."]
    #[inline(always)]
    pub fn num_q_entries_act_disable(&self) -> NumQEntriesActDisableR {
        NumQEntriesActDisableR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable command swapping logic in execution unit. Set to 1 to enable."]
    #[inline(always)]
    pub fn swap_en(&self) -> SwapEnR {
        SwapEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Disable read data interleaving for commands from the same port, regardless of the requestor ID."]
    #[inline(always)]
    pub fn disable_rd_interleave(&self) -> DisableRdInterleaveR {
        DisableRdInterleaveR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Inhibit command types from being executed from the command queue. Clear to 0 to enable any command, program to 1 to inhibit read/write and bank commands, program to 2 to inhibit MRR and peripheral MRR commands, or program to 3 to inhibit MRR and read/write commands."]
    #[inline(always)]
    pub fn inhibit_dram_cmd(&self) -> InhibitDramCmdR {
        InhibitDramCmdR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Number of queue entries in which ACT requests will be disabled. Programming to X will disable ACT requests from the X entries lowest in the command queue."]
    #[inline(always)]
    #[must_use]
    pub fn num_q_entries_act_disable(
        &mut self,
    ) -> NumQEntriesActDisableW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl326Spec> {
        NumQEntriesActDisableW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enable command swapping logic in execution unit. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn swap_en(&mut self) -> SwapEnW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl326Spec> {
        SwapEnW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Disable read data interleaving for commands from the same port, regardless of the requestor ID."]
    #[inline(always)]
    #[must_use]
    pub fn disable_rd_interleave(
        &mut self,
    ) -> DisableRdInterleaveW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl326Spec> {
        DisableRdInterleaveW::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
Inhibit command types from being executed from the command queue. Clear to 0 to enable any command, program to 1 to inhibit read/write and bank commands, program to 2 to inhibit MRR and peripheral MRR commands, or program to 3 to inhibit MRR and read/write commands."]
    #[inline(always)]
    #[must_use]
    pub fn inhibit_dram_cmd(
        &mut self,
    ) -> InhibitDramCmdW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl326Spec> {
        InhibitDramCmdW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_326\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_326::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_326::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl326Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl326Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_326::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl326Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_326::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl326Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_326 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl326Spec {
    const RESET_VALUE: u32 = 0;
}
