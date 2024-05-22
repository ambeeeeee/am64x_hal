#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_383` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl383Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_383` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl383Spec>;
#[doc = "Field `TDQSCK_MIN_F2` reader - 2:0\\]
Additional delay needed for tDQSCK. FC=2"]
pub type TdqsckMinF2R = crate::FieldReader;
#[doc = "Field `TDQSCK_MIN_F2` writer - 2:0\\]
Additional delay needed for tDQSCK. FC=2"]
pub type TdqsckMinF2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `AXI0_ALL_STROBES_USED_ENABLE` reader - 8:8\\]
Enables use of the AWALLSTRB signal for AXI port 0. Set to 1 to enable."]
pub type Axi0AllStrobesUsedEnableR = crate::BitReader;
#[doc = "Field `AXI0_ALL_STROBES_USED_ENABLE` writer - 8:8\\]
Enables use of the AWALLSTRB signal for AXI port 0. Set to 1 to enable."]
pub type Axi0AllStrobesUsedEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI0_FIXED_PORT_PRIORITY_ENABLE` reader - 16:16\\]
Defines the priority control for AXI port 0 as per-port or per-command. Set to 1 for per-port with priority defined through the AXI.0._R_PRIORITY and AXI.0._W_PRIORITY parameters. Clear to 0 for per-command."]
pub type Axi0FixedPortPriorityEnableR = crate::BitReader;
#[doc = "Field `AXI0_FIXED_PORT_PRIORITY_ENABLE` writer - 16:16\\]
Defines the priority control for AXI port 0 as per-port or per-command. Set to 1 for per-port with priority defined through the AXI.0._R_PRIORITY and AXI.0._W_PRIORITY parameters. Clear to 0 for per-command."]
pub type Axi0FixedPortPriorityEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXI0_R_PRIORITY` reader - 26:24\\]
Priority of read commands from AXI port 0. 0 is the highest priority. This may only be changed before initialization begins or when the controller is quiescent, there is no data in the port FIFOs, and the AXI0_FIXED_PORT_PRIORITY_ENABLE parameter is low."]
pub type Axi0RPriorityR = crate::FieldReader;
#[doc = "Field `AXI0_R_PRIORITY` writer - 26:24\\]
Priority of read commands from AXI port 0. 0 is the highest priority. This may only be changed before initialization begins or when the controller is quiescent, there is no data in the port FIFOs, and the AXI0_FIXED_PORT_PRIORITY_ENABLE parameter is low."]
pub type Axi0RPriorityW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Additional delay needed for tDQSCK. FC=2"]
    #[inline(always)]
    pub fn tdqsck_min_f2(&self) -> TdqsckMinF2R {
        TdqsckMinF2R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables use of the AWALLSTRB signal for AXI port 0. Set to 1 to enable."]
    #[inline(always)]
    pub fn axi0_all_strobes_used_enable(&self) -> Axi0AllStrobesUsedEnableR {
        Axi0AllStrobesUsedEnableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
Defines the priority control for AXI port 0 as per-port or per-command. Set to 1 for per-port with priority defined through the AXI.0._R_PRIORITY and AXI.0._W_PRIORITY parameters. Clear to 0 for per-command."]
    #[inline(always)]
    pub fn axi0_fixed_port_priority_enable(&self) -> Axi0FixedPortPriorityEnableR {
        Axi0FixedPortPriorityEnableR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Priority of read commands from AXI port 0. 0 is the highest priority. This may only be changed before initialization begins or when the controller is quiescent, there is no data in the port FIFOs, and the AXI0_FIXED_PORT_PRIORITY_ENABLE parameter is low."]
    #[inline(always)]
    pub fn axi0_r_priority(&self) -> Axi0RPriorityR {
        Axi0RPriorityR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Additional delay needed for tDQSCK. FC=2"]
    #[inline(always)]
    #[must_use]
    pub fn tdqsck_min_f2(&mut self) -> TdqsckMinF2W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl383Spec> {
        TdqsckMinF2W::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Enables use of the AWALLSTRB signal for AXI port 0. Set to 1 to enable."]
    #[inline(always)]
    #[must_use]
    pub fn axi0_all_strobes_used_enable(
        &mut self,
    ) -> Axi0AllStrobesUsedEnableW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl383Spec> {
        Axi0AllStrobesUsedEnableW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Defines the priority control for AXI port 0 as per-port or per-command. Set to 1 for per-port with priority defined through the AXI.0._R_PRIORITY and AXI.0._W_PRIORITY parameters. Clear to 0 for per-command."]
    #[inline(always)]
    #[must_use]
    pub fn axi0_fixed_port_priority_enable(
        &mut self,
    ) -> Axi0FixedPortPriorityEnableW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl383Spec> {
        Axi0FixedPortPriorityEnableW::new(self, 16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Priority of read commands from AXI port 0. 0 is the highest priority. This may only be changed before initialization begins or when the controller is quiescent, there is no data in the port FIFOs, and the AXI0_FIXED_PORT_PRIORITY_ENABLE parameter is low."]
    #[inline(always)]
    #[must_use]
    pub fn axi0_r_priority(&mut self) -> Axi0RPriorityW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl383Spec> {
        Axi0RPriorityW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_383\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_383::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_383::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl383Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl383Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_383::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl383Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_383::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl383Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_383 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl383Spec {
    const RESET_VALUE: u32 = 0;
}
