#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst2` reader"]
pub type R = crate::R<Fsas_FsasOtfaCfg_FsasOtfaRegsRgst2Spec>;
#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst2` writer"]
pub type W = crate::W<Fsas_FsasOtfaCfg_FsasOtfaRegsRgst2Spec>;
#[doc = "Field `R_START2` reader - 19:0\\]
This defines the start of the crypto region in 4KBytes steps"]
pub type RStart2R = crate::FieldReader<u32>;
#[doc = "Field `R_START2` writer - 19:0\\]
This defines the start of the crypto region in 4KBytes steps"]
pub type RStart2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
This defines the start of the crypto region in 4KBytes steps"]
    #[inline(always)]
    pub fn r_start2(&self) -> RStart2R {
        RStart2R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
This defines the start of the crypto region in 4KBytes steps"]
    #[inline(always)]
    #[must_use]
    pub fn r_start2(&mut self) -> RStart2W<Fsas_FsasOtfaCfg_FsasOtfaRegsRgst2Spec> {
        RStart2W::new(self, 0)
    }
}
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasOtfaCfg_FsasOtfaRegsRgst2Spec;
impl crate::RegisterSpec for Fsas_FsasOtfaCfg_FsasOtfaRegsRgst2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst2::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasOtfaCfg_FsasOtfaRegsRgst2Spec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_otfa_cfg__fsas_otfa_regs_rgst2::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasOtfaCfg_FsasOtfaRegsRgst2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_rgst2 to value 0"]
impl crate::Resettable for Fsas_FsasOtfaCfg_FsasOtfaRegsRgst2Spec {
    const RESET_VALUE: u32 = 0;
}
