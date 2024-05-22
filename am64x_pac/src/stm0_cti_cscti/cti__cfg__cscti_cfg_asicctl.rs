#[doc = "Register `CTI__CFG__CSCTI_CFG_ASICCTL` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgAsicctlSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_ASICCTL` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgAsicctlSpec>;
#[doc = "Field `ASICCTL` reader - 7:0\\]
Implementation-defined ASIC control, value written to the register is output on asicctl\\[7 : 0\\]. If external multiplexing of trigger signals is implemented then the number of multiplexed signals on each trigger must be reflected within the Device ID Register. This is done within a Verilog define EXTMUXNUM."]
pub type AsicctlR = crate::FieldReader;
#[doc = "Field `ASICCTL` writer - 7:0\\]
Implementation-defined ASIC control, value written to the register is output on asicctl\\[7 : 0\\]. If external multiplexing of trigger signals is implemented then the number of multiplexed signals on each trigger must be reflected within the Device ID Register. This is done within a Verilog define EXTMUXNUM."]
pub type AsicctlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Implementation-defined ASIC control, value written to the register is output on asicctl\\[7 : 0\\]. If external multiplexing of trigger signals is implemented then the number of multiplexed signals on each trigger must be reflected within the Device ID Register. This is done within a Verilog define EXTMUXNUM."]
    #[inline(always)]
    pub fn asicctl(&self) -> AsicctlR {
        AsicctlR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Implementation-defined ASIC control, value written to the register is output on asicctl\\[7 : 0\\]. If external multiplexing of trigger signals is implemented then the number of multiplexed signals on each trigger must be reflected within the Device ID Register. This is done within a Verilog define EXTMUXNUM."]
    #[inline(always)]
    #[must_use]
    pub fn asicctl(&mut self) -> AsicctlW<Cti_Cfg_CsctiCfgAsicctlSpec> {
        AsicctlW::new(self, 0)
    }
}
#[doc = "Implementation-defined ASIC control, value written to the register is output on asicctl\\[7 : 0\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_asicctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_asicctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgAsicctlSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgAsicctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_asicctl::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgAsicctlSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_asicctl::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgAsicctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_ASICCTL to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgAsicctlSpec {
    const RESET_VALUE: u32 = 0;
}
