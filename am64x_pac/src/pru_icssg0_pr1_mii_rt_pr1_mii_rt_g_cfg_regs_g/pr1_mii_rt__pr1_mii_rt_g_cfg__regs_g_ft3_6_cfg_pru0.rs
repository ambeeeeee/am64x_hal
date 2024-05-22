#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_6_cfg_pru0` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_6CfgPru0Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_6_cfg_pru0` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_6CfgPru0Spec>;
#[doc = "Field `FT3_6CFG` reader - 1:0\\]
0: Disabled 1: EQ 2: GT 3: LT"]
pub type Ft3_6cfgR = crate::FieldReader;
#[doc = "Field `FT3_6CFG` writer - 1:0\\]
0: Disabled 1: EQ 2: GT 3: LT"]
pub type Ft3_6cfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FT3_6_VLAN_SKIP_EN` reader - 2:2\\]
0: Disabled 1: Enable"]
pub type Ft3_6VlanSkipEnR = crate::BitReader;
#[doc = "Field `FT3_6_VLAN_SKIP_EN` writer - 2:2\\]
0: Disabled 1: Enable"]
pub type Ft3_6VlanSkipEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FT3_6_TRIG_OR_EN` reader - 31:16\\]
Trigger ft3 select for auto skip enable, if one or more set the function is enabled note you can not slect the same ft3 only others"]
pub type Ft3_6TrigOrEnR = crate::FieldReader<u16>;
#[doc = "Field `FT3_6_TRIG_OR_EN` writer - 31:16\\]
Trigger ft3 select for auto skip enable, if one or more set the function is enabled note you can not slect the same ft3 only others"]
pub type Ft3_6TrigOrEnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
0: Disabled 1: EQ 2: GT 3: LT"]
    #[inline(always)]
    pub fn ft3_6cfg(&self) -> Ft3_6cfgR {
        Ft3_6cfgR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Disabled 1: Enable"]
    #[inline(always)]
    pub fn ft3_6_vlan_skip_en(&self) -> Ft3_6VlanSkipEnR {
        Ft3_6VlanSkipEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Trigger ft3 select for auto skip enable, if one or more set the function is enabled note you can not slect the same ft3 only others"]
    #[inline(always)]
    pub fn ft3_6_trig_or_en(&self) -> Ft3_6TrigOrEnR {
        Ft3_6TrigOrEnR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
0: Disabled 1: EQ 2: GT 3: LT"]
    #[inline(always)]
    #[must_use]
    pub fn ft3_6cfg(&mut self) -> Ft3_6cfgW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_6CfgPru0Spec> {
        Ft3_6cfgW::new(self, 0)
    }
    #[doc = "Bit 2 - 2:2\\]
0: Disabled 1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ft3_6_vlan_skip_en(
        &mut self,
    ) -> Ft3_6VlanSkipEnW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_6CfgPru0Spec> {
        Ft3_6VlanSkipEnW::new(self, 2)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Trigger ft3 select for auto skip enable, if one or more set the function is enabled note you can not slect the same ft3 only others"]
    #[inline(always)]
    #[must_use]
    pub fn ft3_6_trig_or_en(
        &mut self,
    ) -> Ft3_6TrigOrEnW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_6CfgPru0Spec> {
        Ft3_6TrigOrEnW::new(self, 16)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_6_cfg_pru0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_6_cfg_pru0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_6_cfg_pru0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_6CfgPru0Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_6CfgPru0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_6_cfg_pru0::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_6CfgPru0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_6_cfg_pru0::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_6CfgPru0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_6_cfg_pru0 to value 0x05"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_6CfgPru0Spec {
    const RESET_VALUE: u32 = 0x05;
}