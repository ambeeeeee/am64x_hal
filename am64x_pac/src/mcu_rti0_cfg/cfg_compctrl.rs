#[doc = "Register `CFG_COMPCTRL` reader"]
pub type R = crate::R<CfgCompctrlSpec>;
#[doc = "Register `CFG_COMPCTRL` writer"]
pub type W = crate::W<CfgCompctrlSpec>;
#[doc = "Field `COMPSEL0` reader - 0:0\\]
This bit determines the counter with which the compare value hold in compare register 0 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
pub type Compsel0R = crate::BitReader;
#[doc = "Field `COMPSEL0` writer - 0:0\\]
This bit determines the counter with which the compare value hold in compare register 0 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
pub type Compsel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPSEL1` reader - 4:4\\]
This bit determines the counter with which the compare value hold in compare register 1 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
pub type Compsel1R = crate::BitReader;
#[doc = "Field `COMPSEL1` writer - 4:4\\]
This bit determines the counter with which the compare value hold in compare register 1 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
pub type Compsel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPSEL2` reader - 8:8\\]
This bit determines the counter with which the compare value hold in compare register 2 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
pub type Compsel2R = crate::BitReader;
#[doc = "Field `COMPSEL2` writer - 8:8\\]
This bit determines the counter with which the compare value hold in compare register 2 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
pub type Compsel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPSEL3` reader - 12:12\\]
This bit determines the counter with which the compare value hold in compare register 3 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
pub type Compsel3R = crate::BitReader;
#[doc = "Field `COMPSEL3` writer - 12:12\\]
This bit determines the counter with which the compare value hold in compare register 3 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
pub type Compsel3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit determines the counter with which the compare value hold in compare register 0 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
    #[inline(always)]
    pub fn compsel0(&self) -> Compsel0R {
        Compsel0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
This bit determines the counter with which the compare value hold in compare register 1 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
    #[inline(always)]
    pub fn compsel1(&self) -> Compsel1R {
        Compsel1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
This bit determines the counter with which the compare value hold in compare register 2 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
    #[inline(always)]
    pub fn compsel2(&self) -> Compsel2R {
        Compsel2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
This bit determines the counter with which the compare value hold in compare register 3 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
    #[inline(always)]
    pub fn compsel3(&self) -> Compsel3R {
        Compsel3R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit determines the counter with which the compare value hold in compare register 0 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
    #[inline(always)]
    #[must_use]
    pub fn compsel0(&mut self) -> Compsel0W<CfgCompctrlSpec> {
        Compsel0W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
This bit determines the counter with which the compare value hold in compare register 1 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
    #[inline(always)]
    #[must_use]
    pub fn compsel1(&mut self) -> Compsel1W<CfgCompctrlSpec> {
        Compsel1W::new(self, 4)
    }
    #[doc = "Bit 8 - 8:8\\]
This bit determines the counter with which the compare value hold in compare register 2 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
    #[inline(always)]
    #[must_use]
    pub fn compsel2(&mut self) -> Compsel2W<CfgCompctrlSpec> {
        Compsel2W::new(self, 8)
    }
    #[doc = "Bit 12 - 12:12\\]
This bit determines the counter with which the compare value hold in compare register 3 is compared. User and privilege mode (read): 0 = value will be compared with FRC 0 1 = value will be compared with FRC 1 Privilege mode (write): 0 = enable compare with FRC 0 1 = enable compare with FRC 1"]
    #[inline(always)]
    #[must_use]
    pub fn compsel3(&mut self) -> Compsel3W<CfgCompctrlSpec> {
        Compsel3W::new(self, 12)
    }
}
#[doc = "CFG_COMPCTRL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg_compctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg_compctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgCompctrlSpec;
impl crate::RegisterSpec for CfgCompctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg_compctrl::R`](R) reader structure"]
impl crate::Readable for CfgCompctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg_compctrl::W`](W) writer structure"]
impl crate::Writable for CfgCompctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG_COMPCTRL to value 0"]
impl crate::Resettable for CfgCompctrlSpec {
    const RESET_VALUE: u32 = 0;
}
