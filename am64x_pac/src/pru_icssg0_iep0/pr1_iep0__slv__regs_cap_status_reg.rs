#[doc = "Register `PR1_IEP0__SLV__REGS_cap_status_reg` reader"]
pub type R = crate::R<Pr1Iep0_Slv_RegsCapStatusRegSpec>;
#[doc = "Register `PR1_IEP0__SLV__REGS_cap_status_reg` writer"]
pub type W = crate::W<Pr1Iep0_Slv_RegsCapStatusRegSpec>;
#[doc = "Field `CAP_VALID` reader - "]
pub type CapValidR = crate::FieldReader<u16>;
#[doc = "Field `CAP_VALID` writer - "]
pub type CapValidW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `CAP_RAW` reader - "]
pub type CapRawR = crate::FieldReader;
#[doc = "Field `CAP_RAW` writer - "]
pub type CapRawW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn cap_valid(&self) -> CapValidR {
        CapValidR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn cap_raw(&self) -> CapRawR {
        CapRawR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn cap_valid(&mut self) -> CapValidW<Pr1Iep0_Slv_RegsCapStatusRegSpec> {
        CapValidW::new(self, 0)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn cap_raw(&mut self) -> CapRawW<Pr1Iep0_Slv_RegsCapStatusRegSpec> {
        CapRawW::new(self, 16)
    }
}
#[doc = "PR1_IEP0__SLV__REGS_cap_status_reg\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_iep0__slv__regs_cap_status_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_iep0__slv__regs_cap_status_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Iep0_Slv_RegsCapStatusRegSpec;
impl crate::RegisterSpec for Pr1Iep0_Slv_RegsCapStatusRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_iep0__slv__regs_cap_status_reg::R`](R) reader structure"]
impl crate::Readable for Pr1Iep0_Slv_RegsCapStatusRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_iep0__slv__regs_cap_status_reg::W`](W) writer structure"]
impl crate::Writable for Pr1Iep0_Slv_RegsCapStatusRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_IEP0__SLV__REGS_cap_status_reg to value 0"]
impl crate::Resettable for Pr1Iep0_Slv_RegsCapStatusRegSpec {
    const RESET_VALUE: u32 = 0;
}
