#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_0_jmp_offset_pru1` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_0JmpOffsetPru1Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_0_jmp_offset_pru1` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_0JmpOffsetPru1Spec>;
#[doc = "Field `FT3_0_IJMP_OFFSET_PRU1` reader - 14:0\\]
Defines the Intial Offset to compare when auto arm jump is enabled"]
pub type Ft3_0IjmpOffsetPru1R = crate::FieldReader<u16>;
#[doc = "Field `FT3_0_IJMP_OFFSET_PRU1` writer - 14:0\\]
Defines the Intial Offset to compare when auto arm jump is enabled"]
pub type Ft3_0IjmpOffsetPru1W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `FT3_0_NJMP_OFFSET_PRU1` reader - 30:16\\]
Defines the Next Offset to compare when auto arm jump is enabled"]
pub type Ft3_0NjmpOffsetPru1R = crate::FieldReader<u16>;
#[doc = "Field `FT3_0_NJMP_OFFSET_PRU1` writer - 30:16\\]
Defines the Next Offset to compare when auto arm jump is enabled"]
pub type Ft3_0NjmpOffsetPru1W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - 14:0\\]
Defines the Intial Offset to compare when auto arm jump is enabled"]
    #[inline(always)]
    pub fn ft3_0_ijmp_offset_pru1(&self) -> Ft3_0IjmpOffsetPru1R {
        Ft3_0IjmpOffsetPru1R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:30 - 30:16\\]
Defines the Next Offset to compare when auto arm jump is enabled"]
    #[inline(always)]
    pub fn ft3_0_njmp_offset_pru1(&self) -> Ft3_0NjmpOffsetPru1R {
        Ft3_0NjmpOffsetPru1R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - 14:0\\]
Defines the Intial Offset to compare when auto arm jump is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ft3_0_ijmp_offset_pru1(
        &mut self,
    ) -> Ft3_0IjmpOffsetPru1W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_0JmpOffsetPru1Spec> {
        Ft3_0IjmpOffsetPru1W::new(self, 0)
    }
    #[doc = "Bits 16:30 - 30:16\\]
Defines the Next Offset to compare when auto arm jump is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ft3_0_njmp_offset_pru1(
        &mut self,
    ) -> Ft3_0NjmpOffsetPru1W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_0JmpOffsetPru1Spec> {
        Ft3_0NjmpOffsetPru1W::new(self, 16)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_0_jmp_offset_pru1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_0_jmp_offset_pru1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_0_jmp_offset_pru1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_0JmpOffsetPru1Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_0JmpOffsetPru1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_0_jmp_offset_pru1::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_0JmpOffsetPru1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft3_0_jmp_offset_pru1::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_0JmpOffsetPru1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft3_0_jmp_offset_pru1 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt3_0JmpOffsetPru1Spec {
    const RESET_VALUE: u32 = 0;
}
