#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye13` reader"]
pub type R = crate::R<Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye13Spec>;
#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye13` writer"]
pub type W = crate::W<Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye13Spec>;
#[doc = "Field `R_KEY_E13` reader - 31:0\\]
Key E"]
pub type RKeyE13R = crate::FieldReader<u32>;
#[doc = "Field `R_KEY_E13` writer - 31:0\\]
Key E"]
pub type RKeyE13W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Key E"]
    #[inline(always)]
    pub fn r_key_e13(&self) -> RKeyE13R {
        RKeyE13R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Key E"]
    #[inline(always)]
    #[must_use]
    pub fn r_key_e13(&mut self) -> RKeyE13W<Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye13Spec> {
        RKeyE13W::new(self, 0)
    }
}
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye13Spec;
impl crate::RegisterSpec for Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye13::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye13Spec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeye13::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeye13 to value 0"]
impl crate::Resettable for Fsas_FsasOtfaCfg_FsasOtfaRegsRkeye13Spec {
    const RESET_VALUE: u32 = 0;
}
