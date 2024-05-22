#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_mac_pru0_0` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGMacPru0_0Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_mac_pru0_0` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGMacPru0_0Spec>;
#[doc = "Field `MAC_PRU0_0` reader - 31:0\\]
MAC pru0 DA3:DA0 Used for SAV and DA match"]
pub type MacPru0_0R = crate::FieldReader<u32>;
#[doc = "Field `MAC_PRU0_0` writer - 31:0\\]
MAC pru0 DA3:DA0 Used for SAV and DA match"]
pub type MacPru0_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
MAC pru0 DA3:DA0 Used for SAV and DA match"]
    #[inline(always)]
    pub fn mac_pru0_0(&self) -> MacPru0_0R {
        MacPru0_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
MAC pru0 DA3:DA0 Used for SAV and DA match"]
    #[inline(always)]
    #[must_use]
    pub fn mac_pru0_0(&mut self) -> MacPru0_0W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGMacPru0_0Spec> {
        MacPru0_0W::new(self, 0)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_mac_pru0_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_mac_pru0_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_mac_pru0_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGMacPru0_0Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGMacPru0_0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_mac_pru0_0::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGMacPru0_0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_mac_pru0_0::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGMacPru0_0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_mac_pru0_0 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGMacPru0_0Spec {
    const RESET_VALUE: u32 = 0;
}
