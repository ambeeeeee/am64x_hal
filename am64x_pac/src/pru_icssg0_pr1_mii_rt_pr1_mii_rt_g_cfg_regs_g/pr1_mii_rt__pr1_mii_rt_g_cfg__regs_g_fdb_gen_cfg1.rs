#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_fdb_gen_cfg1` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg1Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_fdb_gen_cfg1` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg1Spec>;
#[doc = "Field `FDB_BUCKET_SIZE` reader - 1:0\\]
FDB buket size 0:1 1:2 2:4 3:8"]
pub type FdbBucketSizeR = crate::FieldReader;
#[doc = "Field `FDB_BUCKET_SIZE` writer - 1:0\\]
FDB buket size 0:1 1:2 2:4 3:8"]
pub type FdbBucketSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FDB_HASH_SIZE` reader - 6:3\\]
FDB hash size 0:64 1:128 2:256 3:512 4:1024 5:2048"]
pub type FdbHashSizeR = crate::FieldReader;
#[doc = "Field `FDB_HASH_SIZE` writer - 6:3\\]
FDB hash size 0:64 1:128 2:256 3:512 4:1024 5:2048"]
pub type FdbHashSizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SMEM_VLAN_OFFSET` reader - 25:8\\]
SMEM VLAN FID table base address"]
pub type SmemVlanOffsetR = crate::FieldReader<u32>;
#[doc = "Field `SMEM_VLAN_OFFSET` writer - 25:8\\]
SMEM VLAN FID table base address"]
pub type SmemVlanOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
FDB buket size 0:1 1:2 2:4 3:8"]
    #[inline(always)]
    pub fn fdb_bucket_size(&self) -> FdbBucketSizeR {
        FdbBucketSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 3:6 - 6:3\\]
FDB hash size 0:64 1:128 2:256 3:512 4:1024 5:2048"]
    #[inline(always)]
    pub fn fdb_hash_size(&self) -> FdbHashSizeR {
        FdbHashSizeR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bits 8:25 - 25:8\\]
SMEM VLAN FID table base address"]
    #[inline(always)]
    pub fn smem_vlan_offset(&self) -> SmemVlanOffsetR {
        SmemVlanOffsetR::new((self.bits >> 8) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
FDB buket size 0:1 1:2 2:4 3:8"]
    #[inline(always)]
    #[must_use]
    pub fn fdb_bucket_size(&mut self) -> FdbBucketSizeW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg1Spec> {
        FdbBucketSizeW::new(self, 0)
    }
    #[doc = "Bits 3:6 - 6:3\\]
FDB hash size 0:64 1:128 2:256 3:512 4:1024 5:2048"]
    #[inline(always)]
    #[must_use]
    pub fn fdb_hash_size(&mut self) -> FdbHashSizeW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg1Spec> {
        FdbHashSizeW::new(self, 3)
    }
    #[doc = "Bits 8:25 - 25:8\\]
SMEM VLAN FID table base address"]
    #[inline(always)]
    #[must_use]
    pub fn smem_vlan_offset(
        &mut self,
    ) -> SmemVlanOffsetW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg1Spec> {
        SmemVlanOffsetW::new(self, 8)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_fdb_gen_cfg1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_fdb_gen_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_fdb_gen_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg1Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_fdb_gen_cfg1::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_fdb_gen_cfg1::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_fdb_gen_cfg1 to value 0x22"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg1Spec {
    const RESET_VALUE: u32 = 0x22;
}
