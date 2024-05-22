#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_fdb_df_vlan` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbDfVlanSpec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_fdb_df_vlan` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbDfVlanSpec>;
#[doc = "Field `FDB_PRU0_DF_VLAN` reader - 11:0\\]
FDB Default VLAN for PRU0"]
pub type FdbPru0DfVlanR = crate::FieldReader<u16>;
#[doc = "Field `FDB_PRU0_DF_VLAN` writer - 11:0\\]
FDB Default VLAN for PRU0"]
pub type FdbPru0DfVlanW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `FDB_PRU1_DF_VLAN` reader - 27:16\\]
FDB Default VLAN for PRU1"]
pub type FdbPru1DfVlanR = crate::FieldReader<u16>;
#[doc = "Field `FDB_PRU1_DF_VLAN` writer - 27:16\\]
FDB Default VLAN for PRU1"]
pub type FdbPru1DfVlanW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - 11:0\\]
FDB Default VLAN for PRU0"]
    #[inline(always)]
    pub fn fdb_pru0_df_vlan(&self) -> FdbPru0DfVlanR {
        FdbPru0DfVlanR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - 27:16\\]
FDB Default VLAN for PRU1"]
    #[inline(always)]
    pub fn fdb_pru1_df_vlan(&self) -> FdbPru1DfVlanR {
        FdbPru1DfVlanR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - 11:0\\]
FDB Default VLAN for PRU0"]
    #[inline(always)]
    #[must_use]
    pub fn fdb_pru0_df_vlan(&mut self) -> FdbPru0DfVlanW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbDfVlanSpec> {
        FdbPru0DfVlanW::new(self, 0)
    }
    #[doc = "Bits 16:27 - 27:16\\]
FDB Default VLAN for PRU1"]
    #[inline(always)]
    #[must_use]
    pub fn fdb_pru1_df_vlan(&mut self) -> FdbPru1DfVlanW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbDfVlanSpec> {
        FdbPru1DfVlanW::new(self, 16)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_fdb_df_vlan\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_fdb_df_vlan::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_fdb_df_vlan::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbDfVlanSpec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbDfVlanSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_fdb_df_vlan::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbDfVlanSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_fdb_df_vlan::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbDfVlanSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_fdb_df_vlan to value 0x0001_0001"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbDfVlanSpec {
    const RESET_VALUE: u32 = 0x0001_0001;
}
