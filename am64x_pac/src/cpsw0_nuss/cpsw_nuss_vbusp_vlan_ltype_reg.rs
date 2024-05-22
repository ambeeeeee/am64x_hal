#[doc = "Register `CPSW_NUSS_VBUSP_VLAN_LTYPE_REG` reader"]
pub type R = crate::R<CpswNussVbuspVlanLtypeRegSpec>;
#[doc = "Register `CPSW_NUSS_VBUSP_VLAN_LTYPE_REG` writer"]
pub type W = crate::W<CpswNussVbuspVlanLtypeRegSpec>;
#[doc = "Field `VLAN_LTYPE_INNER` reader - 15:0\\]
Inner VLAN LType"]
pub type VlanLtypeInnerR = crate::FieldReader<u16>;
#[doc = "Field `VLAN_LTYPE_INNER` writer - 15:0\\]
Inner VLAN LType"]
pub type VlanLtypeInnerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `VLAN_LTYPE_OUTER` reader - 31:16\\]
Outer VLAN LType"]
pub type VlanLtypeOuterR = crate::FieldReader<u16>;
#[doc = "Field `VLAN_LTYPE_OUTER` writer - 31:16\\]
Outer VLAN LType"]
pub type VlanLtypeOuterW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Inner VLAN LType"]
    #[inline(always)]
    pub fn vlan_ltype_inner(&self) -> VlanLtypeInnerR {
        VlanLtypeInnerR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Outer VLAN LType"]
    #[inline(always)]
    pub fn vlan_ltype_outer(&self) -> VlanLtypeOuterR {
        VlanLtypeOuterR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Inner VLAN LType"]
    #[inline(always)]
    #[must_use]
    pub fn vlan_ltype_inner(&mut self) -> VlanLtypeInnerW<CpswNussVbuspVlanLtypeRegSpec> {
        VlanLtypeInnerW::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Outer VLAN LType"]
    #[inline(always)]
    #[must_use]
    pub fn vlan_ltype_outer(&mut self) -> VlanLtypeOuterW<CpswNussVbuspVlanLtypeRegSpec> {
        VlanLtypeOuterW::new(self, 16)
    }
}
#[doc = "VLAN Length/type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpsw_nuss_vbusp_vlan_ltype_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpsw_nuss_vbusp_vlan_ltype_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpswNussVbuspVlanLtypeRegSpec;
impl crate::RegisterSpec for CpswNussVbuspVlanLtypeRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsw_nuss_vbusp_vlan_ltype_reg::R`](R) reader structure"]
impl crate::Readable for CpswNussVbuspVlanLtypeRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpsw_nuss_vbusp_vlan_ltype_reg::W`](W) writer structure"]
impl crate::Writable for CpswNussVbuspVlanLtypeRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSW_NUSS_VBUSP_VLAN_LTYPE_REG to value 0x0003_3024"]
impl crate::Resettable for CpswNussVbuspVlanLtypeRegSpec {
    const RESET_VALUE: u32 = 0x0003_3024;
}
