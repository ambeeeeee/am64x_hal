#[doc = "Register `GIC_REGS_ITS__31_GITS_CIDR2` reader"]
pub type R = crate::R<GicRegsIts_31GitsCidr2Spec>;
#[doc = "Register `GIC_REGS_ITS__31_GITS_CIDR2` writer"]
pub type W = crate::W<GicRegsIts_31GitsCidr2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "GITS_CIDR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gic_regs_its__31_gits_cidr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gic_regs_its__31_gits_cidr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GicRegsIts_31GitsCidr2Spec;
impl crate::RegisterSpec for GicRegsIts_31GitsCidr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gic_regs_its__31_gits_cidr2::R`](R) reader structure"]
impl crate::Readable for GicRegsIts_31GitsCidr2Spec {}
#[doc = "`write(|w| ..)` method takes [`gic_regs_its__31_gits_cidr2::W`](W) writer structure"]
impl crate::Writable for GicRegsIts_31GitsCidr2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GIC_REGS_ITS__31_GITS_CIDR2 to value 0"]
impl crate::Resettable for GicRegsIts_31GitsCidr2Spec {
    const RESET_VALUE: u32 = 0;
}
