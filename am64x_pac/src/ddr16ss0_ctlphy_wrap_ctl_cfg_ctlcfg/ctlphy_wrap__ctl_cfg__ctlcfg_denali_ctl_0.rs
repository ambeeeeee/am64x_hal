#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_0` reader"]
pub type R = crate::R<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl0Spec>;
#[doc = "Register `CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_0` writer"]
pub type W = crate::W<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl0Spec>;
#[doc = "Field `START` reader - 0:0\\]
Initiate command processing in the controller. Set to 1 to initiate."]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - 0:0\\]
Initiate command processing in the controller. Set to 1 to initiate."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRAM_CLASS` reader - 11:8\\]
Defines the class of DRAM memory which is connected to the controller."]
pub type DramClassR = crate::FieldReader;
#[doc = "Field `DRAM_CLASS` writer - 11:8\\]
Defines the class of DRAM memory which is connected to the controller."]
pub type DramClassW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CONTROLLER_ID` reader - 31:16\\]
Holds the controller product id number. READ-ONLY"]
pub type ControllerIdR = crate::FieldReader<u16>;
#[doc = "Field `CONTROLLER_ID` writer - 31:16\\]
Holds the controller product id number. READ-ONLY"]
pub type ControllerIdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Initiate command processing in the controller. Set to 1 to initiate."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the class of DRAM memory which is connected to the controller."]
    #[inline(always)]
    pub fn dram_class(&self) -> DramClassR {
        DramClassR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Holds the controller product id number. READ-ONLY"]
    #[inline(always)]
    pub fn controller_id(&self) -> ControllerIdR {
        ControllerIdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Initiate command processing in the controller. Set to 1 to initiate."]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl0Spec> {
        StartW::new(self, 0)
    }
    #[doc = "Bits 8:11 - 11:8\\]
Defines the class of DRAM memory which is connected to the controller."]
    #[inline(always)]
    #[must_use]
    pub fn dram_class(&mut self) -> DramClassW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl0Spec> {
        DramClassW::new(self, 8)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Holds the controller product id number. READ-ONLY"]
    #[inline(always)]
    #[must_use]
    pub fn controller_id(&mut self) -> ControllerIdW<CtlphyWrap_CtlCfg_CtlcfgDenaliCtl0Spec> {
        ControllerIdW::new(self, 16)
    }
}
#[doc = "CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlphyWrap_CtlCfg_CtlcfgDenaliCtl0Spec;
impl crate::RegisterSpec for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_0::R`](R) reader structure"]
impl crate::Readable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctlphy_wrap__ctl_cfg__ctlcfg_denali_ctl_0::W`](W) writer structure"]
impl crate::Writable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTLPHY_WRAP__CTL_CFG__CTLCFG_DENALI_CTL_0 to value 0x4166_0000"]
impl crate::Resettable for CtlphyWrap_CtlCfg_CtlcfgDenaliCtl0Spec {
    const RESET_VALUE: u32 = 0x4166_0000;
}
