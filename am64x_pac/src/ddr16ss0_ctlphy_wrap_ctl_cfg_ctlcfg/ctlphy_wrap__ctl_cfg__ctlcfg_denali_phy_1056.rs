#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1056` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1056Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1056` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1056Spec>;
#[doc = "Field `PHY_ADR_TSEL_SELECT_2` reader - 7:0\\]
Tsel select values for address slice 2."]
pub type PhyAdrTselSelect2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_TSEL_SELECT_2` writer - 7:0\\]
Tsel select values for address slice 2."]
pub type PhyAdrTselSelect2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PHY_PAD_ADR_IO_CFG_2` reader - 18:8\\]
Controls I/O pads for address pad for address slice 2. Bits \\[10:5\\]
= Park value, bits \\[4\\]
park override, bits \\[2:0\\]
clk divider."]
pub type PhyPadAdrIoCfg2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_PAD_ADR_IO_CFG_2` writer - 18:8\\]
Controls I/O pads for address pad for address slice 2. Bits \\[10:5\\]
= Park value, bits \\[4\\]
park override, bits \\[2:0\\]
clk divider."]
pub type PhyPadAdrIoCfg2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_PAD_ADR_RX_PCLK_CLK_SEL_2` reader - 26:24\\]
Reserved for address slice 2."]
pub type PhyPadAdrRxPclkClkSel2R = crate::FieldReader;
#[doc = "Field `PHY_PAD_ADR_RX_PCLK_CLK_SEL_2` writer - 26:24\\]
Reserved for address slice 2."]
pub type PhyPadAdrRxPclkClkSel2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Tsel select values for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_tsel_select_2(&self) -> PhyAdrTselSelect2R {
        PhyAdrTselSelect2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:18 - 18:8\\]
Controls I/O pads for address pad for address slice 2. Bits \\[10:5\\]
= Park value, bits \\[4\\]
park override, bits \\[2:0\\]
clk divider."]
    #[inline(always)]
    pub fn phy_pad_adr_io_cfg_2(&self) -> PhyPadAdrIoCfg2R {
        PhyPadAdrIoCfg2R::new(((self.bits >> 8) & 0x07ff) as u16)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Reserved for address slice 2."]
    #[inline(always)]
    pub fn phy_pad_adr_rx_pclk_clk_sel_2(&self) -> PhyPadAdrRxPclkClkSel2R {
        PhyPadAdrRxPclkClkSel2R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Tsel select values for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_tsel_select_2(
        &mut self,
    ) -> PhyAdrTselSelect2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1056Spec> {
        PhyAdrTselSelect2W::new(self, 0)
    }
    #[doc = "Bits 8:18 - 18:8\\]
Controls I/O pads for address pad for address slice 2. Bits \\[10:5\\]
= Park value, bits \\[4\\]
park override, bits \\[2:0\\]
clk divider."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_adr_io_cfg_2(
        &mut self,
    ) -> PhyPadAdrIoCfg2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1056Spec> {
        PhyPadAdrIoCfg2W::new(self, 8)
    }
    #[doc = "Bits 24:26 - 26:24\\]
Reserved for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_pad_adr_rx_pclk_clk_sel_2(
        &mut self,
    ) -> PhyPadAdrRxPclkClkSel2W<CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1056Spec> {
        PhyPadAdrRxPclkClkSel2W::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1056\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1056::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1056::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1056Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1056Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1056::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1056Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_phy_1056::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1056Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PHY_1056 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPhy1056Spec {
    const RESET_VALUE: u32 = 0;
}
