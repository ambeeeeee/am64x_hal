#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap01` reader"]
pub type R = crate::R<Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap01Spec>;
#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap01` writer"]
pub type W = crate::W<Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap01Spec>;
#[doc = "Field `R_KEY_AP01` reader - 31:0\\]
Key AP"]
pub type RKeyAp01R = crate::FieldReader<u32>;
#[doc = "Field `R_KEY_AP01` writer - 31:0\\]
Key AP"]
pub type RKeyAp01W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Key AP"]
    #[inline(always)]
    pub fn r_key_ap01(&self) -> RKeyAp01R {
        RKeyAp01R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Key AP"]
    #[inline(always)]
    #[must_use]
    pub fn r_key_ap01(&mut self) -> RKeyAp01W<Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap01Spec> {
        RKeyAp01W::new(self, 0)
    }
}
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap01\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap01::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap01::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap01Spec;
impl crate::RegisterSpec for Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap01::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap01Spec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rkeyap01::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap01Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rkeyap01 to value 0"]
impl crate::Resettable for Fsas_FsasOtfaCfg_FsasOtfaRegsRkeyap01Spec {
    const RESET_VALUE: u32 = 0;
}