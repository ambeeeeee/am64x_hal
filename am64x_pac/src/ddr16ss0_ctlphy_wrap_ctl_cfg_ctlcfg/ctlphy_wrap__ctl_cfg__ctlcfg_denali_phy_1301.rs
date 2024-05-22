#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1301` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1301Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1301` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1301Spec>;
#[doc = "Field `PHY_STATIC_TOG_CONTROL` reader - 15:0\\]
Clock divider to create toggle signal. Use long counter as the base."]
pub type PhyStaticTogControlR = crate::FieldReader<u16>;
#[doc = "Field `PHY_STATIC_TOG_CONTROL` writer - 15:0\\]
Clock divider to create toggle signal. Use long counter as the base."]
pub type PhyStaticTogControlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PHY_ADRCTL_STATIC_TOG_DISABLE` reader - 19:16\\]
Control to disable toggle during static activity. bit0: Write path delay line disable; bit1: clock disable; bit2: adrctl master delay line disable \\[if exists\\]; bit3: adrctl misc core clk disable.\\[if exists\\]"]
pub type PhyAdrctlStaticTogDisableR = crate::FieldReader;
#[doc = "Field `PHY_ADRCTL_STATIC_TOG_DISABLE` writer - 19:16\\]
Control to disable toggle during static activity. bit0: Write path delay line disable; bit1: clock disable; bit2: adrctl master delay line disable \\[if exists\\]; bit3: adrctl misc core clk disable.\\[if exists\\]"]
pub type PhyAdrctlStaticTogDisableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_MEMCLK_STATIC_TOG_DISABLE` reader - 24:24\\]
Control to disable toggle during static activity. bit0: clock disable."]
pub type PhyMemclkStaticTogDisableR = crate::BitReader;
#[doc = "Field `PHY_MEMCLK_STATIC_TOG_DISABLE` writer - 24:24\\]
Control to disable toggle during static activity. bit0: clock disable."]
pub type PhyMemclkStaticTogDisableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Clock divider to create toggle signal. Use long counter as the base."]
    #[inline(always)]
    pub fn phy_static_tog_control(&self) -> PhyStaticTogControlR {
        PhyStaticTogControlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Control to disable toggle during static activity. bit0: Write path delay line disable; bit1: clock disable; bit2: adrctl master delay line disable \\[if exists\\]; bit3: adrctl misc core clk disable.\\[if exists\\]"]
    #[inline(always)]
    pub fn phy_adrctl_static_tog_disable(&self) -> PhyAdrctlStaticTogDisableR {
        PhyAdrctlStaticTogDisableR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Control to disable toggle during static activity. bit0: clock disable."]
    #[inline(always)]
    pub fn phy_memclk_static_tog_disable(&self) -> PhyMemclkStaticTogDisableR {
        PhyMemclkStaticTogDisableR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Clock divider to create toggle signal. Use long counter as the base."]
    #[inline(always)]
    #[must_use]
    pub fn phy_static_tog_control(
        &mut self,
    ) -> PhyStaticTogControlW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1301Spec> {
        PhyStaticTogControlW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Control to disable toggle during static activity. bit0: Write path delay line disable; bit1: clock disable; bit2: adrctl master delay line disable \\[if exists\\]; bit3: adrctl misc core clk disable.\\[if exists\\]"]
    #[inline(always)]
    #[must_use]
    pub fn phy_adrctl_static_tog_disable(
        &mut self,
    ) -> PhyAdrctlStaticTogDisableW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1301Spec> {
        PhyAdrctlStaticTogDisableW::new(self, 16)
    }
    #[doc = "Bit 24 - 24:24\\]
Control to disable toggle during static activity. bit0: clock disable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_memclk_static_tog_disable(
        &mut self,
    ) -> PhyMemclkStaticTogDisableW<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1301Spec> {
        PhyMemclkStaticTogDisableW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1301\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1301::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1301::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1301Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1301Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1301::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1301Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1301::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1301Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1301 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1301Spec {
    const RESET_VALUE: u32 = 0;
}
