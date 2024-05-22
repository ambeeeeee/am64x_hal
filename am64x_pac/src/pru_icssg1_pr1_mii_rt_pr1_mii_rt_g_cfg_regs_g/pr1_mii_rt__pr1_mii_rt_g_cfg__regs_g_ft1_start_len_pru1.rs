#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft1_start_len_pru1` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1StartLenPru1Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft1_start_len_pru1` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1StartLenPru1Spec>;
#[doc = "Field `FT1_START` reader - 14:0\\]
Byte count start for Filter1. Any wrt will clear all Filter1 Status Bits"]
pub type Ft1StartR = crate::FieldReader<u16>;
#[doc = "Field `FT1_START` writer - 14:0\\]
Byte count start for Filter1. Any wrt will clear all Filter1 Status Bits"]
pub type Ft1StartW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `FT1_LEN` reader - 19:16\\]
Defines the total number of Bytes Filter1 will check before Valid bit is set"]
pub type Ft1LenR = crate::FieldReader;
#[doc = "Field `FT1_LEN` writer - 19:16\\]
Defines the total number of Bytes Filter1 will check before Valid bit is set"]
pub type Ft1LenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:14 - 14:0\\]
Byte count start for Filter1. Any wrt will clear all Filter1 Status Bits"]
    #[inline(always)]
    pub fn ft1_start(&self) -> Ft1StartR {
        Ft1StartR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the total number of Bytes Filter1 will check before Valid bit is set"]
    #[inline(always)]
    pub fn ft1_len(&self) -> Ft1LenR {
        Ft1LenR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - 14:0\\]
Byte count start for Filter1. Any wrt will clear all Filter1 Status Bits"]
    #[inline(always)]
    #[must_use]
    pub fn ft1_start(&mut self) -> Ft1StartW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1StartLenPru1Spec> {
        Ft1StartW::new(self, 0)
    }
    #[doc = "Bits 16:19 - 19:16\\]
Defines the total number of Bytes Filter1 will check before Valid bit is set"]
    #[inline(always)]
    #[must_use]
    pub fn ft1_len(&mut self) -> Ft1LenW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1StartLenPru1Spec> {
        Ft1LenW::new(self, 16)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft1_start_len_pru1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft1_start_len_pru1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft1_start_len_pru1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1StartLenPru1Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1StartLenPru1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft1_start_len_pru1::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1StartLenPru1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_ft1_start_len_pru1::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1StartLenPru1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_ft1_start_len_pru1 to value 0x0006_0000"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGFt1StartLenPru1Spec {
    const RESET_VALUE: u32 = 0x0006_0000;
}
