#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_CTRL_5_REG` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgPhyCtrl5RegSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_PHY_CTRL_5_REG` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgPhyCtrl5RegSpec>;
#[doc = "Field `CLKBUFSEL` reader - 2:0\\]
Clock Delay Buffer Select. Selects one of the eight taps in the CLK Delay Buffer based on PVT variation."]
pub type ClkbufselR = crate::FieldReader;
#[doc = "Field `CLKBUFSEL` writer - 2:0\\]
Clock Delay Buffer Select. Selects one of the eight taps in the CLK Delay Buffer based on PVT variation."]
pub type ClkbufselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FRQSEL` reader - 10:8\\]
Select the frequency range of DLL operation: 0 => 200MHz to 170 MHz, 1 => 170MHz to 140 MHz, 2 => 140MHz to 110 MHz, 3 => 110MHz to 80MHz, 4 => 80MHz to 50 MHz, 5 => 275Mhz to 250MHz, 6 => 250MHz to 225MHz, 7 => 225MHz to 200MHz."]
pub type FrqselR = crate::FieldReader;
#[doc = "Field `FRQSEL` writer - 10:8\\]
Select the frequency range of DLL operation: 0 => 200MHz to 170 MHz, 1 => 170MHz to 140 MHz, 2 => 140MHz to 110 MHz, 3 => 110MHz to 80MHz, 4 => 80MHz to 50 MHz, 5 => 275Mhz to 250MHz, 6 => 250MHz to 225MHz, 7 => 225MHz to 200MHz."]
pub type FrqselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SELDLYRXCLK` reader - 16:16\\]
Select the Delay chain based rxclk. Enables the RX clock based delay chain rather than analog DLL based delay chain."]
pub type SeldlyrxclkR = crate::BitReader;
#[doc = "Field `SELDLYRXCLK` writer - 16:16\\]
Select the Delay chain based rxclk. Enables the RX clock based delay chain rather than analog DLL based delay chain."]
pub type SeldlyrxclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELDLYTXCLK` reader - 17:17\\]
Select the Delay chain based txclk. Enables the TX clock based delay chain rather than analog DLL based delay chain."]
pub type SeldlytxclkR = crate::BitReader;
#[doc = "Field `SELDLYTXCLK` writer - 17:17\\]
Select the Delay chain based txclk. Enables the TX clock based delay chain rather than analog DLL based delay chain."]
pub type SeldlytxclkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Clock Delay Buffer Select. Selects one of the eight taps in the CLK Delay Buffer based on PVT variation."]
    #[inline(always)]
    pub fn clkbufsel(&self) -> ClkbufselR {
        ClkbufselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Select the frequency range of DLL operation: 0 => 200MHz to 170 MHz, 1 => 170MHz to 140 MHz, 2 => 140MHz to 110 MHz, 3 => 110MHz to 80MHz, 4 => 80MHz to 50 MHz, 5 => 275Mhz to 250MHz, 6 => 250MHz to 225MHz, 7 => 225MHz to 200MHz."]
    #[inline(always)]
    pub fn frqsel(&self) -> FrqselR {
        FrqselR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Select the Delay chain based rxclk. Enables the RX clock based delay chain rather than analog DLL based delay chain."]
    #[inline(always)]
    pub fn seldlyrxclk(&self) -> SeldlyrxclkR {
        SeldlyrxclkR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Select the Delay chain based txclk. Enables the TX clock based delay chain rather than analog DLL based delay chain."]
    #[inline(always)]
    pub fn seldlytxclk(&self) -> SeldlytxclkR {
        SeldlytxclkR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Clock Delay Buffer Select. Selects one of the eight taps in the CLK Delay Buffer based on PVT variation."]
    #[inline(always)]
    #[must_use]
    pub fn clkbufsel(&mut self) -> ClkbufselW<Regs_SsCfg_SscfgPhyCtrl5RegSpec> {
        ClkbufselW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Select the frequency range of DLL operation: 0 => 200MHz to 170 MHz, 1 => 170MHz to 140 MHz, 2 => 140MHz to 110 MHz, 3 => 110MHz to 80MHz, 4 => 80MHz to 50 MHz, 5 => 275Mhz to 250MHz, 6 => 250MHz to 225MHz, 7 => 225MHz to 200MHz."]
    #[inline(always)]
    #[must_use]
    pub fn frqsel(&mut self) -> FrqselW<Regs_SsCfg_SscfgPhyCtrl5RegSpec> {
        FrqselW::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Select the Delay chain based rxclk. Enables the RX clock based delay chain rather than analog DLL based delay chain."]
    #[inline(always)]
    #[must_use]
    pub fn seldlyrxclk(&mut self) -> SeldlyrxclkW<Regs_SsCfg_SscfgPhyCtrl5RegSpec> {
        SeldlyrxclkW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Select the Delay chain based txclk. Enables the TX clock based delay chain rather than analog DLL based delay chain."]
    #[inline(always)]
    #[must_use]
    pub fn seldlytxclk(&mut self) -> SeldlytxclkW<Regs_SsCfg_SscfgPhyCtrl5RegSpec> {
        SeldlytxclkW::new(self, 17)
    }
}
#[doc = "The PHY Control 5 Register contains various fields to control the ports on the Arasan eMMC/SD PHY. For detailed functionality of the Arasan eMMC/SD PHY control ports please refer to its specification listed in Section 1.2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_phy_ctrl_5_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_phy_ctrl_5_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgPhyCtrl5RegSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgPhyCtrl5RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_phy_ctrl_5_reg::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgPhyCtrl5RegSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_phy_ctrl_5_reg::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgPhyCtrl5RegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_PHY_CTRL_5_REG to value 0"]
impl crate::Resettable for Regs_SsCfg_SscfgPhyCtrl5RegSpec {
    const RESET_VALUE: u32 = 0;
}
