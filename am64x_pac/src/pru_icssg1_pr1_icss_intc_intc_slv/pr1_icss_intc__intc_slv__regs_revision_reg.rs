#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_REVISION_REG` reader"]
pub type R = crate::R<Pr1IcssIntc_IntcSlv_RegsRevisionRegSpec>;
#[doc = "Register `PR1_ICSS_INTC__INTC_SLV__REGS_REVISION_REG` writer"]
pub type W = crate::W<Pr1IcssIntc_IntcSlv_RegsRevisionRegSpec>;
#[doc = "Field `REV_MINOR` reader - 5:0\\]
Minor revision"]
pub type RevMinorR = crate::FieldReader;
#[doc = "Field `REV_MINOR` writer - 5:0\\]
Minor revision"]
pub type RevMinorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `REV_CUSTOM` reader - 7:6\\]
Custom revision"]
pub type RevCustomR = crate::FieldReader;
#[doc = "Field `REV_CUSTOM` writer - 7:6\\]
Custom revision"]
pub type RevCustomW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REV_MAJOR` reader - 10:8\\]
Major revision"]
pub type RevMajorR = crate::FieldReader;
#[doc = "Field `REV_MAJOR` writer - 10:8\\]
Major revision"]
pub type RevMajorW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REV_RTL` reader - 15:11\\]
RTL revisions"]
pub type RevRtlR = crate::FieldReader;
#[doc = "Field `REV_RTL` writer - 15:11\\]
RTL revisions"]
pub type RevRtlW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `REV_MODULE` reader - 27:16\\]
Module ID"]
pub type RevModuleR = crate::FieldReader<u16>;
#[doc = "Field `REV_MODULE` writer - 27:16\\]
Module ID"]
pub type RevModuleW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `REV_SCHEME` reader - 31:30\\]
Scheme"]
pub type RevSchemeR = crate::FieldReader;
#[doc = "Field `REV_SCHEME` writer - 31:30\\]
Scheme"]
pub type RevSchemeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision"]
    #[inline(always)]
    pub fn rev_minor(&self) -> RevMinorR {
        RevMinorR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom revision"]
    #[inline(always)]
    pub fn rev_custom(&self) -> RevCustomR {
        RevCustomR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision"]
    #[inline(always)]
    pub fn rev_major(&self) -> RevMajorR {
        RevMajorR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL revisions"]
    #[inline(always)]
    pub fn rev_rtl(&self) -> RevRtlR {
        RevRtlR::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Module ID"]
    #[inline(always)]
    pub fn rev_module(&self) -> RevModuleR {
        RevModuleR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Scheme"]
    #[inline(always)]
    pub fn rev_scheme(&self) -> RevSchemeR {
        RevSchemeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Minor revision"]
    #[inline(always)]
    #[must_use]
    pub fn rev_minor(&mut self) -> RevMinorW<Pr1IcssIntc_IntcSlv_RegsRevisionRegSpec> {
        RevMinorW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Custom revision"]
    #[inline(always)]
    #[must_use]
    pub fn rev_custom(&mut self) -> RevCustomW<Pr1IcssIntc_IntcSlv_RegsRevisionRegSpec> {
        RevCustomW::new(self, 6)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Major revision"]
    #[inline(always)]
    #[must_use]
    pub fn rev_major(&mut self) -> RevMajorW<Pr1IcssIntc_IntcSlv_RegsRevisionRegSpec> {
        RevMajorW::new(self, 8)
    }
    #[doc = "Bits 11:15 - 15:11\\]
RTL revisions"]
    #[inline(always)]
    #[must_use]
    pub fn rev_rtl(&mut self) -> RevRtlW<Pr1IcssIntc_IntcSlv_RegsRevisionRegSpec> {
        RevRtlW::new(self, 11)
    }
    #[doc = "Bits 16:27 - 27:16\\]
Module ID"]
    #[inline(always)]
    #[must_use]
    pub fn rev_module(&mut self) -> RevModuleW<Pr1IcssIntc_IntcSlv_RegsRevisionRegSpec> {
        RevModuleW::new(self, 16)
    }
    #[doc = "Bits 30:31 - 31:30\\]
Scheme"]
    #[inline(always)]
    #[must_use]
    pub fn rev_scheme(&mut self) -> RevSchemeW<Pr1IcssIntc_IntcSlv_RegsRevisionRegSpec> {
        RevSchemeW::new(self, 30)
    }
}
#[doc = "PR1_ICSS_INTC__INTC_SLV__REGS_REVISION_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_icss_intc__intc_slv__regs_revision_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_icss_intc__intc_slv__regs_revision_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1IcssIntc_IntcSlv_RegsRevisionRegSpec;
impl crate::RegisterSpec for Pr1IcssIntc_IntcSlv_RegsRevisionRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_icss_intc__intc_slv__regs_revision_reg::R`](R) reader structure"]
impl crate::Readable for Pr1IcssIntc_IntcSlv_RegsRevisionRegSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_icss_intc__intc_slv__regs_revision_reg::W`](W) writer structure"]
impl crate::Writable for Pr1IcssIntc_IntcSlv_RegsRevisionRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_ICSS_INTC__INTC_SLV__REGS_REVISION_REG to value 0x7715_0900"]
impl crate::Resettable for Pr1IcssIntc_IntcSlv_RegsRevisionRegSpec {
    const RESET_VALUE: u32 = 0x7715_0900;
}
