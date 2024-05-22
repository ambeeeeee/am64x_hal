#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft1_3_da_mask1_pru1` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1_3DaMask1Pru1Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft1_3_da_mask1_pru1` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1_3DaMask1Pru1Spec>;
#[doc = "Field `FT1_3_DA_MASK1` reader - 15:0\\]
Filter1 MDA6:MDA5 set to 1 to mask that bit"]
pub type Ft1_3DaMask1R = crate::FieldReader<u16>;
#[doc = "Field `FT1_3_DA_MASK1` writer - 15:0\\]
Filter1 MDA6:MDA5 set to 1 to mask that bit"]
pub type Ft1_3DaMask1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15:0\\]
Filter1 MDA6:MDA5 set to 1 to mask that bit"]
    #[inline(always)]
    pub fn ft1_3_da_mask1(&self) -> Ft1_3DaMask1R {
        Ft1_3DaMask1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15:0\\]
Filter1 MDA6:MDA5 set to 1 to mask that bit"]
    #[inline(always)]
    #[must_use]
    pub fn ft1_3_da_mask1(
        &mut self,
    ) -> Ft1_3DaMask1W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1_3DaMask1Pru1Spec> {
        Ft1_3DaMask1W::new(self, 0)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft1_3_da_mask1_pru1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft1_3_da_mask1_pru1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft1_3_da_mask1_pru1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1_3DaMask1Pru1Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1_3DaMask1Pru1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft1_3_da_mask1_pru1::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1_3DaMask1Pru1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft1_3_da_mask1_pru1::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1_3DaMask1Pru1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft1_3_da_mask1_pru1 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1_3DaMask1Pru1Spec {
    const RESET_VALUE: u32 = 0;
}
