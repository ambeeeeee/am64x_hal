#[doc = "Register `EPWM_REGS_DBFED` reader"]
pub type R = crate::R<EpwmRegsDbfedSpec>;
#[doc = "Register `EPWM_REGS_DBFED` writer"]
pub type W = crate::W<EpwmRegsDbfedSpec>;
#[doc = "Field `DEL` reader - 9:0\\]
Falling Edge Delay Count 10 bit counter"]
pub type DelR = crate::FieldReader<u16>;
#[doc = "Field `DEL` writer - 9:0\\]
Falling Edge Delay Count 10 bit counter"]
pub type DelW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - 9:0\\]
Falling Edge Delay Count 10 bit counter"]
    #[inline(always)]
    pub fn del(&self) -> DelR {
        DelR::new(self.bits & 0x03ff)
    }
}
impl W {
    #[doc = "Bits 0:9 - 9:0\\]
Falling Edge Delay Count 10 bit counter"]
    #[inline(always)]
    #[must_use]
    pub fn del(&mut self) -> DelW<EpwmRegsDbfedSpec> {
        DelW::new(self, 0)
    }
}
#[doc = "Dead-Band Generator Falling Edge Delay Count Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epwm_regs_dbfed::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epwm_regs_dbfed::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpwmRegsDbfedSpec;
impl crate::RegisterSpec for EpwmRegsDbfedSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`epwm_regs_dbfed::R`](R) reader structure"]
impl crate::Readable for EpwmRegsDbfedSpec {}
#[doc = "`write(|w| ..)` method takes [`epwm_regs_dbfed::W`](W) writer structure"]
impl crate::Writable for EpwmRegsDbfedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EPWM_REGS_DBFED to value 0"]
impl crate::Resettable for EpwmRegsDbfedSpec {
    const RESET_VALUE: u16 = 0;
}
