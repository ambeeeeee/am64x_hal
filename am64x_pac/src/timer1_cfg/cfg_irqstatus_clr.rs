#[doc = "Register `CFG_IRQSTATUS_CLR` reader"]
pub type R = crate::R<CfgIrqstatusClrSpec>;
#[doc = "Register `CFG_IRQSTATUS_CLR` writer"]
pub type W = crate::W<CfgIrqstatusClrSpec>;
#[doc = "Field `MAT_IT_FLAG` reader - 0:0\\]
Match Interrupt"]
pub type MatItFlagR = crate::BitReader;
#[doc = "Field `MAT_IT_FLAG` writer - 0:0\\]
Match Interrupt"]
pub type MatItFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF_IT_FLAG` reader - 1:1\\]
Overflow Interrupt"]
pub type OvfItFlagR = crate::BitReader;
#[doc = "Field `OVF_IT_FLAG` writer - 1:1\\]
Overflow Interrupt"]
pub type OvfItFlagW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCAR_IT_FLAG` reader - 2:2\\]
Capture Interrupt"]
pub type TcarItFlagR = crate::BitReader;
#[doc = "Field `TCAR_IT_FLAG` writer - 2:2\\]
Capture Interrupt"]
pub type TcarItFlagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Match Interrupt"]
    #[inline(always)]
    pub fn mat_it_flag(&self) -> MatItFlagR {
        MatItFlagR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Overflow Interrupt"]
    #[inline(always)]
    pub fn ovf_it_flag(&self) -> OvfItFlagR {
        OvfItFlagR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Capture Interrupt"]
    #[inline(always)]
    pub fn tcar_it_flag(&self) -> TcarItFlagR {
        TcarItFlagR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Match Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn mat_it_flag(&mut self) -> MatItFlagW<CfgIrqstatusClrSpec> {
        MatItFlagW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Overflow Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ovf_it_flag(&mut self) -> OvfItFlagW<CfgIrqstatusClrSpec> {
        OvfItFlagW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Capture Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn tcar_it_flag(&mut self) -> TcarItFlagW<CfgIrqstatusClrSpec> {
        TcarItFlagW::new(self, 2)
    }
}
#[doc = "Component interrupt request enable. Write 1 to clear; disable interrupt. Readout equal to corresponding _SET register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_irqstatus_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_irqstatus_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgIrqstatusClrSpec;
impl crate::RegisterSpec for CfgIrqstatusClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_irqstatus_clr::R`](R) reader structure"]
impl crate::Readable for CfgIrqstatusClrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_irqstatus_clr::W`](W) writer structure"]
impl crate::Writable for CfgIrqstatusClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_IRQSTATUS_CLR to value 0"]
impl crate::Resettable for CfgIrqstatusClrSpec {
    const RESET_VALUE: u32 = 0;
}
