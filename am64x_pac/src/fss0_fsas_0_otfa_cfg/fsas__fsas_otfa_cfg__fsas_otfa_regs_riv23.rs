#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv23` reader"]
pub type R = crate::R<Fsas_FsasOtfaCfg_FsasOtfaRegsRiv23Spec>;
#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv23` writer"]
pub type W = crate::W<Fsas_FsasOtfaCfg_FsasOtfaRegsRiv23Spec>;
#[doc = "Field `R_IV23` reader - 31:0\\]
IV"]
pub type RIv23R = crate::FieldReader<u32>;
#[doc = "Field `R_IV23` writer - 31:0\\]
IV"]
pub type RIv23W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
IV"]
    #[inline(always)]
    pub fn r_iv23(&self) -> RIv23R {
        RIv23R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
IV"]
    #[inline(always)]
    #[must_use]
    pub fn r_iv23(&mut self) -> RIv23W<Fsas_FsasOtfaCfg_FsasOtfaRegsRiv23Spec> {
        RIv23W::new(self, 0)
    }
}
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv23\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv23::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv23::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasOtfaCfg_FsasOtfaRegsRiv23Spec;
impl crate::RegisterSpec for Fsas_FsasOtfaCfg_FsasOtfaRegsRiv23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv23::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasOtfaCfg_FsasOtfaRegsRiv23Spec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv23::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasOtfaCfg_FsasOtfaRegsRiv23Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv23 to value 0"]
impl crate::Resettable for Fsas_FsasOtfaCfg_FsasOtfaRegsRiv23Spec {
    const RESET_VALUE: u32 = 0;
}
