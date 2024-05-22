#[doc = "Register `CFG0_MCU_MTOG_CTRL` reader"]
pub type R = crate::R<Cfg0McuMtogCtrlSpec>;
#[doc = "Register `CFG0_MCU_MTOG_CTRL` writer"]
pub type W = crate::W<Cfg0McuMtogCtrlSpec>;
#[doc = "Field `MCU_MTOG_CTRL_TIMEOUT_VAL` reader - 2:0\\]
Gasket Timeout Value"]
pub type McuMtogCtrlTimeoutValR = crate::FieldReader;
#[doc = "Field `MCU_MTOG_CTRL_TIMEOUT_VAL` writer - 2:0\\]
Gasket Timeout Value"]
pub type McuMtogCtrlTimeoutValW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MCU_MTOG_CTRL_TIMEOUT_EN` reader - 15:15\\]
Timeout Active"]
pub type McuMtogCtrlTimeoutEnR = crate::BitReader;
#[doc = "Field `MCU_MTOG_CTRL_TIMEOUT_EN` writer - 15:15\\]
Timeout Active"]
pub type McuMtogCtrlTimeoutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCU_MTOG_CTRL_FORCE_TIMEOUT` reader - 23:16\\]
Force Timout"]
pub type McuMtogCtrlForceTimeoutR = crate::FieldReader;
#[doc = "Field `MCU_MTOG_CTRL_FORCE_TIMEOUT` writer - 23:16\\]
Force Timout"]
pub type McuMtogCtrlForceTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MCU_MTOG_CTRL_IDLE_STAT` reader - 31:31\\]
Idle Status:"]
pub type McuMtogCtrlIdleStatR = crate::BitReader;
#[doc = "Field `MCU_MTOG_CTRL_IDLE_STAT` writer - 31:31\\]
Idle Status:"]
pub type McuMtogCtrlIdleStatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Gasket Timeout Value"]
    #[inline(always)]
    pub fn mcu_mtog_ctrl_timeout_val(&self) -> McuMtogCtrlTimeoutValR {
        McuMtogCtrlTimeoutValR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Timeout Active"]
    #[inline(always)]
    pub fn mcu_mtog_ctrl_timeout_en(&self) -> McuMtogCtrlTimeoutEnR {
        McuMtogCtrlTimeoutEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Force Timout"]
    #[inline(always)]
    pub fn mcu_mtog_ctrl_force_timeout(&self) -> McuMtogCtrlForceTimeoutR {
        McuMtogCtrlForceTimeoutR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
Idle Status:"]
    #[inline(always)]
    pub fn mcu_mtog_ctrl_idle_stat(&self) -> McuMtogCtrlIdleStatR {
        McuMtogCtrlIdleStatR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Gasket Timeout Value"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_mtog_ctrl_timeout_val(&mut self) -> McuMtogCtrlTimeoutValW<Cfg0McuMtogCtrlSpec> {
        McuMtogCtrlTimeoutValW::new(self, 0)
    }
    #[doc = "Bit 15 - 15:15\\]
Timeout Active"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_mtog_ctrl_timeout_en(&mut self) -> McuMtogCtrlTimeoutEnW<Cfg0McuMtogCtrlSpec> {
        McuMtogCtrlTimeoutEnW::new(self, 15)
    }
    #[doc = "Bits 16:23 - 23:16\\]
Force Timout"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_mtog_ctrl_force_timeout(&mut self) -> McuMtogCtrlForceTimeoutW<Cfg0McuMtogCtrlSpec> {
        McuMtogCtrlForceTimeoutW::new(self, 16)
    }
    #[doc = "Bit 31 - 31:31\\]
Idle Status:"]
    #[inline(always)]
    #[must_use]
    pub fn mcu_mtog_ctrl_idle_stat(&mut self) -> McuMtogCtrlIdleStatW<Cfg0McuMtogCtrlSpec> {
        McuMtogCtrlIdleStatW::new(self, 31)
    }
}
#[doc = "CFG0_MCU_MTOG_CTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg0_mcu_mtog_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg0_mcu_mtog_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg0McuMtogCtrlSpec;
impl crate::RegisterSpec for Cfg0McuMtogCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg0_mcu_mtog_ctrl::R`](R) reader structure"]
impl crate::Readable for Cfg0McuMtogCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg0_mcu_mtog_ctrl::W`](W) writer structure"]
impl crate::Writable for Cfg0McuMtogCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG0_MCU_MTOG_CTRL to value 0"]
impl crate::Resettable for Cfg0McuMtogCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
