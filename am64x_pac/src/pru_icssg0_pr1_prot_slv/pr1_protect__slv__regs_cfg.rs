#[doc = "Register `PR1_PROTECT__SLV__REGS_cfg` reader"]
pub type R = crate::R<Pr1Protect_Slv_RegsCfgSpec>;
#[doc = "Register `PR1_PROTECT__SLV__REGS_cfg` writer"]
pub type W = crate::W<Pr1Protect_Slv_RegsCfgSpec>;
#[doc = "Field `PRU0_WP_EN` reader - 0:0\\]
Write Protect PRU0 access Debug IMEM 0: disable 1: enable"]
pub type Pru0WpEnR = crate::BitReader;
#[doc = "Field `PRU0_WP_EN` writer - 0:0\\]
Write Protect PRU0 access Debug IMEM 0: disable 1: enable"]
pub type Pru0WpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_WP_EN` reader - 1:1\\]
Write Protect PRU1 access Debug IMEM 0: disable 1: enable"]
pub type Pru1WpEnR = crate::BitReader;
#[doc = "Field `PRU1_WP_EN` writer - 1:1\\]
Write Protect PRU1 access Debug IMEM 0: disable 1: enable"]
pub type Pru1WpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTU0_PRU_WP_EN` reader - 2:2\\]
Write Protect RTU0_PRU access Debug IMEM 0: disable 1: enable"]
pub type Rtu0PruWpEnR = crate::BitReader;
#[doc = "Field `RTU0_PRU_WP_EN` writer - 2:2\\]
Write Protect RTU0_PRU access Debug IMEM 0: disable 1: enable"]
pub type Rtu0PruWpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTU1_PRU_WP_EN` reader - 3:3\\]
Write Protect RTU1_PRU access Debug IMEM 0: disable 1: enable"]
pub type Rtu1PruWpEnR = crate::BitReader;
#[doc = "Field `RTU1_PRU_WP_EN` writer - 3:3\\]
Write Protect RTU1_PRU access Debug IMEM 0: disable 1: enable"]
pub type Rtu1PruWpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICSS_CFG_WP_EN` reader - 4:4\\]
Write Protect ICSS_CFG 0: disable 1: enable"]
pub type IcssCfgWpEnR = crate::BitReader;
#[doc = "Field `ICSS_CFG_WP_EN` writer - 4:4\\]
Write Protect ICSS_CFG 0: disable 1: enable"]
pub type IcssCfgWpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU0_DMEM0_LOCK_EN` reader - 5:5\\]
Write Protect DMEM0 0: disable 1: enable When enabled only PRU0 can write to DMEM0"]
pub type Pru0Dmem0LockEnR = crate::BitReader;
#[doc = "Field `PRU0_DMEM0_LOCK_EN` writer - 5:5\\]
Write Protect DMEM0 0: disable 1: enable When enabled only PRU0 can write to DMEM0"]
pub type Pru0Dmem0LockEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRU1_DMEM1_LOCK_EN` reader - 6:6\\]
Write Protect DMEM1 0: disable 1: enable When enabled only PRU1 can write to DMEM1"]
pub type Pru1Dmem1LockEnR = crate::BitReader;
#[doc = "Field `PRU1_DMEM1_LOCK_EN` writer - 6:6\\]
Write Protect DMEM1 0: disable 1: enable When enabled only PRU1 can write to DMEM1"]
pub type Pru1Dmem1LockEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Write Protect PRU0 access Debug IMEM 0: disable 1: enable"]
    #[inline(always)]
    pub fn pru0_wp_en(&self) -> Pru0WpEnR {
        Pru0WpEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write Protect PRU1 access Debug IMEM 0: disable 1: enable"]
    #[inline(always)]
    pub fn pru1_wp_en(&self) -> Pru1WpEnR {
        Pru1WpEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Write Protect RTU0_PRU access Debug IMEM 0: disable 1: enable"]
    #[inline(always)]
    pub fn rtu0_pru_wp_en(&self) -> Rtu0PruWpEnR {
        Rtu0PruWpEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Write Protect RTU1_PRU access Debug IMEM 0: disable 1: enable"]
    #[inline(always)]
    pub fn rtu1_pru_wp_en(&self) -> Rtu1PruWpEnR {
        Rtu1PruWpEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Write Protect ICSS_CFG 0: disable 1: enable"]
    #[inline(always)]
    pub fn icss_cfg_wp_en(&self) -> IcssCfgWpEnR {
        IcssCfgWpEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Write Protect DMEM0 0: disable 1: enable When enabled only PRU0 can write to DMEM0"]
    #[inline(always)]
    pub fn pru0_dmem0_lock_en(&self) -> Pru0Dmem0LockEnR {
        Pru0Dmem0LockEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Write Protect DMEM1 0: disable 1: enable When enabled only PRU1 can write to DMEM1"]
    #[inline(always)]
    pub fn pru1_dmem1_lock_en(&self) -> Pru1Dmem1LockEnR {
        Pru1Dmem1LockEnR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Write Protect PRU0 access Debug IMEM 0: disable 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_wp_en(&mut self) -> Pru0WpEnW<Pr1Protect_Slv_RegsCfgSpec> {
        Pru0WpEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Write Protect PRU1 access Debug IMEM 0: disable 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_wp_en(&mut self) -> Pru1WpEnW<Pr1Protect_Slv_RegsCfgSpec> {
        Pru1WpEnW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Write Protect RTU0_PRU access Debug IMEM 0: disable 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtu0_pru_wp_en(&mut self) -> Rtu0PruWpEnW<Pr1Protect_Slv_RegsCfgSpec> {
        Rtu0PruWpEnW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Write Protect RTU1_PRU access Debug IMEM 0: disable 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtu1_pru_wp_en(&mut self) -> Rtu1PruWpEnW<Pr1Protect_Slv_RegsCfgSpec> {
        Rtu1PruWpEnW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Write Protect ICSS_CFG 0: disable 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn icss_cfg_wp_en(&mut self) -> IcssCfgWpEnW<Pr1Protect_Slv_RegsCfgSpec> {
        IcssCfgWpEnW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Write Protect DMEM0 0: disable 1: enable When enabled only PRU0 can write to DMEM0"]
    #[inline(always)]
    #[must_use]
    pub fn pru0_dmem0_lock_en(&mut self) -> Pru0Dmem0LockEnW<Pr1Protect_Slv_RegsCfgSpec> {
        Pru0Dmem0LockEnW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Write Protect DMEM1 0: disable 1: enable When enabled only PRU1 can write to DMEM1"]
    #[inline(always)]
    #[must_use]
    pub fn pru1_dmem1_lock_en(&mut self) -> Pru1Dmem1LockEnW<Pr1Protect_Slv_RegsCfgSpec> {
        Pru1Dmem1LockEnW::new(self, 6)
    }
}
#[doc = "Config\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1_protect__slv__regs_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1_protect__slv__regs_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pr1Protect_Slv_RegsCfgSpec;
impl crate::RegisterSpec for Pr1Protect_Slv_RegsCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr1_protect__slv__regs_cfg::R`](R) reader structure"]
impl crate::Readable for Pr1Protect_Slv_RegsCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pr1_protect__slv__regs_cfg::W`](W) writer structure"]
impl crate::Writable for Pr1Protect_Slv_RegsCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PR1_PROTECT__SLV__REGS_cfg to value 0"]
impl crate::Resettable for Pr1Protect_Slv_RegsCfgSpec {
    const RESET_VALUE: u32 = 0;
}
