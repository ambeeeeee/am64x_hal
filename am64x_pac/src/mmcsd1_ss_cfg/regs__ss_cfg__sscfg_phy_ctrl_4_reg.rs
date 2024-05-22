#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_CTRL_4_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgPhyCtrl4RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_CTRL_4_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgPhyCtrl4RegSpec>;
#[doc = "Field `ITAPDLYSEL` reader - 4:0\\]
Input Tap Delay Select. Manual control of the RX clock Tap Delay in the non HS200/HS400 modes."]
pub type ItapdlyselR = crate::FieldReader;
#[doc = "Field `ITAPDLYSEL` writer - 4:0\\]
Input Tap Delay Select. Manual control of the RX clock Tap Delay in the non HS200/HS400 modes."]
pub type ItapdlyselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ITAPDLYENA` reader - 8:8\\]
Input Tap Delay Enable. This is used for the manual control of the RX clock Tap Delay in non HS200/HS400 modes."]
pub type ItapdlyenaR = crate::BitReader;
#[doc = "Field `ITAPDLYENA` writer - 8:8\\]
Input Tap Delay Enable. This is used for the manual control of the RX clock Tap Delay in non HS200/HS400 modes."]
pub type ItapdlyenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAPCHGWIN` reader - 9:9\\]
Input Tap Change Window. It gets asserted by the controller while changing the itapdlysel. Used to gate of the RX clock during switching the clock source while tap is changing to avoid clock glitches."]
pub type ItapchgwinR = crate::BitReader;
#[doc = "Field `ITAPCHGWIN` writer - 9:9\\]
Input Tap Change Window. It gets asserted by the controller while changing the itapdlysel. Used to gate of the RX clock during switching the clock source while tap is changing to avoid clock glitches."]
pub type ItapchgwinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTAPDLYSEL` reader - 15:12\\]
Output Tap Delay Select. Manual control of the TX clock tap delay for clocking the final stage flops for maintaining Hold requirements on EMMC Interface."]
pub type OtapdlyselR = crate::FieldReader;
#[doc = "Field `OTAPDLYSEL` writer - 15:12\\]
Output Tap Delay Select. Manual control of the TX clock tap delay for clocking the final stage flops for maintaining Hold requirements on EMMC Interface."]
pub type OtapdlyselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OTAPDLYENA` reader - 20:20\\]
Output Tap Delay Enable. Enables manual control of the TX clock tap delay, for clocking the final stage flops for maintaining Hold requirements on EMMC Interface."]
pub type OtapdlyenaR = crate::BitReader;
#[doc = "Field `OTAPDLYENA` writer - 20:20\\]
Output Tap Delay Enable. Enables manual control of the TX clock tap delay, for clocking the final stage flops for maintaining Hold requirements on EMMC Interface."]
pub type OtapdlyenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Input Tap Delay Select. Manual control of the RX clock Tap Delay in the non HS200/HS400 modes."]
    #[inline(always)]
    pub fn itapdlysel(&self) -> ItapdlyselR {
        ItapdlyselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Input Tap Delay Enable. This is used for the manual control of the RX clock Tap Delay in non HS200/HS400 modes."]
    #[inline(always)]
    pub fn itapdlyena(&self) -> ItapdlyenaR {
        ItapdlyenaR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Input Tap Change Window. It gets asserted by the controller while changing the itapdlysel. Used to gate of the RX clock during switching the clock source while tap is changing to avoid clock glitches."]
    #[inline(always)]
    pub fn itapchgwin(&self) -> ItapchgwinR {
        ItapchgwinR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Output Tap Delay Select. Manual control of the TX clock tap delay for clocking the final stage flops for maintaining Hold requirements on EMMC Interface."]
    #[inline(always)]
    pub fn otapdlysel(&self) -> OtapdlyselR {
        OtapdlyselR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - 20:20\\]
Output Tap Delay Enable. Enables manual control of the TX clock tap delay, for clocking the final stage flops for maintaining Hold requirements on EMMC Interface."]
    #[inline(always)]
    pub fn otapdlyena(&self) -> OtapdlyenaR {
        OtapdlyenaR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Input Tap Delay Select. Manual control of the RX clock Tap Delay in the non HS200/HS400 modes."]
    #[inline(always)]
    #[must_use]
    pub fn itapdlysel(&mut self) -> ItapdlyselW<Regs_SsCfg_SscfgPhyCtrl4RegSpec> {
        ItapdlyselW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Input Tap Delay Enable. This is used for the manual control of the RX clock Tap Delay in non HS200/HS400 modes."]
    #[inline(always)]
    #[must_use]
    pub fn itapdlyena(&mut self) -> ItapdlyenaW<Regs_SsCfg_SscfgPhyCtrl4RegSpec> {
        ItapdlyenaW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Input Tap Change Window. It gets asserted by the controller while changing the itapdlysel. Used to gate of the RX clock during switching the clock source while tap is changing to avoid clock glitches."]
    #[inline(always)]
    #[must_use]
    pub fn itapchgwin(&mut self) -> ItapchgwinW<Regs_SsCfg_SscfgPhyCtrl4RegSpec> {
        ItapchgwinW::new(self, 9)
    }
    #[doc = "Bits 12:15 - 15:12\\]
Output Tap Delay Select. Manual control of the TX clock tap delay for clocking the final stage flops for maintaining Hold requirements on EMMC Interface."]
    #[inline(always)]
    #[must_use]
    pub fn otapdlysel(&mut self) -> OtapdlyselW<Regs_SsCfg_SscfgPhyCtrl4RegSpec> {
        OtapdlyselW::new(self, 12)
    }
    #[doc = "Bit 20 - 20:20\\]
Output Tap Delay Enable. Enables manual control of the TX clock tap delay, for clocking the final stage flops for maintaining Hold requirements on EMMC Interface."]
    #[inline(always)]
    #[must_use]
    pub fn otapdlyena(&mut self) -> OtapdlyenaW<Regs_SsCfg_SscfgPhyCtrl4RegSpec> {
        OtapdlyenaW::new(self, 20)
    }
}
#[doc = "The PHY Control 4 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_ctrl_4_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_ctrl_4_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgPhyCtrl4RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgPhyCtrl4RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_phy_ctrl_4_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgPhyCtrl4RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_phy_ctrl_4_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgPhyCtrl4RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_PHY_CTRL_4_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgPhyCtrl4RegSpec {
    const RESET_VALUE: u32 = 0;
}
