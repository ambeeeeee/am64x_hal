#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_544` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy544Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_544` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy544Spec>;
#[doc = "Field `PHY_ADR_TSEL_SELECT_0` reader - 7:0\\]
Tsel select values for address slice 0."]
pub type PhyAdrTselSelect0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_TSEL_SELECT_0` writer - 7:0\\]
Tsel select values for address slice 0."]
pub type PhyAdrTselSelect0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_PAD_ADR_IO_CFG_0` reader - 18:8\\]
Controls I/O pads for address pad for address slice 0. Bits \\[10:5\\]
= Park value, bits \\[4\\]
park override, bits \\[2:0\\]
clk divider."]
pub type PhyPadAdrIoCfg0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_PAD_ADR_IO_CFG_0` writer - 18:8\\]
Controls I/O pads for address pad for address slice 0. Bits \\[10:5\\]
= Park value, bits \\[4\\]
park override, bits \\[2:0\\]
clk divider."]
pub type PhyPadAdrIoCfg0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_PAD_ADR_RX_PCLK_CLK_SEL_0` reader - 26:24\\]
Reserved for address slice 0."]
pub type PhyPadAdrRxPclkClkSel0R = crate::FieldReader;
#[doc = "Field `PHY_PAD_ADR_RX_PCLK_CLK_SEL_0` writer - 26:24\\]
Reserved for address slice 0."]
pub type PhyPadAdrRxPclkClkSel0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Tsel select values for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_tsel_select_0(&self) -> PhyAdrTselSelect0R {
        PhyAdrTselSelect0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:18 - 18:8\\]
Controls I/O pads for address pad for address slice 0. Bits \\[10:5\\]
= Park value, bits \\[4\\]
park override, bits \\[2:0\\]
clk divider."]
    #[inline(always)]
    pub fn phy_pad_adr_io_cfg_0(&self) -> PhyPadAdrIoCfg0R {
        PhyPadAdrIoCfg0R::new(((self.bits >> 8) & 0x07ff) as u16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Reserved for address slice 0."]
    #[inline(always)]
    pub fn phy_pad_adr_rx_pclk_clk_sel_0(&self) -> PhyPadAdrRxPclkClkSel0R {
        PhyPadAdrRxPclkClkSel0R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Tsel select values for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_tsel_select_0(
        &mut self,
    ) -> PhyAdrTselSelect0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy544Spec> {
        PhyAdrTselSelect0W::new(self, 0)
    }
    #[doc = "Bits 8:18 - 18:8\\]
Controls I/O pads for address pad for address slice 0. Bits \\[10:5\\]
= Park value, bits \\[4\\]
park override, bits \\[2:0\\]
clk divider."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_adr_io_cfg_0(
        &mut self,
    ) -> PhyPadAdrIoCfg0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy544Spec> {
        PhyPadAdrIoCfg0W::new(self, 8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Reserved for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_adr_rx_pclk_clk_sel_0(
        &mut self,
    ) -> PhyPadAdrRxPclkClkSel0W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy544Spec> {
        PhyPadAdrRxPclkClkSel0W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_544\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_544::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_544::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy544Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy544Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_544::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy544Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_544::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy544Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_544 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy544Spec {
    const RESET_VALUE: u32 = 0;
}
