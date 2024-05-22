#[doc = "Register `EPWM_REGS_ETSEL` reader"]
pub type R = crate::R<EpwmRegsEtselSpec>;
#[doc = "Register `EPWM_REGS_ETSEL` writer"]
pub type W = crate::W<EpwmRegsEtselSpec>;
#[doc = "Field `INTSEL` reader - 2:0\\]
ePWM Interrupt \\[EPWMx_INT\\]
Selection Options"]
pub type IntselR = crate::FieldReader;
#[doc = "Field `INTSEL` writer - 2:0\\]
ePWM Interrupt \\[EPWMx_INT\\]
Selection Options"]
pub type IntselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `INTEN` reader - 3:3\\]
Enable ePWM Interrupt \\[EPWMx_INT\\]
Generation"]
pub type IntenR = crate::BitReader;
#[doc = "Field `INTEN` writer - 3:3\\]
Enable ePWM Interrupt \\[EPWMx_INT\\]
Generation"]
pub type IntenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
ePWM Interrupt \\[EPWMx_INT\\]
Selection Options"]
    #[inline(always)]
    pub fn intsel(&self) -> IntselR {
        IntselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable ePWM Interrupt \\[EPWMx_INT\\]
Generation"]
    #[inline(always)]
    pub fn inten(&self) -> IntenR {
        IntenR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
ePWM Interrupt \\[EPWMx_INT\\]
Selection Options"]
    #[inline(always)]
    #[must_use]
    pub fn intsel(&mut self) -> IntselW<EpwmRegsEtselSpec> {
        IntselW::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Enable ePWM Interrupt \\[EPWMx_INT\\]
Generation"]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> IntenW<EpwmRegsEtselSpec> {
        IntenW::new(self, 3)
    }
}
#[doc = "Event Trigger Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_etsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_etsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsEtselSpec;
impl crate::RegisterSpec for EpwmRegsEtselSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_etsel::R`](R) reader structure"]
impl crate::Readable for EpwmRegsEtselSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_etsel::W`](W) writer structure"]
impl crate::Writable for EpwmRegsEtselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_ETSEL to value 0"]
impl crate::Resettable for EpwmRegsEtselSpec {
    const RESET_VALUE: u16 = 0;
}
