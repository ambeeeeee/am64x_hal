#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_fdb_gen_cfg2` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg2Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_fdb_gen_cfg2` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg2Spec>;
#[doc = "Field `FDB_PRU0_EN` reader - 0:0\\]
FDB PRU0 Enable"]
pub type FdbPru0EnR = crate::BitReader;
#[doc = "Field `FDB_PRU0_EN` writer - 0:0\\]
FDB PRU0 Enable"]
pub type FdbPru0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDB_PRU1_EN` reader - 1:1\\]
FDB PRU1 Enable"]
pub type FdbPru1EnR = crate::BitReader;
#[doc = "Field `FDB_PRU1_EN` writer - 1:1\\]
FDB PRU1 Enable"]
pub type FdbPru1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDB_HOST_EN` reader - 2:2\\]
FDB HOST Enable"]
pub type FdbHostEnR = crate::BitReader;
#[doc = "Field `FDB_HOST_EN` writer - 2:2\\]
FDB HOST Enable"]
pub type FdbHostEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDB_HSR_EN` reader - 5:5\\]
FDB Global HSR Enable note VLAN most be disabled"]
pub type FdbHsrEnR = crate::BitReader;
#[doc = "Field `FDB_HSR_EN` writer - 5:5\\]
FDB Global HSR Enable note VLAN most be disabled"]
pub type FdbHsrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDB_VLAN_EN` reader - 6:6\\]
FDB Global VLAN Enable"]
pub type FdbVlanEnR = crate::BitReader;
#[doc = "Field `FDB_VLAN_EN` writer - 6:6\\]
FDB Global VLAN Enable"]
pub type FdbVlanEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDB_GEN_MODE_EN_BK0` reader - 7:7\\]
FDB General Mode Enable Bank0 if set PRU0/PRU1/HOST will get disabled"]
pub type FdbGenModeEnBk0R = crate::BitReader;
#[doc = "Field `FDB_GEN_MODE_EN_BK0` writer - 7:7\\]
FDB General Mode Enable Bank0 if set PRU0/PRU1/HOST will get disabled"]
pub type FdbGenModeEnBk0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDB_GEN_MODE_EN_BK1` reader - 8:8\\]
FDB General Mode Enable Bank1 if set PRU0/PRU1/HOST will get disabled"]
pub type FdbGenModeEnBk1R = crate::BitReader;
#[doc = "Field `FDB_GEN_MODE_EN_BK1` writer - 8:8\\]
FDB General Mode Enable Bank1 if set PRU0/PRU1/HOST will get disabled"]
pub type FdbGenModeEnBk1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDB_GEN_MODE_BYTE_EN` reader - 12:9\\]
FDB General Mode Byte compare size 0 = 1 Byte, 15 = 16 Bytes"]
pub type FdbGenModeByteEnR = crate::FieldReader;
#[doc = "Field `FDB_GEN_MODE_BYTE_EN` writer - 12:9\\]
FDB General Mode Byte compare size 0 = 1 Byte, 15 = 16 Bytes"]
pub type FdbGenModeByteEnW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
FDB PRU0 Enable"]
    #[inline(always)]
    pub fn fdb_pru0_en(&self) -> FdbPru0EnR {
        FdbPru0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
FDB PRU1 Enable"]
    #[inline(always)]
    pub fn fdb_pru1_en(&self) -> FdbPru1EnR {
        FdbPru1EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
FDB HOST Enable"]
    #[inline(always)]
    pub fn fdb_host_en(&self) -> FdbHostEnR {
        FdbHostEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
FDB Global HSR Enable note VLAN most be disabled"]
    #[inline(always)]
    pub fn fdb_hsr_en(&self) -> FdbHsrEnR {
        FdbHsrEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
FDB Global VLAN Enable"]
    #[inline(always)]
    pub fn fdb_vlan_en(&self) -> FdbVlanEnR {
        FdbVlanEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
FDB General Mode Enable Bank0 if set PRU0/PRU1/HOST will get disabled"]
    #[inline(always)]
    pub fn fdb_gen_mode_en_bk0(&self) -> FdbGenModeEnBk0R {
        FdbGenModeEnBk0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
FDB General Mode Enable Bank1 if set PRU0/PRU1/HOST will get disabled"]
    #[inline(always)]
    pub fn fdb_gen_mode_en_bk1(&self) -> FdbGenModeEnBk1R {
        FdbGenModeEnBk1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:12 - 12:9\\]
FDB General Mode Byte compare size 0 = 1 Byte, 15 = 16 Bytes"]
    #[inline(always)]
    pub fn fdb_gen_mode_byte_en(&self) -> FdbGenModeByteEnR {
        FdbGenModeByteEnR::new(((self.bits >> 9) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
FDB PRU0 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdb_pru0_en(&mut self) -> FdbPru0EnW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg2Spec> {
        FdbPru0EnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
FDB PRU1 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdb_pru1_en(&mut self) -> FdbPru1EnW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg2Spec> {
        FdbPru1EnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
FDB HOST Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdb_host_en(&mut self) -> FdbHostEnW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg2Spec> {
        FdbHostEnW::new(self, 2)
    }
    #[doc = "Bit 5 - 5:5\\]
FDB Global HSR Enable note VLAN most be disabled"]
    #[inline(always)]
    #[must_use]
    pub fn fdb_hsr_en(&mut self) -> FdbHsrEnW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg2Spec> {
        FdbHsrEnW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
FDB Global VLAN Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdb_vlan_en(&mut self) -> FdbVlanEnW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg2Spec> {
        FdbVlanEnW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
FDB General Mode Enable Bank0 if set PRU0/PRU1/HOST will get disabled"]
    #[inline(always)]
    #[must_use]
    pub fn fdb_gen_mode_en_bk0(
        &mut self,
    ) -> FdbGenModeEnBk0W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg2Spec> {
        FdbGenModeEnBk0W::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
FDB General Mode Enable Bank1 if set PRU0/PRU1/HOST will get disabled"]
    #[inline(always)]
    #[must_use]
    pub fn fdb_gen_mode_en_bk1(
        &mut self,
    ) -> FdbGenModeEnBk1W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg2Spec> {
        FdbGenModeEnBk1W::new(self, 8)
    }
    #[doc = "Bits 9:12 - 12:9\\]
FDB General Mode Byte compare size 0 = 1 Byte, 15 = 16 Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn fdb_gen_mode_byte_en(
        &mut self,
    ) -> FdbGenModeByteEnW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg2Spec> {
        FdbGenModeByteEnW::new(self, 9)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_fdb_gen_cfg2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_fdb_gen_cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_fdb_gen_cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg2Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_fdb_gen_cfg2::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_fdb_gen_cfg2::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_fdb_gen_cfg2 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFdbGenCfg2Spec {
    const RESET_VALUE: u32 = 0;
}
