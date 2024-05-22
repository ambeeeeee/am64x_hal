#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_stat_bkt2_size_pru0` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatBkt2SizePru0Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_stat_bkt2_size_pru0` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatBkt2SizePru0Spec>;
#[doc = "Field `RX_STAT_BKT2_SIZE` reader - 13:0\\]
RX Bucket2 Byte Size"]
pub type RxStatBkt2SizeR = crate::FieldReader<u16>;
#[doc = "Field `RX_STAT_BKT2_SIZE` writer - 13:0\\]
RX Bucket2 Byte Size"]
pub type RxStatBkt2SizeW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - 13:0\\]
RX Bucket2 Byte Size"]
    #[inline(always)]
    pub fn rx_stat_bkt2_size(&self) -> RxStatBkt2SizeR {
        RxStatBkt2SizeR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - 13:0\\]
RX Bucket2 Byte Size"]
    #[inline(always)]
    #[must_use]
    pub fn rx_stat_bkt2_size(
        &mut self,
    ) -> RxStatBkt2SizeW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatBkt2SizePru0Spec> {
        RxStatBkt2SizeW::new(self, 0)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_stat_bkt2_size_pru0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_stat_bkt2_size_pru0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_stat_bkt2_size_pru0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatBkt2SizePru0Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatBkt2SizePru0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_stat_bkt2_size_pru0::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatBkt2SizePru0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_stat_bkt2_size_pru0::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatBkt2SizePru0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_stat_bkt2_size_pru0 to value 0x0128"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxStatBkt2SizePru0Spec {
    const RESET_VALUE: u32 = 0x0128;
}
