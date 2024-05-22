#[doc = "Register `CTI__CFG__CSCTI_CFG_CTIINTACK` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgCtiintackSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_CTIINTACK` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgCtiintackSpec>;
#[doc = "Field `INTACK` reader - 7:0\\]
Acknowledges the corresponding ctitrigout output. There is one bit of the register for each ctitrigout output. When a 1 is written to a bit in this register, the corresponding ctitrigout is acknowledged and is cleared when MAPTRIGOUT is LOW. Writing a 0 to any of the bits in this register has no effect."]
pub type IntackR = crate::FieldReader;
#[doc = "Field `INTACK` writer - 7:0\\]
Acknowledges the corresponding ctitrigout output. There is one bit of the register for each ctitrigout output. When a 1 is written to a bit in this register, the corresponding ctitrigout is acknowledged and is cleared when MAPTRIGOUT is LOW. Writing a 0 to any of the bits in this register has no effect."]
pub type IntackW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Acknowledges the corresponding ctitrigout output. There is one bit of the register for each ctitrigout output. When a 1 is written to a bit in this register, the corresponding ctitrigout is acknowledged and is cleared when MAPTRIGOUT is LOW. Writing a 0 to any of the bits in this register has no effect."]
    #[inline(always)]
    pub fn intack(&self) -> IntackR {
        IntackR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Acknowledges the corresponding ctitrigout output. There is one bit of the register for each ctitrigout output. When a 1 is written to a bit in this register, the corresponding ctitrigout is acknowledged and is cleared when MAPTRIGOUT is LOW. Writing a 0 to any of the bits in this register has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn intack(&mut self) -> IntackW<Cti_Cfg_CsctiCfgCtiintackSpec> {
        IntackW::new(self, 0)
    }
}
#[doc = "The CTI Interrupt Acknowledge Register is write-only. Any bits written as a 1 cause the ctitrigout output signal to be acknowledged. The acknowledgement is cleared when MAPTRIGOUT is deactivated. This register is used when the ctitrigout is used as a sticky output, that is, no hardware acknowledge is supplied, and a software acknowledge is required.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiintack::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiintack::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgCtiintackSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgCtiintackSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_ctiintack::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgCtiintackSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_ctiintack::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgCtiintackSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_CTIINTACK to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgCtiintackSpec {
    const RESET_VALUE: u32 = 0;
}
