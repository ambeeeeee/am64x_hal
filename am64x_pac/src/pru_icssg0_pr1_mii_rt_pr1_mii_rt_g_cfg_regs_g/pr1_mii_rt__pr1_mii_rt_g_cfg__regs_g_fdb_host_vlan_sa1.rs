#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_fdb_host_vlan_sa1` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbHostVlanSa1Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_fdb_host_vlan_sa1` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbHostVlanSa1Spec>;
#[doc = "Field `FDB_HOST_SA1` reader - 15:0\\]
FDB HOST SA5:4"]
pub type FdbHostSa1R = crate::FieldReader<u16>;
#[doc = "Field `FDB_HOST_SA1` writer - 15:0\\]
FDB HOST SA5:4"]
pub type FdbHostSa1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `FDB_HOST_VLAN_HSR` reader - 31:16\\]
FDB HOST VLAN\\[11:0\\]
OR HSR\\[15:0\\]"]
pub type FdbHostVlanHsrR = crate::FieldReader<u16>;
#[doc = "Field `FDB_HOST_VLAN_HSR` writer - 31:16\\]
FDB HOST VLAN\\[11:0\\]
OR HSR\\[15:0\\]"]
pub type FdbHostVlanHsrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
FDB HOST SA5:4"]
    #[inline(always)]
    pub fn fdb_host_sa1(&self) -> FdbHostSa1R {
        FdbHostSa1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - 31:16\\]
FDB HOST VLAN\\[11:0\\]
OR HSR\\[15:0\\]"]
    #[inline(always)]
    pub fn fdb_host_vlan_hsr(&self) -> FdbHostVlanHsrR {
        FdbHostVlanHsrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
FDB HOST SA5:4"]
    #[inline(always)]
    #[must_use]
    pub fn fdb_host_sa1(&mut self) -> FdbHostSa1W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbHostVlanSa1Spec> {
        FdbHostSa1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
FDB HOST VLAN\\[11:0\\]
OR HSR\\[15:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn fdb_host_vlan_hsr(
        &mut self,
    ) -> FdbHostVlanHsrW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbHostVlanSa1Spec> {
        FdbHostVlanHsrW::new(self, 16)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_fdb_host_vlan_sa1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_fdb_host_vlan_sa1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_fdb_host_vlan_sa1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbHostVlanSa1Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbHostVlanSa1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_fdb_host_vlan_sa1::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbHostVlanSa1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_fdb_host_vlan_sa1::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbHostVlanSa1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_fdb_host_vlan_sa1 to value 0x0001_0000"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbHostVlanSa1Spec {
    const RESET_VALUE: u32 = 0x0001_0000;
}
