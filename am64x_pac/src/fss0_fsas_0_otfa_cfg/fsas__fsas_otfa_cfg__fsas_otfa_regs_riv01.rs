#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv01` reader"]
pub type R = crate::R<Fsas_FsasOtfaCfg_FsasOtfaRegsRiv01Spec>;
#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv01` writer"]
pub type W = crate::W<Fsas_FsasOtfaCfg_FsasOtfaRegsRiv01Spec>;
#[doc = "Field `R_IV01` reader - 31:0\\]
IV"]
pub type RIv01R = crate::FieldReader<u32>;
#[doc = "Field `R_IV01` writer - 31:0\\]
IV"]
pub type RIv01W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
IV"]
    #[inline(always)]
    pub fn r_iv01(&self) -> RIv01R {
        RIv01R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
IV"]
    #[inline(always)]
    #[must_use]
    pub fn r_iv01(&mut self) -> RIv01W<Fsas_FsasOtfaCfg_FsasOtfaRegsRiv01Spec> {
        RIv01W::new(self, 0)
    }
}
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv01\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasOtfaCfg_FsasOtfaRegsRiv01Spec;
impl crate::RegisterSpec for Fsas_FsasOtfaCfg_FsasOtfaRegsRiv01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv01::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasOtfaCfg_FsasOtfaRegsRiv01Spec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_otfa_cfg__fsas_otfa_regs_riv01::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasOtfaCfg_FsasOtfaRegsRiv01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_riv01 to value 0"]
impl crate::Resettable for Fsas_FsasOtfaCfg_FsasOtfaRegsRiv01Spec {
    const RESET_VALUE: u32 = 0;
}
