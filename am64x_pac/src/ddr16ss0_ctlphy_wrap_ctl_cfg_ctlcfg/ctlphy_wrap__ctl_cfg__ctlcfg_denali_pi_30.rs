#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_30` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliPi30Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_30` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliPi30Spec>;
#[doc = "Field `PI_WRLVL_STROBE_NUM` reader - 4:0\\]
Defines the number of write leveling strobes generated."]
pub type PiWrlvlStrobeNumR = crate::FieldReader;
#[doc = "Field `PI_WRLVL_STROBE_NUM` writer - 4:0\\]
Defines the number of write leveling strobes generated."]
pub type PiWrlvlStrobeNumW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PI_TODTH_WR` reader - 11:8\\]
Defines the minimum DRAM cycles of ODT high time for a write command, in memory clocks."]
pub type PiTodthWrR = crate::FieldReader;
#[doc = "Field `PI_TODTH_WR` writer - 11:8\\]
Defines the minimum DRAM cycles of ODT high time for a write command, in memory clocks."]
pub type PiTodthWrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_TODTH_RD` reader - 19:16\\]
Defines the minimum DRAM cycles of ODT high time for a read command, in memory clocks."]
pub type PiTodthRdR = crate::FieldReader;
#[doc = "Field `PI_TODTH_RD` writer - 19:16\\]
Defines the minimum DRAM cycles of ODT high time for a read command, in memory clocks."]
pub type PiTodthRdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PI_ODT_VALUE` reader - 25:24\\]
When using LPDDR4, this value will be driven out on the dfi_odt signal."]
pub type PiOdtValueR = crate::FieldReader;
#[doc = "Field `PI_ODT_VALUE` writer - 25:24\\]
When using LPDDR4, this value will be driven out on the dfi_odt signal."]
pub type PiOdtValueW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Defines the number of write leveling strobes generated."]
    #[inline(always)]
    pub fn pi_wrlvl_strobe_num(&self) -> PiWrlvlStrobeNumR {
        PiWrlvlStrobeNumR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the minimum DRAM cycles of ODT high time for a write command, in memory clocks."]
    #[inline(always)]
    pub fn pi_todth_wr(&self) -> PiTodthWrR {
        PiTodthWrR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the minimum DRAM cycles of ODT high time for a read command, in memory clocks."]
    #[inline(always)]
    pub fn pi_todth_rd(&self) -> PiTodthRdR {
        PiTodthRdR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - 25:24\\]
When using LPDDR4, this value will be driven out on the dfi_odt signal."]
    #[inline(always)]
    pub fn pi_odt_value(&self) -> PiOdtValueR {
        PiOdtValueR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Defines the number of write leveling strobes generated."]
    #[inline(always)]
    #[must_use]
    pub fn pi_wrlvl_strobe_num(
        &mut self,
    ) -> PiWrlvlStrobeNumW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi30Spec> {
        PiWrlvlStrobeNumW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the minimum DRAM cycles of ODT high time for a write command, in memory clocks."]
    #[inline(always)]
    #[must_use]
    pub fn pi_todth_wr(&mut self) -> PiTodthWrW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi30Spec> {
        PiTodthWrW::new(self, 8)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the minimum DRAM cycles of ODT high time for a read command, in memory clocks."]
    #[inline(always)]
    #[must_use]
    pub fn pi_todth_rd(&mut self) -> PiTodthRdW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi30Spec> {
        PiTodthRdW::new(self, 16)
    }
    #[doc = "Bits 24:25 - 25:24\\]
When using LPDDR4, this value will be driven out on the dfi_odt signal."]
    #[inline(always)]
    #[must_use]
    pub fn pi_odt_value(&mut self) -> PiOdtValueW<CtlphyWrap_CtlCfg_CtlcfgDenaliPi30Spec> {
        PiOdtValueW::new(self, 24)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_30\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_30::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_30::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliPi30Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliPi30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_30::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi30Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_pi_30::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi30Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_PI_30 to value 0"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliPi30Spec {
    const RESET_VALUE: u32 = 0;
}
