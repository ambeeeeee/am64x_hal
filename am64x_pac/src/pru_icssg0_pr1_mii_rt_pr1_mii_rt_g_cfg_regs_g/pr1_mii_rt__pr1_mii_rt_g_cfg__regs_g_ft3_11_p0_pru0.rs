#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_11_p0_pru0` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_11P0Pru0Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_11_p0_pru0` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_11P0Pru0Spec>;
#[doc = "Field `FT3_11_P0` reader - 31:0\\]
Filter3 P4:P1"]
pub type Ft3_11P0R = crate::FieldReader<u32>;
#[doc = "Field `FT3_11_P0` writer - 31:0\\]
Filter3 P4:P1"]
pub type Ft3_11P0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Filter3 P4:P1"]
    #[inline(always)]
    pub fn ft3_11_p0(&self) -> Ft3_11P0R {
        Ft3_11P0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Filter3 P4:P1"]
    #[inline(always)]
    #[must_use]
    pub fn ft3_11_p0(&mut self) -> Ft3_11P0W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_11P0Pru0Spec> {
        Ft3_11P0W::new(self, 0)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_11_p0_pru0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_11_p0_pru0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_11_p0_pru0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_11P0Pru0Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_11P0Pru0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_11_p0_pru0::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_11P0Pru0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_11_p0_pru0::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_11P0Pru0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_11_p0_pru0 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_11P0Pru0Spec {
    const RESET_VALUE: u32 = 0;
}
