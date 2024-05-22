#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_green_pru1` reader"]
pub type R = crate::R<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxGreenPru1Spec>;
#[doc = "Register `PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_green_pru1` writer"]
pub type W = crate::W<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxGreenPru1Spec>;
#[doc = "Field `RX_GREEN_CMP_SEL` reader - 3:0\\]
define which IEP CMP start green"]
pub type RxGreenCmpSelR = crate::FieldReader;
#[doc = "Field `RX_GREEN_CMP_SEL` writer - 3:0\\]
define which IEP CMP start green"]
pub type RxGreenCmpSelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RX_GREEN_VAL` reader - 4:4\\]
0 RED 1 GREEN status"]
pub type RxGreenValR = crate::BitReader;
#[doc = "Field `RX_GREEN_VAL` writer - 4:4\\]
0 RED 1 GREEN status"]
pub type RxGreenValW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
define which IEP CMP start green"]
    #[inline(always)]
    pub fn rx_green_cmp_sel(&self) -> RxGreenCmpSelR {
        RxGreenCmpSelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
0 RED 1 GREEN status"]
    #[inline(always)]
    pub fn rx_green_val(&self) -> RxGreenValR {
        RxGreenValR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
define which IEP CMP start green"]
    #[inline(always)]
    #[must_use]
    pub fn rx_green_cmp_sel(
        &mut self,
    ) -> RxGreenCmpSelW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxGreenPru1Spec> {
        RxGreenCmpSelW::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
0 RED 1 GREEN status"]
    #[inline(always)]
    #[must_use]
    pub fn rx_green_val(&mut self) -> RxGreenValW<Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxGreenPru1Spec> {
        RxGreenValW::new(self, 4)
    }
}
#[doc = "PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_green_pru1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_green_pru1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_green_pru1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxGreenPru1Spec;
impl crate::RegisterSpec for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxGreenPru1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_green_pru1::R`](R) reader structure"]
impl crate::Readable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxGreenPru1Spec {}
#[doc = "`write(|w| ..)` method takes [`pr1_mii_rt__pr1_mii_rt_g_cfg__regs_g_rx_green_pru1::W`](W) writer structure"]
impl crate::Writable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxGreenPru1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_MII_RT__PR1_MII_RT_G_CFG__REGS_G_rx_green_pru1 to value 0"]
impl crate::Resettable for Pr1MiiRt_Pr1MiiRtGCfg_RegsGRxGreenPru1Spec {
    const RESET_VALUE: u32 = 0;
}
