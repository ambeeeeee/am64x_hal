#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_irqaddinfo1` reader"]
pub type R = crate::R<Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo1Spec>;
#[doc = "Register `FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_irqaddinfo1` writer"]
pub type W = crate::W<Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo1Spec>;
#[doc = "Field `IRQ_MID` reader - 7:0\\]
Master TAG ID which caused the event"]
pub type IrqMidR = crate::FieldReader;
#[doc = "Field `IRQ_MID` writer - 7:0\\]
Master TAG ID which caused the event"]
pub type IrqMidW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IRQ_MCMD` reader - 10:8\\]
Master CMD which caused the event"]
pub type IrqMcmdR = crate::FieldReader;
#[doc = "Field `IRQ_MCMD` writer - 10:8\\]
Master CMD which caused the event"]
pub type IrqMcmdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IRQ_MSEQ` reader - 13:11\\]
Master SEQ which caused the event"]
pub type IrqMseqR = crate::FieldReader;
#[doc = "Field `IRQ_MSEQ` writer - 13:11\\]
Master SEQ which caused the event"]
pub type IrqMseqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IRQ_MLEN` reader - 17:14\\]
Master LENGTH which caused the event"]
pub type IrqMlenR = crate::FieldReader;
#[doc = "Field `IRQ_MLEN` writer - 17:14\\]
Master LENGTH which caused the event"]
pub type IrqMlenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Master TAG ID which caused the event"]
    #[inline(always)]
    pub fn irq_mid(&self) -> IrqMidR {
        IrqMidR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Master CMD which caused the event"]
    #[inline(always)]
    pub fn irq_mcmd(&self) -> IrqMcmdR {
        IrqMcmdR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Master SEQ which caused the event"]
    #[inline(always)]
    pub fn irq_mseq(&self) -> IrqMseqR {
        IrqMseqR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:17 - 17:14\\]
Master LENGTH which caused the event"]
    #[inline(always)]
    pub fn irq_mlen(&self) -> IrqMlenR {
        IrqMlenR::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Master TAG ID which caused the event"]
    #[inline(always)]
    #[must_use]
    pub fn irq_mid(&mut self) -> IrqMidW<Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo1Spec> {
        IrqMidW::new(self, 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Master CMD which caused the event"]
    #[inline(always)]
    #[must_use]
    pub fn irq_mcmd(&mut self) -> IrqMcmdW<Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo1Spec> {
        IrqMcmdW::new(self, 8)
    }
    #[doc = "Bits 11:13 - 13:11\\]
Master SEQ which caused the event"]
    #[inline(always)]
    #[must_use]
    pub fn irq_mseq(&mut self) -> IrqMseqW<Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo1Spec> {
        IrqMseqW::new(self, 11)
    }
    #[doc = "Bits 14:17 - 17:14\\]
Master LENGTH which caused the event"]
    #[inline(always)]
    #[must_use]
    pub fn irq_mlen(&mut self) -> IrqMlenW<Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo1Spec> {
        IrqMlenW::new(self, 14)
    }
}
#[doc = "FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_irqaddinfo1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo1Spec;
impl crate::RegisterSpec for Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo1::R`](R) reader structure"]
impl crate::Readable for Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo1Spec {}
#[doc = "`write(|w| ..)` method takes [`fsas__fsas_otfa_cfg__fsas_otfa_regs_irqaddinfo1::W`](W) writer structure"]
impl crate::Writable for Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSAS__FSAS_OTFA_CFG__FSAS_OTFA_REGS_irqaddinfo1 to value 0"]
impl crate::Resettable for Fsas_FsasOtfaCfg_FsasOtfaRegsIrqaddinfo1Spec {
    const RESET_VALUE: u32 = 0;
}
