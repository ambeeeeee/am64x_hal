#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_9_start_pru0` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_9StartPru0Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_9_start_pru0` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_9StartPru0Spec>;
#[doc = "Field `FT3_START` reader - 14:0\\]
Byte count start for Filter3. Any wrt will clear all Filter3 Status Bits SW can read to determine next start during Auto"]
pub type Ft3StartR = crate::FieldReader<u16>;
#[doc = "Field `FT3_START` writer - 14:0\\]
Byte count start for Filter3. Any wrt will clear all Filter3 Status Bits SW can read to determine next start during Auto"]
pub type Ft3StartW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - 14:0\\]
Byte count start for Filter3. Any wrt will clear all Filter3 Status Bits SW can read to determine next start during Auto"]
    #[inline(always)]
    pub fn ft3_start(&self) -> Ft3StartR {
        Ft3StartR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - 14:0\\]
Byte count start for Filter3. Any wrt will clear all Filter3 Status Bits SW can read to determine next start during Auto"]
    #[inline(always)]
    #[must_use]
    pub fn ft3_start(&mut self) -> Ft3StartW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_9StartPru0Spec> {
        Ft3StartW::new(self, 0)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_9_start_pru0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_9_start_pru0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_9_start_pru0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_9StartPru0Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_9StartPru0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_9_start_pru0::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_9StartPru0Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_9_start_pru0::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_9StartPru0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_9_start_pru0 to value 0x12"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_9StartPru0Spec {
    const RESET_VALUE: u32 = 0x12;
}
