#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_irqaddinfo0` reader"]
pub type R = crate::R<Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo0Spec>;
#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_irqaddinfo0` writer"]
pub type W = crate::W<Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo0Spec>;
#[doc = "Field `IRQ_MADDR` reader - 31:0\\]
Master Address which caused the event"]
pub type IrqMaddrR = crate::FieldReader<u32>;
#[doc = "Field `IRQ_MADDR` writer - 31:0\\]
Master Address which caused the event"]
pub type IrqMaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Master Address which caused the event"]
    #[inline(always)]
    pub fn irq_maddr(&self) -> IrqMaddrR {
        IrqMaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Master Address which caused the event"]
    #[inline(always)]
    #[must_use]
    pub fn irq_maddr(&mut self) -> IrqMaddrW<Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo0Spec> {
        IrqMaddrW::new(self, 0)
    }
}
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_irqaddinfo0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo0Spec;
impl crate::RegisterSpec for Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo0::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo0Spec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo0::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_irqaddinfo0 to value 0"]
impl crate::Resettable for Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo0Spec {
    const RESET_VALUE: u32 = 0;
}
