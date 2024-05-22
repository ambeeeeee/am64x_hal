#[doc = "Register `CTI__CFG__CSCTI_CFG_CTIAPPPULSE` reader"]
pub type R = crate::R<Cti_Cfg_CsctiCfgCtiapppulseSpec>;
#[doc = "Register `CTI__CFG__CSCTI_CFG_CTIAPPPULSE` writer"]
pub type W = crate::W<Cti_Cfg_CsctiCfgCtiapppulseSpec>;
#[doc = "Field `APPULSE` reader - 3:0\\]
Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel. When a 1 is written to a bit in this register, a corresponding channel event pulse is generated for one cticlk period. Writing a 0 to any of the bits in this register has no effect."]
pub type AppulseR = crate::FieldReader;
#[doc = "Field `APPULSE` writer - 3:0\\]
Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel. When a 1 is written to a bit in this register, a corresponding channel event pulse is generated for one cticlk period. Writing a 0 to any of the bits in this register has no effect."]
pub type AppulseW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel. When a 1 is written to a bit in this register, a corresponding channel event pulse is generated for one cticlk period. Writing a 0 to any of the bits in this register has no effect."]
    #[inline(always)]
    pub fn appulse(&self) -> AppulseR {
        AppulseR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Setting a bit HIGH generates a channel event pulse for the selected channel. There is one bit of the register for each channel. When a 1 is written to a bit in this register, a corresponding channel event pulse is generated for one cticlk period. Writing a 0 to any of the bits in this register has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn appulse(&mut self) -> AppulseW<Cti_Cfg_CsctiCfgCtiapppulseSpec> {
        AppulseW::new(self, 0)
    }
}
#[doc = "The CTI Application Pulse Register is write-only. A write to this register causes a channel event pulse, one cticlk period, to be generated, corresponding to the bit written to. The pulse external to the ECT can be extended to multi-cycle by the handshaking interface circuits. This register clears itself immediately, so it can be repeatedly written to without software having to clear it.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cti__cfg__cscti_cfg_ctiapppulse::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cti__cfg__cscti_cfg_ctiapppulse::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cti_Cfg_CsctiCfgCtiapppulseSpec;
impl crate::RegisterSpec for Cti_Cfg_CsctiCfgCtiapppulseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cti__cfg__cscti_cfg_ctiapppulse::R`](R) reader structure"]
impl crate::Readable for Cti_Cfg_CsctiCfgCtiapppulseSpec {}
#[doc = "`write(|w| ..)` method takes [`cti__cfg__cscti_cfg_ctiapppulse::W`](W) writer structure"]
impl crate::Writable for Cti_Cfg_CsctiCfgCtiapppulseSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTI__CFG__CSCTI_CFG_CTIAPPPULSE to value 0"]
impl crate::Resettable for Cti_Cfg_CsctiCfgCtiapppulseSpec {
    const RESET_VALUE: u32 = 0;
}
