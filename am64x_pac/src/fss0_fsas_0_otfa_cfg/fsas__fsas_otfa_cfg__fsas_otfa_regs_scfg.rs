#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_scfg` reader"]
pub type R = crate::R<Fsas_FsasOtfaCfg_FsasOtfaRegsScfgSpec>;
#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_scfg` writer"]
pub type W = crate::W<Fsas_FsasOtfaCfg_FsasOtfaRegsScfgSpec>;
#[doc = "Field `IDLE_MODE` reader - 1:0\\]
IDLE MODE"]
pub type IdleModeR = crate::FieldReader;
#[doc = "Field `IDLE_MODE` writer - 1:0\\]
IDLE MODE"]
pub type IdleModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
IDLE MODE"]
    #[inline(always)]
    pub fn idle_mode(&self) -> IdleModeR {
        IdleModeR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
IDLE MODE"]
    #[inline(always)]
    #[must_use]
    pub fn idle_mode(&mut self) -> IdleModeW<Fsas_FsasOtfaCfg_FsasOtfaRegsScfgSpec> {
        IdleModeW::new(self, 0)
    }
}
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_scfg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_scfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_scfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasOtfaCfg_FsasOtfaRegsScfgSpec;
impl crate::RegisterSpec for Fsas_FsasOtfaCfg_FsasOtfaRegsScfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_otfa_cfg__fsas_otfa_regs_scfg::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasOtfaCfg_FsasOtfaRegsScfgSpec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_otfa_cfg__fsas_otfa_regs_scfg::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasOtfaCfg_FsasOtfaRegsScfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_scfg to value 0x02"]
impl crate::Resettable for Fsas_FsasOtfaCfg_FsasOtfaRegsScfgSpec {
    const RESET_VALUE: u32 = 0x02;
}
