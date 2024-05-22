#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_BUS_TO` reader"]
pub type R = crate::R<Regs_SsCfg_SscfgV2aBusToSpec>;
#[doc = "Register `REGS__SS_CFG__SSCFG_V2A_BUS_TO` writer"]
pub type W = crate::W<Regs_SsCfg_SscfgV2aBusToSpec>;
#[doc = "Field `BUS_TIMER` reader - 23:0\\]
AXI bus timeout value. Number of DDR clock cycles after which the VBUSM2AXI bridge times out if a hang on the controller AXI interface is detected. A value of N will be equal to N x 16 clocks. Writing a 0 will disable the timeout feature."]
pub type BusTimerR = crate::FieldReader<u32>;
#[doc = "Field `BUS_TIMER` writer - 23:0\\]
AXI bus timeout value. Number of DDR clock cycles after which the VBUSM2AXI bridge times out if a hang on the controller AXI interface is detected. A value of N will be equal to N x 16 clocks. Writing a 0 will disable the timeout feature."]
pub type BusTimerW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
AXI bus timeout value. Number of DDR clock cycles after which the VBUSM2AXI bridge times out if a hang on the controller AXI interface is detected. A value of N will be equal to N x 16 clocks. Writing a 0 will disable the timeout feature."]
    #[inline(always)]
    pub fn bus_timer(&self) -> BusTimerR {
        BusTimerR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
AXI bus timeout value. Number of DDR clock cycles after which the VBUSM2AXI bridge times out if a hang on the controller AXI interface is detected. A value of N will be equal to N x 16 clocks. Writing a 0 will disable the timeout feature."]
    #[inline(always)]
    #[must_use]
    pub fn bus_timer(&mut self) -> BusTimerW<Regs_SsCfg_SscfgV2aBusToSpec> {
        BusTimerW::new(self, 0)
    }
}
#[doc = "REGS__SS_CFG__SSCFG_V2A_BUS_TO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`regs__ss_cfg__sscfg_v2a_bus_to::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regs__ss_cfg__sscfg_v2a_bus_to::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Regs_SsCfg_SscfgV2aBusToSpec;
impl crate::RegisterSpec for Regs_SsCfg_SscfgV2aBusToSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regs__ss_cfg__sscfg_v2a_bus_to::R`](R) reader structure"]
impl crate::Readable for Regs_SsCfg_SscfgV2aBusToSpec {}
#[doc = "`write(|w| ..)` method takes [`regs__ss_cfg__sscfg_v2a_bus_to::W`](W) writer structure"]
impl crate::Writable for Regs_SsCfg_SscfgV2aBusToSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGS__SS_CFG__SSCFG_V2A_BUS_TO to value 0x1677_7215"]
impl crate::Resettable for Regs_SsCfg_SscfgV2aBusToSpec {
    const RESET_VALUE: u32 = 0x1677_7215;
}
